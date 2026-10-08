//! Precision-specific numerical and mutable-table entry points.
use super::{numerical, scalar_result};
use crate::numerics;

/// Evaluate the source numerical kernel; IEEE nonfinite behavior follows native Rust.
/// Errors leave output unchanged.
/// # Safety
/// Output is independent aligned exclusively writable storage for one f32.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_log_add_f32(
    first: f32,
    second: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward the independent scalar output contract.
    unsafe { scalar_result(output, || Ok(numerics::log_add(first, second))) }
}

/// Evaluate the source numerical kernel; IEEE nonfinite behavior follows native Rust.
/// Errors leave output unchanged.
/// # Safety
/// Output is independent aligned exclusively writable storage for one f32.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_negative_exponential_f32(
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward the independent scalar output contract.
    unsafe { scalar_result(output, || Ok(numerics::negative_exponential(value))) }
}

/// Evaluate the source numerical kernel; IEEE nonfinite behavior follows native Rust.
/// Errors leave output unchanged.
/// # Safety
/// Output is independent aligned exclusively writable storage for one f32.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_exponential_f32(value: f32, output: *mut f32) -> u32 {
    // SAFETY: Forward the independent scalar output contract.
    unsafe { scalar_result(output, || Ok(numerics::exponential(value))) }
}

/// Read an atomic coefficient in table 0 (log-add) or 1 (exponential).
/// Invalid table/range/output leaves the table and caller outputs unchanged.
/// Count zero in copy permits null output at any valid boundary.
/// Configure coefficients before reproducible inference/training; reads observe per-entry atomic state.
/// # Safety
/// Output is independent aligned exclusively writable storage for one scalar,
/// or count scalars in copy. It must not alias coefficient storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_table_get_f32(
    table: u32,
    index: usize,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward the independent output storage and checked interval contracts.
    unsafe { numerical::get(table, index, output) }
}

/// Replace an atomic coefficient, returning its previous bits. All scalar bits are accepted.
/// Invalid table/range/output leaves the table and caller outputs unchanged.
/// Count zero in copy permits null output at any valid boundary.
/// Configure coefficients before reproducible inference/training; reads observe per-entry atomic state.
/// # Safety
/// Output is independent aligned exclusively writable storage for one scalar,
/// or count scalars in copy. It must not alias coefficient storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_table_replace_f32(
    table: u32,
    index: usize,
    value: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Forward the independent output storage and checked interval contracts.
    unsafe { numerical::replace(table, index, value, output) }
}

/// Copy a checked coefficient interval; no whole-table snapshot is implied.
/// Invalid table/range/output leaves the table and caller outputs unchanged.
/// Count zero in copy permits null output at any valid boundary.
/// Configure coefficients before reproducible inference/training; reads observe per-entry atomic state.
/// # Safety
/// Output is independent aligned exclusively writable storage for one scalar,
/// or count scalars in copy. It must not alias coefficient storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_table_copy_f32(
    table: u32,
    offset: usize,
    output: *mut f32,
    count: usize,
) -> u32 {
    // SAFETY: Forward the independent output storage and checked interval contracts.
    unsafe { numerical::copy(table, offset, output, count) }
}

/// Evaluate the source numerical kernel; IEEE nonfinite behavior follows native Rust.
/// Errors leave output unchanged.
/// # Safety
/// Output is independent aligned exclusively writable storage for one f64.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_log_add_f64(
    first: f64,
    second: f64,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward the independent scalar output contract.
    unsafe { scalar_result(output, || Ok(numerics::log_add_f64(first, second))) }
}

/// Evaluate the source numerical kernel; IEEE nonfinite behavior follows native Rust.
/// Errors leave output unchanged.
/// # Safety
/// Output is independent aligned exclusively writable storage for one f64.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_negative_exponential_f64(
    value: f64,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward the independent scalar output contract.
    unsafe { scalar_result(output, || Ok(numerics::negative_exponential_f64(value))) }
}

/// Evaluate the source numerical kernel; IEEE nonfinite behavior follows native Rust.
/// Errors leave output unchanged.
/// # Safety
/// Output is independent aligned exclusively writable storage for one f64.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_exponential_f64(value: f64, output: *mut f64) -> u32 {
    // SAFETY: Forward the independent scalar output contract.
    unsafe { scalar_result(output, || Ok(numerics::exponential_f64(value))) }
}

/// Read an atomic coefficient in table 0 (log-add) or 1 (exponential).
/// Invalid table/range/output leaves the table and caller outputs unchanged.
/// Count zero in copy permits null output at any valid boundary.
/// Configure coefficients before reproducible inference/training; reads observe per-entry atomic state.
/// # Safety
/// Output is independent aligned exclusively writable storage for one scalar,
/// or count scalars in copy. It must not alias coefficient storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_table_get_f64(
    table: u32,
    index: usize,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward the independent output storage and checked interval contracts.
    unsafe { numerical::get(table, index, output) }
}

/// Replace an atomic coefficient, returning its previous bits. All scalar bits are accepted.
/// Invalid table/range/output leaves the table and caller outputs unchanged.
/// Count zero in copy permits null output at any valid boundary.
/// Configure coefficients before reproducible inference/training; reads observe per-entry atomic state.
/// # Safety
/// Output is independent aligned exclusively writable storage for one scalar,
/// or count scalars in copy. It must not alias coefficient storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_table_replace_f64(
    table: u32,
    index: usize,
    value: f64,
    output: *mut f64,
) -> u32 {
    // SAFETY: Forward the independent output storage and checked interval contracts.
    unsafe { numerical::replace(table, index, value, output) }
}

/// Copy a checked coefficient interval; no whole-table snapshot is implied.
/// Invalid table/range/output leaves the table and caller outputs unchanged.
/// Count zero in copy permits null output at any valid boundary.
/// Configure coefficients before reproducible inference/training; reads observe per-entry atomic state.
/// # Safety
/// Output is independent aligned exclusively writable storage for one scalar,
/// or count scalars in copy. It must not alias coefficient storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_table_copy_f64(
    table: u32,
    offset: usize,
    output: *mut f64,
    count: usize,
) -> u32 {
    // SAFETY: Forward the independent output storage and checked interval contracts.
    unsafe { numerical::copy(table, offset, output, count) }
}

/// Read the complete source numerical constants without reinterpreting their precision.
/// # Safety
/// Output is independent aligned exclusively writable descriptor storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_numerical_constants(
    output: *mut super::LiblrhsmmRsNumericalConstants,
) -> u32 {
    // SAFETY: Forward the independent descriptor output contract.
    unsafe { numerical::constants(output) }
}
