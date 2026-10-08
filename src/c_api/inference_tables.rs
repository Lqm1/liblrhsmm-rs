//! Independent log-emission, density and state-score storage for inference.
use super::gaussian::storage_range;
use super::*;
use crate::emission::{EmissionDensityTable, LogEmissionTable};
use crate::geometric::StateScoreTable;

/// Complete time-major table dimensions and scalar storage length.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiblrhsmmRsInferenceTableLayout {
    pub frames: usize,
    pub states: usize,
    pub values: usize,
}

fn shape(frames: usize, states: usize, count: usize) -> Result<(), u32> {
    if frames > i32::MAX as usize || states > i32::MAX as usize {
        return Err(2);
    }
    if frames == 0 || states == 0 || frames.checked_mul(states) != Some(count) {
        return Err(3);
    }
    Ok(())
}

unsafe fn owned_values<T: Copy>(input: *const T, count: usize) -> Result<Vec<T>, ModelError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(count)
        .map_err(|_| ModelError("table allocation failed"))?;
    if count != 0 {
        // SAFETY: Independent initialized readable scalar array, checked by caller.
        values.extend_from_slice(unsafe { std::slice::from_raw_parts(input, count) });
    }
    Ok(values)
}
pub(super) unsafe fn copy_values<T: Copy>(
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
    let Some(values) = values.get(offset..end) else {
        return 2;
    };
    if count != 0 {
        // SAFETY: Independent initialized writable output and checked immutable source.
        unsafe {
            std::ptr::copy_nonoverlapping(values.as_ptr(), output, count);
        }
    }
    0
}

/// Independently owned time-major log-emission table.
#[derive(Debug, Clone)]
pub struct LiblrhsmmRsLogEmissionsF32 {
    pub(super) table: LogEmissionTable<f32>,
}

/// Read complete dimensions and the flat scalar count.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_log_emissions_layout_f32(
    handle: *const LiblrhsmmRsLogEmissionsF32,
    output: *mut LiblrhsmmRsInferenceTableLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let table = &(*handle).table;
            Ok(LiblrhsmmRsInferenceTableLayout {
                frames: table.frames(),
                states: table.states(),
                values: table.values().len(),
            })
        })
    }
}

/// Copy a complete checked interval without exposing internal storage.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_log_emissions_copy_values_f32(
    handle: *const LiblrhsmmRsLogEmissionsF32,
    offset: usize,
    output: *mut f32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { copy_values((*handle).table.values(), offset, output, count) }
}

/// Copy one frame; count must equal the state count.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_log_emissions_copy_frame_f32(
    handle: *const LiblrhsmmRsLogEmissionsF32,
    frame: usize,
    output: *mut f32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    let table = unsafe { &(*handle).table };
    if frame >= table.frames() || count != table.states() {
        return 2;
    }
    unsafe { copy_values(table.values(), frame * table.states(), output, count) }
}

/// Read one frame/state value with the selected precision retained.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_log_emissions_get_f32(
    handle: *const LiblrhsmmRsLogEmissionsF32,
    frame: usize,
    state: usize,
    output: *mut f32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    let Some(value) = (unsafe { &(*handle).table }).get(frame, state) else {
        return 2;
    };
    unsafe { scalar_result(output, || Ok(value)) }
}

/// Independently clone complete table storage and dimensions.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_log_emissions_clone_f32(
    handle: *const LiblrhsmmRsLogEmissionsF32,
    output: *mut *mut LiblrhsmmRsLogEmissionsF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || Ok(Box::into_raw(Box::new((*handle).clone())))) }
}

/// Release a unique table owner and clear its initialized slot.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_log_emissions_release_f32(
    handle: *mut *mut LiblrhsmmRsLogEmissionsF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    let owner = unsafe { *handle };
    if owner.is_null() {
        return 0;
    }
    match boundary(|| {
        unsafe {
            drop(Box::from_raw(owner));
        }
        Ok(())
    }) {
        Ok(()) => {
            unsafe {
                handle.write(std::ptr::null_mut());
            }
            0
        }
        Err(status) => status,
    }
}

/// Import complete dimensions and scalar storage through native validation.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_log_emissions_create_f32(
    frames: usize,
    states: usize,
    values: *const f32,
    count: usize,
    output: *mut *mut LiblrhsmmRsLogEmissionsF32,
) -> u32 {
    if let Err(status) = range(output, 1)
        .and_then(|()| shape(frames, states, count))
        .and_then(|()| storage_range(values, count))
    {
        return status;
    }
    match boundary(|| {
        let values = unsafe { owned_values(values, count) }?;
        let table = LogEmissionTable::from_log_values(frames, states, values)?;
        Ok(Box::into_raw(Box::new(LiblrhsmmRsLogEmissionsF32 {
            table,
        })))
    }) {
        Ok(owner) => {
            unsafe {
                output.write(owner);
            }
            0
        }
        Err(status) => status,
    }
}

/// Replace complete dimensions and scalar storage after native validation succeeds.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_log_emissions_replace_f32(
    handle: *mut LiblrhsmmRsLogEmissionsF32,
    frames: usize,
    states: usize,
    values: *const f32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1)
        .and_then(|()| shape(frames, states, count))
        .and_then(|()| storage_range(values, count))
    {
        return status;
    }
    match boundary(|| {
        let values = unsafe { owned_values(values, count) }?;
        LogEmissionTable::from_log_values(frames, states, values)
    }) {
        Ok(table) => {
            unsafe {
                (*handle).table = table;
            }
            0
        }
        Err(status) => status,
    }
}

/// Convert all log scores with the unchanged native binary32 compatibility exponential.
/// Density storage has a separate type and cannot be passed as log-emission input.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_log_emissions_probabilities_f32(
    handle: *const LiblrhsmmRsLogEmissionsF32,
    output: *mut *mut LiblrhsmmRsEmissionDensitiesF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsEmissionDensitiesF32 {
                table: (*handle).table.probabilities(),
            })))
        })
    }
}

/// Independently owned time-major state-score table.
#[derive(Debug, Clone)]
pub struct LiblrhsmmRsStateScoresF32 {
    pub(super) table: StateScoreTable<f32>,
}

/// Read complete dimensions and the flat scalar count.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_scores_layout_f32(
    handle: *const LiblrhsmmRsStateScoresF32,
    output: *mut LiblrhsmmRsInferenceTableLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let table = &(*handle).table;
            Ok(LiblrhsmmRsInferenceTableLayout {
                frames: table.frames(),
                states: table.states(),
                values: table.values().len(),
            })
        })
    }
}

/// Copy a complete checked interval without exposing internal storage.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_scores_copy_values_f32(
    handle: *const LiblrhsmmRsStateScoresF32,
    offset: usize,
    output: *mut f32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { copy_values((*handle).table.values(), offset, output, count) }
}

/// Copy one frame; count must equal the state count.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_scores_copy_frame_f32(
    handle: *const LiblrhsmmRsStateScoresF32,
    frame: usize,
    output: *mut f32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    let table = unsafe { &(*handle).table };
    if frame >= table.frames() || count != table.states() {
        return 2;
    }
    unsafe { copy_values(table.values(), frame * table.states(), output, count) }
}

/// Read one frame/state value with the selected precision retained.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_scores_get_f32(
    handle: *const LiblrhsmmRsStateScoresF32,
    frame: usize,
    state: usize,
    output: *mut f32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    let Some(value) = (unsafe { &(*handle).table }).get(frame, state) else {
        return 2;
    };
    unsafe { scalar_result(output, || Ok(value)) }
}

/// Independently clone complete table storage and dimensions.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_scores_clone_f32(
    handle: *const LiblrhsmmRsStateScoresF32,
    output: *mut *mut LiblrhsmmRsStateScoresF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || Ok(Box::into_raw(Box::new((*handle).clone())))) }
}

/// Release a unique table owner and clear its initialized slot.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_scores_release_f32(
    handle: *mut *mut LiblrhsmmRsStateScoresF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    let owner = unsafe { *handle };
    if owner.is_null() {
        return 0;
    }
    match boundary(|| {
        unsafe {
            drop(Box::from_raw(owner));
        }
        Ok(())
    }) {
        Ok(()) => {
            unsafe {
                handle.write(std::ptr::null_mut());
            }
            0
        }
        Err(status) => status,
    }
}

/// Import complete dimensions and scalar storage through native validation.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_scores_create_f32(
    frames: usize,
    states: usize,
    values: *const f32,
    count: usize,
    output: *mut *mut LiblrhsmmRsStateScoresF32,
) -> u32 {
    if let Err(status) = range(output, 1)
        .and_then(|()| shape(frames, states, count))
        .and_then(|()| storage_range(values, count))
    {
        return status;
    }
    match boundary(|| {
        let values = unsafe { owned_values(values, count) }?;
        let table = StateScoreTable::from_values(frames, states, values)?;
        Ok(Box::into_raw(Box::new(LiblrhsmmRsStateScoresF32 { table })))
    }) {
        Ok(owner) => {
            unsafe {
                output.write(owner);
            }
            0
        }
        Err(status) => status,
    }
}

/// Replace complete dimensions and scalar storage after native validation succeeds.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_scores_replace_f32(
    handle: *mut LiblrhsmmRsStateScoresF32,
    frames: usize,
    states: usize,
    values: *const f32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1)
        .and_then(|()| shape(frames, states, count))
        .and_then(|()| storage_range(values, count))
    {
        return status;
    }
    match boundary(|| {
        let values = unsafe { owned_values(values, count) }?;
        StateScoreTable::from_values(frames, states, values)
    }) {
        Ok(table) => {
            unsafe {
                (*handle).table = table;
            }
            0
        }
        Err(status) => status,
    }
}

/// Sum the final forward row with the native quantized log-add calculation.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_scores_total_f32(
    handle: *const LiblrhsmmRsStateScoresF32,
    output: *mut f32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || Ok((*handle).table.total_log_probability())) }
}

/// Sum forward/backward products through one state using the native calculation.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_scores_total_through_state_f32(
    forward: *const LiblrhsmmRsStateScoresF32,
    backward: *const LiblrhsmmRsStateScoresF32,
    state: usize,
    output: *mut f32,
) -> u32 {
    if let Err(status) = range(forward, 1).and_then(|()| range(backward, 1)) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            (*forward)
                .table
                .total_through_state(&(*backward).table, state)
        })
    }
}

/// Recover the native backward likelihood from matching emission and score tables.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_scores_backward_total_f32(
    handle: *const LiblrhsmmRsStateScoresF32,
    emissions: *const LiblrhsmmRsLogEmissionsF32,
    output: *mut f32,
) -> u32 {
    if let Err(status) = range(handle, 1).and_then(|()| range(emissions, 1)) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            (*handle)
                .table
                .backward_log_probability(&(*emissions).table)
        })
    }
}

/// Independently owned time-major emission-density table.
#[derive(Debug, Clone)]
pub struct LiblrhsmmRsEmissionDensitiesF32 {
    pub(super) table: EmissionDensityTable<f32>,
}

/// Read complete dimensions and the flat scalar count.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_emission_densities_layout_f32(
    handle: *const LiblrhsmmRsEmissionDensitiesF32,
    output: *mut LiblrhsmmRsInferenceTableLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let table = &(*handle).table;
            Ok(LiblrhsmmRsInferenceTableLayout {
                frames: table.frames(),
                states: table.states(),
                values: table.values().len(),
            })
        })
    }
}

/// Copy a complete checked interval without exposing internal storage.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_emission_densities_copy_values_f32(
    handle: *const LiblrhsmmRsEmissionDensitiesF32,
    offset: usize,
    output: *mut f32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { copy_values((*handle).table.values(), offset, output, count) }
}

/// Copy one frame; count must equal the state count.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_emission_densities_copy_frame_f32(
    handle: *const LiblrhsmmRsEmissionDensitiesF32,
    frame: usize,
    output: *mut f32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    let table = unsafe { &(*handle).table };
    if frame >= table.frames() || count != table.states() {
        return 2;
    }
    unsafe { copy_values(table.values(), frame * table.states(), output, count) }
}

/// Read one frame/state value with the selected precision retained.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_emission_densities_get_f32(
    handle: *const LiblrhsmmRsEmissionDensitiesF32,
    frame: usize,
    state: usize,
    output: *mut f32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    let Some(value) = (unsafe { &(*handle).table }).get(frame, state) else {
        return 2;
    };
    unsafe { scalar_result(output, || Ok(value)) }
}

/// Independently clone complete table storage and dimensions.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_emission_densities_clone_f32(
    handle: *const LiblrhsmmRsEmissionDensitiesF32,
    output: *mut *mut LiblrhsmmRsEmissionDensitiesF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || Ok(Box::into_raw(Box::new((*handle).clone())))) }
}

/// Release a unique table owner and clear its initialized slot.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_emission_densities_release_f32(
    handle: *mut *mut LiblrhsmmRsEmissionDensitiesF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    let owner = unsafe { *handle };
    if owner.is_null() {
        return 0;
    }
    match boundary(|| {
        unsafe {
            drop(Box::from_raw(owner));
        }
        Ok(())
    }) {
        Ok(()) => {
            unsafe {
                handle.write(std::ptr::null_mut());
            }
            0
        }
        Err(status) => status,
    }
}

/// Independently owned time-major log-emission table.
#[derive(Debug, Clone)]
pub struct LiblrhsmmRsLogEmissionsF64 {
    pub(super) table: LogEmissionTable<f64>,
}

/// Read complete dimensions and the flat scalar count.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_log_emissions_layout_f64(
    handle: *const LiblrhsmmRsLogEmissionsF64,
    output: *mut LiblrhsmmRsInferenceTableLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let table = &(*handle).table;
            Ok(LiblrhsmmRsInferenceTableLayout {
                frames: table.frames(),
                states: table.states(),
                values: table.values().len(),
            })
        })
    }
}

/// Copy a complete checked interval without exposing internal storage.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_log_emissions_copy_values_f64(
    handle: *const LiblrhsmmRsLogEmissionsF64,
    offset: usize,
    output: *mut f64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { copy_values((*handle).table.values(), offset, output, count) }
}

/// Copy one frame; count must equal the state count.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_log_emissions_copy_frame_f64(
    handle: *const LiblrhsmmRsLogEmissionsF64,
    frame: usize,
    output: *mut f64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    let table = unsafe { &(*handle).table };
    if frame >= table.frames() || count != table.states() {
        return 2;
    }
    unsafe { copy_values(table.values(), frame * table.states(), output, count) }
}

/// Read one frame/state value with the selected precision retained.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_log_emissions_get_f64(
    handle: *const LiblrhsmmRsLogEmissionsF64,
    frame: usize,
    state: usize,
    output: *mut f64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    let Some(value) = (unsafe { &(*handle).table }).get(frame, state) else {
        return 2;
    };
    unsafe { scalar_result(output, || Ok(value)) }
}

/// Independently clone complete table storage and dimensions.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_log_emissions_clone_f64(
    handle: *const LiblrhsmmRsLogEmissionsF64,
    output: *mut *mut LiblrhsmmRsLogEmissionsF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || Ok(Box::into_raw(Box::new((*handle).clone())))) }
}

/// Release a unique table owner and clear its initialized slot.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_log_emissions_release_f64(
    handle: *mut *mut LiblrhsmmRsLogEmissionsF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    let owner = unsafe { *handle };
    if owner.is_null() {
        return 0;
    }
    match boundary(|| {
        unsafe {
            drop(Box::from_raw(owner));
        }
        Ok(())
    }) {
        Ok(()) => {
            unsafe {
                handle.write(std::ptr::null_mut());
            }
            0
        }
        Err(status) => status,
    }
}

/// Import complete dimensions and scalar storage through native validation.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_log_emissions_create_f64(
    frames: usize,
    states: usize,
    values: *const f64,
    count: usize,
    output: *mut *mut LiblrhsmmRsLogEmissionsF64,
) -> u32 {
    if let Err(status) = range(output, 1)
        .and_then(|()| shape(frames, states, count))
        .and_then(|()| storage_range(values, count))
    {
        return status;
    }
    match boundary(|| {
        let values = unsafe { owned_values(values, count) }?;
        let table = LogEmissionTable::from_log_values(frames, states, values)?;
        Ok(Box::into_raw(Box::new(LiblrhsmmRsLogEmissionsF64 {
            table,
        })))
    }) {
        Ok(owner) => {
            unsafe {
                output.write(owner);
            }
            0
        }
        Err(status) => status,
    }
}

/// Replace complete dimensions and scalar storage after native validation succeeds.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_log_emissions_replace_f64(
    handle: *mut LiblrhsmmRsLogEmissionsF64,
    frames: usize,
    states: usize,
    values: *const f64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1)
        .and_then(|()| shape(frames, states, count))
        .and_then(|()| storage_range(values, count))
    {
        return status;
    }
    match boundary(|| {
        let values = unsafe { owned_values(values, count) }?;
        LogEmissionTable::from_log_values(frames, states, values)
    }) {
        Ok(table) => {
            unsafe {
                (*handle).table = table;
            }
            0
        }
        Err(status) => status,
    }
}

/// Convert all log scores with the unchanged native binary32 compatibility exponential.
/// Density storage has a separate type and cannot be passed as log-emission input.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_log_emissions_probabilities_f64(
    handle: *const LiblrhsmmRsLogEmissionsF64,
    output: *mut *mut LiblrhsmmRsEmissionDensitiesF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsEmissionDensitiesF64 {
                table: (*handle).table.probabilities(),
            })))
        })
    }
}

/// Independently owned time-major state-score table.
#[derive(Debug, Clone)]
pub struct LiblrhsmmRsStateScoresF64 {
    pub(super) table: StateScoreTable<f64>,
}

/// Read complete dimensions and the flat scalar count.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_scores_layout_f64(
    handle: *const LiblrhsmmRsStateScoresF64,
    output: *mut LiblrhsmmRsInferenceTableLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let table = &(*handle).table;
            Ok(LiblrhsmmRsInferenceTableLayout {
                frames: table.frames(),
                states: table.states(),
                values: table.values().len(),
            })
        })
    }
}

/// Copy a complete checked interval without exposing internal storage.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_scores_copy_values_f64(
    handle: *const LiblrhsmmRsStateScoresF64,
    offset: usize,
    output: *mut f64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { copy_values((*handle).table.values(), offset, output, count) }
}

/// Copy one frame; count must equal the state count.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_scores_copy_frame_f64(
    handle: *const LiblrhsmmRsStateScoresF64,
    frame: usize,
    output: *mut f64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    let table = unsafe { &(*handle).table };
    if frame >= table.frames() || count != table.states() {
        return 2;
    }
    unsafe { copy_values(table.values(), frame * table.states(), output, count) }
}

/// Read one frame/state value with the selected precision retained.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_scores_get_f64(
    handle: *const LiblrhsmmRsStateScoresF64,
    frame: usize,
    state: usize,
    output: *mut f64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    let Some(value) = (unsafe { &(*handle).table }).get(frame, state) else {
        return 2;
    };
    unsafe { scalar_result(output, || Ok(value)) }
}

/// Independently clone complete table storage and dimensions.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_scores_clone_f64(
    handle: *const LiblrhsmmRsStateScoresF64,
    output: *mut *mut LiblrhsmmRsStateScoresF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || Ok(Box::into_raw(Box::new((*handle).clone())))) }
}

/// Release a unique table owner and clear its initialized slot.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_scores_release_f64(
    handle: *mut *mut LiblrhsmmRsStateScoresF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    let owner = unsafe { *handle };
    if owner.is_null() {
        return 0;
    }
    match boundary(|| {
        unsafe {
            drop(Box::from_raw(owner));
        }
        Ok(())
    }) {
        Ok(()) => {
            unsafe {
                handle.write(std::ptr::null_mut());
            }
            0
        }
        Err(status) => status,
    }
}

/// Import complete dimensions and scalar storage through native validation.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_scores_create_f64(
    frames: usize,
    states: usize,
    values: *const f64,
    count: usize,
    output: *mut *mut LiblrhsmmRsStateScoresF64,
) -> u32 {
    if let Err(status) = range(output, 1)
        .and_then(|()| shape(frames, states, count))
        .and_then(|()| storage_range(values, count))
    {
        return status;
    }
    match boundary(|| {
        let values = unsafe { owned_values(values, count) }?;
        let table = StateScoreTable::from_values(frames, states, values)?;
        Ok(Box::into_raw(Box::new(LiblrhsmmRsStateScoresF64 { table })))
    }) {
        Ok(owner) => {
            unsafe {
                output.write(owner);
            }
            0
        }
        Err(status) => status,
    }
}

/// Replace complete dimensions and scalar storage after native validation succeeds.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_scores_replace_f64(
    handle: *mut LiblrhsmmRsStateScoresF64,
    frames: usize,
    states: usize,
    values: *const f64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1)
        .and_then(|()| shape(frames, states, count))
        .and_then(|()| storage_range(values, count))
    {
        return status;
    }
    match boundary(|| {
        let values = unsafe { owned_values(values, count) }?;
        StateScoreTable::from_values(frames, states, values)
    }) {
        Ok(table) => {
            unsafe {
                (*handle).table = table;
            }
            0
        }
        Err(status) => status,
    }
}

/// Sum the final forward row with the native quantized log-add calculation.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_scores_total_f64(
    handle: *const LiblrhsmmRsStateScoresF64,
    output: *mut f64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || Ok((*handle).table.total_log_probability())) }
}

/// Sum forward/backward products through one state using the native calculation.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_scores_total_through_state_f64(
    forward: *const LiblrhsmmRsStateScoresF64,
    backward: *const LiblrhsmmRsStateScoresF64,
    state: usize,
    output: *mut f64,
) -> u32 {
    if let Err(status) = range(forward, 1).and_then(|()| range(backward, 1)) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            (*forward)
                .table
                .total_through_state(&(*backward).table, state)
        })
    }
}

/// Recover the native backward likelihood from matching emission and score tables.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_scores_backward_total_f64(
    handle: *const LiblrhsmmRsStateScoresF64,
    emissions: *const LiblrhsmmRsLogEmissionsF64,
    output: *mut f64,
) -> u32 {
    if let Err(status) = range(handle, 1).and_then(|()| range(emissions, 1)) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            (*handle)
                .table
                .backward_log_probability(&(*emissions).table)
        })
    }
}

/// Independently owned time-major emission-density table.
#[derive(Debug, Clone)]
pub struct LiblrhsmmRsEmissionDensitiesF64 {
    pub(super) table: EmissionDensityTable<f64>,
}

/// Read complete dimensions and the flat scalar count.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_emission_densities_layout_f64(
    handle: *const LiblrhsmmRsEmissionDensitiesF64,
    output: *mut LiblrhsmmRsInferenceTableLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let table = &(*handle).table;
            Ok(LiblrhsmmRsInferenceTableLayout {
                frames: table.frames(),
                states: table.states(),
                values: table.values().len(),
            })
        })
    }
}

/// Copy a complete checked interval without exposing internal storage.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_emission_densities_copy_values_f64(
    handle: *const LiblrhsmmRsEmissionDensitiesF64,
    offset: usize,
    output: *mut f64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { copy_values((*handle).table.values(), offset, output, count) }
}

/// Copy one frame; count must equal the state count.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_emission_densities_copy_frame_f64(
    handle: *const LiblrhsmmRsEmissionDensitiesF64,
    frame: usize,
    output: *mut f64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    let table = unsafe { &(*handle).table };
    if frame >= table.frames() || count != table.states() {
        return 2;
    }
    unsafe { copy_values(table.values(), frame * table.states(), output, count) }
}

/// Read one frame/state value with the selected precision retained.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_emission_densities_get_f64(
    handle: *const LiblrhsmmRsEmissionDensitiesF64,
    frame: usize,
    state: usize,
    output: *mut f64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    let Some(value) = (unsafe { &(*handle).table }).get(frame, state) else {
        return 2;
    };
    unsafe { scalar_result(output, || Ok(value)) }
}

/// Independently clone complete table storage and dimensions.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_emission_densities_clone_f64(
    handle: *const LiblrhsmmRsEmissionDensitiesF64,
    output: *mut *mut LiblrhsmmRsEmissionDensitiesF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || Ok(Box::into_raw(Box::new((*handle).clone())))) }
}

/// Release a unique table owner and clear its initialized slot.
/// # Safety
/// Handles are live owners of the declared precision. Arrays are aligned initialized
/// readable storage for stated lengths; outputs are independent aligned writable
/// storage. Reads cannot overlap mutation or release. Constructor slots hold no
/// live owner. Result owners are independent copies; no borrowed pointer escapes.
/// Release consumes unique ownership and clears its slot. Null ownership is a
/// release no-op. Empty interval copies permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_emission_densities_release_f64(
    handle: *mut *mut LiblrhsmmRsEmissionDensitiesF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    let owner = unsafe { *handle };
    if owner.is_null() {
        return 0;
    }
    match boundary(|| {
        unsafe {
            drop(Box::from_raw(owner));
        }
        Ok(())
    }) {
        Ok(()) => {
            unsafe {
                handle.write(std::ptr::null_mut());
            }
            0
        }
        Err(status) => status,
    }
}
