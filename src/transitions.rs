//! Incoming jump construction corresponding to upstream `lrh_seg_buildjumps`.
use crate::ModelError;
use crate::data::{JumpParameters, SegmentationData};
use crate::scalar::Scalar;

/// Immutable segmentation and derived incoming transitions.
/// Each incoming group ends with delta -1, preserving the original ordering.
#[derive(Debug)]
pub struct TransitionGraphParameters<'a, T> {
    segmentation: &'a SegmentationData<T>,
    incoming: Vec<Vec<JumpParameters<T>>>,
}
/// Original binary32 transition graph API.
pub type TransitionGraph<'a> = TransitionGraphParameters<'a, f32>;

impl<T: Scalar> SegmentationData<T> {
    /// Add a relative jump before the ordinary next-state transition.
    /// Delta 1 updates that transition's weight instead of appending a duplicate.
    pub fn add_jump(
        &mut self,
        source: usize,
        delta: i32,
        probability: T,
    ) -> Result<(), ModelError> {
        self.validate()?;
        if !probability.into().is_finite() || probability < T::ZERO {
            return Err(ModelError(
                "transition weights must be finite and nonnegative",
            ));
        }
        let jumps = self
            .outgoing
            .get_mut(source)
            .ok_or(ModelError("jump source is out of range"))?;
        let last = jumps.len() - 1;
        if delta == 1 {
            jumps[last].probability = probability;
        } else {
            jumps
                .try_reserve(1)
                .map_err(|_| ModelError("jump allocation failed"))?;
            jumps.insert(last, JumpParameters { delta, probability });
        }
        Ok(())
    }

    /// Derive incoming transitions without caching mutable parameter pointers.
    /// Jumps outside the sequence are excluded from incoming groups, as upstream.
    pub fn build_transitions(&self) -> Result<TransitionGraphParameters<'_, T>, ModelError> {
        self.validate()?;
        let states = self.boundaries.len();
        if states > i32::MAX as usize {
            return Err(ModelError("state count exceeds the legacy format"));
        }
        let mut incoming = vec![
            vec![JumpParameters {
                delta: -1,
                probability: T::ONE
            }];
            states
        ];
        for (source, outgoing) in self.outgoing.iter().enumerate() {
            for jump in outgoing {
                if !jump.probability.into().is_finite() || jump.probability < T::ZERO {
                    return Err(ModelError(
                        "transition weights must be finite and nonnegative",
                    ));
                }
                let destination = source as i64 + i64::from(jump.delta);
                if destination < 0 || destination >= states as i64 {
                    continue;
                }
                let group = &mut incoming[destination as usize];
                let last = group.len() - 1;
                if jump.delta == 1 {
                    group[last].probability = jump.probability;
                } else {
                    group
                        .try_reserve(1)
                        .map_err(|_| ModelError("jump allocation failed"))?;
                    group.insert(
                        last,
                        JumpParameters {
                            delta: -jump.delta,
                            probability: jump.probability,
                        },
                    );
                }
            }
        }
        Ok(TransitionGraphParameters {
            segmentation: self,
            incoming,
        })
    }
}

impl<'a, T> TransitionGraphParameters<'a, T> {
    pub fn segmentation(&self) -> &'a SegmentationData<T> {
        self.segmentation
    }
    pub fn incoming(&self) -> &[Vec<JumpParameters<T>>] {
        &self.incoming
    }
    pub fn outgoing(&self) -> &[Vec<JumpParameters<T>>] {
        &self.segmentation.outgoing
    }
}
