//! Left-to-right hidden semi-Markov models compatible with liblrhsmm.
//!
//! Binary32 model, inference, and training APIs preserve the legacy wire format.
//! Both storage precisions use the legacy binary32 wire format when the default
//! `serialization` feature is enabled. Full acceptance and language bindings are
//! under development.
#![deny(unsafe_code)]
#![cfg_attr(not(feature = "c-api"), forbid(unsafe_code))]

#[cfg(feature = "c-api")]
#[allow(unsafe_code)]
pub mod c_api;

pub mod data;
pub mod data_operations;
pub mod duration_occupancy;
pub mod emission;
pub mod estimation;
pub mod expectation;
pub mod generation;
pub mod geometric;
pub mod hsmm;
pub mod initialization;
pub mod memory_pool;
pub mod mixture_occupancy;
pub mod model;
mod model_operations;
pub mod numerics;
pub mod occupancy;
pub mod preparation;
pub mod probability;
mod pruning;
pub mod scalar;
#[cfg(feature = "serialization")]
pub mod serial;
pub mod statistics;
pub mod transitions;
pub mod update;

pub use data::{Jump, Observation, ObservationStream, Segmentation};
pub use data_operations::{
    Dataset, ObservationSetBuilder, SampleSetBuilder, SegmentationSetBuilder, regroup_data,
};
pub use duration_occupancy::{DurationOccupancies, DurationPosterior};
pub use emission::{EmissionProbabilities, EmissionPruning, EmissionTable};
pub use expectation::{GeometricExpectation, HsmmExpectation};
pub use geometric::{GeometricAlignment, GeometricOptions, StateScores};
pub use hsmm::{HsmmAlignment, HsmmOptions, HsmmScores, StateOccurrence};
pub use memory_pool::{MemoryPool, PoolAllocation};
pub use mixture_occupancy::{MixtureOccupancies, MixtureOccupancyMode};
pub use model::{
    Duration, GaussianMixture, Model, ModelBuilder, ModelError, Stream, StreamBuilder, StreamLayout,
};
pub use occupancy::{OccupancyTable, TransitionOccupancies, TransitionPosterior};
pub use preparation::{PreparationOptions, PreparedDuration, PreparedModel};
pub use probability::PreparedGaussian;
pub use statistics::{DurationStatistics, GaussianStatistics, ModelStatistics, StreamStatistics};
pub use transitions::{OwnedTransitionGraph, TransitionGraph, TransitionGraphView};
pub use update::{DurationMode, UpdateOptions};
