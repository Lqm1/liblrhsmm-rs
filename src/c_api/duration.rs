//! Owned duration parameters and native density operations.
use super::{range, scalar_result};
use crate::model::DurationParameters;

/// Complete native duration parameters. Flat-start and arbitrary scalar bits are retained.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LiblrhsmmRsDurationParametersF32 {
    pub mean: f32,
    pub variance: f32,
    pub minimum: i32,
    pub maximum: i32,
    pub fixed_mean: i32,
    pub variance_floor: f32,
}
/// Owned duration. Mutation and release require exclusive access.
pub struct LiblrhsmmRsDurationF32 {
    pub(super) parameters: DurationParameters<f32>,
}
impl From<LiblrhsmmRsDurationParametersF32> for DurationParameters<f32> {
    fn from(value: LiblrhsmmRsDurationParametersF32) -> Self {
        Self {
            mean: value.mean,
            variance: value.variance,
            minimum: value.minimum,
            maximum: value.maximum,
            fixed_mean: value.fixed_mean,
            variance_floor: value.variance_floor,
        }
    }
}
impl From<&DurationParameters<f32>> for LiblrhsmmRsDurationParametersF32 {
    fn from(value: &DurationParameters<f32>) -> Self {
        Self {
            mean: value.mean,
            variance: value.variance,
            minimum: value.minimum,
            maximum: value.maximum,
            fixed_mean: value.fixed_mean,
            variance_floor: value.variance_floor,
        }
    }
}

/// Write the complete native flat-start descriptor.
/// # Safety
/// Handles must be live owners from this precision's interface. Release consumes unique ownership.
/// All nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Outputs and descriptors must be independent of handle storage. Writable storage and mutation
/// require exclusive access; no concurrent mutation or release may overlap reads.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_default_f32(
    output: *mut LiblrhsmmRsDurationParametersF32,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsDurationParametersF32::from(
                &DurationParameters::<f32>::default(),
            ))
        })
    }
}

/// Copy every descriptor field into a new independently owned duration.
/// # Safety
/// Handles must be live owners from this precision's interface. Release consumes unique ownership.
/// All nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Outputs and descriptors must be independent of handle storage. Writable storage and mutation
/// require exclusive access; no concurrent mutation or release may overlap reads.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_create_f32(
    parameters: *const LiblrhsmmRsDurationParametersF32,
    output: *mut *mut LiblrhsmmRsDurationF32,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(parameters, 1) {
        return status;
    }
    // SAFETY: Parameters point to an initialized descriptor for this call.
    let parameters = unsafe { parameters.read() };
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsDurationF32 {
                parameters: parameters.into(),
            })))
        })
    }
}

/// Copy all current parameter fields. Outputs remain unchanged on error.
/// # Safety
/// Handles must be live owners from this precision's interface. Release consumes unique ownership.
/// All nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Outputs and descriptors must be independent of handle storage. Writable storage and mutation
/// require exclusive access; no concurrent mutation or release may overlap reads.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_get_f32(
    handle: *const LiblrhsmmRsDurationF32,
    output: *mut LiblrhsmmRsDurationParametersF32,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsDurationParametersF32::from(
                &(*handle).parameters,
            ))
        })
    }
}

/// Replace all fields without numeric restrictions; density calls validate their inputs.
/// # Safety
/// Handles must be live owners from this precision's interface. Release consumes unique ownership.
/// All nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Outputs and descriptors must be independent of handle storage. Writable storage and mutation
/// require exclusive access; no concurrent mutation or release may overlap reads.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_set_f32(
    handle: *mut LiblrhsmmRsDurationF32,
    parameters: *const LiblrhsmmRsDurationParametersF32,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle.cast_const(), 1) {
        return status;
    }
    if let Err(status) = range(parameters, 1) {
        return status;
    }
    // SAFETY: Independent readable descriptor and exclusive live handle.
    let value = unsafe { parameters.read() };
    unsafe {
        (*handle).parameters = value.into();
    }
    0
}

/// Create an independent parameter copy. Outputs remain unchanged on error.
/// # Safety
/// Handles must be live owners from this precision's interface. Release consumes unique ownership.
/// All nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Outputs and descriptors must be independent of handle storage. Writable storage and mutation
/// require exclusive access; no concurrent mutation or release may overlap reads.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_clone_f32(
    handle: *const LiblrhsmmRsDurationF32,
    output: *mut *mut LiblrhsmmRsDurationF32,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsDurationF32 {
                parameters: (*handle).parameters.clone(),
            })))
        })
    }
}

/// Release and clear an initialized ownership slot. A null owned handle is a no-op.
/// # Safety
/// Handles must be live owners from this precision's interface. Release consumes unique ownership.
/// All nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Outputs and descriptors must be independent of handle storage. Writable storage and mutation
/// require exclusive access; no concurrent mutation or release may overlap reads.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_release_f32(
    handle: *mut *mut LiblrhsmmRsDurationF32,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle.cast_const(), 1) {
        return status;
    }
    // SAFETY: Initialized exclusive ownership slot, null or a uniquely owned handle.
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

/// Evaluate the native duration log density with selected-precision scalar frames.
/// # Safety
/// Handles must be live owners from this precision's interface. Release consumes unique ownership.
/// All nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Outputs and descriptors must be independent of handle storage. Writable storage and mutation
/// require exclusive access; no concurrent mutation or release may overlap reads.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_log_probability_f32(
    handle: *const LiblrhsmmRsDurationF32,
    frames: f32,
    weight: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            (*handle).parameters.log_probability(frames, weight)
        })
    }
}

/// Evaluate the native annealed log density. Minimum exclusion remains active at zero temperature.
/// # Safety
/// Handles must be live owners from this precision's interface. Release consumes unique ownership.
/// All nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Outputs and descriptors must be independent of handle storage. Writable storage and mutation
/// require exclusive access; no concurrent mutation or release may overlap reads.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_annealed_log_probability_f32(
    handle: *const LiblrhsmmRsDurationF32,
    frames: usize,
    weight: f32,
    temperature: f32,
    output: *mut f32,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            (*handle)
                .parameters
                .annealed_log_probability(frames, weight, temperature)
        })
    }
}

/// Evaluate the native density, returning its f64 result in both precisions. Maximum constrains search only.
/// # Safety
/// Handles must be live owners from this precision's interface. Release consumes unique ownership.
/// All nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Outputs and descriptors must be independent of handle storage. Writable storage and mutation
/// require exclusive access; no concurrent mutation or release may overlap reads.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_probability_f32(
    handle: *const LiblrhsmmRsDurationF32,
    frames: usize,
    weight: f32,
    temperature: f32,
    output: *mut f64,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            (*handle)
                .parameters
                .probability(frames, weight, temperature)
        })
    }
}

/// Complete native duration parameters. Flat-start and arbitrary scalar bits are retained.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LiblrhsmmRsDurationParametersF64 {
    pub mean: f64,
    pub variance: f64,
    pub minimum: i32,
    pub maximum: i32,
    pub fixed_mean: i32,
    pub variance_floor: f64,
}
/// Owned duration. Mutation and release require exclusive access.
pub struct LiblrhsmmRsDurationF64 {
    pub(super) parameters: DurationParameters<f64>,
}
impl From<LiblrhsmmRsDurationParametersF64> for DurationParameters<f64> {
    fn from(value: LiblrhsmmRsDurationParametersF64) -> Self {
        Self {
            mean: value.mean,
            variance: value.variance,
            minimum: value.minimum,
            maximum: value.maximum,
            fixed_mean: value.fixed_mean,
            variance_floor: value.variance_floor,
        }
    }
}
impl From<&DurationParameters<f64>> for LiblrhsmmRsDurationParametersF64 {
    fn from(value: &DurationParameters<f64>) -> Self {
        Self {
            mean: value.mean,
            variance: value.variance,
            minimum: value.minimum,
            maximum: value.maximum,
            fixed_mean: value.fixed_mean,
            variance_floor: value.variance_floor,
        }
    }
}

/// Write the complete native flat-start descriptor.
/// # Safety
/// Handles must be live owners from this precision's interface. Release consumes unique ownership.
/// All nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Outputs and descriptors must be independent of handle storage. Writable storage and mutation
/// require exclusive access; no concurrent mutation or release may overlap reads.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_default_f64(
    output: *mut LiblrhsmmRsDurationParametersF64,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsDurationParametersF64::from(
                &DurationParameters::<f64>::default(),
            ))
        })
    }
}

/// Copy every descriptor field into a new independently owned duration.
/// # Safety
/// Handles must be live owners from this precision's interface. Release consumes unique ownership.
/// All nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Outputs and descriptors must be independent of handle storage. Writable storage and mutation
/// require exclusive access; no concurrent mutation or release may overlap reads.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_create_f64(
    parameters: *const LiblrhsmmRsDurationParametersF64,
    output: *mut *mut LiblrhsmmRsDurationF64,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(parameters, 1) {
        return status;
    }
    // SAFETY: Parameters point to an initialized descriptor for this call.
    let parameters = unsafe { parameters.read() };
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsDurationF64 {
                parameters: parameters.into(),
            })))
        })
    }
}

/// Copy all current parameter fields. Outputs remain unchanged on error.
/// # Safety
/// Handles must be live owners from this precision's interface. Release consumes unique ownership.
/// All nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Outputs and descriptors must be independent of handle storage. Writable storage and mutation
/// require exclusive access; no concurrent mutation or release may overlap reads.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_get_f64(
    handle: *const LiblrhsmmRsDurationF64,
    output: *mut LiblrhsmmRsDurationParametersF64,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsDurationParametersF64::from(
                &(*handle).parameters,
            ))
        })
    }
}

/// Replace all fields without numeric restrictions; density calls validate their inputs.
/// # Safety
/// Handles must be live owners from this precision's interface. Release consumes unique ownership.
/// All nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Outputs and descriptors must be independent of handle storage. Writable storage and mutation
/// require exclusive access; no concurrent mutation or release may overlap reads.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_set_f64(
    handle: *mut LiblrhsmmRsDurationF64,
    parameters: *const LiblrhsmmRsDurationParametersF64,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle.cast_const(), 1) {
        return status;
    }
    if let Err(status) = range(parameters, 1) {
        return status;
    }
    // SAFETY: Independent readable descriptor and exclusive live handle.
    let value = unsafe { parameters.read() };
    unsafe {
        (*handle).parameters = value.into();
    }
    0
}

/// Create an independent parameter copy. Outputs remain unchanged on error.
/// # Safety
/// Handles must be live owners from this precision's interface. Release consumes unique ownership.
/// All nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Outputs and descriptors must be independent of handle storage. Writable storage and mutation
/// require exclusive access; no concurrent mutation or release may overlap reads.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_clone_f64(
    handle: *const LiblrhsmmRsDurationF64,
    output: *mut *mut LiblrhsmmRsDurationF64,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsDurationF64 {
                parameters: (*handle).parameters.clone(),
            })))
        })
    }
}

/// Release and clear an initialized ownership slot. A null owned handle is a no-op.
/// # Safety
/// Handles must be live owners from this precision's interface. Release consumes unique ownership.
/// All nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Outputs and descriptors must be independent of handle storage. Writable storage and mutation
/// require exclusive access; no concurrent mutation or release may overlap reads.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_release_f64(
    handle: *mut *mut LiblrhsmmRsDurationF64,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle.cast_const(), 1) {
        return status;
    }
    // SAFETY: Initialized exclusive ownership slot, null or a uniquely owned handle.
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

/// Evaluate the native duration log density with selected-precision scalar frames.
/// # Safety
/// Handles must be live owners from this precision's interface. Release consumes unique ownership.
/// All nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Outputs and descriptors must be independent of handle storage. Writable storage and mutation
/// require exclusive access; no concurrent mutation or release may overlap reads.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_log_probability_f64(
    handle: *const LiblrhsmmRsDurationF64,
    frames: f64,
    weight: f64,
    output: *mut f64,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            (*handle).parameters.log_probability(frames, weight)
        })
    }
}

/// Evaluate the native annealed log density. Minimum exclusion remains active at zero temperature.
/// # Safety
/// Handles must be live owners from this precision's interface. Release consumes unique ownership.
/// All nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Outputs and descriptors must be independent of handle storage. Writable storage and mutation
/// require exclusive access; no concurrent mutation or release may overlap reads.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_annealed_log_probability_f64(
    handle: *const LiblrhsmmRsDurationF64,
    frames: usize,
    weight: f64,
    temperature: f64,
    output: *mut f64,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            (*handle)
                .parameters
                .annealed_log_probability(frames, weight, temperature)
        })
    }
}

/// Evaluate the native density, returning its f64 result in both precisions. Maximum constrains search only.
/// # Safety
/// Handles must be live owners from this precision's interface. Release consumes unique ownership.
/// All nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Outputs and descriptors must be independent of handle storage. Writable storage and mutation
/// require exclusive access; no concurrent mutation or release may overlap reads.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_probability_f64(
    handle: *const LiblrhsmmRsDurationF64,
    frames: usize,
    weight: f64,
    temperature: f64,
    output: *mut f64,
) -> u32 {
    // SAFETY: Caller fulfills the live-owner and independent storage contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            (*handle)
                .parameters
                .probability(frames, weight, temperature)
        })
    }
}
