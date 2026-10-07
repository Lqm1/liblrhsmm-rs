//! Consecutive-state occupancy and its normalized frame-wise marginal.
use crate::ModelError;
use crate::data::SegmentationData;
use crate::emission::LogEmissionTable;
use crate::hsmm::{HsmmScoreTables, HsmmSettings};
use crate::model::ModelParameters;
use crate::occupancy::StateOccupancyTable;
use crate::probability::compatibility;
use crate::scalar::Scalar;

#[derive(Debug, Clone, PartialEq)]
pub struct DurationPosterior {
    pub minimum: usize,
    pub probabilities: Vec<f32>,
}
impl DurationPosterior {
    pub fn probability(&self, duration: usize) -> f32 {
        duration
            .checked_sub(self.minimum)
            .and_then(|index| self.probabilities.get(index))
            .copied()
            .unwrap_or(0.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DurationOccupancies {
    frames: usize,
    states: usize,
    values: Vec<Option<DurationPosterior>>,
}
impl DurationOccupancies {
    pub fn frames(&self) -> usize {
        self.frames
    }
    pub fn states(&self) -> usize {
        self.states
    }
    pub fn values(&self) -> &[Option<DurationPosterior>] {
        &self.values
    }
    pub fn get(&self, frame: usize, state: usize) -> Option<&DurationPosterior> {
        if frame >= self.frames || state >= self.states {
            return None;
        }
        self.values[frame * self.states + state].as_ref()
    }
}

impl<T: Scalar> ModelParameters<T> {
    pub fn duration_occupancies(
        &self,
        segmentation: &SegmentationData<T>,
        emissions: &LogEmissionTable<T>,
        forward: &HsmmScoreTables<T>,
        backward: &HsmmScoreTables<T>,
        total: T,
        options: HsmmSettings<T>,
    ) -> Result<DurationOccupancies, ModelError> {
        let frames = emissions.frames();
        let states = emissions.states();
        for scores in [&forward.begin, &forward.end, &backward.begin, &backward.end] {
            if scores.frames() != frames || scores.states() != states {
                return Err(ModelError("duration occupancy score dimensions differ"));
            }
        }
        if !total.into().is_finite() {
            return Err(ModelError("duration occupancy normalizer must be finite"));
        }
        let durations = self.prepare_hsmm_durations(segmentation, emissions, options, false)?;
        let graph = segmentation.build_transitions()?;
        let mut values = Vec::new();
        values
            .try_reserve_exact(emissions.values().len())
            .map_err(|_| ModelError("duration occupancy allocation failed"))?;
        values.resize_with(emissions.values().len(), || None);
        let mut start = 0;
        for (segment, &boundary) in segmentation.boundaries.iter().enumerate() {
            for frame in start..=(boundary as usize).min(frames - 1) {
                for state in
                    crate::pruning::boundary_states(segment, options.state_radius, states - 1)
                {
                    // The final absorbing state is excluded from duration learning.
                    if state + 1 >= states {
                        continue;
                    }
                    let mut previous = T::from_f64(-1e15);
                    if frame == 0 && state == 0 {
                        previous = T::ZERO;
                    } else if frame > 0 {
                        for jump in &graph.incoming()[state] {
                            if jump.probability == T::ZERO {
                                continue;
                            }
                            let source = state as i64 + i64::from(jump.delta);
                            if source < 0 {
                                continue;
                            }
                            let candidate = T::from_f64(
                                forward.end.values()[(frame - 1) * states + source as usize].into()
                                    + jump.probability.into().ln() * options.temperature.into(),
                            );
                            previous = previous.log_add(candidate);
                        }
                    }
                    let mut probabilities = Vec::new();
                    probabilities
                        .try_reserve_exact(durations[state].len().saturating_sub(1))
                        .map_err(|_| ModelError("duration slice allocation failed"))?;
                    let mut observed = T::ZERO;
                    for (duration, &probability) in durations[state].iter().enumerate().skip(1) {
                        let tail = frame
                            .checked_add(duration - 1)
                            .ok_or(ModelError("duration tail overflows"))?;
                        let next = if tail < frames {
                            observed += emissions.values()[tail * states + state];
                            backward.end.values()[tail * states + state]
                        } else {
                            T::ZERO
                        };
                        let density = observed + next + probability + previous - total;
                        probabilities.push(compatibility::exp(density.into().min(0.0) as f32));
                    }
                    while probabilities.len() > 1
                        && probabilities
                            .last()
                            .is_some_and(|value| f64::from(*value) < 1e-5)
                    {
                        probabilities.pop();
                    }
                    if !probabilities.is_empty() {
                        values[frame * states + state] = Some(DurationPosterior {
                            minimum: 1,
                            probabilities,
                        });
                    }
                }
            }
            start = boundary as usize;
        }
        Ok(DurationOccupancies {
            frames,
            states,
            values,
        })
    }

    pub fn hsmm_state_occupancies(
        &self,
        segmentation: &SegmentationData<T>,
        emissions: &LogEmissionTable<T>,
        duration_occupancies: &DurationOccupancies,
        options: HsmmSettings<T>,
    ) -> Result<StateOccupancyTable<T>, ModelError> {
        let frames = emissions.frames();
        let states = emissions.states();
        if duration_occupancies.frames != frames || duration_occupancies.states != states {
            return Err(ModelError("duration occupancy dimensions differ"));
        }
        let durations = self.prepare_hsmm_durations(segmentation, emissions, options, false)?;
        let mut values = Vec::new();
        values
            .try_reserve_exact(emissions.values().len())
            .map_err(|_| ModelError("state occupancy allocation failed"))?;
        values.resize(emissions.values().len(), T::ZERO);
        let mut start = 0;
        for (segment, &boundary) in segmentation.boundaries.iter().enumerate() {
            for frame in start..=(boundary as usize).min(frames - 1) {
                for state in
                    crate::pruning::boundary_states(segment, options.state_radius, states - 1)
                {
                    let maximum = durations[state].len();
                    let mut sum = T::ZERO;
                    if state == 0 {
                        if let Some(slice) = duration_occupancies.get(0, 0) {
                            for duration in frame + 1
                                ..maximum
                                    .checked_add(frame)
                                    .ok_or(ModelError("duration range overflows"))?
                            {
                                sum += T::from_f64(f64::from(slice.probability(duration)));
                            }
                        }
                    } else {
                        for offset in 0..maximum.min(frame) {
                            if let Some(slice) = duration_occupancies.get(frame - offset, state) {
                                let Some(upper) = slice
                                    .minimum
                                    .checked_add(slice.probabilities.len())
                                    .and_then(|end| end.checked_sub(1))
                                else {
                                    continue;
                                };
                                let lower = slice.minimum.saturating_sub(offset).max(1);
                                let Some(upper) = upper.checked_sub(offset) else {
                                    continue;
                                };
                                for extension in lower..=maximum.saturating_sub(1).min(upper) {
                                    sum += T::from_f64(f64::from(
                                        slice.probability(offset + extension),
                                    ));
                                }
                            }
                        }
                    }
                    values[frame * states + state] = sum;
                }
            }
            start = boundary as usize;
        }
        for row in values.chunks_exact_mut(states) {
            let sum = row.iter().copied().fold(T::ZERO, |sum, value| sum + value);
            for value in row {
                *value = T::from_f64(
                    (*value).into() / (sum.into() + crate::numerics::OCCUPANCY_EPSILON),
                );
            }
        }
        StateOccupancyTable::from_values(frames, states, values)
    }
}
