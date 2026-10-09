//! Complete model parameter ownership with independent stream and duration children.
use super::{
    LiblrhsmmRsDurationF32, LiblrhsmmRsDurationF64, LiblrhsmmRsStreamF32, LiblrhsmmRsStreamF64,
    range, scalar_result,
};
use crate::model::{ModelError, ModelParameters, StreamLayout};
/// Flat-start specification for one stream; individual model streams may differ.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiblrhsmmRsStreamSpecification {
    pub emissions: usize,
    pub components: usize,
    pub dimensions: usize,
}
impl From<LiblrhsmmRsStreamSpecification> for StreamLayout {
    fn from(value: LiblrhsmmRsStreamSpecification) -> Self {
        Self {
            emissions: value.emissions,
            components: value.components,
            dimensions: value.dimensions,
        }
    }
}
/// Independent stream and duration counts.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiblrhsmmRsModelLayout {
    pub streams: usize,
    pub durations: usize,
}
/// Independently owned editable model parameters.
pub struct LiblrhsmmRsModelF32 {
    pub(super) parameters: ModelParameters<f32>,
}
unsafe fn input_f32(
    streams: *const *const LiblrhsmmRsStreamF32,
    stream_count: usize,
    durations: *const *const LiblrhsmmRsDurationF32,
    duration_count: usize,
) -> Result<ModelParameters<f32>, u32> {
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
        let mut value = ModelParameters {
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
            value.streams.push(unsafe { (*handle).parameters.clone() });
        }
        for &handle in durations {
            // SAFETY: Validated readable child retained throughout this call.
            value
                .durations
                .push(unsafe { (*handle).parameters.clone() });
        }
        value.validate()?;
        Ok(value)
    })
}
/// Construct native flat-start streams and every default duration distribution.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_new_f32(
    layouts: *const LiblrhsmmRsStreamSpecification,
    stream_count: usize,
    duration_count: usize,
    output: *mut *mut LiblrhsmmRsModelF32,
) -> u32 {
    if let Err(status) = range(output, 1) {
        return status;
    }
    if let Err(status) = range(layouts, stream_count) {
        return status;
    }
    // SAFETY: Validated readable specifications; empty input permits null.
    let layouts = if stream_count == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(layouts, stream_count) }
    };
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            let mut specifications = Vec::new();
            specifications
                .try_reserve_exact(layouts.len())
                .map_err(|_| ModelError("layout allocation failed"))?;
            specifications.extend(layouts.iter().copied().map(StreamLayout::from));
            Ok(Box::into_raw(Box::new(LiblrhsmmRsModelF32 {
                parameters: ModelParameters::new(&specifications, duration_count)?,
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
pub unsafe extern "C" fn liblrhsmm_rs_model_create_f32(
    streams: *const *const LiblrhsmmRsStreamF32,
    stream_count: usize,
    durations: *const *const LiblrhsmmRsDurationF32,
    duration_count: usize,
    output: *mut *mut LiblrhsmmRsModelF32,
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
            Ok(Box::into_raw(Box::new(LiblrhsmmRsModelF32 {
                parameters: value,
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
pub unsafe extern "C" fn liblrhsmm_rs_model_replace_f32(
    handle: *mut LiblrhsmmRsModelF32,
    streams: *const *const LiblrhsmmRsStreamF32,
    stream_count: usize,
    durations: *const *const LiblrhsmmRsDurationF32,
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
        (*handle).parameters = value;
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
pub unsafe extern "C" fn liblrhsmm_rs_model_layout_f32(
    handle: *const LiblrhsmmRsModelF32,
    output: *mut LiblrhsmmRsModelLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsModelLayout {
                streams: (*handle).parameters.streams.len(),
                durations: (*handle).parameters.durations.len(),
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
pub unsafe extern "C" fn liblrhsmm_rs_model_get_stream_f32(
    handle: *const LiblrhsmmRsModelF32,
    index: usize,
    output: *mut *mut LiblrhsmmRsStreamF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Readable live owner; checked child index.
    let Some(value) = (unsafe { &(*handle).parameters.streams }).get(index) else {
        return 2;
    };
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStreamF32 {
                parameters: value.clone(),
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
pub unsafe extern "C" fn liblrhsmm_rs_model_set_stream_f32(
    handle: *mut LiblrhsmmRsModelF32,
    index: usize,
    child: *const LiblrhsmmRsStreamF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(child, 1) {
        return status;
    }
    // SAFETY: Exclusive model and independent readable child.
    if index >= unsafe { (*handle).parameters.streams.len() } {
        return 2;
    }
    let value = match super::boundary(|| {
        let value = unsafe { (*child).parameters.clone() };
        value.validate()?;
        Ok(value)
    }) {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Checked index, exclusive access; deep copy complete.
    unsafe {
        (&mut (*handle).parameters.streams)[index] = value;
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
pub unsafe extern "C" fn liblrhsmm_rs_model_get_duration_f32(
    handle: *const LiblrhsmmRsModelF32,
    index: usize,
    output: *mut *mut LiblrhsmmRsDurationF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Readable live owner; checked child index.
    let Some(value) = (unsafe { &(*handle).parameters.durations }).get(index) else {
        return 2;
    };
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsDurationF32 {
                parameters: value.clone(),
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
pub unsafe extern "C" fn liblrhsmm_rs_model_set_duration_f32(
    handle: *mut LiblrhsmmRsModelF32,
    index: usize,
    child: *const LiblrhsmmRsDurationF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(child, 1) {
        return status;
    }
    // SAFETY: Exclusive model and independent readable child.
    if index >= unsafe { (*handle).parameters.durations.len() } {
        return 2;
    }
    let value = match super::boundary(|| {
        let value = unsafe { (*child).parameters.clone() };
        Ok(value)
    }) {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Checked index, exclusive access; deep copy complete.
    unsafe {
        (&mut (*handle).parameters.durations)[index] = value;
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
pub unsafe extern "C" fn liblrhsmm_rs_model_clone_f32(
    handle: *const LiblrhsmmRsModelF32,
    output: *mut *mut LiblrhsmmRsModelF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsModelF32 {
                parameters: (*handle).parameters.clone(),
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
pub unsafe extern "C" fn liblrhsmm_rs_model_release_f32(
    handle: *mut *mut LiblrhsmmRsModelF32,
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

/// Independently owned editable model parameters.
pub struct LiblrhsmmRsModelF64 {
    pub(super) parameters: ModelParameters<f64>,
}
unsafe fn input_f64(
    streams: *const *const LiblrhsmmRsStreamF64,
    stream_count: usize,
    durations: *const *const LiblrhsmmRsDurationF64,
    duration_count: usize,
) -> Result<ModelParameters<f64>, u32> {
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
        let mut value = ModelParameters {
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
            value.streams.push(unsafe { (*handle).parameters.clone() });
        }
        for &handle in durations {
            // SAFETY: Validated readable child retained throughout this call.
            value
                .durations
                .push(unsafe { (*handle).parameters.clone() });
        }
        value.validate()?;
        Ok(value)
    })
}
/// Construct native flat-start streams and every default duration distribution.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_new_f64(
    layouts: *const LiblrhsmmRsStreamSpecification,
    stream_count: usize,
    duration_count: usize,
    output: *mut *mut LiblrhsmmRsModelF64,
) -> u32 {
    if let Err(status) = range(output, 1) {
        return status;
    }
    if let Err(status) = range(layouts, stream_count) {
        return status;
    }
    // SAFETY: Validated readable specifications; empty input permits null.
    let layouts = if stream_count == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(layouts, stream_count) }
    };
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            let mut specifications = Vec::new();
            specifications
                .try_reserve_exact(layouts.len())
                .map_err(|_| ModelError("layout allocation failed"))?;
            specifications.extend(layouts.iter().copied().map(StreamLayout::from));
            Ok(Box::into_raw(Box::new(LiblrhsmmRsModelF64 {
                parameters: ModelParameters::new(&specifications, duration_count)?,
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
pub unsafe extern "C" fn liblrhsmm_rs_model_create_f64(
    streams: *const *const LiblrhsmmRsStreamF64,
    stream_count: usize,
    durations: *const *const LiblrhsmmRsDurationF64,
    duration_count: usize,
    output: *mut *mut LiblrhsmmRsModelF64,
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
            Ok(Box::into_raw(Box::new(LiblrhsmmRsModelF64 {
                parameters: value,
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
pub unsafe extern "C" fn liblrhsmm_rs_model_replace_f64(
    handle: *mut LiblrhsmmRsModelF64,
    streams: *const *const LiblrhsmmRsStreamF64,
    stream_count: usize,
    durations: *const *const LiblrhsmmRsDurationF64,
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
        (*handle).parameters = value;
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
pub unsafe extern "C" fn liblrhsmm_rs_model_layout_f64(
    handle: *const LiblrhsmmRsModelF64,
    output: *mut LiblrhsmmRsModelLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsModelLayout {
                streams: (*handle).parameters.streams.len(),
                durations: (*handle).parameters.durations.len(),
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
pub unsafe extern "C" fn liblrhsmm_rs_model_get_stream_f64(
    handle: *const LiblrhsmmRsModelF64,
    index: usize,
    output: *mut *mut LiblrhsmmRsStreamF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Readable live owner; checked child index.
    let Some(value) = (unsafe { &(*handle).parameters.streams }).get(index) else {
        return 2;
    };
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStreamF64 {
                parameters: value.clone(),
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
pub unsafe extern "C" fn liblrhsmm_rs_model_set_stream_f64(
    handle: *mut LiblrhsmmRsModelF64,
    index: usize,
    child: *const LiblrhsmmRsStreamF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(child, 1) {
        return status;
    }
    // SAFETY: Exclusive model and independent readable child.
    if index >= unsafe { (*handle).parameters.streams.len() } {
        return 2;
    }
    let value = match super::boundary(|| {
        let value = unsafe { (*child).parameters.clone() };
        value.validate()?;
        Ok(value)
    }) {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Checked index, exclusive access; deep copy complete.
    unsafe {
        (&mut (*handle).parameters.streams)[index] = value;
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
pub unsafe extern "C" fn liblrhsmm_rs_model_get_duration_f64(
    handle: *const LiblrhsmmRsModelF64,
    index: usize,
    output: *mut *mut LiblrhsmmRsDurationF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Readable live owner; checked child index.
    let Some(value) = (unsafe { &(*handle).parameters.durations }).get(index) else {
        return 2;
    };
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsDurationF64 {
                parameters: value.clone(),
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
pub unsafe extern "C" fn liblrhsmm_rs_model_set_duration_f64(
    handle: *mut LiblrhsmmRsModelF64,
    index: usize,
    child: *const LiblrhsmmRsDurationF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(child, 1) {
        return status;
    }
    // SAFETY: Exclusive model and independent readable child.
    if index >= unsafe { (*handle).parameters.durations.len() } {
        return 2;
    }
    let value = match super::boundary(|| {
        let value = unsafe { (*child).parameters.clone() };
        Ok(value)
    }) {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Checked index, exclusive access; deep copy complete.
    unsafe {
        (&mut (*handle).parameters.durations)[index] = value;
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
pub unsafe extern "C" fn liblrhsmm_rs_model_clone_f64(
    handle: *const LiblrhsmmRsModelF64,
    output: *mut *mut LiblrhsmmRsModelF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsModelF64 {
                parameters: (*handle).parameters.clone(),
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
pub unsafe extern "C" fn liblrhsmm_rs_model_release_f64(
    handle: *mut *mut LiblrhsmmRsModelF64,
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
