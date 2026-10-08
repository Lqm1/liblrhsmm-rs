use super::super::LiblrhsmmRsSegmentationSetF64;
use super::*;

/// Decode a complete legacy set with cumulative limits; reject trailing bytes.
/// Native codec and I/O errors return status three and retain initialized output.
/// # Safety
/// Borrowed bytes and metadata are initialized readable valid storage. Owners
/// have the declared type/precision and remain live without concurrent mutation
/// or release. Outputs are independent aligned writable storage; successful
/// constructors require slots holding no live owner. Callback contexts and
/// buffers are borrowed only for this synchronous call. Callbacks must not
/// unwind, retain buffers or access participating owners/output slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_read_bytes_f64(
    input: *const u8,
    count: usize,
    limits: *const LiblrhsmmRsDecodeLimits,
    output: *mut *mut LiblrhsmmRsSegmentationSetF64,
) -> u32 {
    if let Err(status) = byte_range(input, count) {
        return status;
    }
    if let Err(status) = range(limits, 1) {
        return status;
    }
    // SAFETY: Independent initialized limits and readable borrowed bytes.
    let limits = unsafe { limits.read() };
    unsafe {
        scalar_result(output, || {
            let samples = serial::read_segmentation_set_with_limits::<f64>(
                byte_input(input, count),
                DecodeLimits {
                    max_array_entries: limits.max_array_entries,
                },
            )
            .map_err(|_| ModelError("sample set decoding failed"))?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsSegmentationSetF64 {
                samples,
            })))
        })
    }
}

/// Decode a complete set through callbacks; trailing-data checks require EOF.
/// Short reads and interruption retries follow native Read. Errors retain output.
/// # Safety
/// Borrowed bytes and metadata are initialized readable valid storage. Owners
/// have the declared type/precision and remain live without concurrent mutation
/// or release. Outputs are independent aligned writable storage; successful
/// constructors require slots holding no live owner. Callback contexts and
/// buffers are borrowed only for this synchronous call. Callbacks must not
/// unwind, retain buffers or access participating owners/output slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_read_stream_f64(
    stream: *const LiblrhsmmRsReadStream,
    limits: *const LiblrhsmmRsDecodeLimits,
    output: *mut *mut LiblrhsmmRsSegmentationSetF64,
) -> u32 {
    if let Err(status) = range(stream, 1) {
        return status;
    }
    if let Err(status) = range(limits, 1) {
        return status;
    }
    // SAFETY: Independent initialized callback/limit descriptors borrowed for this call.
    let stream = unsafe { stream.read() };
    let limits = unsafe { limits.read() };
    if stream.read.is_none() {
        return 3;
    }
    // SAFETY: Independent output and synchronous borrowed callbacks.
    unsafe {
        scalar_result(output, || {
            let samples = serial::read_segmentation_set_with_limits::<f64>(
                Reader(stream),
                DecodeLimits {
                    max_array_entries: limits.max_array_entries,
                },
            )
            .map_err(|_| ModelError("stream sample set decoding failed"))?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsSegmentationSetF64 {
                samples,
            })))
        })
    }
}

/// Decode one set through callbacks, leaving following bytes unread without an EOF check.
/// Short reads and interruption retries follow native Read. Errors retain output.
/// # Safety
/// Borrowed bytes and metadata are initialized readable valid storage. Owners
/// have the declared type/precision and remain live without concurrent mutation
/// or release. Outputs are independent aligned writable storage; successful
/// constructors require slots holding no live owner. Callback contexts and
/// buffers are borrowed only for this synchronous call. Callbacks must not
/// unwind, retain buffers or access participating owners/output slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_read_stream_prefix_f64(
    stream: *const LiblrhsmmRsReadStream,
    limits: *const LiblrhsmmRsDecodeLimits,
    output: *mut *mut LiblrhsmmRsSegmentationSetF64,
) -> u32 {
    if let Err(status) = range(stream, 1) {
        return status;
    }
    if let Err(status) = range(limits, 1) {
        return status;
    }
    // SAFETY: Independent initialized callback/limit descriptors borrowed for this call.
    let stream = unsafe { stream.read() };
    let limits = unsafe { limits.read() };
    if stream.read.is_none() {
        return 3;
    }
    // SAFETY: Independent output and synchronous borrowed callbacks.
    unsafe {
        scalar_result(output, || {
            let samples = serial::read_segmentation_set_prefix_with_limits::<f64>(
                Reader(stream),
                DecodeLimits {
                    max_array_entries: limits.max_array_entries,
                },
            )
            .map_err(|_| ModelError("stream sample set decoding failed"))?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsSegmentationSetF64 {
                samples,
            })))
        })
    }
}

/// Encode all samples into independent bytes using the legacy binary32 wire in both precisions.
/// # Safety
/// Borrowed bytes and metadata are initialized readable valid storage. Owners
/// have the declared type/precision and remain live without concurrent mutation
/// or release. Outputs are independent aligned writable storage; successful
/// constructors require slots holding no live owner. Callback contexts and
/// buffers are borrowed only for this synchronous call. Callbacks must not
/// unwind, retain buffers or access participating owners/output slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_write_bytes_f64(
    handle: *const LiblrhsmmRsSegmentationSetF64,
    output: *mut *mut LiblrhsmmRsBytes,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable owner and independent writable output.
    unsafe {
        scalar_result(output, || {
            let mut bytes = Vec::new();
            serial::write_segmentation_set_with_precision(&(*handle).samples, &mut bytes)
                .map_err(|_| ModelError("sample set encoding failed"))?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsBytes { bytes })))
        })
    }
}

/// Encode through borrowed callbacks with short-write and interruption handling.
/// No implicit flush/close; failures may leave an emitted prefix in the callback context.
/// # Safety
/// Borrowed bytes and metadata are initialized readable valid storage. Owners
/// have the declared type/precision and remain live without concurrent mutation
/// or release. Outputs are independent aligned writable storage; successful
/// constructors require slots holding no live owner. Callback contexts and
/// buffers are borrowed only for this synchronous call. Callbacks must not
/// unwind, retain buffers or access participating owners/output slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_write_stream_f64(
    handle: *const LiblrhsmmRsSegmentationSetF64,
    stream: *const LiblrhsmmRsWriteStream,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(stream, 1) {
        return status;
    }
    // SAFETY: Initialized callback metadata is borrowed only synchronously.
    let stream = unsafe { stream.read() };
    if stream.write.is_none() {
        return 3;
    }
    match boundary(|| {
        // SAFETY: Caller keeps the complete set readable throughout callback execution.
        unsafe { serial::write_segmentation_set_with_precision(&(*handle).samples, Writer(stream)) }
            .map_err(|_| ModelError("stream sample set encoding failed"))
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}
