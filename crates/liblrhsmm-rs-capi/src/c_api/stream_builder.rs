//! Incremental stream construction with explicit unfilled slots.
use super::{
    LiblrhsmmRsGaussianF32, LiblrhsmmRsGaussianF64, LiblrhsmmRsStreamF32, LiblrhsmmRsStreamF64,
    LiblrhsmmRsStreamLayoutF32, LiblrhsmmRsStreamLayoutF64, range, scalar_result,
};
use crate::model::StreamParametersBuilder;
/// Incremental owner with explicit unfilled emission slots.
pub struct LiblrhsmmRsStreamBuilderF32 {
    pub(super) builder: StreamParametersBuilder<f32>,
}
/// Create unfilled emission slots with native default stream weight.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_builder_new_f32(
    emissions: usize,
    output: *mut *mut LiblrhsmmRsStreamBuilderF32,
) -> u32 {
    // SAFETY: Validated live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStreamBuilderF32 {
                builder: StreamParametersBuilder::new(emissions)?,
            })))
        })
    }
}

/// Read the weight and total slot count, including unfilled slots.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_builder_layout_f32(
    handle: *const LiblrhsmmRsStreamBuilderF32,
    output: *mut LiblrhsmmRsStreamLayoutF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Validated live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsStreamLayoutF32 {
                weight: (*handle).builder.weight,
                emissions: (*handle).builder.emissions.len(),
            })
        })
    }
}

/// Set arbitrary stream weight bits without filling emission slots.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_builder_set_weight_f32(
    handle: *mut LiblrhsmmRsStreamBuilderF32,
    weight: f32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Exclusive initialized builder.
    unsafe {
        (*handle).builder.weight = weight;
    }
    0
}

/// Return one for a populated slot, zero for an unfilled slot.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_builder_has_emission_f32(
    handle: *const LiblrhsmmRsStreamBuilderF32,
    index: usize,
    output: *mut u32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Readable live builder and independent writable output.
    let Some(slot) = (unsafe { &(*handle).builder.emissions }).get(index) else {
        return 2;
    };
    // SAFETY: Validated live inputs and independent writable output.
    unsafe { scalar_result(output, || Ok(u32::from(slot.is_some()))) }
}

/// Get a deep copy of a populated slot; unfilled slots return status three.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_builder_get_emission_f32(
    handle: *const LiblrhsmmRsStreamBuilderF32,
    index: usize,
    output: *mut *mut LiblrhsmmRsGaussianF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Readable live builder. Missing slot is range failure; unfilled is configuration failure.
    let Some(slot) = (unsafe { &(*handle).builder.emissions }).get(index) else {
        return 2;
    };
    let Some(value) = slot else {
        return 3;
    };
    // SAFETY: Validated live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianF32 {
                parameters: value.clone(),
            })))
        })
    }
}

/// Fill or replace one slot with a deep copy of any valid Gaussian shape.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_builder_set_emission_f32(
    handle: *mut LiblrhsmmRsStreamBuilderF32,
    index: usize,
    emission: *const LiblrhsmmRsGaussianF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(emission, 1) {
        return status;
    }
    // SAFETY: Exclusive builder and independent readable child.
    if index >= unsafe { (*handle).builder.emissions.len() } {
        return 2;
    }
    let value = match super::boundary(|| {
        let value = unsafe { (*emission).parameters.clone() };
        value.validate()?;
        Ok(value)
    }) {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Checked slot, exclusive access; deep copy is complete.
    unsafe {
        (&mut (*handle).builder.emissions)[index] = Some(value);
    }
    0
}

/// Make one emission slot unfilled, releasing its previous owned value.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_builder_clear_emission_f32(
    handle: *mut LiblrhsmmRsStreamBuilderF32,
    index: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Exclusive live builder.
    let Some(slot) = (unsafe { &mut (*handle).builder.emissions }).get_mut(index) else {
        return 2;
    };
    *slot = None;
    0
}

/// Clone populated and unfilled slots, preserving the independent source builder.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_builder_clone_f32(
    handle: *const LiblrhsmmRsStreamBuilderF32,
    output: *mut *mut LiblrhsmmRsStreamBuilderF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Validated live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStreamBuilderF32 {
                builder: (*handle).builder.clone(),
            })))
        })
    }
}

/// Build an independent stream using native validation; retain the builder even on failure.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_builder_build_f32(
    handle: *const LiblrhsmmRsStreamBuilderF32,
    output: *mut *mut LiblrhsmmRsStreamF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Validated live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStreamF32 {
                parameters: (*handle).builder.clone().build()?,
            })))
        })
    }
}

/// Release a unique builder and all populated slots; clear the ownership slot.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_builder_release_f32(
    handle: *mut *mut LiblrhsmmRsStreamBuilderF32,
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

/// Incremental owner with explicit unfilled emission slots.
pub struct LiblrhsmmRsStreamBuilderF64 {
    pub(super) builder: StreamParametersBuilder<f64>,
}
/// Create unfilled emission slots with native default stream weight.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_builder_new_f64(
    emissions: usize,
    output: *mut *mut LiblrhsmmRsStreamBuilderF64,
) -> u32 {
    // SAFETY: Validated live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStreamBuilderF64 {
                builder: StreamParametersBuilder::new(emissions)?,
            })))
        })
    }
}

/// Read the weight and total slot count, including unfilled slots.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_builder_layout_f64(
    handle: *const LiblrhsmmRsStreamBuilderF64,
    output: *mut LiblrhsmmRsStreamLayoutF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Validated live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsStreamLayoutF64 {
                weight: (*handle).builder.weight,
                emissions: (*handle).builder.emissions.len(),
            })
        })
    }
}

/// Set arbitrary stream weight bits without filling emission slots.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_builder_set_weight_f64(
    handle: *mut LiblrhsmmRsStreamBuilderF64,
    weight: f64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Exclusive initialized builder.
    unsafe {
        (*handle).builder.weight = weight;
    }
    0
}

/// Return one for a populated slot, zero for an unfilled slot.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_builder_has_emission_f64(
    handle: *const LiblrhsmmRsStreamBuilderF64,
    index: usize,
    output: *mut u32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Readable live builder and independent writable output.
    let Some(slot) = (unsafe { &(*handle).builder.emissions }).get(index) else {
        return 2;
    };
    // SAFETY: Validated live inputs and independent writable output.
    unsafe { scalar_result(output, || Ok(u32::from(slot.is_some()))) }
}

/// Get a deep copy of a populated slot; unfilled slots return status three.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_builder_get_emission_f64(
    handle: *const LiblrhsmmRsStreamBuilderF64,
    index: usize,
    output: *mut *mut LiblrhsmmRsGaussianF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Readable live builder. Missing slot is range failure; unfilled is configuration failure.
    let Some(slot) = (unsafe { &(*handle).builder.emissions }).get(index) else {
        return 2;
    };
    let Some(value) = slot else {
        return 3;
    };
    // SAFETY: Validated live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsGaussianF64 {
                parameters: value.clone(),
            })))
        })
    }
}

/// Fill or replace one slot with a deep copy of any valid Gaussian shape.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_builder_set_emission_f64(
    handle: *mut LiblrhsmmRsStreamBuilderF64,
    index: usize,
    emission: *const LiblrhsmmRsGaussianF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(emission, 1) {
        return status;
    }
    // SAFETY: Exclusive builder and independent readable child.
    if index >= unsafe { (*handle).builder.emissions.len() } {
        return 2;
    }
    let value = match super::boundary(|| {
        let value = unsafe { (*emission).parameters.clone() };
        value.validate()?;
        Ok(value)
    }) {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Checked slot, exclusive access; deep copy is complete.
    unsafe {
        (&mut (*handle).builder.emissions)[index] = Some(value);
    }
    0
}

/// Make one emission slot unfilled, releasing its previous owned value.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_builder_clear_emission_f64(
    handle: *mut LiblrhsmmRsStreamBuilderF64,
    index: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Exclusive live builder.
    let Some(slot) = (unsafe { &mut (*handle).builder.emissions }).get_mut(index) else {
        return 2;
    };
    *slot = None;
    0
}

/// Clone populated and unfilled slots, preserving the independent source builder.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_builder_clone_f64(
    handle: *const LiblrhsmmRsStreamBuilderF64,
    output: *mut *mut LiblrhsmmRsStreamBuilderF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Validated live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStreamBuilderF64 {
                builder: (*handle).builder.clone(),
            })))
        })
    }
}

/// Build an independent stream using native validation; retain the builder even on failure.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_builder_build_f64(
    handle: *const LiblrhsmmRsStreamBuilderF64,
    output: *mut *mut LiblrhsmmRsStreamF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Validated live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStreamF64 {
                parameters: (*handle).builder.clone().build()?,
            })))
        })
    }
}

/// Release a unique builder and all populated slots; clear the ownership slot.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_builder_release_f64(
    handle: *mut *mut LiblrhsmmRsStreamBuilderF64,
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
