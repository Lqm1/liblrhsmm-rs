//! Independent prepared stream snapshots and checked child/cache access.
use super::*;
use crate::preparation::OwnedPreparedStreamParameters;
/// Independently owned immutable parameters and matching caches.
#[derive(Debug, Clone)]
pub struct LiblrhsmmRsPreparedStreamF32 {
    pub(super) prepared: OwnedPreparedStreamParameters<f32>,
}
/// Copy the complete source and prepare an immutable independent snapshot.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Reads cannot overlap mutation or release. Constructor outputs hold no live owner.
/// Child results own independent copies. Release consumes unique ownership through
/// an initialized writable slot and clears it; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_prepare_f32(
    handle: *const LiblrhsmmRsStreamF32,
    output: *mut *mut LiblrhsmmRsPreparedStreamF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsPreparedStreamF32 {
                prepared: (*handle).parameters.clone().prepare_owned()?,
            })))
        })
    }
}
/// Copy every parameter into an independent editable source owner.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Reads cannot overlap mutation or release. Constructor outputs hold no live owner.
/// Child results own independent copies. Release consumes unique ownership through
/// an initialized writable slot and clears it; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_stream_parameters_f32(
    handle: *const LiblrhsmmRsPreparedStreamF32,
    output: *mut *mut LiblrhsmmRsStreamF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStreamF32 {
                parameters: (*handle).prepared.parameters().clone(),
            })))
        })
    }
}
/// Read complete parameter counts and the stream weight where applicable.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Reads cannot overlap mutation or release. Constructor outputs hold no live owner.
/// Child results own independent copies. Release consumes unique ownership through
/// an initialized writable slot and clears it; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_stream_layout_f32(
    handle: *const LiblrhsmmRsPreparedStreamF32,
    output: *mut LiblrhsmmRsStreamLayoutF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
    unsafe {
        scalar_result(output, || {
            let parameters = (*handle).prepared.parameters();
            Ok(LiblrhsmmRsStreamLayoutF32 {
                weight: parameters.weight,
                emissions: parameters.mixtures.len(),
            })
        })
    }
}
/// Copy one prepared emission and its existing cache without recomputation.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Reads cannot overlap mutation or release. Constructor outputs hold no live owner.
/// Child results own independent copies. Release consumes unique ownership through
/// an initialized writable slot and clears it; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_stream_get_emission_f32(
    handle: *const LiblrhsmmRsPreparedStreamF32,
    state: usize,
    output: *mut *mut LiblrhsmmRsPreparedGaussianF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
    let Some(value) = (unsafe { (*handle).prepared.emission(state) }) else {
        return 2;
    };
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsPreparedGaussianF32 {
                parameters: value.to_owned(),
            })))
        })
    }
}
/// Clone every parameter and prepared cache independently.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Reads cannot overlap mutation or release. Constructor outputs hold no live owner.
/// Child results own independent copies. Release consumes unique ownership through
/// an initialized writable slot and clears it; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_stream_clone_f32(
    handle: *const LiblrhsmmRsPreparedStreamF32,
    output: *mut *mut LiblrhsmmRsPreparedStreamF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
    unsafe { scalar_result(output, || Ok(Box::into_raw(Box::new((*handle).clone())))) }
}
/// Release unique ownership and clear its slot; null owned values are accepted.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Reads cannot overlap mutation or release. Constructor outputs hold no live owner.
/// Child results own independent copies. Release consumes unique ownership through
/// an initialized writable slot and clears it; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_stream_release_f32(
    handle: *mut *mut LiblrhsmmRsPreparedStreamF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
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
/// Independently owned immutable parameters and matching caches.
#[derive(Debug, Clone)]
pub struct LiblrhsmmRsPreparedStreamF64 {
    pub(super) prepared: OwnedPreparedStreamParameters<f64>,
}
/// Copy the complete source and prepare an immutable independent snapshot.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Reads cannot overlap mutation or release. Constructor outputs hold no live owner.
/// Child results own independent copies. Release consumes unique ownership through
/// an initialized writable slot and clears it; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_prepare_f64(
    handle: *const LiblrhsmmRsStreamF64,
    output: *mut *mut LiblrhsmmRsPreparedStreamF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsPreparedStreamF64 {
                prepared: (*handle).parameters.clone().prepare_owned()?,
            })))
        })
    }
}
/// Copy every parameter into an independent editable source owner.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Reads cannot overlap mutation or release. Constructor outputs hold no live owner.
/// Child results own independent copies. Release consumes unique ownership through
/// an initialized writable slot and clears it; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_stream_parameters_f64(
    handle: *const LiblrhsmmRsPreparedStreamF64,
    output: *mut *mut LiblrhsmmRsStreamF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStreamF64 {
                parameters: (*handle).prepared.parameters().clone(),
            })))
        })
    }
}
/// Read complete parameter counts and the stream weight where applicable.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Reads cannot overlap mutation or release. Constructor outputs hold no live owner.
/// Child results own independent copies. Release consumes unique ownership through
/// an initialized writable slot and clears it; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_stream_layout_f64(
    handle: *const LiblrhsmmRsPreparedStreamF64,
    output: *mut LiblrhsmmRsStreamLayoutF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
    unsafe {
        scalar_result(output, || {
            let parameters = (*handle).prepared.parameters();
            Ok(LiblrhsmmRsStreamLayoutF64 {
                weight: parameters.weight,
                emissions: parameters.mixtures.len(),
            })
        })
    }
}
/// Copy one prepared emission and its existing cache without recomputation.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Reads cannot overlap mutation or release. Constructor outputs hold no live owner.
/// Child results own independent copies. Release consumes unique ownership through
/// an initialized writable slot and clears it; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_stream_get_emission_f64(
    handle: *const LiblrhsmmRsPreparedStreamF64,
    state: usize,
    output: *mut *mut LiblrhsmmRsPreparedGaussianF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
    let Some(value) = (unsafe { (*handle).prepared.emission(state) }) else {
        return 2;
    };
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsPreparedGaussianF64 {
                parameters: value.to_owned(),
            })))
        })
    }
}
/// Clone every parameter and prepared cache independently.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Reads cannot overlap mutation or release. Constructor outputs hold no live owner.
/// Child results own independent copies. Release consumes unique ownership through
/// an initialized writable slot and clears it; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_stream_clone_f64(
    handle: *const LiblrhsmmRsPreparedStreamF64,
    output: *mut *mut LiblrhsmmRsPreparedStreamF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
    unsafe { scalar_result(output, || Ok(Box::into_raw(Box::new((*handle).clone())))) }
}
/// Release unique ownership and clear its slot; null owned values are accepted.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Reads cannot overlap mutation or release. Constructor outputs hold no live owner.
/// Child results own independent copies. Release consumes unique ownership through
/// an initialized writable slot and clears it; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_stream_release_f64(
    handle: *mut *mut LiblrhsmmRsPreparedStreamF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
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
