//! Left-to-right hidden semi-Markov models compatible with liblrhsmm.
//!
//! Inference, training, and remaining upstream APIs are under development.
#![forbid(unsafe_code)]

pub mod data;
pub mod model;
pub mod serial;

pub use data::{Jump, Observation, ObservationStream, Segmentation};
pub use model::{Duration, GaussianMixture, Model, ModelError, Stream};
