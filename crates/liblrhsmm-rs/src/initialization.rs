//! Sufficient statistics from an existing frame alignment.
use crate::ModelError;
use crate::data::{ObservationData, SegmentationData};
use crate::model::ModelParameters;
use crate::scalar::Scalar;
use crate::statistics::ModelSufficientStatistics;

impl<T: Scalar> ModelSufficientStatistics<T> {
    /// Accumulate one aligned utterance, corresponding to `lrh_collect_init`.
    /// Initialization requires single-component emissions, as in upstream.
    /// Invalid input or overflowing statistics leave this accumulator unchanged.
    pub fn collect_initialization(
        &mut self,
        model: &ModelParameters<T>,
        observation: &ObservationData<T>,
        segmentation: &SegmentationData<T>,
    ) -> Result<(), ModelError> {
        self.validate_for(model)?;
        observation.validate()?;
        segmentation.validate()?;
        if observation.streams.len() != self.streams.len()
            || segmentation.output_states.len() != self.streams.len()
        {
            return Err(ModelError("initialization stream counts differ"));
        }
        let mut previous = 0;
        for (segment, &boundary) in segmentation.boundaries.iter().enumerate() {
            if boundary < previous || boundary as usize > observation.frames {
                return Err(ModelError(
                    "initialization boundaries are unordered or out of range",
                ));
            }
            previous = boundary;
            let duration = segmentation.duration_states[segment];
            if duration < 0 || duration as usize >= self.durations.len() {
                return Err(ModelError("initialization duration state is out of range"));
            }
            for (stream, statistics) in self.streams.iter().enumerate() {
                let state = segmentation.output_states[stream][segment];
                let distribution = usize::try_from(state)
                    .ok()
                    .and_then(|state| statistics.mixtures.get(state))
                    .ok_or(ModelError("initialization emission state is out of range"))?;
                if distribution.dimensions != observation.streams[stream].dimensions {
                    return Err(ModelError("initialization feature dimensions differ"));
                }
                if distribution.component_occupancies.len() != 1 {
                    return Err(ModelError(
                        "initialization requires single-component emissions",
                    ));
                }
            }
        }
        if observation
            .streams
            .iter()
            .flat_map(|stream| &stream.values)
            .any(|value| !(*value).into().is_finite())
        {
            return Err(ModelError("initialization observations must be finite"));
        }
        let mut updated = self.clone();
        let mut begin = 0;
        for (segment, &boundary) in segmentation.boundaries.iter().enumerate() {
            let end = boundary as usize;
            let length = T::from_f64((end - begin) as f64);
            let duration = &mut updated.durations[segmentation.duration_states[segment] as usize];
            duration.value_sum += length;
            duration.squared_sum += length * length;
            duration.occupancy += T::ONE;
            for time in begin..end {
                for (stream, statistics) in updated.streams.iter_mut().enumerate() {
                    let distribution = &mut statistics.mixtures
                        [segmentation.output_states[stream][segment] as usize];
                    distribution.state_occupancy += T::ONE;
                    distribution.component_occupancies[0] += T::ONE;
                    let features = observation
                        .frame(stream, time)
                        .expect("validated observation frame");
                    for (dimension, &value) in features.iter().enumerate() {
                        distribution.weighted_sums[dimension] += value;
                        distribution.weighted_squared_sums[dimension] += value * value;
                    }
                }
            }
            begin = end;
        }
        updated.validate_for(model)?;
        *self = updated;
        Ok(())
    }
}
