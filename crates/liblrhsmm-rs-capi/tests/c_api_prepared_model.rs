use liblrhsmm_rs::model::{DurationParameters, ModelParameters};
use liblrhsmm_rs_capi::*;
use std::ptr::{null, null_mut};
#[path = "common/emissions.rs"]
#[allow(dead_code)]
mod emissions;
unsafe fn compare_f32(handle: *const LiblrhsmmRsModelF32, expected: &ModelParameters<f32>) {
    // SAFETY: Live readable model and independent outputs; all returned owners are released.
    unsafe {
        let mut layout = LiblrhsmmRsModelLayout {
            streams: 99,
            durations: 99,
        };
        assert_eq!(liblrhsmm_rs_model_layout_f32(handle, &mut layout), 0);
        assert_eq!(
            layout,
            LiblrhsmmRsModelLayout {
                streams: expected.streams.len(),
                durations: expected.durations.len()
            }
        );
        for (index, native) in expected.streams.iter().enumerate() {
            let mut stream = null_mut();
            assert_eq!(
                liblrhsmm_rs_model_get_stream_f32(handle, index, &mut stream),
                0
            );
            let mut shape = LiblrhsmmRsStreamLayoutF32 {
                weight: 99.0,
                emissions: 99,
            };
            assert_eq!(liblrhsmm_rs_stream_layout_f32(stream, &mut shape), 0);
            assert_eq!(shape.weight.to_bits(), native.weight.to_bits());
            assert_eq!(shape.emissions, native.mixtures.len());
            for (emission, expected) in native.mixtures.iter().enumerate() {
                let mut gaussian = null_mut();
                assert_eq!(
                    liblrhsmm_rs_stream_get_emission_f32(stream, emission, &mut gaussian),
                    0
                );
                let mut shape = LiblrhsmmRsGaussianLayout {
                    mixtures: 99,
                    dimensions: 99,
                };
                assert_eq!(liblrhsmm_rs_gaussian_layout_f32(gaussian, &mut shape), 0);
                assert_eq!(shape.mixtures, expected.weights.len());
                assert_eq!(shape.dimensions, expected.dimensions);
                for (selector, values) in [
                    &expected.weights,
                    &expected.means,
                    &expected.variances,
                    &expected.variance_floors,
                ]
                .into_iter()
                .enumerate()
                {
                    let mut actual = vec![f32::MAX; values.len()];
                    assert_eq!(
                        liblrhsmm_rs_gaussian_copy_field_f32(
                            gaussian,
                            selector as u32,
                            0,
                            actual.as_mut_ptr(),
                            actual.len()
                        ),
                        0
                    );
                    assert_eq!(
                        actual.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
                        values.iter().map(|x| x.to_bits()).collect::<Vec<_>>()
                    );
                }
                assert_eq!(liblrhsmm_rs_gaussian_release_f32(&mut gaussian), 0);
            }
            assert_eq!(liblrhsmm_rs_stream_release_f32(&mut stream), 0);
        }
        for (index, native) in expected.durations.iter().enumerate() {
            let mut duration = null_mut();
            assert_eq!(
                liblrhsmm_rs_model_get_duration_f32(handle, index, &mut duration),
                0
            );
            let mut fields = LiblrhsmmRsDurationParametersF32 {
                mean: 99.0,
                variance: 99.0,
                minimum: 99,
                maximum: 99,
                fixed_mean: 99,
                variance_floor: 99.0,
            };
            assert_eq!(liblrhsmm_rs_duration_get_f32(duration, &mut fields), 0);
            assert_eq!(fields.mean.to_bits(), native.mean.to_bits());
            assert_eq!(fields.variance.to_bits(), native.variance.to_bits());
            assert_eq!(fields.minimum, native.minimum);
            assert_eq!(fields.maximum, native.maximum);
            assert_eq!(fields.fixed_mean, native.fixed_mean);
            assert_eq!(
                fields.variance_floor.to_bits(),
                native.variance_floor.to_bits()
            );
            assert_eq!(liblrhsmm_rs_duration_release_f32(&mut duration), 0);
        }
    }
}

unsafe fn compare_f64(handle: *const LiblrhsmmRsModelF64, expected: &ModelParameters<f64>) {
    // SAFETY: Live readable model and independent outputs; all returned owners are released.
    unsafe {
        let mut layout = LiblrhsmmRsModelLayout {
            streams: 99,
            durations: 99,
        };
        assert_eq!(liblrhsmm_rs_model_layout_f64(handle, &mut layout), 0);
        assert_eq!(
            layout,
            LiblrhsmmRsModelLayout {
                streams: expected.streams.len(),
                durations: expected.durations.len()
            }
        );
        for (index, native) in expected.streams.iter().enumerate() {
            let mut stream = null_mut();
            assert_eq!(
                liblrhsmm_rs_model_get_stream_f64(handle, index, &mut stream),
                0
            );
            let mut shape = LiblrhsmmRsStreamLayoutF64 {
                weight: 99.0,
                emissions: 99,
            };
            assert_eq!(liblrhsmm_rs_stream_layout_f64(stream, &mut shape), 0);
            assert_eq!(shape.weight.to_bits(), native.weight.to_bits());
            assert_eq!(shape.emissions, native.mixtures.len());
            for (emission, expected) in native.mixtures.iter().enumerate() {
                let mut gaussian = null_mut();
                assert_eq!(
                    liblrhsmm_rs_stream_get_emission_f64(stream, emission, &mut gaussian),
                    0
                );
                let mut shape = LiblrhsmmRsGaussianLayout {
                    mixtures: 99,
                    dimensions: 99,
                };
                assert_eq!(liblrhsmm_rs_gaussian_layout_f64(gaussian, &mut shape), 0);
                assert_eq!(shape.mixtures, expected.weights.len());
                assert_eq!(shape.dimensions, expected.dimensions);
                for (selector, values) in [
                    &expected.weights,
                    &expected.means,
                    &expected.variances,
                    &expected.variance_floors,
                ]
                .into_iter()
                .enumerate()
                {
                    let mut actual = vec![f64::MAX; values.len()];
                    assert_eq!(
                        liblrhsmm_rs_gaussian_copy_field_f64(
                            gaussian,
                            selector as u32,
                            0,
                            actual.as_mut_ptr(),
                            actual.len()
                        ),
                        0
                    );
                    assert_eq!(
                        actual.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
                        values.iter().map(|x| x.to_bits()).collect::<Vec<_>>()
                    );
                }
                assert_eq!(liblrhsmm_rs_gaussian_release_f64(&mut gaussian), 0);
            }
            assert_eq!(liblrhsmm_rs_stream_release_f64(&mut stream), 0);
        }
        for (index, native) in expected.durations.iter().enumerate() {
            let mut duration = null_mut();
            assert_eq!(
                liblrhsmm_rs_model_get_duration_f64(handle, index, &mut duration),
                0
            );
            let mut fields = LiblrhsmmRsDurationParametersF64 {
                mean: 99.0,
                variance: 99.0,
                minimum: 99,
                maximum: 99,
                fixed_mean: 99,
                variance_floor: 99.0,
            };
            assert_eq!(liblrhsmm_rs_duration_get_f64(duration, &mut fields), 0);
            assert_eq!(fields.mean.to_bits(), native.mean.to_bits());
            assert_eq!(fields.variance.to_bits(), native.variance.to_bits());
            assert_eq!(fields.minimum, native.minimum);
            assert_eq!(fields.maximum, native.maximum);
            assert_eq!(fields.fixed_mean, native.fixed_mean);
            assert_eq!(
                fields.variance_floor.to_bits(),
                native.variance_floor.to_bits()
            );
            assert_eq!(liblrhsmm_rs_duration_release_f64(&mut duration), 0);
        }
    }
}

unsafe fn create_f32(expected: &ModelParameters<f32>) -> *mut LiblrhsmmRsModelF32 {
    // SAFETY: Independent input descriptors; child arrays remain live through construction, then uniquely released.
    unsafe {
        let mut streams = Vec::new();
        let mut durations = Vec::new();
        for stream in &expected.streams {
            let mut emissions = Vec::new();
            for distribution in &stream.mixtures {
                let input = LiblrhsmmRsGaussianParametersF32 {
                    mixtures: distribution.weights.len(),
                    dimensions: distribution.dimensions,
                    weights: distribution.weights.as_ptr(),
                    means: distribution.means.as_ptr(),
                    variances: distribution.variances.as_ptr(),
                    variance_floors: distribution.variance_floors.as_ptr(),
                };
                let mut child = null_mut();
                assert_eq!(liblrhsmm_rs_gaussian_create_f32(&input, &mut child), 0);
                emissions.push(child);
            }
            let children: Vec<_> = emissions
                .iter()
                .map(|pointer| pointer.cast_const())
                .collect();
            let mut child = null_mut();
            assert_eq!(
                liblrhsmm_rs_stream_create_f32(
                    stream.weight,
                    children.as_ptr(),
                    children.len(),
                    &mut child
                ),
                0
            );
            for mut child in emissions {
                assert_eq!(liblrhsmm_rs_gaussian_release_f32(&mut child), 0);
            }
            streams.push(child);
        }
        for duration in &expected.durations {
            let input = LiblrhsmmRsDurationParametersF32 {
                mean: duration.mean,
                variance: duration.variance,
                minimum: duration.minimum,
                maximum: duration.maximum,
                fixed_mean: duration.fixed_mean,
                variance_floor: duration.variance_floor,
            };
            let mut child = null_mut();
            assert_eq!(liblrhsmm_rs_duration_create_f32(&input, &mut child), 0);
            durations.push(child);
        }
        let inputs: Vec<_> = streams.iter().map(|pointer| pointer.cast_const()).collect();
        let duration_inputs: Vec<_> = durations
            .iter()
            .map(|pointer| pointer.cast_const())
            .collect();
        let mut model = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_create_f32(
                inputs.as_ptr(),
                inputs.len(),
                duration_inputs.as_ptr(),
                duration_inputs.len(),
                &mut model
            ),
            0
        );
        for mut child in streams {
            assert_eq!(liblrhsmm_rs_stream_release_f32(&mut child), 0);
        }
        for mut child in durations {
            assert_eq!(liblrhsmm_rs_duration_release_f32(&mut child), 0);
        }
        model
    }
}

#[test]
fn prepared_model_f32_full_snapshots_selected_states_caches_and_errors() {
    // SAFETY: Live uniquely released owners and independent initialized output buffers.
    unsafe {
        let (mut expected, observation, _) = emissions::sample_with_precision::<f32>();
        expected.streams[0].mixtures[0].means[0] = -0.125;
        expected.streams[0].mixtures[0].variance_floors[0] = -0.0;
        let mut source = create_f32(&expected);
        let mut prepared = null_mut();
        let mut cloned = null_mut();
        let mut editable = null_mut();
        let options = LiblrhsmmRsDurationPreparationF32 {
            duration_cache_length: 3,
            duration_weight: 0.75,
            temperature: 0.5,
        };
        assert_eq!(
            liblrhsmm_rs_model_prepare_f32(source, &options, &mut prepared),
            0
        );
        assert_eq!(
            liblrhsmm_rs_prepared_model_parameters_f32(prepared, &mut editable),
            0
        );
        compare_f32(editable, &expected);
        assert_eq!(liblrhsmm_rs_model_release_f32(&mut editable), 0);
        assert_eq!(
            liblrhsmm_rs_prepared_model_clone_f32(prepared, &mut cloned),
            0
        );
        let mut layout = LiblrhsmmRsModelLayout {
            streams: 99,
            durations: 99,
        };
        assert_eq!(
            liblrhsmm_rs_prepared_model_layout_f32(prepared, &mut layout),
            0
        );
        assert_eq!(
            layout,
            LiblrhsmmRsModelLayout {
                streams: 2,
                durations: 3
            }
        );
        let mut fetched = LiblrhsmmRsDurationPreparationF32 {
            duration_cache_length: 99,
            duration_weight: 99.0,
            temperature: 99.0,
        };
        assert_eq!(
            liblrhsmm_rs_prepared_model_options_f32(prepared, &mut fetched),
            0
        );
        assert_eq!(fetched, options);
        for (stream_index, stream) in expected.streams.iter().enumerate() {
            let mut shape = LiblrhsmmRsStreamLayoutF32 {
                weight: 99.0,
                emissions: 99,
            };
            assert_eq!(
                liblrhsmm_rs_prepared_model_stream_layout_f32(prepared, stream_index, &mut shape),
                0
            );
            assert_eq!(shape.weight.to_bits(), stream.weight.to_bits());
            assert_eq!(shape.emissions, stream.mixtures.len());
            let mut stream_copy = null_mut();
            assert_eq!(
                liblrhsmm_rs_prepared_model_stream_parameters_f32(
                    prepared,
                    stream_index,
                    &mut stream_copy
                ),
                0
            );
            assert_eq!(liblrhsmm_rs_stream_set_weight_f32(stream_copy, -99.0), 0);
            assert_eq!(liblrhsmm_rs_stream_release_f32(&mut stream_copy), 0);
            for (state, distribution) in stream.mixtures.iter().enumerate() {
                let mut gaussian = null_mut();
                assert_eq!(
                    liblrhsmm_rs_prepared_model_get_emission_f32(
                        prepared,
                        stream_index,
                        state,
                        &mut gaussian
                    ),
                    0
                );
                let native = distribution.prepare().unwrap();
                let mut length = 99;
                assert_eq!(
                    liblrhsmm_rs_prepared_gaussian_cache_length_f32(gaussian, &mut length),
                    0
                );
                let mut cache = vec![99.0; length];
                assert_eq!(
                    liblrhsmm_rs_prepared_gaussian_cache_copy_f32(
                        gaussian,
                        0,
                        cache.as_mut_ptr(),
                        length
                    ),
                    0
                );
                assert_eq!(cache, native.cached_log_normalizers());
                for frame in 0..observation.frames {
                    for temperature in [0.0, 0.75, 1.0] {
                        let input = observation.frame(stream_index, frame).unwrap();
                        let mut actual = 99.0;
                        assert_eq!(
                            liblrhsmm_rs_prepared_gaussian_log_probability_f32(
                                gaussian,
                                input.as_ptr(),
                                input.len(),
                                temperature,
                                &mut actual
                            ),
                            0
                        );
                        assert_eq!(
                            actual.to_bits(),
                            native
                                .log_probability(input, temperature)
                                .unwrap()
                                .to_bits()
                        );
                    }
                }
                assert_eq!(liblrhsmm_rs_prepared_gaussian_release_f32(&mut gaussian), 0);
            }
        }
        for (state, distribution) in expected.durations.iter().enumerate() {
            let mut available = 99;
            assert_eq!(
                liblrhsmm_rs_prepared_model_has_duration_f32(prepared, state, &mut available),
                0
            );
            assert_eq!(available, 1);
            let mut duration = null_mut();
            assert_eq!(
                liblrhsmm_rs_prepared_model_get_duration_f32(prepared, state, &mut duration),
                0
            );
            let native = distribution.prepare(options.into()).unwrap();
            let mut length = 99;
            assert_eq!(
                liblrhsmm_rs_prepared_duration_cache_length_f32(duration, &mut length),
                0
            );
            assert_eq!(length, 3);
            let mut cache = [99.0; 3];
            assert_eq!(
                liblrhsmm_rs_prepared_duration_cache_copy_f32(duration, 0, cache.as_mut_ptr(), 3),
                0
            );
            assert_eq!(cache, native.cached_log_probabilities());
            for frames in [0, 1, 2, 4, 300, 32767, i32::MAX as usize] {
                let mut actual = 99.0;
                assert_eq!(
                    liblrhsmm_rs_prepared_duration_log_probability_f32(
                        duration,
                        frames,
                        &mut actual
                    ),
                    0
                );
                assert_eq!(
                    actual.to_bits(),
                    native.log_probability(frames).unwrap().to_bits()
                );
            }
            assert_eq!(liblrhsmm_rs_prepared_duration_release_f32(&mut duration), 0);
        }
        // Source mutation/release cannot affect either prepared owner or returned parameter copies.
        assert_eq!(
            liblrhsmm_rs_model_replace_f32(source, null(), 0, null(), 0),
            0
        );
        assert_eq!(liblrhsmm_rs_model_release_f32(&mut source), 0);
        assert_eq!(liblrhsmm_rs_prepared_model_release_f32(&mut prepared), 0);
        assert_eq!(
            liblrhsmm_rs_prepared_model_parameters_f32(cloned, &mut editable),
            0
        );
        compare_f32(editable, &expected);
        assert_eq!(liblrhsmm_rs_model_release_f32(&mut editable), 0);
        let mut gaussian = null_mut();
        let mut duration = null_mut();
        assert_eq!(
            liblrhsmm_rs_prepared_model_get_emission_f32(cloned, 2, 0, &mut gaussian),
            2
        );
        assert_eq!(
            liblrhsmm_rs_prepared_model_get_emission_f32(cloned, 0, 3, &mut gaussian),
            2
        );
        assert_eq!(
            liblrhsmm_rs_prepared_model_get_duration_f32(cloned, 3, &mut duration),
            2
        );
        let mut available = 77;
        assert_eq!(
            liblrhsmm_rs_prepared_model_has_duration_f32(cloned, 3, &mut available),
            2
        );
        assert_eq!(available, 77);
        assert_eq!(
            liblrhsmm_rs_prepared_model_options_f32(cloned, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_prepared_model_layout_f32(null(), &mut layout),
            1
        );
        assert_eq!(
            liblrhsmm_rs_prepared_model_get_duration_f32(cloned, 0, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_prepared_model_get_emission_f32(cloned, 0, 0, &mut gaussian),
            0
        );
        assert_eq!(
            liblrhsmm_rs_prepared_model_get_duration_f32(cloned, 0, &mut duration),
            0
        );
        assert_eq!(liblrhsmm_rs_prepared_model_release_f32(&mut cloned), 0);
        let mut normalizer = 99.0;
        assert_eq!(
            liblrhsmm_rs_prepared_duration_log_normalizer_f32(duration, &mut normalizer),
            0
        );
        assert_eq!(
            normalizer.to_bits(),
            expected.durations[0]
                .prepare(options.into())
                .unwrap()
                .log_normalizer()
                .to_bits()
        );
        assert_eq!(liblrhsmm_rs_prepared_gaussian_release_f32(&mut gaussian), 0);
        assert_eq!(liblrhsmm_rs_prepared_duration_release_f32(&mut duration), 0);
        expected.durations.push(DurationParameters::default());
        source = create_f32(&expected);
        assert_eq!(
            liblrhsmm_rs_model_prepare_f32(source, &options, &mut prepared),
            3
        );
        assert!(prepared.is_null());
        let states = [0, 2, 0];
        assert_eq!(
            liblrhsmm_rs_model_prepare_selected_durations_f32(
                source,
                &options,
                states.as_ptr(),
                3,
                &mut prepared
            ),
            0
        );
        assert_eq!(
            liblrhsmm_rs_prepared_model_has_duration_f32(prepared, 1, &mut available),
            0
        );
        assert_eq!(available, 0);
        assert_eq!(
            liblrhsmm_rs_prepared_model_get_duration_f32(prepared, 1, &mut duration),
            3
        );
        assert!(duration.is_null());
        assert_eq!(
            liblrhsmm_rs_prepared_model_get_duration_f32(prepared, 3, &mut duration),
            3
        );
        assert_eq!(liblrhsmm_rs_prepared_model_release_f32(&mut prepared), 0);
        let invalid = [4];
        assert_eq!(
            liblrhsmm_rs_model_prepare_selected_durations_f32(
                source,
                &options,
                invalid.as_ptr(),
                1,
                &mut prepared
            ),
            3
        );
        assert_eq!(
            liblrhsmm_rs_model_prepare_selected_durations_f32(
                source,
                &options,
                null(),
                1,
                &mut prepared
            ),
            1
        );
        assert_eq!(
            liblrhsmm_rs_model_prepare_selected_durations_f32(
                source,
                &options,
                null(),
                0,
                &mut prepared
            ),
            0
        );
        assert_eq!(liblrhsmm_rs_prepared_model_release_f32(&mut prepared), 0);
        let bad = LiblrhsmmRsDurationPreparationF32 {
            duration_weight: -1.0,
            ..options
        };
        assert_eq!(
            liblrhsmm_rs_model_prepare_selected_durations_f32(
                source,
                &bad,
                null(),
                0,
                &mut prepared
            ),
            3
        );
        assert_eq!(
            liblrhsmm_rs_model_prepare_f32(source, null(), &mut prepared),
            1
        );
        assert_eq!(
            liblrhsmm_rs_model_prepare_f32(source, &options, null_mut()),
            1
        );
        assert_eq!(liblrhsmm_rs_model_release_f32(&mut source), 0);
        assert_eq!(liblrhsmm_rs_prepared_model_release_f32(&mut prepared), 0);
        assert_eq!(liblrhsmm_rs_prepared_model_release_f32(null_mut()), 1);
    }
}

unsafe fn create_f64(expected: &ModelParameters<f64>) -> *mut LiblrhsmmRsModelF64 {
    // SAFETY: Independent input descriptors; child arrays remain live through construction, then uniquely released.
    unsafe {
        let mut streams = Vec::new();
        let mut durations = Vec::new();
        for stream in &expected.streams {
            let mut emissions = Vec::new();
            for distribution in &stream.mixtures {
                let input = LiblrhsmmRsGaussianParametersF64 {
                    mixtures: distribution.weights.len(),
                    dimensions: distribution.dimensions,
                    weights: distribution.weights.as_ptr(),
                    means: distribution.means.as_ptr(),
                    variances: distribution.variances.as_ptr(),
                    variance_floors: distribution.variance_floors.as_ptr(),
                };
                let mut child = null_mut();
                assert_eq!(liblrhsmm_rs_gaussian_create_f64(&input, &mut child), 0);
                emissions.push(child);
            }
            let children: Vec<_> = emissions
                .iter()
                .map(|pointer| pointer.cast_const())
                .collect();
            let mut child = null_mut();
            assert_eq!(
                liblrhsmm_rs_stream_create_f64(
                    stream.weight,
                    children.as_ptr(),
                    children.len(),
                    &mut child
                ),
                0
            );
            for mut child in emissions {
                assert_eq!(liblrhsmm_rs_gaussian_release_f64(&mut child), 0);
            }
            streams.push(child);
        }
        for duration in &expected.durations {
            let input = LiblrhsmmRsDurationParametersF64 {
                mean: duration.mean,
                variance: duration.variance,
                minimum: duration.minimum,
                maximum: duration.maximum,
                fixed_mean: duration.fixed_mean,
                variance_floor: duration.variance_floor,
            };
            let mut child = null_mut();
            assert_eq!(liblrhsmm_rs_duration_create_f64(&input, &mut child), 0);
            durations.push(child);
        }
        let inputs: Vec<_> = streams.iter().map(|pointer| pointer.cast_const()).collect();
        let duration_inputs: Vec<_> = durations
            .iter()
            .map(|pointer| pointer.cast_const())
            .collect();
        let mut model = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_create_f64(
                inputs.as_ptr(),
                inputs.len(),
                duration_inputs.as_ptr(),
                duration_inputs.len(),
                &mut model
            ),
            0
        );
        for mut child in streams {
            assert_eq!(liblrhsmm_rs_stream_release_f64(&mut child), 0);
        }
        for mut child in durations {
            assert_eq!(liblrhsmm_rs_duration_release_f64(&mut child), 0);
        }
        model
    }
}

#[test]
fn prepared_model_f64_full_snapshots_selected_states_caches_and_errors() {
    // SAFETY: Live uniquely released owners and independent initialized output buffers.
    unsafe {
        let (mut expected, observation, _) = emissions::sample_with_precision::<f64>();
        expected.streams[0].mixtures[0].means[0] = -0.12500000000000003;
        expected.streams[0].mixtures[0].variance_floors[0] = -0.0;
        let mut source = create_f64(&expected);
        let mut prepared = null_mut();
        let mut cloned = null_mut();
        let mut editable = null_mut();
        let options = LiblrhsmmRsDurationPreparationF64 {
            duration_cache_length: 3,
            duration_weight: 0.75,
            temperature: 0.5,
        };
        assert_eq!(
            liblrhsmm_rs_model_prepare_f64(source, &options, &mut prepared),
            0
        );
        assert_eq!(
            liblrhsmm_rs_prepared_model_parameters_f64(prepared, &mut editable),
            0
        );
        compare_f64(editable, &expected);
        assert_eq!(liblrhsmm_rs_model_release_f64(&mut editable), 0);
        assert_eq!(
            liblrhsmm_rs_prepared_model_clone_f64(prepared, &mut cloned),
            0
        );
        let mut layout = LiblrhsmmRsModelLayout {
            streams: 99,
            durations: 99,
        };
        assert_eq!(
            liblrhsmm_rs_prepared_model_layout_f64(prepared, &mut layout),
            0
        );
        assert_eq!(
            layout,
            LiblrhsmmRsModelLayout {
                streams: 2,
                durations: 3
            }
        );
        let mut fetched = LiblrhsmmRsDurationPreparationF64 {
            duration_cache_length: 99,
            duration_weight: 99.0,
            temperature: 99.0,
        };
        assert_eq!(
            liblrhsmm_rs_prepared_model_options_f64(prepared, &mut fetched),
            0
        );
        assert_eq!(fetched, options);
        for (stream_index, stream) in expected.streams.iter().enumerate() {
            let mut shape = LiblrhsmmRsStreamLayoutF64 {
                weight: 99.0,
                emissions: 99,
            };
            assert_eq!(
                liblrhsmm_rs_prepared_model_stream_layout_f64(prepared, stream_index, &mut shape),
                0
            );
            assert_eq!(shape.weight.to_bits(), stream.weight.to_bits());
            assert_eq!(shape.emissions, stream.mixtures.len());
            let mut stream_copy = null_mut();
            assert_eq!(
                liblrhsmm_rs_prepared_model_stream_parameters_f64(
                    prepared,
                    stream_index,
                    &mut stream_copy
                ),
                0
            );
            assert_eq!(liblrhsmm_rs_stream_set_weight_f64(stream_copy, -99.0), 0);
            assert_eq!(liblrhsmm_rs_stream_release_f64(&mut stream_copy), 0);
            for (state, distribution) in stream.mixtures.iter().enumerate() {
                let mut gaussian = null_mut();
                assert_eq!(
                    liblrhsmm_rs_prepared_model_get_emission_f64(
                        prepared,
                        stream_index,
                        state,
                        &mut gaussian
                    ),
                    0
                );
                let native = distribution.prepare().unwrap();
                let mut length = 99;
                assert_eq!(
                    liblrhsmm_rs_prepared_gaussian_cache_length_f64(gaussian, &mut length),
                    0
                );
                let mut cache = vec![99.0; length];
                assert_eq!(
                    liblrhsmm_rs_prepared_gaussian_cache_copy_f64(
                        gaussian,
                        0,
                        cache.as_mut_ptr(),
                        length
                    ),
                    0
                );
                assert_eq!(cache, native.cached_log_normalizers());
                for frame in 0..observation.frames {
                    for temperature in [0.0, 0.75, 1.0] {
                        let input = observation.frame(stream_index, frame).unwrap();
                        let mut actual = 99.0;
                        assert_eq!(
                            liblrhsmm_rs_prepared_gaussian_log_probability_f64(
                                gaussian,
                                input.as_ptr(),
                                input.len(),
                                temperature,
                                &mut actual
                            ),
                            0
                        );
                        assert_eq!(
                            actual.to_bits(),
                            native
                                .log_probability(input, temperature)
                                .unwrap()
                                .to_bits()
                        );
                    }
                }
                assert_eq!(liblrhsmm_rs_prepared_gaussian_release_f64(&mut gaussian), 0);
            }
        }
        for (state, distribution) in expected.durations.iter().enumerate() {
            let mut available = 99;
            assert_eq!(
                liblrhsmm_rs_prepared_model_has_duration_f64(prepared, state, &mut available),
                0
            );
            assert_eq!(available, 1);
            let mut duration = null_mut();
            assert_eq!(
                liblrhsmm_rs_prepared_model_get_duration_f64(prepared, state, &mut duration),
                0
            );
            let native = distribution.prepare(options.into()).unwrap();
            let mut length = 99;
            assert_eq!(
                liblrhsmm_rs_prepared_duration_cache_length_f64(duration, &mut length),
                0
            );
            assert_eq!(length, 3);
            let mut cache = [99.0; 3];
            assert_eq!(
                liblrhsmm_rs_prepared_duration_cache_copy_f64(duration, 0, cache.as_mut_ptr(), 3),
                0
            );
            assert_eq!(cache, native.cached_log_probabilities());
            for frames in [0, 1, 2, 4, 300, 32767, i32::MAX as usize] {
                let mut actual = 99.0;
                assert_eq!(
                    liblrhsmm_rs_prepared_duration_log_probability_f64(
                        duration,
                        frames,
                        &mut actual
                    ),
                    0
                );
                assert_eq!(
                    actual.to_bits(),
                    native.log_probability(frames).unwrap().to_bits()
                );
            }
            assert_eq!(liblrhsmm_rs_prepared_duration_release_f64(&mut duration), 0);
        }
        // Source mutation/release cannot affect either prepared owner or returned parameter copies.
        assert_eq!(
            liblrhsmm_rs_model_replace_f64(source, null(), 0, null(), 0),
            0
        );
        assert_eq!(liblrhsmm_rs_model_release_f64(&mut source), 0);
        assert_eq!(liblrhsmm_rs_prepared_model_release_f64(&mut prepared), 0);
        assert_eq!(
            liblrhsmm_rs_prepared_model_parameters_f64(cloned, &mut editable),
            0
        );
        compare_f64(editable, &expected);
        assert_eq!(liblrhsmm_rs_model_release_f64(&mut editable), 0);
        let mut gaussian = null_mut();
        let mut duration = null_mut();
        assert_eq!(
            liblrhsmm_rs_prepared_model_get_emission_f64(cloned, 2, 0, &mut gaussian),
            2
        );
        assert_eq!(
            liblrhsmm_rs_prepared_model_get_emission_f64(cloned, 0, 3, &mut gaussian),
            2
        );
        assert_eq!(
            liblrhsmm_rs_prepared_model_get_duration_f64(cloned, 3, &mut duration),
            2
        );
        let mut available = 77;
        assert_eq!(
            liblrhsmm_rs_prepared_model_has_duration_f64(cloned, 3, &mut available),
            2
        );
        assert_eq!(available, 77);
        assert_eq!(
            liblrhsmm_rs_prepared_model_options_f64(cloned, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_prepared_model_layout_f64(null(), &mut layout),
            1
        );
        assert_eq!(
            liblrhsmm_rs_prepared_model_get_duration_f64(cloned, 0, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_prepared_model_get_emission_f64(cloned, 0, 0, &mut gaussian),
            0
        );
        assert_eq!(
            liblrhsmm_rs_prepared_model_get_duration_f64(cloned, 0, &mut duration),
            0
        );
        assert_eq!(liblrhsmm_rs_prepared_model_release_f64(&mut cloned), 0);
        let mut normalizer = 99.0;
        assert_eq!(
            liblrhsmm_rs_prepared_duration_log_normalizer_f64(duration, &mut normalizer),
            0
        );
        assert_eq!(
            normalizer.to_bits(),
            expected.durations[0]
                .prepare(options.into())
                .unwrap()
                .log_normalizer()
                .to_bits()
        );
        assert_eq!(liblrhsmm_rs_prepared_gaussian_release_f64(&mut gaussian), 0);
        assert_eq!(liblrhsmm_rs_prepared_duration_release_f64(&mut duration), 0);
        expected.durations.push(DurationParameters::default());
        source = create_f64(&expected);
        assert_eq!(
            liblrhsmm_rs_model_prepare_f64(source, &options, &mut prepared),
            3
        );
        assert!(prepared.is_null());
        let states = [0, 2, 0];
        assert_eq!(
            liblrhsmm_rs_model_prepare_selected_durations_f64(
                source,
                &options,
                states.as_ptr(),
                3,
                &mut prepared
            ),
            0
        );
        assert_eq!(
            liblrhsmm_rs_prepared_model_has_duration_f64(prepared, 1, &mut available),
            0
        );
        assert_eq!(available, 0);
        assert_eq!(
            liblrhsmm_rs_prepared_model_get_duration_f64(prepared, 1, &mut duration),
            3
        );
        assert!(duration.is_null());
        assert_eq!(
            liblrhsmm_rs_prepared_model_get_duration_f64(prepared, 3, &mut duration),
            3
        );
        assert_eq!(liblrhsmm_rs_prepared_model_release_f64(&mut prepared), 0);
        let invalid = [4];
        assert_eq!(
            liblrhsmm_rs_model_prepare_selected_durations_f64(
                source,
                &options,
                invalid.as_ptr(),
                1,
                &mut prepared
            ),
            3
        );
        assert_eq!(
            liblrhsmm_rs_model_prepare_selected_durations_f64(
                source,
                &options,
                null(),
                1,
                &mut prepared
            ),
            1
        );
        assert_eq!(
            liblrhsmm_rs_model_prepare_selected_durations_f64(
                source,
                &options,
                null(),
                0,
                &mut prepared
            ),
            0
        );
        assert_eq!(liblrhsmm_rs_prepared_model_release_f64(&mut prepared), 0);
        let bad = LiblrhsmmRsDurationPreparationF64 {
            duration_weight: -1.0,
            ..options
        };
        assert_eq!(
            liblrhsmm_rs_model_prepare_selected_durations_f64(
                source,
                &bad,
                null(),
                0,
                &mut prepared
            ),
            3
        );
        assert_eq!(
            liblrhsmm_rs_model_prepare_f64(source, null(), &mut prepared),
            1
        );
        assert_eq!(
            liblrhsmm_rs_model_prepare_f64(source, &options, null_mut()),
            1
        );
        assert_eq!(liblrhsmm_rs_model_release_f64(&mut source), 0);
        assert_eq!(liblrhsmm_rs_prepared_model_release_f64(&mut prepared), 0);
        assert_eq!(liblrhsmm_rs_prepared_model_release_f64(null_mut()), 1);
    }
}
