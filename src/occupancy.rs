//! HMM state and transition posteriors for expectation collection.
use crate::{
    EmissionTable, GeometricOptions, Model, ModelError, Segmentation, StateScores,
    numerics::negative_exponential,
};

#[derive(Debug, Clone, PartialEq)]
pub struct OccupancyTable {
    frames: usize,
    states: usize,
    values: Vec<f32>,
}
impl OccupancyTable {
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
        Some(self.values[frame * self.states + state])
    }
}

/// Probabilities of arriving from the same state or the previous state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransitionPosterior {
    pub stay: f32,
    pub advance: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransitionOccupancies {
    frames: usize,
    states: usize,
    values: Vec<Option<TransitionPosterior>>,
}
impl TransitionOccupancies {
    pub fn frames(&self) -> usize {
        self.frames
    }
    pub fn states(&self) -> usize {
        self.states
    }
    pub fn values(&self) -> &[Option<TransitionPosterior>] {
        &self.values
    }
    pub fn get(&self, frame: usize, state: usize) -> Option<TransitionPosterior> {
        if frame >= self.frames || state >= self.states {
            return None;
        }
        self.values[frame * self.states + state]
    }
}

fn validate(
    forward: &StateScores,
    backward: &StateScores,
    total: f32,
    slope: f32,
) -> Result<(), ModelError> {
    if forward.frames() != backward.frames() || forward.states() != backward.states() {
        return Err(ModelError("occupancy forward/backward dimensions differ"));
    }
    if !total.is_finite() || !slope.is_finite() || slope < 0.0 {
        return Err(ModelError(
            "occupancy total must be finite and pruning slope finite and nonnegative",
        ));
    }
    Ok(())
}
fn band(frame: usize, frames: usize, states: usize, slope: f32) -> std::ops::Range<usize> {
    let center = ((frame as u128 * states as u128) / frames as u128) as usize;
    let radius = (slope * states as f32) as usize;
    center.saturating_sub(radius)..center.saturating_add(radius).min(states)
}
fn filled<T: Clone>(length: usize, value: T) -> Result<Vec<T>, ModelError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(length)
        .map_err(|_| ModelError("occupancy allocation failed"))?;
    values.resize(length, value);
    Ok(values)
}

impl StateScores {
    /// Original HMM occupancy convention excludes the first and final frames.
    pub fn geometric_occupancies(
        &self,
        backward: &Self,
        total: f32,
        pruning_slope: f32,
    ) -> Result<OccupancyTable, ModelError> {
        validate(self, backward, total, pruning_slope)?;
        let frames = self.frames();
        let states = self.states();
        let mut values = filled(self.values().len(), 0.0)?;
        for frame in 1..frames.saturating_sub(1) {
            for state in band(frame, frames, states, pruning_slope) {
                let index = frame * states + state;
                values[index] =
                    negative_exponential(self.values()[index] + backward.values()[index] - total);
            }
        }
        Ok(OccupancyTable {
            frames,
            states,
            values,
        })
    }
}

impl Model {
    pub fn geometric_transition_occupancies(
        &self,
        segmentation: &Segmentation,
        emissions: &EmissionTable,
        forward: &StateScores,
        backward: &StateScores,
        total: f32,
        options: GeometricOptions,
    ) -> Result<TransitionOccupancies, ModelError> {
        validate(forward, backward, total, options.pruning_slope)?;
        segmentation.validate()?;
        let frames = forward.frames();
        let states = forward.states();
        if emissions.frames() != frames
            || emissions.states() != states
            || segmentation.duration_states.len() != states
        {
            return Err(ModelError("transition occupancy dimensions differ"));
        }
        if !options.temperature.is_finite() || options.temperature < 0.0 {
            return Err(ModelError("temperature must be finite and nonnegative"));
        }
        let mut stay = Vec::with_capacity(states);
        let mut advance = Vec::with_capacity(states);
        for &state in &segmentation.duration_states {
            let duration = usize::try_from(state)
                .ok()
                .and_then(|index| self.durations.get(index))
                .ok_or(ModelError("duration state is out of range"))?;
            if !duration.mean.is_finite() {
                return Err(ModelError("duration mean must be finite"));
            }
            let denominator = f64::from(duration.mean).max(1.01);
            stay.push(
                f64::from((1.0 - 1.0 / denominator) as f32).ln() * f64::from(options.temperature),
            );
            advance
                .push(f64::from((1.0 / denominator) as f32).ln() * f64::from(options.temperature));
        }
        let mut values = filled(forward.values().len(), None)?;
        for frame in 1..frames.saturating_sub(1) {
            for state in band(frame, frames, states, options.pruning_slope) {
                let index = frame * states + state;
                let previous = (frame - 1) * states + state;
                let stay_probability = negative_exponential(
                    (f64::from(
                        forward.values()[previous]
                            + backward.values()[index]
                            + emissions.values()[index],
                    ) + stay[state]
                        - f64::from(total)) as f32,
                );
                let advance_probability = if state == 0 {
                    1.0
                } else {
                    negative_exponential(
                        (f64::from(
                            forward.values()[previous - 1]
                                + backward.values()[index]
                                + emissions.values()[index],
                        ) + advance[state - 1]
                            - f64::from(total)) as f32,
                    )
                };
                values[index] = Some(TransitionPosterior {
                    stay: stay_probability,
                    advance: advance_probability,
                });
            }
        }
        Ok(TransitionOccupancies {
            frames,
            states,
            values,
        })
    }
}
