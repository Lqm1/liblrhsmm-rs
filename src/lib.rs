//! Left-to-right hidden semi-Markov models compatible with liblrhsmm.
//!
//! Inference, training, and remaining upstream APIs are under development.
#![forbid(unsafe_code)]

pub mod data;
pub mod emission;
pub mod geometric;
pub mod hsmm;
pub mod model;
mod model_operations;
pub mod numerics;
pub mod occupancy;
pub mod probability;
pub mod serial;
pub mod transitions;

pub use data::{Jump, Observation, ObservationStream, Segmentation};
pub use emission::{EmissionPruning, EmissionTable};
pub use geometric::{GeometricAlignment, GeometricOptions, StateScores};
pub use hsmm::{HsmmAlignment, HsmmOptions, HsmmScores, StateOccurrence};
pub use model::{Duration, GaussianMixture, Model, ModelError, Stream};
pub use occupancy::{OccupancyTable, TransitionOccupancies, TransitionPosterior};
pub use probability::PreparedGaussian;
pub use transitions::TransitionGraph;
