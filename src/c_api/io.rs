//! Synchronous borrowed I/O adapters; foreign callbacks and contexts are never retained.
use super::{boundary, range};
use crate::ModelError;
#[cfg(feature = "serialization")]
use std::io::Read;
use std::{
    ffi::c_void,
    io::{self, Write},
};

pub const LIBLRHSMM_RS_IO_SUCCESS: u32 = 0;
pub const LIBLRHSMM_RS_IO_INTERRUPTED: u32 = 1;
pub const LIBLRHSMM_RS_IO_ERROR: u32 = 2;

/// Callback statuses: zero success, one interrupted, any other value I/O error.
/// Read writes at most capacity bytes and reports count <= capacity; count zero means EOF.
/// Callback/context remain valid for the entire synchronous call, must not unwind or modify
/// owner/result storage, and may only mutate independent context. No callback is retained.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LiblrhsmmRsReadStream {
    pub context: *mut c_void,
    pub read: Option<unsafe extern "C" fn(*mut c_void, *mut u8, usize, *mut usize) -> u32>,
}

/// Borrowed writer with partial transfers. Statuses match ReadStream; count <= capacity.
/// Native codecs do not flush, close or retain this context. Flush is explicitly requested
/// through write_stream_flush and its callback is optional for ordinary encoding.
/// Callbacks must not unwind, retain borrowed buffers or modify owner/result storage.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LiblrhsmmRsWriteStream {
    pub context: *mut c_void,
    pub write: Option<unsafe extern "C" fn(*mut c_void, *const u8, usize, *mut usize) -> u32>,
    pub flush: Option<unsafe extern "C" fn(*mut c_void) -> u32>,
}

fn callback_result(status: u32) -> io::Result<()> {
    match status {
        LIBLRHSMM_RS_IO_SUCCESS => Ok(()),
        LIBLRHSMM_RS_IO_INTERRUPTED => Err(io::ErrorKind::Interrupted.into()),
        _ => Err(io::Error::other("I/O callback failed")),
    }
}

#[cfg(feature = "serialization")]
pub(super) struct Reader(pub LiblrhsmmRsReadStream);
#[cfg(feature = "serialization")]
impl Read for Reader {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }
        let callback = self
            .0
            .read
            .ok_or_else(|| io::Error::other("missing read callback"))?;
        let mut count = 0;
        // SAFETY: Caller guarantees the synchronous callback/context contract.
        let status = unsafe {
            callback(
                self.0.context,
                buffer.as_mut_ptr(),
                buffer.len(),
                &mut count,
            )
        };
        callback_result(status)?;
        if count > buffer.len() {
            return Err(io::Error::other("read count exceeds buffer"));
        }
        Ok(count)
    }
}

pub(super) struct Writer(pub LiblrhsmmRsWriteStream);
impl Write for Writer {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.is_empty() {
            return Ok(0);
        }
        let callback = self
            .0
            .write
            .ok_or_else(|| io::Error::other("missing write callback"))?;
        let mut count = 0;
        // SAFETY: Caller guarantees independent synchronous callback/context validity.
        let status = unsafe { callback(self.0.context, bytes.as_ptr(), bytes.len(), &mut count) };
        callback_result(status)?;
        if count > bytes.len() {
            return Err(io::Error::other("write count exceeds input"));
        }
        Ok(count)
    }
    fn flush(&mut self) -> io::Result<()> {
        let callback = self
            .0
            .flush
            .ok_or_else(|| io::Error::new(io::ErrorKind::Unsupported, "missing flush callback"))?;
        // SAFETY: Caller guarantees synchronous callback/context validity.
        callback_result(unsafe { callback(self.0.context) })
    }
}

/// Explicitly flush borrowed context. No encoding implicitly flushes.
/// Interrupted flushes return status three; callers choose whether to retry.
/// # Safety
/// Stream is an initialized aligned descriptor with valid non-unwinding callback/context
/// for this synchronous call. Context is independent of all owner/result storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_write_stream_flush(
    stream: *const LiblrhsmmRsWriteStream,
) -> u32 {
    if let Err(status) = range(stream, 1) {
        return status;
    }
    // SAFETY: Caller provides independent initialized stream metadata.
    let stream = unsafe { stream.read() };
    match boundary(|| {
        Writer(stream)
            .flush()
            .map_err(|_| ModelError("stream flush failed"))
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}
