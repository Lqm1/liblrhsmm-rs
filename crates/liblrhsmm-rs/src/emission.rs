//! Weighted multistream emission tables and upstream pruning conventions.
use crate::ModelError;
use crate::data::{ObservationData, SegmentationData};
use crate::model::ModelParameters;
use crate::probability::PreparedGaussianParameters;
use crate::scalar::Scalar;

/// Select which frame/state pairs are evaluated.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EmissionPruningOptions<T> {
    /// Evaluate every pair.
    None,
    /// Include neighboring states around each existing segment interval.
    AroundBoundaries { radius: T },
    /// Follow the frame-to-state diagonal, with an exclusive upper state bound.
    Linear { slope: T },
}
/// Original binary32 emission pruning API.
pub type EmissionPruning = EmissionPruningOptions<f32>;

/// Time-major log densities; excluded pairs retain the legacy -1e15 sentinel.
#[derive(Debug, Clone, PartialEq)]
pub struct LogEmissionTable<T> {
    frames: usize,
    states: usize,
    values: Vec<T>,
}
/// Original binary32 log-emission table API.
pub type EmissionTable = LogEmissionTable<f32>;

/// Time-major emission densities, kept separate from log-score input tables.
/// The upstream approximation retains a small positive value for pruned pairs.
#[derive(Debug, Clone, PartialEq)]
pub struct EmissionDensityTable<T> {
    frames: usize,
    states: usize,
    values: Vec<T>,
}
/// Original binary32 emission density API.
pub type EmissionProbabilities = EmissionDensityTable<f32>;

impl<T: Copy> EmissionDensityTable<T> {
    pub fn frames(&self) -> usize {
        self.frames
    }
    pub fn states(&self) -> usize {
        self.states
    }
    pub fn values(&self) -> &[T] {
        &self.values
    }
    pub fn get(&self, frame: usize, state: usize) -> Option<T> {
        if frame >= self.frames || state >= self.states {
            return None;
        }
        self.values.get(frame * self.states + state).copied()
    }
}

impl<T: Scalar> LogEmissionTable<T> {
    /// Convert each log score with the original approximate exponential.
    pub fn probabilities(&self) -> EmissionDensityTable<T> {
        EmissionDensityTable {
            frames: self.frames,
            states: self.states,
            values: self
                .values
                .iter()
                .map(|&value| {
                    T::from_f64(f64::from(crate::probability::compatibility::exp(
                        value.into() as f32,
                    )))
                })
                .collect(),
        }
    }
    /// Accept external time-major log densities for inference.
    pub fn from_log_values(
        frames: usize,
        states: usize,
        values: Vec<T>,
    ) -> Result<Self, ModelError> {
        if frames == 0
            || states == 0
            || frames.checked_mul(states) != Some(values.len())
            || values.iter().any(|value| !(*value).into().is_finite())
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
    pub fn values(&self) -> &[T] {
        &self.values
    }
    pub fn get(&self, frame: usize, state: usize) -> Option<T> {
        if frame >= self.frames || state >= self.states {
            return None;
        }
        self.values.get(frame * self.states + state).copied()
    }
}

impl<T: Scalar> ModelParameters<T> {
    /// Sample non-log densities, corresponding to `lrh_sample_outputprob`.
    pub fn emission_probabilities(
        &self,
        observation: &ObservationData<T>,
        segmentation: &SegmentationData<T>,
        temperature: T,
        pruning: EmissionPruningOptions<T>,
    ) -> Result<EmissionDensityTable<T>, ModelError> {
        Ok(self
            .emission_log_probabilities(observation, segmentation, temperature, pruning)?
            .probabilities())
    }

    /// Prepare and evaluate weighted stream densities for a state sequence.
    /// Pruning variants preserve the original C interval inclusivity.
    pub fn emission_log_probabilities(
        &self,
        observation: &ObservationData<T>,
        segmentation: &SegmentationData<T>,
        temperature: T,
        pruning: EmissionPruningOptions<T>,
    ) -> Result<LogEmissionTable<T>, ModelError> {
        let prepared = self.prepare_emissions()?;
        self.sample_prepared_emissions(&prepared, observation, segmentation, temperature, pruning)
    }

    pub(crate) fn sample_prepared_emissions(
        &self,
        prepared_streams: &[Vec<PreparedGaussianParameters<'_, T>>],
        observation: &ObservationData<T>,
        segmentation: &SegmentationData<T>,
        temperature: T,
        pruning: EmissionPruningOptions<T>,
    ) -> Result<LogEmissionTable<T>, ModelError> {
        self.sample_emission_densities(
            observation,
            segmentation,
            temperature,
            pruning,
            |stream, state, features, temperature| {
                prepared_streams[stream][state].log_probability(features, temperature)
            },
        )
    }

    pub(crate) fn sample_emission_densities(
        &self,
        observation: &ObservationData<T>,
        segmentation: &SegmentationData<T>,
        temperature: T,
        pruning: EmissionPruningOptions<T>,
        emission_density: impl Fn(usize, usize, &[T], T) -> Result<T, ModelError>,
    ) -> Result<LogEmissionTable<T>, ModelError> {
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
        if !temperature.into().is_finite() || temperature < T::ZERO {
            return Err(ModelError("temperature must be finite and nonnegative"));
        }
        match pruning {
            EmissionPruningOptions::AroundBoundaries { radius } => {
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
            EmissionPruningOptions::Linear { slope }
                if !slope.into().is_finite() || slope < T::ZERO =>
            {
                return Err(ModelError(
                    "linear pruning slope must be finite and nonnegative",
                ));
            }
            _ => {}
        }
        for (stream_index, stream) in self.streams.iter().enumerate() {
            if !stream.weight.into().is_finite() || stream.weight < T::ZERO {
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
        values.resize(length, T::from_f64(-1e15));
        let mut evaluate = |frame: usize, state: usize| -> Result<(), ModelError> {
            let mut density = T::ZERO;
            for (stream_index, stream) in self.streams.iter().enumerate() {
                let emission = segmentation.output_states[stream_index][state] as usize;
                let features = observation
                    .frame(stream_index, frame)
                    .ok_or(ModelError("invalid observation frame"))?;
                density += emission_density(stream_index, emission, features, temperature)?
                    * stream.weight;
            }
            values[frame * states + state] = density;
            Ok(())
        };
        match pruning {
            EmissionPruningOptions::None => {
                for frame in 0..observation.frames {
                    for state in 0..states {
                        evaluate(frame, state)?;
                    }
                }
            }
            EmissionPruningOptions::Linear { slope } => {
                let radius = (slope * T::from_f64(states as f64)).into() as usize;
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
            EmissionPruningOptions::AroundBoundaries { radius } => {
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
        Ok(LogEmissionTable {
            frames: observation.frames,
            states,
            values,
        })
    }
}
