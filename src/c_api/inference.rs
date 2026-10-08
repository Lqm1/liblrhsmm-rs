//! Native geometric and explicit-duration inference with independently owned results.
use super::gaussian::storage_range;
use super::inference_tables::copy_values;
use super::*;
use crate::geometric::{GeometricPath, GeometricSettings};
use crate::hsmm::{HsmmPath, HsmmScoreTables, HsmmSettings, StateOccurrence};

/// Complete result dimensions. Occurrences count boundaries for geometric paths.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiblrhsmmRsPathLayout {
    pub frames: usize,
    pub states: usize,
    pub occurrences: usize,
}

unsafe fn copy_occurrences(
    values: &[StateOccurrence],
    offset: usize,
    output: *mut LiblrhsmmRsStateOccurrence,
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
    for (index, value) in values.iter().enumerate() {
        // SAFETY: Whole destination and interval checked before any write; independent storage.
        unsafe {
            output.add(index).write(LiblrhsmmRsStateOccurrence {
                state: value.state,
                end: value.end,
            });
        }
    }
    0
}

/// Complete per-call Hsmm inference configuration.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LiblrhsmmRsHsmmSettingsF32 {
    pub temperature: f32,
    pub duration_weight: f32,
    pub state_radius: f32,
    pub duration_extra: usize,
    pub duration_extra_factor: f32,
}

impl From<LiblrhsmmRsHsmmSettingsF32> for HsmmSettings<f32> {
    fn from(value: LiblrhsmmRsHsmmSettingsF32) -> Self {
        Self {
            temperature: value.temperature,
            duration_weight: value.duration_weight,
            state_radius: value.state_radius,
            duration_extra: value.duration_extra,
            duration_extra_factor: value.duration_extra_factor,
        }
    }
}

impl From<HsmmSettings<f32>> for LiblrhsmmRsHsmmSettingsF32 {
    fn from(value: HsmmSettings<f32>) -> Self {
        Self {
            temperature: value.temperature,
            duration_weight: value.duration_weight,
            state_radius: value.state_radius,
            duration_extra: value.duration_extra,
            duration_extra_factor: value.duration_extra_factor,
        }
    }
}

/// Write the unchanged native default inference settings.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_hsmm_settings_default_f32(
    output: *mut LiblrhsmmRsHsmmSettingsF32,
) -> u32 {
    unsafe { scalar_result(output, || Ok(HsmmSettings::<f32>::default().into())) }
}

/// Complete per-call Geometric inference configuration.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LiblrhsmmRsGeometricSettingsF32 {
    pub temperature: f32,
    pub pruning_slope: f32,
}

impl From<LiblrhsmmRsGeometricSettingsF32> for GeometricSettings<f32> {
    fn from(value: LiblrhsmmRsGeometricSettingsF32) -> Self {
        Self {
            temperature: value.temperature,
            pruning_slope: value.pruning_slope,
        }
    }
}

impl From<GeometricSettings<f32>> for LiblrhsmmRsGeometricSettingsF32 {
    fn from(value: GeometricSettings<f32>) -> Self {
        Self {
            temperature: value.temperature,
            pruning_slope: value.pruning_slope,
        }
    }
}

/// Write the unchanged native default inference settings.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_geometric_settings_default_f32(
    output: *mut LiblrhsmmRsGeometricSettingsF32,
) -> u32 {
    unsafe { scalar_result(output, || Ok(GeometricSettings::<f32>::default().into())) }
}

/// Independent immutable HsmmScores result with complete native storage.
#[derive(Debug, Clone)]
pub struct LiblrhsmmRsHsmmScoresF32 {
    pub(super) scores: HsmmScoreTables<f32>,
}

/// Deep-copy the complete result, including all score and path arrays.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_hsmm_scores_clone_f32(
    handle: *const LiblrhsmmRsHsmmScoresF32,
    output: *mut *mut LiblrhsmmRsHsmmScoresF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || Ok(Box::into_raw(Box::new((*handle).clone())))) }
}

/// Release unique ownership and clear its slot.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_hsmm_scores_release_f32(
    slot: *mut *mut LiblrhsmmRsHsmmScoresF32,
) -> u32 {
    if let Err(status) = range(slot, 1) {
        return status;
    }
    unsafe {
        let handle = slot.read();
        if !handle.is_null() {
            if let Err(status) = range(handle, 1) {
                return status;
            }
            drop(Box::from_raw(handle));
        }
        slot.write(std::ptr::null_mut());
    }
    0
}

/// Copy the complete begin table into an independent score owner.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_hsmm_scores_get_begin_f32(
    handle: *const LiblrhsmmRsHsmmScoresF32,
    output: *mut *mut LiblrhsmmRsStateScoresF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStateScoresF32 {
                table: (*handle).scores.begin.clone(),
            })))
        })
    }
}

/// Copy the complete end table into an independent score owner.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_hsmm_scores_get_end_f32(
    handle: *const LiblrhsmmRsHsmmScoresF32,
    output: *mut *mut LiblrhsmmRsStateScoresF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStateScoresF32 {
                table: (*handle).scores.end.clone(),
            })))
        })
    }
}

/// Copy matching begin/end tables into a complete independent HSMM score result.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_hsmm_scores_create_f32(
    begin: *const LiblrhsmmRsStateScoresF32,
    end: *const LiblrhsmmRsStateScoresF32,
    output: *mut *mut LiblrhsmmRsHsmmScoresF32,
) -> u32 {
    if let Err(status) = range(begin, 1).and_then(|()| range(end, 1)) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let begin = &(*begin).table;
            let end = &(*end).table;
            if begin.frames() != end.frames() || begin.states() != end.states() {
                return Err(ModelError("HSMM score dimensions do not match"));
            }
            Ok(Box::into_raw(Box::new(LiblrhsmmRsHsmmScoresF32 {
                scores: HsmmScoreTables {
                    begin: begin.clone(),
                    end: end.clone(),
                },
            })))
        })
    }
}

/// Independent immutable HsmmPath result with complete native storage.
#[derive(Debug, Clone)]
pub struct LiblrhsmmRsHsmmPathF32 {
    pub(super) path: HsmmPath<f32>,
}

/// Deep-copy the complete result, including all score and path arrays.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_hsmm_path_clone_f32(
    handle: *const LiblrhsmmRsHsmmPathF32,
    output: *mut *mut LiblrhsmmRsHsmmPathF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || Ok(Box::into_raw(Box::new((*handle).clone())))) }
}

/// Release unique ownership and clear its slot.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_hsmm_path_release_f32(
    slot: *mut *mut LiblrhsmmRsHsmmPathF32,
) -> u32 {
    if let Err(status) = range(slot, 1) {
        return status;
    }
    unsafe {
        let handle = slot.read();
        if !handle.is_null() {
            if let Err(status) = range(handle, 1) {
                return status;
            }
            drop(Box::from_raw(handle));
        }
        slot.write(std::ptr::null_mut());
    }
    0
}

/// Read complete score dimensions and occurrence count.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_hsmm_path_layout_f32(
    handle: *const LiblrhsmmRsHsmmPathF32,
    output: *mut LiblrhsmmRsPathLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsPathLayout {
                frames: (*handle).path.scores.begin.frames(),
                states: (*handle).path.scores.begin.states(),
                occurrences: (*handle).path.occurrences.len(),
            })
        })
    }
}

/// Copy the native path likelihood with selected precision retained.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_hsmm_path_log_probability_f32(
    handle: *const LiblrhsmmRsHsmmPathF32,
    output: *mut f32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || Ok((*handle).path.log_probability)) }
}

/// Deep-copy all native path scores into an independent owner.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_hsmm_path_get_scores_f32(
    handle: *const LiblrhsmmRsHsmmPathF32,
    output: *mut *mut LiblrhsmmRsHsmmScoresF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsHsmmScoresF32 {
                scores: (*handle).path.scores.clone(),
            })))
        })
    }
}

/// Copy the whole checked occurrence interval, preserving repeats, skips and boundaries.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_hsmm_path_copy_occurrences_f32(
    handle: *const LiblrhsmmRsHsmmPathF32,
    offset: usize,
    output: *mut LiblrhsmmRsStateOccurrence,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { copy_occurrences(&(*handle).path.occurrences, offset, output, count) }
}

/// Independent immutable GeometricPath result with complete native storage.
#[derive(Debug, Clone)]
pub struct LiblrhsmmRsGeometricPathF32 {
    pub(super) path: GeometricPath<f32>,
}

/// Deep-copy the complete result, including all score and path arrays.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_geometric_path_clone_f32(
    handle: *const LiblrhsmmRsGeometricPathF32,
    output: *mut *mut LiblrhsmmRsGeometricPathF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || Ok(Box::into_raw(Box::new((*handle).clone())))) }
}

/// Release unique ownership and clear its slot.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_geometric_path_release_f32(
    slot: *mut *mut LiblrhsmmRsGeometricPathF32,
) -> u32 {
    if let Err(status) = range(slot, 1) {
        return status;
    }
    unsafe {
        let handle = slot.read();
        if !handle.is_null() {
            if let Err(status) = range(handle, 1) {
                return status;
            }
            drop(Box::from_raw(handle));
        }
        slot.write(std::ptr::null_mut());
    }
    0
}

/// Read complete score dimensions and occurrence count.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_geometric_path_layout_f32(
    handle: *const LiblrhsmmRsGeometricPathF32,
    output: *mut LiblrhsmmRsPathLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsPathLayout {
                frames: (*handle).path.scores.frames(),
                states: (*handle).path.scores.states(),
                occurrences: (*handle).path.boundaries.len(),
            })
        })
    }
}

/// Copy the native path likelihood with selected precision retained.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_geometric_path_log_probability_f32(
    handle: *const LiblrhsmmRsGeometricPathF32,
    output: *mut f32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || Ok((*handle).path.log_probability)) }
}

/// Deep-copy all native path scores into an independent owner.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_geometric_path_get_scores_f32(
    handle: *const LiblrhsmmRsGeometricPathF32,
    output: *mut *mut LiblrhsmmRsStateScoresF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStateScoresF32 {
                table: (*handle).path.scores.clone(),
            })))
        })
    }
}

/// Copy a checked interval of native states.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_geometric_path_copy_states_f32(
    handle: *const LiblrhsmmRsGeometricPathF32,
    offset: usize,
    output: *mut usize,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { copy_values(&(*handle).path.states, offset, output, count) }
}

/// Copy a checked interval of native boundaries.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_geometric_path_copy_boundaries_f32(
    handle: *const LiblrhsmmRsGeometricPathF32,
    offset: usize,
    output: *mut usize,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { copy_values(&(*handle).path.boundaries, offset, output, count) }
}

/// Run unchanged native inference; the returned complete result owns independent storage.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_forward_geometric_f32(
    model: *const LiblrhsmmRsModelF32,
    segmentation: *const LiblrhsmmRsSegmentationF32,
    emissions: *const LiblrhsmmRsLogEmissionsF32,
    settings: *const LiblrhsmmRsGeometricSettingsF32,
    output: *mut *mut LiblrhsmmRsStateScoresF32,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(segmentation, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).parameters.forward_geometric(
                &(*segmentation).segmentation,
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStateScoresF32 {
                table: result,
            })))
        })
    }
}

/// Run unchanged native inference; the returned complete result owns independent storage.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_backward_geometric_f32(
    model: *const LiblrhsmmRsModelF32,
    segmentation: *const LiblrhsmmRsSegmentationF32,
    emissions: *const LiblrhsmmRsLogEmissionsF32,
    settings: *const LiblrhsmmRsGeometricSettingsF32,
    output: *mut *mut LiblrhsmmRsStateScoresF32,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(segmentation, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).parameters.backward_geometric(
                &(*segmentation).segmentation,
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStateScoresF32 {
                table: result,
            })))
        })
    }
}

/// Run unchanged native inference; the returned complete result owns independent storage.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_viterbi_geometric_f32(
    model: *const LiblrhsmmRsModelF32,
    segmentation: *const LiblrhsmmRsSegmentationF32,
    emissions: *const LiblrhsmmRsLogEmissionsF32,
    settings: *const LiblrhsmmRsGeometricSettingsF32,
    output: *mut *mut LiblrhsmmRsGeometricPathF32,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(segmentation, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).parameters.viterbi_geometric(
                &(*segmentation).segmentation,
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGeometricPathF32 {
                path: result,
            })))
        })
    }
}

/// Run unchanged native inference; the returned complete result owns independent storage.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_forward_hsmm_f32(
    model: *const LiblrhsmmRsModelF32,
    segmentation: *const LiblrhsmmRsSegmentationF32,
    emissions: *const LiblrhsmmRsLogEmissionsF32,
    settings: *const LiblrhsmmRsHsmmSettingsF32,
    output: *mut *mut LiblrhsmmRsHsmmScoresF32,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(segmentation, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).parameters.forward_hsmm(
                &(*segmentation).segmentation,
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsHsmmScoresF32 {
                scores: result,
            })))
        })
    }
}

/// Run unchanged native inference; the returned complete result owns independent storage.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_backward_hsmm_f32(
    model: *const LiblrhsmmRsModelF32,
    segmentation: *const LiblrhsmmRsSegmentationF32,
    emissions: *const LiblrhsmmRsLogEmissionsF32,
    settings: *const LiblrhsmmRsHsmmSettingsF32,
    output: *mut *mut LiblrhsmmRsHsmmScoresF32,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(segmentation, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).parameters.backward_hsmm(
                &(*segmentation).segmentation,
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsHsmmScoresF32 {
                scores: result,
            })))
        })
    }
}

/// Run unchanged native inference; the returned complete result owns independent storage.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_viterbi_hsmm_f32(
    model: *const LiblrhsmmRsModelF32,
    segmentation: *const LiblrhsmmRsSegmentationF32,
    emissions: *const LiblrhsmmRsLogEmissionsF32,
    settings: *const LiblrhsmmRsHsmmSettingsF32,
    output: *mut *mut LiblrhsmmRsHsmmPathF32,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(segmentation, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).parameters.viterbi_hsmm(
                &(*segmentation).segmentation,
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsHsmmPathF32 {
                path: result,
            })))
        })
    }
}

/// Run unchanged native inference; the returned complete result owns independent storage. Cached graph and owned model preparations are reused without copying.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_forward_hsmm_with_graph_f32(
    model: *const LiblrhsmmRsModelF32,
    graph: *const LiblrhsmmRsTransitionGraphF32,
    emissions: *const LiblrhsmmRsLogEmissionsF32,
    settings: *const LiblrhsmmRsHsmmSettingsF32,
    output: *mut *mut LiblrhsmmRsHsmmScoresF32,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(graph, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).parameters.forward_hsmm_with_graph(
                (*graph).graph.as_view(),
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsHsmmScoresF32 {
                scores: result,
            })))
        })
    }
}

/// Run unchanged native inference; the returned complete result owns independent storage. Cached graph and owned model preparations are reused without copying.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_backward_hsmm_with_graph_f32(
    model: *const LiblrhsmmRsModelF32,
    graph: *const LiblrhsmmRsTransitionGraphF32,
    emissions: *const LiblrhsmmRsLogEmissionsF32,
    settings: *const LiblrhsmmRsHsmmSettingsF32,
    output: *mut *mut LiblrhsmmRsHsmmScoresF32,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(graph, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).parameters.backward_hsmm_with_graph(
                (*graph).graph.as_view(),
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsHsmmScoresF32 {
                scores: result,
            })))
        })
    }
}

/// Run unchanged native inference; the returned complete result owns independent storage. Cached graph and owned model preparations are reused without copying.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_viterbi_hsmm_with_graph_f32(
    model: *const LiblrhsmmRsModelF32,
    graph: *const LiblrhsmmRsTransitionGraphF32,
    emissions: *const LiblrhsmmRsLogEmissionsF32,
    settings: *const LiblrhsmmRsHsmmSettingsF32,
    output: *mut *mut LiblrhsmmRsHsmmPathF32,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(graph, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).parameters.viterbi_hsmm_with_graph(
                (*graph).graph.as_view(),
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsHsmmPathF32 {
                path: result,
            })))
        })
    }
}

/// Run unchanged native inference; the returned complete result owns independent storage.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_forward_hsmm_f32(
    model: *const LiblrhsmmRsPreparedModelF32,
    segmentation: *const LiblrhsmmRsSegmentationF32,
    emissions: *const LiblrhsmmRsLogEmissionsF32,
    settings: *const LiblrhsmmRsHsmmSettingsF32,
    output: *mut *mut LiblrhsmmRsHsmmScoresF32,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(segmentation, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).prepared.forward_hsmm(
                &(*segmentation).segmentation,
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsHsmmScoresF32 {
                scores: result,
            })))
        })
    }
}

/// Run unchanged native inference; the returned complete result owns independent storage.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_backward_hsmm_f32(
    model: *const LiblrhsmmRsPreparedModelF32,
    segmentation: *const LiblrhsmmRsSegmentationF32,
    emissions: *const LiblrhsmmRsLogEmissionsF32,
    settings: *const LiblrhsmmRsHsmmSettingsF32,
    output: *mut *mut LiblrhsmmRsHsmmScoresF32,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(segmentation, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).prepared.backward_hsmm(
                &(*segmentation).segmentation,
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsHsmmScoresF32 {
                scores: result,
            })))
        })
    }
}

/// Run unchanged native inference; the returned complete result owns independent storage.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_viterbi_hsmm_f32(
    model: *const LiblrhsmmRsPreparedModelF32,
    segmentation: *const LiblrhsmmRsSegmentationF32,
    emissions: *const LiblrhsmmRsLogEmissionsF32,
    settings: *const LiblrhsmmRsHsmmSettingsF32,
    output: *mut *mut LiblrhsmmRsHsmmPathF32,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(segmentation, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).prepared.viterbi_hsmm(
                &(*segmentation).segmentation,
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsHsmmPathF32 {
                path: result,
            })))
        })
    }
}

/// Run unchanged native inference; the returned complete result owns independent storage. Cached graph and owned model preparations are reused without copying.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_forward_hsmm_with_graph_f32(
    model: *const LiblrhsmmRsPreparedModelF32,
    graph: *const LiblrhsmmRsTransitionGraphF32,
    emissions: *const LiblrhsmmRsLogEmissionsF32,
    settings: *const LiblrhsmmRsHsmmSettingsF32,
    output: *mut *mut LiblrhsmmRsHsmmScoresF32,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(graph, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).prepared.forward_hsmm_with_graph(
                (*graph).graph.as_view(),
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsHsmmScoresF32 {
                scores: result,
            })))
        })
    }
}

/// Run unchanged native inference; the returned complete result owns independent storage. Cached graph and owned model preparations are reused without copying.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_backward_hsmm_with_graph_f32(
    model: *const LiblrhsmmRsPreparedModelF32,
    graph: *const LiblrhsmmRsTransitionGraphF32,
    emissions: *const LiblrhsmmRsLogEmissionsF32,
    settings: *const LiblrhsmmRsHsmmSettingsF32,
    output: *mut *mut LiblrhsmmRsHsmmScoresF32,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(graph, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).prepared.backward_hsmm_with_graph(
                (*graph).graph.as_view(),
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsHsmmScoresF32 {
                scores: result,
            })))
        })
    }
}

/// Run unchanged native inference; the returned complete result owns independent storage. Cached graph and owned model preparations are reused without copying.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_viterbi_hsmm_with_graph_f32(
    model: *const LiblrhsmmRsPreparedModelF32,
    graph: *const LiblrhsmmRsTransitionGraphF32,
    emissions: *const LiblrhsmmRsLogEmissionsF32,
    settings: *const LiblrhsmmRsHsmmSettingsF32,
    output: *mut *mut LiblrhsmmRsHsmmPathF32,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(graph, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).prepared.viterbi_hsmm_with_graph(
                (*graph).graph.as_view(),
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsHsmmPathF32 {
                path: result,
            })))
        })
    }
}

/// Complete per-call Hsmm inference configuration.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LiblrhsmmRsHsmmSettingsF64 {
    pub temperature: f64,
    pub duration_weight: f64,
    pub state_radius: f64,
    pub duration_extra: usize,
    pub duration_extra_factor: f64,
}

impl From<LiblrhsmmRsHsmmSettingsF64> for HsmmSettings<f64> {
    fn from(value: LiblrhsmmRsHsmmSettingsF64) -> Self {
        Self {
            temperature: value.temperature,
            duration_weight: value.duration_weight,
            state_radius: value.state_radius,
            duration_extra: value.duration_extra,
            duration_extra_factor: value.duration_extra_factor,
        }
    }
}

impl From<HsmmSettings<f64>> for LiblrhsmmRsHsmmSettingsF64 {
    fn from(value: HsmmSettings<f64>) -> Self {
        Self {
            temperature: value.temperature,
            duration_weight: value.duration_weight,
            state_radius: value.state_radius,
            duration_extra: value.duration_extra,
            duration_extra_factor: value.duration_extra_factor,
        }
    }
}

/// Write the unchanged native default inference settings.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_hsmm_settings_default_f64(
    output: *mut LiblrhsmmRsHsmmSettingsF64,
) -> u32 {
    unsafe { scalar_result(output, || Ok(HsmmSettings::<f64>::default().into())) }
}

/// Complete per-call Geometric inference configuration.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LiblrhsmmRsGeometricSettingsF64 {
    pub temperature: f64,
    pub pruning_slope: f64,
}

impl From<LiblrhsmmRsGeometricSettingsF64> for GeometricSettings<f64> {
    fn from(value: LiblrhsmmRsGeometricSettingsF64) -> Self {
        Self {
            temperature: value.temperature,
            pruning_slope: value.pruning_slope,
        }
    }
}

impl From<GeometricSettings<f64>> for LiblrhsmmRsGeometricSettingsF64 {
    fn from(value: GeometricSettings<f64>) -> Self {
        Self {
            temperature: value.temperature,
            pruning_slope: value.pruning_slope,
        }
    }
}

/// Write the unchanged native default inference settings.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_geometric_settings_default_f64(
    output: *mut LiblrhsmmRsGeometricSettingsF64,
) -> u32 {
    unsafe { scalar_result(output, || Ok(GeometricSettings::<f64>::default().into())) }
}

/// Independent immutable HsmmScores result with complete native storage.
#[derive(Debug, Clone)]
pub struct LiblrhsmmRsHsmmScoresF64 {
    pub(super) scores: HsmmScoreTables<f64>,
}

/// Deep-copy the complete result, including all score and path arrays.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_hsmm_scores_clone_f64(
    handle: *const LiblrhsmmRsHsmmScoresF64,
    output: *mut *mut LiblrhsmmRsHsmmScoresF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || Ok(Box::into_raw(Box::new((*handle).clone())))) }
}

/// Release unique ownership and clear its slot.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_hsmm_scores_release_f64(
    slot: *mut *mut LiblrhsmmRsHsmmScoresF64,
) -> u32 {
    if let Err(status) = range(slot, 1) {
        return status;
    }
    unsafe {
        let handle = slot.read();
        if !handle.is_null() {
            if let Err(status) = range(handle, 1) {
                return status;
            }
            drop(Box::from_raw(handle));
        }
        slot.write(std::ptr::null_mut());
    }
    0
}

/// Copy the complete begin table into an independent score owner.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_hsmm_scores_get_begin_f64(
    handle: *const LiblrhsmmRsHsmmScoresF64,
    output: *mut *mut LiblrhsmmRsStateScoresF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStateScoresF64 {
                table: (*handle).scores.begin.clone(),
            })))
        })
    }
}

/// Copy the complete end table into an independent score owner.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_hsmm_scores_get_end_f64(
    handle: *const LiblrhsmmRsHsmmScoresF64,
    output: *mut *mut LiblrhsmmRsStateScoresF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStateScoresF64 {
                table: (*handle).scores.end.clone(),
            })))
        })
    }
}

/// Copy matching begin/end tables into a complete independent HSMM score result.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_hsmm_scores_create_f64(
    begin: *const LiblrhsmmRsStateScoresF64,
    end: *const LiblrhsmmRsStateScoresF64,
    output: *mut *mut LiblrhsmmRsHsmmScoresF64,
) -> u32 {
    if let Err(status) = range(begin, 1).and_then(|()| range(end, 1)) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let begin = &(*begin).table;
            let end = &(*end).table;
            if begin.frames() != end.frames() || begin.states() != end.states() {
                return Err(ModelError("HSMM score dimensions do not match"));
            }
            Ok(Box::into_raw(Box::new(LiblrhsmmRsHsmmScoresF64 {
                scores: HsmmScoreTables {
                    begin: begin.clone(),
                    end: end.clone(),
                },
            })))
        })
    }
}

/// Independent immutable HsmmPath result with complete native storage.
#[derive(Debug, Clone)]
pub struct LiblrhsmmRsHsmmPathF64 {
    pub(super) path: HsmmPath<f64>,
}

/// Deep-copy the complete result, including all score and path arrays.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_hsmm_path_clone_f64(
    handle: *const LiblrhsmmRsHsmmPathF64,
    output: *mut *mut LiblrhsmmRsHsmmPathF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || Ok(Box::into_raw(Box::new((*handle).clone())))) }
}

/// Release unique ownership and clear its slot.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_hsmm_path_release_f64(
    slot: *mut *mut LiblrhsmmRsHsmmPathF64,
) -> u32 {
    if let Err(status) = range(slot, 1) {
        return status;
    }
    unsafe {
        let handle = slot.read();
        if !handle.is_null() {
            if let Err(status) = range(handle, 1) {
                return status;
            }
            drop(Box::from_raw(handle));
        }
        slot.write(std::ptr::null_mut());
    }
    0
}

/// Read complete score dimensions and occurrence count.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_hsmm_path_layout_f64(
    handle: *const LiblrhsmmRsHsmmPathF64,
    output: *mut LiblrhsmmRsPathLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsPathLayout {
                frames: (*handle).path.scores.begin.frames(),
                states: (*handle).path.scores.begin.states(),
                occurrences: (*handle).path.occurrences.len(),
            })
        })
    }
}

/// Copy the native path likelihood with selected precision retained.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_hsmm_path_log_probability_f64(
    handle: *const LiblrhsmmRsHsmmPathF64,
    output: *mut f64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || Ok((*handle).path.log_probability)) }
}

/// Deep-copy all native path scores into an independent owner.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_hsmm_path_get_scores_f64(
    handle: *const LiblrhsmmRsHsmmPathF64,
    output: *mut *mut LiblrhsmmRsHsmmScoresF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsHsmmScoresF64 {
                scores: (*handle).path.scores.clone(),
            })))
        })
    }
}

/// Copy the whole checked occurrence interval, preserving repeats, skips and boundaries.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_hsmm_path_copy_occurrences_f64(
    handle: *const LiblrhsmmRsHsmmPathF64,
    offset: usize,
    output: *mut LiblrhsmmRsStateOccurrence,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { copy_occurrences(&(*handle).path.occurrences, offset, output, count) }
}

/// Independent immutable GeometricPath result with complete native storage.
#[derive(Debug, Clone)]
pub struct LiblrhsmmRsGeometricPathF64 {
    pub(super) path: GeometricPath<f64>,
}

/// Deep-copy the complete result, including all score and path arrays.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_geometric_path_clone_f64(
    handle: *const LiblrhsmmRsGeometricPathF64,
    output: *mut *mut LiblrhsmmRsGeometricPathF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || Ok(Box::into_raw(Box::new((*handle).clone())))) }
}

/// Release unique ownership and clear its slot.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_geometric_path_release_f64(
    slot: *mut *mut LiblrhsmmRsGeometricPathF64,
) -> u32 {
    if let Err(status) = range(slot, 1) {
        return status;
    }
    unsafe {
        let handle = slot.read();
        if !handle.is_null() {
            if let Err(status) = range(handle, 1) {
                return status;
            }
            drop(Box::from_raw(handle));
        }
        slot.write(std::ptr::null_mut());
    }
    0
}

/// Read complete score dimensions and occurrence count.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_geometric_path_layout_f64(
    handle: *const LiblrhsmmRsGeometricPathF64,
    output: *mut LiblrhsmmRsPathLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsPathLayout {
                frames: (*handle).path.scores.frames(),
                states: (*handle).path.scores.states(),
                occurrences: (*handle).path.boundaries.len(),
            })
        })
    }
}

/// Copy the native path likelihood with selected precision retained.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_geometric_path_log_probability_f64(
    handle: *const LiblrhsmmRsGeometricPathF64,
    output: *mut f64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { scalar_result(output, || Ok((*handle).path.log_probability)) }
}

/// Deep-copy all native path scores into an independent owner.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_geometric_path_get_scores_f64(
    handle: *const LiblrhsmmRsGeometricPathF64,
    output: *mut *mut LiblrhsmmRsStateScoresF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStateScoresF64 {
                table: (*handle).path.scores.clone(),
            })))
        })
    }
}

/// Copy a checked interval of native states.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_geometric_path_copy_states_f64(
    handle: *const LiblrhsmmRsGeometricPathF64,
    offset: usize,
    output: *mut usize,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { copy_values(&(*handle).path.states, offset, output, count) }
}

/// Copy a checked interval of native boundaries.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_geometric_path_copy_boundaries_f64(
    handle: *const LiblrhsmmRsGeometricPathF64,
    offset: usize,
    output: *mut usize,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    unsafe { copy_values(&(*handle).path.boundaries, offset, output, count) }
}

/// Run unchanged native inference; the returned complete result owns independent storage.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_forward_geometric_f64(
    model: *const LiblrhsmmRsModelF64,
    segmentation: *const LiblrhsmmRsSegmentationF64,
    emissions: *const LiblrhsmmRsLogEmissionsF64,
    settings: *const LiblrhsmmRsGeometricSettingsF64,
    output: *mut *mut LiblrhsmmRsStateScoresF64,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(segmentation, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).parameters.forward_geometric(
                &(*segmentation).segmentation,
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStateScoresF64 {
                table: result,
            })))
        })
    }
}

/// Run unchanged native inference; the returned complete result owns independent storage.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_backward_geometric_f64(
    model: *const LiblrhsmmRsModelF64,
    segmentation: *const LiblrhsmmRsSegmentationF64,
    emissions: *const LiblrhsmmRsLogEmissionsF64,
    settings: *const LiblrhsmmRsGeometricSettingsF64,
    output: *mut *mut LiblrhsmmRsStateScoresF64,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(segmentation, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).parameters.backward_geometric(
                &(*segmentation).segmentation,
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStateScoresF64 {
                table: result,
            })))
        })
    }
}

/// Run unchanged native inference; the returned complete result owns independent storage.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_viterbi_geometric_f64(
    model: *const LiblrhsmmRsModelF64,
    segmentation: *const LiblrhsmmRsSegmentationF64,
    emissions: *const LiblrhsmmRsLogEmissionsF64,
    settings: *const LiblrhsmmRsGeometricSettingsF64,
    output: *mut *mut LiblrhsmmRsGeometricPathF64,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(segmentation, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).parameters.viterbi_geometric(
                &(*segmentation).segmentation,
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGeometricPathF64 {
                path: result,
            })))
        })
    }
}

/// Run unchanged native inference; the returned complete result owns independent storage.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_forward_hsmm_f64(
    model: *const LiblrhsmmRsModelF64,
    segmentation: *const LiblrhsmmRsSegmentationF64,
    emissions: *const LiblrhsmmRsLogEmissionsF64,
    settings: *const LiblrhsmmRsHsmmSettingsF64,
    output: *mut *mut LiblrhsmmRsHsmmScoresF64,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(segmentation, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).parameters.forward_hsmm(
                &(*segmentation).segmentation,
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsHsmmScoresF64 {
                scores: result,
            })))
        })
    }
}

/// Run unchanged native inference; the returned complete result owns independent storage.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_backward_hsmm_f64(
    model: *const LiblrhsmmRsModelF64,
    segmentation: *const LiblrhsmmRsSegmentationF64,
    emissions: *const LiblrhsmmRsLogEmissionsF64,
    settings: *const LiblrhsmmRsHsmmSettingsF64,
    output: *mut *mut LiblrhsmmRsHsmmScoresF64,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(segmentation, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).parameters.backward_hsmm(
                &(*segmentation).segmentation,
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsHsmmScoresF64 {
                scores: result,
            })))
        })
    }
}

/// Run unchanged native inference; the returned complete result owns independent storage.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_viterbi_hsmm_f64(
    model: *const LiblrhsmmRsModelF64,
    segmentation: *const LiblrhsmmRsSegmentationF64,
    emissions: *const LiblrhsmmRsLogEmissionsF64,
    settings: *const LiblrhsmmRsHsmmSettingsF64,
    output: *mut *mut LiblrhsmmRsHsmmPathF64,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(segmentation, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).parameters.viterbi_hsmm(
                &(*segmentation).segmentation,
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsHsmmPathF64 {
                path: result,
            })))
        })
    }
}

/// Run unchanged native inference; the returned complete result owns independent storage. Cached graph and owned model preparations are reused without copying.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_forward_hsmm_with_graph_f64(
    model: *const LiblrhsmmRsModelF64,
    graph: *const LiblrhsmmRsTransitionGraphF64,
    emissions: *const LiblrhsmmRsLogEmissionsF64,
    settings: *const LiblrhsmmRsHsmmSettingsF64,
    output: *mut *mut LiblrhsmmRsHsmmScoresF64,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(graph, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).parameters.forward_hsmm_with_graph(
                (*graph).graph.as_view(),
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsHsmmScoresF64 {
                scores: result,
            })))
        })
    }
}

/// Run unchanged native inference; the returned complete result owns independent storage. Cached graph and owned model preparations are reused without copying.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_backward_hsmm_with_graph_f64(
    model: *const LiblrhsmmRsModelF64,
    graph: *const LiblrhsmmRsTransitionGraphF64,
    emissions: *const LiblrhsmmRsLogEmissionsF64,
    settings: *const LiblrhsmmRsHsmmSettingsF64,
    output: *mut *mut LiblrhsmmRsHsmmScoresF64,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(graph, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).parameters.backward_hsmm_with_graph(
                (*graph).graph.as_view(),
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsHsmmScoresF64 {
                scores: result,
            })))
        })
    }
}

/// Run unchanged native inference; the returned complete result owns independent storage. Cached graph and owned model preparations are reused without copying.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_viterbi_hsmm_with_graph_f64(
    model: *const LiblrhsmmRsModelF64,
    graph: *const LiblrhsmmRsTransitionGraphF64,
    emissions: *const LiblrhsmmRsLogEmissionsF64,
    settings: *const LiblrhsmmRsHsmmSettingsF64,
    output: *mut *mut LiblrhsmmRsHsmmPathF64,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(graph, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).parameters.viterbi_hsmm_with_graph(
                (*graph).graph.as_view(),
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsHsmmPathF64 {
                path: result,
            })))
        })
    }
}

/// Run unchanged native inference; the returned complete result owns independent storage.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_forward_hsmm_f64(
    model: *const LiblrhsmmRsPreparedModelF64,
    segmentation: *const LiblrhsmmRsSegmentationF64,
    emissions: *const LiblrhsmmRsLogEmissionsF64,
    settings: *const LiblrhsmmRsHsmmSettingsF64,
    output: *mut *mut LiblrhsmmRsHsmmScoresF64,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(segmentation, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).prepared.forward_hsmm(
                &(*segmentation).segmentation,
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsHsmmScoresF64 {
                scores: result,
            })))
        })
    }
}

/// Run unchanged native inference; the returned complete result owns independent storage.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_backward_hsmm_f64(
    model: *const LiblrhsmmRsPreparedModelF64,
    segmentation: *const LiblrhsmmRsSegmentationF64,
    emissions: *const LiblrhsmmRsLogEmissionsF64,
    settings: *const LiblrhsmmRsHsmmSettingsF64,
    output: *mut *mut LiblrhsmmRsHsmmScoresF64,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(segmentation, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).prepared.backward_hsmm(
                &(*segmentation).segmentation,
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsHsmmScoresF64 {
                scores: result,
            })))
        })
    }
}

/// Run unchanged native inference; the returned complete result owns independent storage.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_viterbi_hsmm_f64(
    model: *const LiblrhsmmRsPreparedModelF64,
    segmentation: *const LiblrhsmmRsSegmentationF64,
    emissions: *const LiblrhsmmRsLogEmissionsF64,
    settings: *const LiblrhsmmRsHsmmSettingsF64,
    output: *mut *mut LiblrhsmmRsHsmmPathF64,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(segmentation, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).prepared.viterbi_hsmm(
                &(*segmentation).segmentation,
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsHsmmPathF64 {
                path: result,
            })))
        })
    }
}

/// Run unchanged native inference; the returned complete result owns independent storage. Cached graph and owned model preparations are reused without copying.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_forward_hsmm_with_graph_f64(
    model: *const LiblrhsmmRsPreparedModelF64,
    graph: *const LiblrhsmmRsTransitionGraphF64,
    emissions: *const LiblrhsmmRsLogEmissionsF64,
    settings: *const LiblrhsmmRsHsmmSettingsF64,
    output: *mut *mut LiblrhsmmRsHsmmScoresF64,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(graph, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).prepared.forward_hsmm_with_graph(
                (*graph).graph.as_view(),
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsHsmmScoresF64 {
                scores: result,
            })))
        })
    }
}

/// Run unchanged native inference; the returned complete result owns independent storage. Cached graph and owned model preparations are reused without copying.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_backward_hsmm_with_graph_f64(
    model: *const LiblrhsmmRsPreparedModelF64,
    graph: *const LiblrhsmmRsTransitionGraphF64,
    emissions: *const LiblrhsmmRsLogEmissionsF64,
    settings: *const LiblrhsmmRsHsmmSettingsF64,
    output: *mut *mut LiblrhsmmRsHsmmScoresF64,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(graph, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).prepared.backward_hsmm_with_graph(
                (*graph).graph.as_view(),
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsHsmmScoresF64 {
                scores: result,
            })))
        })
    }
}

/// Run unchanged native inference; the returned complete result owns independent storage. Cached graph and owned model preparations are reused without copying.
/// # Safety
/// Inputs are live owners of the declared precision and aligned initialized readable
/// descriptors. Outputs are independent aligned writable storage for stated lengths.
/// No concurrent mutation or release overlaps reads. Constructor slots contain no
/// live owner. Returned owners own independent storage and must be released once.
/// Release consumes unique ownership and clears its initialized slot; null ownership
/// is a release no-op. Empty interval copies permit null destination storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_prepared_model_viterbi_hsmm_with_graph_f64(
    model: *const LiblrhsmmRsPreparedModelF64,
    graph: *const LiblrhsmmRsTransitionGraphF64,
    emissions: *const LiblrhsmmRsLogEmissionsF64,
    settings: *const LiblrhsmmRsHsmmSettingsF64,
    output: *mut *mut LiblrhsmmRsHsmmPathF64,
) -> u32 {
    if let Err(status) = range(model, 1).and_then(|()| {
        range(graph, 1).and_then(|()| range(emissions, 1).and_then(|()| range(settings, 1)))
    }) {
        return status;
    }
    unsafe {
        scalar_result(output, || {
            let result = (*model).prepared.viterbi_hsmm_with_graph(
                (*graph).graph.as_view(),
                &(*emissions).table,
                settings.read().into(),
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsHsmmPathF64 {
                path: result,
            })))
        })
    }
}
