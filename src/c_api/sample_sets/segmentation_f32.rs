use super::super::LiblrhsmmRsSegmentationF32;
use super::*;
/// Independent ordered collection of complete native samples.
pub struct LiblrhsmmRsSegmentationSetF32 {
    pub(in crate::c_api) samples: Vec<SegmentationData<f32>>,
}
/// Incremental ordered slots; missing samples remain distinct from empty samples.
pub struct LiblrhsmmRsSegmentationSetBuilderF32 {
    builder: SampleSetBuilder<SegmentationData<f32>>,
}
impl SampleOwner for LiblrhsmmRsSegmentationF32 {
    type Data = SegmentationData<f32>;
    fn sample(&self) -> &Self::Data {
        &self.segmentation
    }
}
/// Copy complete samples in input order; repeated inputs become independent samples.
/// # Safety
/// Owners have the declared type/precision and remain live for this call. Inputs
/// are initialized readable storage; outputs are independent aligned writable
/// storage. Successful constructors use slots holding no live owner. Mutation
/// and release require exclusive access; no overlapping reads/mutation/release.
/// Child pointers are borrowed only for this call and may repeat; all resulting
/// samples are deep copies. Output and child storage cannot alias participating
/// owners. Empty arrays permit null pointers. Release clears unique owner slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_create_f32(
    input: *const *const LiblrhsmmRsSegmentationF32,
    count: usize,
    output: *mut *mut LiblrhsmmRsSegmentationSetF32,
) -> u32 {
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Independent caller arrays and live child owners.
    let samples = match unsafe { copied_input(input, count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Output storage is independent and writable.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsSegmentationSetF32 {
                samples,
            })))
        })
    }
}

/// Copy the sample count; errors retain the initialized output.
/// # Safety
/// Owners have the declared type/precision and remain live for this call. Inputs
/// are initialized readable storage; outputs are independent aligned writable
/// storage. Successful constructors use slots holding no live owner. Mutation
/// and release require exclusive access; no overlapping reads/mutation/release.
/// Child pointers are borrowed only for this call and may repeat; all resulting
/// samples are deep copies. Output and child storage cannot alias participating
/// owners. Empty arrays permit null pointers. Release clears unique owner slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_length_f32(
    handle: *const LiblrhsmmRsSegmentationSetF32,
    output: *mut usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable owner and independent output.
    unsafe { scalar_result(output, || Ok((*handle).samples.len())) }
}

/// Clone one complete sample into an independent editable owner.
/// # Safety
/// Owners have the declared type/precision and remain live for this call. Inputs
/// are initialized readable storage; outputs are independent aligned writable
/// storage. Successful constructors use slots holding no live owner. Mutation
/// and release require exclusive access; no overlapping reads/mutation/release.
/// Child pointers are borrowed only for this call and may repeat; all resulting
/// samples are deep copies. Output and child storage cannot alias participating
/// owners. Empty arrays permit null pointers. Release clears unique owner slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_get_f32(
    handle: *const LiblrhsmmRsSegmentationSetF32,
    index: usize,
    output: *mut *mut LiblrhsmmRsSegmentationF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Live readable owner; bounds checked before cloning.
    let Some(sample) = (unsafe { &(*handle).samples }).get(index) else {
        return 2;
    };
    // SAFETY: Independent output and readable sample.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsSegmentationF32 {
                segmentation: sample.clone(),
            })))
        })
    }
}

/// Replace one sample with a complete independent copy; errors retain the set.
/// # Safety
/// Owners have the declared type/precision and remain live for this call. Inputs
/// are initialized readable storage; outputs are independent aligned writable
/// storage. Successful constructors use slots holding no live owner. Mutation
/// and release require exclusive access; no overlapping reads/mutation/release.
/// Child pointers are borrowed only for this call and may repeat; all resulting
/// samples are deep copies. Output and child storage cannot alias participating
/// owners. Empty arrays permit null pointers. Release clears unique owner slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_set_f32(
    handle: *mut LiblrhsmmRsSegmentationSetF32,
    index: usize,
    child: *const LiblrhsmmRsSegmentationF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Caller grants exclusive set access.
    if index >= unsafe { (*handle).samples.len() } {
        return 2;
    }
    // SAFETY: Independent readable child owner.
    let sample = match unsafe { copied_child(child) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Checked index, exclusive access; clone is complete before replacement.
    unsafe {
        (&mut (*handle).samples)[index] = sample;
    }
    0
}

/// Append a complete sample; previous samples remain unchanged on errors.
/// # Safety
/// Owners have the declared type/precision and remain live for this call. Inputs
/// are initialized readable storage; outputs are independent aligned writable
/// storage. Successful constructors use slots holding no live owner. Mutation
/// and release require exclusive access; no overlapping reads/mutation/release.
/// Child pointers are borrowed only for this call and may repeat; all resulting
/// samples are deep copies. Output and child storage cannot alias participating
/// owners. Empty arrays permit null pointers. Release clears unique owner slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_append_f32(
    handle: *mut LiblrhsmmRsSegmentationSetF32,
    child: *const LiblrhsmmRsSegmentationF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Caller grants exclusive set access.
    if unsafe { (*handle).samples.len() } >= i32::MAX as usize {
        return 2;
    }
    // SAFETY: Independent readable child owner.
    let sample = match unsafe { copied_child(child) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    match boundary(|| {
        // SAFETY: Exclusive set access; reserve completes before observable mutation.
        let samples = unsafe { &mut (*handle).samples };
        samples
            .try_reserve(1)
            .map_err(|_| ModelError("sample set growth failed"))?;
        samples.push(sample);
        Ok(())
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}

/// Replace the complete ordered set transactionally, including an empty set.
/// # Safety
/// Owners have the declared type/precision and remain live for this call. Inputs
/// are initialized readable storage; outputs are independent aligned writable
/// storage. Successful constructors use slots holding no live owner. Mutation
/// and release require exclusive access; no overlapping reads/mutation/release.
/// Child pointers are borrowed only for this call and may repeat; all resulting
/// samples are deep copies. Output and child storage cannot alias participating
/// owners. Empty arrays permit null pointers. Release clears unique owner slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_replace_f32(
    handle: *mut LiblrhsmmRsSegmentationSetF32,
    input: *const *const LiblrhsmmRsSegmentationF32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Independent arrays and live child owners.
    let samples = match unsafe { copied_input(input, count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Exclusive set access; all replacement samples validated and copied first.
    unsafe {
        (*handle).samples = samples;
    }
    0
}

/// Deep copy every sample and all nested storage.
/// # Safety
/// Owners have the declared type/precision and remain live for this call. Inputs
/// are initialized readable storage; outputs are independent aligned writable
/// storage. Successful constructors use slots holding no live owner. Mutation
/// and release require exclusive access; no overlapping reads/mutation/release.
/// Child pointers are borrowed only for this call and may repeat; all resulting
/// samples are deep copies. Output and child storage cannot alias participating
/// owners. Empty arrays permit null pointers. Release clears unique owner slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_clone_f32(
    handle: *const LiblrhsmmRsSegmentationSetF32,
    output: *mut *mut LiblrhsmmRsSegmentationSetF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Independent output and live readable owner.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsSegmentationSetF32 {
                samples: (*handle).samples.clone(),
            })))
        })
    }
}

/// Release the complete set and clear its owner slot.
/// # Safety
/// Owners have the declared type/precision and remain live for this call. Inputs
/// are initialized readable storage; outputs are independent aligned writable
/// storage. Successful constructors use slots holding no live owner. Mutation
/// and release require exclusive access; no overlapping reads/mutation/release.
/// Child pointers are borrowed only for this call and may repeat; all resulting
/// samples are deep copies. Output and child storage cannot alias participating
/// owners. Empty arrays permit null pointers. Release clears unique owner slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_release_f32(
    handle: *mut *mut LiblrhsmmRsSegmentationSetF32,
) -> u32 {
    // SAFETY: Caller transfers initialized unique ownership.
    unsafe { release(handle) }
}

/// Create ordered unfilled slots, including an empty builder.
/// # Safety
/// Owners have the declared type/precision and remain live for this call. Inputs
/// are initialized readable storage; outputs are independent aligned writable
/// storage. Successful constructors use slots holding no live owner. Mutation
/// and release require exclusive access; no overlapping reads/mutation/release.
/// Child pointers are borrowed only for this call and may repeat; all resulting
/// samples are deep copies. Output and child storage cannot alias participating
/// owners. Empty arrays permit null pointers. Release clears unique owner slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_builder_new_f32(
    count: usize,
    output: *mut *mut LiblrhsmmRsSegmentationSetBuilderF32,
) -> u32 {
    // SAFETY: Independent output; native construction retains missing slots.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(
                LiblrhsmmRsSegmentationSetBuilderF32 {
                    builder: SampleSetBuilder::new(count)?,
                },
            )))
        })
    }
}

/// Copy the number of ordered slots, including missing samples.
/// # Safety
/// Owners have the declared type/precision and remain live for this call. Inputs
/// are initialized readable storage; outputs are independent aligned writable
/// storage. Successful constructors use slots holding no live owner. Mutation
/// and release require exclusive access; no overlapping reads/mutation/release.
/// Child pointers are borrowed only for this call and may repeat; all resulting
/// samples are deep copies. Output and child storage cannot alias participating
/// owners. Empty arrays permit null pointers. Release clears unique owner slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_builder_length_f32(
    handle: *const LiblrhsmmRsSegmentationSetBuilderF32,
    output: *mut usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Readable owner and independent output.
    unsafe { scalar_result(output, || Ok((*handle).builder.samples.len())) }
}

/// Inspect slot presence without discarding unfilled slots.
/// # Safety
/// Owners have the declared type/precision and remain live for this call. Inputs
/// are initialized readable storage; outputs are independent aligned writable
/// storage. Successful constructors use slots holding no live owner. Mutation
/// and release require exclusive access; no overlapping reads/mutation/release.
/// Child pointers are borrowed only for this call and may repeat; all resulting
/// samples are deep copies. Output and child storage cannot alias participating
/// owners. Empty arrays permit null pointers. Release clears unique owner slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_builder_has_sample_f32(
    handle: *const LiblrhsmmRsSegmentationSetBuilderF32,
    index: usize,
    output: *mut u32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable builder; index checked before slot access.
    let Some(slot) = (unsafe { &(*handle).builder.samples }).get(index) else {
        return 2;
    };
    // SAFETY: Independent output; one means present, zero means missing.
    unsafe { scalar_result(output, || Ok(u32::from(slot.is_some()))) }
}

/// Clone a populated slot; missing samples return three, invalid indices two.
/// # Safety
/// Owners have the declared type/precision and remain live for this call. Inputs
/// are initialized readable storage; outputs are independent aligned writable
/// storage. Successful constructors use slots holding no live owner. Mutation
/// and release require exclusive access; no overlapping reads/mutation/release.
/// Child pointers are borrowed only for this call and may repeat; all resulting
/// samples are deep copies. Output and child storage cannot alias participating
/// owners. Empty arrays permit null pointers. Release clears unique owner slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_builder_get_sample_f32(
    handle: *const LiblrhsmmRsSegmentationSetBuilderF32,
    index: usize,
    output: *mut *mut LiblrhsmmRsSegmentationF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Live readable builder; index checked before slot access.
    let Some(slot) = (unsafe { &(*handle).builder.samples }).get(index) else {
        return 2;
    };
    let Some(sample) = slot else {
        return 3;
    };
    // SAFETY: Readable sample and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsSegmentationF32 {
                segmentation: sample.clone(),
            })))
        })
    }
}

/// Populate a slot with an independent complete sample.
/// # Safety
/// Owners have the declared type/precision and remain live for this call. Inputs
/// are initialized readable storage; outputs are independent aligned writable
/// storage. Successful constructors use slots holding no live owner. Mutation
/// and release require exclusive access; no overlapping reads/mutation/release.
/// Child pointers are borrowed only for this call and may repeat; all resulting
/// samples are deep copies. Output and child storage cannot alias participating
/// owners. Empty arrays permit null pointers. Release clears unique owner slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_builder_set_sample_f32(
    handle: *mut LiblrhsmmRsSegmentationSetBuilderF32,
    index: usize,
    child: *const LiblrhsmmRsSegmentationF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Exclusive builder access.
    if index >= unsafe { (*handle).builder.samples.len() } {
        return 2;
    }
    // SAFETY: Independent readable child.
    let sample = match unsafe { copied_child(child) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Checked index; clone completed before slot replacement.
    unsafe {
        (&mut (*handle).builder.samples)[index] = Some(sample);
    }
    0
}

/// Clear one slot, preserving its place in the ordered builder.
/// # Safety
/// Owners have the declared type/precision and remain live for this call. Inputs
/// are initialized readable storage; outputs are independent aligned writable
/// storage. Successful constructors use slots holding no live owner. Mutation
/// and release require exclusive access; no overlapping reads/mutation/release.
/// Child pointers are borrowed only for this call and may repeat; all resulting
/// samples are deep copies. Output and child storage cannot alias participating
/// owners. Empty arrays permit null pointers. Release clears unique owner slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_builder_clear_sample_f32(
    handle: *mut LiblrhsmmRsSegmentationSetBuilderF32,
    index: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Exclusive builder access, checked slot index.
    let Some(slot) = (unsafe { &mut (*handle).builder.samples }).get_mut(index) else {
        return 2;
    };
    *slot = None;
    0
}

/// Resize ordered slots; retain the surviving prefix and append missing slots.
/// # Safety
/// Owners have the declared type/precision and remain live for this call. Inputs
/// are initialized readable storage; outputs are independent aligned writable
/// storage. Successful constructors use slots holding no live owner. Mutation
/// and release require exclusive access; no overlapping reads/mutation/release.
/// Child pointers are borrowed only for this call and may repeat; all resulting
/// samples are deep copies. Output and child storage cannot alias participating
/// owners. Empty arrays permit null pointers. Release clears unique owner slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_builder_resize_f32(
    handle: *mut LiblrhsmmRsSegmentationSetBuilderF32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if count > i32::MAX as usize {
        return 2;
    }
    match boundary(|| {
        // SAFETY: Caller grants exclusive builder access; new slots are missing.
        resize_builder(unsafe { &mut (*handle).builder }, count)
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}

/// Deep copy all slots, retaining the distinction between missing and empty samples.
/// # Safety
/// Owners have the declared type/precision and remain live for this call. Inputs
/// are initialized readable storage; outputs are independent aligned writable
/// storage. Successful constructors use slots holding no live owner. Mutation
/// and release require exclusive access; no overlapping reads/mutation/release.
/// Child pointers are borrowed only for this call and may repeat; all resulting
/// samples are deep copies. Output and child storage cannot alias participating
/// owners. Empty arrays permit null pointers. Release clears unique owner slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_builder_clone_f32(
    handle: *const LiblrhsmmRsSegmentationSetBuilderF32,
    output: *mut *mut LiblrhsmmRsSegmentationSetBuilderF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Independent output and readable complete builder state.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(
                LiblrhsmmRsSegmentationSetBuilderF32 {
                    builder: (*handle).builder.clone(),
                },
            )))
        })
    }
}

/// Build a complete validated set without consuming the builder; missing slots retain output.
/// # Safety
/// Owners have the declared type/precision and remain live for this call. Inputs
/// are initialized readable storage; outputs are independent aligned writable
/// storage. Successful constructors use slots holding no live owner. Mutation
/// and release require exclusive access; no overlapping reads/mutation/release.
/// Child pointers are borrowed only for this call and may repeat; all resulting
/// samples are deep copies. Output and child storage cannot alias participating
/// owners. Empty arrays permit null pointers. Release clears unique owner slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_builder_build_f32(
    handle: *const LiblrhsmmRsSegmentationSetBuilderF32,
    output: *mut *mut LiblrhsmmRsSegmentationSetF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Readable builder and independent output; native build validates every slot.
    unsafe {
        scalar_result(output, || {
            let samples = (*handle).builder.clone().build()?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsSegmentationSetF32 {
                samples,
            })))
        })
    }
}

/// Release all slots and clear the builder owner slot.
/// # Safety
/// Owners have the declared type/precision and remain live for this call. Inputs
/// are initialized readable storage; outputs are independent aligned writable
/// storage. Successful constructors use slots holding no live owner. Mutation
/// and release require exclusive access; no overlapping reads/mutation/release.
/// Child pointers are borrowed only for this call and may repeat; all resulting
/// samples are deep copies. Output and child storage cannot alias participating
/// owners. Empty arrays permit null pointers. Release clears unique owner slots.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_builder_release_f32(
    handle: *mut *mut LiblrhsmmRsSegmentationSetBuilderF32,
) -> u32 {
    // SAFETY: Caller transfers initialized unique ownership.
    unsafe { release(handle) }
}
