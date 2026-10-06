//! Left-to-right hidden semi-Markov models compatible with liblrhsmm.
//!
//! Inference, training, and remaining upstream APIs are under development.
#![forbid(unsafe_code)]

pub mod data;
pub mod emission;
pub mod model;
mod model_operations;
pub mod probability;
pub mod serial;

pub use data::{Jump, Observation, ObservationStream, Segmentation};
pub use emission::{EmissionPruning, EmissionTable};
pub use model::{Duration, GaussianMixture, Model, ModelError, Stream};
pub use probability::PreparedGaussian;
