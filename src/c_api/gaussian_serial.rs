//! Optional native Gaussian codecs and explicit historical/current encodings.
use super::bytes::{LiblrhsmmRsBytes, byte_input, byte_range};
use super::gaussian::{LiblrhsmmRsGaussianF32, LiblrhsmmRsGaussianF64};
use super::io::{LiblrhsmmRsReadStream, LiblrhsmmRsWriteStream, Reader, Writer};
use super::{LiblrhsmmRsDecodeLimits, boundary, range, scalar_result};
use crate::{
    ModelError,
    model::GaussianMixtureParameters,
    serial::{DecodeLimits, ModelEncoding},
};

/// Native schema with per-parameter variance floors.
pub const LIBLRHSMM_RS_ENCODING_WITH_VARIANCE_FLOORS: u32 = 0;
/// Historical native schema without per-parameter variance floors.
/// Nonzero floors are rejected rather than silently discarded.
pub const LIBLRHSMM_RS_ENCODING_WITHOUT_VARIANCE_FLOORS: u32 = 1;
pub(super) fn model_encoding(value: u32) -> Result<ModelEncoding, ModelError> {
    match value {
        LIBLRHSMM_RS_ENCODING_WITH_VARIANCE_FLOORS => Ok(ModelEncoding::WithVarianceFloors),
        LIBLRHSMM_RS_ENCODING_WITHOUT_VARIANCE_FLOORS => Ok(ModelEncoding::WithoutVarianceFloors),
        _ => Err(ModelError("unknown model encoding")),
    }
}

/// Read one native standalone gaussian from borrowed bytes. Trailing data, truncation and resource limits retain output.
/// Native codec/I/O errors return status three.
/// # Safety
/// Nonnull inputs are aligned initialized valid storage. Byte input is readable for count bytes.
/// Handles are live owners from this precision's interface; reads cannot overlap mutation/release.
/// Outputs are independent exclusively writable storage; successful constructors use unowned slots.
/// Callbacks/context remain valid only for this synchronous call and must not unwind, retain
/// borrowed buffers or access source owner/output storage. The callback context may be modified.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_read_bytes_f32(
    input: *const u8,
    count: usize,
    limits: *const LiblrhsmmRsDecodeLimits,
    output: *mut *mut LiblrhsmmRsGaussianF32,
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
            let parameters = GaussianMixtureParameters::<f32>::read_with_limits(
                byte_input(input, count),
                DecodeLimits {
                    max_array_entries: limits.max_array_entries,
                },
            )
            .map_err(|_| ModelError("gaussian decoding failed"))?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianF32 {
                parameters,
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
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_read_stream_f32(
    stream: *const LiblrhsmmRsReadStream,
    limits: *const LiblrhsmmRsDecodeLimits,
    output: *mut *mut LiblrhsmmRsGaussianF32,
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
            let parameters = GaussianMixtureParameters::<f32>::read_with_limits(
                Reader(stream),
                DecodeLimits {
                    max_array_entries: limits.max_array_entries,
                },
            )
            .map_err(|_| ModelError("gaussian stream decoding failed"))?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianF32 {
                parameters,
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
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_read_stream_prefix_f32(
    stream: *const LiblrhsmmRsReadStream,
    limits: *const LiblrhsmmRsDecodeLimits,
    output: *mut *mut LiblrhsmmRsGaussianF32,
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
            let parameters = GaussianMixtureParameters::<f32>::read_prefix_with_limits(
                Reader(stream),
                DecodeLimits {
                    max_array_entries: limits.max_array_entries,
                },
            )
            .map_err(|_| ModelError("gaussian stream decoding failed"))?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianF32 {
                parameters,
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
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_write_bytes_f32(
    handle: *const LiblrhsmmRsGaussianF32,
    encoding: u32,
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
                .parameters
                .write_with_encoding(&mut bytes, model_encoding(encoding)?)
                .map_err(|_| ModelError("gaussian encoding failed"))?;
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
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_write_stream_f32(
    handle: *const LiblrhsmmRsGaussianF32,
    encoding: u32,
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
        unsafe {
            (*handle)
                .parameters
                .write_with_encoding(Writer(stream), model_encoding(encoding)?)
        }
        .map_err(|_| ModelError("gaussian stream encoding failed"))
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}

/// Read one native standalone gaussian from borrowed bytes. Trailing data, truncation and resource limits retain output.
/// Native codec/I/O errors return status three.
/// # Safety
/// Nonnull inputs are aligned initialized valid storage. Byte input is readable for count bytes.
/// Handles are live owners from this precision's interface; reads cannot overlap mutation/release.
/// Outputs are independent exclusively writable storage; successful constructors use unowned slots.
/// Callbacks/context remain valid only for this synchronous call and must not unwind, retain
/// borrowed buffers or access source owner/output storage. The callback context may be modified.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_read_bytes_f64(
    input: *const u8,
    count: usize,
    limits: *const LiblrhsmmRsDecodeLimits,
    output: *mut *mut LiblrhsmmRsGaussianF64,
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
            let parameters = GaussianMixtureParameters::<f64>::read_with_limits(
                byte_input(input, count),
                DecodeLimits {
                    max_array_entries: limits.max_array_entries,
                },
            )
            .map_err(|_| ModelError("gaussian decoding failed"))?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianF64 {
                parameters,
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
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_read_stream_f64(
    stream: *const LiblrhsmmRsReadStream,
    limits: *const LiblrhsmmRsDecodeLimits,
    output: *mut *mut LiblrhsmmRsGaussianF64,
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
            let parameters = GaussianMixtureParameters::<f64>::read_with_limits(
                Reader(stream),
                DecodeLimits {
                    max_array_entries: limits.max_array_entries,
                },
            )
            .map_err(|_| ModelError("gaussian stream decoding failed"))?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianF64 {
                parameters,
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
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_read_stream_prefix_f64(
    stream: *const LiblrhsmmRsReadStream,
    limits: *const LiblrhsmmRsDecodeLimits,
    output: *mut *mut LiblrhsmmRsGaussianF64,
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
            let parameters = GaussianMixtureParameters::<f64>::read_prefix_with_limits(
                Reader(stream),
                DecodeLimits {
                    max_array_entries: limits.max_array_entries,
                },
            )
            .map_err(|_| ModelError("gaussian stream decoding failed"))?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianF64 {
                parameters,
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
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_write_bytes_f64(
    handle: *const LiblrhsmmRsGaussianF64,
    encoding: u32,
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
                .parameters
                .write_with_encoding(&mut bytes, model_encoding(encoding)?)
                .map_err(|_| ModelError("gaussian encoding failed"))?;
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
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_write_stream_f64(
    handle: *const LiblrhsmmRsGaussianF64,
    encoding: u32,
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
        unsafe {
            (*handle)
                .parameters
                .write_with_encoding(Writer(stream), model_encoding(encoding)?)
        }
        .map_err(|_| ModelError("gaussian stream encoding failed"))
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}
