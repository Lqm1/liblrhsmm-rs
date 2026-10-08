//! Atomic paired regrouping with complete independent observation/segmentation sets.
use super::{boundary, range, scalar_result};
use crate::data_operations::{AlignedDataset, regroup_with_precision};
use std::panic::RefUnwindSafe;

unsafe fn release<T: RefUnwindSafe>(owner: *mut *mut T) -> u32 {
    if let Err(status) = range(owner, 1) {
        return status;
    }
    // SAFETY: Caller provides an initialized unique ownership slot.
    let pointer = unsafe { owner.read() };
    if pointer.is_null() {
        return 0;
    }
    if let Err(status) = range(pointer, 1) {
        return status;
    }
    match boundary(|| {
        // SAFETY: Clear the exclusively borrowed slot before dropping its unique owner.
        unsafe {
            owner.write(std::ptr::null_mut());
            drop(Box::from_raw(pointer));
        }
        Ok(())
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}

mod f32;
pub use f32::*;

mod f64;
pub use f64::*;
