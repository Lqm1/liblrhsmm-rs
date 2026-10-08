//! Immutable prepared Gaussian snapshots and unchanged native density kernels.
use super::gaussian::storage_range;
use super::{LiblrhsmmRsGaussianF32, LiblrhsmmRsGaussianF64};
use super::{range, scalar_result};
use crate::probability::OwnedPreparedGaussianParameters;

/// Independent immutable Gaussian parameters with matching normalizers.
#[derive(Debug, Clone)]
pub struct LiblrhsmmRsPreparedGaussianF32 {
    parameters: OwnedPreparedGaussianParameters<f32>,
}

/// Copy and prepare an immutable snapshot; later source edits or release cannot invalidate it.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable, aligned storage; outputs identify independent aligned writable storage.
/// No concurrent mutation or release overlaps reads. All outputs and buffers are
/// independent of owner storage. Release consumes exclusive ownership through an
/// initialized writable slot. Observation and output arrays live for this call only.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_prepare_f32(
    handle: *const LiblrhsmmRsGaussianF32,
    output: *mut *mut LiblrhsmmRsPreparedGaussianF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable source and independent output; retain no source borrow.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsPreparedGaussianF32 {
                parameters: (*handle).parameters.clone().prepare_owned()?,
            })))
        })
    }
}

/// Copy the complete parameters into a new editable Gaussian owner.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable, aligned storage; outputs identify independent aligned writable storage.
/// No concurrent mutation or release overlaps reads. All outputs and buffers are
/// independent of owner storage. Release consumes exclusive ownership through an
/// initialized writable slot. Observation and output arrays live for this call only.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_gaussian_parameters_f32(
    handle: *const LiblrhsmmRsPreparedGaussianF32,
    output: *mut *mut LiblrhsmmRsGaussianF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live immutable owner; output receives independently owned parameters.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianF32 {
                parameters: (*handle).parameters.parameters().clone(),
            })))
        })
    }
}

/// Clone both parameters and cached normalizers independently.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable, aligned storage; outputs identify independent aligned writable storage.
/// No concurrent mutation or release overlaps reads. All outputs and buffers are
/// independent of owner storage. Release consumes exclusive ownership through an
/// initialized writable slot. Observation and output arrays live for this call only.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_gaussian_clone_f32(
    handle: *const LiblrhsmmRsPreparedGaussianF32,
    output: *mut *mut LiblrhsmmRsPreparedGaussianF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Immutable live owner and independent output slot.
    unsafe { scalar_result(output, || Ok(Box::into_raw(Box::new((*handle).clone())))) }
}

/// Release and clear unique ownership. Null ownership is a no-op.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable, aligned storage; outputs identify independent aligned writable storage.
/// No concurrent mutation or release overlaps reads. All outputs and buffers are
/// independent of owner storage. Release consumes exclusive ownership through an
/// initialized writable slot. Observation and output arrays live for this call only.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_gaussian_release_f32(
    handle: *mut *mut LiblrhsmmRsPreparedGaussianF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Initialized exclusive ownership slot, null or a unique live owner.
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

/// Return the number of cached component normalizers.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable, aligned storage; outputs identify independent aligned writable storage.
/// No concurrent mutation or release overlaps reads. All outputs and buffers are
/// independent of owner storage. Release consumes exclusive ownership through an
/// initialized writable slot. Observation and output arrays live for this call only.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_gaussian_cache_length_f32(
    handle: *const LiblrhsmmRsPreparedGaussianF32,
    output: *mut usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable owner and independent scalar output.
    unsafe {
        scalar_result(output, || {
            Ok((*handle).parameters.cached_log_normalizers().len())
        })
    }
}

/// Copy component normalizers before the density factor of one half; errors preserve output.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable, aligned storage; outputs identify independent aligned writable storage.
/// No concurrent mutation or release overlaps reads. All outputs and buffers are
/// independent of owner storage. Release consumes exclusive ownership through an
/// initialized writable slot. Observation and output arrays live for this call only.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_gaussian_cache_copy_f32(
    handle: *const LiblrhsmmRsPreparedGaussianF32,
    offset: usize,
    output: *mut f32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = storage_range(output, count) {
        return status;
    }
    // SAFETY: Immutable live owner; the cache is borrowed only during this call.
    let values = unsafe { (*handle).parameters.cached_log_normalizers() };
    let end = match offset.checked_add(count) {
        Some(end) => end,
        None => return 2,
    };
    let selected = match values.get(offset..end) {
        Some(values) => values,
        None => return 2,
    };
    if count != 0 {
        unsafe {
            std::ptr::copy_nonoverlapping(selected.as_ptr(), output, count);
        }
    }
    0
}

/// Evaluate the unchanged native prepared kernel. Scalar output is unchanged on failure.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable, aligned storage; outputs identify independent aligned writable storage.
/// No concurrent mutation or release overlaps reads. All outputs and buffers are
/// independent of owner storage. Release consumes exclusive ownership through an
/// initialized writable slot. Observation and output arrays live for this call only.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_gaussian_component_log_probability_f32(
    handle: *const LiblrhsmmRsPreparedGaussianF32,
    observation: *const f32,
    dimensions: usize,
    component: usize,
    output: *mut f32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = storage_range(observation, dimensions) {
        return status;
    }
    // SAFETY: Borrowed initialized observation, immutable owner and independent output.
    let observation = if dimensions == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(observation, dimensions) }
    };
    unsafe {
        scalar_result(output, || {
            (*handle)
                .parameters
                .component_log_probability(observation, component)
        })
    }
}

/// Evaluate the unchanged native prepared kernel. Scalar output is unchanged on failure.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable, aligned storage; outputs identify independent aligned writable storage.
/// No concurrent mutation or release overlaps reads. All outputs and buffers are
/// independent of owner storage. Release consumes exclusive ownership through an
/// initialized writable slot. Observation and output arrays live for this call only.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_gaussian_component_probability_f32(
    handle: *const LiblrhsmmRsPreparedGaussianF32,
    observation: *const f32,
    dimensions: usize,
    component: usize,
    output: *mut f64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = storage_range(observation, dimensions) {
        return status;
    }
    // SAFETY: Borrowed initialized observation, immutable owner and independent output.
    let observation = if dimensions == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(observation, dimensions) }
    };
    unsafe {
        scalar_result(output, || {
            (*handle)
                .parameters
                .component_probability(observation, component)
        })
    }
}

/// Evaluate the unchanged native prepared kernel. Scalar output is unchanged on failure.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable, aligned storage; outputs identify independent aligned writable storage.
/// No concurrent mutation or release overlaps reads. All outputs and buffers are
/// independent of owner storage. Release consumes exclusive ownership through an
/// initialized writable slot. Observation and output arrays live for this call only.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_gaussian_log_probability_f32(
    handle: *const LiblrhsmmRsPreparedGaussianF32,
    observation: *const f32,
    dimensions: usize,
    temperature: f32,
    output: *mut f32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = storage_range(observation, dimensions) {
        return status;
    }
    // SAFETY: Borrowed initialized observation, immutable owner and independent output.
    let observation = if dimensions == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(observation, dimensions) }
    };
    unsafe {
        scalar_result(output, || {
            (*handle)
                .parameters
                .log_probability(observation, temperature)
        })
    }
}

/// Evaluate the unchanged native prepared kernel. Scalar output is unchanged on failure.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable, aligned storage; outputs identify independent aligned writable storage.
/// No concurrent mutation or release overlaps reads. All outputs and buffers are
/// independent of owner storage. Release consumes exclusive ownership through an
/// initialized writable slot. Observation and output arrays live for this call only.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_gaussian_probability_f32(
    handle: *const LiblrhsmmRsPreparedGaussianF32,
    observation: *const f32,
    dimensions: usize,
    temperature: f32,
    output: *mut f64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = storage_range(observation, dimensions) {
        return status;
    }
    // SAFETY: Borrowed initialized observation, immutable owner and independent output.
    let observation = if dimensions == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(observation, dimensions) }
    };
    unsafe {
        scalar_result(output, || {
            (*handle).parameters.probability(observation, temperature)
        })
    }
}
/// Independent immutable Gaussian parameters with matching normalizers.
#[derive(Debug, Clone)]
pub struct LiblrhsmmRsPreparedGaussianF64 {
    parameters: OwnedPreparedGaussianParameters<f64>,
}

/// Copy and prepare an immutable snapshot; later source edits or release cannot invalidate it.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable, aligned storage; outputs identify independent aligned writable storage.
/// No concurrent mutation or release overlaps reads. All outputs and buffers are
/// independent of owner storage. Release consumes exclusive ownership through an
/// initialized writable slot. Observation and output arrays live for this call only.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_prepare_f64(
    handle: *const LiblrhsmmRsGaussianF64,
    output: *mut *mut LiblrhsmmRsPreparedGaussianF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable source and independent output; retain no source borrow.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsPreparedGaussianF64 {
                parameters: (*handle).parameters.clone().prepare_owned()?,
            })))
        })
    }
}

/// Copy the complete parameters into a new editable Gaussian owner.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable, aligned storage; outputs identify independent aligned writable storage.
/// No concurrent mutation or release overlaps reads. All outputs and buffers are
/// independent of owner storage. Release consumes exclusive ownership through an
/// initialized writable slot. Observation and output arrays live for this call only.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_gaussian_parameters_f64(
    handle: *const LiblrhsmmRsPreparedGaussianF64,
    output: *mut *mut LiblrhsmmRsGaussianF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live immutable owner; output receives independently owned parameters.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianF64 {
                parameters: (*handle).parameters.parameters().clone(),
            })))
        })
    }
}

/// Clone both parameters and cached normalizers independently.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable, aligned storage; outputs identify independent aligned writable storage.
/// No concurrent mutation or release overlaps reads. All outputs and buffers are
/// independent of owner storage. Release consumes exclusive ownership through an
/// initialized writable slot. Observation and output arrays live for this call only.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_gaussian_clone_f64(
    handle: *const LiblrhsmmRsPreparedGaussianF64,
    output: *mut *mut LiblrhsmmRsPreparedGaussianF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Immutable live owner and independent output slot.
    unsafe { scalar_result(output, || Ok(Box::into_raw(Box::new((*handle).clone())))) }
}

/// Release and clear unique ownership. Null ownership is a no-op.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable, aligned storage; outputs identify independent aligned writable storage.
/// No concurrent mutation or release overlaps reads. All outputs and buffers are
/// independent of owner storage. Release consumes exclusive ownership through an
/// initialized writable slot. Observation and output arrays live for this call only.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_gaussian_release_f64(
    handle: *mut *mut LiblrhsmmRsPreparedGaussianF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Initialized exclusive ownership slot, null or a unique live owner.
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

/// Return the number of cached component normalizers.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable, aligned storage; outputs identify independent aligned writable storage.
/// No concurrent mutation or release overlaps reads. All outputs and buffers are
/// independent of owner storage. Release consumes exclusive ownership through an
/// initialized writable slot. Observation and output arrays live for this call only.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_gaussian_cache_length_f64(
    handle: *const LiblrhsmmRsPreparedGaussianF64,
    output: *mut usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable owner and independent scalar output.
    unsafe {
        scalar_result(output, || {
            Ok((*handle).parameters.cached_log_normalizers().len())
        })
    }
}

/// Copy component normalizers before the density factor of one half; errors preserve output.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable, aligned storage; outputs identify independent aligned writable storage.
/// No concurrent mutation or release overlaps reads. All outputs and buffers are
/// independent of owner storage. Release consumes exclusive ownership through an
/// initialized writable slot. Observation and output arrays live for this call only.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_gaussian_cache_copy_f64(
    handle: *const LiblrhsmmRsPreparedGaussianF64,
    offset: usize,
    output: *mut f64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = storage_range(output, count) {
        return status;
    }
    // SAFETY: Immutable live owner; the cache is borrowed only during this call.
    let values = unsafe { (*handle).parameters.cached_log_normalizers() };
    let end = match offset.checked_add(count) {
        Some(end) => end,
        None => return 2,
    };
    let selected = match values.get(offset..end) {
        Some(values) => values,
        None => return 2,
    };
    if count != 0 {
        unsafe {
            std::ptr::copy_nonoverlapping(selected.as_ptr(), output, count);
        }
    }
    0
}

/// Evaluate the unchanged native prepared kernel. Scalar output is unchanged on failure.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable, aligned storage; outputs identify independent aligned writable storage.
/// No concurrent mutation or release overlaps reads. All outputs and buffers are
/// independent of owner storage. Release consumes exclusive ownership through an
/// initialized writable slot. Observation and output arrays live for this call only.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_gaussian_component_log_probability_f64(
    handle: *const LiblrhsmmRsPreparedGaussianF64,
    observation: *const f64,
    dimensions: usize,
    component: usize,
    output: *mut f64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = storage_range(observation, dimensions) {
        return status;
    }
    // SAFETY: Borrowed initialized observation, immutable owner and independent output.
    let observation = if dimensions == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(observation, dimensions) }
    };
    unsafe {
        scalar_result(output, || {
            (*handle)
                .parameters
                .component_log_probability(observation, component)
        })
    }
}

/// Evaluate the unchanged native prepared kernel. Scalar output is unchanged on failure.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable, aligned storage; outputs identify independent aligned writable storage.
/// No concurrent mutation or release overlaps reads. All outputs and buffers are
/// independent of owner storage. Release consumes exclusive ownership through an
/// initialized writable slot. Observation and output arrays live for this call only.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_gaussian_component_probability_f64(
    handle: *const LiblrhsmmRsPreparedGaussianF64,
    observation: *const f64,
    dimensions: usize,
    component: usize,
    output: *mut f64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = storage_range(observation, dimensions) {
        return status;
    }
    // SAFETY: Borrowed initialized observation, immutable owner and independent output.
    let observation = if dimensions == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(observation, dimensions) }
    };
    unsafe {
        scalar_result(output, || {
            (*handle)
                .parameters
                .component_probability(observation, component)
        })
    }
}

/// Evaluate the unchanged native prepared kernel. Scalar output is unchanged on failure.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable, aligned storage; outputs identify independent aligned writable storage.
/// No concurrent mutation or release overlaps reads. All outputs and buffers are
/// independent of owner storage. Release consumes exclusive ownership through an
/// initialized writable slot. Observation and output arrays live for this call only.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_gaussian_log_probability_f64(
    handle: *const LiblrhsmmRsPreparedGaussianF64,
    observation: *const f64,
    dimensions: usize,
    temperature: f64,
    output: *mut f64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = storage_range(observation, dimensions) {
        return status;
    }
    // SAFETY: Borrowed initialized observation, immutable owner and independent output.
    let observation = if dimensions == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(observation, dimensions) }
    };
    unsafe {
        scalar_result(output, || {
            (*handle)
                .parameters
                .log_probability(observation, temperature)
        })
    }
}

/// Evaluate the unchanged native prepared kernel. Scalar output is unchanged on failure.
/// # Safety
/// Handles are live owners of the declared precision. Inputs identify initialized,
/// readable, aligned storage; outputs identify independent aligned writable storage.
/// No concurrent mutation or release overlaps reads. All outputs and buffers are
/// independent of owner storage. Release consumes exclusive ownership through an
/// initialized writable slot. Observation and output arrays live for this call only.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_gaussian_probability_f64(
    handle: *const LiblrhsmmRsPreparedGaussianF64,
    observation: *const f64,
    dimensions: usize,
    temperature: f64,
    output: *mut f64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = storage_range(observation, dimensions) {
        return status;
    }
    // SAFETY: Borrowed initialized observation, immutable owner and independent output.
    let observation = if dimensions == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(observation, dimensions) }
    };
    unsafe {
        scalar_result(output, || {
            (*handle).parameters.probability(observation, temperature)
        })
    }
}
