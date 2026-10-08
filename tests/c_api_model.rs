#![cfg(feature = "c-api")]
use liblrhsmm_rs::c_api::*;
use liblrhsmm_rs::model::{DurationParameters, ModelParameters, StreamLayout, StreamParameters};
use std::ptr::{null, null_mut};

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

#[test]
fn model_f32_complete_fields_c_construction_and_independent_builders() {
    // SAFETY: Initialized independent buffers and live owners; all mutation exclusive and releases unique.
    unsafe {
        let layouts = [
            LiblrhsmmRsStreamSpecification {
                emissions: 2,
                components: 3,
                dimensions: 2,
            },
            LiblrhsmmRsStreamSpecification {
                emissions: 1,
                components: 2,
                dimensions: 1,
            },
        ];
        let native_layouts = layouts.map(StreamLayout::from);
        let mut model = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_new_f32(layouts.as_ptr(), 2, 3, &mut model),
            0
        );
        compare_f32(
            model,
            &ModelParameters::<f32>::new(&native_layouts, 3).unwrap(),
        );
        #[cfg(feature = "serialization")]
        compare_f32(
            model,
            &ModelParameters::<f32>::read_from(
                include_bytes!("fixtures/c-construction-model.hsmm").as_slice(),
            )
            .unwrap(),
        );
        let sentinel = model;
        let mut failure = sentinel;
        assert_eq!(liblrhsmm_rs_model_new_f32(null(), 2, 3, &mut failure), 1);
        assert_eq!(failure, sentinel);
        assert_eq!(
            liblrhsmm_rs_model_new_f32(null(), 0, i32::MAX as usize + 1, &mut failure),
            3
        );
        assert_eq!(failure, sentinel);
        let invalid_layout = LiblrhsmmRsStreamSpecification {
            emissions: 1,
            components: 0,
            dimensions: 1,
        };
        assert_eq!(
            liblrhsmm_rs_model_new_f32(&invalid_layout, 1, 0, &mut failure),
            3
        );
        assert_eq!(failure, sentinel);
        assert_eq!(
            liblrhsmm_rs_model_new_f32(layouts.as_ptr(), 2, 3, null_mut()),
            1
        );

        let mut first = null_mut();
        let mut second = null_mut();
        let mut duration = null_mut();
        assert_eq!(liblrhsmm_rs_stream_new_f32(2, 1, 2, &mut first), 0);
        assert_eq!(liblrhsmm_rs_stream_new_f32(1, 2, 3, &mut second), 0);
        let weight = f32::from_bits(0x7fc01234);
        assert_eq!(liblrhsmm_rs_stream_set_weight_f32(first, weight), 0);
        assert_eq!(liblrhsmm_rs_stream_set_weight_f32(second, -0.0), 0);
        // Put a differently sized emission inside the first stream.
        let mut gaussian = null_mut();
        assert_eq!(
            liblrhsmm_rs_stream_get_emission_f32(second, 0, &mut gaussian),
            0
        );
        let values = [
            f32::from_bits(0x7fc01234),
            -0.0,
            f32::INFINITY,
            -2.5,
            0.125,
            3.25,
        ];
        for field in 0..4 {
            let count = if field == 0 { 2 } else { 6 };
            assert_eq!(
                liblrhsmm_rs_gaussian_set_field_f32(gaussian, field, 0, values.as_ptr(), count),
                0
            );
        }
        assert_eq!(liblrhsmm_rs_stream_set_emission_f32(first, 1, gaussian), 0);
        let mut a = StreamParameters::<f32>::new(2, 1, 2).unwrap();
        let mut b = StreamParameters::<f32>::new(1, 2, 3).unwrap();
        a.weight = weight;
        b.weight = -0.0;
        let mut changed = b.mixtures[0].clone();
        changed.weights.copy_from_slice(&values[..2]);
        changed.means.copy_from_slice(&values);
        changed.variances.copy_from_slice(&values);
        changed.variance_floors.copy_from_slice(&values);
        a.mixtures[1] = changed;
        assert_eq!(liblrhsmm_rs_gaussian_release_f32(&mut gaussian), 0);
        let descriptor = LiblrhsmmRsDurationParametersF32 {
            mean: weight,
            variance: -0.0,
            minimum: i32::MIN,
            maximum: i32::MAX,
            fixed_mean: 7,
            variance_floor: f32::INFINITY,
        };
        assert_eq!(
            liblrhsmm_rs_duration_create_f32(&descriptor, &mut duration),
            0
        );
        let d = DurationParameters::<f32>::from(descriptor);
        let streams = [first.cast_const(), second.cast_const(), first.cast_const()];
        let durations = [duration.cast_const(), duration.cast_const()];
        let mut expected = ModelParameters {
            streams: vec![a.clone(), b.clone(), a.clone()],
            durations: vec![d.clone(), d.clone()],
        };
        assert_eq!(
            liblrhsmm_rs_model_replace_f32(model, streams.as_ptr(), 3, durations.as_ptr(), 2),
            0
        );
        compare_f32(model, &expected);
        let mut created = null_mut();
        let mut clone = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_create_f32(streams.as_ptr(), 3, durations.as_ptr(), 2, &mut created),
            0
        );
        assert_eq!(liblrhsmm_rs_model_clone_f32(model, &mut clone), 0);
        assert_eq!(liblrhsmm_rs_model_set_stream_f32(model, 0, second), 0);
        expected.streams[0] = b.clone();
        let revised = LiblrhsmmRsDurationParametersF32 {
            mean: 5.0,
            variance: 2.0,
            minimum: 1,
            maximum: 9,
            fixed_mean: -1,
            variance_floor: 0.25,
        };
        assert_eq!(liblrhsmm_rs_duration_set_f32(duration, &revised), 0);
        assert_eq!(liblrhsmm_rs_model_set_duration_f32(model, 0, duration), 0);
        expected.durations[0] = revised.into();
        compare_f32(model, &expected);
        let original = ModelParameters {
            streams: vec![a.clone(), b.clone(), a.clone()],
            durations: vec![d.clone(), d.clone()],
        };
        compare_f32(clone, &original);
        compare_f32(created, &original);
        let mut returned_stream = null_mut();
        let mut returned_duration = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_get_stream_f32(model, 1, &mut returned_stream),
            0
        );
        assert_eq!(liblrhsmm_rs_stream_set_weight_f32(returned_stream, 99.0), 0);
        assert_eq!(
            liblrhsmm_rs_model_get_duration_f32(model, 1, &mut returned_duration),
            0
        );
        assert_eq!(
            liblrhsmm_rs_duration_set_f32(returned_duration, &revised),
            0
        );
        compare_f32(model, &expected);
        assert_eq!(liblrhsmm_rs_stream_release_f32(&mut returned_stream), 0);
        assert_eq!(liblrhsmm_rs_duration_release_f32(&mut returned_duration), 0);
        assert_eq!(
            liblrhsmm_rs_model_get_stream_f32(model, 3, &mut returned_stream),
            2
        );
        assert_eq!(
            liblrhsmm_rs_model_get_duration_f32(model, 2, &mut returned_duration),
            2
        );
        assert!(returned_stream.is_null() && returned_duration.is_null());
        assert_eq!(liblrhsmm_rs_model_set_stream_f32(model, 3, first), 2);
        assert_eq!(liblrhsmm_rs_model_set_duration_f32(model, 2, duration), 2);
        assert_eq!(liblrhsmm_rs_model_set_stream_f32(model, 0, null()), 1);
        assert_eq!(liblrhsmm_rs_model_set_duration_f32(model, 0, null()), 1);
        let bad_streams = [first.cast_const(), null()];
        let bad_durations = [duration.cast_const(), null()];
        assert_eq!(
            liblrhsmm_rs_model_replace_f32(model, bad_streams.as_ptr(), 2, durations.as_ptr(), 2),
            1
        );
        assert_eq!(
            liblrhsmm_rs_model_replace_f32(model, streams.as_ptr(), 3, bad_durations.as_ptr(), 2),
            1
        );
        compare_f32(model, &expected);
        assert_eq!(
            liblrhsmm_rs_model_create_f32(null(), 1, durations.as_ptr(), 2, &mut failure),
            1
        );
        assert_eq!(failure, sentinel);
        assert_eq!(
            liblrhsmm_rs_model_create_f32(
                streams.as_ptr(),
                3,
                null(),
                i32::MAX as usize + 1,
                &mut failure
            ),
            2
        );
        assert_eq!(failure, sentinel);
        assert_eq!(liblrhsmm_rs_model_clone_f32(model, null_mut()), 1);

        let mut builder = null_mut();
        assert_eq!(liblrhsmm_rs_model_builder_new_f32(2, 2, &mut builder), 0);
        let mut layout = LiblrhsmmRsModelLayout {
            streams: 99,
            durations: 99,
        };
        assert_eq!(
            liblrhsmm_rs_model_builder_layout_f32(builder, &mut layout),
            0
        );
        assert_eq!(
            layout,
            LiblrhsmmRsModelLayout {
                streams: 2,
                durations: 2
            }
        );
        let mut present = 99;
        assert_eq!(
            liblrhsmm_rs_model_builder_has_stream_f32(builder, 0, &mut present),
            0
        );
        assert_eq!(present, 0);
        assert_eq!(
            liblrhsmm_rs_model_builder_has_duration_f32(builder, 0, &mut present),
            0
        );
        assert_eq!(present, 0);
        returned_stream = first;
        returned_duration = duration;
        assert_eq!(
            liblrhsmm_rs_model_builder_get_stream_f32(builder, 0, &mut returned_stream),
            3
        );
        assert_eq!(returned_stream, first);
        assert_eq!(
            liblrhsmm_rs_model_builder_get_duration_f32(builder, 0, &mut returned_duration),
            3
        );
        assert_eq!(returned_duration, duration);
        assert_eq!(
            liblrhsmm_rs_model_builder_build_f32(builder, &mut failure),
            3
        );
        assert_eq!(failure, sentinel);
        assert_eq!(
            liblrhsmm_rs_model_builder_set_stream_f32(builder, 0, first),
            0
        );
        assert_eq!(
            liblrhsmm_rs_model_builder_set_stream_f32(builder, 1, second),
            0
        );
        assert_eq!(
            liblrhsmm_rs_model_builder_build_f32(builder, &mut failure),
            3
        );
        assert_eq!(failure, sentinel);
        assert_eq!(liblrhsmm_rs_duration_set_f32(duration, &descriptor), 0);
        assert_eq!(
            liblrhsmm_rs_model_builder_set_duration_f32(builder, 0, duration),
            0
        );
        assert_eq!(
            liblrhsmm_rs_model_builder_set_duration_f32(builder, 1, duration),
            0
        );
        assert_eq!(
            liblrhsmm_rs_model_builder_has_stream_f32(builder, 0, &mut present),
            0
        );
        assert_eq!(present, 1);
        assert_eq!(
            liblrhsmm_rs_model_builder_has_duration_f32(builder, 0, &mut present),
            0
        );
        assert_eq!(present, 1);
        let mut saved = null_mut();
        let mut built = null_mut();
        assert_eq!(liblrhsmm_rs_model_builder_clone_f32(builder, &mut saved), 0);
        assert_eq!(liblrhsmm_rs_model_builder_build_f32(builder, &mut built), 0);
        let built_expected = ModelParameters {
            streams: vec![a.clone(), b.clone()],
            durations: vec![d.clone(), d.clone()],
        };
        compare_f32(built, &built_expected);
        returned_stream = null_mut();
        returned_duration = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_builder_get_stream_f32(builder, 0, &mut returned_stream),
            0
        );
        assert_eq!(liblrhsmm_rs_stream_set_weight_f32(returned_stream, 99.0), 0);
        assert_eq!(
            liblrhsmm_rs_model_builder_get_duration_f32(builder, 0, &mut returned_duration),
            0
        );
        assert_eq!(
            liblrhsmm_rs_duration_set_f32(returned_duration, &revised),
            0
        );
        assert_eq!(liblrhsmm_rs_stream_release_f32(&mut returned_stream), 0);
        assert_eq!(liblrhsmm_rs_duration_release_f32(&mut returned_duration), 0);
        assert_eq!(liblrhsmm_rs_model_builder_clear_stream_f32(builder, 0), 0);
        assert_eq!(
            liblrhsmm_rs_model_builder_build_f32(builder, &mut failure),
            3
        );
        assert_eq!(failure, sentinel);
        assert_eq!(
            liblrhsmm_rs_model_builder_set_stream_f32(builder, 0, second),
            0
        );
        assert_eq!(liblrhsmm_rs_model_builder_clear_duration_f32(builder, 1), 0);
        assert_eq!(
            liblrhsmm_rs_model_builder_build_f32(builder, &mut failure),
            3
        );
        assert_eq!(failure, sentinel);
        assert_eq!(liblrhsmm_rs_model_builder_clear_duration_f32(builder, 1), 0);
        present = 99;
        assert_eq!(
            liblrhsmm_rs_model_builder_has_stream_f32(builder, 2, &mut present),
            2
        );
        assert_eq!(present, 99);
        assert_eq!(
            liblrhsmm_rs_model_builder_has_duration_f32(builder, 2, &mut present),
            2
        );
        assert_eq!(present, 99);
        assert_eq!(
            liblrhsmm_rs_model_builder_get_stream_f32(builder, 2, &mut returned_stream),
            2
        );
        assert_eq!(
            liblrhsmm_rs_model_builder_get_duration_f32(builder, 2, &mut returned_duration),
            2
        );
        assert_eq!(
            liblrhsmm_rs_model_builder_set_stream_f32(builder, 2, first),
            2
        );
        assert_eq!(
            liblrhsmm_rs_model_builder_set_duration_f32(builder, 2, duration),
            2
        );
        assert_eq!(liblrhsmm_rs_model_builder_clear_stream_f32(builder, 2), 2);
        assert_eq!(liblrhsmm_rs_model_builder_clear_duration_f32(builder, 2), 2);
        assert_eq!(
            liblrhsmm_rs_model_builder_set_stream_f32(builder, 0, null()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_model_builder_set_duration_f32(builder, 0, null()),
            1
        );
        assert_eq!(liblrhsmm_rs_model_builder_build_f32(builder, null_mut()), 1);
        let mut bad_builder = builder;
        assert_eq!(
            liblrhsmm_rs_model_builder_new_f32(i32::MAX as usize + 1, 0, &mut bad_builder),
            3
        );
        assert_eq!(bad_builder, builder);
        assert_eq!(
            liblrhsmm_rs_model_builder_new_f32(0, i32::MAX as usize + 1, &mut bad_builder),
            3
        );
        assert_eq!(bad_builder, builder);
        assert_eq!(liblrhsmm_rs_stream_release_f32(&mut first), 0);
        assert_eq!(liblrhsmm_rs_stream_release_f32(&mut second), 0);
        assert_eq!(liblrhsmm_rs_duration_release_f32(&mut duration), 0);
        assert_eq!(liblrhsmm_rs_model_builder_release_f32(&mut builder), 0);
        assert!(builder.is_null());
        assert_eq!(liblrhsmm_rs_model_builder_release_f32(&mut builder), 0);
        let mut rebuilt = null_mut();
        assert_eq!(liblrhsmm_rs_model_builder_build_f32(saved, &mut rebuilt), 0);
        compare_f32(rebuilt, &built_expected);
        compare_f32(built, &built_expected);
        assert_eq!(liblrhsmm_rs_model_builder_release_f32(&mut saved), 0);
        compare_f32(model, &expected);
        for mut handle in [model, clone, created, built, rebuilt] {
            assert_eq!(liblrhsmm_rs_model_release_f32(&mut handle), 0);
            assert!(handle.is_null());
            assert_eq!(liblrhsmm_rs_model_release_f32(&mut handle), 0);
        }
        assert_eq!(
            liblrhsmm_rs_model_create_f32(null(), 0, null(), 0, &mut model),
            0
        );
        compare_f32(model, &ModelParameters::default());
        assert_eq!(
            liblrhsmm_rs_model_replace_f32(model, null(), 0, null(), 0),
            0
        );
        assert_eq!(liblrhsmm_rs_model_release_f32(&mut model), 0);
        assert_eq!(liblrhsmm_rs_model_new_f32(null(), 0, 0, &mut model), 0);
        compare_f32(model, &ModelParameters::default());
        assert_eq!(liblrhsmm_rs_model_release_f32(&mut model), 0);
        assert_eq!(liblrhsmm_rs_model_builder_new_f32(0, 0, &mut builder), 0);
        assert_eq!(liblrhsmm_rs_model_builder_build_f32(builder, &mut model), 0);
        compare_f32(model, &ModelParameters::default());
        assert_eq!(liblrhsmm_rs_model_release_f32(&mut model), 0);
        assert_eq!(liblrhsmm_rs_model_builder_release_f32(&mut builder), 0);
        assert_eq!(liblrhsmm_rs_model_release_f32(null_mut()), 1);
        assert_eq!(liblrhsmm_rs_model_builder_release_f32(null_mut()), 1);
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

#[test]
fn model_f64_complete_fields_c_construction_and_independent_builders() {
    // SAFETY: Initialized independent buffers and live owners; all mutation exclusive and releases unique.
    unsafe {
        let layouts = [
            LiblrhsmmRsStreamSpecification {
                emissions: 2,
                components: 3,
                dimensions: 2,
            },
            LiblrhsmmRsStreamSpecification {
                emissions: 1,
                components: 2,
                dimensions: 1,
            },
        ];
        let native_layouts = layouts.map(StreamLayout::from);
        let mut model = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_new_f64(layouts.as_ptr(), 2, 3, &mut model),
            0
        );
        compare_f64(
            model,
            &ModelParameters::<f64>::new(&native_layouts, 3).unwrap(),
        );

        let sentinel = model;
        let mut failure = sentinel;
        assert_eq!(liblrhsmm_rs_model_new_f64(null(), 2, 3, &mut failure), 1);
        assert_eq!(failure, sentinel);
        assert_eq!(
            liblrhsmm_rs_model_new_f64(null(), 0, i32::MAX as usize + 1, &mut failure),
            3
        );
        assert_eq!(failure, sentinel);
        let invalid_layout = LiblrhsmmRsStreamSpecification {
            emissions: 1,
            components: 0,
            dimensions: 1,
        };
        assert_eq!(
            liblrhsmm_rs_model_new_f64(&invalid_layout, 1, 0, &mut failure),
            3
        );
        assert_eq!(failure, sentinel);
        assert_eq!(
            liblrhsmm_rs_model_new_f64(layouts.as_ptr(), 2, 3, null_mut()),
            1
        );

        let mut first = null_mut();
        let mut second = null_mut();
        let mut duration = null_mut();
        assert_eq!(liblrhsmm_rs_stream_new_f64(2, 1, 2, &mut first), 0);
        assert_eq!(liblrhsmm_rs_stream_new_f64(1, 2, 3, &mut second), 0);
        let weight = f64::from_bits(0x7ff8000000001234);
        assert_eq!(liblrhsmm_rs_stream_set_weight_f64(first, weight), 0);
        assert_eq!(liblrhsmm_rs_stream_set_weight_f64(second, -0.0), 0);
        // Put a differently sized emission inside the first stream.
        let mut gaussian = null_mut();
        assert_eq!(
            liblrhsmm_rs_stream_get_emission_f64(second, 0, &mut gaussian),
            0
        );
        let values = [
            f64::from_bits(0x7ff8000000001234),
            -0.0,
            f64::INFINITY,
            -2.5,
            0.125,
            3.25,
        ];
        for field in 0..4 {
            let count = if field == 0 { 2 } else { 6 };
            assert_eq!(
                liblrhsmm_rs_gaussian_set_field_f64(gaussian, field, 0, values.as_ptr(), count),
                0
            );
        }
        assert_eq!(liblrhsmm_rs_stream_set_emission_f64(first, 1, gaussian), 0);
        let mut a = StreamParameters::<f64>::new(2, 1, 2).unwrap();
        let mut b = StreamParameters::<f64>::new(1, 2, 3).unwrap();
        a.weight = weight;
        b.weight = -0.0;
        let mut changed = b.mixtures[0].clone();
        changed.weights.copy_from_slice(&values[..2]);
        changed.means.copy_from_slice(&values);
        changed.variances.copy_from_slice(&values);
        changed.variance_floors.copy_from_slice(&values);
        a.mixtures[1] = changed;
        assert_eq!(liblrhsmm_rs_gaussian_release_f64(&mut gaussian), 0);
        let descriptor = LiblrhsmmRsDurationParametersF64 {
            mean: weight,
            variance: -0.0,
            minimum: i32::MIN,
            maximum: i32::MAX,
            fixed_mean: 7,
            variance_floor: f64::INFINITY,
        };
        assert_eq!(
            liblrhsmm_rs_duration_create_f64(&descriptor, &mut duration),
            0
        );
        let d = DurationParameters::<f64>::from(descriptor);
        let streams = [first.cast_const(), second.cast_const(), first.cast_const()];
        let durations = [duration.cast_const(), duration.cast_const()];
        let mut expected = ModelParameters {
            streams: vec![a.clone(), b.clone(), a.clone()],
            durations: vec![d.clone(), d.clone()],
        };
        assert_eq!(
            liblrhsmm_rs_model_replace_f64(model, streams.as_ptr(), 3, durations.as_ptr(), 2),
            0
        );
        compare_f64(model, &expected);
        let mut created = null_mut();
        let mut clone = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_create_f64(streams.as_ptr(), 3, durations.as_ptr(), 2, &mut created),
            0
        );
        assert_eq!(liblrhsmm_rs_model_clone_f64(model, &mut clone), 0);
        assert_eq!(liblrhsmm_rs_model_set_stream_f64(model, 0, second), 0);
        expected.streams[0] = b.clone();
        let revised = LiblrhsmmRsDurationParametersF64 {
            mean: 5.0,
            variance: 2.0,
            minimum: 1,
            maximum: 9,
            fixed_mean: -1,
            variance_floor: 0.25,
        };
        assert_eq!(liblrhsmm_rs_duration_set_f64(duration, &revised), 0);
        assert_eq!(liblrhsmm_rs_model_set_duration_f64(model, 0, duration), 0);
        expected.durations[0] = revised.into();
        compare_f64(model, &expected);
        let original = ModelParameters {
            streams: vec![a.clone(), b.clone(), a.clone()],
            durations: vec![d.clone(), d.clone()],
        };
        compare_f64(clone, &original);
        compare_f64(created, &original);
        let mut returned_stream = null_mut();
        let mut returned_duration = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_get_stream_f64(model, 1, &mut returned_stream),
            0
        );
        assert_eq!(liblrhsmm_rs_stream_set_weight_f64(returned_stream, 99.0), 0);
        assert_eq!(
            liblrhsmm_rs_model_get_duration_f64(model, 1, &mut returned_duration),
            0
        );
        assert_eq!(
            liblrhsmm_rs_duration_set_f64(returned_duration, &revised),
            0
        );
        compare_f64(model, &expected);
        assert_eq!(liblrhsmm_rs_stream_release_f64(&mut returned_stream), 0);
        assert_eq!(liblrhsmm_rs_duration_release_f64(&mut returned_duration), 0);
        assert_eq!(
            liblrhsmm_rs_model_get_stream_f64(model, 3, &mut returned_stream),
            2
        );
        assert_eq!(
            liblrhsmm_rs_model_get_duration_f64(model, 2, &mut returned_duration),
            2
        );
        assert!(returned_stream.is_null() && returned_duration.is_null());
        assert_eq!(liblrhsmm_rs_model_set_stream_f64(model, 3, first), 2);
        assert_eq!(liblrhsmm_rs_model_set_duration_f64(model, 2, duration), 2);
        assert_eq!(liblrhsmm_rs_model_set_stream_f64(model, 0, null()), 1);
        assert_eq!(liblrhsmm_rs_model_set_duration_f64(model, 0, null()), 1);
        let bad_streams = [first.cast_const(), null()];
        let bad_durations = [duration.cast_const(), null()];
        assert_eq!(
            liblrhsmm_rs_model_replace_f64(model, bad_streams.as_ptr(), 2, durations.as_ptr(), 2),
            1
        );
        assert_eq!(
            liblrhsmm_rs_model_replace_f64(model, streams.as_ptr(), 3, bad_durations.as_ptr(), 2),
            1
        );
        compare_f64(model, &expected);
        assert_eq!(
            liblrhsmm_rs_model_create_f64(null(), 1, durations.as_ptr(), 2, &mut failure),
            1
        );
        assert_eq!(failure, sentinel);
        assert_eq!(
            liblrhsmm_rs_model_create_f64(
                streams.as_ptr(),
                3,
                null(),
                i32::MAX as usize + 1,
                &mut failure
            ),
            2
        );
        assert_eq!(failure, sentinel);
        assert_eq!(liblrhsmm_rs_model_clone_f64(model, null_mut()), 1);

        let mut builder = null_mut();
        assert_eq!(liblrhsmm_rs_model_builder_new_f64(2, 2, &mut builder), 0);
        let mut layout = LiblrhsmmRsModelLayout {
            streams: 99,
            durations: 99,
        };
        assert_eq!(
            liblrhsmm_rs_model_builder_layout_f64(builder, &mut layout),
            0
        );
        assert_eq!(
            layout,
            LiblrhsmmRsModelLayout {
                streams: 2,
                durations: 2
            }
        );
        let mut present = 99;
        assert_eq!(
            liblrhsmm_rs_model_builder_has_stream_f64(builder, 0, &mut present),
            0
        );
        assert_eq!(present, 0);
        assert_eq!(
            liblrhsmm_rs_model_builder_has_duration_f64(builder, 0, &mut present),
            0
        );
        assert_eq!(present, 0);
        returned_stream = first;
        returned_duration = duration;
        assert_eq!(
            liblrhsmm_rs_model_builder_get_stream_f64(builder, 0, &mut returned_stream),
            3
        );
        assert_eq!(returned_stream, first);
        assert_eq!(
            liblrhsmm_rs_model_builder_get_duration_f64(builder, 0, &mut returned_duration),
            3
        );
        assert_eq!(returned_duration, duration);
        assert_eq!(
            liblrhsmm_rs_model_builder_build_f64(builder, &mut failure),
            3
        );
        assert_eq!(failure, sentinel);
        assert_eq!(
            liblrhsmm_rs_model_builder_set_stream_f64(builder, 0, first),
            0
        );
        assert_eq!(
            liblrhsmm_rs_model_builder_set_stream_f64(builder, 1, second),
            0
        );
        assert_eq!(
            liblrhsmm_rs_model_builder_build_f64(builder, &mut failure),
            3
        );
        assert_eq!(failure, sentinel);
        assert_eq!(liblrhsmm_rs_duration_set_f64(duration, &descriptor), 0);
        assert_eq!(
            liblrhsmm_rs_model_builder_set_duration_f64(builder, 0, duration),
            0
        );
        assert_eq!(
            liblrhsmm_rs_model_builder_set_duration_f64(builder, 1, duration),
            0
        );
        assert_eq!(
            liblrhsmm_rs_model_builder_has_stream_f64(builder, 0, &mut present),
            0
        );
        assert_eq!(present, 1);
        assert_eq!(
            liblrhsmm_rs_model_builder_has_duration_f64(builder, 0, &mut present),
            0
        );
        assert_eq!(present, 1);
        let mut saved = null_mut();
        let mut built = null_mut();
        assert_eq!(liblrhsmm_rs_model_builder_clone_f64(builder, &mut saved), 0);
        assert_eq!(liblrhsmm_rs_model_builder_build_f64(builder, &mut built), 0);
        let built_expected = ModelParameters {
            streams: vec![a.clone(), b.clone()],
            durations: vec![d.clone(), d.clone()],
        };
        compare_f64(built, &built_expected);
        returned_stream = null_mut();
        returned_duration = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_builder_get_stream_f64(builder, 0, &mut returned_stream),
            0
        );
        assert_eq!(liblrhsmm_rs_stream_set_weight_f64(returned_stream, 99.0), 0);
        assert_eq!(
            liblrhsmm_rs_model_builder_get_duration_f64(builder, 0, &mut returned_duration),
            0
        );
        assert_eq!(
            liblrhsmm_rs_duration_set_f64(returned_duration, &revised),
            0
        );
        assert_eq!(liblrhsmm_rs_stream_release_f64(&mut returned_stream), 0);
        assert_eq!(liblrhsmm_rs_duration_release_f64(&mut returned_duration), 0);
        assert_eq!(liblrhsmm_rs_model_builder_clear_stream_f64(builder, 0), 0);
        assert_eq!(
            liblrhsmm_rs_model_builder_build_f64(builder, &mut failure),
            3
        );
        assert_eq!(failure, sentinel);
        assert_eq!(
            liblrhsmm_rs_model_builder_set_stream_f64(builder, 0, second),
            0
        );
        assert_eq!(liblrhsmm_rs_model_builder_clear_duration_f64(builder, 1), 0);
        assert_eq!(
            liblrhsmm_rs_model_builder_build_f64(builder, &mut failure),
            3
        );
        assert_eq!(failure, sentinel);
        assert_eq!(liblrhsmm_rs_model_builder_clear_duration_f64(builder, 1), 0);
        present = 99;
        assert_eq!(
            liblrhsmm_rs_model_builder_has_stream_f64(builder, 2, &mut present),
            2
        );
        assert_eq!(present, 99);
        assert_eq!(
            liblrhsmm_rs_model_builder_has_duration_f64(builder, 2, &mut present),
            2
        );
        assert_eq!(present, 99);
        assert_eq!(
            liblrhsmm_rs_model_builder_get_stream_f64(builder, 2, &mut returned_stream),
            2
        );
        assert_eq!(
            liblrhsmm_rs_model_builder_get_duration_f64(builder, 2, &mut returned_duration),
            2
        );
        assert_eq!(
            liblrhsmm_rs_model_builder_set_stream_f64(builder, 2, first),
            2
        );
        assert_eq!(
            liblrhsmm_rs_model_builder_set_duration_f64(builder, 2, duration),
            2
        );
        assert_eq!(liblrhsmm_rs_model_builder_clear_stream_f64(builder, 2), 2);
        assert_eq!(liblrhsmm_rs_model_builder_clear_duration_f64(builder, 2), 2);
        assert_eq!(
            liblrhsmm_rs_model_builder_set_stream_f64(builder, 0, null()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_model_builder_set_duration_f64(builder, 0, null()),
            1
        );
        assert_eq!(liblrhsmm_rs_model_builder_build_f64(builder, null_mut()), 1);
        let mut bad_builder = builder;
        assert_eq!(
            liblrhsmm_rs_model_builder_new_f64(i32::MAX as usize + 1, 0, &mut bad_builder),
            3
        );
        assert_eq!(bad_builder, builder);
        assert_eq!(
            liblrhsmm_rs_model_builder_new_f64(0, i32::MAX as usize + 1, &mut bad_builder),
            3
        );
        assert_eq!(bad_builder, builder);
        assert_eq!(liblrhsmm_rs_stream_release_f64(&mut first), 0);
        assert_eq!(liblrhsmm_rs_stream_release_f64(&mut second), 0);
        assert_eq!(liblrhsmm_rs_duration_release_f64(&mut duration), 0);
        assert_eq!(liblrhsmm_rs_model_builder_release_f64(&mut builder), 0);
        assert!(builder.is_null());
        assert_eq!(liblrhsmm_rs_model_builder_release_f64(&mut builder), 0);
        let mut rebuilt = null_mut();
        assert_eq!(liblrhsmm_rs_model_builder_build_f64(saved, &mut rebuilt), 0);
        compare_f64(rebuilt, &built_expected);
        compare_f64(built, &built_expected);
        assert_eq!(liblrhsmm_rs_model_builder_release_f64(&mut saved), 0);
        compare_f64(model, &expected);
        for mut handle in [model, clone, created, built, rebuilt] {
            assert_eq!(liblrhsmm_rs_model_release_f64(&mut handle), 0);
            assert!(handle.is_null());
            assert_eq!(liblrhsmm_rs_model_release_f64(&mut handle), 0);
        }
        assert_eq!(
            liblrhsmm_rs_model_create_f64(null(), 0, null(), 0, &mut model),
            0
        );
        compare_f64(model, &ModelParameters::default());
        assert_eq!(
            liblrhsmm_rs_model_replace_f64(model, null(), 0, null(), 0),
            0
        );
        assert_eq!(liblrhsmm_rs_model_release_f64(&mut model), 0);
        assert_eq!(liblrhsmm_rs_model_new_f64(null(), 0, 0, &mut model), 0);
        compare_f64(model, &ModelParameters::default());
        assert_eq!(liblrhsmm_rs_model_release_f64(&mut model), 0);
        assert_eq!(liblrhsmm_rs_model_builder_new_f64(0, 0, &mut builder), 0);
        assert_eq!(liblrhsmm_rs_model_builder_build_f64(builder, &mut model), 0);
        compare_f64(model, &ModelParameters::default());
        assert_eq!(liblrhsmm_rs_model_release_f64(&mut model), 0);
        assert_eq!(liblrhsmm_rs_model_builder_release_f64(&mut builder), 0);
        assert_eq!(liblrhsmm_rs_model_release_f64(null_mut()), 1);
        assert_eq!(liblrhsmm_rs_model_builder_release_f64(null_mut()), 1);
    }
}
