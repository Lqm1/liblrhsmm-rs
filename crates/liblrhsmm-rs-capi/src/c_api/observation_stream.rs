//! Independently owned time-major observation streams.
use super::gaussian::storage_range;
use super::{boundary, range, scalar_result};
use crate::data::{ObservationData, ObservationStreamData};
use crate::{ModelError, scalar::Scalar};

/// Complete stream shape; zero frames are supported with positive dimensions.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiblrhsmmRsObservationStreamLayout {
    pub frames: usize,
    pub dimensions: usize,
    pub values: usize,
}

unsafe fn input<T: Scalar + std::panic::RefUnwindSafe>(
    dimensions: usize,
    values: *const T,
    count: usize,
) -> Result<ObservationStreamData<T>, u32> {
    storage_range(values, count)?;
    if dimensions == 0
        || dimensions > i32::MAX as usize
        || !count.is_multiple_of(dimensions)
        || count / dimensions > i32::MAX as usize
    {
        return Err(3);
    }
    boundary(|| {
        let mut copied = Vec::new();
        copied
            .try_reserve_exact(count)
            .map_err(|_| ModelError("observation stream allocation failed"))?;
        if count != 0 {
            // SAFETY: Caller provides independent readable initialized scalar storage.
            copied.extend_from_slice(unsafe { std::slice::from_raw_parts(values, count) });
        }
        Ok(ObservationStreamData {
            dimensions,
            values: copied,
        })
    })
}

fn interval(offset: usize, count: usize, length: usize) -> Result<std::ops::Range<usize>, u32> {
    let end = offset.checked_add(count).ok_or(2u32)?;
    if end > length {
        return Err(2);
    }
    Ok(offset..end)
}
/// Independently owned editable feature storage. Arbitrary scalar bits are retained.
pub struct LiblrhsmmRsObservationStreamF32 {
    pub(super) stream: ObservationStreamData<f32>,
}
/// Allocate zeroed feature storage using the native observation constructor.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Buffers cannot alias owner storage. Mutation/release requires exclusive access;
/// reads cannot overlap mutation/release. Successful constructors receive slots
/// holding no previous live owner. Release consumes unique ownership and clears
/// its initialized slot. Null owned values are accepted. No internal pointer escapes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_stream_new_f32(
    frames: usize,
    dimensions: usize,
    output: *mut *mut LiblrhsmmRsObservationStreamF32,
) -> u32 {
    // SAFETY: Checked live owners and independent buffers satisfy the caller contract.
    unsafe {
        scalar_result(output, || {
            let mut observation = ObservationData::<f32>::new(frames, &[dimensions])?;
            let stream = observation
                .streams
                .pop()
                .ok_or(ModelError("observation stream is missing"))?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsObservationStreamF32 {
                stream,
            })))
        })
    }
}
/// Copy complete dimensions/features atomically; preserve inputs and existing storage on error.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Buffers cannot alias owner storage. Mutation/release requires exclusive access;
/// reads cannot overlap mutation/release. Successful constructors receive slots
/// holding no previous live owner. Release consumes unique ownership and clears
/// its initialized slot. Null owned values are accepted. No internal pointer escapes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_stream_create_f32(
    dimensions: usize,
    values: *const f32,
    count: usize,
    output: *mut *mut LiblrhsmmRsObservationStreamF32,
) -> u32 {
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the caller contract.
    let stream = match unsafe { input(dimensions, values, count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsObservationStreamF32 {
                stream,
            })))
        })
    }
}
/// Copy complete dimensions/features atomically; preserve inputs and existing storage on error.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Buffers cannot alias owner storage. Mutation/release requires exclusive access;
/// reads cannot overlap mutation/release. Successful constructors receive slots
/// holding no previous live owner. Release consumes unique ownership and clears
/// its initialized slot. Null owned values are accepted. No internal pointer escapes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_stream_replace_f32(
    handle: *mut LiblrhsmmRsObservationStreamF32,
    dimensions: usize,
    values: *const f32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the caller contract.
    let stream = match unsafe { input(dimensions, values, count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    unsafe {
        (*handle).stream = stream;
    }
    0
}
/// Read complete frame/dimension/value counts without exposing internal storage.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Buffers cannot alias owner storage. Mutation/release requires exclusive access;
/// reads cannot overlap mutation/release. Successful constructors receive slots
/// holding no previous live owner. Release consumes unique ownership and clears
/// its initialized slot. Null owned values are accepted. No internal pointer escapes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_stream_layout_f32(
    handle: *const LiblrhsmmRsObservationStreamF32,
    output: *mut LiblrhsmmRsObservationStreamLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the caller contract.
    unsafe {
        scalar_result(output, || {
            let stream = &(*handle).stream;
            Ok(LiblrhsmmRsObservationStreamLayout {
                frames: stream.values.len() / stream.dimensions,
                dimensions: stream.dimensions,
                values: stream.values.len(),
            })
        })
    }
}
/// Copy checked scalar storage.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Buffers cannot alias owner storage. Mutation/release requires exclusive access;
/// reads cannot overlap mutation/release. Successful constructors receive slots
/// holding no previous live owner. Release consumes unique ownership and clears
/// its initialized slot. Null owned values are accepted. No internal pointer escapes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_stream_copy_values_f32(
    handle: *const LiblrhsmmRsObservationStreamF32,
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
    // SAFETY: Checked live owners and independent buffers satisfy the caller contract.
    let stream = unsafe { &(*handle).stream };
    let indices = match interval(offset, count, stream.values.len()) {
        Ok(value) => value,
        Err(status) => return status,
    };
    if count != 0 {
        unsafe {
            std::ptr::copy_nonoverlapping(stream.values[indices].as_ptr(), output, count);
        }
    }
    0
}
/// Replace checked scalar storage without changing shape.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Buffers cannot alias owner storage. Mutation/release requires exclusive access;
/// reads cannot overlap mutation/release. Successful constructors receive slots
/// holding no previous live owner. Release consumes unique ownership and clears
/// its initialized slot. Null owned values are accepted. No internal pointer escapes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_stream_set_values_f32(
    handle: *mut LiblrhsmmRsObservationStreamF32,
    offset: usize,
    input: *const f32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = storage_range(input, count) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the caller contract.
    let stream = unsafe { &mut (*handle).stream };
    let indices = match interval(offset, count, stream.values.len()) {
        Ok(value) => value,
        Err(status) => return status,
    };
    if count != 0 {
        unsafe {
            stream.values[indices].copy_from_slice(std::slice::from_raw_parts(input, count));
        }
    }
    0
}
/// Copy a checked complete frame.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Buffers cannot alias owner storage. Mutation/release requires exclusive access;
/// reads cannot overlap mutation/release. Successful constructors receive slots
/// holding no previous live owner. Release consumes unique ownership and clears
/// its initialized slot. Null owned values are accepted. No internal pointer escapes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_stream_copy_frame_f32(
    handle: *const LiblrhsmmRsObservationStreamF32,
    frame: usize,
    output: *mut f32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = storage_range(output, count) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the caller contract.
    let stream = unsafe { &(*handle).stream };
    if count != stream.dimensions || frame >= stream.values.len() / stream.dimensions {
        return 2;
    }
    let Some(offset) = frame.checked_mul(stream.dimensions) else {
        return 2;
    };
    let indices = match interval(offset, count, stream.values.len()) {
        Ok(value) => value,
        Err(status) => return status,
    };
    if count != 0 {
        unsafe {
            std::ptr::copy_nonoverlapping(stream.values[indices].as_ptr(), output, count);
        }
    }
    0
}
/// Replace a checked complete frame.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Buffers cannot alias owner storage. Mutation/release requires exclusive access;
/// reads cannot overlap mutation/release. Successful constructors receive slots
/// holding no previous live owner. Release consumes unique ownership and clears
/// its initialized slot. Null owned values are accepted. No internal pointer escapes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_stream_set_frame_f32(
    handle: *mut LiblrhsmmRsObservationStreamF32,
    frame: usize,
    input: *const f32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = storage_range(input, count) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the caller contract.
    let stream = unsafe { &mut (*handle).stream };
    if count != stream.dimensions || frame >= stream.values.len() / stream.dimensions {
        return 2;
    }
    let Some(offset) = frame.checked_mul(stream.dimensions) else {
        return 2;
    };
    let indices = match interval(offset, count, stream.values.len()) {
        Ok(value) => value,
        Err(status) => return status,
    };
    if count != 0 {
        unsafe {
            stream.values[indices].copy_from_slice(std::slice::from_raw_parts(input, count));
        }
    }
    0
}
/// Clone all dimensions and feature bits into independent editable storage.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Buffers cannot alias owner storage. Mutation/release requires exclusive access;
/// reads cannot overlap mutation/release. Successful constructors receive slots
/// holding no previous live owner. Release consumes unique ownership and clears
/// its initialized slot. Null owned values are accepted. No internal pointer escapes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_stream_clone_f32(
    handle: *const LiblrhsmmRsObservationStreamF32,
    output: *mut *mut LiblrhsmmRsObservationStreamF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the caller contract.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsObservationStreamF32 {
                stream: (*handle).stream.clone(),
            })))
        })
    }
}
/// Release feature storage and clear unique ownership; null owned values are a no-op.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Buffers cannot alias owner storage. Mutation/release requires exclusive access;
/// reads cannot overlap mutation/release. Successful constructors receive slots
/// holding no previous live owner. Release consumes unique ownership and clears
/// its initialized slot. Null owned values are accepted. No internal pointer escapes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_stream_release_f32(
    handle: *mut *mut LiblrhsmmRsObservationStreamF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the caller contract.
    let value = unsafe { handle.read() };
    if !value.is_null() {
        if let Err(status) = range(value, 1) {
            return status;
        }
        unsafe {
            drop(Box::from_raw(value));
            handle.write(std::ptr::null_mut());
        }
    }
    0
}
/// Independently owned editable feature storage. Arbitrary scalar bits are retained.
pub struct LiblrhsmmRsObservationStreamF64 {
    pub(super) stream: ObservationStreamData<f64>,
}
/// Allocate zeroed feature storage using the native observation constructor.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Buffers cannot alias owner storage. Mutation/release requires exclusive access;
/// reads cannot overlap mutation/release. Successful constructors receive slots
/// holding no previous live owner. Release consumes unique ownership and clears
/// its initialized slot. Null owned values are accepted. No internal pointer escapes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_stream_new_f64(
    frames: usize,
    dimensions: usize,
    output: *mut *mut LiblrhsmmRsObservationStreamF64,
) -> u32 {
    // SAFETY: Checked live owners and independent buffers satisfy the caller contract.
    unsafe {
        scalar_result(output, || {
            let mut observation = ObservationData::<f64>::new(frames, &[dimensions])?;
            let stream = observation
                .streams
                .pop()
                .ok_or(ModelError("observation stream is missing"))?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsObservationStreamF64 {
                stream,
            })))
        })
    }
}
/// Copy complete dimensions/features atomically; preserve inputs and existing storage on error.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Buffers cannot alias owner storage. Mutation/release requires exclusive access;
/// reads cannot overlap mutation/release. Successful constructors receive slots
/// holding no previous live owner. Release consumes unique ownership and clears
/// its initialized slot. Null owned values are accepted. No internal pointer escapes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_stream_create_f64(
    dimensions: usize,
    values: *const f64,
    count: usize,
    output: *mut *mut LiblrhsmmRsObservationStreamF64,
) -> u32 {
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the caller contract.
    let stream = match unsafe { input(dimensions, values, count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsObservationStreamF64 {
                stream,
            })))
        })
    }
}
/// Copy complete dimensions/features atomically; preserve inputs and existing storage on error.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Buffers cannot alias owner storage. Mutation/release requires exclusive access;
/// reads cannot overlap mutation/release. Successful constructors receive slots
/// holding no previous live owner. Release consumes unique ownership and clears
/// its initialized slot. Null owned values are accepted. No internal pointer escapes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_stream_replace_f64(
    handle: *mut LiblrhsmmRsObservationStreamF64,
    dimensions: usize,
    values: *const f64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the caller contract.
    let stream = match unsafe { input(dimensions, values, count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    unsafe {
        (*handle).stream = stream;
    }
    0
}
/// Read complete frame/dimension/value counts without exposing internal storage.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Buffers cannot alias owner storage. Mutation/release requires exclusive access;
/// reads cannot overlap mutation/release. Successful constructors receive slots
/// holding no previous live owner. Release consumes unique ownership and clears
/// its initialized slot. Null owned values are accepted. No internal pointer escapes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_stream_layout_f64(
    handle: *const LiblrhsmmRsObservationStreamF64,
    output: *mut LiblrhsmmRsObservationStreamLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the caller contract.
    unsafe {
        scalar_result(output, || {
            let stream = &(*handle).stream;
            Ok(LiblrhsmmRsObservationStreamLayout {
                frames: stream.values.len() / stream.dimensions,
                dimensions: stream.dimensions,
                values: stream.values.len(),
            })
        })
    }
}
/// Copy checked scalar storage.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Buffers cannot alias owner storage. Mutation/release requires exclusive access;
/// reads cannot overlap mutation/release. Successful constructors receive slots
/// holding no previous live owner. Release consumes unique ownership and clears
/// its initialized slot. Null owned values are accepted. No internal pointer escapes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_stream_copy_values_f64(
    handle: *const LiblrhsmmRsObservationStreamF64,
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
    // SAFETY: Checked live owners and independent buffers satisfy the caller contract.
    let stream = unsafe { &(*handle).stream };
    let indices = match interval(offset, count, stream.values.len()) {
        Ok(value) => value,
        Err(status) => return status,
    };
    if count != 0 {
        unsafe {
            std::ptr::copy_nonoverlapping(stream.values[indices].as_ptr(), output, count);
        }
    }
    0
}
/// Replace checked scalar storage without changing shape.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Buffers cannot alias owner storage. Mutation/release requires exclusive access;
/// reads cannot overlap mutation/release. Successful constructors receive slots
/// holding no previous live owner. Release consumes unique ownership and clears
/// its initialized slot. Null owned values are accepted. No internal pointer escapes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_stream_set_values_f64(
    handle: *mut LiblrhsmmRsObservationStreamF64,
    offset: usize,
    input: *const f64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = storage_range(input, count) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the caller contract.
    let stream = unsafe { &mut (*handle).stream };
    let indices = match interval(offset, count, stream.values.len()) {
        Ok(value) => value,
        Err(status) => return status,
    };
    if count != 0 {
        unsafe {
            stream.values[indices].copy_from_slice(std::slice::from_raw_parts(input, count));
        }
    }
    0
}
/// Copy a checked complete frame.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Buffers cannot alias owner storage. Mutation/release requires exclusive access;
/// reads cannot overlap mutation/release. Successful constructors receive slots
/// holding no previous live owner. Release consumes unique ownership and clears
/// its initialized slot. Null owned values are accepted. No internal pointer escapes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_stream_copy_frame_f64(
    handle: *const LiblrhsmmRsObservationStreamF64,
    frame: usize,
    output: *mut f64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = storage_range(output, count) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the caller contract.
    let stream = unsafe { &(*handle).stream };
    if count != stream.dimensions || frame >= stream.values.len() / stream.dimensions {
        return 2;
    }
    let Some(offset) = frame.checked_mul(stream.dimensions) else {
        return 2;
    };
    let indices = match interval(offset, count, stream.values.len()) {
        Ok(value) => value,
        Err(status) => return status,
    };
    if count != 0 {
        unsafe {
            std::ptr::copy_nonoverlapping(stream.values[indices].as_ptr(), output, count);
        }
    }
    0
}
/// Replace a checked complete frame.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Buffers cannot alias owner storage. Mutation/release requires exclusive access;
/// reads cannot overlap mutation/release. Successful constructors receive slots
/// holding no previous live owner. Release consumes unique ownership and clears
/// its initialized slot. Null owned values are accepted. No internal pointer escapes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_stream_set_frame_f64(
    handle: *mut LiblrhsmmRsObservationStreamF64,
    frame: usize,
    input: *const f64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = storage_range(input, count) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the caller contract.
    let stream = unsafe { &mut (*handle).stream };
    if count != stream.dimensions || frame >= stream.values.len() / stream.dimensions {
        return 2;
    }
    let Some(offset) = frame.checked_mul(stream.dimensions) else {
        return 2;
    };
    let indices = match interval(offset, count, stream.values.len()) {
        Ok(value) => value,
        Err(status) => return status,
    };
    if count != 0 {
        unsafe {
            stream.values[indices].copy_from_slice(std::slice::from_raw_parts(input, count));
        }
    }
    0
}
/// Clone all dimensions and feature bits into independent editable storage.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Buffers cannot alias owner storage. Mutation/release requires exclusive access;
/// reads cannot overlap mutation/release. Successful constructors receive slots
/// holding no previous live owner. Release consumes unique ownership and clears
/// its initialized slot. Null owned values are accepted. No internal pointer escapes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_stream_clone_f64(
    handle: *const LiblrhsmmRsObservationStreamF64,
    output: *mut *mut LiblrhsmmRsObservationStreamF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the caller contract.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsObservationStreamF64 {
                stream: (*handle).stream.clone(),
            })))
        })
    }
}
/// Release feature storage and clear unique ownership; null owned values are a no-op.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Buffers cannot alias owner storage. Mutation/release requires exclusive access;
/// reads cannot overlap mutation/release. Successful constructors receive slots
/// holding no previous live owner. Release consumes unique ownership and clears
/// its initialized slot. Null owned values are accepted. No internal pointer escapes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_stream_release_f64(
    handle: *mut *mut LiblrhsmmRsObservationStreamF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the caller contract.
    let value = unsafe { handle.read() };
    if !value.is_null() {
        if let Err(status) = range(value, 1) {
            return status;
        }
        unsafe {
            drop(Box::from_raw(value));
            handle.write(std::ptr::null_mut());
        }
    }
    0
}
