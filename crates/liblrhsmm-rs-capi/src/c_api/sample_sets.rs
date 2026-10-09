//! Complete sample sets and incremental builders with independent owned samples.
use super::{boundary, range, scalar_result};
use crate::data::{ObservationData, SegmentationData};
use crate::data_operations::SampleSetBuilder;
use crate::{ModelError, scalar::Scalar};
use std::panic::RefUnwindSafe;

trait Sample: Clone + RefUnwindSafe {
    fn validate(&self) -> Result<(), ModelError>;
}
impl<T: Scalar + RefUnwindSafe> Sample for ObservationData<T> {
    fn validate(&self) -> Result<(), ModelError> {
        ObservationData::validate(self)
    }
}
impl<T: Scalar + RefUnwindSafe> Sample for SegmentationData<T> {
    fn validate(&self) -> Result<(), ModelError> {
        SegmentationData::validate(self)
    }
}
trait SampleOwner: RefUnwindSafe {
    type Data: Sample;
    fn sample(&self) -> &Self::Data;
}
unsafe fn copied_child<H: SampleOwner>(owner: *const H) -> Result<H::Data, u32> {
    range(owner, 1)?;
    boundary(|| {
        // SAFETY: Public caller supplies a live readable independent child owner.
        let sample = unsafe { &*owner }.sample();
        sample.validate()?;
        Ok(sample.clone())
    })
}
unsafe fn copied_input<H: SampleOwner>(
    input: *const *const H,
    count: usize,
) -> Result<Vec<H::Data>, u32> {
    range(input, count)?;
    // SAFETY: Public caller supplies initialized readable pointers for count entries.
    let owners = if count == 0 {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(input, count) }
    };
    for &owner in owners {
        range(owner, 1)?;
    }
    boundary(|| {
        let mut samples = Vec::new();
        samples
            .try_reserve_exact(count)
            .map_err(|_| ModelError("sample set allocation failed"))?;
        for &owner in owners {
            // SAFETY: All child owners were checked; caller guarantees live readable allocation validity.
            let sample = unsafe { &*owner }.sample();
            sample.validate()?;
            samples.push(sample.clone());
        }
        Ok(samples)
    })
}
fn resize_builder<T: Sample>(
    builder: &mut SampleSetBuilder<T>,
    count: usize,
) -> Result<(), ModelError> {
    if count > builder.samples.len() {
        builder
            .samples
            .try_reserve_exact(count - builder.samples.len())
            .map_err(|_| ModelError("sample slot allocation failed"))?;
    }
    builder.samples.resize_with(count, || None);
    Ok(())
}
unsafe fn release<H: RefUnwindSafe>(owner: *mut *mut H) -> u32 {
    range(owner.cast_const(), 1).map_or_else(
        |status| status,
        |()| {
            // SAFETY: Public caller supplies initialized exclusively writable unique ownership.
            let pointer = unsafe { owner.read() };
            if pointer.is_null() {
                return 0;
            }
            if let Err(status) = range(pointer, 1) {
                return status;
            }
            match boundary(|| {
                // SAFETY: Public caller transfers ownership; clear before dropping the allocation.
                unsafe {
                    owner.write(std::ptr::null_mut());
                    drop(Box::from_raw(pointer));
                }
                Ok(())
            }) {
                Ok(()) => 0,
                Err(status) => status,
            }
        },
    )
}

mod observation_f32;
pub use observation_f32::*;

mod observation_f64;
pub use observation_f64::*;

mod segmentation_f32;
pub use segmentation_f32::*;

mod segmentation_f64;
pub use segmentation_f64::*;
