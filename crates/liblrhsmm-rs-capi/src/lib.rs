//! C ABI for liblrhsmm-rs.
#![deny(unsafe_op_in_unsafe_fn)]
use liblrhsmm_rs::*;
pub mod c_api;
pub use c_api::*;
