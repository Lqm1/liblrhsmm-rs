//! Complete editable segmentations with copied integer fields and outgoing jumps.
use super::gaussian::storage_range;
use super::{boundary, range, scalar_result};
use crate::data::{JumpParameters, SegmentationData};
use crate::{ModelError, StateOccurrence};

/// Complete stream and segment counts, including empty segmentations.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiblrhsmmRsSegmentationLayout {
    pub streams: usize,
    pub segments: usize,
}
/// One state occurrence with its exclusive right frame boundary.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiblrhsmmRsStateOccurrence {
    pub state: usize,
    pub end: usize,
}

fn shape(streams: usize, segments: usize) -> Result<(), u32> {
    if streams > i32::MAX as usize
        || segments > i32::MAX as usize
        || streams > isize::MAX as usize / size_of::<Vec<i32>>()
        || segments > isize::MAX as usize / size_of::<Vec<i32>>()
    {
        return Err(2);
    }
    Ok(())
}
unsafe fn copied_integers(input: *const i32, count: usize) -> Result<Vec<i32>, u32> {
    storage_range(input, count)?;
    boundary(|| {
        let mut values = Vec::new();
        values
            .try_reserve_exact(count)
            .map_err(|_| ModelError("segmentation allocation failed"))?;
        if count != 0 {
            // SAFETY: Caller supplies independent initialized readable scalar storage.
            values.extend_from_slice(unsafe { std::slice::from_raw_parts(input, count) });
        }
        Ok(values)
    })
}
unsafe fn copy_integers(values: &[i32], offset: usize, output: *mut i32, count: usize) -> u32 {
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
        // SAFETY: Caller supplies independent initialized readable/writable buffers.
        unsafe {
            std::ptr::copy_nonoverlapping(values.as_ptr(), output, count);
        }
    }
    0
}
unsafe fn set_integers(values: &mut [i32], offset: usize, input: *const i32, count: usize) -> u32 {
    if let Err(status) = storage_range(input, count) {
        return status;
    }
    let Some(end) = offset.checked_add(count) else {
        return 2;
    };
    let Some(values) = values.get_mut(offset..end) else {
        return 2;
    };
    if count != 0 {
        // SAFETY: Caller supplies independent initialized readable/writable buffers.
        unsafe {
            std::ptr::copy_nonoverlapping(input, values.as_mut_ptr(), count);
        }
    }
    0
}

/// Complete jump delta and selected-precision probability; stored bits are retained.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LiblrhsmmRsJumpF32 {
    pub delta: i32,
    pub probability: f32,
}
/// Synchronous borrowed outgoing jump group; no pointer is retained.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LiblrhsmmRsJumpSliceF32 {
    pub jumps: *const LiblrhsmmRsJumpF32,
    pub count: usize,
}
/// Independently owned editable segmentation parameters.
pub struct LiblrhsmmRsSegmentationF32 {
    pub(super) segmentation: SegmentationData<f32>,
}

unsafe fn jumps_f32(
    input: *const LiblrhsmmRsJumpF32,
    count: usize,
) -> Result<Vec<JumpParameters<f32>>, u32> {
    storage_range(input, count)?;
    boundary(|| {
        let mut values = Vec::new();
        values
            .try_reserve_exact(count)
            .map_err(|_| ModelError("jump allocation failed"))?;
        let input = if count == 0 {
            &[]
        } else {
            // SAFETY: Independent initialized readable jump array supplied by caller.
            unsafe { std::slice::from_raw_parts(input, count) }
        };
        for value in input {
            values.push(JumpParameters {
                delta: value.delta,
                probability: value.probability,
            });
        }
        Ok(values)
    })
}
unsafe fn input_f32(
    streams: usize,
    segments: usize,
    boundaries: *const i32,
    duration_states: *const i32,
    output_states: *const *const i32,
    groups: *const LiblrhsmmRsJumpSliceF32,
) -> Result<SegmentationData<f32>, u32> {
    shape(streams, segments)?;
    storage_range(boundaries, segments)?;
    storage_range(duration_states, segments)?;
    range(output_states, streams)?;
    range(groups, segments)?;
    let groups = if segments == 0 {
        &[]
    } else {
        // SAFETY: Initialized readable group descriptors supplied for every segment.
        unsafe { std::slice::from_raw_parts(groups, segments) }
    };
    for group in groups {
        storage_range(group.jumps, group.count)?;
    }
    let rows = if streams == 0 {
        &[]
    } else {
        // SAFETY: Initialized readable stream-wise state pointers.
        unsafe { std::slice::from_raw_parts(output_states, streams) }
    };
    for &row in rows {
        storage_range(row, segments)?;
    }
    let boundaries = unsafe { copied_integers(boundaries, segments) }?;
    let duration_states = unsafe { copied_integers(duration_states, segments) }?;
    let mut output = boundary(|| {
        let mut values = Vec::new();
        values
            .try_reserve_exact(streams)
            .map_err(|_| ModelError("state matrix allocation failed"))?;
        Ok(values)
    })?;

    for &row in rows {
        output.push(unsafe { copied_integers(row, segments) }?);
    }
    let mut outgoing = boundary(|| {
        let mut values = Vec::new();
        values
            .try_reserve_exact(segments)
            .map_err(|_| ModelError("jump group allocation failed"))?;
        Ok(values)
    })?;
    for group in groups {
        outgoing.push(unsafe { jumps_f32(group.jumps, group.count) }?);
    }
    let segmentation = SegmentationData {
        boundaries,
        duration_states,
        output_states: output,
        outgoing,
    };
    match segmentation.validate() {
        Ok(()) => Ok(segmentation),
        Err(_) => Err(3),
    }
}

/// Construct native zeroed state arrays and ordinary next-state jumps.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_new_f32(
    streams: usize,
    segments: usize,
    output: *mut *mut LiblrhsmmRsSegmentationF32,
) -> u32 {
    if let Err(status) = shape(streams, segments) {
        return status;
    }
    // SAFETY: Independent writable ownership slot supplied by caller.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsSegmentationF32 {
                segmentation: SegmentationData::new(streams, segments),
            })))
        })
    }
}

/// Copy every field from stream-major state arrays and ragged outgoing groups.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_create_f32(
    streams: usize,
    segments: usize,
    boundaries: *const i32,
    duration_states: *const i32,
    output_states: *const *const i32,
    outgoing: *const LiblrhsmmRsJumpSliceF32,
    output: *mut *mut LiblrhsmmRsSegmentationF32,
) -> u32 {
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Independent readable arrays and writable ownership output.
    let segmentation = match unsafe {
        input_f32(
            streams,
            segments,
            boundaries,
            duration_states,
            output_states,
            outgoing,
        )
    } {
        Ok(value) => value,
        Err(status) => return status,
    };
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsSegmentationF32 {
                segmentation,
            })))
        })
    }
}

/// Stage a complete validated copy before replacing any existing field.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_replace_f32(
    handle: *mut LiblrhsmmRsSegmentationF32,
    streams: usize,
    segments: usize,
    boundaries: *const i32,
    duration_states: *const i32,
    output_states: *const *const i32,
    outgoing: *const LiblrhsmmRsJumpSliceF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Exclusive live owner and independent readable inputs.
    let segmentation = match unsafe {
        input_f32(
            streams,
            segments,
            boundaries,
            duration_states,
            output_states,
            outgoing,
        )
    } {
        Ok(value) => value,
        Err(status) => return status,
    };
    unsafe {
        (*handle).segmentation = segmentation;
    }
    0
}

/// Read complete stream and segment counts.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_layout_f32(
    handle: *const LiblrhsmmRsSegmentationF32,
    output: *mut LiblrhsmmRsSegmentationLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable owner and independent writable descriptor.
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsSegmentationLayout {
                streams: (*handle).segmentation.output_states.len(),
                segments: (*handle).segmentation.boundaries.len(),
            })
        })
    }
}

/// Copy a checked integer range without exposing internal storage.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_copy_boundaries_f32(
    handle: *const LiblrhsmmRsSegmentationF32,
    offset: usize,
    output: *mut i32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live owner and independent scalar buffers.
    let segmentation = unsafe { &(*handle).segmentation };
    let values = &segmentation.boundaries;
    unsafe { copy_integers(values, offset, output, count) }
}

/// Update a checked integer range atomically.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_boundaries_f32(
    handle: *mut LiblrhsmmRsSegmentationF32,
    offset: usize,
    input: *const i32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live owner and independent scalar buffers.
    let segmentation = unsafe { &mut (*handle).segmentation };
    let values = &mut segmentation.boundaries;
    unsafe { set_integers(values, offset, input, count) }
}

/// Copy a checked integer range without exposing internal storage.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_copy_duration_states_f32(
    handle: *const LiblrhsmmRsSegmentationF32,
    offset: usize,
    output: *mut i32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live owner and independent scalar buffers.
    let segmentation = unsafe { &(*handle).segmentation };
    let values = &segmentation.duration_states;
    unsafe { copy_integers(values, offset, output, count) }
}

/// Update a checked integer range atomically.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_duration_states_f32(
    handle: *mut LiblrhsmmRsSegmentationF32,
    offset: usize,
    input: *const i32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live owner and independent scalar buffers.
    let segmentation = unsafe { &mut (*handle).segmentation };
    let values = &mut segmentation.duration_states;
    unsafe { set_integers(values, offset, input, count) }
}

/// Copy a checked integer range without exposing internal storage.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_copy_output_states_f32(
    handle: *const LiblrhsmmRsSegmentationF32,
    stream: usize,
    offset: usize,
    output: *mut i32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live owner and independent scalar buffers.
    let segmentation = unsafe { &(*handle).segmentation };
    let Some(values) = segmentation.output_states.get(stream) else {
        return 2;
    };
    unsafe { copy_integers(values, offset, output, count) }
}

/// Update a checked integer range atomically.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_output_states_f32(
    handle: *mut LiblrhsmmRsSegmentationF32,
    stream: usize,
    offset: usize,
    input: *const i32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live owner and independent scalar buffers.
    let segmentation = unsafe { &mut (*handle).segmentation };
    let Some(values) = segmentation.output_states.get_mut(stream) else {
        return 2;
    };
    unsafe { set_integers(values, offset, input, count) }
}

/// Read the complete number of outgoing jumps for one segment.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_outgoing_count_f32(
    handle: *const LiblrhsmmRsSegmentationF32,
    segment: usize,
    output: *mut usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable owner and independent writable scalar result.
    let segmentation = unsafe { &(*handle).segmentation };
    let Some(group) = segmentation.outgoing.get(segment) else {
        return 2;
    };
    unsafe { scalar_result(output, || Ok(group.len())) }
}

/// Copy complete jump delta/probability fields in original group order.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_copy_outgoing_f32(
    handle: *const LiblrhsmmRsSegmentationF32,
    segment: usize,
    offset: usize,
    output: *mut LiblrhsmmRsJumpF32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1).and_then(|()| storage_range(output, count)) {
        return status;
    }
    // SAFETY: Live readable owner and independent writable jump array.
    let segmentation = unsafe { &(*handle).segmentation };
    let Some(group) = segmentation.outgoing.get(segment) else {
        return 2;
    };
    let Some(end) = offset.checked_add(count) else {
        return 2;
    };
    let Some(values) = group.get(offset..end) else {
        return 2;
    };
    for (index, value) in values.iter().enumerate() {
        unsafe {
            output.add(index).write(LiblrhsmmRsJumpF32 {
                delta: value.delta,
                probability: value.probability,
            });
        }
    }
    0
}

/// Replace one copied outgoing group, requiring one final delta-1 sentinel.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_outgoing_f32(
    handle: *mut LiblrhsmmRsSegmentationF32,
    segment: usize,
    input: *const LiblrhsmmRsJumpF32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live exclusive owner and independent readable jump array.
    if segment >= unsafe { (*handle).segmentation.outgoing.len() } {
        return 2;
    }
    let values = match unsafe { jumps_f32(input, count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    if crate::data::validate_jump_group(&values).is_err() {
        return 3;
    }
    unsafe {
        (&mut (*handle).segmentation.outgoing)[segment] = values;
    }
    0
}

/// Delegate native jump insertion or ordinary-edge weight replacement.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_add_jump_f32(
    handle: *mut LiblrhsmmRsSegmentationF32,
    segment: usize,
    delta: i32,
    probability: f32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live exclusive owner follows the native mutation contract.
    match boundary(|| unsafe { (*handle).segmentation.add_jump(segment, delta, probability) }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}

/// Create an independent native resegmentation from checked state occurrences.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_resegment_f32(
    handle: *const LiblrhsmmRsSegmentationF32,
    occurrences: *const LiblrhsmmRsStateOccurrence,
    count: usize,
    output: *mut *mut LiblrhsmmRsSegmentationF32,
) -> u32 {
    if let Err(status) = range(handle, 1).and_then(|()| range(occurrences, count)) {
        return status;
    }
    // SAFETY: Live readable owner, borrowed occurrences and independent output slot.
    unsafe {
        scalar_result(output, || {
            let input = if count == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(occurrences, count)
            };
            let mut path = Vec::new();
            path.try_reserve_exact(count)
                .map_err(|_| ModelError("state occurrence allocation failed"))?;
            for value in input {
                path.push(StateOccurrence {
                    state: value.state,
                    end: value.end,
                });
            }
            Ok(Box::into_raw(Box::new(LiblrhsmmRsSegmentationF32 {
                segmentation: (*handle).segmentation.resegment(&path)?,
            })))
        })
    }
}

/// Deep-copy all integer fields and outgoing probability bits.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_clone_f32(
    handle: *const LiblrhsmmRsSegmentationF32,
    output: *mut *mut LiblrhsmmRsSegmentationF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable owner and independent writable ownership slot.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsSegmentationF32 {
                segmentation: (*handle).segmentation.clone(),
            })))
        })
    }
}

/// Release unique ownership and clear its initialized slot; null owners are accepted.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_release_f32(
    handle: *mut *mut LiblrhsmmRsSegmentationF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Initialized exclusive ownership slot.
    let owner = unsafe { *handle };
    if owner.is_null() {
        return 0;
    }
    if let Err(status) = range(owner, 1) {
        return status;
    }
    unsafe {
        drop(Box::from_raw(owner));
        handle.write(std::ptr::null_mut());
    }
    0
}

/// Complete jump delta and selected-precision probability; stored bits are retained.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LiblrhsmmRsJumpF64 {
    pub delta: i32,
    pub probability: f64,
}
/// Synchronous borrowed outgoing jump group; no pointer is retained.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LiblrhsmmRsJumpSliceF64 {
    pub jumps: *const LiblrhsmmRsJumpF64,
    pub count: usize,
}
/// Independently owned editable segmentation parameters.
pub struct LiblrhsmmRsSegmentationF64 {
    pub(super) segmentation: SegmentationData<f64>,
}

unsafe fn jumps_f64(
    input: *const LiblrhsmmRsJumpF64,
    count: usize,
) -> Result<Vec<JumpParameters<f64>>, u32> {
    storage_range(input, count)?;
    boundary(|| {
        let mut values = Vec::new();
        values
            .try_reserve_exact(count)
            .map_err(|_| ModelError("jump allocation failed"))?;
        let input = if count == 0 {
            &[]
        } else {
            // SAFETY: Independent initialized readable jump array supplied by caller.
            unsafe { std::slice::from_raw_parts(input, count) }
        };
        for value in input {
            values.push(JumpParameters {
                delta: value.delta,
                probability: value.probability,
            });
        }
        Ok(values)
    })
}
unsafe fn input_f64(
    streams: usize,
    segments: usize,
    boundaries: *const i32,
    duration_states: *const i32,
    output_states: *const *const i32,
    groups: *const LiblrhsmmRsJumpSliceF64,
) -> Result<SegmentationData<f64>, u32> {
    shape(streams, segments)?;
    storage_range(boundaries, segments)?;
    storage_range(duration_states, segments)?;
    range(output_states, streams)?;
    range(groups, segments)?;
    let groups = if segments == 0 {
        &[]
    } else {
        // SAFETY: Initialized readable group descriptors supplied for every segment.
        unsafe { std::slice::from_raw_parts(groups, segments) }
    };
    for group in groups {
        storage_range(group.jumps, group.count)?;
    }
    let rows = if streams == 0 {
        &[]
    } else {
        // SAFETY: Initialized readable stream-wise state pointers.
        unsafe { std::slice::from_raw_parts(output_states, streams) }
    };
    for &row in rows {
        storage_range(row, segments)?;
    }
    let boundaries = unsafe { copied_integers(boundaries, segments) }?;
    let duration_states = unsafe { copied_integers(duration_states, segments) }?;
    let mut output = boundary(|| {
        let mut values = Vec::new();
        values
            .try_reserve_exact(streams)
            .map_err(|_| ModelError("state matrix allocation failed"))?;
        Ok(values)
    })?;

    for &row in rows {
        output.push(unsafe { copied_integers(row, segments) }?);
    }
    let mut outgoing = boundary(|| {
        let mut values = Vec::new();
        values
            .try_reserve_exact(segments)
            .map_err(|_| ModelError("jump group allocation failed"))?;
        Ok(values)
    })?;
    for group in groups {
        outgoing.push(unsafe { jumps_f64(group.jumps, group.count) }?);
    }
    let segmentation = SegmentationData {
        boundaries,
        duration_states,
        output_states: output,
        outgoing,
    };
    match segmentation.validate() {
        Ok(()) => Ok(segmentation),
        Err(_) => Err(3),
    }
}

/// Construct native zeroed state arrays and ordinary next-state jumps.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_new_f64(
    streams: usize,
    segments: usize,
    output: *mut *mut LiblrhsmmRsSegmentationF64,
) -> u32 {
    if let Err(status) = shape(streams, segments) {
        return status;
    }
    // SAFETY: Independent writable ownership slot supplied by caller.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsSegmentationF64 {
                segmentation: SegmentationData::new(streams, segments),
            })))
        })
    }
}

/// Copy every field from stream-major state arrays and ragged outgoing groups.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_create_f64(
    streams: usize,
    segments: usize,
    boundaries: *const i32,
    duration_states: *const i32,
    output_states: *const *const i32,
    outgoing: *const LiblrhsmmRsJumpSliceF64,
    output: *mut *mut LiblrhsmmRsSegmentationF64,
) -> u32 {
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Independent readable arrays and writable ownership output.
    let segmentation = match unsafe {
        input_f64(
            streams,
            segments,
            boundaries,
            duration_states,
            output_states,
            outgoing,
        )
    } {
        Ok(value) => value,
        Err(status) => return status,
    };
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsSegmentationF64 {
                segmentation,
            })))
        })
    }
}

/// Stage a complete validated copy before replacing any existing field.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_replace_f64(
    handle: *mut LiblrhsmmRsSegmentationF64,
    streams: usize,
    segments: usize,
    boundaries: *const i32,
    duration_states: *const i32,
    output_states: *const *const i32,
    outgoing: *const LiblrhsmmRsJumpSliceF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Exclusive live owner and independent readable inputs.
    let segmentation = match unsafe {
        input_f64(
            streams,
            segments,
            boundaries,
            duration_states,
            output_states,
            outgoing,
        )
    } {
        Ok(value) => value,
        Err(status) => return status,
    };
    unsafe {
        (*handle).segmentation = segmentation;
    }
    0
}

/// Read complete stream and segment counts.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_layout_f64(
    handle: *const LiblrhsmmRsSegmentationF64,
    output: *mut LiblrhsmmRsSegmentationLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable owner and independent writable descriptor.
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsSegmentationLayout {
                streams: (*handle).segmentation.output_states.len(),
                segments: (*handle).segmentation.boundaries.len(),
            })
        })
    }
}

/// Copy a checked integer range without exposing internal storage.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_copy_boundaries_f64(
    handle: *const LiblrhsmmRsSegmentationF64,
    offset: usize,
    output: *mut i32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live owner and independent scalar buffers.
    let segmentation = unsafe { &(*handle).segmentation };
    let values = &segmentation.boundaries;
    unsafe { copy_integers(values, offset, output, count) }
}

/// Update a checked integer range atomically.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_boundaries_f64(
    handle: *mut LiblrhsmmRsSegmentationF64,
    offset: usize,
    input: *const i32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live owner and independent scalar buffers.
    let segmentation = unsafe { &mut (*handle).segmentation };
    let values = &mut segmentation.boundaries;
    unsafe { set_integers(values, offset, input, count) }
}

/// Copy a checked integer range without exposing internal storage.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_copy_duration_states_f64(
    handle: *const LiblrhsmmRsSegmentationF64,
    offset: usize,
    output: *mut i32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live owner and independent scalar buffers.
    let segmentation = unsafe { &(*handle).segmentation };
    let values = &segmentation.duration_states;
    unsafe { copy_integers(values, offset, output, count) }
}

/// Update a checked integer range atomically.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_duration_states_f64(
    handle: *mut LiblrhsmmRsSegmentationF64,
    offset: usize,
    input: *const i32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live owner and independent scalar buffers.
    let segmentation = unsafe { &mut (*handle).segmentation };
    let values = &mut segmentation.duration_states;
    unsafe { set_integers(values, offset, input, count) }
}

/// Copy a checked integer range without exposing internal storage.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_copy_output_states_f64(
    handle: *const LiblrhsmmRsSegmentationF64,
    stream: usize,
    offset: usize,
    output: *mut i32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live owner and independent scalar buffers.
    let segmentation = unsafe { &(*handle).segmentation };
    let Some(values) = segmentation.output_states.get(stream) else {
        return 2;
    };
    unsafe { copy_integers(values, offset, output, count) }
}

/// Update a checked integer range atomically.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_output_states_f64(
    handle: *mut LiblrhsmmRsSegmentationF64,
    stream: usize,
    offset: usize,
    input: *const i32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live owner and independent scalar buffers.
    let segmentation = unsafe { &mut (*handle).segmentation };
    let Some(values) = segmentation.output_states.get_mut(stream) else {
        return 2;
    };
    unsafe { set_integers(values, offset, input, count) }
}

/// Read the complete number of outgoing jumps for one segment.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_outgoing_count_f64(
    handle: *const LiblrhsmmRsSegmentationF64,
    segment: usize,
    output: *mut usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable owner and independent writable scalar result.
    let segmentation = unsafe { &(*handle).segmentation };
    let Some(group) = segmentation.outgoing.get(segment) else {
        return 2;
    };
    unsafe { scalar_result(output, || Ok(group.len())) }
}

/// Copy complete jump delta/probability fields in original group order.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_copy_outgoing_f64(
    handle: *const LiblrhsmmRsSegmentationF64,
    segment: usize,
    offset: usize,
    output: *mut LiblrhsmmRsJumpF64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1).and_then(|()| storage_range(output, count)) {
        return status;
    }
    // SAFETY: Live readable owner and independent writable jump array.
    let segmentation = unsafe { &(*handle).segmentation };
    let Some(group) = segmentation.outgoing.get(segment) else {
        return 2;
    };
    let Some(end) = offset.checked_add(count) else {
        return 2;
    };
    let Some(values) = group.get(offset..end) else {
        return 2;
    };
    for (index, value) in values.iter().enumerate() {
        unsafe {
            output.add(index).write(LiblrhsmmRsJumpF64 {
                delta: value.delta,
                probability: value.probability,
            });
        }
    }
    0
}

/// Replace one copied outgoing group, requiring one final delta-1 sentinel.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_set_outgoing_f64(
    handle: *mut LiblrhsmmRsSegmentationF64,
    segment: usize,
    input: *const LiblrhsmmRsJumpF64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live exclusive owner and independent readable jump array.
    if segment >= unsafe { (*handle).segmentation.outgoing.len() } {
        return 2;
    }
    let values = match unsafe { jumps_f64(input, count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    if crate::data::validate_jump_group(&values).is_err() {
        return 3;
    }
    unsafe {
        (&mut (*handle).segmentation.outgoing)[segment] = values;
    }
    0
}

/// Delegate native jump insertion or ordinary-edge weight replacement.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_add_jump_f64(
    handle: *mut LiblrhsmmRsSegmentationF64,
    segment: usize,
    delta: i32,
    probability: f64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live exclusive owner follows the native mutation contract.
    match boundary(|| unsafe { (*handle).segmentation.add_jump(segment, delta, probability) }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}

/// Create an independent native resegmentation from checked state occurrences.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_resegment_f64(
    handle: *const LiblrhsmmRsSegmentationF64,
    occurrences: *const LiblrhsmmRsStateOccurrence,
    count: usize,
    output: *mut *mut LiblrhsmmRsSegmentationF64,
) -> u32 {
    if let Err(status) = range(handle, 1).and_then(|()| range(occurrences, count)) {
        return status;
    }
    // SAFETY: Live readable owner, borrowed occurrences and independent output slot.
    unsafe {
        scalar_result(output, || {
            let input = if count == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(occurrences, count)
            };
            let mut path = Vec::new();
            path.try_reserve_exact(count)
                .map_err(|_| ModelError("state occurrence allocation failed"))?;
            for value in input {
                path.push(StateOccurrence {
                    state: value.state,
                    end: value.end,
                });
            }
            Ok(Box::into_raw(Box::new(LiblrhsmmRsSegmentationF64 {
                segmentation: (*handle).segmentation.resegment(&path)?,
            })))
        })
    }
}

/// Deep-copy all integer fields and outgoing probability bits.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_clone_f64(
    handle: *const LiblrhsmmRsSegmentationF64,
    output: *mut *mut LiblrhsmmRsSegmentationF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live readable owner and independent writable ownership slot.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsSegmentationF64 {
                segmentation: (*handle).segmentation.clone(),
            })))
        })
    }
}

/// Release unique ownership and clear its initialized slot; null owners are accepted.
/// # Safety
/// Handles are live owners of the declared precision. Borrowed arrays, jump groups
/// and ownership slots are aligned initialized readable storage for stated lengths.
/// Outputs are independent aligned writable storage and cannot alias inputs or owners.
/// Mutation/release requires exclusive access; reads cannot overlap mutation/release.
/// Successful constructors receive slots holding no live owner. Release consumes
/// unique ownership and clears its initialized slot. Empty arrays permit null.
/// All parameter and jump copies are independent; no internal pointer is exported.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_release_f64(
    handle: *mut *mut LiblrhsmmRsSegmentationF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Initialized exclusive ownership slot.
    let owner = unsafe { *handle };
    if owner.is_null() {
        return 0;
    }
    if let Err(status) = range(owner, 1) {
        return status;
    }
    unsafe {
        drop(Box::from_raw(owner));
        handle.write(std::ptr::null_mut());
    }
    0
}
