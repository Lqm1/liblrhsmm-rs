//! Independently editable stream sufficient statistics and native maximization.
use super::{
    LiblrhsmmRsGaussianStatisticsF32, LiblrhsmmRsGaussianStatisticsF64, LiblrhsmmRsStreamF32,
    LiblrhsmmRsStreamF64, LiblrhsmmRsUpdateOptionsF32, LiblrhsmmRsUpdateOptionsF64, boundary,
    range, scalar_result,
};
use crate::{ModelError, statistics::StreamSufficientStatistics};
/// Independent editable stream statistics.
pub struct LiblrhsmmRsStreamStatisticsF32 {
    pub(super) statistics: StreamSufficientStatistics<f32>,
}
unsafe fn input_f32(
    emissions: *const *const LiblrhsmmRsGaussianStatisticsF32,
    count: usize,
) -> Result<StreamSufficientStatistics<f32>, u32> {
    range(emissions, count)?;
    // SAFETY: Initialized readable array; empty arrays permit null.
    let emissions = if count == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(emissions, count) }
    };
    for &child in emissions {
        range(child, 1)?;
    }
    boundary(|| {
        let mut mixtures = Vec::new();
        mixtures
            .try_reserve_exact(count)
            .map_err(|_| ModelError("statistics allocation failed"))?;
        for &child in emissions {
            // SAFETY: Validated readable child retained throughout this call.
            mixtures.push(unsafe { (*child).statistics.clone() });
        }
        Ok(StreamSufficientStatistics { mixtures })
    })
}
/// Copy all emission accumulators, including arbitrary scalar bits and mixed shapes.
/// # Safety
/// Handles are live owners of the declared precision. Arrays/descriptors and
/// ownership slots are initialized and readable; outputs are aligned independent
/// writable storage. Inputs/outputs cannot alias owner storage. Mutation/release
/// requires exclusive access; no concurrent mutation/release overlaps reads. Child
/// inputs are borrowed only for this call and may repeat; child outputs are deep copies.
/// Constructor slots cannot overwrite live ownership. Release consumes unique
/// ownership and clears its slot; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_statistics_create_f32(
    emissions: *const *const LiblrhsmmRsGaussianStatisticsF32,
    count: usize,
    output: *mut *mut LiblrhsmmRsStreamStatisticsF32,
) -> u32 {
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Independent readable borrowed child array.
    let value = match unsafe { input_f32(emissions, count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Live readable inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStreamStatisticsF32 {
                statistics: value,
            })))
        })
    }
}

/// Copy all emission accumulators, including arbitrary scalar bits and mixed shapes.
/// # Safety
/// Handles are live owners of the declared precision. Arrays/descriptors and
/// ownership slots are initialized and readable; outputs are aligned independent
/// writable storage. Inputs/outputs cannot alias owner storage. Mutation/release
/// requires exclusive access; no concurrent mutation/release overlaps reads. Child
/// inputs are borrowed only for this call and may repeat; child outputs are deep copies.
/// Constructor slots cannot overwrite live ownership. Release consumes unique
/// ownership and clears its slot; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_statistics_replace_f32(
    handle: *mut LiblrhsmmRsStreamStatisticsF32,
    emissions: *const *const LiblrhsmmRsGaussianStatisticsF32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Independent readable borrowed child array.
    let value = match unsafe { input_f32(emissions, count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Exclusive owner; complete deep copy is ready.
    unsafe {
        (*handle).statistics = value;
    }
    0
}

/// Construct zero accumulators matching every native stream emission shape.
/// # Safety
/// Handles are live owners of the declared precision. Arrays/descriptors and
/// ownership slots are initialized and readable; outputs are aligned independent
/// writable storage. Inputs/outputs cannot alias owner storage. Mutation/release
/// requires exclusive access; no concurrent mutation/release overlaps reads. Child
/// inputs are borrowed only for this call and may repeat; child outputs are deep copies.
/// Constructor slots cannot overwrite live ownership. Release consumes unique
/// ownership and clears its slot; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_statistics_from_stream_f32(
    handle: *const LiblrhsmmRsStreamF32,
    output: *mut *mut LiblrhsmmRsStreamStatisticsF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStreamStatisticsF32 {
                statistics: StreamSufficientStatistics::from_stream(&(*handle).parameters)?,
            })))
        })
    }
}

/// Read the total emission count without exposing owned storage.
/// # Safety
/// Handles are live owners of the declared precision. Arrays/descriptors and
/// ownership slots are initialized and readable; outputs are aligned independent
/// writable storage. Inputs/outputs cannot alias owner storage. Mutation/release
/// requires exclusive access; no concurrent mutation/release overlaps reads. Child
/// inputs are borrowed only for this call and may repeat; child outputs are deep copies.
/// Constructor slots cannot overwrite live ownership. Release consumes unique
/// ownership and clears its slot; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_statistics_count_f32(
    handle: *const LiblrhsmmRsStreamStatisticsF32,
    output: *mut usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable inputs and independent writable output.
    unsafe { scalar_result(output, || Ok((*handle).statistics.mixtures.len())) }
}

/// Return an independent Gaussian-statistics owner containing every accumulator field.
/// # Safety
/// Handles are live owners of the declared precision. Arrays/descriptors and
/// ownership slots are initialized and readable; outputs are aligned independent
/// writable storage. Inputs/outputs cannot alias owner storage. Mutation/release
/// requires exclusive access; no concurrent mutation/release overlaps reads. Child
/// inputs are borrowed only for this call and may repeat; child outputs are deep copies.
/// Constructor slots cannot overwrite live ownership. Release consumes unique
/// ownership and clears its slot; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_statistics_get_emission_f32(
    handle: *const LiblrhsmmRsStreamStatisticsF32,
    index: usize,
    output: *mut *mut LiblrhsmmRsGaussianStatisticsF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Readable owner and checked emission index.
    let Some(value) = (unsafe { &(*handle).statistics.mixtures }).get(index) else {
        return 2;
    };
    // SAFETY: Live readable inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianStatisticsF32 {
                statistics: value.clone(),
            })))
        })
    }
}

/// Deep-copy one accumulator; differing Gaussian shapes are retained until native update validation.
/// # Safety
/// Handles are live owners of the declared precision. Arrays/descriptors and
/// ownership slots are initialized and readable; outputs are aligned independent
/// writable storage. Inputs/outputs cannot alias owner storage. Mutation/release
/// requires exclusive access; no concurrent mutation/release overlaps reads. Child
/// inputs are borrowed only for this call and may repeat; child outputs are deep copies.
/// Constructor slots cannot overwrite live ownership. Release consumes unique
/// ownership and clears its slot; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_statistics_set_emission_f32(
    handle: *mut LiblrhsmmRsStreamStatisticsF32,
    index: usize,
    child: *const LiblrhsmmRsGaussianStatisticsF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(child, 1) {
        return status;
    }
    // SAFETY: Exclusive stream statistics and independent readable child.
    if index >= unsafe { (*handle).statistics.mixtures.len() } {
        return 2;
    }
    let value = match boundary(|| Ok(unsafe { (*child).statistics.clone() })) {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Checked index, exclusive access; complete deep copy is ready.
    unsafe {
        (&mut (*handle).statistics.mixtures)[index] = value;
    }
    0
}

/// Deep-clone all accumulator shapes, vectors and state occupancies.
/// # Safety
/// Handles are live owners of the declared precision. Arrays/descriptors and
/// ownership slots are initialized and readable; outputs are aligned independent
/// writable storage. Inputs/outputs cannot alias owner storage. Mutation/release
/// requires exclusive access; no concurrent mutation/release overlaps reads. Child
/// inputs are borrowed only for this call and may repeat; child outputs are deep copies.
/// Constructor slots cannot overwrite live ownership. Release consumes unique
/// ownership and clears its slot; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_statistics_clone_f32(
    handle: *const LiblrhsmmRsStreamStatisticsF32,
    output: *mut *mut LiblrhsmmRsStreamStatisticsF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStreamStatisticsF32 {
                statistics: (*handle).statistics.clone(),
            })))
        })
    }
}

/// Use native clear to reset every accumulator while retaining emission shapes.
/// # Safety
/// Handles are live owners of the declared precision. Arrays/descriptors and
/// ownership slots are initialized and readable; outputs are aligned independent
/// writable storage. Inputs/outputs cannot alias owner storage. Mutation/release
/// requires exclusive access; no concurrent mutation/release overlaps reads. Child
/// inputs are borrowed only for this call and may repeat; child outputs are deep copies.
/// Constructor slots cannot overwrite live ownership. Release consumes unique
/// ownership and clears its slot; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_statistics_clear_f32(
    handle: *mut LiblrhsmmRsStreamStatisticsF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Exclusive owner; native clear preserves every shape and zeroes all accumulators.
    match boundary(|| {
        unsafe {
            (*handle).statistics.clear();
        }
        Ok(())
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}

/// Release unique stream statistics and every child vector; clear the ownership slot.
/// # Safety
/// Handles are live owners of the declared precision. Arrays/descriptors and
/// ownership slots are initialized and readable; outputs are aligned independent
/// writable storage. Inputs/outputs cannot alias owner storage. Mutation/release
/// requires exclusive access; no concurrent mutation/release overlaps reads. Child
/// inputs are borrowed only for this call and may repeat; child outputs are deep copies.
/// Constructor slots cannot overwrite live ownership. Release consumes unique
/// ownership and clears its slot; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_statistics_release_f32(
    handle: *mut *mut LiblrhsmmRsStreamStatisticsF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Initialized unique ownership slot, null or a live owner.
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

/// Delegate complete native stream maximization; errors retain all parameter fields.
/// # Safety
/// Handles are live owners of the declared precision. Arrays/descriptors and
/// ownership slots are initialized and readable; outputs are aligned independent
/// writable storage. Inputs/outputs cannot alias owner storage. Mutation/release
/// requires exclusive access; no concurrent mutation/release overlaps reads. Child
/// inputs are borrowed only for this call and may repeat; child outputs are deep copies.
/// Constructor slots cannot overwrite live ownership. Release consumes unique
/// ownership and clears its slot; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_update_f32(
    handle: *mut LiblrhsmmRsStreamF32,
    statistics: *const LiblrhsmmRsStreamStatisticsF32,
    options: *const LiblrhsmmRsUpdateOptionsF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(statistics, 1) {
        return status;
    }
    if let Err(status) = range(options, 1) {
        return status;
    }
    // SAFETY: Exclusive model and independent readable complete statistics/options.
    // Native stream update stages publication until every emission succeeds.
    match boundary(|| unsafe {
        (*handle)
            .parameters
            .update(&(*statistics).statistics, options.read().try_into()?)
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}

/// Independent editable stream statistics.
pub struct LiblrhsmmRsStreamStatisticsF64 {
    pub(super) statistics: StreamSufficientStatistics<f64>,
}
unsafe fn input_f64(
    emissions: *const *const LiblrhsmmRsGaussianStatisticsF64,
    count: usize,
) -> Result<StreamSufficientStatistics<f64>, u32> {
    range(emissions, count)?;
    // SAFETY: Initialized readable array; empty arrays permit null.
    let emissions = if count == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(emissions, count) }
    };
    for &child in emissions {
        range(child, 1)?;
    }
    boundary(|| {
        let mut mixtures = Vec::new();
        mixtures
            .try_reserve_exact(count)
            .map_err(|_| ModelError("statistics allocation failed"))?;
        for &child in emissions {
            // SAFETY: Validated readable child retained throughout this call.
            mixtures.push(unsafe { (*child).statistics.clone() });
        }
        Ok(StreamSufficientStatistics { mixtures })
    })
}
/// Copy all emission accumulators, including arbitrary scalar bits and mixed shapes.
/// # Safety
/// Handles are live owners of the declared precision. Arrays/descriptors and
/// ownership slots are initialized and readable; outputs are aligned independent
/// writable storage. Inputs/outputs cannot alias owner storage. Mutation/release
/// requires exclusive access; no concurrent mutation/release overlaps reads. Child
/// inputs are borrowed only for this call and may repeat; child outputs are deep copies.
/// Constructor slots cannot overwrite live ownership. Release consumes unique
/// ownership and clears its slot; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_statistics_create_f64(
    emissions: *const *const LiblrhsmmRsGaussianStatisticsF64,
    count: usize,
    output: *mut *mut LiblrhsmmRsStreamStatisticsF64,
) -> u32 {
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Independent readable borrowed child array.
    let value = match unsafe { input_f64(emissions, count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Live readable inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStreamStatisticsF64 {
                statistics: value,
            })))
        })
    }
}

/// Copy all emission accumulators, including arbitrary scalar bits and mixed shapes.
/// # Safety
/// Handles are live owners of the declared precision. Arrays/descriptors and
/// ownership slots are initialized and readable; outputs are aligned independent
/// writable storage. Inputs/outputs cannot alias owner storage. Mutation/release
/// requires exclusive access; no concurrent mutation/release overlaps reads. Child
/// inputs are borrowed only for this call and may repeat; child outputs are deep copies.
/// Constructor slots cannot overwrite live ownership. Release consumes unique
/// ownership and clears its slot; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_statistics_replace_f64(
    handle: *mut LiblrhsmmRsStreamStatisticsF64,
    emissions: *const *const LiblrhsmmRsGaussianStatisticsF64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Independent readable borrowed child array.
    let value = match unsafe { input_f64(emissions, count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Exclusive owner; complete deep copy is ready.
    unsafe {
        (*handle).statistics = value;
    }
    0
}

/// Construct zero accumulators matching every native stream emission shape.
/// # Safety
/// Handles are live owners of the declared precision. Arrays/descriptors and
/// ownership slots are initialized and readable; outputs are aligned independent
/// writable storage. Inputs/outputs cannot alias owner storage. Mutation/release
/// requires exclusive access; no concurrent mutation/release overlaps reads. Child
/// inputs are borrowed only for this call and may repeat; child outputs are deep copies.
/// Constructor slots cannot overwrite live ownership. Release consumes unique
/// ownership and clears its slot; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_statistics_from_stream_f64(
    handle: *const LiblrhsmmRsStreamF64,
    output: *mut *mut LiblrhsmmRsStreamStatisticsF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStreamStatisticsF64 {
                statistics: StreamSufficientStatistics::from_stream(&(*handle).parameters)?,
            })))
        })
    }
}

/// Read the total emission count without exposing owned storage.
/// # Safety
/// Handles are live owners of the declared precision. Arrays/descriptors and
/// ownership slots are initialized and readable; outputs are aligned independent
/// writable storage. Inputs/outputs cannot alias owner storage. Mutation/release
/// requires exclusive access; no concurrent mutation/release overlaps reads. Child
/// inputs are borrowed only for this call and may repeat; child outputs are deep copies.
/// Constructor slots cannot overwrite live ownership. Release consumes unique
/// ownership and clears its slot; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_statistics_count_f64(
    handle: *const LiblrhsmmRsStreamStatisticsF64,
    output: *mut usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable inputs and independent writable output.
    unsafe { scalar_result(output, || Ok((*handle).statistics.mixtures.len())) }
}

/// Return an independent Gaussian-statistics owner containing every accumulator field.
/// # Safety
/// Handles are live owners of the declared precision. Arrays/descriptors and
/// ownership slots are initialized and readable; outputs are aligned independent
/// writable storage. Inputs/outputs cannot alias owner storage. Mutation/release
/// requires exclusive access; no concurrent mutation/release overlaps reads. Child
/// inputs are borrowed only for this call and may repeat; child outputs are deep copies.
/// Constructor slots cannot overwrite live ownership. Release consumes unique
/// ownership and clears its slot; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_statistics_get_emission_f64(
    handle: *const LiblrhsmmRsStreamStatisticsF64,
    index: usize,
    output: *mut *mut LiblrhsmmRsGaussianStatisticsF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Readable owner and checked emission index.
    let Some(value) = (unsafe { &(*handle).statistics.mixtures }).get(index) else {
        return 2;
    };
    // SAFETY: Live readable inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianStatisticsF64 {
                statistics: value.clone(),
            })))
        })
    }
}

/// Deep-copy one accumulator; differing Gaussian shapes are retained until native update validation.
/// # Safety
/// Handles are live owners of the declared precision. Arrays/descriptors and
/// ownership slots are initialized and readable; outputs are aligned independent
/// writable storage. Inputs/outputs cannot alias owner storage. Mutation/release
/// requires exclusive access; no concurrent mutation/release overlaps reads. Child
/// inputs are borrowed only for this call and may repeat; child outputs are deep copies.
/// Constructor slots cannot overwrite live ownership. Release consumes unique
/// ownership and clears its slot; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_statistics_set_emission_f64(
    handle: *mut LiblrhsmmRsStreamStatisticsF64,
    index: usize,
    child: *const LiblrhsmmRsGaussianStatisticsF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(child, 1) {
        return status;
    }
    // SAFETY: Exclusive stream statistics and independent readable child.
    if index >= unsafe { (*handle).statistics.mixtures.len() } {
        return 2;
    }
    let value = match boundary(|| Ok(unsafe { (*child).statistics.clone() })) {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Checked index, exclusive access; complete deep copy is ready.
    unsafe {
        (&mut (*handle).statistics.mixtures)[index] = value;
    }
    0
}

/// Deep-clone all accumulator shapes, vectors and state occupancies.
/// # Safety
/// Handles are live owners of the declared precision. Arrays/descriptors and
/// ownership slots are initialized and readable; outputs are aligned independent
/// writable storage. Inputs/outputs cannot alias owner storage. Mutation/release
/// requires exclusive access; no concurrent mutation/release overlaps reads. Child
/// inputs are borrowed only for this call and may repeat; child outputs are deep copies.
/// Constructor slots cannot overwrite live ownership. Release consumes unique
/// ownership and clears its slot; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_statistics_clone_f64(
    handle: *const LiblrhsmmRsStreamStatisticsF64,
    output: *mut *mut LiblrhsmmRsStreamStatisticsF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStreamStatisticsF64 {
                statistics: (*handle).statistics.clone(),
            })))
        })
    }
}

/// Use native clear to reset every accumulator while retaining emission shapes.
/// # Safety
/// Handles are live owners of the declared precision. Arrays/descriptors and
/// ownership slots are initialized and readable; outputs are aligned independent
/// writable storage. Inputs/outputs cannot alias owner storage. Mutation/release
/// requires exclusive access; no concurrent mutation/release overlaps reads. Child
/// inputs are borrowed only for this call and may repeat; child outputs are deep copies.
/// Constructor slots cannot overwrite live ownership. Release consumes unique
/// ownership and clears its slot; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_statistics_clear_f64(
    handle: *mut LiblrhsmmRsStreamStatisticsF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Exclusive owner; native clear preserves every shape and zeroes all accumulators.
    match boundary(|| {
        unsafe {
            (*handle).statistics.clear();
        }
        Ok(())
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}

/// Release unique stream statistics and every child vector; clear the ownership slot.
/// # Safety
/// Handles are live owners of the declared precision. Arrays/descriptors and
/// ownership slots are initialized and readable; outputs are aligned independent
/// writable storage. Inputs/outputs cannot alias owner storage. Mutation/release
/// requires exclusive access; no concurrent mutation/release overlaps reads. Child
/// inputs are borrowed only for this call and may repeat; child outputs are deep copies.
/// Constructor slots cannot overwrite live ownership. Release consumes unique
/// ownership and clears its slot; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_statistics_release_f64(
    handle: *mut *mut LiblrhsmmRsStreamStatisticsF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Initialized unique ownership slot, null or a live owner.
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

/// Delegate complete native stream maximization; errors retain all parameter fields.
/// # Safety
/// Handles are live owners of the declared precision. Arrays/descriptors and
/// ownership slots are initialized and readable; outputs are aligned independent
/// writable storage. Inputs/outputs cannot alias owner storage. Mutation/release
/// requires exclusive access; no concurrent mutation/release overlaps reads. Child
/// inputs are borrowed only for this call and may repeat; child outputs are deep copies.
/// Constructor slots cannot overwrite live ownership. Release consumes unique
/// ownership and clears its slot; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_update_f64(
    handle: *mut LiblrhsmmRsStreamF64,
    statistics: *const LiblrhsmmRsStreamStatisticsF64,
    options: *const LiblrhsmmRsUpdateOptionsF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(statistics, 1) {
        return status;
    }
    if let Err(status) = range(options, 1) {
        return status;
    }
    // SAFETY: Exclusive model and independent readable complete statistics/options.
    // Native stream update stages publication until every emission succeeds.
    match boundary(|| unsafe {
        (*handle)
            .parameters
            .update(&(*statistics).statistics, options.read().try_into()?)
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}
