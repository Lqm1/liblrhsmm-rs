//! Complete native initialization, expectation collection and estimation.
use super::*;
use crate::expectation::{GeometricPosteriorExpectation, HsmmPosteriorExpectation};
use crate::mixture_occupancy::MixtureOccupancies;

/// Validate every borrowed posterior owner before constructing temporary native inputs.
unsafe fn mixture_inputs(
    input: *const *const LiblrhsmmRsMixtureOccupancies,
    count: usize,
) -> Result<Vec<MixtureOccupancies>, u32> {
    range(input, count)?;
    let handles = if count == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(input, count) }
    };
    for &handle in handles {
        range(handle, 1)?;
    }
    boundary(|| {
        let mut values = Vec::new();
        values
            .try_reserve_exact(count)
            .map_err(|_| ModelError("mixture posterior allocation failed"))?;
        for &handle in handles {
            // SAFETY: Validated readable owners, borrowed only during this call.
            values.push(unsafe { (*handle).table.clone() });
        }
        Ok(values)
    })
}

/// Accumulate a complete aligned observation using native initialization semantics.
/// # Safety
/// Owners are live and match the declared precision. Inputs and descriptors are
/// aligned initialized readable storage. The statistics owner is exclusively
/// writable and cannot alias inputs. Borrowed mixture-owner arrays contain count
/// live readable entries, which may repeat. Likelihood outputs are independent
/// aligned writable scalar storage and cannot alias any input or owner. No
/// concurrent mutation or release overlaps reads. All inputs remain caller-owned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_collect_initialization_f32(
    statistics: *mut LiblrhsmmRsModelStatisticsF32,
    model: *const LiblrhsmmRsModelF32,
    observation: *const LiblrhsmmRsObservationF32,
    segmentation: *const LiblrhsmmRsSegmentationF32,
) -> u32 {
    for result in [
        range(statistics, 1),
        range(model, 1),
        range(observation, 1),
        range(segmentation, 1),
    ] {
        if let Err(status) = result {
            return status;
        }
    }
    match boundary(|| unsafe {
        (*statistics).statistics.collect_initialization(
            &(*model).parameters,
            &(*observation).observation,
            &(*segmentation).segmentation,
        )
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}

/// Accumulate every supplied HSMM posterior, preserving inclusive boundary traversal.
/// Native validation and staged accumulation leave statistics unchanged on failure.
/// # Safety
/// Owners are live and match the declared precision. Inputs and descriptors are
/// aligned initialized readable storage. The statistics owner is exclusively
/// writable and cannot alias inputs. Borrowed mixture-owner arrays contain count
/// live readable entries, which may repeat. Likelihood outputs are independent
/// aligned writable scalar storage and cannot alias any input or owner. No
/// concurrent mutation or release overlaps reads. All inputs remain caller-owned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_collect_hsmm_f32(
    statistics: *mut LiblrhsmmRsModelStatisticsF32,
    model: *const LiblrhsmmRsModelF32,
    observation: *const LiblrhsmmRsObservationF32,
    segmentation: *const LiblrhsmmRsSegmentationF32,
    durations: *const LiblrhsmmRsDurationOccupancies,
    states: *const LiblrhsmmRsStateOccupanciesF32,
    mixtures: *const *const LiblrhsmmRsMixtureOccupancies,
    mixture_count: usize,
    state_radius: f32,
) -> u32 {
    for result in [
        range(statistics, 1),
        range(model, 1),
        range(observation, 1),
        range(segmentation, 1),
        range(durations, 1),
        range(states, 1),
    ] {
        if let Err(status) = result {
            return status;
        }
    }
    let mixtures = match unsafe { mixture_inputs(mixtures, mixture_count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    match boundary(|| unsafe {
        (*statistics).statistics.collect_hsmm(
            &(*model).parameters,
            &(*observation).observation,
            &(*segmentation).segmentation,
            HsmmPosteriorExpectation {
                durations: &(*durations).table,
                states: &(*states).table,
                mixtures: &mixtures,
                state_radius,
            },
        )
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}

/// Accumulate complete geometric posteriors using the native endpoint convention.
/// Native validation and staged accumulation leave statistics unchanged on failure.
/// # Safety
/// Owners are live and match the declared precision. Inputs and descriptors are
/// aligned initialized readable storage. The statistics owner is exclusively
/// writable and cannot alias inputs. Borrowed mixture-owner arrays contain count
/// live readable entries, which may repeat. Likelihood outputs are independent
/// aligned writable scalar storage and cannot alias any input or owner. No
/// concurrent mutation or release overlaps reads. All inputs remain caller-owned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_collect_geometric_f32(
    statistics: *mut LiblrhsmmRsModelStatisticsF32,
    model: *const LiblrhsmmRsModelF32,
    observation: *const LiblrhsmmRsObservationF32,
    segmentation: *const LiblrhsmmRsSegmentationF32,
    transitions: *const LiblrhsmmRsTransitionOccupancies,
    states: *const LiblrhsmmRsStateOccupanciesF32,
    mixtures: *const *const LiblrhsmmRsMixtureOccupancies,
    mixture_count: usize,
    pruning_slope: f32,
) -> u32 {
    for result in [
        range(statistics, 1),
        range(model, 1),
        range(observation, 1),
        range(segmentation, 1),
        range(transitions, 1),
        range(states, 1),
    ] {
        if let Err(status) = result {
            return status;
        }
    }
    let mixtures = match unsafe { mixture_inputs(mixtures, mixture_count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    match boundary(|| unsafe {
        (*statistics).statistics.collect_geometric(
            &(*model).parameters,
            &(*observation).observation,
            &(*segmentation).segmentation,
            GeometricPosteriorExpectation {
                transitions: &(*transitions).table,
                states: &(*states).table,
                mixtures: &mixtures,
                pruning_slope,
            },
        )
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}

/// Run one complete native HSMM expectation step and return its log likelihood.
/// Output validation precedes accumulation; failure preserves statistics and output.
/// # Safety
/// Owners are live and match the declared precision. Inputs and descriptors are
/// aligned initialized readable storage. The statistics owner is exclusively
/// writable and cannot alias inputs. Borrowed mixture-owner arrays contain count
/// live readable entries, which may repeat. Likelihood outputs are independent
/// aligned writable scalar storage and cannot alias any input or owner. No
/// concurrent mutation or release overlaps reads. All inputs remain caller-owned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_estimate_hsmm_f32(
    statistics: *mut LiblrhsmmRsModelStatisticsF32,
    model: *const LiblrhsmmRsModelF32,
    observation: *const LiblrhsmmRsObservationF32,
    segmentation: *const LiblrhsmmRsSegmentationF32,
    settings: *const LiblrhsmmRsHsmmSettingsF32,
    output: *mut f32,
) -> u32 {
    for result in [
        range(statistics, 1),
        range(model, 1),
        range(observation, 1),
        range(segmentation, 1),
        range(settings, 1),
    ] {
        if let Err(status) = result {
            return status;
        }
    }
    unsafe {
        scalar_result(output, || {
            (*statistics).statistics.estimate_hsmm(
                &(*model).parameters,
                &(*observation).observation,
                &(*segmentation).segmentation,
                settings.read().into(),
            )
        })
    }
}

/// Run one complete native geometric expectation step and return its log likelihood.
/// Output validation precedes accumulation; failure preserves statistics and output.
/// # Safety
/// Owners are live and match the declared precision. Inputs and descriptors are
/// aligned initialized readable storage. The statistics owner is exclusively
/// writable and cannot alias inputs. Borrowed mixture-owner arrays contain count
/// live readable entries, which may repeat. Likelihood outputs are independent
/// aligned writable scalar storage and cannot alias any input or owner. No
/// concurrent mutation or release overlaps reads. All inputs remain caller-owned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_estimate_geometric_f32(
    statistics: *mut LiblrhsmmRsModelStatisticsF32,
    model: *const LiblrhsmmRsModelF32,
    observation: *const LiblrhsmmRsObservationF32,
    segmentation: *const LiblrhsmmRsSegmentationF32,
    settings: *const LiblrhsmmRsGeometricSettingsF32,
    output: *mut f32,
) -> u32 {
    for result in [
        range(statistics, 1),
        range(model, 1),
        range(observation, 1),
        range(segmentation, 1),
        range(settings, 1),
    ] {
        if let Err(status) = result {
            return status;
        }
    }
    unsafe {
        scalar_result(output, || {
            (*statistics).statistics.estimate_geometric(
                &(*model).parameters,
                &(*observation).observation,
                &(*segmentation).segmentation,
                settings.read().into(),
            )
        })
    }
}

/// Accumulate a complete aligned observation using native initialization semantics.
/// # Safety
/// Owners are live and match the declared precision. Inputs and descriptors are
/// aligned initialized readable storage. The statistics owner is exclusively
/// writable and cannot alias inputs. Borrowed mixture-owner arrays contain count
/// live readable entries, which may repeat. Likelihood outputs are independent
/// aligned writable scalar storage and cannot alias any input or owner. No
/// concurrent mutation or release overlaps reads. All inputs remain caller-owned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_collect_initialization_f64(
    statistics: *mut LiblrhsmmRsModelStatisticsF64,
    model: *const LiblrhsmmRsModelF64,
    observation: *const LiblrhsmmRsObservationF64,
    segmentation: *const LiblrhsmmRsSegmentationF64,
) -> u32 {
    for result in [
        range(statistics, 1),
        range(model, 1),
        range(observation, 1),
        range(segmentation, 1),
    ] {
        if let Err(status) = result {
            return status;
        }
    }
    match boundary(|| unsafe {
        (*statistics).statistics.collect_initialization(
            &(*model).parameters,
            &(*observation).observation,
            &(*segmentation).segmentation,
        )
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}

/// Accumulate every supplied HSMM posterior, preserving inclusive boundary traversal.
/// Native validation and staged accumulation leave statistics unchanged on failure.
/// # Safety
/// Owners are live and match the declared precision. Inputs and descriptors are
/// aligned initialized readable storage. The statistics owner is exclusively
/// writable and cannot alias inputs. Borrowed mixture-owner arrays contain count
/// live readable entries, which may repeat. Likelihood outputs are independent
/// aligned writable scalar storage and cannot alias any input or owner. No
/// concurrent mutation or release overlaps reads. All inputs remain caller-owned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_collect_hsmm_f64(
    statistics: *mut LiblrhsmmRsModelStatisticsF64,
    model: *const LiblrhsmmRsModelF64,
    observation: *const LiblrhsmmRsObservationF64,
    segmentation: *const LiblrhsmmRsSegmentationF64,
    durations: *const LiblrhsmmRsDurationOccupancies,
    states: *const LiblrhsmmRsStateOccupanciesF64,
    mixtures: *const *const LiblrhsmmRsMixtureOccupancies,
    mixture_count: usize,
    state_radius: f64,
) -> u32 {
    for result in [
        range(statistics, 1),
        range(model, 1),
        range(observation, 1),
        range(segmentation, 1),
        range(durations, 1),
        range(states, 1),
    ] {
        if let Err(status) = result {
            return status;
        }
    }
    let mixtures = match unsafe { mixture_inputs(mixtures, mixture_count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    match boundary(|| unsafe {
        (*statistics).statistics.collect_hsmm(
            &(*model).parameters,
            &(*observation).observation,
            &(*segmentation).segmentation,
            HsmmPosteriorExpectation {
                durations: &(*durations).table,
                states: &(*states).table,
                mixtures: &mixtures,
                state_radius,
            },
        )
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}

/// Accumulate complete geometric posteriors using the native endpoint convention.
/// Native validation and staged accumulation leave statistics unchanged on failure.
/// # Safety
/// Owners are live and match the declared precision. Inputs and descriptors are
/// aligned initialized readable storage. The statistics owner is exclusively
/// writable and cannot alias inputs. Borrowed mixture-owner arrays contain count
/// live readable entries, which may repeat. Likelihood outputs are independent
/// aligned writable scalar storage and cannot alias any input or owner. No
/// concurrent mutation or release overlaps reads. All inputs remain caller-owned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_collect_geometric_f64(
    statistics: *mut LiblrhsmmRsModelStatisticsF64,
    model: *const LiblrhsmmRsModelF64,
    observation: *const LiblrhsmmRsObservationF64,
    segmentation: *const LiblrhsmmRsSegmentationF64,
    transitions: *const LiblrhsmmRsTransitionOccupancies,
    states: *const LiblrhsmmRsStateOccupanciesF64,
    mixtures: *const *const LiblrhsmmRsMixtureOccupancies,
    mixture_count: usize,
    pruning_slope: f64,
) -> u32 {
    for result in [
        range(statistics, 1),
        range(model, 1),
        range(observation, 1),
        range(segmentation, 1),
        range(transitions, 1),
        range(states, 1),
    ] {
        if let Err(status) = result {
            return status;
        }
    }
    let mixtures = match unsafe { mixture_inputs(mixtures, mixture_count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    match boundary(|| unsafe {
        (*statistics).statistics.collect_geometric(
            &(*model).parameters,
            &(*observation).observation,
            &(*segmentation).segmentation,
            GeometricPosteriorExpectation {
                transitions: &(*transitions).table,
                states: &(*states).table,
                mixtures: &mixtures,
                pruning_slope,
            },
        )
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}

/// Run one complete native HSMM expectation step and return its log likelihood.
/// Output validation precedes accumulation; failure preserves statistics and output.
/// # Safety
/// Owners are live and match the declared precision. Inputs and descriptors are
/// aligned initialized readable storage. The statistics owner is exclusively
/// writable and cannot alias inputs. Borrowed mixture-owner arrays contain count
/// live readable entries, which may repeat. Likelihood outputs are independent
/// aligned writable scalar storage and cannot alias any input or owner. No
/// concurrent mutation or release overlaps reads. All inputs remain caller-owned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_estimate_hsmm_f64(
    statistics: *mut LiblrhsmmRsModelStatisticsF64,
    model: *const LiblrhsmmRsModelF64,
    observation: *const LiblrhsmmRsObservationF64,
    segmentation: *const LiblrhsmmRsSegmentationF64,
    settings: *const LiblrhsmmRsHsmmSettingsF64,
    output: *mut f64,
) -> u32 {
    for result in [
        range(statistics, 1),
        range(model, 1),
        range(observation, 1),
        range(segmentation, 1),
        range(settings, 1),
    ] {
        if let Err(status) = result {
            return status;
        }
    }
    unsafe {
        scalar_result(output, || {
            (*statistics).statistics.estimate_hsmm(
                &(*model).parameters,
                &(*observation).observation,
                &(*segmentation).segmentation,
                settings.read().into(),
            )
        })
    }
}

/// Run one complete native geometric expectation step and return its log likelihood.
/// Output validation precedes accumulation; failure preserves statistics and output.
/// # Safety
/// Owners are live and match the declared precision. Inputs and descriptors are
/// aligned initialized readable storage. The statistics owner is exclusively
/// writable and cannot alias inputs. Borrowed mixture-owner arrays contain count
/// live readable entries, which may repeat. Likelihood outputs are independent
/// aligned writable scalar storage and cannot alias any input or owner. No
/// concurrent mutation or release overlaps reads. All inputs remain caller-owned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_estimate_geometric_f64(
    statistics: *mut LiblrhsmmRsModelStatisticsF64,
    model: *const LiblrhsmmRsModelF64,
    observation: *const LiblrhsmmRsObservationF64,
    segmentation: *const LiblrhsmmRsSegmentationF64,
    settings: *const LiblrhsmmRsGeometricSettingsF64,
    output: *mut f64,
) -> u32 {
    for result in [
        range(statistics, 1),
        range(model, 1),
        range(observation, 1),
        range(segmentation, 1),
        range(settings, 1),
    ] {
        if let Err(status) = result {
            return status;
        }
    }
    unsafe {
        scalar_result(output, || {
            (*statistics).statistics.estimate_geometric(
                &(*model).parameters,
                &(*observation).observation,
                &(*segmentation).segmentation,
                settings.read().into(),
            )
        })
    }
}
