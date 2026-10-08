//! Optional observation codecs using the unchanged binary32 feature schema.
use super::bytes::{LiblrhsmmRsBytes, byte_input, byte_range};
use super::io::{LiblrhsmmRsReadStream, LiblrhsmmRsWriteStream, Reader, Writer};
use super::observation::{LiblrhsmmRsObservationF32, LiblrhsmmRsObservationF64};
use super::{LiblrhsmmRsDecodeLimits, boundary, range, scalar_result};
use crate::{ModelError, data::ObservationData, serial::DecodeLimits};
/// Read one native observation from borrowed bytes. Trailing data, truncation and resource limits retain output.
/// Native codec/I/O errors return status three.
/// # Safety
/// Nonnull inputs are aligned initialized valid storage. Byte input is readable for count bytes.
/// Handles are live owners from this precision's interface; reads cannot overlap mutation/release.
/// Outputs are independent exclusively writable storage; successful constructors use unowned slots.
/// Callbacks/context remain valid only for this synchronous call and must not unwind, retain
/// borrowed buffers or access source owner/output storage. The callback context may be modified.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_read_bytes_f32(
    input: *const u8,
    count: usize,
    limits: *const LiblrhsmmRsDecodeLimits,
    output: *mut *mut LiblrhsmmRsObservationF32,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and synchronous callback contracts above.
    if let Err(status) = byte_range(input, count) {
        return status;
    }
    if let Err(status) = range(limits, 1) {
        return status;
    }
    // SAFETY: Independent initialized limits and borrowed input.
    let limits = unsafe { limits.read() };
    unsafe {
        scalar_result(output, || {
            let observation = ObservationData::<f32>::read_with_limits(
                byte_input(input, count),
                DecodeLimits {
                    max_array_entries: limits.max_array_entries,
                },
            )
            .map_err(|_| ModelError("observation decoding failed"))?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsObservationF32 {
                observation,
            })))
        })
    }
}

/// Decode through synchronous borrowed callbacks. Short reads and interruption retries follow native Read; trailing-data checks require EOF.
/// Native codec/I/O errors return status three.
/// # Safety
/// Nonnull inputs are aligned initialized valid storage. Byte input is readable for count bytes.
/// Handles are live owners from this precision's interface; reads cannot overlap mutation/release.
/// Outputs are independent exclusively writable storage; successful constructors use unowned slots.
/// Callbacks/context remain valid only for this synchronous call and must not unwind, retain
/// borrowed buffers or access source owner/output storage. The callback context may be modified.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_read_stream_f32(
    stream: *const LiblrhsmmRsReadStream,
    limits: *const LiblrhsmmRsDecodeLimits,
    output: *mut *mut LiblrhsmmRsObservationF32,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and synchronous callback contracts above.
    if let Err(status) = range(stream, 1) {
        return status;
    }
    if let Err(status) = range(limits, 1) {
        return status;
    }
    // SAFETY: Independent initialized borrowed stream/limit metadata.
    let stream = unsafe { stream.read() };
    let limits = unsafe { limits.read() };
    if stream.read.is_none() {
        return 3;
    }
    unsafe {
        scalar_result(output, || {
            let observation = ObservationData::<f32>::read_with_limits(
                Reader(stream),
                DecodeLimits {
                    max_array_entries: limits.max_array_entries,
                },
            )
            .map_err(|_| ModelError("stream observation decoding failed"))?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsObservationF32 {
                observation,
            })))
        })
    }
}

/// Read one object through synchronous callbacks, leaving following bytes unread. No EOF check.
/// Native codec/I/O errors return status three.
/// # Safety
/// Nonnull inputs are aligned initialized valid storage. Byte input is readable for count bytes.
/// Handles are live owners from this precision's interface; reads cannot overlap mutation/release.
/// Outputs are independent exclusively writable storage; successful constructors use unowned slots.
/// Callbacks/context remain valid only for this synchronous call and must not unwind, retain
/// borrowed buffers or access source owner/output storage. The callback context may be modified.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_read_stream_prefix_f32(
    stream: *const LiblrhsmmRsReadStream,
    limits: *const LiblrhsmmRsDecodeLimits,
    output: *mut *mut LiblrhsmmRsObservationF32,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and synchronous callback contracts above.
    if let Err(status) = range(stream, 1) {
        return status;
    }
    if let Err(status) = range(limits, 1) {
        return status;
    }
    // SAFETY: Independent initialized borrowed stream/limit metadata.
    let stream = unsafe { stream.read() };
    let limits = unsafe { limits.read() };
    if stream.read.is_none() {
        return 3;
    }
    unsafe {
        scalar_result(output, || {
            let observation = ObservationData::<f32>::read_prefix_with_limits(
                Reader(stream),
                DecodeLimits {
                    max_array_entries: limits.max_array_entries,
                },
            )
            .map_err(|_| ModelError("stream observation decoding failed"))?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsObservationF32 {
                observation,
            })))
        })
    }
}

/// Write the unchanged native binary32 wire schema into a new byte owner in both storage precisions.
/// Native codec/I/O errors return status three.
/// # Safety
/// Nonnull inputs are aligned initialized valid storage. Byte input is readable for count bytes.
/// Handles are live owners from this precision's interface; reads cannot overlap mutation/release.
/// Outputs are independent exclusively writable storage; successful constructors use unowned slots.
/// Callbacks/context remain valid only for this synchronous call and must not unwind, retain
/// borrowed buffers or access source owner/output storage. The callback context may be modified.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_write_bytes_f32(
    handle: *const LiblrhsmmRsObservationF32,
    output: *mut *mut LiblrhsmmRsBytes,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and synchronous callback contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let mut bytes = Vec::new();
            (*handle)
                .observation
                .write_to(&mut bytes)
                .map_err(|_| ModelError("observation encoding failed"))?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsBytes { bytes })))
        })
    }
}

/// Write through borrowed callbacks, supporting short writes and native interruption retry. No implicit flush/close; I/O failure may leave an emitted prefix.
/// Native codec/I/O errors return status three.
/// # Safety
/// Nonnull inputs are aligned initialized valid storage. Byte input is readable for count bytes.
/// Handles are live owners from this precision's interface; reads cannot overlap mutation/release.
/// Outputs are independent exclusively writable storage; successful constructors use unowned slots.
/// Callbacks/context remain valid only for this synchronous call and must not unwind, retain
/// borrowed buffers or access source owner/output storage. The callback context may be modified.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_write_stream_f32(
    handle: *const LiblrhsmmRsObservationF32,
    stream: *const LiblrhsmmRsWriteStream,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and synchronous callback contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(stream, 1) {
        return status;
    }
    // SAFETY: Initialized independent synchronous stream metadata.
    let stream = unsafe { stream.read() };
    if stream.write.is_none() {
        return 3;
    }
    match boundary(|| {
        unsafe { (*handle).observation.write_to(Writer(stream)) }
            .map_err(|_| ModelError("stream observation encoding failed"))
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}

/// Read one native observation from borrowed bytes. Trailing data, truncation and resource limits retain output.
/// Native codec/I/O errors return status three.
/// # Safety
/// Nonnull inputs are aligned initialized valid storage. Byte input is readable for count bytes.
/// Handles are live owners from this precision's interface; reads cannot overlap mutation/release.
/// Outputs are independent exclusively writable storage; successful constructors use unowned slots.
/// Callbacks/context remain valid only for this synchronous call and must not unwind, retain
/// borrowed buffers or access source owner/output storage. The callback context may be modified.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_read_bytes_f64(
    input: *const u8,
    count: usize,
    limits: *const LiblrhsmmRsDecodeLimits,
    output: *mut *mut LiblrhsmmRsObservationF64,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and synchronous callback contracts above.
    if let Err(status) = byte_range(input, count) {
        return status;
    }
    if let Err(status) = range(limits, 1) {
        return status;
    }
    // SAFETY: Independent initialized limits and borrowed input.
    let limits = unsafe { limits.read() };
    unsafe {
        scalar_result(output, || {
            let observation = ObservationData::<f64>::read_with_limits(
                byte_input(input, count),
                DecodeLimits {
                    max_array_entries: limits.max_array_entries,
                },
            )
            .map_err(|_| ModelError("observation decoding failed"))?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsObservationF64 {
                observation,
            })))
        })
    }
}

/// Decode through synchronous borrowed callbacks. Short reads and interruption retries follow native Read; trailing-data checks require EOF.
/// Native codec/I/O errors return status three.
/// # Safety
/// Nonnull inputs are aligned initialized valid storage. Byte input is readable for count bytes.
/// Handles are live owners from this precision's interface; reads cannot overlap mutation/release.
/// Outputs are independent exclusively writable storage; successful constructors use unowned slots.
/// Callbacks/context remain valid only for this synchronous call and must not unwind, retain
/// borrowed buffers or access source owner/output storage. The callback context may be modified.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_read_stream_f64(
    stream: *const LiblrhsmmRsReadStream,
    limits: *const LiblrhsmmRsDecodeLimits,
    output: *mut *mut LiblrhsmmRsObservationF64,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and synchronous callback contracts above.
    if let Err(status) = range(stream, 1) {
        return status;
    }
    if let Err(status) = range(limits, 1) {
        return status;
    }
    // SAFETY: Independent initialized borrowed stream/limit metadata.
    let stream = unsafe { stream.read() };
    let limits = unsafe { limits.read() };
    if stream.read.is_none() {
        return 3;
    }
    unsafe {
        scalar_result(output, || {
            let observation = ObservationData::<f64>::read_with_limits(
                Reader(stream),
                DecodeLimits {
                    max_array_entries: limits.max_array_entries,
                },
            )
            .map_err(|_| ModelError("stream observation decoding failed"))?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsObservationF64 {
                observation,
            })))
        })
    }
}

/// Read one object through synchronous callbacks, leaving following bytes unread. No EOF check.
/// Native codec/I/O errors return status three.
/// # Safety
/// Nonnull inputs are aligned initialized valid storage. Byte input is readable for count bytes.
/// Handles are live owners from this precision's interface; reads cannot overlap mutation/release.
/// Outputs are independent exclusively writable storage; successful constructors use unowned slots.
/// Callbacks/context remain valid only for this synchronous call and must not unwind, retain
/// borrowed buffers or access source owner/output storage. The callback context may be modified.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_read_stream_prefix_f64(
    stream: *const LiblrhsmmRsReadStream,
    limits: *const LiblrhsmmRsDecodeLimits,
    output: *mut *mut LiblrhsmmRsObservationF64,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and synchronous callback contracts above.
    if let Err(status) = range(stream, 1) {
        return status;
    }
    if let Err(status) = range(limits, 1) {
        return status;
    }
    // SAFETY: Independent initialized borrowed stream/limit metadata.
    let stream = unsafe { stream.read() };
    let limits = unsafe { limits.read() };
    if stream.read.is_none() {
        return 3;
    }
    unsafe {
        scalar_result(output, || {
            let observation = ObservationData::<f64>::read_prefix_with_limits(
                Reader(stream),
                DecodeLimits {
                    max_array_entries: limits.max_array_entries,
                },
            )
            .map_err(|_| ModelError("stream observation decoding failed"))?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsObservationF64 {
                observation,
            })))
        })
    }
}

/// Write the unchanged native binary32 wire schema into a new byte owner in both storage precisions.
/// Native codec/I/O errors return status three.
/// # Safety
/// Nonnull inputs are aligned initialized valid storage. Byte input is readable for count bytes.
/// Handles are live owners from this precision's interface; reads cannot overlap mutation/release.
/// Outputs are independent exclusively writable storage; successful constructors use unowned slots.
/// Callbacks/context remain valid only for this synchronous call and must not unwind, retain
/// borrowed buffers or access source owner/output storage. The callback context may be modified.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_write_bytes_f64(
    handle: *const LiblrhsmmRsObservationF64,
    output: *mut *mut LiblrhsmmRsBytes,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and synchronous callback contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let mut bytes = Vec::new();
            (*handle)
                .observation
                .write_to(&mut bytes)
                .map_err(|_| ModelError("observation encoding failed"))?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsBytes { bytes })))
        })
    }
}

/// Write through borrowed callbacks, supporting short writes and native interruption retry. No implicit flush/close; I/O failure may leave an emitted prefix.
/// Native codec/I/O errors return status three.
/// # Safety
/// Nonnull inputs are aligned initialized valid storage. Byte input is readable for count bytes.
/// Handles are live owners from this precision's interface; reads cannot overlap mutation/release.
/// Outputs are independent exclusively writable storage; successful constructors use unowned slots.
/// Callbacks/context remain valid only for this synchronous call and must not unwind, retain
/// borrowed buffers or access source owner/output storage. The callback context may be modified.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_observation_write_stream_f64(
    handle: *const LiblrhsmmRsObservationF64,
    stream: *const LiblrhsmmRsWriteStream,
) -> u32 {
    // SAFETY: Caller fulfills the independent storage and synchronous callback contracts above.
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(stream, 1) {
        return status;
    }
    // SAFETY: Initialized independent synchronous stream metadata.
    let stream = unsafe { stream.read() };
    if stream.write.is_none() {
        return 3;
    }
    match boundary(|| {
        unsafe { (*handle).observation.write_to(Writer(stream)) }
            .map_err(|_| ModelError("stream observation encoding failed"))
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}
