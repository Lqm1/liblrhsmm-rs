//! Complete set codecs with cumulative limits and the unchanged binary32 wire.
use super::bytes::{LiblrhsmmRsBytes, byte_input, byte_range};
use super::io::{LiblrhsmmRsReadStream, LiblrhsmmRsWriteStream, Reader, Writer};
use super::{LiblrhsmmRsDecodeLimits, boundary, range, scalar_result};
use crate::{
    ModelError,
    serial::{self, DecodeLimits},
};

mod observation_f32;
pub use observation_f32::*;

mod observation_f64;
pub use observation_f64::*;

mod segmentation_f32;
pub use segmentation_f32::*;

mod segmentation_f64;
pub use segmentation_f64::*;
