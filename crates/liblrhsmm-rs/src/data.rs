//! Owned observations and segmentation. Reference: upstream `data.h`.
use crate::ModelError;
use crate::scalar::Scalar;

/// One stream of time-major acoustic feature vectors.
#[derive(Debug, Clone, PartialEq)]
pub struct ObservationStreamData<T> {
    pub dimensions: usize,
    pub values: Vec<T>,
}
/// Original binary32 observation stream API.
pub type ObservationStream = ObservationStreamData<f32>;

/// Acoustic feature streams with a shared frame count.
#[derive(Debug, Clone, PartialEq)]
pub struct ObservationData<T> {
    pub frames: usize,
    pub streams: Vec<ObservationStreamData<T>>,
}
/// Original binary32 observation API.
pub type Observation = ObservationData<f32>;

impl<T: Scalar> ObservationData<T> {
    /// Allocate zeroed time-major streams, corresponding to `lrh_create_observ`.
    pub fn new(frames: usize, dimensions: &[usize]) -> Result<Self, ModelError> {
        if frames > i32::MAX as usize || dimensions.is_empty() {
            return Err(ModelError(
                "observation requires streams and a frame count fitting i32",
            ));
        }
        let streams = dimensions
            .iter()
            .map(|&dimension| {
                if dimension == 0 || dimension > i32::MAX as usize {
                    return Err(ModelError(
                        "observation dimensions must be positive and fit i32",
                    ));
                }
                let length = frames
                    .checked_mul(dimension)
                    .ok_or(ModelError("observation allocation overflows"))?;
                let mut values = Vec::new();
                values
                    .try_reserve_exact(length)
                    .map_err(|_| ModelError("observation allocation failed"))?;
                values.resize(length, T::ZERO);
                Ok(ObservationStreamData {
                    dimensions: dimension,
                    values,
                })
            })
            .collect::<Result<Vec<_>, ModelError>>()?;
        Ok(Self { frames, streams })
    }

    pub fn validate(&self) -> Result<(), ModelError> {
        if self.frames > i32::MAX as usize {
            return Err(ModelError("frame count exceeds the legacy format"));
        }
        if self.streams.is_empty() {
            return Err(ModelError("an observation requires at least one stream"));
        }
        for stream in &self.streams {
            if stream.dimensions == 0 || stream.dimensions > i32::MAX as usize {
                return Err(ModelError(
                    "observation dimensions must be positive and fit i32",
                ));
            }
            let length = stream
                .dimensions
                .checked_mul(self.frames)
                .ok_or(ModelError("observation dimensions overflow"))?;
            if stream.values.len() != length {
                return Err(ModelError("observation storage does not match dimensions"));
            }
        }
        Ok(())
    }

    /// Return a complete feature vector without exposing unchecked indexing.
    pub fn frame(&self, stream: usize, time: usize) -> Option<&[T]> {
        if time >= self.frames {
            return None;
        }
        let stream = self.streams.get(stream)?;
        let start = time.checked_mul(stream.dimensions)?;
        stream
            .values
            .get(start..start.checked_add(stream.dimensions)?)
    }
}

/// Relative transition to another segment, with its probability.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JumpParameters<T> {
    pub delta: i32,
    pub probability: T,
}
/// Original binary32 state-jump API.
pub type Jump = JumpParameters<f32>;

/// State sequence and right boundaries, with per-stream emission-state indices.
#[derive(Debug, Clone, PartialEq)]
pub struct SegmentationData<T> {
    pub boundaries: Vec<i32>,
    pub duration_states: Vec<i32>,
    pub output_states: Vec<Vec<i32>>,
    /// Each group ends in the ordinary next-state transition, delta 1.
    pub outgoing: Vec<Vec<JumpParameters<T>>>,
}
/// Original binary32 segmentation API.
pub type Segmentation = SegmentationData<f32>;

/// Validate that a jump group ends with exactly one ordinary next-state jump.
pub fn validate_jump_group<T>(jumps: &[JumpParameters<T>]) -> Result<(), ModelError> {
    if jumps.last().is_none_or(|jump| jump.delta != 1)
        || jumps[..jumps.len() - 1].iter().any(|jump| jump.delta == 1)
    {
        return Err(ModelError(
            "outgoing jumps must terminate with a single delta-1 transition",
        ));
    }
    Ok(())
}

impl<T: Scalar> SegmentationData<T> {
    /// Create an unaligned sequence with ordinary next-state transitions.
    pub fn new(streams: usize, segments: usize) -> Self {
        Self {
            boundaries: vec![0; segments],
            duration_states: vec![0; segments],
            output_states: vec![vec![0; segments]; streams],
            outgoing: vec![
                vec![JumpParameters {
                    delta: 1,
                    probability: T::ONE
                }];
                segments
            ],
        }
    }

    pub fn validate(&self) -> Result<(), ModelError> {
        let segments = self.boundaries.len();
        if self.duration_states.len() != segments
            || self.outgoing.len() != segments
            || self
                .output_states
                .iter()
                .any(|stream| stream.len() != segments)
        {
            return Err(ModelError("segmentation arrays have inconsistent lengths"));
        }
        for jumps in &self.outgoing {
            validate_jump_group(jumps)?;
        }
        Ok(())
    }
}
