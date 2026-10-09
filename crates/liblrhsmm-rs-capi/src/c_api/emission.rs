//! Complete weighted multistream emission computation using native pruning/cache semantics.
use super::*;
use crate::emission::EmissionPruningOptions;
use crate::geometric::GeometricSettings;
use crate::hsmm::HsmmSettings;

/// Evaluate every frame/state pair.
pub const LIBLRHSMM_RS_EMISSION_PRUNING_NONE: u32 = 0;
/// Evaluate the native inclusive boundary band using radius.
pub const LIBLRHSMM_RS_EMISSION_PRUNING_BOUNDARIES: u32 = 1;
/// Evaluate the native diagonal band using slope and its exclusive upper bound.
pub const LIBLRHSMM_RS_EMISSION_PRUNING_LINEAR: u32 = 2;

/// Complete per-call emission configuration. Only the selected band's field is read.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LiblrhsmmRsEmissionSettingsF32 {
    pub temperature: f32,
    pub pruning: u32,
    pub radius: f32,
    pub slope: f32,
}
impl TryFrom<LiblrhsmmRsEmissionSettingsF32> for EmissionPruningOptions<f32> {
    type Error = ModelError;
    fn try_from(value: LiblrhsmmRsEmissionSettingsF32) -> Result<Self, Self::Error> {
        match value.pruning {
            LIBLRHSMM_RS_EMISSION_PRUNING_NONE => Ok(Self::None),
            LIBLRHSMM_RS_EMISSION_PRUNING_BOUNDARIES => Ok(Self::AroundBoundaries {
                radius: value.radius,
            }),
            LIBLRHSMM_RS_EMISSION_PRUNING_LINEAR => Ok(Self::Linear { slope: value.slope }),
            _ => Err(ModelError("unknown emission pruning mode")),
        }
    }
}
/// Write an unpruned configuration with unchanged native scalar defaults.
/// # Safety
/// Handles are live owners of the declared precision. Settings are aligned initialized
/// readable storage. Outputs are independent aligned writable storage, with no live
/// owner in successful constructor slots. Reads cannot overlap mutation or release.
/// Result storage is independently owned and must be uniquely released. No pointer
/// into input storage escapes, and inputs are borrowed only during this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_emission_settings_default_f32(
    output: *mut LiblrhsmmRsEmissionSettingsF32,
) -> u32 {
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsEmissionSettingsF32 {
                temperature: GeometricSettings::<f32>::default().temperature,
                pruning: LIBLRHSMM_RS_EMISSION_PRUNING_NONE,
                radius: HsmmSettings::<f32>::default().state_radius,
                slope: GeometricSettings::<f32>::default().pruning_slope,
            })
        })
    }
}

/// Evaluate weighted multistream emissions with the unchanged native computation.
/// All pruning modes are supported. Prepared-model calls reuse existing normalizers.
/// # Safety
/// Handles are live owners of the declared precision. Settings are aligned initialized
/// readable storage. Outputs are independent aligned writable storage, with no live
/// owner in successful constructor slots. Reads cannot overlap mutation or release.
/// Result storage is independently owned and must be uniquely released. No pointer
/// into input storage escapes, and inputs are borrowed only during this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_emission_log_probabilities_f32(
    model: *const LiblrhsmmRsModelF32,
    observation: *const LiblrhsmmRsObservationF32,
    segmentation: *const LiblrhsmmRsSegmentationF32,
    settings: *const LiblrhsmmRsEmissionSettingsF32,
    output: *mut *mut LiblrhsmmRsLogEmissionsF32,
) -> u32 {
    if let Err(status) = range(model, 1)
        .and_then(|()| range(observation, 1))
        .and_then(|()| range(segmentation, 1))
        .and_then(|()| range(settings, 1))
    {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let settings = settings.read();
            let pruning = settings.try_into()?;
            let table = (*model).parameters.emission_log_probabilities(
                &(*observation).observation,
                &(*segmentation).segmentation,
                settings.temperature,
                pruning,
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsLogEmissionsF32 {
                table,
            })))
        })
    }
}

/// Evaluate weighted multistream emissions with the unchanged native computation.
/// All pruning modes are supported. Prepared-model calls reuse existing normalizers.
/// # Safety
/// Handles are live owners of the declared precision. Settings are aligned initialized
/// readable storage. Outputs are independent aligned writable storage, with no live
/// owner in successful constructor slots. Reads cannot overlap mutation or release.
/// Result storage is independently owned and must be uniquely released. No pointer
/// into input storage escapes, and inputs are borrowed only during this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_emission_probabilities_f32(
    model: *const LiblrhsmmRsModelF32,
    observation: *const LiblrhsmmRsObservationF32,
    segmentation: *const LiblrhsmmRsSegmentationF32,
    settings: *const LiblrhsmmRsEmissionSettingsF32,
    output: *mut *mut LiblrhsmmRsEmissionDensitiesF32,
) -> u32 {
    if let Err(status) = range(model, 1)
        .and_then(|()| range(observation, 1))
        .and_then(|()| range(segmentation, 1))
        .and_then(|()| range(settings, 1))
    {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let settings = settings.read();
            let pruning = settings.try_into()?;
            let table = (*model).parameters.emission_probabilities(
                &(*observation).observation,
                &(*segmentation).segmentation,
                settings.temperature,
                pruning,
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsEmissionDensitiesF32 {
                table,
            })))
        })
    }
}

/// Evaluate weighted multistream emissions with the unchanged native computation.
/// All pruning modes are supported. Prepared-model calls reuse existing normalizers.
/// # Safety
/// Handles are live owners of the declared precision. Settings are aligned initialized
/// readable storage. Outputs are independent aligned writable storage, with no live
/// owner in successful constructor slots. Reads cannot overlap mutation or release.
/// Result storage is independently owned and must be uniquely released. No pointer
/// into input storage escapes, and inputs are borrowed only during this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_emission_log_probabilities_f32(
    model: *const LiblrhsmmRsPreparedModelF32,
    observation: *const LiblrhsmmRsObservationF32,
    segmentation: *const LiblrhsmmRsSegmentationF32,
    settings: *const LiblrhsmmRsEmissionSettingsF32,
    output: *mut *mut LiblrhsmmRsLogEmissionsF32,
) -> u32 {
    if let Err(status) = range(model, 1)
        .and_then(|()| range(observation, 1))
        .and_then(|()| range(segmentation, 1))
        .and_then(|()| range(settings, 1))
    {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let settings = settings.read();
            let pruning = settings.try_into()?;
            let table = (*model).prepared.emission_log_probabilities(
                &(*observation).observation,
                &(*segmentation).segmentation,
                settings.temperature,
                pruning,
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsLogEmissionsF32 {
                table,
            })))
        })
    }
}

/// Evaluate weighted multistream emissions with the unchanged native computation.
/// All pruning modes are supported. Prepared-model calls reuse existing normalizers.
/// # Safety
/// Handles are live owners of the declared precision. Settings are aligned initialized
/// readable storage. Outputs are independent aligned writable storage, with no live
/// owner in successful constructor slots. Reads cannot overlap mutation or release.
/// Result storage is independently owned and must be uniquely released. No pointer
/// into input storage escapes, and inputs are borrowed only during this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_emission_probabilities_f32(
    model: *const LiblrhsmmRsPreparedModelF32,
    observation: *const LiblrhsmmRsObservationF32,
    segmentation: *const LiblrhsmmRsSegmentationF32,
    settings: *const LiblrhsmmRsEmissionSettingsF32,
    output: *mut *mut LiblrhsmmRsEmissionDensitiesF32,
) -> u32 {
    if let Err(status) = range(model, 1)
        .and_then(|()| range(observation, 1))
        .and_then(|()| range(segmentation, 1))
        .and_then(|()| range(settings, 1))
    {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let settings = settings.read();
            let pruning = settings.try_into()?;
            let table = (*model).prepared.emission_probabilities(
                &(*observation).observation,
                &(*segmentation).segmentation,
                settings.temperature,
                pruning,
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsEmissionDensitiesF32 {
                table,
            })))
        })
    }
}

/// Complete per-call emission configuration. Only the selected band's field is read.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LiblrhsmmRsEmissionSettingsF64 {
    pub temperature: f64,
    pub pruning: u32,
    pub radius: f64,
    pub slope: f64,
}
impl TryFrom<LiblrhsmmRsEmissionSettingsF64> for EmissionPruningOptions<f64> {
    type Error = ModelError;
    fn try_from(value: LiblrhsmmRsEmissionSettingsF64) -> Result<Self, Self::Error> {
        match value.pruning {
            LIBLRHSMM_RS_EMISSION_PRUNING_NONE => Ok(Self::None),
            LIBLRHSMM_RS_EMISSION_PRUNING_BOUNDARIES => Ok(Self::AroundBoundaries {
                radius: value.radius,
            }),
            LIBLRHSMM_RS_EMISSION_PRUNING_LINEAR => Ok(Self::Linear { slope: value.slope }),
            _ => Err(ModelError("unknown emission pruning mode")),
        }
    }
}
/// Write an unpruned configuration with unchanged native scalar defaults.
/// # Safety
/// Handles are live owners of the declared precision. Settings are aligned initialized
/// readable storage. Outputs are independent aligned writable storage, with no live
/// owner in successful constructor slots. Reads cannot overlap mutation or release.
/// Result storage is independently owned and must be uniquely released. No pointer
/// into input storage escapes, and inputs are borrowed only during this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_emission_settings_default_f64(
    output: *mut LiblrhsmmRsEmissionSettingsF64,
) -> u32 {
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsEmissionSettingsF64 {
                temperature: GeometricSettings::<f64>::default().temperature,
                pruning: LIBLRHSMM_RS_EMISSION_PRUNING_NONE,
                radius: HsmmSettings::<f64>::default().state_radius,
                slope: GeometricSettings::<f64>::default().pruning_slope,
            })
        })
    }
}

/// Evaluate weighted multistream emissions with the unchanged native computation.
/// All pruning modes are supported. Prepared-model calls reuse existing normalizers.
/// # Safety
/// Handles are live owners of the declared precision. Settings are aligned initialized
/// readable storage. Outputs are independent aligned writable storage, with no live
/// owner in successful constructor slots. Reads cannot overlap mutation or release.
/// Result storage is independently owned and must be uniquely released. No pointer
/// into input storage escapes, and inputs are borrowed only during this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_emission_log_probabilities_f64(
    model: *const LiblrhsmmRsModelF64,
    observation: *const LiblrhsmmRsObservationF64,
    segmentation: *const LiblrhsmmRsSegmentationF64,
    settings: *const LiblrhsmmRsEmissionSettingsF64,
    output: *mut *mut LiblrhsmmRsLogEmissionsF64,
) -> u32 {
    if let Err(status) = range(model, 1)
        .and_then(|()| range(observation, 1))
        .and_then(|()| range(segmentation, 1))
        .and_then(|()| range(settings, 1))
    {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let settings = settings.read();
            let pruning = settings.try_into()?;
            let table = (*model).parameters.emission_log_probabilities(
                &(*observation).observation,
                &(*segmentation).segmentation,
                settings.temperature,
                pruning,
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsLogEmissionsF64 {
                table,
            })))
        })
    }
}

/// Evaluate weighted multistream emissions with the unchanged native computation.
/// All pruning modes are supported. Prepared-model calls reuse existing normalizers.
/// # Safety
/// Handles are live owners of the declared precision. Settings are aligned initialized
/// readable storage. Outputs are independent aligned writable storage, with no live
/// owner in successful constructor slots. Reads cannot overlap mutation or release.
/// Result storage is independently owned and must be uniquely released. No pointer
/// into input storage escapes, and inputs are borrowed only during this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_emission_probabilities_f64(
    model: *const LiblrhsmmRsModelF64,
    observation: *const LiblrhsmmRsObservationF64,
    segmentation: *const LiblrhsmmRsSegmentationF64,
    settings: *const LiblrhsmmRsEmissionSettingsF64,
    output: *mut *mut LiblrhsmmRsEmissionDensitiesF64,
) -> u32 {
    if let Err(status) = range(model, 1)
        .and_then(|()| range(observation, 1))
        .and_then(|()| range(segmentation, 1))
        .and_then(|()| range(settings, 1))
    {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let settings = settings.read();
            let pruning = settings.try_into()?;
            let table = (*model).parameters.emission_probabilities(
                &(*observation).observation,
                &(*segmentation).segmentation,
                settings.temperature,
                pruning,
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsEmissionDensitiesF64 {
                table,
            })))
        })
    }
}

/// Evaluate weighted multistream emissions with the unchanged native computation.
/// All pruning modes are supported. Prepared-model calls reuse existing normalizers.
/// # Safety
/// Handles are live owners of the declared precision. Settings are aligned initialized
/// readable storage. Outputs are independent aligned writable storage, with no live
/// owner in successful constructor slots. Reads cannot overlap mutation or release.
/// Result storage is independently owned and must be uniquely released. No pointer
/// into input storage escapes, and inputs are borrowed only during this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_emission_log_probabilities_f64(
    model: *const LiblrhsmmRsPreparedModelF64,
    observation: *const LiblrhsmmRsObservationF64,
    segmentation: *const LiblrhsmmRsSegmentationF64,
    settings: *const LiblrhsmmRsEmissionSettingsF64,
    output: *mut *mut LiblrhsmmRsLogEmissionsF64,
) -> u32 {
    if let Err(status) = range(model, 1)
        .and_then(|()| range(observation, 1))
        .and_then(|()| range(segmentation, 1))
        .and_then(|()| range(settings, 1))
    {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let settings = settings.read();
            let pruning = settings.try_into()?;
            let table = (*model).prepared.emission_log_probabilities(
                &(*observation).observation,
                &(*segmentation).segmentation,
                settings.temperature,
                pruning,
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsLogEmissionsF64 {
                table,
            })))
        })
    }
}

/// Evaluate weighted multistream emissions with the unchanged native computation.
/// All pruning modes are supported. Prepared-model calls reuse existing normalizers.
/// # Safety
/// Handles are live owners of the declared precision. Settings are aligned initialized
/// readable storage. Outputs are independent aligned writable storage, with no live
/// owner in successful constructor slots. Reads cannot overlap mutation or release.
/// Result storage is independently owned and must be uniquely released. No pointer
/// into input storage escapes, and inputs are borrowed only during this call.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_emission_probabilities_f64(
    model: *const LiblrhsmmRsPreparedModelF64,
    observation: *const LiblrhsmmRsObservationF64,
    segmentation: *const LiblrhsmmRsSegmentationF64,
    settings: *const LiblrhsmmRsEmissionSettingsF64,
    output: *mut *mut LiblrhsmmRsEmissionDensitiesF64,
) -> u32 {
    if let Err(status) = range(model, 1)
        .and_then(|()| range(observation, 1))
        .and_then(|()| range(segmentation, 1))
        .and_then(|()| range(settings, 1))
    {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let settings = settings.read();
            let pruning = settings.try_into()?;
            let table = (*model).prepared.emission_probabilities(
                &(*observation).observation,
                &(*segmentation).segmentation,
                settings.temperature,
                pruning,
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsEmissionDensitiesF64 {
                table,
            })))
        })
    }
}
