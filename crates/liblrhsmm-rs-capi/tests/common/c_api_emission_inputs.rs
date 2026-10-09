use super::common;
use liblrhsmm_rs_capi::*;

use std::ptr::null_mut;

pub(super) unsafe fn inputs_f32() -> (
    *mut LiblrhsmmRsModelF32,
    *mut LiblrhsmmRsObservationF32,
    *mut LiblrhsmmRsSegmentationF32,
) {
    let (model, observation, segmentation) = common::sample_with_precision::<f32>();
    unsafe {
        let mut streams = Vec::new();
        for native in &model.streams {
            let mut emissions = Vec::new();
            for mixture in &native.mixtures {
                let descriptor = LiblrhsmmRsGaussianParametersF32 {
                    mixtures: mixture.weights.len(),
                    dimensions: mixture.dimensions,
                    weights: mixture.weights.as_ptr(),
                    means: mixture.means.as_ptr(),
                    variances: mixture.variances.as_ptr(),
                    variance_floors: mixture.variance_floors.as_ptr(),
                };
                let mut handle = null_mut();
                assert_eq!(
                    liblrhsmm_rs_gaussian_create_f32(&descriptor, &mut handle),
                    0
                );
                emissions.push(handle);
            }
            let borrowed: Vec<_> = emissions.iter().map(|p| p.cast_const()).collect();
            let mut stream = null_mut();
            assert_eq!(
                liblrhsmm_rs_stream_create_f32(
                    native.weight,
                    borrowed.as_ptr(),
                    borrowed.len(),
                    &mut stream
                ),
                0
            );
            for handle in &mut emissions {
                assert_eq!(liblrhsmm_rs_gaussian_release_f32(handle), 0);
            }
            streams.push(stream);
        }
        let mut durations = Vec::new();
        for native in &model.durations {
            let descriptor = LiblrhsmmRsDurationParametersF32::from(native);
            let mut handle = null_mut();
            assert_eq!(
                liblrhsmm_rs_duration_create_f32(&descriptor, &mut handle),
                0
            );
            durations.push(handle);
        }
        let borrowed_streams: Vec<_> = streams.iter().map(|p| p.cast_const()).collect();
        let borrowed_durations: Vec<_> = durations.iter().map(|p| p.cast_const()).collect();
        let mut model_handle = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_create_f32(
                borrowed_streams.as_ptr(),
                streams.len(),
                borrowed_durations.as_ptr(),
                durations.len(),
                &mut model_handle
            ),
            0
        );
        for handle in &mut streams {
            assert_eq!(liblrhsmm_rs_stream_release_f32(handle), 0);
        }
        for handle in &mut durations {
            assert_eq!(liblrhsmm_rs_duration_release_f32(handle), 0);
        }
        let mut observation_streams = Vec::new();
        for native in &observation.streams {
            let mut handle = null_mut();
            assert_eq!(
                liblrhsmm_rs_observation_stream_create_f32(
                    native.dimensions,
                    native.values.as_ptr(),
                    native.values.len(),
                    &mut handle
                ),
                0
            );
            observation_streams.push(handle);
        }
        let borrowed: Vec<_> = observation_streams.iter().map(|p| p.cast_const()).collect();
        let mut observation_handle = null_mut();
        assert_eq!(
            liblrhsmm_rs_observation_create_f32(
                observation.frames,
                borrowed.as_ptr(),
                borrowed.len(),
                &mut observation_handle
            ),
            0
        );
        for handle in &mut observation_streams {
            assert_eq!(liblrhsmm_rs_observation_stream_release_f32(handle), 0);
        }
        let states: Vec<_> = segmentation
            .output_states
            .iter()
            .map(|row| row.as_ptr())
            .collect();
        let jumps: Vec<Vec<_>> = segmentation
            .outgoing
            .iter()
            .map(|row| {
                row.iter()
                    .map(|v| LiblrhsmmRsJumpF32 {
                        delta: v.delta,
                        probability: v.probability,
                    })
                    .collect()
            })
            .collect();
        let groups: Vec<_> = jumps
            .iter()
            .map(|row| LiblrhsmmRsJumpSliceF32 {
                jumps: row.as_ptr(),
                count: row.len(),
            })
            .collect();
        let mut seg = null_mut();
        assert_eq!(
            liblrhsmm_rs_segmentation_create_f32(
                2,
                3,
                segmentation.boundaries.as_ptr(),
                segmentation.duration_states.as_ptr(),
                states.as_ptr(),
                groups.as_ptr(),
                &mut seg
            ),
            0
        );
        (model_handle, observation_handle, seg)
    }
}
pub(super) unsafe fn inputs_f64() -> (
    *mut LiblrhsmmRsModelF64,
    *mut LiblrhsmmRsObservationF64,
    *mut LiblrhsmmRsSegmentationF64,
) {
    let (model, observation, segmentation) = common::sample_with_precision::<f64>();
    unsafe {
        let mut streams = Vec::new();
        for native in &model.streams {
            let mut emissions = Vec::new();
            for mixture in &native.mixtures {
                let descriptor = LiblrhsmmRsGaussianParametersF64 {
                    mixtures: mixture.weights.len(),
                    dimensions: mixture.dimensions,
                    weights: mixture.weights.as_ptr(),
                    means: mixture.means.as_ptr(),
                    variances: mixture.variances.as_ptr(),
                    variance_floors: mixture.variance_floors.as_ptr(),
                };
                let mut handle = null_mut();
                assert_eq!(
                    liblrhsmm_rs_gaussian_create_f64(&descriptor, &mut handle),
                    0
                );
                emissions.push(handle);
            }
            let borrowed: Vec<_> = emissions.iter().map(|p| p.cast_const()).collect();
            let mut stream = null_mut();
            assert_eq!(
                liblrhsmm_rs_stream_create_f64(
                    native.weight,
                    borrowed.as_ptr(),
                    borrowed.len(),
                    &mut stream
                ),
                0
            );
            for handle in &mut emissions {
                assert_eq!(liblrhsmm_rs_gaussian_release_f64(handle), 0);
            }
            streams.push(stream);
        }
        let mut durations = Vec::new();
        for native in &model.durations {
            let descriptor = LiblrhsmmRsDurationParametersF64::from(native);
            let mut handle = null_mut();
            assert_eq!(
                liblrhsmm_rs_duration_create_f64(&descriptor, &mut handle),
                0
            );
            durations.push(handle);
        }
        let borrowed_streams: Vec<_> = streams.iter().map(|p| p.cast_const()).collect();
        let borrowed_durations: Vec<_> = durations.iter().map(|p| p.cast_const()).collect();
        let mut model_handle = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_create_f64(
                borrowed_streams.as_ptr(),
                streams.len(),
                borrowed_durations.as_ptr(),
                durations.len(),
                &mut model_handle
            ),
            0
        );
        for handle in &mut streams {
            assert_eq!(liblrhsmm_rs_stream_release_f64(handle), 0);
        }
        for handle in &mut durations {
            assert_eq!(liblrhsmm_rs_duration_release_f64(handle), 0);
        }
        let mut observation_streams = Vec::new();
        for native in &observation.streams {
            let mut handle = null_mut();
            assert_eq!(
                liblrhsmm_rs_observation_stream_create_f64(
                    native.dimensions,
                    native.values.as_ptr(),
                    native.values.len(),
                    &mut handle
                ),
                0
            );
            observation_streams.push(handle);
        }
        let borrowed: Vec<_> = observation_streams.iter().map(|p| p.cast_const()).collect();
        let mut observation_handle = null_mut();
        assert_eq!(
            liblrhsmm_rs_observation_create_f64(
                observation.frames,
                borrowed.as_ptr(),
                borrowed.len(),
                &mut observation_handle
            ),
            0
        );
        for handle in &mut observation_streams {
            assert_eq!(liblrhsmm_rs_observation_stream_release_f64(handle), 0);
        }
        let states: Vec<_> = segmentation
            .output_states
            .iter()
            .map(|row| row.as_ptr())
            .collect();
        let jumps: Vec<Vec<_>> = segmentation
            .outgoing
            .iter()
            .map(|row| {
                row.iter()
                    .map(|v| LiblrhsmmRsJumpF64 {
                        delta: v.delta,
                        probability: v.probability,
                    })
                    .collect()
            })
            .collect();
        let groups: Vec<_> = jumps
            .iter()
            .map(|row| LiblrhsmmRsJumpSliceF64 {
                jumps: row.as_ptr(),
                count: row.len(),
            })
            .collect();
        let mut seg = null_mut();
        assert_eq!(
            liblrhsmm_rs_segmentation_create_f64(
                2,
                3,
                segmentation.boundaries.as_ptr(),
                segmentation.duration_states.as_ptr(),
                states.as_ptr(),
                groups.as_ptr(),
                &mut seg
            ),
            0
        );
        (model_handle, observation_handle, seg)
    }
}
