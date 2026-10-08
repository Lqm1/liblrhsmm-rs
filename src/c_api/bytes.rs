//! Owned byte buffers and checked copies, without borrowed internal pointers.
use super::{range, scalar_result};
use crate::ModelError;

pub struct LiblrhsmmRsBytes {
    pub(super) bytes: Vec<u8>,
}

pub(super) fn byte_range(pointer: *const u8, count: usize) -> Result<(), u32> {
    if count > isize::MAX as usize {
        return Err(2);
    }
    if count != 0 && pointer.is_null() {
        return Err(1);
    }
    Ok(())
}

pub(super) unsafe fn byte_input<'a>(pointer: *const u8, count: usize) -> &'a [u8] {
    if count == 0 {
        &[]
    } else {
        // SAFETY: Boundary caller supplies readable initialized bytes valid for the call.
        unsafe { std::slice::from_raw_parts(pointer, count) }
    }
}

/// Copy caller bytes into a new independent owner. Empty input permits null.
/// # Safety
/// Input is readable initialized storage; output is independent aligned writable storage.
/// Store successful constructor results in slots holding no live owner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_bytes_create(
    input: *const u8,
    count: usize,
    output: *mut *mut LiblrhsmmRsBytes,
) -> u32 {
    if let Err(status) = byte_range(input, count) {
        return status;
    }
    // SAFETY: Caller fulfills the checked independent byte input/output contract.
    unsafe {
        scalar_result(output, || {
            let input = byte_input(input, count);
            let mut bytes = Vec::new();
            bytes
                .try_reserve_exact(count)
                .map_err(|_| ModelError("byte allocation failed"))?;
            bytes.extend_from_slice(input);
            Ok(Box::into_raw(Box::new(LiblrhsmmRsBytes { bytes })))
        })
    }
}

/// Copy bytes into a new independent owner.
/// # Safety
/// Handle is live; output is independent aligned writable owner storage with no live owner.
/// No read may overlap release, and no operation may overlap output mutation.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_bytes_clone(
    handle: *const LiblrhsmmRsBytes,
    output: *mut *mut LiblrhsmmRsBytes,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Caller fulfills the live-owner and independent output contract.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsBytes {
                bytes: (*handle).bytes.clone(),
            })))
        })
    }
}

/// Read the byte length; errors retain output.
/// # Safety
/// Handle is live; output is independent aligned exclusively writable usize storage.
/// Reads may not overlap release.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_bytes_length(
    handle: *const LiblrhsmmRsBytes,
    output: *mut usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Caller fulfills the live-owner and independent output contract.
    unsafe { scalar_result(output, || Ok((*handle).bytes.len())) }
}

/// Copy a checked byte interval. Empty copies permit null output at valid boundaries.
/// Errors retain output. Byte counts use address-space bounds, not legacy frame limits.
/// # Safety
/// Handle is live; output is independent exclusively writable storage for count bytes.
/// Reads may not overlap release; output may not alias any owner storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_bytes_copy(
    handle: *const LiblrhsmmRsBytes,
    offset: usize,
    output: *mut u8,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = byte_range(output.cast_const(), count) {
        return status;
    }
    let Some(end) = offset.checked_add(count) else {
        return 2;
    };
    // SAFETY: Live owner and independent output; count zero never invokes raw copying.
    let Some(bytes) = (unsafe { &(*handle).bytes }).get(offset..end) else {
        return 2;
    };
    if count != 0 {
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), output, count);
        }
    }
    0
}

/// Release and clear initialized unique-owner storage. A null owned pointer is a no-op.
/// # Safety
/// Slot is aligned, initialized and exclusively writable, containing a unique live owner or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_bytes_release(handle: *mut *mut LiblrhsmmRsBytes) -> u32 {
    if let Err(status) = range(handle.cast_const(), 1) {
        return status;
    }
    // SAFETY: Caller retains exclusive initialized ownership storage.
    let pointer = unsafe { handle.read() };
    if pointer.is_null() {
        return 0;
    }
    if let Err(status) = range(pointer.cast_const(), 1) {
        return status;
    }
    unsafe {
        handle.write(std::ptr::null_mut());
        drop(Box::from_raw(pointer));
    }
    0
}
