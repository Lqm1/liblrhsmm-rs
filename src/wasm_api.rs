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
