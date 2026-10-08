//! Source numerical kernels, mutable coefficient access and constant descriptors.
use super::{boundary, range, scalar_result};
use crate::{ModelError, numerics};
use std::panic::{RefUnwindSafe, UnwindSafe};

/// Select the source log-add coefficient table.
pub const LIBLRHSMM_RS_LOG_ADD_TABLE: u32 = 0;
/// Select the source negative-exponential coefficient table.
pub const LIBLRHSMM_RS_EXPONENTIAL_TABLE: u32 = 1;
/// Source constants retain their declared precision; size fields are coefficient counts.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LiblrhsmmRsNumericalConstants {
    pub log_add_count: usize,
    pub exponential_count: usize,
    pub log_add_step: f64,
    pub exponential_step: f64,
    pub occupancy_epsilon: f64,
    pub log_zero_f32: f32,
    pub log_zero_f64: f64,
    pub pi: f64,
}
pub(super) trait Precision: Copy + RefUnwindSafe + UnwindSafe {
    fn get(table: u32, index: usize) -> Option<Self>;
    fn replace(table: u32, index: usize, value: Self) -> Option<Self>;
}
macro_rules! precision {
    ($scalar:ty, $log:ident, $exp:ident) => {
        impl Precision for $scalar {
            fn get(table: u32, index: usize) -> Option<Self> {
                match table {
                    0 => numerics::$log.get(index),
                    1 => numerics::$exp.get(index),
                    _ => None,
                }
            }
            fn replace(table: u32, index: usize, value: Self) -> Option<Self> {
                match table {
                    0 => numerics::$log.replace(index, value),
                    1 => numerics::$exp.replace(index, value),
                    _ => None,
                }
            }
        }
    };
}
precision!(f32, LOG_ADD_COEFFICIENTS, EXPONENTIAL_COEFFICIENTS);
precision!(f64, LOG_ADD_COEFFICIENTS_F64, EXPONENTIAL_COEFFICIENTS_F64);
fn interval(table: u32, offset: usize, count: usize) -> Result<usize, u32> {
    let length = match table {
        0 => numerics::LOG_ADD_TABLE_SIZE,
        1 => numerics::EXPONENTIAL_TABLE_SIZE,
        _ => return Err(3),
    };
    offset
        .checked_add(count)
        .filter(|&end| end <= length)
        .ok_or(2)
}
pub(super) unsafe fn get<T: Precision>(table: u32, index: usize, output: *mut T) -> u32 {
    if let Err(status) = interval(table, index, 1) {
        return status;
    }
    // SAFETY: Independent exclusively writable scalar output; coefficient is read atomically.
    unsafe {
        scalar_result(output, || {
            T::get(table, index).ok_or(ModelError("table index is invalid"))
        })
    }
}
pub(super) unsafe fn replace<T: Precision>(
    table: u32,
    index: usize,
    value: T,
    output: *mut T,
) -> u32 {
    if let Err(status) = interval(table, index, 1) {
        return status;
    }
    // SAFETY: Validate output before changing the atomically stored coefficient.
    unsafe {
        scalar_result(output, || {
            T::replace(table, index, value).ok_or(ModelError("table index is invalid"))
        })
    }
}
pub(super) unsafe fn copy<T: Precision>(
    table: u32,
    offset: usize,
    output: *mut T,
    count: usize,
) -> u32 {
    let end = match interval(table, offset, count) {
        Ok(v) => v,
        Err(e) => return e,
    };
    if let Err(status) = range(output.cast_const(), count) {
        return status;
    }
    let values = match boundary(|| {
        (offset..end)
            .map(|i| T::get(table, i).ok_or(ModelError("table index is invalid")))
            .collect::<Result<Vec<_>, _>>()
    }) {
        Ok(v) => v,
        Err(e) => return e,
    };
    if count != 0 {
        // SAFETY: Checked caller output is exclusively writable and independent of owned staging storage.
        unsafe { std::ptr::copy_nonoverlapping(values.as_ptr(), output, count) };
    }
    0
}
pub(super) unsafe fn constants(output: *mut LiblrhsmmRsNumericalConstants) -> u32 {
    // SAFETY: Independent aligned exclusively writable complete descriptor storage.
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsNumericalConstants {
                log_add_count: numerics::LOG_ADD_TABLE_SIZE,
                exponential_count: numerics::EXPONENTIAL_TABLE_SIZE,
                log_add_step: numerics::LOG_ADD_TABLE_STEP,
                exponential_step: numerics::EXPONENTIAL_TABLE_STEP,
                occupancy_epsilon: numerics::OCCUPANCY_EPSILON,
                log_zero_f32: numerics::LOG_ZERO,
                log_zero_f64: numerics::LOG_ZERO_F64,
                pi: std::f64::consts::PI,
            })
        })
    }
}
