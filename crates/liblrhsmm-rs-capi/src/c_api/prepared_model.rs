//! Independent prepared model snapshots and checked child/cache access.
use super::*;
use crate::preparation::OwnedPreparedModelParameters;
/// Independently owned immutable parameters and matching caches.
#[derive(Debug, Clone)]
pub struct LiblrhsmmRsPreparedModelF32 {
    pub(super) prepared: OwnedPreparedModelParameters<f32>,
}
/// Copy the complete source and prepare an immutable independent snapshot.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Reads cannot overlap mutation or release. Constructor outputs hold no live owner.
/// Child results own independent copies. Release consumes unique ownership through
/// an initialized writable slot and clears it; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_prepare_f32(
    handle: *const LiblrhsmmRsModelF32,
    options: *const LiblrhsmmRsDurationPreparationF32,
    output: *mut *mut LiblrhsmmRsPreparedModelF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(options, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsPreparedModelF32 {
                prepared: (*handle)
                    .parameters
                    .clone()
                    .prepare_owned(options.read().into())?,
            })))
        })
    }
}
/// Prepare emissions and selected durations; empty selections permit null, duplicates prepare once.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Reads cannot overlap mutation or release. Constructor outputs hold no live owner.
/// Child results own independent copies. Release consumes unique ownership through
/// an initialized writable slot and clears it; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_prepare_selected_durations_f32(
    handle: *const LiblrhsmmRsModelF32,
    options: *const LiblrhsmmRsDurationPreparationF32,
    states: *const usize,
    count: usize,
    output: *mut *mut LiblrhsmmRsPreparedModelF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(options, 1) {
        return status;
    }
    if let Err(status) = range(states, count) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
    unsafe {
        scalar_result(output, || {
            let states = if count == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(states, count)
            };
            Ok(Box::into_raw(Box::new(LiblrhsmmRsPreparedModelF32 {
                prepared: (*handle)
                    .parameters
                    .clone()
                    .prepare_owned_selected_durations(options.read().into(), states)?,
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
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_parameters_f32(
    handle: *const LiblrhsmmRsPreparedModelF32,
    output: *mut *mut LiblrhsmmRsModelF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsModelF32 {
                parameters: (*handle).prepared.model().clone(),
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
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_layout_f32(
    handle: *const LiblrhsmmRsPreparedModelF32,
    output: *mut LiblrhsmmRsModelLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
    unsafe {
        scalar_result(output, || {
            let parameters = (*handle).prepared.model();
            Ok(LiblrhsmmRsModelLayout {
                streams: parameters.streams.len(),
                durations: parameters.durations.len(),
            })
        })
    }
}
/// Read all preparation settings used by this snapshot.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Reads cannot overlap mutation or release. Constructor outputs hold no live owner.
/// Child results own independent copies. Release consumes unique ownership through
/// an initialized writable slot and clears it; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_options_f32(
    handle: *const LiblrhsmmRsPreparedModelF32,
    output: *mut LiblrhsmmRsDurationPreparationF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
    unsafe { scalar_result(output, || Ok((*handle).prepared.options().into())) }
}
/// Read one stream layout.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Reads cannot overlap mutation or release. Constructor outputs hold no live owner.
/// Child results own independent copies. Release consumes unique ownership through
/// an initialized writable slot and clears it; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_stream_layout_f32(
    handle: *const LiblrhsmmRsPreparedModelF32,
    index: usize,
    output: *mut LiblrhsmmRsStreamLayoutF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
    let Some(stream) = (unsafe { (*handle).prepared.model() }).streams.get(index) else {
        return 2;
    };
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsStreamLayoutF32 {
                weight: stream.weight,
                emissions: stream.mixtures.len(),
            })
        })
    }
}
/// Copy every parameter of one stream into an editable independent owner.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Reads cannot overlap mutation or release. Constructor outputs hold no live owner.
/// Child results own independent copies. Release consumes unique ownership through
/// an initialized writable slot and clears it; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_stream_parameters_f32(
    handle: *const LiblrhsmmRsPreparedModelF32,
    index: usize,
    output: *mut *mut LiblrhsmmRsStreamF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
    let Some(stream) = (unsafe { (*handle).prepared.model() }).streams.get(index) else {
        return 2;
    };
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStreamF32 {
                parameters: stream.clone(),
            })))
        })
    }
}
/// Report whether an in-range duration was prepared; unselected durations return zero.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Reads cannot overlap mutation or release. Constructor outputs hold no live owner.
/// Child results own independent copies. Release consumes unique ownership through
/// an initialized writable slot and clears it; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_has_duration_f32(
    handle: *const LiblrhsmmRsPreparedModelF32,
    index: usize,
    output: *mut u32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
    if index >= unsafe { (*handle).prepared.model().durations.len() } {
        return 2;
    }
    unsafe {
        scalar_result(output, || {
            Ok(u32::from((*handle).prepared.duration(index).is_some()))
        })
    }
}
/// Copy a prepared duration and its existing cache; unselected in-range states return status three.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Reads cannot overlap mutation or release. Constructor outputs hold no live owner.
/// Child results own independent copies. Release consumes unique ownership through
/// an initialized writable slot and clears it; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_get_duration_f32(
    handle: *const LiblrhsmmRsPreparedModelF32,
    index: usize,
    output: *mut *mut LiblrhsmmRsPreparedDurationF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
    if index >= unsafe { (*handle).prepared.model().durations.len() } {
        return 2;
    }
    let Some(value) = (unsafe { (*handle).prepared.duration(index) }) else {
        return 3;
    };
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsPreparedDurationF32 {
                prepared: value.to_owned(),
            })))
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
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_get_emission_f32(
    handle: *const LiblrhsmmRsPreparedModelF32,
    stream: usize,
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
    let Some(value) = (unsafe { (*handle).prepared.emission(stream, state) }) else {
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
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_clone_f32(
    handle: *const LiblrhsmmRsPreparedModelF32,
    output: *mut *mut LiblrhsmmRsPreparedModelF32,
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
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_release_f32(
    handle: *mut *mut LiblrhsmmRsPreparedModelF32,
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
pub struct LiblrhsmmRsPreparedModelF64 {
    pub(super) prepared: OwnedPreparedModelParameters<f64>,
}
/// Copy the complete source and prepare an immutable independent snapshot.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Reads cannot overlap mutation or release. Constructor outputs hold no live owner.
/// Child results own independent copies. Release consumes unique ownership through
/// an initialized writable slot and clears it; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_prepare_f64(
    handle: *const LiblrhsmmRsModelF64,
    options: *const LiblrhsmmRsDurationPreparationF64,
    output: *mut *mut LiblrhsmmRsPreparedModelF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(options, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsPreparedModelF64 {
                prepared: (*handle)
                    .parameters
                    .clone()
                    .prepare_owned(options.read().into())?,
            })))
        })
    }
}
/// Prepare emissions and selected durations; empty selections permit null, duplicates prepare once.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Reads cannot overlap mutation or release. Constructor outputs hold no live owner.
/// Child results own independent copies. Release consumes unique ownership through
/// an initialized writable slot and clears it; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_prepare_selected_durations_f64(
    handle: *const LiblrhsmmRsModelF64,
    options: *const LiblrhsmmRsDurationPreparationF64,
    states: *const usize,
    count: usize,
    output: *mut *mut LiblrhsmmRsPreparedModelF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(options, 1) {
        return status;
    }
    if let Err(status) = range(states, count) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
    unsafe {
        scalar_result(output, || {
            let states = if count == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(states, count)
            };
            Ok(Box::into_raw(Box::new(LiblrhsmmRsPreparedModelF64 {
                prepared: (*handle)
                    .parameters
                    .clone()
                    .prepare_owned_selected_durations(options.read().into(), states)?,
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
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_parameters_f64(
    handle: *const LiblrhsmmRsPreparedModelF64,
    output: *mut *mut LiblrhsmmRsModelF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsModelF64 {
                parameters: (*handle).prepared.model().clone(),
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
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_layout_f64(
    handle: *const LiblrhsmmRsPreparedModelF64,
    output: *mut LiblrhsmmRsModelLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
    unsafe {
        scalar_result(output, || {
            let parameters = (*handle).prepared.model();
            Ok(LiblrhsmmRsModelLayout {
                streams: parameters.streams.len(),
                durations: parameters.durations.len(),
            })
        })
    }
}
/// Read all preparation settings used by this snapshot.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Reads cannot overlap mutation or release. Constructor outputs hold no live owner.
/// Child results own independent copies. Release consumes unique ownership through
/// an initialized writable slot and clears it; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_options_f64(
    handle: *const LiblrhsmmRsPreparedModelF64,
    output: *mut LiblrhsmmRsDurationPreparationF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
    unsafe { scalar_result(output, || Ok((*handle).prepared.options().into())) }
}
/// Read one stream layout.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Reads cannot overlap mutation or release. Constructor outputs hold no live owner.
/// Child results own independent copies. Release consumes unique ownership through
/// an initialized writable slot and clears it; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_stream_layout_f64(
    handle: *const LiblrhsmmRsPreparedModelF64,
    index: usize,
    output: *mut LiblrhsmmRsStreamLayoutF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
    let Some(stream) = (unsafe { (*handle).prepared.model() }).streams.get(index) else {
        return 2;
    };
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsStreamLayoutF64 {
                weight: stream.weight,
                emissions: stream.mixtures.len(),
            })
        })
    }
}
/// Copy every parameter of one stream into an editable independent owner.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Reads cannot overlap mutation or release. Constructor outputs hold no live owner.
/// Child results own independent copies. Release consumes unique ownership through
/// an initialized writable slot and clears it; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_stream_parameters_f64(
    handle: *const LiblrhsmmRsPreparedModelF64,
    index: usize,
    output: *mut *mut LiblrhsmmRsStreamF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
    let Some(stream) = (unsafe { (*handle).prepared.model() }).streams.get(index) else {
        return 2;
    };
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStreamF64 {
                parameters: stream.clone(),
            })))
        })
    }
}
/// Report whether an in-range duration was prepared; unselected durations return zero.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Reads cannot overlap mutation or release. Constructor outputs hold no live owner.
/// Child results own independent copies. Release consumes unique ownership through
/// an initialized writable slot and clears it; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_has_duration_f64(
    handle: *const LiblrhsmmRsPreparedModelF64,
    index: usize,
    output: *mut u32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
    if index >= unsafe { (*handle).prepared.model().durations.len() } {
        return 2;
    }
    unsafe {
        scalar_result(output, || {
            Ok(u32::from((*handle).prepared.duration(index).is_some()))
        })
    }
}
/// Copy a prepared duration and its existing cache; unselected in-range states return status three.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned,
/// initialized readable storage; outputs are independent aligned writable storage.
/// Reads cannot overlap mutation or release. Constructor outputs hold no live owner.
/// Child results own independent copies. Release consumes unique ownership through
/// an initialized writable slot and clears it; null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_get_duration_f64(
    handle: *const LiblrhsmmRsPreparedModelF64,
    index: usize,
    output: *mut *mut LiblrhsmmRsPreparedDurationF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Checked live owners and independent buffers satisfy the contract.
    if index >= unsafe { (*handle).prepared.model().durations.len() } {
        return 2;
    }
    let Some(value) = (unsafe { (*handle).prepared.duration(index) }) else {
        return 3;
    };
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsPreparedDurationF64 {
                prepared: value.to_owned(),
            })))
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
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_get_emission_f64(
    handle: *const LiblrhsmmRsPreparedModelF64,
    stream: usize,
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
    let Some(value) = (unsafe { (*handle).prepared.emission(stream, state) }) else {
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
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_clone_f64(
    handle: *const LiblrhsmmRsPreparedModelF64,
    output: *mut *mut LiblrhsmmRsPreparedModelF64,
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
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_release_f64(
    handle: *mut *mut LiblrhsmmRsPreparedModelF64,
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
