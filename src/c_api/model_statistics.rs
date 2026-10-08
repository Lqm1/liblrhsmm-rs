//! Complete model sufficient-statistics ownership and native transactional updates.
use super::{
    LiblrhsmmRsDurationStatisticsF32, LiblrhsmmRsDurationStatisticsF64, LiblrhsmmRsModelF32,
    LiblrhsmmRsModelF64, LiblrhsmmRsModelLayout, LiblrhsmmRsStreamStatisticsF32,
    LiblrhsmmRsStreamStatisticsF64, LiblrhsmmRsUpdateOptionsF32, LiblrhsmmRsUpdateOptionsF64,
    boundary, range, scalar_result,
};
use crate::{ModelError, statistics::ModelSufficientStatistics};
/// Independently owned editable model statistics.
pub struct LiblrhsmmRsModelStatisticsF32 {
    pub(super) statistics: ModelSufficientStatistics<f32>,
}
unsafe fn input_f32(
    streams: *const *const LiblrhsmmRsStreamStatisticsF32,
    stream_count: usize,
    durations: *const *const LiblrhsmmRsDurationStatisticsF32,
    duration_count: usize,
) -> Result<ModelSufficientStatistics<f32>, u32> {
    range(streams, stream_count)?;
    range(durations, duration_count)?;
    // SAFETY: Initialized readable arrays; empty arrays permit null.
    let streams = if stream_count == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(streams, stream_count) }
    };
    let durations = if duration_count == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(durations, duration_count) }
    };
    for &handle in streams {
        range(handle, 1)?;
    }
    for &handle in durations {
        range(handle, 1)?;
    }
    super::boundary(|| {
        let mut value = ModelSufficientStatistics {
            streams: Vec::new(),
            durations: Vec::new(),
        };
        value
            .streams
            .try_reserve_exact(stream_count)
            .map_err(|_| ModelError("stream allocation failed"))?;
        value
            .durations
            .try_reserve_exact(duration_count)
            .map_err(|_| ModelError("duration allocation failed"))?;
        for &handle in streams {
            // SAFETY: Validated readable child retained throughout this call.
            value.streams.push(unsafe { (*handle).statistics.clone() });
        }
        for &handle in durations {
            // SAFETY: Validated readable child retained throughout this call.
            value.durations.push(unsafe { (*handle).statistics });
        }
        Ok(value)
    })
}
/// Copy complete stream/duration arrays; retain inputs and preserve storage on failure.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_create_f32(
    streams: *const *const LiblrhsmmRsStreamStatisticsF32,
    stream_count: usize,
    durations: *const *const LiblrhsmmRsDurationStatisticsF32,
    duration_count: usize,
    output: *mut *mut LiblrhsmmRsModelStatisticsF32,
) -> u32 {
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Independent borrowed child arrays meet the input contract.
    let value = match unsafe { input_f32(streams, stream_count, durations, duration_count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsModelStatisticsF32 {
                statistics: value,
            })))
        })
    }
}

/// Copy complete stream/duration arrays; retain inputs and preserve storage on failure.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_replace_f32(
    handle: *mut LiblrhsmmRsModelStatisticsF32,
    streams: *const *const LiblrhsmmRsStreamStatisticsF32,
    stream_count: usize,
    durations: *const *const LiblrhsmmRsDurationStatisticsF32,
    duration_count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Independent borrowed child arrays meet the input contract.
    let value = match unsafe { input_f32(streams, stream_count, durations, duration_count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Exclusive owner; complete validated copy is ready.
    unsafe {
        (*handle).statistics = value;
    }
    0
}

/// Read both model counts without exposing internal storage.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_layout_f32(
    handle: *const LiblrhsmmRsModelStatisticsF32,
    output: *mut LiblrhsmmRsModelLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsModelLayout {
                streams: (*handle).statistics.streams.len(),
                durations: (*handle).statistics.durations.len(),
            })
        })
    }
}

/// Clone one stream and all nested fields into an independent editable owner.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_get_stream_f32(
    handle: *const LiblrhsmmRsModelStatisticsF32,
    index: usize,
    output: *mut *mut LiblrhsmmRsStreamStatisticsF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Readable live owner; checked child index.
    let Some(value) = (unsafe { &(*handle).statistics.streams }).get(index) else {
        return 2;
    };
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStreamStatisticsF32 {
                statistics: value.clone(),
            })))
        })
    }
}

/// Replace one stream with a deep copy, retaining the input owner.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_set_stream_f32(
    handle: *mut LiblrhsmmRsModelStatisticsF32,
    index: usize,
    child: *const LiblrhsmmRsStreamStatisticsF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(child, 1) {
        return status;
    }
    // SAFETY: Exclusive model and independent readable child.
    if index >= unsafe { (*handle).statistics.streams.len() } {
        return 2;
    }
    let value = match super::boundary(|| {
        let value = unsafe { (*child).statistics.clone() };
        Ok(value)
    }) {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Checked index, exclusive access; deep copy complete.
    unsafe {
        (&mut (*handle).statistics.streams)[index] = value;
    }
    0
}

/// Clone one duration and all nested fields into an independent editable owner.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_get_duration_f32(
    handle: *const LiblrhsmmRsModelStatisticsF32,
    index: usize,
    output: *mut *mut LiblrhsmmRsDurationStatisticsF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Readable live owner; checked child index.
    let Some(value) = (unsafe { &(*handle).statistics.durations }).get(index) else {
        return 2;
    };
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsDurationStatisticsF32 {
                statistics: *value,
            })))
        })
    }
}

/// Replace one duration with a deep copy, retaining the input owner.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_set_duration_f32(
    handle: *mut LiblrhsmmRsModelStatisticsF32,
    index: usize,
    child: *const LiblrhsmmRsDurationStatisticsF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(child, 1) {
        return status;
    }
    // SAFETY: Exclusive model and independent readable child.
    if index >= unsafe { (*handle).statistics.durations.len() } {
        return 2;
    }
    let value = match super::boundary(|| {
        let value = unsafe { (*child).statistics };
        Ok(value)
    }) {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Checked index, exclusive access; deep copy complete.
    unsafe {
        (&mut (*handle).statistics.durations)[index] = value;
    }
    0
}

/// Clone all streams/Gaussians and durations into independent model storage.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_clone_f32(
    handle: *const LiblrhsmmRsModelStatisticsF32,
    output: *mut *mut LiblrhsmmRsModelStatisticsF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsModelStatisticsF32 {
                statistics: (*handle).statistics.clone(),
            })))
        })
    }
}

/// Release the unique model and all owned children; clear its initialized slot.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_release_f32(
    handle: *mut *mut LiblrhsmmRsModelStatisticsF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Initialized unique owner slot, null or a live aligned allocation.
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

/// Independently owned editable model statistics.
pub struct LiblrhsmmRsModelStatisticsF64 {
    pub(super) statistics: ModelSufficientStatistics<f64>,
}
unsafe fn input_f64(
    streams: *const *const LiblrhsmmRsStreamStatisticsF64,
    stream_count: usize,
    durations: *const *const LiblrhsmmRsDurationStatisticsF64,
    duration_count: usize,
) -> Result<ModelSufficientStatistics<f64>, u32> {
    range(streams, stream_count)?;
    range(durations, duration_count)?;
    // SAFETY: Initialized readable arrays; empty arrays permit null.
    let streams = if stream_count == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(streams, stream_count) }
    };
    let durations = if duration_count == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(durations, duration_count) }
    };
    for &handle in streams {
        range(handle, 1)?;
    }
    for &handle in durations {
        range(handle, 1)?;
    }
    super::boundary(|| {
        let mut value = ModelSufficientStatistics {
            streams: Vec::new(),
            durations: Vec::new(),
        };
        value
            .streams
            .try_reserve_exact(stream_count)
            .map_err(|_| ModelError("stream allocation failed"))?;
        value
            .durations
            .try_reserve_exact(duration_count)
            .map_err(|_| ModelError("duration allocation failed"))?;
        for &handle in streams {
            // SAFETY: Validated readable child retained throughout this call.
            value.streams.push(unsafe { (*handle).statistics.clone() });
        }
        for &handle in durations {
            // SAFETY: Validated readable child retained throughout this call.
            value.durations.push(unsafe { (*handle).statistics });
        }
        Ok(value)
    })
}
/// Copy complete stream/duration arrays; retain inputs and preserve storage on failure.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_create_f64(
    streams: *const *const LiblrhsmmRsStreamStatisticsF64,
    stream_count: usize,
    durations: *const *const LiblrhsmmRsDurationStatisticsF64,
    duration_count: usize,
    output: *mut *mut LiblrhsmmRsModelStatisticsF64,
) -> u32 {
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Independent borrowed child arrays meet the input contract.
    let value = match unsafe { input_f64(streams, stream_count, durations, duration_count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsModelStatisticsF64 {
                statistics: value,
            })))
        })
    }
}

/// Copy complete stream/duration arrays; retain inputs and preserve storage on failure.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_replace_f64(
    handle: *mut LiblrhsmmRsModelStatisticsF64,
    streams: *const *const LiblrhsmmRsStreamStatisticsF64,
    stream_count: usize,
    durations: *const *const LiblrhsmmRsDurationStatisticsF64,
    duration_count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Independent borrowed child arrays meet the input contract.
    let value = match unsafe { input_f64(streams, stream_count, durations, duration_count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Exclusive owner; complete validated copy is ready.
    unsafe {
        (*handle).statistics = value;
    }
    0
}

/// Read both model counts without exposing internal storage.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_layout_f64(
    handle: *const LiblrhsmmRsModelStatisticsF64,
    output: *mut LiblrhsmmRsModelLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsModelLayout {
                streams: (*handle).statistics.streams.len(),
                durations: (*handle).statistics.durations.len(),
            })
        })
    }
}

/// Clone one stream and all nested fields into an independent editable owner.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_get_stream_f64(
    handle: *const LiblrhsmmRsModelStatisticsF64,
    index: usize,
    output: *mut *mut LiblrhsmmRsStreamStatisticsF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Readable live owner; checked child index.
    let Some(value) = (unsafe { &(*handle).statistics.streams }).get(index) else {
        return 2;
    };
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStreamStatisticsF64 {
                statistics: value.clone(),
            })))
        })
    }
}

/// Replace one stream with a deep copy, retaining the input owner.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_set_stream_f64(
    handle: *mut LiblrhsmmRsModelStatisticsF64,
    index: usize,
    child: *const LiblrhsmmRsStreamStatisticsF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(child, 1) {
        return status;
    }
    // SAFETY: Exclusive model and independent readable child.
    if index >= unsafe { (*handle).statistics.streams.len() } {
        return 2;
    }
    let value = match super::boundary(|| {
        let value = unsafe { (*child).statistics.clone() };
        Ok(value)
    }) {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Checked index, exclusive access; deep copy complete.
    unsafe {
        (&mut (*handle).statistics.streams)[index] = value;
    }
    0
}

/// Clone one duration and all nested fields into an independent editable owner.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_get_duration_f64(
    handle: *const LiblrhsmmRsModelStatisticsF64,
    index: usize,
    output: *mut *mut LiblrhsmmRsDurationStatisticsF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Readable live owner; checked child index.
    let Some(value) = (unsafe { &(*handle).statistics.durations }).get(index) else {
        return 2;
    };
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsDurationStatisticsF64 {
                statistics: *value,
            })))
        })
    }
}

/// Replace one duration with a deep copy, retaining the input owner.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_set_duration_f64(
    handle: *mut LiblrhsmmRsModelStatisticsF64,
    index: usize,
    child: *const LiblrhsmmRsDurationStatisticsF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(child, 1) {
        return status;
    }
    // SAFETY: Exclusive model and independent readable child.
    if index >= unsafe { (*handle).statistics.durations.len() } {
        return 2;
    }
    let value = match super::boundary(|| {
        let value = unsafe { (*child).statistics };
        Ok(value)
    }) {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Checked index, exclusive access; deep copy complete.
    unsafe {
        (&mut (*handle).statistics.durations)[index] = value;
    }
    0
}

/// Clone all streams/Gaussians and durations into independent model storage.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_clone_f64(
    handle: *const LiblrhsmmRsModelStatisticsF64,
    output: *mut *mut LiblrhsmmRsModelStatisticsF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsModelStatisticsF64 {
                statistics: (*handle).statistics.clone(),
            })))
        })
    }
}

/// Release the unique model and all owned children; clear its initialized slot.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_release_f64(
    handle: *mut *mut LiblrhsmmRsModelStatisticsF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Initialized unique owner slot, null or a live aligned allocation.
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
/// Create every native stream/Gaussian/duration accumulator without retaining the model.
/// # Safety
/// Handles are live owners of the declared precision. Inputs and ownership slots
/// are initialized aligned readable storage; outputs are aligned independently
/// writable. Inputs/outputs cannot alias owners. Mutation/release is exclusive;
/// no concurrent mutation/release overlaps reads. Constructor outputs are unowned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_from_model_f32(
    handle: *const LiblrhsmmRsModelF32,
    output: *mut *mut LiblrhsmmRsModelStatisticsF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Readable model and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsModelStatisticsF32 {
                statistics: ModelSufficientStatistics::from_model(&(*handle).parameters)?,
            })))
        })
    }
}
/// Reset every native accumulator while retaining the complete nested shape.
/// # Safety
/// Handles are live owners of the declared precision. Inputs and ownership slots
/// are initialized aligned readable storage; outputs are aligned independently
/// writable. Inputs/outputs cannot alias owners. Mutation/release is exclusive;
/// no concurrent mutation/release overlaps reads. Constructor outputs are unowned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_clear_f32(
    handle: *mut LiblrhsmmRsModelStatisticsF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Exclusive owner; native clear retains shapes and zeroes all values.
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
/// Delegate full native maximization; all model parameters remain unchanged on error.
/// # Safety
/// Handles are live owners of the declared precision. Inputs and ownership slots
/// are initialized aligned readable storage; outputs are aligned independently
/// writable. Inputs/outputs cannot alias owners. Mutation/release is exclusive;
/// no concurrent mutation/release overlaps reads. Constructor outputs are unowned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_update_f32(
    handle: *mut LiblrhsmmRsModelF32,
    statistics: *const LiblrhsmmRsModelStatisticsF32,
    options: *const LiblrhsmmRsUpdateOptionsF32,
) -> u32 {
    for result in [range(handle, 1), range(statistics, 1), range(options, 1)] {
        if let Err(status) = result {
            return status;
        }
    }
    // SAFETY: Exclusive model; independent readable statistics/options. Native update stages publication.
    match boundary(|| unsafe {
        (*handle)
            .parameters
            .update(&(*statistics).statistics, options.read().try_into()?)
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}
/// Create every native stream/Gaussian/duration accumulator without retaining the model.
/// # Safety
/// Handles are live owners of the declared precision. Inputs and ownership slots
/// are initialized aligned readable storage; outputs are aligned independently
/// writable. Inputs/outputs cannot alias owners. Mutation/release is exclusive;
/// no concurrent mutation/release overlaps reads. Constructor outputs are unowned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_from_model_f64(
    handle: *const LiblrhsmmRsModelF64,
    output: *mut *mut LiblrhsmmRsModelStatisticsF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Readable model and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsModelStatisticsF64 {
                statistics: ModelSufficientStatistics::from_model(&(*handle).parameters)?,
            })))
        })
    }
}
/// Reset every native accumulator while retaining the complete nested shape.
/// # Safety
/// Handles are live owners of the declared precision. Inputs and ownership slots
/// are initialized aligned readable storage; outputs are aligned independently
/// writable. Inputs/outputs cannot alias owners. Mutation/release is exclusive;
/// no concurrent mutation/release overlaps reads. Constructor outputs are unowned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_statistics_clear_f64(
    handle: *mut LiblrhsmmRsModelStatisticsF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Exclusive owner; native clear retains shapes and zeroes all values.
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
/// Delegate full native maximization; all model parameters remain unchanged on error.
/// # Safety
/// Handles are live owners of the declared precision. Inputs and ownership slots
/// are initialized aligned readable storage; outputs are aligned independently
/// writable. Inputs/outputs cannot alias owners. Mutation/release is exclusive;
/// no concurrent mutation/release overlaps reads. Constructor outputs are unowned.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_update_f64(
    handle: *mut LiblrhsmmRsModelF64,
    statistics: *const LiblrhsmmRsModelStatisticsF64,
    options: *const LiblrhsmmRsUpdateOptionsF64,
) -> u32 {
    for result in [range(handle, 1), range(statistics, 1), range(options, 1)] {
        if let Err(status) = result {
            return status;
        }
    }
    // SAFETY: Exclusive model; independent readable statistics/options. Native update stages publication.
    match boundary(|| unsafe {
        (*handle)
            .parameters
            .update(&(*statistics).statistics, options.read().try_into()?)
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}
