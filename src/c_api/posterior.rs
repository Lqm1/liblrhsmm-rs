//! Complete posterior storage, preserving absent and calculated empty pairs.
use super::gaussian::storage_range;
use super::inference_tables::copy_values;
use super::*;
use crate::duration_occupancy::{DurationOccupancies, DurationPosterior};
use crate::mixture_occupancy::{MixtureOccupancies, MixtureOccupancySettings};
use crate::occupancy::{StateOccupancyTable, TransitionOccupancies, TransitionPosterior};

/// Borrowed optional float slice; present must be zero or one. Absent fields are ignored.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LiblrhsmmRsPosteriorInput {
    pub present: u32,
    pub minimum: usize,
    pub values: *const f32,
    pub count: usize,
}
/// Complete copied optional slice metadata. Present empty slices remain distinct from absent.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiblrhsmmRsPosteriorSlice {
    pub present: u32,
    pub minimum: usize,
    pub values: usize,
}
/// Complete optional arrival weights; present must be zero or one.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LiblrhsmmRsTransitionPosterior {
    pub present: u32,
    pub stay: f32,
    pub advance: f32,
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
unsafe fn float_values(input: *const f32, count: usize) -> Result<Vec<f32>, ModelError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(count)
        .map_err(|_| ModelError("posterior allocation failed"))?;
    if count != 0 {
        unsafe {
            values.extend_from_slice(std::slice::from_raw_parts(input, count));
        }
    }
    Ok(values)
}
unsafe fn validate_slices(
    input: *const LiblrhsmmRsPosteriorInput,
    count: usize,
) -> Result<(), u32> {
    storage_range(input, count)?;
    for value in unsafe { std::slice::from_raw_parts(input, count) } {
        match value.present {
            0 => {}
            1 => storage_range(value.values, value.count)?,
            _ => return Err(3),
        }
    }
    Ok(())
}

/// Complete independent duration posterior table. Values retain native binary32 storage.
#[derive(Debug, Clone)]
pub struct LiblrhsmmRsDurationOccupancies {
    pub(super) table: DurationOccupancies,
}

unsafe fn import_duration(
    frames: usize,
    states: usize,
    input: *const LiblrhsmmRsPosteriorInput,
    count: usize,
) -> Result<DurationOccupancies, u32> {
    shape(frames, states, count)?;

    unsafe {
        validate_slices(input, count)?;
    }
    boundary(|| {
        let input = unsafe { std::slice::from_raw_parts(input, count) };
        let mut values = Vec::new();
        values
            .try_reserve_exact(count)
            .map_err(|_| ModelError("posterior allocation failed"))?;
        for value in input {
            values.push(if value.present == 0 {
                None
            } else {
                Some(unsafe {
                    DurationPosterior {
                        minimum: value.minimum,
                        probabilities: float_values(value.values, value.count)?,
                    }
                })
            });
        }
        DurationOccupancies::from_values(frames, states, values)
    })
}

/// Copy complete external weights, retaining optional presence; failures preserve initialized state.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_occupancies_create(
    frames: usize,
    states: usize,
    input: *const LiblrhsmmRsPosteriorInput,
    count: usize,
    output: *mut *mut LiblrhsmmRsDurationOccupancies,
) -> u32 {
    if let Err(status) = range(output, 1) {
        return status;
    }
    let table = match unsafe { import_duration(frames, states, input, count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsDurationOccupancies {
                table,
            })))
        })
    }
}

/// Copy complete external weights, retaining optional presence; failures preserve initialized state.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_occupancies_replace(
    handle: *mut LiblrhsmmRsDurationOccupancies,
    frames: usize,
    states: usize,
    input: *const LiblrhsmmRsPosteriorInput,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    let table = match unsafe { import_duration(frames, states, input, count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    unsafe {
        (*handle).table = table;
    }
    0
}

/// Read complete time-major dimensions and entry count.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_occupancies_layout(
    handle: *const LiblrhsmmRsDurationOccupancies,
    output: *mut LiblrhsmmRsInferenceTableLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsInferenceTableLayout {
                frames: (*handle).table.frames(),
                states: (*handle).table.states(),
                values: (*handle).table.values().len(),
            })
        })
    }
}

/// Deep-copy every optional entry and complete scalar storage.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_occupancies_clone(
    handle: *const LiblrhsmmRsDurationOccupancies,
    output: *mut *mut LiblrhsmmRsDurationOccupancies,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || Ok(Box::into_raw(Box::new((*handle).clone())))) }
}

/// Release unique ownership and clear its slot.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_occupancies_release(
    slot: *mut *mut LiblrhsmmRsDurationOccupancies,
) -> u32 {
    if let Err(status) = range(slot, 1) {
        return status;
    }
    unsafe {
        let value = slot.read();
        if !value.is_null() {
            if let Err(status) = range(value, 1) {
                return status;
            }
            drop(Box::from_raw(value));
        }
        slot.write(std::ptr::null_mut());
    }
    0
}

/// Read presence, starting index and complete scalar count, including present empty slices.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_occupancies_get_slice(
    handle: *const LiblrhsmmRsDurationOccupancies,
    frame: usize,
    state: usize,
    output: *mut LiblrhsmmRsPosteriorSlice,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        let table = &(*handle).table;
        if frame >= table.frames() || state >= table.states() {
            return 2;
        }
        scalar_result(output, || {
            Ok(match table.get(frame, state) {
                Some(value) => LiblrhsmmRsPosteriorSlice {
                    present: 1,
                    minimum: value.minimum,
                    values: value.probabilities.as_slice().len(),
                },
                None => LiblrhsmmRsPosteriorSlice {
                    present: 0,
                    minimum: 0,
                    values: 0,
                },
            })
        })
    }
}

/// Copy a complete checked slice interval. Absent entries expose only an empty interval.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_occupancies_copy_values(
    handle: *const LiblrhsmmRsDurationOccupancies,
    frame: usize,
    state: usize,
    offset: usize,
    output: *mut f32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        let table = &(*handle).table;
        if frame >= table.frames() || state >= table.states() {
            return 2;
        }
        let values = table
            .get(frame, state)
            .map(|value| value.probabilities.as_slice())
            .unwrap_or(&[]);
        copy_values(values, offset, output, count)
    }
}

/// Read the native absolute-duration weight, returning zero for absent or out-of-slice durations.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_duration_occupancies_probability(
    handle: *const LiblrhsmmRsDurationOccupancies,
    frame: usize,
    state: usize,
    duration: usize,
    output: *mut f32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        let table = &(*handle).table;
        if frame >= table.frames() || state >= table.states() {
            return 2;
        }
        scalar_result(output, || {
            Ok(table
                .get(frame, state)
                .map_or(0.0, |value| value.probability(duration)))
        })
    }
}

/// Complete independent mixture posterior table. Values retain native binary32 storage.
#[derive(Debug, Clone)]
pub struct LiblrhsmmRsMixtureOccupancies {
    pub(super) table: MixtureOccupancies,
}

unsafe fn import_mixture(
    frames: usize,
    states: usize,
    input: *const LiblrhsmmRsPosteriorInput,
    count: usize,
) -> Result<MixtureOccupancies, u32> {
    shape(frames, states, count)?;

    unsafe {
        validate_slices(input, count)?;
    }
    boundary(|| {
        let input = unsafe { std::slice::from_raw_parts(input, count) };
        let mut values = Vec::new();
        values
            .try_reserve_exact(count)
            .map_err(|_| ModelError("posterior allocation failed"))?;
        for value in input {
            values.push(if value.present == 0 {
                None
            } else {
                Some(unsafe { float_values(value.values, value.count)? })
            });
        }
        MixtureOccupancies::from_values(frames, states, values)
    })
}

/// Copy complete external weights, retaining optional presence; failures preserve initialized state.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_mixture_occupancies_create(
    frames: usize,
    states: usize,
    input: *const LiblrhsmmRsPosteriorInput,
    count: usize,
    output: *mut *mut LiblrhsmmRsMixtureOccupancies,
) -> u32 {
    if let Err(status) = range(output, 1) {
        return status;
    }
    let table = match unsafe { import_mixture(frames, states, input, count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsMixtureOccupancies {
                table,
            })))
        })
    }
}

/// Copy complete external weights, retaining optional presence; failures preserve initialized state.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_mixture_occupancies_replace(
    handle: *mut LiblrhsmmRsMixtureOccupancies,
    frames: usize,
    states: usize,
    input: *const LiblrhsmmRsPosteriorInput,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    let table = match unsafe { import_mixture(frames, states, input, count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    unsafe {
        (*handle).table = table;
    }
    0
}

/// Read complete time-major dimensions and entry count.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_mixture_occupancies_layout(
    handle: *const LiblrhsmmRsMixtureOccupancies,
    output: *mut LiblrhsmmRsInferenceTableLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsInferenceTableLayout {
                frames: (*handle).table.frames(),
                states: (*handle).table.states(),
                values: (*handle).table.values().len(),
            })
        })
    }
}

/// Deep-copy every optional entry and complete scalar storage.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_mixture_occupancies_clone(
    handle: *const LiblrhsmmRsMixtureOccupancies,
    output: *mut *mut LiblrhsmmRsMixtureOccupancies,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || Ok(Box::into_raw(Box::new((*handle).clone())))) }
}

/// Release unique ownership and clear its slot.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_mixture_occupancies_release(
    slot: *mut *mut LiblrhsmmRsMixtureOccupancies,
) -> u32 {
    if let Err(status) = range(slot, 1) {
        return status;
    }
    unsafe {
        let value = slot.read();
        if !value.is_null() {
            if let Err(status) = range(value, 1) {
                return status;
            }
            drop(Box::from_raw(value));
        }
        slot.write(std::ptr::null_mut());
    }
    0
}

/// Read presence, starting index and complete scalar count, including present empty slices.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_mixture_occupancies_get_slice(
    handle: *const LiblrhsmmRsMixtureOccupancies,
    frame: usize,
    state: usize,
    output: *mut LiblrhsmmRsPosteriorSlice,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        let table = &(*handle).table;
        if frame >= table.frames() || state >= table.states() {
            return 2;
        }
        scalar_result(output, || {
            Ok(match table.get(frame, state) {
                Some(value) => LiblrhsmmRsPosteriorSlice {
                    present: 1,
                    minimum: 0,
                    values: value.len(),
                },
                None => LiblrhsmmRsPosteriorSlice {
                    present: 0,
                    minimum: 0,
                    values: 0,
                },
            })
        })
    }
}

/// Copy a complete checked slice interval. Absent entries expose only an empty interval.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_mixture_occupancies_copy_values(
    handle: *const LiblrhsmmRsMixtureOccupancies,
    frame: usize,
    state: usize,
    offset: usize,
    output: *mut f32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        let table = &(*handle).table;
        if frame >= table.frames() || state >= table.states() {
            return 2;
        }
        let values = table.get(frame, state).unwrap_or(&[]);
        copy_values(values, offset, output, count)
    }
}

/// Complete independent transition posterior table. Values retain native binary32 storage.
#[derive(Debug, Clone)]
pub struct LiblrhsmmRsTransitionOccupancies {
    pub(super) table: TransitionOccupancies,
}

unsafe fn import_transition(
    frames: usize,
    states: usize,
    input: *const LiblrhsmmRsTransitionPosterior,
    count: usize,
) -> Result<TransitionOccupancies, u32> {
    shape(frames, states, count)?;

    storage_range(input, count)?;
    let input = unsafe { std::slice::from_raw_parts(input, count) };
    if input.iter().any(|value| value.present > 1) {
        return Err(3);
    }
    boundary(|| {
        let mut values = Vec::new();
        values
            .try_reserve_exact(count)
            .map_err(|_| ModelError("posterior allocation failed"))?;
        for value in input {
            values.push(if value.present == 0 {
                None
            } else {
                Some(TransitionPosterior {
                    stay: value.stay,
                    advance: value.advance,
                })
            });
        }
        TransitionOccupancies::from_values(frames, states, values)
    })
}

/// Copy complete external weights, retaining optional presence; failures preserve initialized state.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_transition_occupancies_create(
    frames: usize,
    states: usize,
    input: *const LiblrhsmmRsTransitionPosterior,
    count: usize,
    output: *mut *mut LiblrhsmmRsTransitionOccupancies,
) -> u32 {
    if let Err(status) = range(output, 1) {
        return status;
    }
    let table = match unsafe { import_transition(frames, states, input, count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsTransitionOccupancies {
                table,
            })))
        })
    }
}

/// Copy complete external weights, retaining optional presence; failures preserve initialized state.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_transition_occupancies_replace(
    handle: *mut LiblrhsmmRsTransitionOccupancies,
    frames: usize,
    states: usize,
    input: *const LiblrhsmmRsTransitionPosterior,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    let table = match unsafe { import_transition(frames, states, input, count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    unsafe {
        (*handle).table = table;
    }
    0
}

/// Read complete time-major dimensions and entry count.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_transition_occupancies_layout(
    handle: *const LiblrhsmmRsTransitionOccupancies,
    output: *mut LiblrhsmmRsInferenceTableLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsInferenceTableLayout {
                frames: (*handle).table.frames(),
                states: (*handle).table.states(),
                values: (*handle).table.values().len(),
            })
        })
    }
}

/// Deep-copy every optional entry and complete scalar storage.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_transition_occupancies_clone(
    handle: *const LiblrhsmmRsTransitionOccupancies,
    output: *mut *mut LiblrhsmmRsTransitionOccupancies,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || Ok(Box::into_raw(Box::new((*handle).clone())))) }
}

/// Release unique ownership and clear its slot.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_transition_occupancies_release(
    slot: *mut *mut LiblrhsmmRsTransitionOccupancies,
) -> u32 {
    if let Err(status) = range(slot, 1) {
        return status;
    }
    unsafe {
        let value = slot.read();
        if !value.is_null() {
            if let Err(status) = range(value, 1) {
                return status;
            }
            drop(Box::from_raw(value));
        }
        slot.write(std::ptr::null_mut());
    }
    0
}

/// Read optional arrival weights without conflating missing and calculated zero values.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_transition_occupancies_get(
    handle: *const LiblrhsmmRsTransitionOccupancies,
    frame: usize,
    state: usize,
    output: *mut LiblrhsmmRsTransitionPosterior,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        let table = &(*handle).table;
        if frame >= table.frames() || state >= table.states() {
            return 2;
        }
        scalar_result(output, || {
            Ok(match table.get(frame, state) {
                Some(value) => LiblrhsmmRsTransitionPosterior {
                    present: 1,
                    stay: value.stay,
                    advance: value.advance,
                },
                None => LiblrhsmmRsTransitionPosterior {
                    present: 0,
                    stay: 0.0,
                    advance: 0.0,
                },
            })
        })
    }
}

/// Independent time-major state occupancies in the declared precision.
#[derive(Debug, Clone)]
pub struct LiblrhsmmRsStateOccupanciesF32 {
    pub(super) table: StateOccupancyTable<f32>,
}

/// Copy complete finite nonnegative weights; complete validation precedes replacement/publication.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_occupancies_create_f32(
    frames: usize,
    states: usize,
    input: *const f32,
    count: usize,
    output: *mut *mut LiblrhsmmRsStateOccupanciesF32,
) -> u32 {
    if let Err(status) = range(output, 1) {
        return status;
    }
    if let Err(status) = shape(frames, states, count).and_then(|()| storage_range(input, count)) {
        return status;
    }
    let table = match boundary(|| {
        let mut values = Vec::new();
        values
            .try_reserve_exact(count)
            .map_err(|_| ModelError("posterior allocation failed"))?;
        unsafe {
            values.extend_from_slice(std::slice::from_raw_parts(input, count));
        }
        StateOccupancyTable::from_values(frames, states, values)
    }) {
        Ok(value) => value,
        Err(status) => return status,
    };
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStateOccupanciesF32 {
                table,
            })))
        })
    }
}

/// Copy complete finite nonnegative weights; complete validation precedes replacement/publication.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_occupancies_replace_f32(
    handle: *mut LiblrhsmmRsStateOccupanciesF32,
    frames: usize,
    states: usize,
    input: *const f32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = shape(frames, states, count).and_then(|()| storage_range(input, count)) {
        return status;
    }
    let table = match boundary(|| {
        let mut values = Vec::new();
        values
            .try_reserve_exact(count)
            .map_err(|_| ModelError("posterior allocation failed"))?;
        unsafe {
            values.extend_from_slice(std::slice::from_raw_parts(input, count));
        }
        StateOccupancyTable::from_values(frames, states, values)
    }) {
        Ok(value) => value,
        Err(status) => return status,
    };
    unsafe {
        (*handle).table = table;
    }
    0
}

/// Read complete time-major dimensions and entry count.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_occupancies_layout_f32(
    handle: *const LiblrhsmmRsStateOccupanciesF32,
    output: *mut LiblrhsmmRsInferenceTableLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsInferenceTableLayout {
                frames: (*handle).table.frames(),
                states: (*handle).table.states(),
                values: (*handle).table.values().len(),
            })
        })
    }
}

/// Deep-copy every optional entry and complete scalar storage.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_occupancies_clone_f32(
    handle: *const LiblrhsmmRsStateOccupanciesF32,
    output: *mut *mut LiblrhsmmRsStateOccupanciesF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || Ok(Box::into_raw(Box::new((*handle).clone())))) }
}

/// Release unique ownership and clear its slot.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_occupancies_release_f32(
    slot: *mut *mut LiblrhsmmRsStateOccupanciesF32,
) -> u32 {
    if let Err(status) = range(slot, 1) {
        return status;
    }
    unsafe {
        let value = slot.read();
        if !value.is_null() {
            if let Err(status) = range(value, 1) {
                return status;
            }
            drop(Box::from_raw(value));
        }
        slot.write(std::ptr::null_mut());
    }
    0
}

/// Read a checked state weight with selected precision retained.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_occupancies_get_f32(
    handle: *const LiblrhsmmRsStateOccupanciesF32,
    frame: usize,
    state: usize,
    output: *mut f32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        let table = &(*handle).table;
        if frame >= table.frames() || state >= table.states() {
            return 2;
        }
        scalar_result(output, || {
            Ok(table.get(frame, state).expect("checked state index"))
        })
    }
}

/// Copy a whole checked interval without exposing internal storage.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_occupancies_copy_values_f32(
    handle: *const LiblrhsmmRsStateOccupanciesF32,
    offset: usize,
    output: *mut f32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { copy_values((*handle).table.values(), offset, output, count) }
}

/// Copy one complete frame after validating its exact count.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_occupancies_copy_frame_f32(
    handle: *const LiblrhsmmRsStateOccupanciesF32,
    frame: usize,
    output: *mut f32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        let table = &(*handle).table;
        if frame >= table.frames() || count != table.states() {
            return 2;
        }
        copy_values(table.values(), frame * table.states(), output, count)
    }
}

/// Run the unchanged native geometric state occupancy calculation.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_scores_geometric_occupancies_f32(
    forward: *const LiblrhsmmRsStateScoresF32,
    backward: *const LiblrhsmmRsStateScoresF32,
    total: f32,
    pruning_slope: f32,
    output: *mut *mut LiblrhsmmRsStateOccupanciesF32,
) -> u32 {
    if let Err(status) = range(forward, 1).and_then(|()| range(backward, 1)) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let table =
                (*forward)
                    .table
                    .geometric_occupancies(&(*backward).table, total, pruning_slope)?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStateOccupanciesF32 {
                table,
            })))
        })
    }
}

/// Run unchanged corrected native HSMM duration posteriors, retaining absent pairs.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_duration_occupancies_f32(
    model: *const LiblrhsmmRsModelF32,
    segmentation: *const LiblrhsmmRsSegmentationF32,
    emissions: *const LiblrhsmmRsLogEmissionsF32,
    forward: *const LiblrhsmmRsHsmmScoresF32,
    backward: *const LiblrhsmmRsHsmmScoresF32,
    total: f32,
    settings: *const LiblrhsmmRsHsmmSettingsF32,
    output: *mut *mut LiblrhsmmRsDurationOccupancies,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(segmentation, 1).and_then(|()| {
            range(emissions, 1).and_then(|()| {
                range(forward, 1)
                    .and_then(|()| range(backward, 1).and_then(|()| range(settings, 1)))
            })
        })
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let table = (*model).parameters.duration_occupancies(
                &(*segmentation).segmentation,
                &(*emissions).table,
                &(*forward).scores,
                &(*backward).scores,
                total,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsDurationOccupancies {
                table,
            })))
        })
    }
}

/// Run unchanged corrected native HSMM duration posteriors, retaining absent pairs.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_duration_occupancies_with_graph_f32(
    model: *const LiblrhsmmRsModelF32,
    graph: *const LiblrhsmmRsTransitionGraphF32,
    emissions: *const LiblrhsmmRsLogEmissionsF32,
    forward: *const LiblrhsmmRsHsmmScoresF32,
    backward: *const LiblrhsmmRsHsmmScoresF32,
    total: f32,
    settings: *const LiblrhsmmRsHsmmSettingsF32,
    output: *mut *mut LiblrhsmmRsDurationOccupancies,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(graph, 1).and_then(|()| {
            range(emissions, 1).and_then(|()| {
                range(forward, 1)
                    .and_then(|()| range(backward, 1).and_then(|()| range(settings, 1)))
            })
        })
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let table = (*model).parameters.duration_occupancies_with_graph(
                (*graph).graph.as_view(),
                &(*emissions).table,
                &(*forward).scores,
                &(*backward).scores,
                total,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsDurationOccupancies {
                table,
            })))
        })
    }
}

/// Run the unchanged native HSMM frame occupancy normalization.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_hsmm_state_occupancies_f32(
    model: *const LiblrhsmmRsModelF32,
    segmentation: *const LiblrhsmmRsSegmentationF32,
    emissions: *const LiblrhsmmRsLogEmissionsF32,
    durations: *const LiblrhsmmRsDurationOccupancies,
    settings: *const LiblrhsmmRsHsmmSettingsF32,
    output: *mut *mut LiblrhsmmRsStateOccupanciesF32,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(segmentation, 1).and_then(|()| {
            range(emissions, 1).and_then(|()| range(durations, 1).and_then(|()| range(settings, 1)))
        })
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let table = (*model).parameters.hsmm_state_occupancies(
                &(*segmentation).segmentation,
                &(*emissions).table,
                &(*durations).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStateOccupanciesF32 {
                table,
            })))
        })
    }
}

/// Run unchanged geometric arrival posteriors with native binary32 slice storage.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_geometric_transition_occupancies_f32(
    model: *const LiblrhsmmRsModelF32,
    segmentation: *const LiblrhsmmRsSegmentationF32,
    emissions: *const LiblrhsmmRsLogEmissionsF32,
    forward: *const LiblrhsmmRsStateScoresF32,
    backward: *const LiblrhsmmRsStateScoresF32,
    total: f32,
    settings: *const LiblrhsmmRsGeometricSettingsF32,
    output: *mut *mut LiblrhsmmRsTransitionOccupancies,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(segmentation, 1).and_then(|()| {
            range(emissions, 1).and_then(|()| {
                range(forward, 1)
                    .and_then(|()| range(backward, 1).and_then(|()| range(settings, 1)))
            })
        })
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let table = (*model).parameters.geometric_transition_occupancies(
                &(*segmentation).segmentation,
                &(*emissions).table,
                &(*forward).table,
                &(*backward).table,
                total,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsTransitionOccupancies {
                table,
            })))
        })
    }
}

/// Run native per-component occupancies with the selected native pruning convention.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_mixture_occupancies_hsmm_f32(
    model: *const LiblrhsmmRsModelF32,
    observation: *const LiblrhsmmRsObservationF32,
    segmentation: *const LiblrhsmmRsSegmentationF32,
    occupancies: *const LiblrhsmmRsStateOccupanciesF32,
    stream: usize,
    temperature: f32,
    state_radius: f32,
    output: *mut *mut LiblrhsmmRsMixtureOccupancies,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(observation, 1)
            .and_then(|()| range(segmentation, 1).and_then(|()| range(occupancies, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let table = (*model).parameters.mixture_occupancies(
                &(*observation).observation,
                &(*segmentation).segmentation,
                &(*occupancies).table,
                stream,
                temperature,
                MixtureOccupancySettings::Hsmm { state_radius },
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsMixtureOccupancies {
                table,
            })))
        })
    }
}

/// Run native per-component occupancies with the selected native pruning convention.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_mixture_occupancies_geometric_f32(
    model: *const LiblrhsmmRsModelF32,
    observation: *const LiblrhsmmRsObservationF32,
    segmentation: *const LiblrhsmmRsSegmentationF32,
    occupancies: *const LiblrhsmmRsStateOccupanciesF32,
    stream: usize,
    temperature: f32,
    pruning_slope: f32,
    output: *mut *mut LiblrhsmmRsMixtureOccupancies,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(observation, 1)
            .and_then(|()| range(segmentation, 1).and_then(|()| range(occupancies, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let table = (*model).parameters.mixture_occupancies(
                &(*observation).observation,
                &(*segmentation).segmentation,
                &(*occupancies).table,
                stream,
                temperature,
                MixtureOccupancySettings::Geometric { pruning_slope },
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsMixtureOccupancies {
                table,
            })))
        })
    }
}

/// Independent time-major state occupancies in the declared precision.
#[derive(Debug, Clone)]
pub struct LiblrhsmmRsStateOccupanciesF64 {
    pub(super) table: StateOccupancyTable<f64>,
}

/// Copy complete finite nonnegative weights; complete validation precedes replacement/publication.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_occupancies_create_f64(
    frames: usize,
    states: usize,
    input: *const f64,
    count: usize,
    output: *mut *mut LiblrhsmmRsStateOccupanciesF64,
) -> u32 {
    if let Err(status) = range(output, 1) {
        return status;
    }
    if let Err(status) = shape(frames, states, count).and_then(|()| storage_range(input, count)) {
        return status;
    }
    let table = match boundary(|| {
        let mut values = Vec::new();
        values
            .try_reserve_exact(count)
            .map_err(|_| ModelError("posterior allocation failed"))?;
        unsafe {
            values.extend_from_slice(std::slice::from_raw_parts(input, count));
        }
        StateOccupancyTable::from_values(frames, states, values)
    }) {
        Ok(value) => value,
        Err(status) => return status,
    };
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStateOccupanciesF64 {
                table,
            })))
        })
    }
}

/// Copy complete finite nonnegative weights; complete validation precedes replacement/publication.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_occupancies_replace_f64(
    handle: *mut LiblrhsmmRsStateOccupanciesF64,
    frames: usize,
    states: usize,
    input: *const f64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = shape(frames, states, count).and_then(|()| storage_range(input, count)) {
        return status;
    }
    let table = match boundary(|| {
        let mut values = Vec::new();
        values
            .try_reserve_exact(count)
            .map_err(|_| ModelError("posterior allocation failed"))?;
        unsafe {
            values.extend_from_slice(std::slice::from_raw_parts(input, count));
        }
        StateOccupancyTable::from_values(frames, states, values)
    }) {
        Ok(value) => value,
        Err(status) => return status,
    };
    unsafe {
        (*handle).table = table;
    }
    0
}

/// Read complete time-major dimensions and entry count.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_occupancies_layout_f64(
    handle: *const LiblrhsmmRsStateOccupanciesF64,
    output: *mut LiblrhsmmRsInferenceTableLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsInferenceTableLayout {
                frames: (*handle).table.frames(),
                states: (*handle).table.states(),
                values: (*handle).table.values().len(),
            })
        })
    }
}

/// Deep-copy every optional entry and complete scalar storage.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_occupancies_clone_f64(
    handle: *const LiblrhsmmRsStateOccupanciesF64,
    output: *mut *mut LiblrhsmmRsStateOccupanciesF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || Ok(Box::into_raw(Box::new((*handle).clone())))) }
}

/// Release unique ownership and clear its slot.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_occupancies_release_f64(
    slot: *mut *mut LiblrhsmmRsStateOccupanciesF64,
) -> u32 {
    if let Err(status) = range(slot, 1) {
        return status;
    }
    unsafe {
        let value = slot.read();
        if !value.is_null() {
            if let Err(status) = range(value, 1) {
                return status;
            }
            drop(Box::from_raw(value));
        }
        slot.write(std::ptr::null_mut());
    }
    0
}

/// Read a checked state weight with selected precision retained.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_occupancies_get_f64(
    handle: *const LiblrhsmmRsStateOccupanciesF64,
    frame: usize,
    state: usize,
    output: *mut f64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        let table = &(*handle).table;
        if frame >= table.frames() || state >= table.states() {
            return 2;
        }
        scalar_result(output, || {
            Ok(table.get(frame, state).expect("checked state index"))
        })
    }
}

/// Copy a whole checked interval without exposing internal storage.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_occupancies_copy_values_f64(
    handle: *const LiblrhsmmRsStateOccupanciesF64,
    offset: usize,
    output: *mut f64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { copy_values((*handle).table.values(), offset, output, count) }
}

/// Copy one complete frame after validating its exact count.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_occupancies_copy_frame_f64(
    handle: *const LiblrhsmmRsStateOccupanciesF64,
    frame: usize,
    output: *mut f64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        let table = &(*handle).table;
        if frame >= table.frames() || count != table.states() {
            return 2;
        }
        copy_values(table.values(), frame * table.states(), output, count)
    }
}

/// Run the unchanged native geometric state occupancy calculation.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_state_scores_geometric_occupancies_f64(
    forward: *const LiblrhsmmRsStateScoresF64,
    backward: *const LiblrhsmmRsStateScoresF64,
    total: f64,
    pruning_slope: f64,
    output: *mut *mut LiblrhsmmRsStateOccupanciesF64,
) -> u32 {
    if let Err(status) = range(forward, 1).and_then(|()| range(backward, 1)) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let table =
                (*forward)
                    .table
                    .geometric_occupancies(&(*backward).table, total, pruning_slope)?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStateOccupanciesF64 {
                table,
            })))
        })
    }
}

/// Run unchanged corrected native HSMM duration posteriors, retaining absent pairs.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_duration_occupancies_f64(
    model: *const LiblrhsmmRsModelF64,
    segmentation: *const LiblrhsmmRsSegmentationF64,
    emissions: *const LiblrhsmmRsLogEmissionsF64,
    forward: *const LiblrhsmmRsHsmmScoresF64,
    backward: *const LiblrhsmmRsHsmmScoresF64,
    total: f64,
    settings: *const LiblrhsmmRsHsmmSettingsF64,
    output: *mut *mut LiblrhsmmRsDurationOccupancies,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(segmentation, 1).and_then(|()| {
            range(emissions, 1).and_then(|()| {
                range(forward, 1)
                    .and_then(|()| range(backward, 1).and_then(|()| range(settings, 1)))
            })
        })
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let table = (*model).parameters.duration_occupancies(
                &(*segmentation).segmentation,
                &(*emissions).table,
                &(*forward).scores,
                &(*backward).scores,
                total,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsDurationOccupancies {
                table,
            })))
        })
    }
}

/// Run unchanged corrected native HSMM duration posteriors, retaining absent pairs.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_duration_occupancies_with_graph_f64(
    model: *const LiblrhsmmRsModelF64,
    graph: *const LiblrhsmmRsTransitionGraphF64,
    emissions: *const LiblrhsmmRsLogEmissionsF64,
    forward: *const LiblrhsmmRsHsmmScoresF64,
    backward: *const LiblrhsmmRsHsmmScoresF64,
    total: f64,
    settings: *const LiblrhsmmRsHsmmSettingsF64,
    output: *mut *mut LiblrhsmmRsDurationOccupancies,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(graph, 1).and_then(|()| {
            range(emissions, 1).and_then(|()| {
                range(forward, 1)
                    .and_then(|()| range(backward, 1).and_then(|()| range(settings, 1)))
            })
        })
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let table = (*model).parameters.duration_occupancies_with_graph(
                (*graph).graph.as_view(),
                &(*emissions).table,
                &(*forward).scores,
                &(*backward).scores,
                total,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsDurationOccupancies {
                table,
            })))
        })
    }
}

/// Run the unchanged native HSMM frame occupancy normalization.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_hsmm_state_occupancies_f64(
    model: *const LiblrhsmmRsModelF64,
    segmentation: *const LiblrhsmmRsSegmentationF64,
    emissions: *const LiblrhsmmRsLogEmissionsF64,
    durations: *const LiblrhsmmRsDurationOccupancies,
    settings: *const LiblrhsmmRsHsmmSettingsF64,
    output: *mut *mut LiblrhsmmRsStateOccupanciesF64,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(segmentation, 1).and_then(|()| {
            range(emissions, 1).and_then(|()| range(durations, 1).and_then(|()| range(settings, 1)))
        })
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let table = (*model).parameters.hsmm_state_occupancies(
                &(*segmentation).segmentation,
                &(*emissions).table,
                &(*durations).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStateOccupanciesF64 {
                table,
            })))
        })
    }
}

/// Run unchanged geometric arrival posteriors with native binary32 slice storage.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_geometric_transition_occupancies_f64(
    model: *const LiblrhsmmRsModelF64,
    segmentation: *const LiblrhsmmRsSegmentationF64,
    emissions: *const LiblrhsmmRsLogEmissionsF64,
    forward: *const LiblrhsmmRsStateScoresF64,
    backward: *const LiblrhsmmRsStateScoresF64,
    total: f64,
    settings: *const LiblrhsmmRsGeometricSettingsF64,
    output: *mut *mut LiblrhsmmRsTransitionOccupancies,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(segmentation, 1).and_then(|()| {
            range(emissions, 1).and_then(|()| {
                range(forward, 1)
                    .and_then(|()| range(backward, 1).and_then(|()| range(settings, 1)))
            })
        })
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let table = (*model).parameters.geometric_transition_occupancies(
                &(*segmentation).segmentation,
                &(*emissions).table,
                &(*forward).table,
                &(*backward).table,
                total,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsTransitionOccupancies {
                table,
            })))
        })
    }
}

/// Run native per-component occupancies with the selected native pruning convention.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_mixture_occupancies_hsmm_f64(
    model: *const LiblrhsmmRsModelF64,
    observation: *const LiblrhsmmRsObservationF64,
    segmentation: *const LiblrhsmmRsSegmentationF64,
    occupancies: *const LiblrhsmmRsStateOccupanciesF64,
    stream: usize,
    temperature: f64,
    state_radius: f64,
    output: *mut *mut LiblrhsmmRsMixtureOccupancies,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(observation, 1)
            .and_then(|()| range(segmentation, 1).and_then(|()| range(occupancies, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let table = (*model).parameters.mixture_occupancies(
                &(*observation).observation,
                &(*segmentation).segmentation,
                &(*occupancies).table,
                stream,
                temperature,
                MixtureOccupancySettings::Hsmm { state_radius },
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsMixtureOccupancies {
                table,
            })))
        })
    }
}

/// Run native per-component occupancies with the selected native pruning convention.
/// # Safety
/// Handles are live owners of the declared type. Descriptors and arrays are aligned,
/// initialized readable storage for stated lengths. Outputs are independent aligned
/// writable storage. Reads cannot overlap mutation or release. Constructor slots
/// contain no live owner; replacement/release require unique access. Returned owners
/// are independent copies. Release clears its initialized slot and accepts null
/// ownership. Empty interval copies permit null destination storage. No internal
/// storage pointer escapes. Optional input tags must be zero or one.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_mixture_occupancies_geometric_f64(
    model: *const LiblrhsmmRsModelF64,
    observation: *const LiblrhsmmRsObservationF64,
    segmentation: *const LiblrhsmmRsSegmentationF64,
    occupancies: *const LiblrhsmmRsStateOccupanciesF64,
    stream: usize,
    temperature: f64,
    pruning_slope: f64,
    output: *mut *mut LiblrhsmmRsMixtureOccupancies,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(observation, 1)
            .and_then(|()| range(segmentation, 1).and_then(|()| range(occupancies, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let table = (*model).parameters.mixture_occupancies(
                &(*observation).observation,
                &(*segmentation).segmentation,
                &(*occupancies).table,
                stream,
                temperature,
                MixtureOccupancySettings::Geometric { pruning_slope },
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsMixtureOccupancies {
                table,
            })))
        })
    }
}
