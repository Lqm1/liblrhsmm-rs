//! Independent immutable duration preparations and their native cache state.
use super::duration::{
    LiblrhsmmRsDurationF32, LiblrhsmmRsDurationF64, LiblrhsmmRsDurationParametersF32,
    LiblrhsmmRsDurationParametersF64,
};
use super::{range, scalar_result};
use crate::preparation::{OwnedPreparedDurationParameters, PreparationSettings};

unsafe fn copy_cache<T: Copy>(values: &[T], offset: usize, output: *mut T, count: usize) -> u32 {
    if let Err(status) = range(output.cast_const(), count) {
        return status;
    }
    let Some(end) = offset.checked_add(count) else {
        return 2;
    };
    let Some(values) = values.get(offset..end) else {
        return 2;
    };
    if count != 0 {
        // SAFETY: Caller provides independent aligned writable storage for count scalars.
        unsafe {
            std::ptr::copy_nonoverlapping(values.as_ptr(), output, count);
        }
    }
    0
}

/// Complete preparation configuration. Cache length may be zero.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LiblrhsmmRsDurationPreparationF32 {
    pub duration_cache_length: usize,
    pub duration_weight: f32,
    pub temperature: f32,
}
impl From<LiblrhsmmRsDurationPreparationF32> for PreparationSettings<f32> {
    fn from(value: LiblrhsmmRsDurationPreparationF32) -> Self {
        Self {
            duration_cache_length: value.duration_cache_length,
            duration_weight: value.duration_weight,
            temperature: value.temperature,
        }
    }
}
impl From<PreparationSettings<f32>> for LiblrhsmmRsDurationPreparationF32 {
    fn from(value: PreparationSettings<f32>) -> Self {
        Self {
            duration_cache_length: value.duration_cache_length,
            duration_weight: value.duration_weight,
            temperature: value.temperature,
        }
    }
}
/// Independent immutable parameter/cache snapshot. The source owner may be edited or released.
pub struct LiblrhsmmRsPreparedDurationF32 {
    prepared: OwnedPreparedDurationParameters<f32>,
}

/// Write the source preparation defaults: length 300, weight one and temperature one.
/// Errors retain result storage and source parameters.
/// # Safety
/// Nonnull pointers identify aligned valid storage of their declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners created by this precision's interface; release consumes unique ownership.
/// Outputs/descriptors are independent of handle storage and exclusively writable when used as output.
/// Constructor output slots hold no live owner. No operation may overlap release; source reads may
/// not overlap source mutation. Prepared snapshots are immutable and may be read concurrently.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_preparation_default_f32(
    output: *mut LiblrhsmmRsDurationPreparationF32,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    unsafe { scalar_result(output, || Ok(PreparationSettings::<f32>::default().into())) }
}

/// Prepare an independent snapshot with the native cache. Invalid output prevents allocation.
/// Errors retain result storage and source parameters.
/// # Safety
/// Nonnull pointers identify aligned valid storage of their declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners created by this precision's interface; release consumes unique ownership.
/// Outputs/descriptors are independent of handle storage and exclusively writable when used as output.
/// Constructor output slots hold no live owner. No operation may overlap release; source reads may
/// not overlap source mutation. Prepared snapshots are immutable and may be read concurrently.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_prepare_f32(
    handle: *const LiblrhsmmRsDurationF32,
    options: *const LiblrhsmmRsDurationPreparationF32,
    output: *mut *mut LiblrhsmmRsPreparedDurationF32,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(options, 1) {
        return status;
    }
    // SAFETY: Options point to an independent initialized preparation descriptor.
    let options = unsafe { options.read() };
    unsafe {
        scalar_result(output, || {
            let prepared = (*handle).parameters.clone().prepare_owned(options.into())?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsPreparedDurationF32 {
                prepared,
            })))
        })
    }
}

/// Clone parameters and cache into independent storage without recomputation.
/// Errors retain result storage and source parameters.
/// # Safety
/// Nonnull pointers identify aligned valid storage of their declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners created by this precision's interface; release consumes unique ownership.
/// Outputs/descriptors are independent of handle storage and exclusively writable when used as output.
/// Constructor output slots hold no live owner. No operation may overlap release; source reads may
/// not overlap source mutation. Prepared snapshots are immutable and may be read concurrently.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_duration_clone_f32(
    handle: *const LiblrhsmmRsPreparedDurationF32,
    output: *mut *mut LiblrhsmmRsPreparedDurationF32,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsPreparedDurationF32 {
                prepared: (*handle).prepared.clone(),
            })))
        })
    }
}

/// Release parameters and cache, clearing the initialized unique-owner slot. Null owned pointers are a no-op.
/// Errors retain result storage and source parameters.
/// # Safety
/// Nonnull pointers identify aligned valid storage of their declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners created by this precision's interface; release consumes unique ownership.
/// Outputs/descriptors are independent of handle storage and exclusively writable when used as output.
/// Constructor output slots hold no live owner. No operation may overlap release; source reads may
/// not overlap source mutation. Prepared snapshots are immutable and may be read concurrently.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_duration_release_f32(
    handle: *mut *mut LiblrhsmmRsPreparedDurationF32,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle.cast_const(), 1) {
        return status;
    }
    // SAFETY: Initialized exclusive ownership slot, null or a unique live owner.
    let pointer = unsafe { handle.read() };
    if pointer.is_null() {
        return 0;
    }
    if let Err(status) = range(pointer.cast_const(), 1) {
        return status;
    }
    unsafe {
        handle.write(std::ptr::null_mut());
        drop(Box::from_raw(pointer));
    }
    0
}

/// Copy all immutable parameter fields belonging to this cache.
/// Errors retain result storage and source parameters.
/// # Safety
/// Nonnull pointers identify aligned valid storage of their declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners created by this precision's interface; release consumes unique ownership.
/// Outputs/descriptors are independent of handle storage and exclusively writable when used as output.
/// Constructor output slots hold no live owner. No operation may overlap release; source reads may
/// not overlap source mutation. Prepared snapshots are immutable and may be read concurrently.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_duration_parameters_f32(
    handle: *const LiblrhsmmRsPreparedDurationF32,
    output: *mut LiblrhsmmRsDurationParametersF32,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsDurationParametersF32::from(
                (*handle).prepared.parameters(),
            ))
        })
    }
}

/// Copy the exact preparation settings used by this snapshot.
/// Errors retain result storage and source parameters.
/// # Safety
/// Nonnull pointers identify aligned valid storage of their declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners created by this precision's interface; release consumes unique ownership.
/// Outputs/descriptors are independent of handle storage and exclusively writable when used as output.
/// Constructor output slots hold no live owner. No operation may overlap release; source reads may
/// not overlap source mutation. Prepared snapshots are immutable and may be read concurrently.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_duration_options_f32(
    handle: *const LiblrhsmmRsPreparedDurationF32,
    output: *mut LiblrhsmmRsDurationPreparationF32,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || Ok((*handle).prepared.options().into())) }
}

/// Read the cached normalizer before weighting and annealing.
/// Errors retain result storage and source parameters.
/// # Safety
/// Nonnull pointers identify aligned valid storage of their declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners created by this precision's interface; release consumes unique ownership.
/// Outputs/descriptors are independent of handle storage and exclusively writable when used as output.
/// Constructor output slots hold no live owner. No operation may overlap release; source reads may
/// not overlap source mutation. Prepared snapshots are immutable and may be read concurrently.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_duration_log_normalizer_f32(
    handle: *const LiblrhsmmRsPreparedDurationF32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || Ok((*handle).prepared.log_normalizer())) }
}

/// Read the number of stored log probabilities.
/// Errors retain result storage and source parameters.
/// # Safety
/// Nonnull pointers identify aligned valid storage of their declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners created by this precision's interface; release consumes unique ownership.
/// Outputs/descriptors are independent of handle storage and exclusively writable when used as output.
/// Constructor output slots hold no live owner. No operation may overlap release; source reads may
/// not overlap source mutation. Prepared snapshots are immutable and may be read concurrently.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_duration_cache_length_f32(
    handle: *const LiblrhsmmRsPreparedDurationF32,
    output: *mut usize,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok((*handle).prepared.cached_log_probabilities().len())
        })
    }
}

/// Copy a checked cache interval. Empty copies permit null output at any valid boundary. Errors retain output.
/// Errors retain result storage and source parameters.
/// # Safety
/// Nonnull pointers identify aligned valid storage of their declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners created by this precision's interface; release consumes unique ownership.
/// Outputs/descriptors are independent of handle storage and exclusively writable when used as output.
/// Constructor output slots hold no live owner. No operation may overlap release; source reads may
/// not overlap source mutation. Prepared snapshots are immutable and may be read concurrently.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_duration_cache_copy_f32(
    handle: *const LiblrhsmmRsPreparedDurationF32,
    offset: usize,
    output: *mut f32,
    count: usize,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        copy_cache(
            (*handle).prepared.cached_log_probabilities(),
            offset,
            output,
            count,
        )
    }
}

/// Use the native prepared lookup within its range and cached-normalizer evaluation beyond it.
/// Errors retain result storage and source parameters.
/// # Safety
/// Nonnull pointers identify aligned valid storage of their declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners created by this precision's interface; release consumes unique ownership.
/// Outputs/descriptors are independent of handle storage and exclusively writable when used as output.
/// Constructor output slots hold no live owner. No operation may overlap release; source reads may
/// not overlap source mutation. Prepared snapshots are immutable and may be read concurrently.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_duration_log_probability_f32(
    handle: *const LiblrhsmmRsPreparedDurationF32,
    frames: usize,
    output: *mut f32,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || (*handle).prepared.log_probability(frames)) }
}

/// Evaluate the native prepared density, returning f64 in both precisions.
/// Errors retain result storage and source parameters.
/// # Safety
/// Nonnull pointers identify aligned valid storage of their declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners created by this precision's interface; release consumes unique ownership.
/// Outputs/descriptors are independent of handle storage and exclusively writable when used as output.
/// Constructor output slots hold no live owner. No operation may overlap release; source reads may
/// not overlap source mutation. Prepared snapshots are immutable and may be read concurrently.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_duration_probability_f32(
    handle: *const LiblrhsmmRsPreparedDurationF32,
    frames: usize,
    output: *mut f64,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || (*handle).prepared.probability(frames)) }
}

/// Complete preparation configuration. Cache length may be zero.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LiblrhsmmRsDurationPreparationF64 {
    pub duration_cache_length: usize,
    pub duration_weight: f64,
    pub temperature: f64,
}
impl From<LiblrhsmmRsDurationPreparationF64> for PreparationSettings<f64> {
    fn from(value: LiblrhsmmRsDurationPreparationF64) -> Self {
        Self {
            duration_cache_length: value.duration_cache_length,
            duration_weight: value.duration_weight,
            temperature: value.temperature,
        }
    }
}
impl From<PreparationSettings<f64>> for LiblrhsmmRsDurationPreparationF64 {
    fn from(value: PreparationSettings<f64>) -> Self {
        Self {
            duration_cache_length: value.duration_cache_length,
            duration_weight: value.duration_weight,
            temperature: value.temperature,
        }
    }
}
/// Independent immutable parameter/cache snapshot. The source owner may be edited or released.
pub struct LiblrhsmmRsPreparedDurationF64 {
    prepared: OwnedPreparedDurationParameters<f64>,
}

/// Write the source preparation defaults: length 300, weight one and temperature one.
/// Errors retain result storage and source parameters.
/// # Safety
/// Nonnull pointers identify aligned valid storage of their declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners created by this precision's interface; release consumes unique ownership.
/// Outputs/descriptors are independent of handle storage and exclusively writable when used as output.
/// Constructor output slots hold no live owner. No operation may overlap release; source reads may
/// not overlap source mutation. Prepared snapshots are immutable and may be read concurrently.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_preparation_default_f64(
    output: *mut LiblrhsmmRsDurationPreparationF64,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    unsafe { scalar_result(output, || Ok(PreparationSettings::<f64>::default().into())) }
}

/// Prepare an independent snapshot with the native cache. Invalid output prevents allocation.
/// Errors retain result storage and source parameters.
/// # Safety
/// Nonnull pointers identify aligned valid storage of their declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners created by this precision's interface; release consumes unique ownership.
/// Outputs/descriptors are independent of handle storage and exclusively writable when used as output.
/// Constructor output slots hold no live owner. No operation may overlap release; source reads may
/// not overlap source mutation. Prepared snapshots are immutable and may be read concurrently.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_prepare_f64(
    handle: *const LiblrhsmmRsDurationF64,
    options: *const LiblrhsmmRsDurationPreparationF64,
    output: *mut *mut LiblrhsmmRsPreparedDurationF64,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(options, 1) {
        return status;
    }
    // SAFETY: Options point to an independent initialized preparation descriptor.
    let options = unsafe { options.read() };
    unsafe {
        scalar_result(output, || {
            let prepared = (*handle).parameters.clone().prepare_owned(options.into())?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsPreparedDurationF64 {
                prepared,
            })))
        })
    }
}

/// Clone parameters and cache into independent storage without recomputation.
/// Errors retain result storage and source parameters.
/// # Safety
/// Nonnull pointers identify aligned valid storage of their declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners created by this precision's interface; release consumes unique ownership.
/// Outputs/descriptors are independent of handle storage and exclusively writable when used as output.
/// Constructor output slots hold no live owner. No operation may overlap release; source reads may
/// not overlap source mutation. Prepared snapshots are immutable and may be read concurrently.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_duration_clone_f64(
    handle: *const LiblrhsmmRsPreparedDurationF64,
    output: *mut *mut LiblrhsmmRsPreparedDurationF64,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsPreparedDurationF64 {
                prepared: (*handle).prepared.clone(),
            })))
        })
    }
}

/// Release parameters and cache, clearing the initialized unique-owner slot. Null owned pointers are a no-op.
/// Errors retain result storage and source parameters.
/// # Safety
/// Nonnull pointers identify aligned valid storage of their declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners created by this precision's interface; release consumes unique ownership.
/// Outputs/descriptors are independent of handle storage and exclusively writable when used as output.
/// Constructor output slots hold no live owner. No operation may overlap release; source reads may
/// not overlap source mutation. Prepared snapshots are immutable and may be read concurrently.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_duration_release_f64(
    handle: *mut *mut LiblrhsmmRsPreparedDurationF64,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle.cast_const(), 1) {
        return status;
    }
    // SAFETY: Initialized exclusive ownership slot, null or a unique live owner.
    let pointer = unsafe { handle.read() };
    if pointer.is_null() {
        return 0;
    }
    if let Err(status) = range(pointer.cast_const(), 1) {
        return status;
    }
    unsafe {
        handle.write(std::ptr::null_mut());
        drop(Box::from_raw(pointer));
    }
    0
}

/// Copy all immutable parameter fields belonging to this cache.
/// Errors retain result storage and source parameters.
/// # Safety
/// Nonnull pointers identify aligned valid storage of their declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners created by this precision's interface; release consumes unique ownership.
/// Outputs/descriptors are independent of handle storage and exclusively writable when used as output.
/// Constructor output slots hold no live owner. No operation may overlap release; source reads may
/// not overlap source mutation. Prepared snapshots are immutable and may be read concurrently.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_duration_parameters_f64(
    handle: *const LiblrhsmmRsPreparedDurationF64,
    output: *mut LiblrhsmmRsDurationParametersF64,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsDurationParametersF64::from(
                (*handle).prepared.parameters(),
            ))
        })
    }
}

/// Copy the exact preparation settings used by this snapshot.
/// Errors retain result storage and source parameters.
/// # Safety
/// Nonnull pointers identify aligned valid storage of their declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners created by this precision's interface; release consumes unique ownership.
/// Outputs/descriptors are independent of handle storage and exclusively writable when used as output.
/// Constructor output slots hold no live owner. No operation may overlap release; source reads may
/// not overlap source mutation. Prepared snapshots are immutable and may be read concurrently.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_duration_options_f64(
    handle: *const LiblrhsmmRsPreparedDurationF64,
    output: *mut LiblrhsmmRsDurationPreparationF64,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || Ok((*handle).prepared.options().into())) }
}

/// Read the cached normalizer before weighting and annealing.
/// Errors retain result storage and source parameters.
/// # Safety
/// Nonnull pointers identify aligned valid storage of their declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners created by this precision's interface; release consumes unique ownership.
/// Outputs/descriptors are independent of handle storage and exclusively writable when used as output.
/// Constructor output slots hold no live owner. No operation may overlap release; source reads may
/// not overlap source mutation. Prepared snapshots are immutable and may be read concurrently.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_duration_log_normalizer_f64(
    handle: *const LiblrhsmmRsPreparedDurationF64,
    output: *mut f64,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || Ok((*handle).prepared.log_normalizer())) }
}

/// Read the number of stored log probabilities.
/// Errors retain result storage and source parameters.
/// # Safety
/// Nonnull pointers identify aligned valid storage of their declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners created by this precision's interface; release consumes unique ownership.
/// Outputs/descriptors are independent of handle storage and exclusively writable when used as output.
/// Constructor output slots hold no live owner. No operation may overlap release; source reads may
/// not overlap source mutation. Prepared snapshots are immutable and may be read concurrently.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_duration_cache_length_f64(
    handle: *const LiblrhsmmRsPreparedDurationF64,
    output: *mut usize,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok((*handle).prepared.cached_log_probabilities().len())
        })
    }
}

/// Copy a checked cache interval. Empty copies permit null output at any valid boundary. Errors retain output.
/// Errors retain result storage and source parameters.
/// # Safety
/// Nonnull pointers identify aligned valid storage of their declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners created by this precision's interface; release consumes unique ownership.
/// Outputs/descriptors are independent of handle storage and exclusively writable when used as output.
/// Constructor output slots hold no live owner. No operation may overlap release; source reads may
/// not overlap source mutation. Prepared snapshots are immutable and may be read concurrently.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_duration_cache_copy_f64(
    handle: *const LiblrhsmmRsPreparedDurationF64,
    offset: usize,
    output: *mut f64,
    count: usize,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        copy_cache(
            (*handle).prepared.cached_log_probabilities(),
            offset,
            output,
            count,
        )
    }
}

/// Use the native prepared lookup within its range and cached-normalizer evaluation beyond it.
/// Errors retain result storage and source parameters.
/// # Safety
/// Nonnull pointers identify aligned valid storage of their declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners created by this precision's interface; release consumes unique ownership.
/// Outputs/descriptors are independent of handle storage and exclusively writable when used as output.
/// Constructor output slots hold no live owner. No operation may overlap release; source reads may
/// not overlap source mutation. Prepared snapshots are immutable and may be read concurrently.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_duration_log_probability_f64(
    handle: *const LiblrhsmmRsPreparedDurationF64,
    frames: usize,
    output: *mut f64,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || (*handle).prepared.log_probability(frames)) }
}

/// Evaluate the native prepared density, returning f64 in both precisions.
/// Errors retain result storage and source parameters.
/// # Safety
/// Nonnull pointers identify aligned valid storage of their declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners created by this precision's interface; release consumes unique ownership.
/// Outputs/descriptors are independent of handle storage and exclusively writable when used as output.
/// Constructor output slots hold no live owner. No operation may overlap release; source reads may
/// not overlap source mutation. Prepared snapshots are immutable and may be read concurrently.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_duration_probability_f64(
    handle: *const LiblrhsmmRsPreparedDurationF64,
    frames: usize,
    output: *mut f64,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || (*handle).prepared.probability(frames)) }
}
