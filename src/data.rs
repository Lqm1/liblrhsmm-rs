//! Owned observations and segmentation. Reference: upstream `data.h`.
use crate::ModelError;

/// One stream of time-major acoustic feature vectors.
#[derive(Debug, Clone, PartialEq)]
pub struct ObservationStream {
    pub dimensions: usize,
    pub values: Vec<f32>,
}

/// Acoustic feature streams with a shared frame count.
#[derive(Debug, Clone, PartialEq)]
pub struct Observation {
    pub frames: usize,
    pub streams: Vec<ObservationStream>,
}

impl Observation {
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
    pub fn frame(&self, stream: usize, time: usize) -> Option<&[f32]> {
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
pub struct Jump {
    pub delta: i32,
    pub probability: f32,
}

/// State sequence and right boundaries, with per-stream emission-state indices.
#[derive(Debug, Clone, PartialEq)]
pub struct Segmentation {
    pub boundaries: Vec<i32>,
    pub duration_states: Vec<i32>,
    pub output_states: Vec<Vec<i32>>,
    /// Each group ends in the ordinary next-state transition, delta 1.
    pub outgoing: Vec<Vec<Jump>>,
}

impl Segmentation {
    /// Create an unaligned sequence with ordinary next-state transitions.
    pub fn new(streams: usize, segments: usize) -> Self {
        Self {
            boundaries: vec![0; segments],
            duration_states: vec![0; segments],
            output_states: vec![vec![0; segments]; streams],
            outgoing: vec![
                vec![Jump {
                    delta: 1,
                    probability: 1.0
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
            if jumps.last().is_none_or(|jump| jump.delta != 1)
                || jumps[..jumps.len() - 1].iter().any(|jump| jump.delta == 1)
            {
                return Err(ModelError(
                    "outgoing jumps must terminate with a single delta-1 transition",
                ));
            }
        }
        Ok(())
    }
}
