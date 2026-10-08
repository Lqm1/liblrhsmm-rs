//! Independent Gaussian-mixture parameter owners and native transformations.
use super::{LiblrhsmmRsDurationF32, LiblrhsmmRsDurationF64};
use super::{boundary, range, scalar_result};
use crate::model::{GaussianMixtureParameters, ModelError};
use crate::scalar::Scalar;

/// Gaussian mixture weights.
pub const LIBLRHSMM_RS_GAUSSIAN_WEIGHTS: u32 = 0;
/// Mixture-major means.
pub const LIBLRHSMM_RS_GAUSSIAN_MEANS: u32 = 1;
/// Mixture-major diagonal variances.
pub const LIBLRHSMM_RS_GAUSSIAN_VARIANCES: u32 = 2;
/// Mixture-major variance floors.
pub const LIBLRHSMM_RS_GAUSSIAN_VARIANCE_FLOORS: u32 = 3;

/// Current shape; each non-weight field has mixtures * dimensions entries.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiblrhsmmRsGaussianLayout {
    pub mixtures: usize,
    pub dimensions: usize,
}
fn field<T: Scalar>(value: &GaussianMixtureParameters<T>, selector: u32) -> Result<&[T], u32> {
    match selector {
        0 => Ok(&value.weights),
        1 => Ok(&value.means),
        2 => Ok(&value.variances),
        3 => Ok(&value.variance_floors),
        _ => Err(3),
    }
}
fn field_mut<T: Scalar>(
    value: &mut GaussianMixtureParameters<T>,
    selector: u32,
) -> Result<&mut [T], u32> {
    match selector {
        0 => Ok(&mut value.weights),
        1 => Ok(&mut value.means),
        2 => Ok(&mut value.variances),
        3 => Ok(&mut value.variance_floors),
        _ => Err(3),
    }
}
// Parameter arrays use address-space limits, not a legacy frame-count limit.
pub(super) fn storage_range<T>(pointer: *const T, count: usize) -> Result<(), u32> {
    if count > isize::MAX as usize / size_of::<T>() {
        return Err(2);
    }
    if count != 0 && (pointer.is_null() || !pointer.is_aligned()) {
        return Err(1);
    }
    Ok(())
}
fn interval(offset: usize, count: usize, length: usize) -> Result<std::ops::Range<usize>, u32> {
    let end = offset.checked_add(count).ok_or(2u32)?;
    if end > length {
        return Err(2);
    }
    Ok(offset..end)
}

/// Complete borrowed input. All arrays use the shape-derived lengths.
/// Scalars retain arbitrary bits, including flat-start zero variances.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LiblrhsmmRsGaussianParametersF32 {
    pub mixtures: usize,
    pub dimensions: usize,
    pub weights: *const f32,
    pub means: *const f32,
    pub variances: *const f32,
    pub variance_floors: *const f32,
}
/// Independent editable Gaussian-mixture parameters.
pub struct LiblrhsmmRsGaussianF32 {
    pub(super) parameters: GaussianMixtureParameters<f32>,
}
unsafe fn input_f32(
    descriptor: *const LiblrhsmmRsGaussianParametersF32,
) -> Result<GaussianMixtureParameters<f32>, u32> {
    range(descriptor, 1)?;
    // SAFETY: The caller supplies an initialized independent descriptor.
    let input = unsafe { descriptor.read() };
    let length = input.mixtures.checked_mul(input.dimensions).ok_or(2u32)?;
    for (pointer, count) in [
        (input.weights, input.mixtures),
        (input.means, length),
        (input.variances, length),
        (input.variance_floors, length),
    ] {
        storage_range(pointer, count)?;
    }
    boundary(|| {
        let mut value = GaussianMixtureParameters::new(input.mixtures, input.dimensions)?;
        // SAFETY: Checked nonempty input arrays have the declared readable lengths.
        unsafe {
            value
                .weights
                .copy_from_slice(std::slice::from_raw_parts(input.weights, input.mixtures));
            value
                .means
                .copy_from_slice(std::slice::from_raw_parts(input.means, length));
            value
                .variances
                .copy_from_slice(std::slice::from_raw_parts(input.variances, length));
            value
                .variance_floors
                .copy_from_slice(std::slice::from_raw_parts(input.variance_floors, length));
        }
        Ok(value)
    })
}

/// Construct native uniform-weight, zero-parameter flat-start storage.
/// # Safety
/// Handles are live owners of the declared precision. Descriptor and input storage
/// is initialized and readable; output storage is aligned and independently writable.
/// Writable storage and mutated or released owners require exclusive access.
/// No concurrent mutation/release overlaps reads. Inputs/outputs cannot alias owner
/// storage; input handles in borrowed arrays may repeat. Release consumes unique
/// ownership and clears its initialized slot. Buffers are borrowed only for this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_new_f32(
    mixtures: usize,
    dimensions: usize,
    output: *mut *mut LiblrhsmmRsGaussianF32,
) -> u32 {
    // SAFETY: The output slot is independent writable storage.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianF32 {
                parameters: GaussianMixtureParameters::new(mixtures, dimensions)?,
            })))
        })
    }
}

/// Copy all four parameter arrays into an independent owner.
/// # Safety
/// Handles are live owners of the declared precision. Descriptor and input storage
/// is initialized and readable; output storage is aligned and independently writable.
/// Writable storage and mutated or released owners require exclusive access.
/// No concurrent mutation/release overlaps reads. Inputs/outputs cannot alias owner
/// storage; input handles in borrowed arrays may repeat. Release consumes unique
/// ownership and clears its initialized slot. Buffers are borrowed only for this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_create_f32(
    parameters: *const LiblrhsmmRsGaussianParametersF32,
    output: *mut *mut LiblrhsmmRsGaussianF32,
) -> u32 {
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Validated output; descriptor follows the input contract.
    let value = match unsafe { input_f32(parameters) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianF32 {
                parameters: value,
            })))
        })
    }
}

/// Replace the complete shape and parameter arrays. Failures preserve the owner.
/// # Safety
/// Handles are live owners of the declared precision. Descriptor and input storage
/// is initialized and readable; output storage is aligned and independently writable.
/// Writable storage and mutated or released owners require exclusive access.
/// No concurrent mutation/release overlaps reads. Inputs/outputs cannot alias owner
/// storage; input handles in borrowed arrays may repeat. Release consumes unique
/// ownership and clears its initialized slot. Buffers are borrowed only for this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_replace_f32(
    handle: *mut LiblrhsmmRsGaussianF32,
    parameters: *const LiblrhsmmRsGaussianParametersF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Exclusive owner and independent borrowed arrays; stage before publishing.
    match unsafe { input_f32(parameters) } {
        Ok(value) => {
            unsafe {
                (*handle).parameters = value;
            }
            0
        }
        Err(status) => status,
    }
}

/// Copy current dimensions and mixture count.
/// # Safety
/// Handles are live owners of the declared precision. Descriptor and input storage
/// is initialized and readable; output storage is aligned and independently writable.
/// Writable storage and mutated or released owners require exclusive access.
/// No concurrent mutation/release overlaps reads. Inputs/outputs cannot alias owner
/// storage; input handles in borrowed arrays may repeat. Release consumes unique
/// ownership and clears its initialized slot. Buffers are borrowed only for this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_layout_f32(
    handle: *const LiblrhsmmRsGaussianF32,
    output: *mut LiblrhsmmRsGaussianLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable owner and independent output.
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsGaussianLayout {
                mixtures: (*handle).parameters.weights.len(),
                dimensions: (*handle).parameters.dimensions,
            })
        })
    }
}

/// Copy a field interval; zero length permits a null buffer.
/// # Safety
/// Handles are live owners of the declared precision. Descriptor and input storage
/// is initialized and readable; output storage is aligned and independently writable.
/// Writable storage and mutated or released owners require exclusive access.
/// No concurrent mutation/release overlaps reads. Inputs/outputs cannot alias owner
/// storage; input handles in borrowed arrays may repeat. Release consumes unique
/// ownership and clears its initialized slot. Buffers are borrowed only for this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_copy_field_f32(
    handle: *const LiblrhsmmRsGaussianF32,
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
    // SAFETY: Live owner, independent buffers, exclusive mutation where required.
    let values = match unsafe { field(&(*handle).parameters, selector) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    let selected = match interval(offset, count, values.len()) {
        Ok(value) => value,
        Err(status) => return status,
    };
    if count != 0 {
        unsafe {
            std::ptr::copy_nonoverlapping(values[selected].as_ptr(), buffer, count);
        }
    }
    0
}

/// Edit a field interval retaining all scalar bits; errors preserve parameters.
/// # Safety
/// Handles are live owners of the declared precision. Descriptor and input storage
/// is initialized and readable; output storage is aligned and independently writable.
/// Writable storage and mutated or released owners require exclusive access.
/// No concurrent mutation/release overlaps reads. Inputs/outputs cannot alias owner
/// storage; input handles in borrowed arrays may repeat. Release consumes unique
/// ownership and clears its initialized slot. Buffers are borrowed only for this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_set_field_f32(
    handle: *mut LiblrhsmmRsGaussianF32,
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
    // SAFETY: Live owner, independent buffers, exclusive mutation where required.
    let values = match unsafe { field_mut(&mut (*handle).parameters, selector) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    let selected = match interval(offset, count, values.len()) {
        Ok(value) => value,
        Err(status) => return status,
    };
    if count != 0 {
        unsafe {
            values[selected].copy_from_slice(std::slice::from_raw_parts(buffer, count));
        }
    }
    0
}

/// Clone all fields into independent owned storage.
/// # Safety
/// Handles are live owners of the declared precision. Descriptor and input storage
/// is initialized and readable; output storage is aligned and independently writable.
/// Writable storage and mutated or released owners require exclusive access.
/// No concurrent mutation/release overlaps reads. Inputs/outputs cannot alias owner
/// storage; input handles in borrowed arrays may repeat. Release consumes unique
/// ownership and clears its initialized slot. Buffers are borrowed only for this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_clone_f32(
    handle: *const LiblrhsmmRsGaussianF32,
    output: *mut *mut LiblrhsmmRsGaussianF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Independent output and readable live owner.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianF32 {
                parameters: (*handle).parameters.clone(),
            })))
        })
    }
}

/// Release a unique owner and clear its slot. Null ownership is a no-op.
/// # Safety
/// Handles are live owners of the declared precision. Descriptor and input storage
/// is initialized and readable; output storage is aligned and independently writable.
/// Writable storage and mutated or released owners require exclusive access.
/// No concurrent mutation/release overlaps reads. Inputs/outputs cannot alias owner
/// storage; input handles in borrowed arrays may repeat. Release consumes unique
/// ownership and clears its initialized slot. Buffers are borrowed only for this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_release_f32(
    handle: *mut *mut LiblrhsmmRsGaussianF32,
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

/// Combine borrowed owners using the unchanged native transformation; inputs are retained.
/// # Safety
/// Handles are live owners of the declared precision. Descriptor and input storage
/// is initialized and readable; output storage is aligned and independently writable.
/// Writable storage and mutated or released owners require exclusive access.
/// No concurrent mutation/release overlaps reads. Inputs/outputs cannot alias owner
/// storage; input handles in borrowed arrays may repeat. Release consumes unique
/// ownership and clears its initialized slot. Buffers are borrowed only for this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_combine_f32(
    handles: *const *const LiblrhsmmRsGaussianF32,
    count: usize,
    output: *mut *mut LiblrhsmmRsGaussianF32,
) -> u32 {
    if let Err(status) = range(output, 1) {
        return status;
    }
    if let Err(status) = range(handles, count) {
        return status;
    }
    // SAFETY: Borrowed initialized handle array. Empty arrays permit null.
    let handles = if count == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(handles, count) }
    };
    for &handle in handles {
        if let Err(status) = range(handle, 1) {
            return status;
        }
    }
    // SAFETY: All handles validated, readable, independent of output storage.
    unsafe {
        scalar_result(output, || {
            let mut values = Vec::new();
            values
                .try_reserve_exact(count)
                .map_err(|_| ModelError("input allocation failed"))?;
            for &handle in handles {
                values.push((*handle).parameters.clone());
            }
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianF32 {
                parameters: GaussianMixtureParameters::combine(&values)?,
            })))
        })
    }
}

/// Combine borrowed owners using the unchanged native transformation; inputs are retained.
/// # Safety
/// Handles are live owners of the declared precision. Descriptor and input storage
/// is initialized and readable; output storage is aligned and independently writable.
/// Writable storage and mutated or released owners require exclusive access.
/// No concurrent mutation/release overlaps reads. Inputs/outputs cannot alias owner
/// storage; input handles in borrowed arrays may repeat. Release consumes unique
/// ownership and clears its initialized slot. Buffers are borrowed only for this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_from_durations_f32(
    handles: *const *const LiblrhsmmRsDurationF32,
    count: usize,
    output: *mut *mut LiblrhsmmRsGaussianF32,
) -> u32 {
    if let Err(status) = range(output, 1) {
        return status;
    }
    if let Err(status) = range(handles, count) {
        return status;
    }
    // SAFETY: Borrowed initialized handle array. Empty arrays permit null.
    let handles = if count == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(handles, count) }
    };
    for &handle in handles {
        if let Err(status) = range(handle, 1) {
            return status;
        }
    }
    // SAFETY: All handles validated, readable, independent of output storage.
    unsafe {
        scalar_result(output, || {
            let mut values = Vec::new();
            values
                .try_reserve_exact(count)
                .map_err(|_| ModelError("input allocation failed"))?;
            for &handle in handles {
                values.push((*handle).parameters.clone());
            }
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianF32 {
                parameters: GaussianMixtureParameters::from_durations(&values)?,
            })))
        })
    }
}

/// Apply native repeated maximum-weight splitting; stage changes before publication.
/// # Safety
/// Handles are live owners of the declared precision. Descriptor and input storage
/// is initialized and readable; output storage is aligned and independently writable.
/// Writable storage and mutated or released owners require exclusive access.
/// No concurrent mutation/release overlaps reads. Inputs/outputs cannot alias owner
/// storage; input handles in borrowed arrays may repeat. Release consumes unique
/// ownership and clears its initialized slot. Buffers are borrowed only for this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_increase_mixtures_f32(
    handle: *mut LiblrhsmmRsGaussianF32,
    count: usize,
    displacement: f32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Exclusive live owner. Stage native splitting so all failures are atomic.
    let result = boundary(|| {
        let mut value = unsafe { (*handle).parameters.clone() };
        value.increase_mixtures(count, displacement)?;
        Ok(value)
    });
    match result {
        Ok(value) => {
            unsafe {
                (*handle).parameters = value;
            }
            0
        }
        Err(status) => status,
    }
}

/// Complete borrowed input. All arrays use the shape-derived lengths.
/// Scalars retain arbitrary bits, including flat-start zero variances.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LiblrhsmmRsGaussianParametersF64 {
    pub mixtures: usize,
    pub dimensions: usize,
    pub weights: *const f64,
    pub means: *const f64,
    pub variances: *const f64,
    pub variance_floors: *const f64,
}
/// Independent editable Gaussian-mixture parameters.
pub struct LiblrhsmmRsGaussianF64 {
    pub(super) parameters: GaussianMixtureParameters<f64>,
}
unsafe fn input_f64(
    descriptor: *const LiblrhsmmRsGaussianParametersF64,
) -> Result<GaussianMixtureParameters<f64>, u32> {
    range(descriptor, 1)?;
    // SAFETY: The caller supplies an initialized independent descriptor.
    let input = unsafe { descriptor.read() };
    let length = input.mixtures.checked_mul(input.dimensions).ok_or(2u32)?;
    for (pointer, count) in [
        (input.weights, input.mixtures),
        (input.means, length),
        (input.variances, length),
        (input.variance_floors, length),
    ] {
        storage_range(pointer, count)?;
    }
    boundary(|| {
        let mut value = GaussianMixtureParameters::new(input.mixtures, input.dimensions)?;
        // SAFETY: Checked nonempty input arrays have the declared readable lengths.
        unsafe {
            value
                .weights
                .copy_from_slice(std::slice::from_raw_parts(input.weights, input.mixtures));
            value
                .means
                .copy_from_slice(std::slice::from_raw_parts(input.means, length));
            value
                .variances
                .copy_from_slice(std::slice::from_raw_parts(input.variances, length));
            value
                .variance_floors
                .copy_from_slice(std::slice::from_raw_parts(input.variance_floors, length));
        }
        Ok(value)
    })
}

/// Construct native uniform-weight, zero-parameter flat-start storage.
/// # Safety
/// Handles are live owners of the declared precision. Descriptor and input storage
/// is initialized and readable; output storage is aligned and independently writable.
/// Writable storage and mutated or released owners require exclusive access.
/// No concurrent mutation/release overlaps reads. Inputs/outputs cannot alias owner
/// storage; input handles in borrowed arrays may repeat. Release consumes unique
/// ownership and clears its initialized slot. Buffers are borrowed only for this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_new_f64(
    mixtures: usize,
    dimensions: usize,
    output: *mut *mut LiblrhsmmRsGaussianF64,
) -> u32 {
    // SAFETY: The output slot is independent writable storage.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianF64 {
                parameters: GaussianMixtureParameters::new(mixtures, dimensions)?,
            })))
        })
    }
}

/// Copy all four parameter arrays into an independent owner.
/// # Safety
/// Handles are live owners of the declared precision. Descriptor and input storage
/// is initialized and readable; output storage is aligned and independently writable.
/// Writable storage and mutated or released owners require exclusive access.
/// No concurrent mutation/release overlaps reads. Inputs/outputs cannot alias owner
/// storage; input handles in borrowed arrays may repeat. Release consumes unique
/// ownership and clears its initialized slot. Buffers are borrowed only for this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_create_f64(
    parameters: *const LiblrhsmmRsGaussianParametersF64,
    output: *mut *mut LiblrhsmmRsGaussianF64,
) -> u32 {
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Validated output; descriptor follows the input contract.
    let value = match unsafe { input_f64(parameters) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianF64 {
                parameters: value,
            })))
        })
    }
}

/// Replace the complete shape and parameter arrays. Failures preserve the owner.
/// # Safety
/// Handles are live owners of the declared precision. Descriptor and input storage
/// is initialized and readable; output storage is aligned and independently writable.
/// Writable storage and mutated or released owners require exclusive access.
/// No concurrent mutation/release overlaps reads. Inputs/outputs cannot alias owner
/// storage; input handles in borrowed arrays may repeat. Release consumes unique
/// ownership and clears its initialized slot. Buffers are borrowed only for this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_replace_f64(
    handle: *mut LiblrhsmmRsGaussianF64,
    parameters: *const LiblrhsmmRsGaussianParametersF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Exclusive owner and independent borrowed arrays; stage before publishing.
    match unsafe { input_f64(parameters) } {
        Ok(value) => {
            unsafe {
                (*handle).parameters = value;
            }
            0
        }
        Err(status) => status,
    }
}

/// Copy current dimensions and mixture count.
/// # Safety
/// Handles are live owners of the declared precision. Descriptor and input storage
/// is initialized and readable; output storage is aligned and independently writable.
/// Writable storage and mutated or released owners require exclusive access.
/// No concurrent mutation/release overlaps reads. Inputs/outputs cannot alias owner
/// storage; input handles in borrowed arrays may repeat. Release consumes unique
/// ownership and clears its initialized slot. Buffers are borrowed only for this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_layout_f64(
    handle: *const LiblrhsmmRsGaussianF64,
    output: *mut LiblrhsmmRsGaussianLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable owner and independent output.
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsGaussianLayout {
                mixtures: (*handle).parameters.weights.len(),
                dimensions: (*handle).parameters.dimensions,
            })
        })
    }
}

/// Copy a field interval; zero length permits a null buffer.
/// # Safety
/// Handles are live owners of the declared precision. Descriptor and input storage
/// is initialized and readable; output storage is aligned and independently writable.
/// Writable storage and mutated or released owners require exclusive access.
/// No concurrent mutation/release overlaps reads. Inputs/outputs cannot alias owner
/// storage; input handles in borrowed arrays may repeat. Release consumes unique
/// ownership and clears its initialized slot. Buffers are borrowed only for this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_copy_field_f64(
    handle: *const LiblrhsmmRsGaussianF64,
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
    // SAFETY: Live owner, independent buffers, exclusive mutation where required.
    let values = match unsafe { field(&(*handle).parameters, selector) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    let selected = match interval(offset, count, values.len()) {
        Ok(value) => value,
        Err(status) => return status,
    };
    if count != 0 {
        unsafe {
            std::ptr::copy_nonoverlapping(values[selected].as_ptr(), buffer, count);
        }
    }
    0
}

/// Edit a field interval retaining all scalar bits; errors preserve parameters.
/// # Safety
/// Handles are live owners of the declared precision. Descriptor and input storage
/// is initialized and readable; output storage is aligned and independently writable.
/// Writable storage and mutated or released owners require exclusive access.
/// No concurrent mutation/release overlaps reads. Inputs/outputs cannot alias owner
/// storage; input handles in borrowed arrays may repeat. Release consumes unique
/// ownership and clears its initialized slot. Buffers are borrowed only for this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_set_field_f64(
    handle: *mut LiblrhsmmRsGaussianF64,
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
    // SAFETY: Live owner, independent buffers, exclusive mutation where required.
    let values = match unsafe { field_mut(&mut (*handle).parameters, selector) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    let selected = match interval(offset, count, values.len()) {
        Ok(value) => value,
        Err(status) => return status,
    };
    if count != 0 {
        unsafe {
            values[selected].copy_from_slice(std::slice::from_raw_parts(buffer, count));
        }
    }
    0
}

/// Clone all fields into independent owned storage.
/// # Safety
/// Handles are live owners of the declared precision. Descriptor and input storage
/// is initialized and readable; output storage is aligned and independently writable.
/// Writable storage and mutated or released owners require exclusive access.
/// No concurrent mutation/release overlaps reads. Inputs/outputs cannot alias owner
/// storage; input handles in borrowed arrays may repeat. Release consumes unique
/// ownership and clears its initialized slot. Buffers are borrowed only for this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_clone_f64(
    handle: *const LiblrhsmmRsGaussianF64,
    output: *mut *mut LiblrhsmmRsGaussianF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Independent output and readable live owner.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianF64 {
                parameters: (*handle).parameters.clone(),
            })))
        })
    }
}

/// Release a unique owner and clear its slot. Null ownership is a no-op.
/// # Safety
/// Handles are live owners of the declared precision. Descriptor and input storage
/// is initialized and readable; output storage is aligned and independently writable.
/// Writable storage and mutated or released owners require exclusive access.
/// No concurrent mutation/release overlaps reads. Inputs/outputs cannot alias owner
/// storage; input handles in borrowed arrays may repeat. Release consumes unique
/// ownership and clears its initialized slot. Buffers are borrowed only for this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_release_f64(
    handle: *mut *mut LiblrhsmmRsGaussianF64,
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

/// Combine borrowed owners using the unchanged native transformation; inputs are retained.
/// # Safety
/// Handles are live owners of the declared precision. Descriptor and input storage
/// is initialized and readable; output storage is aligned and independently writable.
/// Writable storage and mutated or released owners require exclusive access.
/// No concurrent mutation/release overlaps reads. Inputs/outputs cannot alias owner
/// storage; input handles in borrowed arrays may repeat. Release consumes unique
/// ownership and clears its initialized slot. Buffers are borrowed only for this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_combine_f64(
    handles: *const *const LiblrhsmmRsGaussianF64,
    count: usize,
    output: *mut *mut LiblrhsmmRsGaussianF64,
) -> u32 {
    if let Err(status) = range(output, 1) {
        return status;
    }
    if let Err(status) = range(handles, count) {
        return status;
    }
    // SAFETY: Borrowed initialized handle array. Empty arrays permit null.
    let handles = if count == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(handles, count) }
    };
    for &handle in handles {
        if let Err(status) = range(handle, 1) {
            return status;
        }
    }
    // SAFETY: All handles validated, readable, independent of output storage.
    unsafe {
        scalar_result(output, || {
            let mut values = Vec::new();
            values
                .try_reserve_exact(count)
                .map_err(|_| ModelError("input allocation failed"))?;
            for &handle in handles {
                values.push((*handle).parameters.clone());
            }
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianF64 {
                parameters: GaussianMixtureParameters::combine(&values)?,
            })))
        })
    }
}

/// Combine borrowed owners using the unchanged native transformation; inputs are retained.
/// # Safety
/// Handles are live owners of the declared precision. Descriptor and input storage
/// is initialized and readable; output storage is aligned and independently writable.
/// Writable storage and mutated or released owners require exclusive access.
/// No concurrent mutation/release overlaps reads. Inputs/outputs cannot alias owner
/// storage; input handles in borrowed arrays may repeat. Release consumes unique
/// ownership and clears its initialized slot. Buffers are borrowed only for this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_from_durations_f64(
    handles: *const *const LiblrhsmmRsDurationF64,
    count: usize,
    output: *mut *mut LiblrhsmmRsGaussianF64,
) -> u32 {
    if let Err(status) = range(output, 1) {
        return status;
    }
    if let Err(status) = range(handles, count) {
        return status;
    }
    // SAFETY: Borrowed initialized handle array. Empty arrays permit null.
    let handles = if count == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(handles, count) }
    };
    for &handle in handles {
        if let Err(status) = range(handle, 1) {
            return status;
        }
    }
    // SAFETY: All handles validated, readable, independent of output storage.
    unsafe {
        scalar_result(output, || {
            let mut values = Vec::new();
            values
                .try_reserve_exact(count)
                .map_err(|_| ModelError("input allocation failed"))?;
            for &handle in handles {
                values.push((*handle).parameters.clone());
            }
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianF64 {
                parameters: GaussianMixtureParameters::from_durations(&values)?,
            })))
        })
    }
}

/// Apply native repeated maximum-weight splitting; stage changes before publication.
/// # Safety
/// Handles are live owners of the declared precision. Descriptor and input storage
/// is initialized and readable; output storage is aligned and independently writable.
/// Writable storage and mutated or released owners require exclusive access.
/// No concurrent mutation/release overlaps reads. Inputs/outputs cannot alias owner
/// storage; input handles in borrowed arrays may repeat. Release consumes unique
/// ownership and clears its initialized slot. Buffers are borrowed only for this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_increase_mixtures_f64(
    handle: *mut LiblrhsmmRsGaussianF64,
    count: usize,
    displacement: f64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Exclusive live owner. Stage native splitting so all failures are atomic.
    let result = boundary(|| {
        let mut value = unsafe { (*handle).parameters.clone() };
        value.increase_mixtures(count, displacement)?;
        Ok(value)
    });
    match result {
        Ok(value) => {
            unsafe {
                (*handle).parameters = value;
            }
            0
        }
        Err(status) => status,
    }
}
