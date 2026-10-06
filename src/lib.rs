//! Left-to-right hidden semi-Markov models compatible with liblrhsmm.
//!
//! Inference, training, and remaining upstream APIs are under development.
#![forbid(unsafe_code)]

pub mod data;
pub mod duration_occupancy;
pub mod emission;
pub mod geometric;
pub mod hsmm;
pub mod initialization;
pub mod mixture_occupancy;
pub mod model;
mod model_operations;
pub mod numerics;
pub mod occupancy;
pub mod probability;
pub mod serial;
pub mod statistics;
pub mod transitions;
pub mod update;

pub use data::{Jump, Observation, ObservationStream, Segmentation};
pub use duration_occupancy::{DurationOccupancies, DurationPosterior};
pub use emission::{EmissionPruning, EmissionTable};
pub use geometric::{GeometricAlignment, GeometricOptions, StateScores};
pub use hsmm::{HsmmAlignment, HsmmOptions, HsmmScores, StateOccurrence};
pub use mixture_occupancy::{MixtureOccupancies, MixtureOccupancyMode};
pub use model::{Duration, GaussianMixture, Model, ModelError, Stream};
pub use occupancy::{OccupancyTable, TransitionOccupancies, TransitionPosterior};
pub use probability::PreparedGaussian;
pub use statistics::{DurationStatistics, GaussianStatistics, ModelStatistics, StreamStatistics};
pub use transitions::TransitionGraph;
pub use update::{DurationMode, UpdateOptions};
