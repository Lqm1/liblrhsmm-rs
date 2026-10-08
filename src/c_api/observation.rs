//! Complete observations with independent time-major feature streams.
use super::LiblrhsmmRsObservationStreamLayout;
use super::gaussian::storage_range;
use super::observation_stream::{LiblrhsmmRsObservationStreamF32, LiblrhsmmRsObservationStreamF64};
use super::{range, scalar_result};
use crate::data::ObservationData;
use crate::{ModelError, scalar::Scalar};

/// Shared frame count and number of independently shaped feature streams.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiblrhsmmRsObservationLayout {
    pub frames: usize,
    pub streams: usize,
}

unsafe fn copy_features<T: Scalar>(
    values: &[T],
    offset: usize,
    output: *mut T,
    count: usize,
) -> u32 {
    if let Err(status) = storage_range(output, count) {
        return status;
    }
    let Some(end) = offset.checked_add(count) else {
        return 2;
    };
    let Some(source) = values.get(offset..end) else {
        return 2;
    };
    if count != 0 {
        // SAFETY: Caller supplies independent initialized readable/writable buffers.
        unsafe {
            std::ptr::copy_nonoverlapping(source.as_ptr(), output, count);
        }
    }
    0
}
unsafe fn set_features<T: Scalar>(
    values: &mut [T],
    offset: usize,
    input: *const T,
    count: usize,
) -> u32 {
    if let Err(status) = storage_range(input, count) {
        return status;
    }
    let Some(end) = offset.checked_add(count) else {
        return 2;
    };
    let Some(destination) = values.get_mut(offset..end) else {
        return 2;
    };
    if count != 0 {
        // SAFETY: Caller supplies independent initialized readable/writable buffers.
        unsafe {
            std::ptr::copy_nonoverlapping(input, destination.as_mut_ptr(), count);
        }
    }
    0
}

/// Independently owned complete observation features.
pub struct LiblrhsmmRsObservationF32 {
    pub(super) observation: ObservationData<f32>,
}
unsafe fn input_f32(
    frames: usize,
    handles: *const *const LiblrhsmmRsObservationStreamF32,
    count: usize,
) -> Result<ObservationData<f32>, u32> {
    range(handles, count)?;
    let handles = if count == 0 {
        &[]
    } else {
        // SAFETY: Caller supplies a readable initialized borrowed owner array.
        unsafe { std::slice::from_raw_parts(handles, count) }
    };
    for &handle in handles {
        range(handle, 1)?;
    }
    super::boundary(|| {
        let mut streams = Vec::new();
        streams
            .try_reserve_exact(count)
            .map_err(|_| ModelError("observation allocation failed"))?;
        for &handle in handles {
            // SAFETY: Readable child remains live throughout the call.
            streams.push(unsafe { (*handle).stream.clone() });
        }
        let observation = ObservationData { frames, streams };
        observation.validate()?;
        Ok(observation)
    })
}

/// Allocate zeroed streams through the native observation constructor.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed child arrays and
/// scalar arrays are initialized, aligned and readable for their stated lengths.
/// Outputs are independent aligned writable storage. Buffers cannot alias owner
/// storage or each other. Reads cannot overlap mutation/release; mutation/release
/// requires exclusive access. Successful constructors receive slots with no live
/// owner. Release consumes unique ownership and clears its initialized slot.
/// Repeated borrowed children are permitted; returned children are deep copies.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_new_f32(
    frames: usize,
    dimensions: *const usize,
    streams: usize,
    output: *mut *mut LiblrhsmmRsObservationF32,
) -> u32 {
    if let Err(status) = range(dimensions, streams) {
        return status;
    }
    // SAFETY: Checked array and independent output satisfy the caller contract.
    unsafe {
        scalar_result(output, || {
            let dimensions = if streams == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(dimensions, streams)
            };
            Ok(Box::into_raw(Box::new(LiblrhsmmRsObservationF32 {
                observation: ObservationData::new(frames, dimensions)?,
            })))
        })
    }
}

/// Copy all stream shapes and features, validating the shared frame count.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed child arrays and
/// scalar arrays are initialized, aligned and readable for their stated lengths.
/// Outputs are independent aligned writable storage. Buffers cannot alias owner
/// storage or each other. Reads cannot overlap mutation/release; mutation/release
/// requires exclusive access. Successful constructors receive slots with no live
/// owner. Release consumes unique ownership and clears its initialized slot.
/// Repeated borrowed children are permitted; returned children are deep copies.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_create_f32(
    frames: usize,
    streams: *const *const LiblrhsmmRsObservationStreamF32,
    count: usize,
    output: *mut *mut LiblrhsmmRsObservationF32,
) -> u32 {
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Validated borrowed child array and independent output.
    let observation = match unsafe { input_f32(frames, streams, count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsObservationF32 {
                observation,
            })))
        })
    }
}

/// Replace the complete observation atomically with independent child copies.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed child arrays and
/// scalar arrays are initialized, aligned and readable for their stated lengths.
/// Outputs are independent aligned writable storage. Buffers cannot alias owner
/// storage or each other. Reads cannot overlap mutation/release; mutation/release
/// requires exclusive access. Successful constructors receive slots with no live
/// owner. Release consumes unique ownership and clears its initialized slot.
/// Repeated borrowed children are permitted; returned children are deep copies.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_replace_f32(
    handle: *mut LiblrhsmmRsObservationF32,
    frames: usize,
    streams: *const *const LiblrhsmmRsObservationStreamF32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live exclusive owner and readable borrowed children.
    let observation = match unsafe { input_f32(frames, streams, count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    unsafe {
        (*handle).observation = observation;
    }
    0
}

/// Read shared observation frame and stream counts.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed child arrays and
/// scalar arrays are initialized, aligned and readable for their stated lengths.
/// Outputs are independent aligned writable storage. Buffers cannot alias owner
/// storage or each other. Reads cannot overlap mutation/release; mutation/release
/// requires exclusive access. Successful constructors receive slots with no live
/// owner. Release consumes unique ownership and clears its initialized slot.
/// Repeated borrowed children are permitted; returned children are deep copies.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_layout_f32(
    handle: *const LiblrhsmmRsObservationF32,
    output: *mut LiblrhsmmRsObservationLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable owner and independent writable descriptor.
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsObservationLayout {
                frames: (*handle).observation.frames,
                streams: (*handle).observation.streams.len(),
            })
        })
    }
}

/// Read the complete layout of one independently shaped stream.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed child arrays and
/// scalar arrays are initialized, aligned and readable for their stated lengths.
/// Outputs are independent aligned writable storage. Buffers cannot alias owner
/// storage or each other. Reads cannot overlap mutation/release; mutation/release
/// requires exclusive access. Successful constructors receive slots with no live
/// owner. Release consumes unique ownership and clears its initialized slot.
/// Repeated borrowed children are permitted; returned children are deep copies.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_get_stream_layout_f32(
    handle: *const LiblrhsmmRsObservationF32,
    stream: usize,
    output: *mut LiblrhsmmRsObservationStreamLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable owner retained throughout the call.
    let observation = unsafe { &(*handle).observation };
    let Some(child) = observation.streams.get(stream) else {
        return 2;
    };
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsObservationStreamLayout {
                frames: observation.frames,
                dimensions: child.dimensions,
                values: child.values.len(),
            })
        })
    }
}

/// Return an independent editable copy of one feature stream.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed child arrays and
/// scalar arrays are initialized, aligned and readable for their stated lengths.
/// Outputs are independent aligned writable storage. Buffers cannot alias owner
/// storage or each other. Reads cannot overlap mutation/release; mutation/release
/// requires exclusive access. Successful constructors receive slots with no live
/// owner. Release consumes unique ownership and clears its initialized slot.
/// Repeated borrowed children are permitted; returned children are deep copies.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_get_stream_f32(
    handle: *const LiblrhsmmRsObservationF32,
    stream: usize,
    output: *mut *mut LiblrhsmmRsObservationStreamF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable owner and independent output ownership slot.
    let observation = unsafe { &(*handle).observation };
    let Some(child) = observation.streams.get(stream) else {
        return 2;
    };
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsObservationStreamF32 {
                stream: child.clone(),
            })))
        })
    }
}

/// Replace one stream with a deep copy matching the shared frame count.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed child arrays and
/// scalar arrays are initialized, aligned and readable for their stated lengths.
/// Outputs are independent aligned writable storage. Buffers cannot alias owner
/// storage or each other. Reads cannot overlap mutation/release; mutation/release
/// requires exclusive access. Successful constructors receive slots with no live
/// owner. Release consumes unique ownership and clears its initialized slot.
/// Repeated borrowed children are permitted; returned children are deep copies.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_set_stream_f32(
    handle: *mut LiblrhsmmRsObservationF32,
    stream: usize,
    input: *const LiblrhsmmRsObservationStreamF32,
) -> u32 {
    if let Err(status) = range(handle, 1).and_then(|()| range(input, 1)) {
        return status;
    }
    // SAFETY: Exclusive owner and independently borrowed readable child.
    let observation = unsafe { &(*handle).observation };
    if stream >= observation.streams.len() {
        return 2;
    }
    let replacement = match super::boundary(|| {
        let stream = unsafe { (*input).stream.clone() };
        let candidate = ObservationData {
            frames: observation.frames,
            streams: vec![stream],
        };
        candidate.validate()?;
        Ok(candidate)
    }) {
        Ok(mut candidate) => candidate.streams.remove(0),
        Err(status) => return status,
    };
    unsafe {
        (&mut (*handle).observation.streams)[stream] = replacement;
    }
    0
}

/// Copy a scalar range without exposing internal pointers.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed child arrays and
/// scalar arrays are initialized, aligned and readable for their stated lengths.
/// Outputs are independent aligned writable storage. Buffers cannot alias owner
/// storage or each other. Reads cannot overlap mutation/release; mutation/release
/// requires exclusive access. Successful constructors receive slots with no live
/// owner. Release consumes unique ownership and clears its initialized slot.
/// Repeated borrowed children are permitted; returned children are deep copies.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_copy_values_f32(
    handle: *const LiblrhsmmRsObservationF32,
    stream: usize,
    offset: usize,
    output: *mut f32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live owner retained for the entire independent buffer operation.
    let observation = unsafe { &(*handle).observation };
    let Some(child) = observation.streams.get(stream) else {
        return 2;
    };
    unsafe { copy_features(&child.values, offset, output, count) }
}

/// Update a scalar range atomically.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed child arrays and
/// scalar arrays are initialized, aligned and readable for their stated lengths.
/// Outputs are independent aligned writable storage. Buffers cannot alias owner
/// storage or each other. Reads cannot overlap mutation/release; mutation/release
/// requires exclusive access. Successful constructors receive slots with no live
/// owner. Release consumes unique ownership and clears its initialized slot.
/// Repeated borrowed children are permitted; returned children are deep copies.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_set_values_f32(
    handle: *mut LiblrhsmmRsObservationF32,
    stream: usize,
    offset: usize,
    input: *const f32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live owner retained for the entire independent buffer operation.
    let observation = unsafe { &mut (*handle).observation };
    let Some(child) = observation.streams.get_mut(stream) else {
        return 2;
    };
    unsafe { set_features(&mut child.values, offset, input, count) }
}

/// Copy a complete frame.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed child arrays and
/// scalar arrays are initialized, aligned and readable for their stated lengths.
/// Outputs are independent aligned writable storage. Buffers cannot alias owner
/// storage or each other. Reads cannot overlap mutation/release; mutation/release
/// requires exclusive access. Successful constructors receive slots with no live
/// owner. Release consumes unique ownership and clears its initialized slot.
/// Repeated borrowed children are permitted; returned children are deep copies.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_copy_frame_f32(
    handle: *const LiblrhsmmRsObservationF32,
    stream: usize,
    frame: usize,
    output: *mut f32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live owner retained for the entire independent buffer operation.
    let observation = unsafe { &(*handle).observation };
    let Some(values) = observation.frame(stream, frame) else {
        return 2;
    };
    if count != values.len() {
        return 2;
    }
    unsafe { copy_features(values, 0, output, count) }
}

/// Update a complete frame atomically.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed child arrays and
/// scalar arrays are initialized, aligned and readable for their stated lengths.
/// Outputs are independent aligned writable storage. Buffers cannot alias owner
/// storage or each other. Reads cannot overlap mutation/release; mutation/release
/// requires exclusive access. Successful constructors receive slots with no live
/// owner. Release consumes unique ownership and clears its initialized slot.
/// Repeated borrowed children are permitted; returned children are deep copies.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_set_frame_f32(
    handle: *mut LiblrhsmmRsObservationF32,
    stream: usize,
    frame: usize,
    input: *const f32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live owner retained for the entire independent buffer operation.
    let observation = unsafe { &mut (*handle).observation };
    if frame >= observation.frames {
        return 2;
    }
    let Some(child) = observation.streams.get_mut(stream) else {
        return 2;
    };
    if count != child.dimensions {
        return 2;
    }
    let Some(offset) = frame.checked_mul(child.dimensions) else {
        return 2;
    };
    unsafe { set_features(&mut child.values, offset, input, count) }
}

/// Deep-copy every stream and its feature bits.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed child arrays and
/// scalar arrays are initialized, aligned and readable for their stated lengths.
/// Outputs are independent aligned writable storage. Buffers cannot alias owner
/// storage or each other. Reads cannot overlap mutation/release; mutation/release
/// requires exclusive access. Successful constructors receive slots with no live
/// owner. Release consumes unique ownership and clears its initialized slot.
/// Repeated borrowed children are permitted; returned children are deep copies.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_clone_f32(
    handle: *const LiblrhsmmRsObservationF32,
    output: *mut *mut LiblrhsmmRsObservationF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable owner and independent writable ownership slot.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsObservationF32 {
                observation: (*handle).observation.clone(),
            })))
        })
    }
}

/// Release unique ownership and clear the initialized slot; null owners are accepted.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed child arrays and
/// scalar arrays are initialized, aligned and readable for their stated lengths.
/// Outputs are independent aligned writable storage. Buffers cannot alias owner
/// storage or each other. Reads cannot overlap mutation/release; mutation/release
/// requires exclusive access. Successful constructors receive slots with no live
/// owner. Release consumes unique ownership and clears its initialized slot.
/// Repeated borrowed children are permitted; returned children are deep copies.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_release_f32(
    handle: *mut *mut LiblrhsmmRsObservationF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Initialized exclusive ownership slot.
    let owner = unsafe { *handle };
    if owner.is_null() {
        return 0;
    }
    if let Err(status) = range(owner, 1) {
        return status;
    }
    unsafe {
        drop(Box::from_raw(owner));
        handle.write(std::ptr::null_mut());
    }
    0
}

/// Independently owned complete observation features.
pub struct LiblrhsmmRsObservationF64 {
    pub(super) observation: ObservationData<f64>,
}
unsafe fn input_f64(
    frames: usize,
    handles: *const *const LiblrhsmmRsObservationStreamF64,
    count: usize,
) -> Result<ObservationData<f64>, u32> {
    range(handles, count)?;
    let handles = if count == 0 {
        &[]
    } else {
        // SAFETY: Caller supplies a readable initialized borrowed owner array.
        unsafe { std::slice::from_raw_parts(handles, count) }
    };
    for &handle in handles {
        range(handle, 1)?;
    }
    super::boundary(|| {
        let mut streams = Vec::new();
        streams
            .try_reserve_exact(count)
            .map_err(|_| ModelError("observation allocation failed"))?;
        for &handle in handles {
            // SAFETY: Readable child remains live throughout the call.
            streams.push(unsafe { (*handle).stream.clone() });
        }
        let observation = ObservationData { frames, streams };
        observation.validate()?;
        Ok(observation)
    })
}

/// Allocate zeroed streams through the native observation constructor.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed child arrays and
/// scalar arrays are initialized, aligned and readable for their stated lengths.
/// Outputs are independent aligned writable storage. Buffers cannot alias owner
/// storage or each other. Reads cannot overlap mutation/release; mutation/release
/// requires exclusive access. Successful constructors receive slots with no live
/// owner. Release consumes unique ownership and clears its initialized slot.
/// Repeated borrowed children are permitted; returned children are deep copies.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_new_f64(
    frames: usize,
    dimensions: *const usize,
    streams: usize,
    output: *mut *mut LiblrhsmmRsObservationF64,
) -> u32 {
    if let Err(status) = range(dimensions, streams) {
        return status;
    }
    // SAFETY: Checked array and independent output satisfy the caller contract.
    unsafe {
        scalar_result(output, || {
            let dimensions = if streams == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(dimensions, streams)
            };
            Ok(Box::into_raw(Box::new(LiblrhsmmRsObservationF64 {
                observation: ObservationData::new(frames, dimensions)?,
            })))
        })
    }
}

/// Copy all stream shapes and features, validating the shared frame count.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed child arrays and
/// scalar arrays are initialized, aligned and readable for their stated lengths.
/// Outputs are independent aligned writable storage. Buffers cannot alias owner
/// storage or each other. Reads cannot overlap mutation/release; mutation/release
/// requires exclusive access. Successful constructors receive slots with no live
/// owner. Release consumes unique ownership and clears its initialized slot.
/// Repeated borrowed children are permitted; returned children are deep copies.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_create_f64(
    frames: usize,
    streams: *const *const LiblrhsmmRsObservationStreamF64,
    count: usize,
    output: *mut *mut LiblrhsmmRsObservationF64,
) -> u32 {
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Validated borrowed child array and independent output.
    let observation = match unsafe { input_f64(frames, streams, count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsObservationF64 {
                observation,
            })))
        })
    }
}

/// Replace the complete observation atomically with independent child copies.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed child arrays and
/// scalar arrays are initialized, aligned and readable for their stated lengths.
/// Outputs are independent aligned writable storage. Buffers cannot alias owner
/// storage or each other. Reads cannot overlap mutation/release; mutation/release
/// requires exclusive access. Successful constructors receive slots with no live
/// owner. Release consumes unique ownership and clears its initialized slot.
/// Repeated borrowed children are permitted; returned children are deep copies.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_replace_f64(
    handle: *mut LiblrhsmmRsObservationF64,
    frames: usize,
    streams: *const *const LiblrhsmmRsObservationStreamF64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live exclusive owner and readable borrowed children.
    let observation = match unsafe { input_f64(frames, streams, count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    unsafe {
        (*handle).observation = observation;
    }
    0
}

/// Read shared observation frame and stream counts.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed child arrays and
/// scalar arrays are initialized, aligned and readable for their stated lengths.
/// Outputs are independent aligned writable storage. Buffers cannot alias owner
/// storage or each other. Reads cannot overlap mutation/release; mutation/release
/// requires exclusive access. Successful constructors receive slots with no live
/// owner. Release consumes unique ownership and clears its initialized slot.
/// Repeated borrowed children are permitted; returned children are deep copies.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_layout_f64(
    handle: *const LiblrhsmmRsObservationF64,
    output: *mut LiblrhsmmRsObservationLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable owner and independent writable descriptor.
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsObservationLayout {
                frames: (*handle).observation.frames,
                streams: (*handle).observation.streams.len(),
            })
        })
    }
}

/// Read the complete layout of one independently shaped stream.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed child arrays and
/// scalar arrays are initialized, aligned and readable for their stated lengths.
/// Outputs are independent aligned writable storage. Buffers cannot alias owner
/// storage or each other. Reads cannot overlap mutation/release; mutation/release
/// requires exclusive access. Successful constructors receive slots with no live
/// owner. Release consumes unique ownership and clears its initialized slot.
/// Repeated borrowed children are permitted; returned children are deep copies.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_get_stream_layout_f64(
    handle: *const LiblrhsmmRsObservationF64,
    stream: usize,
    output: *mut LiblrhsmmRsObservationStreamLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable owner retained throughout the call.
    let observation = unsafe { &(*handle).observation };
    let Some(child) = observation.streams.get(stream) else {
        return 2;
    };
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsObservationStreamLayout {
                frames: observation.frames,
                dimensions: child.dimensions,
                values: child.values.len(),
            })
        })
    }
}

/// Return an independent editable copy of one feature stream.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed child arrays and
/// scalar arrays are initialized, aligned and readable for their stated lengths.
/// Outputs are independent aligned writable storage. Buffers cannot alias owner
/// storage or each other. Reads cannot overlap mutation/release; mutation/release
/// requires exclusive access. Successful constructors receive slots with no live
/// owner. Release consumes unique ownership and clears its initialized slot.
/// Repeated borrowed children are permitted; returned children are deep copies.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_get_stream_f64(
    handle: *const LiblrhsmmRsObservationF64,
    stream: usize,
    output: *mut *mut LiblrhsmmRsObservationStreamF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable owner and independent output ownership slot.
    let observation = unsafe { &(*handle).observation };
    let Some(child) = observation.streams.get(stream) else {
        return 2;
    };
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsObservationStreamF64 {
                stream: child.clone(),
            })))
        })
    }
}

/// Replace one stream with a deep copy matching the shared frame count.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed child arrays and
/// scalar arrays are initialized, aligned and readable for their stated lengths.
/// Outputs are independent aligned writable storage. Buffers cannot alias owner
/// storage or each other. Reads cannot overlap mutation/release; mutation/release
/// requires exclusive access. Successful constructors receive slots with no live
/// owner. Release consumes unique ownership and clears its initialized slot.
/// Repeated borrowed children are permitted; returned children are deep copies.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_set_stream_f64(
    handle: *mut LiblrhsmmRsObservationF64,
    stream: usize,
    input: *const LiblrhsmmRsObservationStreamF64,
) -> u32 {
    if let Err(status) = range(handle, 1).and_then(|()| range(input, 1)) {
        return status;
    }
    // SAFETY: Exclusive owner and independently borrowed readable child.
    let observation = unsafe { &(*handle).observation };
    if stream >= observation.streams.len() {
        return 2;
    }
    let replacement = match super::boundary(|| {
        let stream = unsafe { (*input).stream.clone() };
        let candidate = ObservationData {
            frames: observation.frames,
            streams: vec![stream],
        };
        candidate.validate()?;
        Ok(candidate)
    }) {
        Ok(mut candidate) => candidate.streams.remove(0),
        Err(status) => return status,
    };
    unsafe {
        (&mut (*handle).observation.streams)[stream] = replacement;
    }
    0
}

/// Copy a scalar range without exposing internal pointers.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed child arrays and
/// scalar arrays are initialized, aligned and readable for their stated lengths.
/// Outputs are independent aligned writable storage. Buffers cannot alias owner
/// storage or each other. Reads cannot overlap mutation/release; mutation/release
/// requires exclusive access. Successful constructors receive slots with no live
/// owner. Release consumes unique ownership and clears its initialized slot.
/// Repeated borrowed children are permitted; returned children are deep copies.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_copy_values_f64(
    handle: *const LiblrhsmmRsObservationF64,
    stream: usize,
    offset: usize,
    output: *mut f64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live owner retained for the entire independent buffer operation.
    let observation = unsafe { &(*handle).observation };
    let Some(child) = observation.streams.get(stream) else {
        return 2;
    };
    unsafe { copy_features(&child.values, offset, output, count) }
}

/// Update a scalar range atomically.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed child arrays and
/// scalar arrays are initialized, aligned and readable for their stated lengths.
/// Outputs are independent aligned writable storage. Buffers cannot alias owner
/// storage or each other. Reads cannot overlap mutation/release; mutation/release
/// requires exclusive access. Successful constructors receive slots with no live
/// owner. Release consumes unique ownership and clears its initialized slot.
/// Repeated borrowed children are permitted; returned children are deep copies.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_set_values_f64(
    handle: *mut LiblrhsmmRsObservationF64,
    stream: usize,
    offset: usize,
    input: *const f64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live owner retained for the entire independent buffer operation.
    let observation = unsafe { &mut (*handle).observation };
    let Some(child) = observation.streams.get_mut(stream) else {
        return 2;
    };
    unsafe { set_features(&mut child.values, offset, input, count) }
}

/// Copy a complete frame.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed child arrays and
/// scalar arrays are initialized, aligned and readable for their stated lengths.
/// Outputs are independent aligned writable storage. Buffers cannot alias owner
/// storage or each other. Reads cannot overlap mutation/release; mutation/release
/// requires exclusive access. Successful constructors receive slots with no live
/// owner. Release consumes unique ownership and clears its initialized slot.
/// Repeated borrowed children are permitted; returned children are deep copies.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_copy_frame_f64(
    handle: *const LiblrhsmmRsObservationF64,
    stream: usize,
    frame: usize,
    output: *mut f64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live owner retained for the entire independent buffer operation.
    let observation = unsafe { &(*handle).observation };
    let Some(values) = observation.frame(stream, frame) else {
        return 2;
    };
    if count != values.len() {
        return 2;
    }
    unsafe { copy_features(values, 0, output, count) }
}

/// Update a complete frame atomically.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed child arrays and
/// scalar arrays are initialized, aligned and readable for their stated lengths.
/// Outputs are independent aligned writable storage. Buffers cannot alias owner
/// storage or each other. Reads cannot overlap mutation/release; mutation/release
/// requires exclusive access. Successful constructors receive slots with no live
/// owner. Release consumes unique ownership and clears its initialized slot.
/// Repeated borrowed children are permitted; returned children are deep copies.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_set_frame_f64(
    handle: *mut LiblrhsmmRsObservationF64,
    stream: usize,
    frame: usize,
    input: *const f64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live owner retained for the entire independent buffer operation.
    let observation = unsafe { &mut (*handle).observation };
    if frame >= observation.frames {
        return 2;
    }
    let Some(child) = observation.streams.get_mut(stream) else {
        return 2;
    };
    if count != child.dimensions {
        return 2;
    }
    let Some(offset) = frame.checked_mul(child.dimensions) else {
        return 2;
    };
    unsafe { set_features(&mut child.values, offset, input, count) }
}

/// Deep-copy every stream and its feature bits.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed child arrays and
/// scalar arrays are initialized, aligned and readable for their stated lengths.
/// Outputs are independent aligned writable storage. Buffers cannot alias owner
/// storage or each other. Reads cannot overlap mutation/release; mutation/release
/// requires exclusive access. Successful constructors receive slots with no live
/// owner. Release consumes unique ownership and clears its initialized slot.
/// Repeated borrowed children are permitted; returned children are deep copies.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_clone_f64(
    handle: *const LiblrhsmmRsObservationF64,
    output: *mut *mut LiblrhsmmRsObservationF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable owner and independent writable ownership slot.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsObservationF64 {
                observation: (*handle).observation.clone(),
            })))
        })
    }
}

/// Release unique ownership and clear the initialized slot; null owners are accepted.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed child arrays and
/// scalar arrays are initialized, aligned and readable for their stated lengths.
/// Outputs are independent aligned writable storage. Buffers cannot alias owner
/// storage or each other. Reads cannot overlap mutation/release; mutation/release
/// requires exclusive access. Successful constructors receive slots with no live
/// owner. Release consumes unique ownership and clears its initialized slot.
/// Repeated borrowed children are permitted; returned children are deep copies.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_release_f64(
    handle: *mut *mut LiblrhsmmRsObservationF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Initialized exclusive ownership slot.
    let owner = unsafe { *handle };
    if owner.is_null() {
        return 0;
    }
    if let Err(status) = range(owner, 1) {
        return status;
    }
    unsafe {
        drop(Box::from_raw(owner));
        handle.write(std::ptr::null_mut());
    }
    0
}
