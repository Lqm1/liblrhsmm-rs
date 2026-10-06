//! Weighted multistream emission tables and upstream pruning conventions.
use crate::{Model, ModelError, Observation, Segmentation};

/// Select which frame/state pairs are evaluated.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EmissionPruning {
    /// Evaluate every pair.
    None,
    /// Include neighboring states around each existing segment interval.
    AroundBoundaries { radius: f32 },
    /// Follow the frame-to-state diagonal, with an exclusive upper state bound.
    Linear { slope: f32 },
}

/// Time-major log densities; excluded pairs retain the legacy -1e15 sentinel.
#[derive(Debug, Clone, PartialEq)]
pub struct EmissionTable {
    frames: usize,
    states: usize,
    values: Vec<f32>,
}

/// Time-major emission densities, kept separate from log-score input tables.
/// The upstream approximation retains a small positive value for pruned pairs.
#[derive(Debug, Clone, PartialEq)]
pub struct EmissionProbabilities {
    frames: usize,
    states: usize,
    values: Vec<f32>,
}
impl EmissionProbabilities {
    pub fn frames(&self) -> usize {
        self.frames
    }
    pub fn states(&self) -> usize {
        self.states
    }
    pub fn values(&self) -> &[f32] {
        &self.values
    }
    pub fn get(&self, frame: usize, state: usize) -> Option<f32> {
        if frame >= self.frames || state >= self.states {
            return None;
        }
        self.values.get(frame * self.states + state).copied()
    }
}

impl EmissionTable {
    /// Convert each log score with the original approximate exponential.
    pub fn probabilities(&self) -> EmissionProbabilities {
        EmissionProbabilities {
            frames: self.frames,
            states: self.states,
            values: self
                .values
                .iter()
                .map(|&value| crate::probability::compatibility::exp(value))
                .collect(),
        }
    }
    /// Accept external time-major log densities for inference.
    pub fn from_log_values(
        frames: usize,
        states: usize,
        values: Vec<f32>,
    ) -> Result<Self, ModelError> {
        if frames == 0
            || states == 0
            || frames.checked_mul(states) != Some(values.len())
            || values.iter().any(|value| !value.is_finite())
        {
            return Err(ModelError(
                "log emission table requires positive matching dimensions and finite values",
            ));
        }
        Ok(Self {
            frames,
            states,
            values,
        })
    }
    pub fn frames(&self) -> usize {
        self.frames
    }
    pub fn states(&self) -> usize {
        self.states
    }
    pub fn values(&self) -> &[f32] {
        &self.values
    }
    pub fn get(&self, frame: usize, state: usize) -> Option<f32> {
        if frame >= self.frames || state >= self.states {
            return None;
        }
        self.values.get(frame * self.states + state).copied()
    }
}

impl Model {
    /// Sample non-log densities, corresponding to `lrh_sample_outputprob`.
    pub fn emission_probabilities(
        &self,
        observation: &Observation,
        segmentation: &Segmentation,
        temperature: f32,
        pruning: EmissionPruning,
    ) -> Result<EmissionProbabilities, ModelError> {
        Ok(self
            .emission_log_probabilities(observation, segmentation, temperature, pruning)?
            .probabilities())
    }

    /// Prepare and evaluate weighted stream densities for a state sequence.
    /// Pruning variants preserve the original C interval inclusivity.
    pub fn emission_log_probabilities(
        &self,
        observation: &Observation,
        segmentation: &Segmentation,
        temperature: f32,
        pruning: EmissionPruning,
    ) -> Result<EmissionTable, ModelError> {
        let prepared = self.prepare_emissions()?;
        self.sample_prepared_emissions(&prepared, observation, segmentation, temperature, pruning)
    }

    pub(crate) fn sample_prepared_emissions(
        &self,
        prepared_streams: &[Vec<crate::PreparedGaussian<'_>>],
        observation: &Observation,
        segmentation: &Segmentation,
        temperature: f32,
        pruning: EmissionPruning,
    ) -> Result<EmissionTable, ModelError> {
        observation.validate()?;
        segmentation.validate()?;
        self.validate()?;
        let states = segmentation.boundaries.len();
        if states == 0 || states > i32::MAX as usize || observation.frames == 0 {
            return Err(ModelError(
                "emission sampling requires positive frame and state counts",
            ));
        }
        if self.streams.len() != observation.streams.len()
            || self.streams.len() != segmentation.output_states.len()
        {
            return Err(ModelError(
                "model, observation, and segmentation stream counts differ",
            ));
        }
        if !temperature.is_finite() || temperature < 0.0 {
            return Err(ModelError("temperature must be finite and nonnegative"));
        }
        match pruning {
            EmissionPruning::AroundBoundaries { radius } => {
                crate::pruning::validate_boundary_radius(radius)?;
                let mut previous = 0;
                for &boundary in &segmentation.boundaries {
                    if boundary < previous || boundary < 0 || boundary as usize > observation.frames
                    {
                        return Err(ModelError(
                            "segment boundaries must be ordered within the observation",
                        ));
                    }
                    previous = boundary;
                }
                if previous as usize != observation.frames {
                    return Err(ModelError(
                        "final segment boundary must match the frame count",
                    ));
                }
            }
            EmissionPruning::Linear { slope } if !slope.is_finite() || slope < 0.0 => {
                return Err(ModelError(
                    "linear pruning slope must be finite and nonnegative",
                ));
            }
            _ => {}
        }
        for (stream_index, stream) in self.streams.iter().enumerate() {
            if !stream.weight.is_finite() || stream.weight < 0.0 {
                return Err(ModelError("stream weights must be finite and nonnegative"));
            }
            for &state in &segmentation.output_states[stream_index] {
                if state < 0 || state as usize >= stream.mixtures.len() {
                    return Err(ModelError("emission state is out of range"));
                }
            }
            for distribution in &stream.mixtures {
                if distribution.dimensions != observation.streams[stream_index].dimensions {
                    return Err(ModelError("emission and observation dimensions differ"));
                }
            }
        }
        let length = observation
            .frames
            .checked_mul(states)
            .ok_or(ModelError("emission table dimensions overflow"))?;
        let mut values = Vec::new();
        values
            .try_reserve_exact(length)
            .map_err(|_| ModelError("emission table allocation failed"))?;
        values.resize(length, -1e15);
        let mut evaluate = |frame: usize, state: usize| -> Result<(), ModelError> {
            let mut density = 0.0f32;
            for (stream_index, prepared) in prepared_streams.iter().enumerate() {
                let emission = segmentation.output_states[stream_index][state] as usize;
                let features = observation
                    .frame(stream_index, frame)
                    .ok_or(ModelError("invalid observation frame"))?;
                density += prepared[emission].log_probability(features, temperature)?
                    * self.streams[stream_index].weight;
            }
            values[frame * states + state] = density;
            Ok(())
        };
        match pruning {
            EmissionPruning::None => {
                for frame in 0..observation.frames {
                    for state in 0..states {
                        evaluate(frame, state)?;
                    }
                }
            }
            EmissionPruning::Linear { slope } => {
                let radius = (slope * states as f32) as usize;
                for frame in 0..observation.frames {
                    let center =
                        ((frame as u128 * states as u128) / observation.frames as u128) as usize;
                    for state in
                        center.saturating_sub(radius)..center.saturating_add(radius).min(states)
                    {
                        evaluate(frame, state)?;
                    }
                }
            }
            EmissionPruning::AroundBoundaries { radius } => {
                let mut start = 0;
                for (segment, &boundary) in segmentation.boundaries.iter().enumerate() {
                    let end = (boundary as usize).min(observation.frames - 1);
                    for frame in start..=end {
                        for state in crate::pruning::boundary_states(segment, radius, states - 1) {
                            evaluate(frame, state)?;
                        }
                    }
                    start = boundary as usize;
                }
            }
        }
        Ok(EmissionTable {
            frames: observation.frames,
            states,
            values,
        })
    }
}
