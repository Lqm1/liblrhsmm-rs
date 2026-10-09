//! Caller-supplied randomness, native scalar sampling and complete observation generation.
use super::*;
use core::convert::Infallible;
use rand::TryRng;
use std::ffi::c_void;

/// Write one random word and return zero, or fail with any nonzero status.
pub type LiblrhsmmRsNextU32 = Option<unsafe extern "C" fn(*mut c_void, *mut u32) -> u32>;
/// Write one random word and return zero, or fail with any nonzero status.
pub type LiblrhsmmRsNextU64 = Option<unsafe extern "C" fn(*mut c_void, *mut u64) -> u32>;
/// Borrowed synchronous random-word source. Only the callback for the selected precision is required.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LiblrhsmmRsRandomSource {
    pub context: *mut c_void,
    pub next_u32: LiblrhsmmRsNextU32,
    pub next_u64: LiblrhsmmRsNextU64,
}

// The native distribution requires an infallible RNG. Callback failure is retained
// in this private bridge and checked before publishing a scalar or returning a
// generation draw. No synthetic value after failure reaches public results.
struct RandomBridge {
    source: LiblrhsmmRsRandomSource,
    failed: bool,
}
impl TryRng for RandomBridge {
    type Error = Infallible;
    fn try_next_u32(&mut self) -> Result<u32, Infallible> {
        let mut value = 0;
        if !self.failed {
            self.failed = match self.source.next_u32 {
                Some(callback) => unsafe { callback(self.source.context, &mut value) != 0 },
                None => true,
            };
        }
        Ok(value)
    }
    fn try_next_u64(&mut self) -> Result<u64, Infallible> {
        let mut value = 0;
        if !self.failed {
            self.failed = match self.source.next_u64 {
                Some(callback) => unsafe { callback(self.source.context, &mut value) != 0 },
                None => true,
            };
        }
        Ok(value)
    }
    fn try_fill_bytes(&mut self, output: &mut [u8]) -> Result<(), Infallible> {
        for chunk in output.chunks_mut(8) {
            let bytes = self.try_next_u64()?.to_le_bytes();
            chunk.copy_from_slice(&bytes[..chunk.len()]);
        }
        Ok(())
    }
}

/// Borrowed synchronous scalar draw source. Angular/component draws may include zero;
/// Gaussian radial draws must be in (0,1], as documented by native generation.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LiblrhsmmRsUniformSourceF32 {
    pub context: *mut c_void,
    pub draw: Option<unsafe extern "C" fn(*mut c_void, *mut f32) -> u32>,
}

/// Sample native OpenClosed01 without OS entropy. Only next_u32 is required.
/// Nonzero callback status returns three and preserves the initialized output.
/// # Safety
/// Inputs identify aligned initialized readable descriptors, arrays and live owners.
/// Output slots are independent aligned writable storage and hold no live owner.
/// Model/sequence/output cannot alias; sequence mutation and output publication
/// require exclusive access. No concurrent mutation or release overlaps reads.
/// Callbacks and their contexts remain valid for this synchronous call, must not
/// unwind or reenter owners, and cannot retain or access borrowed output pointers
/// outside the callback. Contexts may be mutated only as permitted by their caller.
/// A successful callback initializes its scalar output. Failed callbacks return
/// nonzero; no further callbacks are invoked after a failure. Empty arrays may be
/// null. Inputs remain caller-owned; generated observations own independent storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_uniform_sample_f32(
    source: *const LiblrhsmmRsRandomSource,
    output: *mut f32,
) -> u32 {
    if let Err(status) = range(source, 1) {
        return status;
    }
    let source = unsafe { source.read() };
    if source.next_u32.is_none() {
        return 1;
    }
    unsafe {
        scalar_result(output, || {
            let mut rng = RandomBridge {
                source,
                failed: false,
            };
            let value = crate::generation::uniform_with_precision::<f32, _>(&mut rng);
            if rng.failed {
                return Err(ModelError("random callback failed"));
            }
            Ok(value)
        })
    }
}

/// Run the native Gaussian sampler with supplied radial and angular draws.
/// # Safety
/// Inputs identify aligned initialized readable descriptors, arrays and live owners.
/// Output slots are independent aligned writable storage and hold no live owner.
/// Model/sequence/output cannot alias; sequence mutation and output publication
/// require exclusive access. No concurrent mutation or release overlaps reads.
/// Callbacks and their contexts remain valid for this synchronous call, must not
/// unwind or reenter owners, and cannot retain or access borrowed output pointers
/// outside the callback. Contexts may be mutated only as permitted by their caller.
/// A successful callback initializes its scalar output. Failed callbacks return
/// nonzero; no further callbacks are invoked after a failure. Empty arrays may be
/// null. Inputs remain caller-owned; generated observations own independent storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_sample_f32(
    mean: f32,
    variance: f32,
    radial: f32,
    angular: f32,
    output: *mut f32,
) -> u32 {
    unsafe {
        scalar_result(output, || {
            crate::generation::gaussian_with_precision(mean, variance, radial, angular)
        })
    }
}

/// Run native cumulative mixture choice without normalization, retaining last-index fallback.
/// # Safety
/// Inputs identify aligned initialized readable descriptors, arrays and live owners.
/// Output slots are independent aligned writable storage and hold no live owner.
/// Model/sequence/output cannot alias; sequence mutation and output publication
/// require exclusive access. No concurrent mutation or release overlaps reads.
/// Callbacks and their contexts remain valid for this synchronous call, must not
/// unwind or reenter owners, and cannot retain or access borrowed output pointers
/// outside the callback. Contexts may be mutated only as permitted by their caller.
/// A successful callback initializes its scalar output. Failed callbacks return
/// nonzero; no further callbacks are invoked after a failure. Empty arrays may be
/// null. Inputs remain caller-owned; generated observations own independent storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_choose_component_f32(
    weights: *const f32,
    count: usize,
    uniform: f32,
    output: *mut usize,
) -> u32 {
    if let Err(status) = range(weights, count) {
        return status;
    }
    let weights = if count == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(weights, count) }
    };
    unsafe {
        scalar_result(output, || {
            crate::generation::choose_with_precision(weights, uniform)
        })
    }
}

/// Generate every native stream and publish sampled sequence boundaries on success.
/// Failed draws/model validation preserve both sequence and initialized output.
/// Consumed caller randomness is not rolled back. Native duration constraints/jumps
/// are ignored during generation, as in the upstream implementation.
/// # Safety
/// Inputs identify aligned initialized readable descriptors, arrays and live owners.
/// Output slots are independent aligned writable storage and hold no live owner.
/// Model/sequence/output cannot alias; sequence mutation and output publication
/// require exclusive access. No concurrent mutation or release overlaps reads.
/// Callbacks and their contexts remain valid for this synchronous call, must not
/// unwind or reenter owners, and cannot retain or access borrowed output pointers
/// outside the callback. Contexts may be mutated only as permitted by their caller.
/// A successful callback initializes its scalar output. Failed callbacks return
/// nonzero; no further callbacks are invoked after a failure. Empty arrays may be
/// null. Inputs remain caller-owned; generated observations own independent storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_generate_with_uniform_f32(
    model: *const LiblrhsmmRsModelF32,
    sequence: *mut LiblrhsmmRsSegmentationF32,
    source: *const LiblrhsmmRsUniformSourceF32,
    output: *mut *mut LiblrhsmmRsObservationF32,
) -> u32 {
    for result in [range(model, 1), range(sequence, 1), range(source, 1)] {
        if let Err(status) = result {
            return status;
        }
    }
    let source = unsafe { source.read() };
    let Some(callback) = source.draw else {
        return 1;
    };
    unsafe {
        scalar_result(output, || {
            let mut staged = (*sequence).segmentation.clone();
            let mut failed = false;
            let observation = (*model).parameters.generate_with_uniform(&mut staged, || {
                if failed {
                    return f32::NAN;
                }
                let mut value = f32::NAN;
                failed = callback(source.context, &mut value) != 0;
                if failed { f32::NAN } else { value }
            })?;
            if failed {
                return Err(ModelError("uniform callback failed"));
            }
            let owner = Box::new(LiblrhsmmRsObservationF32 { observation });
            (*sequence).segmentation = staged;
            Ok(Box::into_raw(owner))
        })
    }
}

/// Generate using native OpenClosed01 conversion of caller-supplied random words.
/// Failed callbacks/model validation preserve sequence and initialized output.
/// Only next_u32 is required; consumed random words are not restored.
/// # Safety
/// Inputs identify aligned initialized readable descriptors, arrays and live owners.
/// Output slots are independent aligned writable storage and hold no live owner.
/// Model/sequence/output cannot alias; sequence mutation and output publication
/// require exclusive access. No concurrent mutation or release overlaps reads.
/// Callbacks and their contexts remain valid for this synchronous call, must not
/// unwind or reenter owners, and cannot retain or access borrowed output pointers
/// outside the callback. Contexts may be mutated only as permitted by their caller.
/// A successful callback initializes its scalar output. Failed callbacks return
/// nonzero; no further callbacks are invoked after a failure. Empty arrays may be
/// null. Inputs remain caller-owned; generated observations own independent storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_generate_f32(
    model: *const LiblrhsmmRsModelF32,
    sequence: *mut LiblrhsmmRsSegmentationF32,
    source: *const LiblrhsmmRsRandomSource,
    output: *mut *mut LiblrhsmmRsObservationF32,
) -> u32 {
    for result in [range(model, 1), range(sequence, 1), range(source, 1)] {
        if let Err(status) = result {
            return status;
        }
    }
    let source = unsafe { source.read() };
    if source.next_u32.is_none() {
        return 1;
    }
    unsafe {
        scalar_result(output, || {
            let mut staged = (*sequence).segmentation.clone();
            let mut rng = RandomBridge {
                source,
                failed: false,
            };
            let observation = (*model).parameters.generate_with_uniform(&mut staged, || {
                if rng.failed {
                    return f32::NAN;
                }
                let value = crate::generation::uniform_with_precision::<f32, _>(&mut rng);
                if rng.failed { f32::NAN } else { value }
            })?;
            if rng.failed {
                return Err(ModelError("random callback failed"));
            }
            let owner = Box::new(LiblrhsmmRsObservationF32 { observation });
            (*sequence).segmentation = staged;
            Ok(Box::into_raw(owner))
        })
    }
}

/// Borrowed synchronous scalar draw source. Angular/component draws may include zero;
/// Gaussian radial draws must be in (0,1], as documented by native generation.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LiblrhsmmRsUniformSourceF64 {
    pub context: *mut c_void,
    pub draw: Option<unsafe extern "C" fn(*mut c_void, *mut f64) -> u32>,
}

/// Sample native OpenClosed01 without OS entropy. Only next_u64 is required.
/// Nonzero callback status returns three and preserves the initialized output.
/// # Safety
/// Inputs identify aligned initialized readable descriptors, arrays and live owners.
/// Output slots are independent aligned writable storage and hold no live owner.
/// Model/sequence/output cannot alias; sequence mutation and output publication
/// require exclusive access. No concurrent mutation or release overlaps reads.
/// Callbacks and their contexts remain valid for this synchronous call, must not
/// unwind or reenter owners, and cannot retain or access borrowed output pointers
/// outside the callback. Contexts may be mutated only as permitted by their caller.
/// A successful callback initializes its scalar output. Failed callbacks return
/// nonzero; no further callbacks are invoked after a failure. Empty arrays may be
/// null. Inputs remain caller-owned; generated observations own independent storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_uniform_sample_f64(
    source: *const LiblrhsmmRsRandomSource,
    output: *mut f64,
) -> u32 {
    if let Err(status) = range(source, 1) {
        return status;
    }
    let source = unsafe { source.read() };
    if source.next_u64.is_none() {
        return 1;
    }
    unsafe {
        scalar_result(output, || {
            let mut rng = RandomBridge {
                source,
                failed: false,
            };
            let value = crate::generation::uniform_with_precision::<f64, _>(&mut rng);
            if rng.failed {
                return Err(ModelError("random callback failed"));
            }
            Ok(value)
        })
    }
}

/// Run the native Gaussian sampler with supplied radial and angular draws.
/// # Safety
/// Inputs identify aligned initialized readable descriptors, arrays and live owners.
/// Output slots are independent aligned writable storage and hold no live owner.
/// Model/sequence/output cannot alias; sequence mutation and output publication
/// require exclusive access. No concurrent mutation or release overlaps reads.
/// Callbacks and their contexts remain valid for this synchronous call, must not
/// unwind or reenter owners, and cannot retain or access borrowed output pointers
/// outside the callback. Contexts may be mutated only as permitted by their caller.
/// A successful callback initializes its scalar output. Failed callbacks return
/// nonzero; no further callbacks are invoked after a failure. Empty arrays may be
/// null. Inputs remain caller-owned; generated observations own independent storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_gaussian_sample_f64(
    mean: f64,
    variance: f64,
    radial: f64,
    angular: f64,
    output: *mut f64,
) -> u32 {
    unsafe {
        scalar_result(output, || {
            crate::generation::gaussian_with_precision(mean, variance, radial, angular)
        })
    }
}

/// Run native cumulative mixture choice without normalization, retaining last-index fallback.
/// # Safety
/// Inputs identify aligned initialized readable descriptors, arrays and live owners.
/// Output slots are independent aligned writable storage and hold no live owner.
/// Model/sequence/output cannot alias; sequence mutation and output publication
/// require exclusive access. No concurrent mutation or release overlaps reads.
/// Callbacks and their contexts remain valid for this synchronous call, must not
/// unwind or reenter owners, and cannot retain or access borrowed output pointers
/// outside the callback. Contexts may be mutated only as permitted by their caller.
/// A successful callback initializes its scalar output. Failed callbacks return
/// nonzero; no further callbacks are invoked after a failure. Empty arrays may be
/// null. Inputs remain caller-owned; generated observations own independent storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_choose_component_f64(
    weights: *const f64,
    count: usize,
    uniform: f64,
    output: *mut usize,
) -> u32 {
    if let Err(status) = range(weights, count) {
        return status;
    }
    let weights = if count == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(weights, count) }
    };
    unsafe {
        scalar_result(output, || {
            crate::generation::choose_with_precision(weights, uniform)
        })
    }
}

/// Generate every native stream and publish sampled sequence boundaries on success.
/// Failed draws/model validation preserve both sequence and initialized output.
/// Consumed caller randomness is not rolled back. Native duration constraints/jumps
/// are ignored during generation, as in the upstream implementation.
/// # Safety
/// Inputs identify aligned initialized readable descriptors, arrays and live owners.
/// Output slots are independent aligned writable storage and hold no live owner.
/// Model/sequence/output cannot alias; sequence mutation and output publication
/// require exclusive access. No concurrent mutation or release overlaps reads.
/// Callbacks and their contexts remain valid for this synchronous call, must not
/// unwind or reenter owners, and cannot retain or access borrowed output pointers
/// outside the callback. Contexts may be mutated only as permitted by their caller.
/// A successful callback initializes its scalar output. Failed callbacks return
/// nonzero; no further callbacks are invoked after a failure. Empty arrays may be
/// null. Inputs remain caller-owned; generated observations own independent storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_generate_with_uniform_f64(
    model: *const LiblrhsmmRsModelF64,
    sequence: *mut LiblrhsmmRsSegmentationF64,
    source: *const LiblrhsmmRsUniformSourceF64,
    output: *mut *mut LiblrhsmmRsObservationF64,
) -> u32 {
    for result in [range(model, 1), range(sequence, 1), range(source, 1)] {
        if let Err(status) = result {
            return status;
        }
    }
    let source = unsafe { source.read() };
    let Some(callback) = source.draw else {
        return 1;
    };
    unsafe {
        scalar_result(output, || {
            let mut staged = (*sequence).segmentation.clone();
            let mut failed = false;
            let observation = (*model).parameters.generate_with_uniform(&mut staged, || {
                if failed {
                    return f64::NAN;
                }
                let mut value = f64::NAN;
                failed = callback(source.context, &mut value) != 0;
                if failed { f64::NAN } else { value }
            })?;
            if failed {
                return Err(ModelError("uniform callback failed"));
            }
            let owner = Box::new(LiblrhsmmRsObservationF64 { observation });
            (*sequence).segmentation = staged;
            Ok(Box::into_raw(owner))
        })
    }
}

/// Generate using native OpenClosed01 conversion of caller-supplied random words.
/// Failed callbacks/model validation preserve sequence and initialized output.
/// Only next_u64 is required; consumed random words are not restored.
/// # Safety
/// Inputs identify aligned initialized readable descriptors, arrays and live owners.
/// Output slots are independent aligned writable storage and hold no live owner.
/// Model/sequence/output cannot alias; sequence mutation and output publication
/// require exclusive access. No concurrent mutation or release overlaps reads.
/// Callbacks and their contexts remain valid for this synchronous call, must not
/// unwind or reenter owners, and cannot retain or access borrowed output pointers
/// outside the callback. Contexts may be mutated only as permitted by their caller.
/// A successful callback initializes its scalar output. Failed callbacks return
/// nonzero; no further callbacks are invoked after a failure. Empty arrays may be
/// null. Inputs remain caller-owned; generated observations own independent storage.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_generate_f64(
    model: *const LiblrhsmmRsModelF64,
    sequence: *mut LiblrhsmmRsSegmentationF64,
    source: *const LiblrhsmmRsRandomSource,
    output: *mut *mut LiblrhsmmRsObservationF64,
) -> u32 {
    for result in [range(model, 1), range(sequence, 1), range(source, 1)] {
        if let Err(status) = result {
            return status;
        }
    }
    let source = unsafe { source.read() };
    if source.next_u64.is_none() {
        return 1;
    }
    unsafe {
        scalar_result(output, || {
            let mut staged = (*sequence).segmentation.clone();
            let mut rng = RandomBridge {
                source,
                failed: false,
            };
            let observation = (*model).parameters.generate_with_uniform(&mut staged, || {
                if rng.failed {
                    return f64::NAN;
                }
                let value = crate::generation::uniform_with_precision::<f64, _>(&mut rng);
                if rng.failed { f64::NAN } else { value }
            })?;
            if rng.failed {
                return Err(ModelError("random callback failed"));
            }
            let owner = Box::new(LiblrhsmmRsObservationF64 { observation });
            (*sequence).segmentation = staged;
            Ok(Box::into_raw(owner))
        })
    }
}
