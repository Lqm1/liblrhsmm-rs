//! Independent stream parameters with copied Gaussian children.
use super::{LiblrhsmmRsGaussianF32, LiblrhsmmRsGaussianF64, range, scalar_result};
use crate::model::{ModelError, StreamParameters};
/// Stream weight and emission count. Emissions may have different Gaussian layouts.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LiblrhsmmRsStreamLayoutF32 {
    pub weight: f32,
    pub emissions: usize,
}
/// Independent editable stream parameters.
pub struct LiblrhsmmRsStreamF32 {
    pub(super) parameters: StreamParameters<f32>,
}
unsafe fn input_f32(
    weight: f32,
    handles: *const *const LiblrhsmmRsGaussianF32,
    count: usize,
) -> Result<StreamParameters<f32>, u32> {
    range(handles, count)?;
    // SAFETY: Initialized readable borrowed array; empty arrays permit null.
    let handles = if count == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(handles, count) }
    };
    for &handle in handles {
        range(handle, 1)?;
    }
    super::boundary(|| {
        let mut mixtures = Vec::new();
        mixtures
            .try_reserve_exact(count)
            .map_err(|_| ModelError("stream allocation failed"))?;
        for &handle in handles {
            // SAFETY: Validated readable child retained by caller throughout this call.
            mixtures.push(unsafe { (*handle).parameters.clone() });
        }
        let value = StreamParameters { weight, mixtures };
        value.validate()?;
        Ok(value)
    })
}
/// Construct native equally sized flat-start emissions, including an empty stream.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_new_f32(
    emissions: usize,
    components: usize,
    dimensions: usize,
    output: *mut *mut LiblrhsmmRsStreamF32,
) -> u32 {
    // SAFETY: Validated live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStreamF32 {
                parameters: StreamParameters::new(emissions, components, dimensions)?,
            })))
        })
    }
}

/// Copy complete weight and emission parameters; failures preserve existing storage.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_create_f32(
    weight: f32,
    emissions: *const *const LiblrhsmmRsGaussianF32,
    count: usize,
    output: *mut *mut LiblrhsmmRsStreamF32,
) -> u32 {
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Borrowed independent child array follows the input contract.
    let value = match unsafe { input_f32(weight, emissions, count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Validated live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStreamF32 {
                parameters: value,
            })))
        })
    }
}

/// Copy complete weight and emission parameters; failures preserve existing storage.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_replace_f32(
    handle: *mut LiblrhsmmRsStreamF32,
    weight: f32,
    emissions: *const *const LiblrhsmmRsGaussianF32,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Borrowed independent child array follows the input contract.
    let value = match unsafe { input_f32(weight, emissions, count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Exclusive owner; all input validation and copying completed.
    unsafe {
        (*handle).parameters = value;
    }
    0
}

/// Read the complete stream weight and emission count.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_layout_f32(
    handle: *const LiblrhsmmRsStreamF32,
    output: *mut LiblrhsmmRsStreamLayoutF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Validated live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsStreamLayoutF32 {
                weight: (*handle).parameters.weight,
                emissions: (*handle).parameters.mixtures.len(),
            })
        })
    }
}

/// Replace the stream weight without changing any emission.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_set_weight_f32(
    handle: *mut LiblrhsmmRsStreamF32,
    weight: f32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Exclusive live owner. Arbitrary scalar bits are retained.
    unsafe {
        (*handle).parameters.weight = weight;
    }
    0
}

/// Clone one emission into an independent editable Gaussian owner.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_get_emission_f32(
    handle: *const LiblrhsmmRsStreamF32,
    index: usize,
    output: *mut *mut LiblrhsmmRsGaussianF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Live readable owner, independent output.
    let Some(value) = (unsafe { &(*handle).parameters.mixtures }).get(index) else {
        return 2;
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

/// Replace one emission with a deep copy; its shape may change.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_set_emission_f32(
    handle: *mut LiblrhsmmRsStreamF32,
    index: usize,
    emission: *const LiblrhsmmRsGaussianF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(emission, 1) {
        return status;
    }
    // SAFETY: Exclusive stream and independent readable Gaussian input.
    if index >= unsafe { (*handle).parameters.mixtures.len() } {
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
    // SAFETY: Checked index, exclusive owner; input copy is complete.
    unsafe {
        (&mut (*handle).parameters.mixtures)[index] = value;
    }
    0
}

/// Clone the weight and every Gaussian child into an independent owner.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_clone_f32(
    handle: *const LiblrhsmmRsStreamF32,
    output: *mut *mut LiblrhsmmRsStreamF32,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Validated live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStreamF32 {
                parameters: (*handle).parameters.clone(),
            })))
        })
    }
}

/// Release a unique stream and all its owned children; clear the slot.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_release_f32(
    handle: *mut *mut LiblrhsmmRsStreamF32,
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

/// Stream weight and emission count. Emissions may have different Gaussian layouts.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LiblrhsmmRsStreamLayoutF64 {
    pub weight: f64,
    pub emissions: usize,
}
/// Independent editable stream parameters.
pub struct LiblrhsmmRsStreamF64 {
    pub(super) parameters: StreamParameters<f64>,
}
unsafe fn input_f64(
    weight: f64,
    handles: *const *const LiblrhsmmRsGaussianF64,
    count: usize,
) -> Result<StreamParameters<f64>, u32> {
    range(handles, count)?;
    // SAFETY: Initialized readable borrowed array; empty arrays permit null.
    let handles = if count == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(handles, count) }
    };
    for &handle in handles {
        range(handle, 1)?;
    }
    super::boundary(|| {
        let mut mixtures = Vec::new();
        mixtures
            .try_reserve_exact(count)
            .map_err(|_| ModelError("stream allocation failed"))?;
        for &handle in handles {
            // SAFETY: Validated readable child retained by caller throughout this call.
            mixtures.push(unsafe { (*handle).parameters.clone() });
        }
        let value = StreamParameters { weight, mixtures };
        value.validate()?;
        Ok(value)
    })
}
/// Construct native equally sized flat-start emissions, including an empty stream.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_new_f64(
    emissions: usize,
    components: usize,
    dimensions: usize,
    output: *mut *mut LiblrhsmmRsStreamF64,
) -> u32 {
    // SAFETY: Validated live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStreamF64 {
                parameters: StreamParameters::new(emissions, components, dimensions)?,
            })))
        })
    }
}

/// Copy complete weight and emission parameters; failures preserve existing storage.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_create_f64(
    weight: f64,
    emissions: *const *const LiblrhsmmRsGaussianF64,
    count: usize,
    output: *mut *mut LiblrhsmmRsStreamF64,
) -> u32 {
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Borrowed independent child array follows the input contract.
    let value = match unsafe { input_f64(weight, emissions, count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Validated live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStreamF64 {
                parameters: value,
            })))
        })
    }
}

/// Copy complete weight and emission parameters; failures preserve existing storage.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_replace_f64(
    handle: *mut LiblrhsmmRsStreamF64,
    weight: f64,
    emissions: *const *const LiblrhsmmRsGaussianF64,
    count: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Borrowed independent child array follows the input contract.
    let value = match unsafe { input_f64(weight, emissions, count) } {
        Ok(value) => value,
        Err(status) => return status,
    };
    // SAFETY: Exclusive owner; all input validation and copying completed.
    unsafe {
        (*handle).parameters = value;
    }
    0
}

/// Read the complete stream weight and emission count.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_layout_f64(
    handle: *const LiblrhsmmRsStreamF64,
    output: *mut LiblrhsmmRsStreamLayoutF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Validated live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(LiblrhsmmRsStreamLayoutF64 {
                weight: (*handle).parameters.weight,
                emissions: (*handle).parameters.mixtures.len(),
            })
        })
    }
}

/// Replace the stream weight without changing any emission.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_set_weight_f64(
    handle: *mut LiblrhsmmRsStreamF64,
    weight: f64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Exclusive live owner. Arbitrary scalar bits are retained.
    unsafe {
        (*handle).parameters.weight = weight;
    }
    0
}

/// Clone one emission into an independent editable Gaussian owner.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_get_emission_f64(
    handle: *const LiblrhsmmRsStreamF64,
    index: usize,
    output: *mut *mut LiblrhsmmRsGaussianF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(output, 1) {
        return status;
    }
    // SAFETY: Live readable owner, independent output.
    let Some(value) = (unsafe { &(*handle).parameters.mixtures }).get(index) else {
        return 2;
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

/// Replace one emission with a deep copy; its shape may change.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_set_emission_f64(
    handle: *mut LiblrhsmmRsStreamF64,
    index: usize,
    emission: *const LiblrhsmmRsGaussianF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    if let Err(status) = range(emission, 1) {
        return status;
    }
    // SAFETY: Exclusive stream and independent readable Gaussian input.
    if index >= unsafe { (*handle).parameters.mixtures.len() } {
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
    // SAFETY: Checked index, exclusive owner; input copy is complete.
    unsafe {
        (&mut (*handle).parameters.mixtures)[index] = value;
    }
    0
}

/// Clone the weight and every Gaussian child into an independent owner.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_clone_f64(
    handle: *const LiblrhsmmRsStreamF64,
    output: *mut *mut LiblrhsmmRsStreamF64,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    // SAFETY: Validated live inputs and independent writable output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsStreamF64 {
                parameters: (*handle).parameters.clone(),
            })))
        })
    }
}

/// Release a unique stream and all its owned children; clear the slot.
/// # Safety
/// Handles are live owners of the declared precision. Input arrays and ownership
/// slots are initialized and readable. Outputs are aligned independently writable
/// storage and cannot alias owners or inputs. Mutation and release require exclusive
/// access; no concurrent mutation or release overlaps reads. Child inputs are borrowed
/// only during the call and may repeat; returned children are independent deep copies.
/// Release consumes unique ownership and clears its slot. Null ownership is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_release_f64(
    handle: *mut *mut LiblrhsmmRsStreamF64,
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
