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

/// Immutable segmentation and incoming transitions borrowed from a graph owner.
/// Copying a view copies only references, never parameters or cached jumps.
#[derive(Debug)]
pub struct TransitionGraphView<'a, T> {
    segmentation: &'a SegmentationData<T>,
    incoming: &'a [Vec<JumpParameters<T>>],
}

/// Internal selection between legacy parameters and an existing immutable cache.
pub(crate) enum TransitionGraphInput<'a, T> {
    Segmentation(&'a SegmentationData<T>),
    Cached(TransitionGraphView<'a, T>),
}

impl<'a, T> TransitionGraphInput<'a, T> {
    pub(crate) fn segmentation(&self) -> &'a SegmentationData<T> {
        match self {
            Self::Segmentation(segmentation) => segmentation,
            Self::Cached(graph) => graph.segmentation(),
        }
    }

    pub(crate) fn cached(&self) -> Option<TransitionGraphView<'a, T>> {
        match self {
            Self::Segmentation(_) => None,
            Self::Cached(graph) => Some(*graph),
        }
    }
}

impl<T> Copy for TransitionGraphView<'_, T> {}
impl<T> Clone for TransitionGraphView<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

/// Independent immutable segmentation and its matching incoming transition cache.
/// The snapshot remains valid after the editable source is changed or dropped.
/// Views borrow both parameters and cached jumps without rebuilding either.
///
/// A live view prevents moving its owner into editable parameters:
///
/// ```compile_fail
/// use liblrhsmm_rs::Segmentation;
/// let owned = Segmentation::new(0, 2).build_transitions_owned().unwrap();
/// let view = owned.as_view();
/// let editable = owned.into_segmentation();
/// assert_eq!(view.incoming().len(), 2);
/// ```
#[derive(Debug, Clone)]
pub struct OwnedTransitionGraphParameters<T> {
    segmentation: SegmentationData<T>,
    incoming: Vec<Vec<JumpParameters<T>>>,
}

/// Binary32 owned transition graph.
pub type OwnedTransitionGraph = OwnedTransitionGraphParameters<f32>;

impl<T: Scalar> SegmentationData<T> {
    /// Consume a segmentation into an independent graph, deriving incoming jumps
    /// once and moving the parameters and cache without cloning their vectors.
    /// Clone first to retain an editable source segmentation.
    pub fn build_transitions_owned(self) -> Result<OwnedTransitionGraphParameters<T>, ModelError> {
        let incoming = self.build_transitions()?.incoming;
        Ok(OwnedTransitionGraphParameters {
            segmentation: self,
            incoming,
        })
    }

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
    /// Borrow complete parameter and cache storage without allocation.
    pub fn as_view(&self) -> TransitionGraphView<'_, T> {
        TransitionGraphView {
            segmentation: self.segmentation,
            incoming: &self.incoming,
        }
    }

    /// Copy parameters and already-derived jumps into an independent snapshot.
    /// Incoming transitions are not recomputed.
    pub fn to_owned(&self) -> OwnedTransitionGraphParameters<T>
    where
        T: Clone,
    {
        OwnedTransitionGraphParameters {
            segmentation: self.segmentation.clone(),
            incoming: self.incoming.clone(),
        }
    }

    /// Copy borrowed parameters and move the existing derived cache into a snapshot.
    pub fn into_owned(self) -> OwnedTransitionGraphParameters<T>
    where
        T: Clone,
    {
        OwnedTransitionGraphParameters {
            segmentation: self.segmentation.clone(),
            incoming: self.incoming,
        }
    }

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

impl<'a, T> TransitionGraphView<'a, T> {
    pub fn segmentation(&self) -> &'a SegmentationData<T> {
        self.segmentation
    }

    pub fn incoming(&self) -> &'a [Vec<JumpParameters<T>>] {
        self.incoming
    }

    pub fn outgoing(&self) -> &'a [Vec<JumpParameters<T>>] {
        &self.segmentation.outgoing
    }

    /// Copy parameters and cached transitions without rebuilding the topology.
    pub fn to_owned(&self) -> OwnedTransitionGraphParameters<T>
    where
        T: Clone,
    {
        OwnedTransitionGraphParameters {
            segmentation: self.segmentation.clone(),
            incoming: self.incoming.to_vec(),
        }
    }
}

impl<T> OwnedTransitionGraphParameters<T> {
    pub fn segmentation(&self) -> &SegmentationData<T> {
        &self.segmentation
    }

    pub fn incoming(&self) -> &[Vec<JumpParameters<T>>] {
        &self.incoming
    }

    pub fn outgoing(&self) -> &[Vec<JumpParameters<T>>] {
        &self.segmentation.outgoing
    }

    /// Borrow complete immutable parameters and derived jumps without allocation.
    pub fn as_view(&self) -> TransitionGraphView<'_, T> {
        TransitionGraphView {
            segmentation: &self.segmentation,
            incoming: &self.incoming,
        }
    }

    /// Discard derived jumps and recover editable parameters without copying vectors.
    pub fn into_segmentation(self) -> SegmentationData<T> {
        self.segmentation
    }
}
