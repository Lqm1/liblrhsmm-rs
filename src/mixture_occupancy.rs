//! Component occupancy calculations corresponding to `lrh_mixocp*`.
use crate::ModelError;
use crate::data::{ObservationData, SegmentationData};
use crate::model::ModelParameters;
use crate::occupancy::StateOccupancyTable;
use crate::probability::compatibility;
use crate::scalar::Scalar;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MixtureOccupancySettings<T> {
    Hsmm { state_radius: T },
    Geometric { pruning_slope: T },
}
/// Original binary32 component occupancy settings.
pub type MixtureOccupancyMode = MixtureOccupancySettings<f32>;

/// Each calculated frame/state pair contains one value per mixture component.
/// Uncomputed pairs retain `None`, corresponding to the original null slices.
#[derive(Debug, Clone, PartialEq)]
pub struct MixtureOccupancies {
    frames: usize,
    states: usize,
    values: Vec<Option<Vec<f32>>>,
}
impl MixtureOccupancies {
    pub fn frames(&self) -> usize {
        self.frames
    }
    pub fn states(&self) -> usize {
        self.states
    }
    pub fn values(&self) -> &[Option<Vec<f32>>] {
        &self.values
    }
    pub fn get(&self, frame: usize, state: usize) -> Option<&[f32]> {
        if frame >= self.frames || state >= self.states {
            return None;
        }
        self.values[frame * self.states + state].as_deref()
    }
}

impl<T: Scalar> ModelParameters<T> {
    pub fn mixture_occupancies(
        &self,
        observation: &ObservationData<T>,
        segmentation: &SegmentationData<T>,
        occupancy: &StateOccupancyTable<T>,
        stream_index: usize,
        temperature: T,
        mode: MixtureOccupancySettings<T>,
    ) -> Result<MixtureOccupancies, ModelError> {
        observation.validate()?;
        segmentation.validate()?;
        let frames = observation.frames;
        let states = segmentation.boundaries.len();
        if frames != occupancy.frames()
            || states != occupancy.states()
            || self.streams.len() != observation.streams.len()
            || self.streams.len() != segmentation.output_states.len()
        {
            return Err(ModelError(
                "mixture occupancy dimensions or stream counts differ",
            ));
        }
        if !temperature.into().is_finite() || temperature < T::ZERO {
            return Err(ModelError("temperature must be finite and nonnegative"));
        }
        match mode {
            MixtureOccupancySettings::Hsmm { state_radius } => {
                crate::pruning::validate_boundary_radius(state_radius)?;
                let mut previous = 0;
                for &boundary in &segmentation.boundaries {
                    if boundary < previous || boundary < 0 || boundary as usize > frames {
                        return Err(ModelError(
                            "mixture occupancy boundaries must be ordered within the observation",
                        ));
                    }
                    previous = boundary;
                }
                if previous as usize != frames {
                    return Err(ModelError("final boundary must match the frame count"));
                }
            }
            MixtureOccupancySettings::Geometric { pruning_slope }
                if !pruning_slope.into().is_finite() || pruning_slope < T::ZERO =>
            {
                return Err(ModelError("pruning slope must be finite and nonnegative"));
            }
            _ => {}
        }
        let stream = self
            .streams
            .get(stream_index)
            .ok_or(ModelError("mixture occupancy stream is out of range"))?;
        let mapping = &segmentation.output_states[stream_index];
        for &state in mapping {
            if state < 0 || state as usize >= stream.mixtures.len() {
                return Err(ModelError(
                    "mixture occupancy emission state is out of range",
                ));
            }
        }
        let mut prepared = Vec::with_capacity(stream.mixtures.len());
        for distribution in &stream.mixtures {
            if distribution.dimensions != observation.streams[stream_index].dimensions {
                return Err(ModelError("emission and observation dimensions differ"));
            }
            prepared.push(distribution.prepare()?);
        }
        let mut values = Vec::new();
        values
            .try_reserve_exact(occupancy.values().len())
            .map_err(|_| ModelError("mixture occupancy allocation failed"))?;
        values.resize_with(occupancy.values().len(), || None);
        let geometric = matches!(mode, MixtureOccupancySettings::Geometric { .. });
        let mut evaluate = |frame: usize, state: usize| -> Result<(), ModelError> {
            let emission = mapping[state] as usize;
            let features = observation
                .frame(stream_index, frame)
                .ok_or(ModelError("invalid observation frame"))?;
            let offset = T::from_f64(f64::from(compatibility::ln(
                occupancy.values()[frame * states + state].into() as f32,
            ))) - prepared[emission].log_probability(features, temperature)?;
            let mut components = Vec::new();
            components
                .try_reserve_exact(stream.mixtures[emission].weights.len())
                .map_err(|_| ModelError("component occupancy allocation failed"))?;
            for (component, &weight) in stream.mixtures[emission].weights.iter().enumerate() {
                let log_probability = offset
                    + prepared[emission].component_log_probability(features, component)?
                        * temperature;
                let probability = weight
                    * if geometric {
                        log_probability.negative_exponential()
                    } else {
                        log_probability.exponential()
                    };
                let probability = probability.into() as f32;
                components.push(if probability.is_nan() {
                    0.0
                } else {
                    probability
                });
            }
            values[frame * states + state] = Some(components);
            Ok(())
        };
        match mode {
            MixtureOccupancySettings::Geometric { pruning_slope } => {
                let radius = (pruning_slope * T::from_f64(states as f64)).into() as usize;
                for frame in 1..frames.saturating_sub(1) {
                    let center = ((frame as u128 * states as u128) / frames as u128) as usize;
                    for state in
                        center.saturating_sub(radius)..center.saturating_add(radius).min(states)
                    {
                        evaluate(frame, state)?;
                    }
                }
            }
            MixtureOccupancySettings::Hsmm { state_radius } => {
                let mut start = 0;
                for (segment, &boundary) in segmentation.boundaries.iter().enumerate() {
                    for frame in start..=(boundary as usize).min(frames - 1) {
                        for state in
                            crate::pruning::boundary_states(segment, state_radius, states - 1)
                        {
                            evaluate(frame, state)?;
                        }
                    }
                    start = boundary as usize;
                }
            }
        }
        Ok(MixtureOccupancies {
            frames,
            states,
            values,
        })
    }
}
