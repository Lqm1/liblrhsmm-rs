//! HMM state and transition posteriors for expectation collection.
use crate::ModelError;
use crate::data::SegmentationData;
use crate::emission::LogEmissionTable;
use crate::geometric::{GeometricSettings, StateScoreTable};
use crate::model::ModelParameters;
use crate::scalar::Scalar;

#[derive(Debug, Clone, PartialEq)]
pub struct StateOccupancyTable<T> {
    frames: usize,
    states: usize,
    values: Vec<T>,
}
/// Original binary32 state occupancy API.
pub type OccupancyTable = StateOccupancyTable<f32>;
impl<T: Scalar> StateOccupancyTable<T> {
    /// Import external time-major state occupancies.
    pub fn from_values(frames: usize, states: usize, values: Vec<T>) -> Result<Self, ModelError> {
        if frames == 0
            || states == 0
            || frames.checked_mul(states) != Some(values.len())
            || values
                .iter()
                .any(|value| !(*value).into().is_finite() || *value < T::ZERO)
        {
            return Err(ModelError(
                "occupancies require positive matching dimensions and finite nonnegative values",
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

fn validate<T: Scalar>(
    forward: &StateScoreTable<T>,
    backward: &StateScoreTable<T>,
    total: T,
    slope: T,
) -> Result<(), ModelError> {
    if forward.frames() != backward.frames() || forward.states() != backward.states() {
        return Err(ModelError("occupancy forward/backward dimensions differ"));
    }
    if !total.into().is_finite() || !slope.into().is_finite() || slope < T::ZERO {
        return Err(ModelError(
            "occupancy total must be finite and pruning slope finite and nonnegative",
        ));
    }
    Ok(())
}
fn band<T: Scalar>(frame: usize, frames: usize, states: usize, slope: T) -> std::ops::Range<usize> {
    let center = ((frame as u128 * states as u128) / frames as u128) as usize;
    let radius = (slope * T::from_f64(states as f64)).into() as usize;
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

impl<T: Scalar> StateScoreTable<T> {
    /// Original HMM occupancy convention excludes the first and final frames.
    pub fn geometric_occupancies(
        &self,
        backward: &Self,
        total: T,
        pruning_slope: T,
    ) -> Result<StateOccupancyTable<T>, ModelError> {
        validate(self, backward, total, pruning_slope)?;
        let frames = self.frames();
        let states = self.states();
        let mut values = filled(self.values().len(), T::ZERO)?;
        for frame in 1..frames.saturating_sub(1) {
            for state in band(frame, frames, states, pruning_slope) {
                let index = frame * states + state;
                values[index] = (self.values()[index] + backward.values()[index] - total)
                    .negative_exponential();
            }
        }
        Ok(StateOccupancyTable {
            frames,
            states,
            values,
        })
    }
}

impl<T: Scalar> ModelParameters<T> {
    pub fn geometric_transition_occupancies(
        &self,
        segmentation: &SegmentationData<T>,
        emissions: &LogEmissionTable<T>,
        forward: &StateScoreTable<T>,
        backward: &StateScoreTable<T>,
        total: T,
        options: GeometricSettings<T>,
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
        if !options.temperature.into().is_finite() || options.temperature < T::ZERO {
            return Err(ModelError("temperature must be finite and nonnegative"));
        }
        let mut stay = Vec::with_capacity(states);
        let mut advance = Vec::with_capacity(states);
        for &state in &segmentation.duration_states {
            let duration = usize::try_from(state)
                .ok()
                .and_then(|index| self.durations.get(index))
                .ok_or(ModelError("duration state is out of range"))?;
            if !duration.mean.into().is_finite() {
                return Err(ModelError("duration mean must be finite"));
            }
            let denominator = duration.mean.into().max(1.01);
            stay.push(
                T::from_f64(1.0 - 1.0 / denominator).into().ln() * options.temperature.into(),
            );
            advance.push(T::from_f64(1.0 / denominator).into().ln() * options.temperature.into());
        }
        let mut values = filled(forward.values().len(), None)?;
        for frame in 1..frames.saturating_sub(1) {
            for state in band(frame, frames, states, options.pruning_slope) {
                let index = frame * states + state;
                let previous = (frame - 1) * states + state;
                let stay_probability = T::from_f64(
                    (forward.values()[previous]
                        + backward.values()[index]
                        + emissions.values()[index])
                        .into()
                        + stay[state]
                        - total.into(),
                )
                .negative_exponential()
                .into() as f32;
                let advance_probability = if state == 0 {
                    1.0
                } else {
                    T::from_f64(
                        (forward.values()[previous - 1]
                            + backward.values()[index]
                            + emissions.values()[index])
                            .into()
                            + advance[state - 1]
                            - total.into(),
                    )
                    .negative_exponential()
                    .into() as f32
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
