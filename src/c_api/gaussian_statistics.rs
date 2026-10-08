//! Editable Gaussian sufficient statistics and native transactional maximization.
use super::gaussian::storage_range;
use super::{
    LiblrhsmmRsGaussianF32, LiblrhsmmRsGaussianF64, LiblrhsmmRsUpdateOptionsF32,
    LiblrhsmmRsUpdateOptionsF64,
};
use super::{boundary, range, scalar_result};
use crate::scalar::Scalar;
use crate::statistics::GaussianSufficientStatistics;

/// Mixture-major weighted observation sums.
pub const LIBLRHSMM_RS_GAUSSIAN_WEIGHTED_SUMS: u32 = 0;
/// Mixture-major weighted squared observation sums.
pub const LIBLRHSMM_RS_GAUSSIAN_WEIGHTED_SQUARED_SUMS: u32 = 1;
/// Per-component occupancies.
pub const LIBLRHSMM_RS_GAUSSIAN_COMPONENT_OCCUPANCIES: u32 = 2;
fn field<T: Scalar>(value: &GaussianSufficientStatistics<T>, selector: u32) -> Result<&[T], u32> {
    match selector {
        0 => Ok(&value.weighted_sums),
        1 => Ok(&value.weighted_squared_sums),
        2 => Ok(&value.component_occupancies),
        _ => Err(3),
    }
}
fn field_mut<T: Scalar>(
    value: &mut GaussianSufficientStatistics<T>,
    selector: u32,
) -> Result<&mut [T], u32> {
    match selector {
        0 => Ok(&mut value.weighted_sums),
        1 => Ok(&mut value.weighted_squared_sums),
        2 => Ok(&mut value.component_occupancies),
        _ => Err(3),
    }
}
fn interval(offset: usize, count: usize, length: usize) -> Result<std::ops::Range<usize>, u32> {
    let end = offset.checked_add(count).ok_or(2u32)?;
    if end > length {
        return Err(2);
    }
    Ok(offset..end)
}

/// Complete borrowed statistics input; shape determines all array lengths.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LiblrhsmmRsGaussianStatisticsValuesF32 {
    pub mixtures: usize,
    pub dimensions: usize,
    pub state_occupancy: f32,
    pub weighted_sums: *const f32,
    pub weighted_squared_sums: *const f32,
    pub component_occupancies: *const f32,
}
/// Complete scalar metadata for the current statistics.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LiblrhsmmRsGaussianStatisticsLayoutF32 {
    pub mixtures: usize,
    pub dimensions: usize,
    pub state_occupancy: f32,
}
/// Independent editable statistics; no source-model borrow is retained.
pub struct LiblrhsmmRsGaussianStatisticsF32 {
    pub(super) statistics: GaussianSufficientStatistics<f32>,
}
unsafe fn input_f32(
    descriptor: *const LiblrhsmmRsGaussianStatisticsValuesF32,
) -> Result<GaussianSufficientStatistics<f32>, u32> {
    range(descriptor, 1)?;
    // SAFETY: Caller supplies a readable initialized descriptor.
    let input = unsafe { descriptor.read() };
    let length = input.mixtures.checked_mul(input.dimensions).ok_or(2u32)?;
    for (pointer, count) in [
        (input.weighted_sums, length),
        (input.weighted_squared_sums, length),
        (input.component_occupancies, input.mixtures),
    ] {
        storage_range(pointer, count)?;
    }
    boundary(|| {
        let mut value = GaussianSufficientStatistics::new(input.mixtures, input.dimensions)?;
        // SAFETY: Shape is nonempty and checked; input arrays have independent readable storage.
        unsafe {
            value
                .weighted_sums
                .copy_from_slice(std::slice::from_raw_parts(input.weighted_sums, length));
            value
                .weighted_squared_sums
                .copy_from_slice(std::slice::from_raw_parts(
                    input.weighted_squared_sums,
                    length,
                ));
            value
                .component_occupancies
                .copy_from_slice(std::slice::from_raw_parts(
                    input.component_occupancies,
                    input.mixtures,
                ));
        }
        value.state_occupancy = input.state_occupancy;
        Ok(value)
    })
}

/// Allocate native zeroed statistics for a specified Gaussian shape.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable aligned storage; outputs identify independent aligned writable storage.
/// Owners being mutated or released and output buffers require exclusive access.
/// No reads overlap mutation/release. Arrays and descriptors cannot alias owner
/// storage and are borrowed only for this call. Ownership slots are initialized;
/// constructor outputs must not overwrite live ownership. Release consumes unique
/// ownership and clears its slot. Numeric validation is delegated to native update.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_statistics_new_f32(
    mixtures: usize,
    dimensions: usize,
    output: *mut *mut LiblrhsmmRsGaussianStatisticsF32,
) -> u32 {
    // SAFETY: Independent output slot is validated before allocating native statistics.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianStatisticsF32 {
                statistics: GaussianSufficientStatistics::new(mixtures, dimensions)?,
            })))
        })
    }
}

/// Copy every field into independent editable storage; scalar validation is deferred.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable aligned storage; outputs identify independent aligned writable storage.
/// Owners being mutated or released and output buffers require exclusive access.
/// No reads overlap mutation/release. Arrays and descriptors cannot alias owner
/// storage and are borrowed only for this call. Ownership slots are initialized;
/// constructor outputs must not overwrite live ownership. Release consumes unique
/// ownership and clears its slot. Numeric validation is delegated to native update.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_statistics_create_f32(
    values: *const LiblrhsmmRsGaussianStatisticsValuesF32,
    output: *mut *mut LiblrhsmmRsGaussianStatisticsF32,
) -> u32 {
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Independent initialized input, shape-derived readable arrays and output slot.
    let value = match unsafe { input_f32(values) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianStatisticsF32 {
                statistics: value,
            })))
        })
    }
}

/// Replace the complete shape and values atomically.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable aligned storage; outputs identify independent aligned writable storage.
/// Owners being mutated or released and output buffers require exclusive access.
/// No reads overlap mutation/release. Arrays and descriptors cannot alias owner
/// storage and are borrowed only for this call. Ownership slots are initialized;
/// constructor outputs must not overwrite live ownership. Release consumes unique
/// ownership and clears its slot. Numeric validation is delegated to native update.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_statistics_replace_f32(
    handle: *mut LiblrhsmmRsGaussianStatisticsF32,
    values: *const LiblrhsmmRsGaussianStatisticsValuesF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Exclusive owner and independent borrowed descriptor; stage before publishing.
    match unsafe { input_f32(values) } {
        Ok(value) => {
            unsafe {
                (*handle).statistics = value;
            }
            0
        }
        Err(status) => status,
    }
}

/// Create native zeroed sufficient statistics matched to a model shape.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable aligned storage; outputs identify independent aligned writable storage.
/// Owners being mutated or released and output buffers require exclusive access.
/// No reads overlap mutation/release. Arrays and descriptors cannot alias owner
/// storage and are borrowed only for this call. Ownership slots are initialized;
/// constructor outputs must not overwrite live ownership. Release consumes unique
/// ownership and clears its slot. Numeric validation is delegated to native update.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_statistics_from_gaussian_f32(
    handle: *const LiblrhsmmRsGaussianF32,
    output: *mut *mut LiblrhsmmRsGaussianStatisticsF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Readable source model and independent output; no source lifetime is retained.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianStatisticsF32 {
                statistics: GaussianSufficientStatistics::from_gaussian(&(*handle).parameters)?,
            })))
        })
    }
}

/// Read mixture count, dimensions and state occupancy.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable aligned storage; outputs identify independent aligned writable storage.
/// Owners being mutated or released and output buffers require exclusive access.
/// No reads overlap mutation/release. Arrays and descriptors cannot alias owner
/// storage and are borrowed only for this call. Ownership slots are initialized;
/// constructor outputs must not overwrite live ownership. Release consumes unique
/// ownership and clears its slot. Numeric validation is delegated to native update.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_statistics_layout_f32(
    handle: *const LiblrhsmmRsGaussianStatisticsF32,
    output: *mut LiblrhsmmRsGaussianStatisticsLayoutF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Readable owner and independently writable scalar descriptor.
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsGaussianStatisticsLayoutF32 {
                mixtures: (*handle).statistics.component_occupancies.len(),
                dimensions: (*handle).statistics.dimensions,
                state_occupancy: (*handle).statistics.state_occupancy,
            })
        })
    }
}

/// Edit the state occupancy independently of component occupancies.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable aligned storage; outputs identify independent aligned writable storage.
/// Owners being mutated or released and output buffers require exclusive access.
/// No reads overlap mutation/release. Arrays and descriptors cannot alias owner
/// storage and are borrowed only for this call. Ownership slots are initialized;
/// constructor outputs must not overwrite live ownership. Release consumes unique
/// ownership and clears its slot. Numeric validation is delegated to native update.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_statistics_set_occupancy_f32(
    handle: *mut LiblrhsmmRsGaussianStatisticsF32,
    occupancy: f32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Exclusive live owner; no numerical validation occurs during editing.
    unsafe {
        (*handle).statistics.state_occupancy = occupancy;
    }
    0
}

/// Copy an interval without exposing internal pointers.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable aligned storage; outputs identify independent aligned writable storage.
/// Owners being mutated or released and output buffers require exclusive access.
/// No reads overlap mutation/release. Arrays and descriptors cannot alias owner
/// storage and are borrowed only for this call. Ownership slots are initialized;
/// constructor outputs must not overwrite live ownership. Release consumes unique
/// ownership and clears its slot. Numeric validation is delegated to native update.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_statistics_copy_field_f32(
    handle: *const LiblrhsmmRsGaussianStatisticsF32,
    selector: u32,
    offset: usize,
    buffer: *mut f32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = storage_range(buffer, count) {
        return status;
    }
    // SAFETY: Live owner, independent storage and exclusive mutation where required.
    let values = match unsafe { field(&(*handle).statistics, selector) } {
        Ok(values) => values,
        Err(status) => return status,
    };
    let selected = match interval(offset, count, values.len()) {
        Ok(selected) => selected,
        Err(status) => return status,
    };
    if count != 0 {
        unsafe {
            std::ptr::copy_nonoverlapping(values[selected].as_ptr(), buffer, count);
        }
    }
    0
}

/// Edit a field interval; failure preserves all statistics.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable aligned storage; outputs identify independent aligned writable storage.
/// Owners being mutated or released and output buffers require exclusive access.
/// No reads overlap mutation/release. Arrays and descriptors cannot alias owner
/// storage and are borrowed only for this call. Ownership slots are initialized;
/// constructor outputs must not overwrite live ownership. Release consumes unique
/// ownership and clears its slot. Numeric validation is delegated to native update.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_statistics_set_field_f32(
    handle: *mut LiblrhsmmRsGaussianStatisticsF32,
    selector: u32,
    offset: usize,
    buffer: *const f32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = storage_range(buffer, count) {
        return status;
    }
    // SAFETY: Live owner, independent storage and exclusive mutation where required.
    let values = match unsafe { field_mut(&mut (*handle).statistics, selector) } {
        Ok(values) => values,
        Err(status) => return status,
    };
    let selected = match interval(offset, count, values.len()) {
        Ok(selected) => selected,
        Err(status) => return status,
    };
    if count != 0 {
        unsafe {
            values[selected].copy_from_slice(std::slice::from_raw_parts(buffer, count));
        }
    }
    0
}

/// Clone all scalar/vector fields independently.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable aligned storage; outputs identify independent aligned writable storage.
/// Owners being mutated or released and output buffers require exclusive access.
/// No reads overlap mutation/release. Arrays and descriptors cannot alias owner
/// storage and are borrowed only for this call. Ownership slots are initialized;
/// constructor outputs must not overwrite live ownership. Release consumes unique
/// ownership and clears its slot. Numeric validation is delegated to native update.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_statistics_clone_f32(
    handle: *const LiblrhsmmRsGaussianStatisticsF32,
    output: *mut *mut LiblrhsmmRsGaussianStatisticsF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Independent output slot and readable model-independent statistics.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianStatisticsF32 {
                statistics: (*handle).statistics.clone(),
            })))
        })
    }
}

/// Clear all accumulators with the unchanged native operation.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable aligned storage; outputs identify independent aligned writable storage.
/// Owners being mutated or released and output buffers require exclusive access.
/// No reads overlap mutation/release. Arrays and descriptors cannot alias owner
/// storage and are borrowed only for this call. Ownership slots are initialized;
/// constructor outputs must not overwrite live ownership. Release consumes unique
/// ownership and clears its slot. Numeric validation is delegated to native update.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_statistics_clear_f32(
    handle: *mut LiblrhsmmRsGaussianStatisticsF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Exclusive live owner; native clearing preserves shape.
    unsafe {
        (*handle).statistics.clear();
    }
    0
}

/// Release unique ownership and clear the slot; null ownership is a no-op.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable aligned storage; outputs identify independent aligned writable storage.
/// Owners being mutated or released and output buffers require exclusive access.
/// No reads overlap mutation/release. Arrays and descriptors cannot alias owner
/// storage and are borrowed only for this call. Ownership slots are initialized;
/// constructor outputs must not overwrite live ownership. Release consumes unique
/// ownership and clears its slot. Numeric validation is delegated to native update.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_statistics_release_f32(
    handle: *mut *mut LiblrhsmmRsGaussianStatisticsF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Initialized exclusively owned slot, null or a live unique owner.
    let pointer = unsafe { handle.read() };
    if pointer.is_null() {
        return 0;
    }
    if let Err(status) = range(pointer, 1) {
        return status;
    }
    unsafe {
        handle.write(std::ptr::null_mut());
        drop(Box::from_raw(pointer));
    }
    0
}

/// Apply unchanged native maximization; output is one if updated, zero for low occupancy.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable aligned storage; outputs identify independent aligned writable storage.
/// Owners being mutated or released and output buffers require exclusive access.
/// No reads overlap mutation/release. Arrays and descriptors cannot alias owner
/// storage and are borrowed only for this call. Ownership slots are initialized;
/// constructor outputs must not overwrite live ownership. Release consumes unique
/// ownership and clears its slot. Numeric validation is delegated to native update.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_update_f32(
    handle: *mut LiblrhsmmRsGaussianF32,
    statistics: *const LiblrhsmmRsGaussianStatisticsF32,
    options: *const LiblrhsmmRsUpdateOptionsF32,
    output: *mut u32,
) -> u32 {
    for result in [range(handle, 1), range(statistics, 1), range(options, 1)] {
        if let Err(status) = result {
            return status;
        }
    }
    // SAFETY: Exclusive model, independent readable inputs and output prevalidated by scalar_result.
    // Native Gaussian update stages its own replacement and publishes only on success.
    unsafe {
        scalar_result(output, || {
            let updated = (*handle)
                .parameters
                .update(&(*statistics).statistics, options.read().try_into()?)?;
            Ok(u32::from(updated))
        })
    }
}

/// Complete borrowed statistics input; shape determines all array lengths.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LiblrhsmmRsGaussianStatisticsValuesF64 {
    pub mixtures: usize,
    pub dimensions: usize,
    pub state_occupancy: f64,
    pub weighted_sums: *const f64,
    pub weighted_squared_sums: *const f64,
    pub component_occupancies: *const f64,
}
/// Complete scalar metadata for the current statistics.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LiblrhsmmRsGaussianStatisticsLayoutF64 {
    pub mixtures: usize,
    pub dimensions: usize,
    pub state_occupancy: f64,
}
/// Independent editable statistics; no source-model borrow is retained.
pub struct LiblrhsmmRsGaussianStatisticsF64 {
    pub(super) statistics: GaussianSufficientStatistics<f64>,
}
unsafe fn input_f64(
    descriptor: *const LiblrhsmmRsGaussianStatisticsValuesF64,
) -> Result<GaussianSufficientStatistics<f64>, u32> {
    range(descriptor, 1)?;
    // SAFETY: Caller supplies a readable initialized descriptor.
    let input = unsafe { descriptor.read() };
    let length = input.mixtures.checked_mul(input.dimensions).ok_or(2u32)?;
    for (pointer, count) in [
        (input.weighted_sums, length),
        (input.weighted_squared_sums, length),
        (input.component_occupancies, input.mixtures),
    ] {
        storage_range(pointer, count)?;
    }
    boundary(|| {
        let mut value = GaussianSufficientStatistics::new(input.mixtures, input.dimensions)?;
        // SAFETY: Shape is nonempty and checked; input arrays have independent readable storage.
        unsafe {
            value
                .weighted_sums
                .copy_from_slice(std::slice::from_raw_parts(input.weighted_sums, length));
            value
                .weighted_squared_sums
                .copy_from_slice(std::slice::from_raw_parts(
                    input.weighted_squared_sums,
                    length,
                ));
            value
                .component_occupancies
                .copy_from_slice(std::slice::from_raw_parts(
                    input.component_occupancies,
                    input.mixtures,
                ));
        }
        value.state_occupancy = input.state_occupancy;
        Ok(value)
    })
}

/// Allocate native zeroed statistics for a specified Gaussian shape.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable aligned storage; outputs identify independent aligned writable storage.
/// Owners being mutated or released and output buffers require exclusive access.
/// No reads overlap mutation/release. Arrays and descriptors cannot alias owner
/// storage and are borrowed only for this call. Ownership slots are initialized;
/// constructor outputs must not overwrite live ownership. Release consumes unique
/// ownership and clears its slot. Numeric validation is delegated to native update.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_statistics_new_f64(
    mixtures: usize,
    dimensions: usize,
    output: *mut *mut LiblrhsmmRsGaussianStatisticsF64,
) -> u32 {
    // SAFETY: Independent output slot is validated before allocating native statistics.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianStatisticsF64 {
                statistics: GaussianSufficientStatistics::new(mixtures, dimensions)?,
            })))
        })
    }
}

/// Copy every field into independent editable storage; scalar validation is deferred.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable aligned storage; outputs identify independent aligned writable storage.
/// Owners being mutated or released and output buffers require exclusive access.
/// No reads overlap mutation/release. Arrays and descriptors cannot alias owner
/// storage and are borrowed only for this call. Ownership slots are initialized;
/// constructor outputs must not overwrite live ownership. Release consumes unique
/// ownership and clears its slot. Numeric validation is delegated to native update.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_statistics_create_f64(
    values: *const LiblrhsmmRsGaussianStatisticsValuesF64,
    output: *mut *mut LiblrhsmmRsGaussianStatisticsF64,
) -> u32 {
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Independent initialized input, shape-derived readable arrays and output slot.
    let value = match unsafe { input_f64(values) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianStatisticsF64 {
                statistics: value,
            })))
        })
    }
}

/// Replace the complete shape and values atomically.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable aligned storage; outputs identify independent aligned writable storage.
/// Owners being mutated or released and output buffers require exclusive access.
/// No reads overlap mutation/release. Arrays and descriptors cannot alias owner
/// storage and are borrowed only for this call. Ownership slots are initialized;
/// constructor outputs must not overwrite live ownership. Release consumes unique
/// ownership and clears its slot. Numeric validation is delegated to native update.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_statistics_replace_f64(
    handle: *mut LiblrhsmmRsGaussianStatisticsF64,
    values: *const LiblrhsmmRsGaussianStatisticsValuesF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Exclusive owner and independent borrowed descriptor; stage before publishing.
    match unsafe { input_f64(values) } {
        Ok(value) => {
            unsafe {
                (*handle).statistics = value;
            }
            0
        }
        Err(status) => status,
    }
}

/// Create native zeroed sufficient statistics matched to a model shape.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable aligned storage; outputs identify independent aligned writable storage.
/// Owners being mutated or released and output buffers require exclusive access.
/// No reads overlap mutation/release. Arrays and descriptors cannot alias owner
/// storage and are borrowed only for this call. Ownership slots are initialized;
/// constructor outputs must not overwrite live ownership. Release consumes unique
/// ownership and clears its slot. Numeric validation is delegated to native update.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_statistics_from_gaussian_f64(
    handle: *const LiblrhsmmRsGaussianF64,
    output: *mut *mut LiblrhsmmRsGaussianStatisticsF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Readable source model and independent output; no source lifetime is retained.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianStatisticsF64 {
                statistics: GaussianSufficientStatistics::from_gaussian(&(*handle).parameters)?,
            })))
        })
    }
}

/// Read mixture count, dimensions and state occupancy.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable aligned storage; outputs identify independent aligned writable storage.
/// Owners being mutated or released and output buffers require exclusive access.
/// No reads overlap mutation/release. Arrays and descriptors cannot alias owner
/// storage and are borrowed only for this call. Ownership slots are initialized;
/// constructor outputs must not overwrite live ownership. Release consumes unique
/// ownership and clears its slot. Numeric validation is delegated to native update.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_statistics_layout_f64(
    handle: *const LiblrhsmmRsGaussianStatisticsF64,
    output: *mut LiblrhsmmRsGaussianStatisticsLayoutF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Readable owner and independently writable scalar descriptor.
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsGaussianStatisticsLayoutF64 {
                mixtures: (*handle).statistics.component_occupancies.len(),
                dimensions: (*handle).statistics.dimensions,
                state_occupancy: (*handle).statistics.state_occupancy,
            })
        })
    }
}

/// Edit the state occupancy independently of component occupancies.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable aligned storage; outputs identify independent aligned writable storage.
/// Owners being mutated or released and output buffers require exclusive access.
/// No reads overlap mutation/release. Arrays and descriptors cannot alias owner
/// storage and are borrowed only for this call. Ownership slots are initialized;
/// constructor outputs must not overwrite live ownership. Release consumes unique
/// ownership and clears its slot. Numeric validation is delegated to native update.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_statistics_set_occupancy_f64(
    handle: *mut LiblrhsmmRsGaussianStatisticsF64,
    occupancy: f64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Exclusive live owner; no numerical validation occurs during editing.
    unsafe {
        (*handle).statistics.state_occupancy = occupancy;
    }
    0
}

/// Copy an interval without exposing internal pointers.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable aligned storage; outputs identify independent aligned writable storage.
/// Owners being mutated or released and output buffers require exclusive access.
/// No reads overlap mutation/release. Arrays and descriptors cannot alias owner
/// storage and are borrowed only for this call. Ownership slots are initialized;
/// constructor outputs must not overwrite live ownership. Release consumes unique
/// ownership and clears its slot. Numeric validation is delegated to native update.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_statistics_copy_field_f64(
    handle: *const LiblrhsmmRsGaussianStatisticsF64,
    selector: u32,
    offset: usize,
    buffer: *mut f64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = storage_range(buffer, count) {
        return status;
    }
    // SAFETY: Live owner, independent storage and exclusive mutation where required.
    let values = match unsafe { field(&(*handle).statistics, selector) } {
        Ok(values) => values,
        Err(status) => return status,
    };
    let selected = match interval(offset, count, values.len()) {
        Ok(selected) => selected,
        Err(status) => return status,
    };
    if count != 0 {
        unsafe {
            std::ptr::copy_nonoverlapping(values[selected].as_ptr(), buffer, count);
        }
    }
    0
}

/// Edit a field interval; failure preserves all statistics.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable aligned storage; outputs identify independent aligned writable storage.
/// Owners being mutated or released and output buffers require exclusive access.
/// No reads overlap mutation/release. Arrays and descriptors cannot alias owner
/// storage and are borrowed only for this call. Ownership slots are initialized;
/// constructor outputs must not overwrite live ownership. Release consumes unique
/// ownership and clears its slot. Numeric validation is delegated to native update.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_statistics_set_field_f64(
    handle: *mut LiblrhsmmRsGaussianStatisticsF64,
    selector: u32,
    offset: usize,
    buffer: *const f64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = storage_range(buffer, count) {
        return status;
    }
    // SAFETY: Live owner, independent storage and exclusive mutation where required.
    let values = match unsafe { field_mut(&mut (*handle).statistics, selector) } {
        Ok(values) => values,
        Err(status) => return status,
    };
    let selected = match interval(offset, count, values.len()) {
        Ok(selected) => selected,
        Err(status) => return status,
    };
    if count != 0 {
        unsafe {
            values[selected].copy_from_slice(std::slice::from_raw_parts(buffer, count));
        }
    }
    0
}

/// Clone all scalar/vector fields independently.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable aligned storage; outputs identify independent aligned writable storage.
/// Owners being mutated or released and output buffers require exclusive access.
/// No reads overlap mutation/release. Arrays and descriptors cannot alias owner
/// storage and are borrowed only for this call. Ownership slots are initialized;
/// constructor outputs must not overwrite live ownership. Release consumes unique
/// ownership and clears its slot. Numeric validation is delegated to native update.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_statistics_clone_f64(
    handle: *const LiblrhsmmRsGaussianStatisticsF64,
    output: *mut *mut LiblrhsmmRsGaussianStatisticsF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Independent output slot and readable model-independent statistics.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianStatisticsF64 {
                statistics: (*handle).statistics.clone(),
            })))
        })
    }
}

/// Clear all accumulators with the unchanged native operation.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable aligned storage; outputs identify independent aligned writable storage.
/// Owners being mutated or released and output buffers require exclusive access.
/// No reads overlap mutation/release. Arrays and descriptors cannot alias owner
/// storage and are borrowed only for this call. Ownership slots are initialized;
/// constructor outputs must not overwrite live ownership. Release consumes unique
/// ownership and clears its slot. Numeric validation is delegated to native update.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_statistics_clear_f64(
    handle: *mut LiblrhsmmRsGaussianStatisticsF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Exclusive live owner; native clearing preserves shape.
    unsafe {
        (*handle).statistics.clear();
    }
    0
}

/// Release unique ownership and clear the slot; null ownership is a no-op.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable aligned storage; outputs identify independent aligned writable storage.
/// Owners being mutated or released and output buffers require exclusive access.
/// No reads overlap mutation/release. Arrays and descriptors cannot alias owner
/// storage and are borrowed only for this call. Ownership slots are initialized;
/// constructor outputs must not overwrite live ownership. Release consumes unique
/// ownership and clears its slot. Numeric validation is delegated to native update.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_statistics_release_f64(
    handle: *mut *mut LiblrhsmmRsGaussianStatisticsF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Initialized exclusively owned slot, null or a live unique owner.
    let pointer = unsafe { handle.read() };
    if pointer.is_null() {
        return 0;
    }
    if let Err(status) = range(pointer, 1) {
        return status;
    }
    unsafe {
        handle.write(std::ptr::null_mut());
        drop(Box::from_raw(pointer));
    }
    0
}

/// Apply unchanged native maximization; output is one if updated, zero for low occupancy.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable aligned storage; outputs identify independent aligned writable storage.
/// Owners being mutated or released and output buffers require exclusive access.
/// No reads overlap mutation/release. Arrays and descriptors cannot alias owner
/// storage and are borrowed only for this call. Ownership slots are initialized;
/// constructor outputs must not overwrite live ownership. Release consumes unique
/// ownership and clears its slot. Numeric validation is delegated to native update.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_update_f64(
    handle: *mut LiblrhsmmRsGaussianF64,
    statistics: *const LiblrhsmmRsGaussianStatisticsF64,
    options: *const LiblrhsmmRsUpdateOptionsF64,
    output: *mut u32,
) -> u32 {
    for result in [range(handle, 1), range(statistics, 1), range(options, 1)] {
        if let Err(status) = result {
            return status;
        }
    }
    // SAFETY: Exclusive model, independent readable inputs and output prevalidated by scalar_result.
    // Native Gaussian update stages its own replacement and publishes only on success.
    unsafe {
        scalar_result(output, || {
            let updated = (*handle)
                .parameters
                .update(&(*statistics).statistics, options.read().try_into()?)?;
            Ok(u32::from(updated))
        })
    }
}
