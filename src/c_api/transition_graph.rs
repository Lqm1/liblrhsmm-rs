//! Independent immutable segmentation snapshots with complete cached jump access.
use super::gaussian::storage_range;
use super::*;
use crate::transitions::OwnedTransitionGraphParameters;

/// Independently owned immutable parameters and matching incoming jump cache.
#[derive(Debug, Clone)]
pub struct LiblrhsmmRsTransitionGraphF32 {
    pub(super) graph: OwnedTransitionGraphParameters<f32>,
}

/// Copy source parameters and derive incoming transitions exactly once.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned initialized
/// readable storage; outputs are independent aligned writable storage of stated lengths.
/// Reads cannot overlap mutation or release. Constructor slots hold no live owner.
/// Returned graph and segmentation owners are independent copies; no borrowed pointer
/// escapes. Release consumes unique ownership and clears its initialized slot.
/// Null ownership is a release no-op; empty copy intervals permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_build_transitions_f32(
    handle: *const LiblrhsmmRsSegmentationF32,
    output: *mut *mut LiblrhsmmRsTransitionGraphF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live source and independent writable unowned slot.
    unsafe {
        scalar_result(output, || {
            let graph = (*handle).segmentation.clone().build_transitions_owned()?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsTransitionGraphF32 {
                graph,
            })))
        })
    }
}

/// Read complete stream and segment counts without copying or rebuilding caches.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned initialized
/// readable storage; outputs are independent aligned writable storage of stated lengths.
/// Reads cannot overlap mutation or release. Constructor slots hold no live owner.
/// Returned graph and segmentation owners are independent copies; no borrowed pointer
/// escapes. Release consumes unique ownership and clears its initialized slot.
/// Null ownership is a release no-op; empty copy intervals permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_transition_graph_layout_f32(
    handle: *const LiblrhsmmRsTransitionGraphF32,
    output: *mut LiblrhsmmRsSegmentationLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live graph and independent writable descriptor.
    unsafe {
        scalar_result(output, || {
            let segmentation = (*handle).graph.segmentation();
            Ok(LiblrhsmmRsSegmentationLayout {
                streams: segmentation.output_states.len(),
                segments: segmentation.boundaries.len(),
            })
        })
    }
}

/// Copy every parameter field and outgoing group into an independent editable owner.
/// The immutable graph and its incoming cache remain unchanged.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned initialized
/// readable storage; outputs are independent aligned writable storage of stated lengths.
/// Reads cannot overlap mutation or release. Constructor slots hold no live owner.
/// Returned graph and segmentation owners are independent copies; no borrowed pointer
/// escapes. Release consumes unique ownership and clears its initialized slot.
/// Null ownership is a release no-op; empty copy intervals permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_transition_graph_get_segmentation_f32(
    handle: *const LiblrhsmmRsTransitionGraphF32,
    output: *mut *mut LiblrhsmmRsSegmentationF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live graph and independent writable unowned slot.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsSegmentationF32 {
                segmentation: (*handle).graph.segmentation().clone(),
            })))
        })
    }
}

/// Read one incoming group length, including the native ordinary-edge sentinel.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned initialized
/// readable storage; outputs are independent aligned writable storage of stated lengths.
/// Reads cannot overlap mutation or release. Constructor slots hold no live owner.
/// Returned graph and segmentation owners are independent copies; no borrowed pointer
/// escapes. Release consumes unique ownership and clears its initialized slot.
/// Null ownership is a release no-op; empty copy intervals permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_transition_graph_incoming_count_f32(
    handle: *const LiblrhsmmRsTransitionGraphF32,
    segment: usize,
    output: *mut usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live immutable graph and independent writable result.
    let graph = unsafe { &(*handle).graph };
    let Some(group) = graph.incoming().get(segment) else {
        return 2;
    };
    unsafe { scalar_result(output, || Ok(group.len())) }
}

/// Copy a checked incoming interval, retaining every delta and probability bit.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned initialized
/// readable storage; outputs are independent aligned writable storage of stated lengths.
/// Reads cannot overlap mutation or release. Constructor slots hold no live owner.
/// Returned graph and segmentation owners are independent copies; no borrowed pointer
/// escapes. Release consumes unique ownership and clears its initialized slot.
/// Null ownership is a release no-op; empty copy intervals permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_transition_graph_copy_incoming_f32(
    handle: *const LiblrhsmmRsTransitionGraphF32,
    segment: usize,
    offset: usize,
    output: *mut LiblrhsmmRsJumpF32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1).and_then(|()| storage_range(output, count)) {
        return status;
    }
    // SAFETY: Live immutable graph and independent writable jump storage.
    let graph = unsafe { &(*handle).graph };
    let Some(end) = offset.checked_add(count) else {
        return 2;
    };
    let Some(group) = graph
        .incoming()
        .get(segment)
        .and_then(|group| group.get(offset..end))
    else {
        return 2;
    };
    for (index, jump) in group.iter().enumerate() {
        // SAFETY: All output ranges and immutable source intervals checked before writes.
        unsafe {
            output.add(index).write(LiblrhsmmRsJumpF32 {
                delta: jump.delta,
                probability: jump.probability,
            });
        }
    }
    0
}

/// Read one outgoing group length, including the native ordinary-edge sentinel.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned initialized
/// readable storage; outputs are independent aligned writable storage of stated lengths.
/// Reads cannot overlap mutation or release. Constructor slots hold no live owner.
/// Returned graph and segmentation owners are independent copies; no borrowed pointer
/// escapes. Release consumes unique ownership and clears its initialized slot.
/// Null ownership is a release no-op; empty copy intervals permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_transition_graph_outgoing_count_f32(
    handle: *const LiblrhsmmRsTransitionGraphF32,
    segment: usize,
    output: *mut usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live immutable graph and independent writable result.
    let graph = unsafe { &(*handle).graph };
    let Some(group) = graph.outgoing().get(segment) else {
        return 2;
    };
    unsafe { scalar_result(output, || Ok(group.len())) }
}

/// Copy a checked outgoing interval, retaining every delta and probability bit.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned initialized
/// readable storage; outputs are independent aligned writable storage of stated lengths.
/// Reads cannot overlap mutation or release. Constructor slots hold no live owner.
/// Returned graph and segmentation owners are independent copies; no borrowed pointer
/// escapes. Release consumes unique ownership and clears its initialized slot.
/// Null ownership is a release no-op; empty copy intervals permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_transition_graph_copy_outgoing_f32(
    handle: *const LiblrhsmmRsTransitionGraphF32,
    segment: usize,
    offset: usize,
    output: *mut LiblrhsmmRsJumpF32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1).and_then(|()| storage_range(output, count)) {
        return status;
    }
    // SAFETY: Live immutable graph and independent writable jump storage.
    let graph = unsafe { &(*handle).graph };
    let Some(end) = offset.checked_add(count) else {
        return 2;
    };
    let Some(group) = graph
        .outgoing()
        .get(segment)
        .and_then(|group| group.get(offset..end))
    else {
        return 2;
    };
    for (index, jump) in group.iter().enumerate() {
        // SAFETY: All output ranges and immutable source intervals checked before writes.
        unsafe {
            output.add(index).write(LiblrhsmmRsJumpF32 {
                delta: jump.delta,
                probability: jump.probability,
            });
        }
    }
    0
}

/// Independently copy all parameters and already-derived incoming jumps.
/// This operation does not recompute transitions.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned initialized
/// readable storage; outputs are independent aligned writable storage of stated lengths.
/// Reads cannot overlap mutation or release. Constructor slots hold no live owner.
/// Returned graph and segmentation owners are independent copies; no borrowed pointer
/// escapes. Release consumes unique ownership and clears its initialized slot.
/// Null ownership is a release no-op; empty copy intervals permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_transition_graph_clone_f32(
    handle: *const LiblrhsmmRsTransitionGraphF32,
    output: *mut *mut LiblrhsmmRsTransitionGraphF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live graph and independent writable unowned slot.
    unsafe { scalar_result(output, || Ok(Box::into_raw(Box::new((*handle).clone())))) }
}

/// Consume a unique graph owner and clear its initialized ownership slot.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned initialized
/// readable storage; outputs are independent aligned writable storage of stated lengths.
/// Reads cannot overlap mutation or release. Constructor slots hold no live owner.
/// Returned graph and segmentation owners are independent copies; no borrowed pointer
/// escapes. Release consumes unique ownership and clears its initialized slot.
/// Null ownership is a release no-op; empty copy intervals permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_transition_graph_release_f32(
    handle: *mut *mut LiblrhsmmRsTransitionGraphF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Initialized exclusive ownership slot.
    let owner = unsafe { *handle };
    if owner.is_null() {
        return 0;
    }
    match boundary(|| {
        // SAFETY: Unique Box owner of the declared precision; no live borrows.
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

/// Independently owned immutable parameters and matching incoming jump cache.
#[derive(Debug, Clone)]
pub struct LiblrhsmmRsTransitionGraphF64 {
    pub(super) graph: OwnedTransitionGraphParameters<f64>,
}

/// Copy source parameters and derive incoming transitions exactly once.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned initialized
/// readable storage; outputs are independent aligned writable storage of stated lengths.
/// Reads cannot overlap mutation or release. Constructor slots hold no live owner.
/// Returned graph and segmentation owners are independent copies; no borrowed pointer
/// escapes. Release consumes unique ownership and clears its initialized slot.
/// Null ownership is a release no-op; empty copy intervals permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_segmentation_build_transitions_f64(
    handle: *const LiblrhsmmRsSegmentationF64,
    output: *mut *mut LiblrhsmmRsTransitionGraphF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live source and independent writable unowned slot.
    unsafe {
        scalar_result(output, || {
            let graph = (*handle).segmentation.clone().build_transitions_owned()?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsTransitionGraphF64 {
                graph,
            })))
        })
    }
}

/// Read complete stream and segment counts without copying or rebuilding caches.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned initialized
/// readable storage; outputs are independent aligned writable storage of stated lengths.
/// Reads cannot overlap mutation or release. Constructor slots hold no live owner.
/// Returned graph and segmentation owners are independent copies; no borrowed pointer
/// escapes. Release consumes unique ownership and clears its initialized slot.
/// Null ownership is a release no-op; empty copy intervals permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_transition_graph_layout_f64(
    handle: *const LiblrhsmmRsTransitionGraphF64,
    output: *mut LiblrhsmmRsSegmentationLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live graph and independent writable descriptor.
    unsafe {
        scalar_result(output, || {
            let segmentation = (*handle).graph.segmentation();
            Ok(LiblrhsmmRsSegmentationLayout {
                streams: segmentation.output_states.len(),
                segments: segmentation.boundaries.len(),
            })
        })
    }
}

/// Copy every parameter field and outgoing group into an independent editable owner.
/// The immutable graph and its incoming cache remain unchanged.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned initialized
/// readable storage; outputs are independent aligned writable storage of stated lengths.
/// Reads cannot overlap mutation or release. Constructor slots hold no live owner.
/// Returned graph and segmentation owners are independent copies; no borrowed pointer
/// escapes. Release consumes unique ownership and clears its initialized slot.
/// Null ownership is a release no-op; empty copy intervals permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_transition_graph_get_segmentation_f64(
    handle: *const LiblrhsmmRsTransitionGraphF64,
    output: *mut *mut LiblrhsmmRsSegmentationF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live graph and independent writable unowned slot.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsSegmentationF64 {
                segmentation: (*handle).graph.segmentation().clone(),
            })))
        })
    }
}

/// Read one incoming group length, including the native ordinary-edge sentinel.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned initialized
/// readable storage; outputs are independent aligned writable storage of stated lengths.
/// Reads cannot overlap mutation or release. Constructor slots hold no live owner.
/// Returned graph and segmentation owners are independent copies; no borrowed pointer
/// escapes. Release consumes unique ownership and clears its initialized slot.
/// Null ownership is a release no-op; empty copy intervals permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_transition_graph_incoming_count_f64(
    handle: *const LiblrhsmmRsTransitionGraphF64,
    segment: usize,
    output: *mut usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live immutable graph and independent writable result.
    let graph = unsafe { &(*handle).graph };
    let Some(group) = graph.incoming().get(segment) else {
        return 2;
    };
    unsafe { scalar_result(output, || Ok(group.len())) }
}

/// Copy a checked incoming interval, retaining every delta and probability bit.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned initialized
/// readable storage; outputs are independent aligned writable storage of stated lengths.
/// Reads cannot overlap mutation or release. Constructor slots hold no live owner.
/// Returned graph and segmentation owners are independent copies; no borrowed pointer
/// escapes. Release consumes unique ownership and clears its initialized slot.
/// Null ownership is a release no-op; empty copy intervals permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_transition_graph_copy_incoming_f64(
    handle: *const LiblrhsmmRsTransitionGraphF64,
    segment: usize,
    offset: usize,
    output: *mut LiblrhsmmRsJumpF64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1).and_then(|()| storage_range(output, count)) {
        return status;
    }
    // SAFETY: Live immutable graph and independent writable jump storage.
    let graph = unsafe { &(*handle).graph };
    let Some(end) = offset.checked_add(count) else {
        return 2;
    };
    let Some(group) = graph
        .incoming()
        .get(segment)
        .and_then(|group| group.get(offset..end))
    else {
        return 2;
    };
    for (index, jump) in group.iter().enumerate() {
        // SAFETY: All output ranges and immutable source intervals checked before writes.
        unsafe {
            output.add(index).write(LiblrhsmmRsJumpF64 {
                delta: jump.delta,
                probability: jump.probability,
            });
        }
    }
    0
}

/// Read one outgoing group length, including the native ordinary-edge sentinel.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned initialized
/// readable storage; outputs are independent aligned writable storage of stated lengths.
/// Reads cannot overlap mutation or release. Constructor slots hold no live owner.
/// Returned graph and segmentation owners are independent copies; no borrowed pointer
/// escapes. Release consumes unique ownership and clears its initialized slot.
/// Null ownership is a release no-op; empty copy intervals permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_transition_graph_outgoing_count_f64(
    handle: *const LiblrhsmmRsTransitionGraphF64,
    segment: usize,
    output: *mut usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live immutable graph and independent writable result.
    let graph = unsafe { &(*handle).graph };
    let Some(group) = graph.outgoing().get(segment) else {
        return 2;
    };
    unsafe { scalar_result(output, || Ok(group.len())) }
}

/// Copy a checked outgoing interval, retaining every delta and probability bit.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned initialized
/// readable storage; outputs are independent aligned writable storage of stated lengths.
/// Reads cannot overlap mutation or release. Constructor slots hold no live owner.
/// Returned graph and segmentation owners are independent copies; no borrowed pointer
/// escapes. Release consumes unique ownership and clears its initialized slot.
/// Null ownership is a release no-op; empty copy intervals permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_transition_graph_copy_outgoing_f64(
    handle: *const LiblrhsmmRsTransitionGraphF64,
    segment: usize,
    offset: usize,
    output: *mut LiblrhsmmRsJumpF64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1).and_then(|()| storage_range(output, count)) {
        return status;
    }
    // SAFETY: Live immutable graph and independent writable jump storage.
    let graph = unsafe { &(*handle).graph };
    let Some(end) = offset.checked_add(count) else {
        return 2;
    };
    let Some(group) = graph
        .outgoing()
        .get(segment)
        .and_then(|group| group.get(offset..end))
    else {
        return 2;
    };
    for (index, jump) in group.iter().enumerate() {
        // SAFETY: All output ranges and immutable source intervals checked before writes.
        unsafe {
            output.add(index).write(LiblrhsmmRsJumpF64 {
                delta: jump.delta,
                probability: jump.probability,
            });
        }
    }
    0
}

/// Independently copy all parameters and already-derived incoming jumps.
/// This operation does not recompute transitions.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned initialized
/// readable storage; outputs are independent aligned writable storage of stated lengths.
/// Reads cannot overlap mutation or release. Constructor slots hold no live owner.
/// Returned graph and segmentation owners are independent copies; no borrowed pointer
/// escapes. Release consumes unique ownership and clears its initialized slot.
/// Null ownership is a release no-op; empty copy intervals permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_transition_graph_clone_f64(
    handle: *const LiblrhsmmRsTransitionGraphF64,
    output: *mut *mut LiblrhsmmRsTransitionGraphF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Live graph and independent writable unowned slot.
    unsafe { scalar_result(output, || Ok(Box::into_raw(Box::new((*handle).clone())))) }
}

/// Consume a unique graph owner and clear its initialized ownership slot.
/// # Safety
/// Handles are live owners of the declared precision. Inputs are aligned initialized
/// readable storage; outputs are independent aligned writable storage of stated lengths.
/// Reads cannot overlap mutation or release. Constructor slots hold no live owner.
/// Returned graph and segmentation owners are independent copies; no borrowed pointer
/// escapes. Release consumes unique ownership and clears its initialized slot.
/// Null ownership is a release no-op; empty copy intervals permit null storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_transition_graph_release_f64(
    handle: *mut *mut LiblrhsmmRsTransitionGraphF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Initialized exclusive ownership slot.
    let owner = unsafe { *handle };
    if owner.is_null() {
        return 0;
    }
    match boundary(|| {
        // SAFETY: Unique Box owner of the declared precision; no live borrows.
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
