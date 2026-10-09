//! Atomic length changes for public optional builder slot lists.
use super::{
    LiblrhsmmRsModelBuilderF32, LiblrhsmmRsModelBuilderF64, LiblrhsmmRsStreamBuilderF32,
    LiblrhsmmRsStreamBuilderF64, boundary, range,
};
use crate::model::{ModelParametersBuilder, StreamParametersBuilder};
use crate::scalar::Scalar;

fn resize_stream<T: Scalar>(
    builder: &mut StreamParametersBuilder<T>,
    emissions: usize,
) -> Result<(), crate::ModelError> {
    let mut replacement = StreamParametersBuilder::new(emissions)?;
    replacement.weight = builder.weight;
    for (target, source) in replacement.emissions.iter_mut().zip(&builder.emissions) {
        target.clone_from(source);
    }
    *builder = replacement;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{DurationParameters, GaussianMixtureParameters, StreamParameters};

    fn verify<T: Scalar>() {
        let gaussian = GaussianMixtureParameters::new(1, 2).unwrap();
        let mut stream = StreamParametersBuilder::new(2).unwrap();
        stream.weight = T::from_f64(0.75);
        stream.emissions[0] = Some(gaussian.clone());
        resize_stream(&mut stream, 4).unwrap();
        assert_eq!(stream.weight, T::from_f64(0.75));
        assert_eq!(stream.emissions[0], Some(gaussian));
        assert!(stream.emissions[1..].iter().all(Option::is_none));
        let before = stream.clone();
        assert!(resize_stream(&mut stream, i32::MAX as usize + 1).is_err());
        assert_eq!(stream, before);
        resize_stream(&mut stream, 0).unwrap();
        assert!(stream.emissions.is_empty());
        assert_eq!(stream.weight, T::from_f64(0.75));

        let mut model = ModelParametersBuilder::<T>::new(2, 2).unwrap();
        model.streams[0] = Some(StreamParameters::new(1, 1, 2).unwrap());
        model.durations[0] = Some(DurationParameters::default());
        resize_model(&mut model, 4, 3).unwrap();
        let before = model.clone();
        for (streams, durations) in [(i32::MAX as usize + 1, 1), (1, i32::MAX as usize + 1)] {
            assert!(resize_model(&mut model, streams, durations).is_err());
            assert_eq!(model, before);
        }
        assert!(model.streams[0].is_some() && model.durations[0].is_some());
        assert!(model.streams[1..].iter().all(Option::is_none));
        assert!(model.durations[1..].iter().all(Option::is_none));
        resize_model(&mut model, 1, 1).unwrap();
        assert_eq!(model.streams[0], before.streams[0]);
        assert_eq!(model.durations[0], before.durations[0]);
        resize_model(&mut model, 0, 0).unwrap();
        assert!(model.streams.is_empty() && model.durations.is_empty());
    }

    #[test]
    fn binary32_resize_retains_fields_and_is_atomic() {
        verify::<f32>();
    }
    #[test]
    fn binary64_resize_retains_fields_and_is_atomic() {
        verify::<f64>();
    }
}
fn resize_model<T: Scalar>(
    builder: &mut ModelParametersBuilder<T>,
    streams: usize,
    durations: usize,
) -> Result<(), crate::ModelError> {
    let mut replacement = ModelParametersBuilder::new(streams, durations)?;
    for (target, source) in replacement.streams.iter_mut().zip(&builder.streams) {
        target.clone_from(source);
    }
    for (target, source) in replacement.durations.iter_mut().zip(&builder.durations) {
        target.clone_from(source);
    }
    *builder = replacement;
    Ok(())
}

/// Resize emission slots, preserving weight and the overlapping slot prefix.
/// New slots are empty; removed slots are dropped. Errors preserve the builder.
/// # Safety
/// Handle is a live uniquely accessible builder; no mutation or release overlaps.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_builder_resize_f32(
    handle: *mut LiblrhsmmRsStreamBuilderF32,
    emissions: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    boundary(|| {
        // SAFETY: Caller provides exclusive access to the validated live owner.
        unsafe { resize_stream(&mut (*handle).builder, emissions) }
    })
    .map_or_else(|status| status, |()| 0)
}

/// Resize emission slots, preserving weight and the overlapping slot prefix.
/// New slots are empty; removed slots are dropped. Errors preserve the builder.
/// # Safety
/// Handle is a live uniquely accessible builder; no mutation or release overlaps.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_stream_builder_resize_f64(
    handle: *mut LiblrhsmmRsStreamBuilderF64,
    emissions: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    boundary(|| {
        // SAFETY: Caller provides exclusive access to the validated live owner.
        unsafe { resize_stream(&mut (*handle).builder, emissions) }
    })
    .map_or_else(|status| status, |()| 0)
}

/// Resize stream and duration slots atomically, preserving both overlapping prefixes.
/// New slots are empty; removed slots are dropped. Errors preserve both lists.
/// # Safety
/// Handle is a live uniquely accessible builder; no mutation or release overlaps.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_builder_resize_f32(
    handle: *mut LiblrhsmmRsModelBuilderF32,
    streams: usize,
    durations: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    boundary(|| {
        // SAFETY: Caller provides exclusive access to the validated live owner.
        unsafe { resize_model(&mut (*handle).builder, streams, durations) }
    })
    .map_or_else(|status| status, |()| 0)
}

/// Resize stream and duration slots atomically, preserving both overlapping prefixes.
/// New slots are empty; removed slots are dropped. Errors preserve both lists.
/// # Safety
/// Handle is a live uniquely accessible builder; no mutation or release overlaps.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_model_builder_resize_f64(
    handle: *mut LiblrhsmmRsModelBuilderF64,
    streams: usize,
    durations: usize,
) -> u32 {
    if let Err(status) = range(handle, 1) {
        return status;
    }
    boundary(|| {
        // SAFETY: Caller provides exclusive access to the validated live owner.
        unsafe { resize_model(&mut (*handle).builder, streams, durations) }
    })
    .map_or_else(|status| status, |()| 0)
}
