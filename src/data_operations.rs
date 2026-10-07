//! Aligned dataset partitioning and Viterbi path state remapping.
use crate::data::{ObservationData, SegmentationData};
use crate::scalar::Scalar;
use crate::{ModelError, StateOccurrence};

/// Paired observations and their alignments, corresponding to `lrh_dataset`.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct AlignedDataset<T> {
    pub observations: Vec<ObservationData<T>>,
    pub segmentations: Vec<SegmentationData<T>>,
}
/// Original binary32 paired dataset.
pub type Dataset = AlignedDataset<f32>;

/// Slots for incremental construction of an observation or segmentation set.
/// Use the typed aliases to build a validated, fully populated sample vector.
#[derive(Debug, Clone, PartialEq)]
pub struct SampleSetBuilder<T> {
    pub samples: Vec<Option<T>>,
}

pub type ObservationSetBuilder = SampleSetBuilder<ObservationData<f32>>;
pub type SegmentationSetBuilder = SampleSetBuilder<SegmentationData<f32>>;

impl<T> SampleSetBuilder<T> {
    /// Allocate empty slots, preserving the original sample ordering.
    pub fn new(samples: usize) -> Result<Self, ModelError> {
        if samples > i32::MAX as usize {
            return Err(ModelError("sample count exceeds the legacy format"));
        }
        let mut slots = Vec::new();
        slots
            .try_reserve_exact(samples)
            .map_err(|_| ModelError("sample set allocation failed"))?;
        slots.resize_with(samples, || None);
        Ok(Self { samples: slots })
    }

    fn finish(self, validate: impl Fn(&T) -> Result<(), ModelError>) -> Result<Vec<T>, ModelError> {
        self.samples
            .into_iter()
            .map(|slot| {
                let sample = slot.ok_or(ModelError("sample set contains an unfilled slot"))?;
                validate(&sample)?;
                Ok(sample)
            })
            .collect()
    }
}

impl<T: Scalar> SampleSetBuilder<ObservationData<T>> {
    pub fn build(self) -> Result<Vec<ObservationData<T>>, ModelError> {
        self.finish(ObservationData::validate)
    }
}

impl<T: Scalar> SampleSetBuilder<SegmentationData<T>> {
    pub fn build(self) -> Result<Vec<SegmentationData<T>>, ModelError> {
        self.finish(SegmentationData::validate)
    }
}

impl<T: Scalar> SegmentationData<T> {
    /// Build an ordinary next-state sequence from Viterbi occurrences.
    /// Repeated, skipped, and backward state references are permitted.
    /// Boundaries come from the path; jump probabilities are reset as upstream.
    pub fn resegment(&self, occurrences: &[StateOccurrence]) -> Result<Self, ModelError> {
        self.validate()?;
        if occurrences.len() > i32::MAX as usize {
            return Err(ModelError(
                "resegmentation length exceeds the legacy format",
            ));
        }
        let mut previous = 0;
        for occurrence in occurrences {
            if occurrence.state >= self.boundaries.len()
                || occurrence.end < previous
                || occurrence.end > i32::MAX as usize
            {
                return Err(ModelError(
                    "resegmentation has invalid states or boundaries",
                ));
            }
            previous = occurrence.end;
        }
        let mut result = Self::new(self.output_states.len(), occurrences.len());
        for (index, occurrence) in occurrences.iter().enumerate() {
            result.boundaries[index] = occurrence.end as i32;
            result.duration_states[index] = self.duration_states[occurrence.state];
            for (stream, states) in self.output_states.iter().enumerate() {
                result.output_states[stream][index] = states[occurrence.state];
            }
        }
        Ok(result)
    }
}

/// Split each aligned sample into groups of at most `segments_per_unit` states.
/// Includes the final short group and all frames, without combining samples.
/// New groups use ordinary next-state transitions, as in `lrh_regroup_data`.
pub fn regroup_data(
    observations: &[ObservationData<f32>],
    segmentations: &[SegmentationData<f32>],
    segments_per_unit: usize,
) -> Result<Dataset, ModelError> {
    regroup_with_precision(observations, segmentations, segments_per_unit)
}

/// Partition an aligned dataset without narrowing its selected scalar precision.
pub fn regroup_with_precision<T: Scalar>(
    observations: &[ObservationData<T>],
    segmentations: &[SegmentationData<T>],
    segments_per_unit: usize,
) -> Result<AlignedDataset<T>, ModelError> {
    if segments_per_unit == 0 || observations.len() != segmentations.len() {
        return Err(ModelError(
            "regrouping requires a positive unit size and matching sample counts",
        ));
    }
    let mut count = 0usize;
    for (observation, segmentation) in observations.iter().zip(segmentations) {
        observation.validate()?;
        segmentation.validate()?;
        if observation.streams.len() != segmentation.output_states.len() {
            return Err(ModelError("regrouping stream counts differ"));
        }
        let mut previous = 0;
        for &boundary in &segmentation.boundaries {
            if boundary < previous || boundary as usize > observation.frames {
                return Err(ModelError(
                    "regrouping boundaries are unordered or out of range",
                ));
            }
            previous = boundary;
        }
        if previous as usize != observation.frames {
            return Err(ModelError(
                "regrouping final boundary must match frame count",
            ));
        }
        count = count
            .checked_add(segmentation.boundaries.len().div_ceil(segments_per_unit))
            .ok_or(ModelError("regrouped sample count overflows"))?;
    }
    let mut result = AlignedDataset::default();
    result
        .observations
        .try_reserve_exact(count)
        .map_err(|_| ModelError("dataset allocation failed"))?;
    result
        .segmentations
        .try_reserve_exact(count)
        .map_err(|_| ModelError("dataset allocation failed"))?;
    for (observation, segmentation) in observations.iter().zip(segmentations) {
        let dimensions: Vec<_> = observation
            .streams
            .iter()
            .map(|stream| stream.dimensions)
            .collect();
        for first in (0..segmentation.boundaries.len()).step_by(segments_per_unit) {
            let end = first
                .saturating_add(segments_per_unit)
                .min(segmentation.boundaries.len());
            let frame_begin = if first == 0 {
                0
            } else {
                segmentation.boundaries[first - 1] as usize
            };
            let frame_end = segmentation.boundaries[end - 1] as usize;
            let mut regrouped = ObservationData::new(frame_end - frame_begin, &dimensions)?;
            for (destination, source) in regrouped.streams.iter_mut().zip(&observation.streams) {
                destination.values.copy_from_slice(
                    &source.values[frame_begin * source.dimensions..frame_end * source.dimensions],
                );
            }
            let mut states = SegmentationData::new(dimensions.len(), end - first);
            for index in first..end {
                states.boundaries[index - first] =
                    segmentation.boundaries[index] - frame_begin as i32;
                states.duration_states[index - first] = segmentation.duration_states[index];
                for stream in 0..dimensions.len() {
                    states.output_states[stream][index - first] =
                        segmentation.output_states[stream][index];
                }
            }
            result.observations.push(regrouped);
            result.segmentations.push(states);
        }
    }
    Ok(result)
}
