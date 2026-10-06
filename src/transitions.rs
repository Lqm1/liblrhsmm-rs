//! Incoming jump construction corresponding to upstream `lrh_seg_buildjumps`.
use crate::{Jump, ModelError, Segmentation};

/// Immutable segmentation and derived incoming transitions.
/// Each incoming group ends with delta -1, preserving the original ordering.
#[derive(Debug)]
pub struct TransitionGraph<'a> {
    segmentation: &'a Segmentation,
    incoming: Vec<Vec<Jump>>,
}

impl Segmentation {
    /// Add a relative jump before the ordinary next-state transition.
    /// Delta 1 updates that transition's weight instead of appending a duplicate.
    pub fn add_jump(
        &mut self,
        source: usize,
        delta: i32,
        probability: f32,
    ) -> Result<(), ModelError> {
        self.validate()?;
        if !probability.is_finite() || probability < 0.0 {
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
            jumps.insert(last, Jump { delta, probability });
        }
        Ok(())
    }

    /// Derive incoming transitions without caching mutable parameter pointers.
    /// Jumps outside the sequence are excluded from incoming groups, as upstream.
    pub fn build_transitions(&self) -> Result<TransitionGraph<'_>, ModelError> {
        self.validate()?;
        let states = self.boundaries.len();
        if states > i32::MAX as usize {
            return Err(ModelError("state count exceeds the legacy format"));
        }
        let mut incoming = vec![
            vec![Jump {
                delta: -1,
                probability: 1.0
            }];
            states
        ];
        for (source, outgoing) in self.outgoing.iter().enumerate() {
            for jump in outgoing {
                if !jump.probability.is_finite() || jump.probability < 0.0 {
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
                        Jump {
                            delta: -jump.delta,
                            probability: jump.probability,
                        },
                    );
                }
            }
        }
        Ok(TransitionGraph {
            segmentation: self,
            incoming,
        })
    }
}

impl<'a> TransitionGraph<'a> {
    pub fn segmentation(&self) -> &'a Segmentation {
        self.segmentation
    }
    pub fn incoming(&self) -> &[Vec<Jump>] {
        &self.incoming
    }
    pub fn outgoing(&self) -> &[Vec<Jump>] {
        &self.segmentation.outgoing
    }
}
