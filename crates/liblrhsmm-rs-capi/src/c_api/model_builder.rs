//! Incremental model construction with explicit unfilled stream and duration slots.
use super::{
    LiblrhsmmRsDurationF32, LiblrhsmmRsDurationF64, LiblrhsmmRsModelF32, LiblrhsmmRsModelF64,
    LiblrhsmmRsModelLayout, LiblrhsmmRsStreamF32, LiblrhsmmRsStreamF64, range, scalar_result,
};
use crate::model::ModelParametersBuilder;
/// Independent incremental model owner.
pub struct LiblrhsmmRsModelBuilderF32 {
    pub(super) builder: ModelParametersBuilder<f32>,
}
/// Construct unfilled native stream and duration slots.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_builder_new_f32(
    streams: usize,
    durations: usize,
    output: *mut *mut LiblrhsmmRsModelBuilderF32,
) -> u32 {
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsModelBuilderF32 {
                builder: ModelParametersBuilder::new(streams, durations)?,
            })))
        })
    }
}

/// Read total stream and duration slot counts, including unfilled slots.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_builder_layout_f32(
    handle: *const LiblrhsmmRsModelBuilderF32,
    output: *mut LiblrhsmmRsModelLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsModelLayout {
                streams: (*handle).builder.streams.len(),
                durations: (*handle).builder.durations.len(),
            })
        })
    }
}

/// Return one for a populated stream slot, zero for an unfilled slot.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_builder_has_stream_f32(
    handle: *const LiblrhsmmRsModelBuilderF32,
    index: usize,
    output: *mut u32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Readable live builder and independent output.
    let Some(slot) = (unsafe { &(*handle).builder.streams }).get(index) else {
        return 2;
    };
    // SAFETY: Readable live inputs and independent writable output.
    unsafe { scalar_result(output, || Ok(u32::from(slot.is_some()))) }
}

/// Clone a populated stream slot; unfilled slots return status three.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_builder_get_stream_f32(
    handle: *const LiblrhsmmRsModelBuilderF32,
    index: usize,
    output: *mut *mut LiblrhsmmRsStreamF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Readable live builder. Invalid index is range failure, unfilled is configuration failure.
    let Some(slot) = (unsafe { &(*handle).builder.streams }).get(index) else {
        return 2;
    };
    let Some(value) = slot else {
        return 3;
    };
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStreamF32 {
                parameters: value.clone(),
            })))
        })
    }
}

/// Fill or replace a stream slot with an independent deep copy.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_builder_set_stream_f32(
    handle: *mut LiblrhsmmRsModelBuilderF32,
    index: usize,
    child: *const LiblrhsmmRsStreamF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(child, 1) {
        return status;
    }
    // SAFETY: Exclusive builder and independent readable child.
    if index >= unsafe { (*handle).builder.streams.len() } {
        return 2;
    }
    let value = match super::boundary(|| {
        let value = unsafe { (*child).parameters.clone() };
        value.validate()?;
        Ok(value)
    }) {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Checked slot, exclusive access; input copy complete.
    unsafe {
        (&mut (*handle).builder.streams)[index] = Some(value);
    }
    0
}

/// Make one stream slot unfilled, releasing its previously stored child.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_builder_clear_stream_f32(
    handle: *mut LiblrhsmmRsModelBuilderF32,
    index: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Exclusive initialized builder and checked slot.
    let Some(slot) = (unsafe { &mut (*handle).builder.streams }).get_mut(index) else {
        return 2;
    };
    *slot = None;
    0
}

/// Return one for a populated duration slot, zero for an unfilled slot.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_builder_has_duration_f32(
    handle: *const LiblrhsmmRsModelBuilderF32,
    index: usize,
    output: *mut u32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Readable live builder and independent output.
    let Some(slot) = (unsafe { &(*handle).builder.durations }).get(index) else {
        return 2;
    };
    // SAFETY: Readable live inputs and independent writable output.
    unsafe { scalar_result(output, || Ok(u32::from(slot.is_some()))) }
}

/// Clone a populated duration slot; unfilled slots return status three.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_builder_get_duration_f32(
    handle: *const LiblrhsmmRsModelBuilderF32,
    index: usize,
    output: *mut *mut LiblrhsmmRsDurationF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Readable live builder. Invalid index is range failure, unfilled is configuration failure.
    let Some(slot) = (unsafe { &(*handle).builder.durations }).get(index) else {
        return 2;
    };
    let Some(value) = slot else {
        return 3;
    };
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsDurationF32 {
                parameters: value.clone(),
            })))
        })
    }
}

/// Fill or replace a duration slot with an independent deep copy.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_builder_set_duration_f32(
    handle: *mut LiblrhsmmRsModelBuilderF32,
    index: usize,
    child: *const LiblrhsmmRsDurationF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(child, 1) {
        return status;
    }
    // SAFETY: Exclusive builder and independent readable child.
    if index >= unsafe { (*handle).builder.durations.len() } {
        return 2;
    }
    let value = match super::boundary(|| {
        let value = unsafe { (*child).parameters.clone() };
        Ok(value)
    }) {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Checked slot, exclusive access; input copy complete.
    unsafe {
        (&mut (*handle).builder.durations)[index] = Some(value);
    }
    0
}

/// Make one duration slot unfilled, releasing its previously stored child.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_builder_clear_duration_f32(
    handle: *mut LiblrhsmmRsModelBuilderF32,
    index: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Exclusive initialized builder and checked slot.
    let Some(slot) = (unsafe { &mut (*handle).builder.durations }).get_mut(index) else {
        return 2;
    };
    *slot = None;
    0
}

/// Clone populated and unfilled slots into independent builder storage.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_builder_clone_f32(
    handle: *const LiblrhsmmRsModelBuilderF32,
    output: *mut *mut LiblrhsmmRsModelBuilderF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsModelBuilderF32 {
                builder: (*handle).builder.clone(),
            })))
        })
    }
}

/// Build a validated independent native model; keep the source builder on success/failure.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_builder_build_f32(
    handle: *const LiblrhsmmRsModelBuilderF32,
    output: *mut *mut LiblrhsmmRsModelF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsModelF32 {
                parameters: (*handle).builder.clone().build()?,
            })))
        })
    }
}

/// Release a unique builder and all populated slots; clear its ownership slot.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_builder_release_f32(
    handle: *mut *mut LiblrhsmmRsModelBuilderF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Initialized unique owner slot, null or a live aligned allocation.
    let pointer = unsafe { handle.read() };
    if pointer.is_null() {
        return 0;
    }
    if let Err(status) = range(pointer, 1) {
        return status;
    }
    unsafe {
        handle.write(std::ptr::null_mut());
        drop(Box::from_raw(pointer));
    }
    0
}

/// Independent incremental model owner.
pub struct LiblrhsmmRsModelBuilderF64 {
    pub(super) builder: ModelParametersBuilder<f64>,
}
/// Construct unfilled native stream and duration slots.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_builder_new_f64(
    streams: usize,
    durations: usize,
    output: *mut *mut LiblrhsmmRsModelBuilderF64,
) -> u32 {
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsModelBuilderF64 {
                builder: ModelParametersBuilder::new(streams, durations)?,
            })))
        })
    }
}

/// Read total stream and duration slot counts, including unfilled slots.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_builder_layout_f64(
    handle: *const LiblrhsmmRsModelBuilderF64,
    output: *mut LiblrhsmmRsModelLayout,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsModelLayout {
                streams: (*handle).builder.streams.len(),
                durations: (*handle).builder.durations.len(),
            })
        })
    }
}

/// Return one for a populated stream slot, zero for an unfilled slot.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_builder_has_stream_f64(
    handle: *const LiblrhsmmRsModelBuilderF64,
    index: usize,
    output: *mut u32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Readable live builder and independent output.
    let Some(slot) = (unsafe { &(*handle).builder.streams }).get(index) else {
        return 2;
    };
    // SAFETY: Readable live inputs and independent writable output.
    unsafe { scalar_result(output, || Ok(u32::from(slot.is_some()))) }
}

/// Clone a populated stream slot; unfilled slots return status three.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_builder_get_stream_f64(
    handle: *const LiblrhsmmRsModelBuilderF64,
    index: usize,
    output: *mut *mut LiblrhsmmRsStreamF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Readable live builder. Invalid index is range failure, unfilled is configuration failure.
    let Some(slot) = (unsafe { &(*handle).builder.streams }).get(index) else {
        return 2;
    };
    let Some(value) = slot else {
        return 3;
    };
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStreamF64 {
                parameters: value.clone(),
            })))
        })
    }
}

/// Fill or replace a stream slot with an independent deep copy.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_builder_set_stream_f64(
    handle: *mut LiblrhsmmRsModelBuilderF64,
    index: usize,
    child: *const LiblrhsmmRsStreamF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(child, 1) {
        return status;
    }
    // SAFETY: Exclusive builder and independent readable child.
    if index >= unsafe { (*handle).builder.streams.len() } {
        return 2;
    }
    let value = match super::boundary(|| {
        let value = unsafe { (*child).parameters.clone() };
        value.validate()?;
        Ok(value)
    }) {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Checked slot, exclusive access; input copy complete.
    unsafe {
        (&mut (*handle).builder.streams)[index] = Some(value);
    }
    0
}

/// Make one stream slot unfilled, releasing its previously stored child.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_builder_clear_stream_f64(
    handle: *mut LiblrhsmmRsModelBuilderF64,
    index: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Exclusive initialized builder and checked slot.
    let Some(slot) = (unsafe { &mut (*handle).builder.streams }).get_mut(index) else {
        return 2;
    };
    *slot = None;
    0
}

/// Return one for a populated duration slot, zero for an unfilled slot.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_builder_has_duration_f64(
    handle: *const LiblrhsmmRsModelBuilderF64,
    index: usize,
    output: *mut u32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Readable live builder and independent output.
    let Some(slot) = (unsafe { &(*handle).builder.durations }).get(index) else {
        return 2;
    };
    // SAFETY: Readable live inputs and independent writable output.
    unsafe { scalar_result(output, || Ok(u32::from(slot.is_some()))) }
}

/// Clone a populated duration slot; unfilled slots return status three.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_builder_get_duration_f64(
    handle: *const LiblrhsmmRsModelBuilderF64,
    index: usize,
    output: *mut *mut LiblrhsmmRsDurationF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Readable live builder. Invalid index is range failure, unfilled is configuration failure.
    let Some(slot) = (unsafe { &(*handle).builder.durations }).get(index) else {
        return 2;
    };
    let Some(value) = slot else {
        return 3;
    };
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsDurationF64 {
                parameters: value.clone(),
            })))
        })
    }
}

/// Fill or replace a duration slot with an independent deep copy.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_builder_set_duration_f64(
    handle: *mut LiblrhsmmRsModelBuilderF64,
    index: usize,
    child: *const LiblrhsmmRsDurationF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(child, 1) {
        return status;
    }
    // SAFETY: Exclusive builder and independent readable child.
    if index >= unsafe { (*handle).builder.durations.len() } {
        return 2;
    }
    let value = match super::boundary(|| {
        let value = unsafe { (*child).parameters.clone() };
        Ok(value)
    }) {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Checked slot, exclusive access; input copy complete.
    unsafe {
        (&mut (*handle).builder.durations)[index] = Some(value);
    }
    0
}

/// Make one duration slot unfilled, releasing its previously stored child.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_builder_clear_duration_f64(
    handle: *mut LiblrhsmmRsModelBuilderF64,
    index: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Exclusive initialized builder and checked slot.
    let Some(slot) = (unsafe { &mut (*handle).builder.durations }).get_mut(index) else {
        return 2;
    };
    *slot = None;
    0
}

/// Clone populated and unfilled slots into independent builder storage.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_builder_clone_f64(
    handle: *const LiblrhsmmRsModelBuilderF64,
    output: *mut *mut LiblrhsmmRsModelBuilderF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsModelBuilderF64 {
                builder: (*handle).builder.clone(),
            })))
        })
    }
}

/// Build a validated independent native model; keep the source builder on success/failure.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_builder_build_f64(
    handle: *const LiblrhsmmRsModelBuilderF64,
    output: *mut *mut LiblrhsmmRsModelF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Readable live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsModelF64 {
                parameters: (*handle).builder.clone().build()?,
            })))
        })
    }
}

/// Release a unique builder and all populated slots; clear its ownership slot.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays/descriptors and
/// ownership slots are initialized and readable. Outputs are aligned independently
/// writable storage and cannot alias owners or inputs. Mutation/release requires
/// exclusive access; no concurrent mutation or release overlaps reads. Inputs are
/// borrowed only during this call and may repeat; child results are deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_builder_release_f64(
    handle: *mut *mut LiblrhsmmRsModelBuilderF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Initialized unique owner slot, null or a live aligned allocation.
    let pointer = unsafe { handle.read() };
    if pointer.is_null() {
        return 0;
    }
    if let Err(status) = range(pointer, 1) {
        return status;
    }
    unsafe {
        handle.write(std::ptr::null_mut());
        drop(Box::from_raw(pointer));
    }
    0
}
