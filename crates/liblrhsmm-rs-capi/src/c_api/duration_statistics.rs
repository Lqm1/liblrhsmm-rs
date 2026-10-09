//! Owned editable duration sufficient statistics and native maximization.
use super::duration::{LiblrhsmmRsDurationF32, LiblrhsmmRsDurationF64};
use super::{range, scalar_result};
use crate::ModelError;
use crate::statistics::DurationSufficientStatistics;
use crate::update::{DurationMode, UpdateSettings};

pub const LIBLRHSMM_RS_DURATION_NORMAL: u32 = 0;
pub const LIBLRHSMM_RS_DURATION_GEOMETRIC: u32 = 1;

/// All native accumulator fields. Editing retains arbitrary scalar bits.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LiblrhsmmRsDurationStatisticsValuesF32 {
    pub value_sum: f32,
    pub squared_sum: f32,
    pub occupancy: f32,
}
impl From<LiblrhsmmRsDurationStatisticsValuesF32> for DurationSufficientStatistics<f32> {
    fn from(value: LiblrhsmmRsDurationStatisticsValuesF32) -> Self {
        Self {
            value_sum: value.value_sum,
            squared_sum: value.squared_sum,
            occupancy: value.occupancy,
        }
    }
}
impl From<DurationSufficientStatistics<f32>> for LiblrhsmmRsDurationStatisticsValuesF32 {
    fn from(value: DurationSufficientStatistics<f32>) -> Self {
        Self {
            value_sum: value.value_sum,
            squared_sum: value.squared_sum,
            occupancy: value.occupancy,
        }
    }
}
/// Owned editable accumulator, independent of the source duration lifetime.
pub struct LiblrhsmmRsDurationStatisticsF32 {
    pub(super) statistics: DurationSufficientStatistics<f32>,
}
/// Complete native update settings; mode zero is normal and one geometric.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LiblrhsmmRsUpdateOptionsF32 {
    pub duration_mode: u32,
    pub duration_variance_floor: f32,
    pub emission_variance_floor: f32,
}
impl TryFrom<LiblrhsmmRsUpdateOptionsF32> for UpdateSettings<f32> {
    type Error = ModelError;
    fn try_from(value: LiblrhsmmRsUpdateOptionsF32) -> Result<Self, ModelError> {
        let duration_mode = match value.duration_mode {
            LIBLRHSMM_RS_DURATION_NORMAL => DurationMode::Normal,
            LIBLRHSMM_RS_DURATION_GEOMETRIC => DurationMode::Geometric,
            _ => return Err(ModelError("unknown duration update mode")),
        };
        Ok(Self {
            duration_mode,
            duration_variance_floor: value.duration_variance_floor,
            emission_variance_floor: value.emission_variance_floor,
        })
    }
}
impl From<UpdateSettings<f32>> for LiblrhsmmRsUpdateOptionsF32 {
    fn from(value: UpdateSettings<f32>) -> Self {
        Self {
            duration_mode: match value.duration_mode {
                DurationMode::Normal => LIBLRHSMM_RS_DURATION_NORMAL,
                DurationMode::Geometric => LIBLRHSMM_RS_DURATION_GEOMETRIC,
            },
            duration_variance_floor: value.duration_variance_floor,
            emission_variance_floor: value.emission_variance_floor,
        }
    }
}

/// Write all three native zero accumulator fields.
/// # Safety
/// Nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners from this precision's interface. Outputs and descriptors are independent
/// of owner storage. Mutation/release and writable outputs require exclusive access; reads cannot
/// overlap mutation/release. Store successful constructor results in slots holding no live owner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_statistics_default_f32(
    output: *mut LiblrhsmmRsDurationStatisticsValuesF32,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and live-owner contracts above.
    unsafe {
        scalar_result(output, || {
            Ok(DurationSufficientStatistics::<f32>::default().into())
        })
    }
}

/// Copy all fields into an independent owner. Numeric validation occurs during update.
/// # Safety
/// Nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners from this precision's interface. Outputs and descriptors are independent
/// of owner storage. Mutation/release and writable outputs require exclusive access; reads cannot
/// overlap mutation/release. Store successful constructor results in slots holding no live owner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_statistics_create_f32(
    values: *const LiblrhsmmRsDurationStatisticsValuesF32,
    output: *mut *mut LiblrhsmmRsDurationStatisticsF32,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and live-owner contracts above.
    if let Err(status) = range(values, 1) {
        return status;
    }
    // SAFETY: Caller provides an independent initialized descriptor.
    let values = unsafe { values.read() };
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsDurationStatisticsF32 {
                statistics: values.into(),
            })))
        })
    }
}

/// Construct zero statistics from a live duration, without retaining its lifetime or requiring initialization.
/// # Safety
/// Nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners from this precision's interface. Outputs and descriptors are independent
/// of owner storage. Mutation/release and writable outputs require exclusive access; reads cannot
/// overlap mutation/release. Store successful constructor results in slots holding no live owner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_statistics_from_duration_f32(
    duration: *const LiblrhsmmRsDurationF32,
    output: *mut *mut LiblrhsmmRsDurationStatisticsF32,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and live-owner contracts above.
    if let Err(status) = range(duration, 1) {
        return status;
    }
    // Source construction does not read or retain the distribution; flat-start durations are valid.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsDurationStatisticsF32 {
                statistics: DurationSufficientStatistics::default(),
            })))
        })
    }
}

/// Copy all current accumulator fields.
/// # Safety
/// Nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners from this precision's interface. Outputs and descriptors are independent
/// of owner storage. Mutation/release and writable outputs require exclusive access; reads cannot
/// overlap mutation/release. Store successful constructor results in slots holding no live owner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_statistics_get_f32(
    handle: *const LiblrhsmmRsDurationStatisticsF32,
    output: *mut LiblrhsmmRsDurationStatisticsValuesF32,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and live-owner contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || Ok((*handle).statistics.into())) }
}

/// Replace every accumulator field. Native public-field editability, including nonfinite bits, is retained.
/// # Safety
/// Nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners from this precision's interface. Outputs and descriptors are independent
/// of owner storage. Mutation/release and writable outputs require exclusive access; reads cannot
/// overlap mutation/release. Store successful constructor results in slots holding no live owner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_statistics_set_f32(
    handle: *mut LiblrhsmmRsDurationStatisticsF32,
    values: *const LiblrhsmmRsDurationStatisticsValuesF32,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and live-owner contracts above.
    if let Err(status) = range(handle.cast_const(), 1) {
        return status;
    }
    if let Err(status) = range(values, 1) {
        return status;
    }
    // SAFETY: Independent initialized input and exclusive live accumulator.
    let values = unsafe { values.read() };
    unsafe {
        (*handle).statistics = values.into();
    }
    0
}

/// Copy statistics into an independent owner.
/// # Safety
/// Nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners from this precision's interface. Outputs and descriptors are independent
/// of owner storage. Mutation/release and writable outputs require exclusive access; reads cannot
/// overlap mutation/release. Store successful constructor results in slots holding no live owner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_statistics_clone_f32(
    handle: *const LiblrhsmmRsDurationStatisticsF32,
    output: *mut *mut LiblrhsmmRsDurationStatisticsF32,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and live-owner contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsDurationStatisticsF32 {
                statistics: (*handle).statistics,
            })))
        })
    }
}

/// Clear all fields using native accumulator clearing.
/// # Safety
/// Nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners from this precision's interface. Outputs and descriptors are independent
/// of owner storage. Mutation/release and writable outputs require exclusive access; reads cannot
/// overlap mutation/release. Store successful constructor results in slots holding no live owner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_statistics_clear_f32(
    handle: *mut LiblrhsmmRsDurationStatisticsF32,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and live-owner contracts above.
    if let Err(status) = range(handle.cast_const(), 1) {
        return status;
    }
    // SAFETY: Exclusive live owner; native clear is infallible and accepts all stored bits.
    unsafe {
        (*handle).statistics.clear();
    }
    0
}

/// Release and clear the unique-owner slot. Null owned pointers are a successful no-op.
/// # Safety
/// Nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners from this precision's interface. Outputs and descriptors are independent
/// of owner storage. Mutation/release and writable outputs require exclusive access; reads cannot
/// overlap mutation/release. Store successful constructor results in slots holding no live owner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_statistics_release_f32(
    handle: *mut *mut LiblrhsmmRsDurationStatisticsF32,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and live-owner contracts above.
    if let Err(status) = range(handle.cast_const(), 1) {
        return status;
    }
    // SAFETY: Initialized exclusive unique-owner slot.
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

/// Write the complete native maximization defaults.
/// # Safety
/// Nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners from this precision's interface. Outputs and descriptors are independent
/// of owner storage. Mutation/release and writable outputs require exclusive access; reads cannot
/// overlap mutation/release. Store successful constructor results in slots holding no live owner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_update_options_default_f32(
    output: *mut LiblrhsmmRsUpdateOptionsF32,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and live-owner contracts above.
    unsafe { scalar_result(output, || Ok(UpdateSettings::<f32>::default().into())) }
}

/// Delegate native normal/geometric maximization; write one when updated, zero for insufficient occupancy. Errors retain parameters, statistics and result storage.
/// # Safety
/// Nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners from this precision's interface. Outputs and descriptors are independent
/// of owner storage. Mutation/release and writable outputs require exclusive access; reads cannot
/// overlap mutation/release. Store successful constructor results in slots holding no live owner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_update_f32(
    handle: *mut LiblrhsmmRsDurationF32,
    statistics: *const LiblrhsmmRsDurationStatisticsF32,
    options: *const LiblrhsmmRsUpdateOptionsF32,
    updated: *mut u32,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and live-owner contracts above.
    if let Err(status) = range(handle.cast_const(), 1) {
        return status;
    }
    if let Err(status) = range(statistics, 1) {
        return status;
    }
    if let Err(status) = range(options, 1) {
        return status;
    }
    // SAFETY: Independent initialized settings; output is checked before any mutation.
    let options = unsafe { options.read() };
    unsafe {
        scalar_result(updated, || {
            let mut parameters = (*handle).parameters.clone();
            let changed = parameters.update(&(*statistics).statistics, options.try_into()?)?;
            (*handle).parameters = parameters;
            Ok(u32::from(changed))
        })
    }
}

/// All native accumulator fields. Editing retains arbitrary scalar bits.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LiblrhsmmRsDurationStatisticsValuesF64 {
    pub value_sum: f64,
    pub squared_sum: f64,
    pub occupancy: f64,
}
impl From<LiblrhsmmRsDurationStatisticsValuesF64> for DurationSufficientStatistics<f64> {
    fn from(value: LiblrhsmmRsDurationStatisticsValuesF64) -> Self {
        Self {
            value_sum: value.value_sum,
            squared_sum: value.squared_sum,
            occupancy: value.occupancy,
        }
    }
}
impl From<DurationSufficientStatistics<f64>> for LiblrhsmmRsDurationStatisticsValuesF64 {
    fn from(value: DurationSufficientStatistics<f64>) -> Self {
        Self {
            value_sum: value.value_sum,
            squared_sum: value.squared_sum,
            occupancy: value.occupancy,
        }
    }
}
/// Owned editable accumulator, independent of the source duration lifetime.
pub struct LiblrhsmmRsDurationStatisticsF64 {
    pub(super) statistics: DurationSufficientStatistics<f64>,
}
/// Complete native update settings; mode zero is normal and one geometric.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LiblrhsmmRsUpdateOptionsF64 {
    pub duration_mode: u32,
    pub duration_variance_floor: f64,
    pub emission_variance_floor: f64,
}
impl TryFrom<LiblrhsmmRsUpdateOptionsF64> for UpdateSettings<f64> {
    type Error = ModelError;
    fn try_from(value: LiblrhsmmRsUpdateOptionsF64) -> Result<Self, ModelError> {
        let duration_mode = match value.duration_mode {
            LIBLRHSMM_RS_DURATION_NORMAL => DurationMode::Normal,
            LIBLRHSMM_RS_DURATION_GEOMETRIC => DurationMode::Geometric,
            _ => return Err(ModelError("unknown duration update mode")),
        };
        Ok(Self {
            duration_mode,
            duration_variance_floor: value.duration_variance_floor,
            emission_variance_floor: value.emission_variance_floor,
        })
    }
}
impl From<UpdateSettings<f64>> for LiblrhsmmRsUpdateOptionsF64 {
    fn from(value: UpdateSettings<f64>) -> Self {
        Self {
            duration_mode: match value.duration_mode {
                DurationMode::Normal => LIBLRHSMM_RS_DURATION_NORMAL,
                DurationMode::Geometric => LIBLRHSMM_RS_DURATION_GEOMETRIC,
            },
            duration_variance_floor: value.duration_variance_floor,
            emission_variance_floor: value.emission_variance_floor,
        }
    }
}

/// Write all three native zero accumulator fields.
/// # Safety
/// Nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners from this precision's interface. Outputs and descriptors are independent
/// of owner storage. Mutation/release and writable outputs require exclusive access; reads cannot
/// overlap mutation/release. Store successful constructor results in slots holding no live owner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_statistics_default_f64(
    output: *mut LiblrhsmmRsDurationStatisticsValuesF64,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and live-owner contracts above.
    unsafe {
        scalar_result(output, || {
            Ok(DurationSufficientStatistics::<f64>::default().into())
        })
    }
}

/// Copy all fields into an independent owner. Numeric validation occurs during update.
/// # Safety
/// Nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners from this precision's interface. Outputs and descriptors are independent
/// of owner storage. Mutation/release and writable outputs require exclusive access; reads cannot
/// overlap mutation/release. Store successful constructor results in slots holding no live owner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_statistics_create_f64(
    values: *const LiblrhsmmRsDurationStatisticsValuesF64,
    output: *mut *mut LiblrhsmmRsDurationStatisticsF64,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and live-owner contracts above.
    if let Err(status) = range(values, 1) {
        return status;
    }
    // SAFETY: Caller provides an independent initialized descriptor.
    let values = unsafe { values.read() };
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsDurationStatisticsF64 {
                statistics: values.into(),
            })))
        })
    }
}

/// Construct zero statistics from a live duration, without retaining its lifetime or requiring initialization.
/// # Safety
/// Nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners from this precision's interface. Outputs and descriptors are independent
/// of owner storage. Mutation/release and writable outputs require exclusive access; reads cannot
/// overlap mutation/release. Store successful constructor results in slots holding no live owner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_statistics_from_duration_f64(
    duration: *const LiblrhsmmRsDurationF64,
    output: *mut *mut LiblrhsmmRsDurationStatisticsF64,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and live-owner contracts above.
    if let Err(status) = range(duration, 1) {
        return status;
    }
    // Source construction does not read or retain the distribution; flat-start durations are valid.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsDurationStatisticsF64 {
                statistics: DurationSufficientStatistics::default(),
            })))
        })
    }
}

/// Copy all current accumulator fields.
/// # Safety
/// Nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners from this precision's interface. Outputs and descriptors are independent
/// of owner storage. Mutation/release and writable outputs require exclusive access; reads cannot
/// overlap mutation/release. Store successful constructor results in slots holding no live owner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_statistics_get_f64(
    handle: *const LiblrhsmmRsDurationStatisticsF64,
    output: *mut LiblrhsmmRsDurationStatisticsValuesF64,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and live-owner contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || Ok((*handle).statistics.into())) }
}

/// Replace every accumulator field. Native public-field editability, including nonfinite bits, is retained.
/// # Safety
/// Nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners from this precision's interface. Outputs and descriptors are independent
/// of owner storage. Mutation/release and writable outputs require exclusive access; reads cannot
/// overlap mutation/release. Store successful constructor results in slots holding no live owner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_statistics_set_f64(
    handle: *mut LiblrhsmmRsDurationStatisticsF64,
    values: *const LiblrhsmmRsDurationStatisticsValuesF64,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and live-owner contracts above.
    if let Err(status) = range(handle.cast_const(), 1) {
        return status;
    }
    if let Err(status) = range(values, 1) {
        return status;
    }
    // SAFETY: Independent initialized input and exclusive live accumulator.
    let values = unsafe { values.read() };
    unsafe {
        (*handle).statistics = values.into();
    }
    0
}

/// Copy statistics into an independent owner.
/// # Safety
/// Nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners from this precision's interface. Outputs and descriptors are independent
/// of owner storage. Mutation/release and writable outputs require exclusive access; reads cannot
/// overlap mutation/release. Store successful constructor results in slots holding no live owner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_statistics_clone_f64(
    handle: *const LiblrhsmmRsDurationStatisticsF64,
    output: *mut *mut LiblrhsmmRsDurationStatisticsF64,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and live-owner contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsDurationStatisticsF64 {
                statistics: (*handle).statistics,
            })))
        })
    }
}

/// Clear all fields using native accumulator clearing.
/// # Safety
/// Nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners from this precision's interface. Outputs and descriptors are independent
/// of owner storage. Mutation/release and writable outputs require exclusive access; reads cannot
/// overlap mutation/release. Store successful constructor results in slots holding no live owner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_statistics_clear_f64(
    handle: *mut LiblrhsmmRsDurationStatisticsF64,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and live-owner contracts above.
    if let Err(status) = range(handle.cast_const(), 1) {
        return status;
    }
    // SAFETY: Exclusive live owner; native clear is infallible and accepts all stored bits.
    unsafe {
        (*handle).statistics.clear();
    }
    0
}

/// Release and clear the unique-owner slot. Null owned pointers are a successful no-op.
/// # Safety
/// Nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners from this precision's interface. Outputs and descriptors are independent
/// of owner storage. Mutation/release and writable outputs require exclusive access; reads cannot
/// overlap mutation/release. Store successful constructor results in slots holding no live owner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_statistics_release_f64(
    handle: *mut *mut LiblrhsmmRsDurationStatisticsF64,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and live-owner contracts above.
    if let Err(status) = range(handle.cast_const(), 1) {
        return status;
    }
    // SAFETY: Initialized exclusive unique-owner slot.
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

/// Write the complete native maximization defaults.
/// # Safety
/// Nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners from this precision's interface. Outputs and descriptors are independent
/// of owner storage. Mutation/release and writable outputs require exclusive access; reads cannot
/// overlap mutation/release. Store successful constructor results in slots holding no live owner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_update_options_default_f64(
    output: *mut LiblrhsmmRsUpdateOptionsF64,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and live-owner contracts above.
    unsafe { scalar_result(output, || Ok(UpdateSettings::<f64>::default().into())) }
}

/// Delegate native normal/geometric maximization; write one when updated, zero for insufficient occupancy. Errors retain parameters, statistics and result storage.
/// # Safety
/// Nonnull pointers identify aligned valid storage of the declared type. Input descriptors
/// and ownership slots are initialized; scalar/descriptor outputs need only writable storage.
/// Handles are live owners from this precision's interface. Outputs and descriptors are independent
/// of owner storage. Mutation/release and writable outputs require exclusive access; reads cannot
/// overlap mutation/release. Store successful constructor results in slots holding no live owner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_update_f64(
    handle: *mut LiblrhsmmRsDurationF64,
    statistics: *const LiblrhsmmRsDurationStatisticsF64,
    options: *const LiblrhsmmRsUpdateOptionsF64,
    updated: *mut u32,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and live-owner contracts above.
    if let Err(status) = range(handle.cast_const(), 1) {
        return status;
    }
    if let Err(status) = range(statistics, 1) {
        return status;
    }
    if let Err(status) = range(options, 1) {
        return status;
    }
    // SAFETY: Independent initialized settings; output is checked before any mutation.
    let options = unsafe { options.read() };
    unsafe {
        scalar_result(updated, || {
            let mut parameters = (*handle).parameters.clone();
            let changed = parameters.update(&(*statistics).statistics, options.try_into()?)?;
            (*handle).parameters = parameters;
            Ok(u32::from(changed))
        })
    }
}
