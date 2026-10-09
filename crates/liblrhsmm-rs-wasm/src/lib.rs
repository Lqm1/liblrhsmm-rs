//! Browser and Node.js WebAssembly bindings for liblrhsmm-rs.
#![forbid(unsafe_code)]
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use liblrhsmm_rs::*;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub mod wasm_api;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub use wasm_api::*;
