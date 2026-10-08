use super::super::{LiblrhsmmRsObservationSetF32, LiblrhsmmRsSegmentationSetF32};
use super::*;

/// Complete paired regrouping result; no child borrows escape its lifetime.
pub struct LiblrhsmmRsDatasetF32 {
    dataset: AlignedDataset<f32>,
}

/// Partition every aligned sample, retaining all frames and the final short group.
/// Input sets are borrowed directly; successful output owns both complete result sets.
/// Native regrouping resets outgoing transitions to ordinary next-state jumps.
/// # Safety
/// Owners have the declared type and precision and remain live throughout the
/// call. Outputs are independent aligned writable storage and do not alias any
/// participating owner. Constructors require slots holding no live owner.
/// Release requires exclusive access and transfers unique ownership, clearing
/// the initialized slot. No reads may overlap mutation or release.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_regroup_data_f32(
    observations: *const LiblrhsmmRsObservationSetF32,
    segmentations: *const LiblrhsmmRsSegmentationSetF32,
    segments_per_unit: usize,
    output: *mut *mut LiblrhsmmRsDatasetF32,
) -> u32 {
    if let Err(status) = range(observations, 1) {
        return status;
    }
    if let Err(status) = range(segmentations, 1) {
        return status;
    }
    // SAFETY: Live readable sets and independent writable output. Native validation
    // precedes publication; failures preserve the initialized output slot.
    unsafe {
        scalar_result(output, || {
            let dataset = regroup_with_precision(
                &(*observations).samples,
                &(*segmentations).samples,
                segments_per_unit,
            )?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsDatasetF32 { dataset })))
        })
    }
}

/// Copy the common number of paired result samples.
/// # Safety
/// Owners have the declared type and precision and remain live throughout the
/// call. Outputs are independent aligned writable storage and do not alias any
/// participating owner. Constructors require slots holding no live owner.
/// Release requires exclusive access and transfers unique ownership, clearing
/// the initialized slot. No reads may overlap mutation or release.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_dataset_length_f32(
    handle: *const LiblrhsmmRsDatasetF32,
    output: *mut usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable dataset and independent output.
    unsafe { scalar_result(output, || Ok((*handle).dataset.observations.len())) }
}

/// Clone the complete observations into an independent editable set.
/// # Safety
/// Owners have the declared type and precision and remain live throughout the
/// call. Outputs are independent aligned writable storage and do not alias any
/// participating owner. Constructors require slots holding no live owner.
/// Release requires exclusive access and transfers unique ownership, clearing
/// the initialized slot. No reads may overlap mutation or release.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_dataset_get_observations_f32(
    handle: *const LiblrhsmmRsDatasetF32,
    output: *mut *mut LiblrhsmmRsObservationSetF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable dataset and independent output; no borrowed data escapes.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsObservationSetF32 {
                samples: (*handle).dataset.observations.clone(),
            })))
        })
    }
}

/// Clone the complete segmentations into an independent editable set.
/// # Safety
/// Owners have the declared type and precision and remain live throughout the
/// call. Outputs are independent aligned writable storage and do not alias any
/// participating owner. Constructors require slots holding no live owner.
/// Release requires exclusive access and transfers unique ownership, clearing
/// the initialized slot. No reads may overlap mutation or release.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_dataset_get_segmentations_f32(
    handle: *const LiblrhsmmRsDatasetF32,
    output: *mut *mut LiblrhsmmRsSegmentationSetF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable dataset and independent output; no borrowed data escapes.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsSegmentationSetF32 {
                samples: (*handle).dataset.segmentations.clone(),
            })))
        })
    }
}

/// Deep-copy both complete result sets into an independent paired owner.
/// # Safety
/// Owners have the declared type and precision and remain live throughout the
/// call. Outputs are independent aligned writable storage and do not alias any
/// participating owner. Constructors require slots holding no live owner.
/// Release requires exclusive access and transfers unique ownership, clearing
/// the initialized slot. No reads may overlap mutation or release.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_dataset_clone_f32(
    handle: *const LiblrhsmmRsDatasetF32,
    output: *mut *mut LiblrhsmmRsDatasetF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable dataset and independent output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsDatasetF32 {
                dataset: (*handle).dataset.clone(),
            })))
        })
    }
}

/// Release the paired owner; independently retrieved sets remain valid.
/// # Safety
/// Owners have the declared type and precision and remain live throughout the
/// call. Outputs are independent aligned writable storage and do not alias any
/// participating owner. Constructors require slots holding no live owner.
/// Release requires exclusive access and transfers unique ownership, clearing
/// the initialized slot. No reads may overlap mutation or release.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_dataset_release_f32(
    owner: *mut *mut LiblrhsmmRsDatasetF32,
) -> u32 {
    // SAFETY: Caller transfers the unique owner through an independent writable slot.
    unsafe { release(owner) }
}
