//! Optional complete browser and Node.js numerical, model and training interfaces.
//!
//! Arrays are copied across the boundary. Validation errors become JavaScript
//! exceptions; WebAssembly panic aborts are not recoverable.
mod numerics;
pub use numerics::*;
mod duration;
pub use duration::*;
mod gaussian;
pub use gaussian::*;
mod containers;
pub use containers::*;
mod container_statistics;
pub use container_statistics::*;
mod data;
pub use data::*;
mod datasets;
pub use datasets::*;
#[cfg(feature = "serialization")]
mod set_codec;
#[cfg(feature = "serialization")]
pub use set_codec::*;
#[cfg(feature = "serialization")]
mod data_codec;
#[cfg(feature = "serialization")]
pub use data_codec::*;
#[cfg(feature = "serialization")]
mod container_codec;
#[cfg(feature = "serialization")]
pub use container_codec::*;
