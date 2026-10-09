use liblrhsmm_rs::model::{GaussianMixtureParameters, StreamParameters};
use liblrhsmm_rs_capi::*;
use std::ptr::{null, null_mut};

unsafe fn compare_f32(handle: *const LiblrhsmmRsStreamF32, expected: &StreamParameters<f32>) {
    // SAFETY: Live readable owner, independent initialized outputs; returned children are released.
    unsafe {
        let mut layout = LiblrhsmmRsStreamLayoutF32 {
            weight: 99.0,
            emissions: usize::MAX,
        };
        assert_eq!(liblrhsmm_rs_stream_layout_f32(handle, &mut layout), 0);
        assert_eq!(layout.weight.to_bits(), expected.weight.to_bits());
        assert_eq!(layout.emissions, expected.mixtures.len());
        for (index, native) in expected.mixtures.iter().enumerate() {
            let mut child = null_mut();
            assert_eq!(
                liblrhsmm_rs_stream_get_emission_f32(handle, index, &mut child),
                0
            );
            let mut shape = LiblrhsmmRsGaussianLayout {
                mixtures: 0,
                dimensions: 0,
            };
            assert_eq!(liblrhsmm_rs_gaussian_layout_f32(child, &mut shape), 0);
            assert_eq!(shape.mixtures, native.weights.len());
            assert_eq!(shape.dimensions, native.dimensions);
            for (field, expected) in [
                &native.weights,
                &native.means,
                &native.variances,
                &native.variance_floors,
            ]
            .into_iter()
            .enumerate()
            {
                let mut actual = vec![99.0_f32; expected.len()];
                assert_eq!(
                    liblrhsmm_rs_gaussian_copy_field_f32(
                        child,
                        field as u32,
                        0,
                        actual.as_mut_ptr(),
                        actual.len()
                    ),
                    0
                );
                assert_eq!(
                    actual.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
                    expected.iter().map(|x| x.to_bits()).collect::<Vec<_>>()
                );
            }
            assert_eq!(liblrhsmm_rs_gaussian_release_f32(&mut child), 0);
        }
    }
}

#[test]
fn stream_f32_complete_owners_and_incremental_construction() {
    // SAFETY: Every input is live/initialized/independent; mutation is exclusive; owners released once.
    unsafe {
        let mut stream = null_mut();
        assert_eq!(liblrhsmm_rs_stream_new_f32(2, 2, 3, &mut stream), 0);
        compare_f32(stream, &StreamParameters::<f32>::new(2, 2, 3).unwrap());
        let sentinel = stream;
        let mut failure = sentinel;
        assert_eq!(liblrhsmm_rs_stream_new_f32(1, 0, 1, &mut failure), 3);
        assert_eq!(failure, sentinel);
        assert_eq!(
            liblrhsmm_rs_stream_new_f32(i32::MAX as usize + 1, 1, 1, &mut failure),
            3
        );
        assert_eq!(failure, sentinel);
        assert_eq!(liblrhsmm_rs_stream_new_f32(2, 2, 3, null_mut()), 1);
        assert_eq!(liblrhsmm_rs_stream_layout_f32(null(), null_mut()), 1);

        let mut first = null_mut();
        let mut second = null_mut();
        assert_eq!(liblrhsmm_rs_gaussian_new_f32(1, 1, &mut first), 0);
        assert_eq!(liblrhsmm_rs_gaussian_new_f32(2, 3, &mut second), 0);
        let mut a = GaussianMixtureParameters::<f32>::new(1, 1).unwrap();
        let mut b = GaussianMixtureParameters::<f32>::new(2, 3).unwrap();
        for (handle, native) in [(first, &mut a), (second, &mut b)] {
            for (selector, values) in [
                &mut native.weights,
                &mut native.means,
                &mut native.variances,
                &mut native.variance_floors,
            ]
            .into_iter()
            .enumerate()
            {
                for (i, value) in values.iter_mut().enumerate() {
                    *value = match (selector + i) % 5 {
                        0 => f32::from_bits(0x7fc01234),
                        1 => -0.0,
                        2 => f32::INFINITY,
                        3 => -2.5,
                        _ => 0.125,
                    };
                }
                assert_eq!(
                    liblrhsmm_rs_gaussian_set_field_f32(
                        handle,
                        selector as u32,
                        0,
                        values.as_ptr(),
                        values.len()
                    ),
                    0
                );
            }
        }
        let children = [first.cast_const(), second.cast_const(), first.cast_const()];
        let weight = f32::from_bits(0x7fc04321);
        let mut expected = StreamParameters {
            weight,
            mixtures: vec![a.clone(), b.clone(), a.clone()],
        };
        assert_eq!(
            liblrhsmm_rs_stream_replace_f32(stream, weight, children.as_ptr(), children.len()),
            0
        );
        compare_f32(stream, &expected);
        let mut created = null_mut();
        assert_eq!(
            liblrhsmm_rs_stream_create_f32(weight, children.as_ptr(), 3, &mut created),
            0
        );
        compare_f32(created, &expected);
        let mut clone = null_mut();
        assert_eq!(liblrhsmm_rs_stream_clone_f32(stream, &mut clone), 0);
        assert_eq!(liblrhsmm_rs_stream_set_weight_f32(stream, -0.0), 0);
        expected.weight = -0.0;
        assert_eq!(liblrhsmm_rs_stream_set_emission_f32(stream, 0, second), 0);
        expected.mixtures[0] = b.clone();
        compare_f32(stream, &expected);
        compare_f32(
            clone,
            &StreamParameters {
                weight,
                mixtures: vec![a.clone(), b.clone(), a.clone()],
            },
        );
        let mut returned = null_mut();
        assert_eq!(
            liblrhsmm_rs_stream_get_emission_f32(stream, 0, &mut returned),
            0
        );
        let changed = [42.0_f32];
        assert_eq!(
            liblrhsmm_rs_gaussian_set_field_f32(returned, 1, 0, changed.as_ptr(), 1),
            0
        );
        compare_f32(stream, &expected);
        assert_eq!(liblrhsmm_rs_gaussian_release_f32(&mut returned), 0);
        assert_eq!(liblrhsmm_rs_stream_set_emission_f32(stream, 3, second), 2);
        assert_eq!(liblrhsmm_rs_stream_set_emission_f32(stream, 0, null()), 1);
        failure = sentinel;
        assert_eq!(
            liblrhsmm_rs_stream_get_emission_f32(stream, 3, &mut returned),
            2
        );
        assert!(returned.is_null());
        assert_eq!(
            liblrhsmm_rs_stream_create_f32(weight, null(), 3, &mut failure),
            1
        );
        assert_eq!(failure, sentinel);
        let invalid_children = [first.cast_const(), null()];
        assert_eq!(
            liblrhsmm_rs_stream_replace_f32(stream, 4.0, invalid_children.as_ptr(), 2),
            1
        );
        compare_f32(stream, &expected);
        assert_eq!(
            liblrhsmm_rs_stream_replace_f32(stream, 4.0, null(), i32::MAX as usize + 1),
            2
        );
        compare_f32(stream, &expected);

        let mut builder = null_mut();
        assert_eq!(liblrhsmm_rs_stream_builder_new_f32(2, &mut builder), 0);
        let mut layout = LiblrhsmmRsStreamLayoutF32 {
            weight: 99.0,
            emissions: 99,
        };
        assert_eq!(
            liblrhsmm_rs_stream_builder_layout_f32(builder, &mut layout),
            0
        );
        assert_eq!(
            layout,
            LiblrhsmmRsStreamLayoutF32 {
                weight: 1.0,
                emissions: 2
            }
        );
        let mut present = 99;
        assert_eq!(
            liblrhsmm_rs_stream_builder_has_emission_f32(builder, 0, &mut present),
            0
        );
        assert_eq!(present, 0);
        returned = first;
        assert_eq!(
            liblrhsmm_rs_stream_builder_get_emission_f32(builder, 0, &mut returned),
            3
        );
        assert_eq!(returned, first);
        failure = sentinel;
        assert_eq!(
            liblrhsmm_rs_stream_builder_build_f32(builder, &mut failure),
            3
        );
        assert_eq!(failure, sentinel);
        assert_eq!(
            liblrhsmm_rs_stream_builder_build_f32(builder, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_stream_builder_set_weight_f32(builder, weight),
            0
        );
        assert_eq!(
            liblrhsmm_rs_stream_builder_set_emission_f32(builder, 0, first),
            0
        );
        assert_eq!(
            liblrhsmm_rs_stream_builder_has_emission_f32(builder, 0, &mut present),
            0
        );
        assert_eq!(present, 1);
        assert_eq!(
            liblrhsmm_rs_stream_builder_set_emission_f32(builder, 1, second),
            0
        );
        let mut saved_builder = null_mut();
        assert_eq!(
            liblrhsmm_rs_stream_builder_clone_f32(builder, &mut saved_builder),
            0
        );
        let mut built = null_mut();
        assert_eq!(
            liblrhsmm_rs_stream_builder_build_f32(builder, &mut built),
            0
        );
        let built_expected = StreamParameters {
            weight,
            mixtures: vec![a.clone(), b.clone()],
        };
        compare_f32(built, &built_expected);
        returned = null_mut();
        assert_eq!(
            liblrhsmm_rs_stream_builder_get_emission_f32(builder, 1, &mut returned),
            0
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_set_field_f32(returned, 1, 0, changed.as_ptr(), 1),
            0
        );
        assert_eq!(liblrhsmm_rs_gaussian_release_f32(&mut returned), 0);
        assert_eq!(
            liblrhsmm_rs_stream_builder_clear_emission_f32(builder, 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_stream_builder_clear_emission_f32(builder, 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_stream_builder_build_f32(builder, &mut failure),
            3
        );
        assert_eq!(failure, sentinel);
        present = 99;
        assert_eq!(
            liblrhsmm_rs_stream_builder_has_emission_f32(builder, 2, &mut present),
            2
        );
        assert_eq!(present, 99);
        assert_eq!(
            liblrhsmm_rs_stream_builder_set_emission_f32(builder, 2, first),
            2
        );
        assert_eq!(
            liblrhsmm_rs_stream_builder_clear_emission_f32(builder, 2),
            2
        );
        assert_eq!(
            liblrhsmm_rs_stream_builder_set_emission_f32(builder, 0, null()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_stream_builder_get_emission_f32(builder, 2, &mut returned),
            2
        );
        let mut bad_builder = builder;
        assert_eq!(
            liblrhsmm_rs_stream_builder_new_f32(i32::MAX as usize + 1, &mut bad_builder),
            3
        );
        assert_eq!(bad_builder, builder);
        assert_eq!(liblrhsmm_rs_gaussian_release_f32(&mut first), 0);
        assert_eq!(liblrhsmm_rs_gaussian_release_f32(&mut second), 0);
        assert_eq!(liblrhsmm_rs_stream_builder_release_f32(&mut builder), 0);
        assert!(builder.is_null());
        assert_eq!(liblrhsmm_rs_stream_builder_release_f32(&mut builder), 0);
        compare_f32(built, &built_expected);
        let mut rebuilt = null_mut();
        assert_eq!(
            liblrhsmm_rs_stream_builder_build_f32(saved_builder, &mut rebuilt),
            0
        );
        compare_f32(rebuilt, &built_expected);
        assert_eq!(
            liblrhsmm_rs_stream_builder_release_f32(&mut saved_builder),
            0
        );
        for mut handle in [stream, clone, created, built, rebuilt] {
            assert_eq!(liblrhsmm_rs_stream_release_f32(&mut handle), 0);
            assert!(handle.is_null());
            assert_eq!(liblrhsmm_rs_stream_release_f32(&mut handle), 0);
        }
        assert_eq!(
            liblrhsmm_rs_stream_create_f32(-0.0, null(), 0, &mut stream),
            0
        );
        compare_f32(
            stream,
            &StreamParameters {
                weight: -0.0,
                mixtures: vec![],
            },
        );
        assert_eq!(liblrhsmm_rs_stream_replace_f32(stream, 1.0, null(), 0), 0);
        compare_f32(stream, &StreamParameters::default());
        assert_eq!(liblrhsmm_rs_stream_release_f32(&mut stream), 0);
        assert_eq!(liblrhsmm_rs_stream_new_f32(0, 0, 0, &mut stream), 0);
        compare_f32(stream, &StreamParameters::default());
        assert_eq!(liblrhsmm_rs_stream_release_f32(&mut stream), 0);
        assert_eq!(liblrhsmm_rs_stream_builder_new_f32(0, &mut builder), 0);
        assert_eq!(
            liblrhsmm_rs_stream_builder_build_f32(builder, &mut stream),
            0
        );
        compare_f32(stream, &StreamParameters::default());
        assert_eq!(liblrhsmm_rs_stream_release_f32(&mut stream), 0);
        assert_eq!(liblrhsmm_rs_stream_builder_release_f32(&mut builder), 0);
        assert_eq!(liblrhsmm_rs_stream_release_f32(null_mut()), 1);
        assert_eq!(liblrhsmm_rs_stream_builder_release_f32(null_mut()), 1);
    }
}

unsafe fn compare_f64(handle: *const LiblrhsmmRsStreamF64, expected: &StreamParameters<f64>) {
    // SAFETY: Live readable owner, independent initialized outputs; returned children are released.
    unsafe {
        let mut layout = LiblrhsmmRsStreamLayoutF64 {
            weight: 99.0,
            emissions: usize::MAX,
        };
        assert_eq!(liblrhsmm_rs_stream_layout_f64(handle, &mut layout), 0);
        assert_eq!(layout.weight.to_bits(), expected.weight.to_bits());
        assert_eq!(layout.emissions, expected.mixtures.len());
        for (index, native) in expected.mixtures.iter().enumerate() {
            let mut child = null_mut();
            assert_eq!(
                liblrhsmm_rs_stream_get_emission_f64(handle, index, &mut child),
                0
            );
            let mut shape = LiblrhsmmRsGaussianLayout {
                mixtures: 0,
                dimensions: 0,
            };
            assert_eq!(liblrhsmm_rs_gaussian_layout_f64(child, &mut shape), 0);
            assert_eq!(shape.mixtures, native.weights.len());
            assert_eq!(shape.dimensions, native.dimensions);
            for (field, expected) in [
                &native.weights,
                &native.means,
                &native.variances,
                &native.variance_floors,
            ]
            .into_iter()
            .enumerate()
            {
                let mut actual = vec![99.0_f64; expected.len()];
                assert_eq!(
                    liblrhsmm_rs_gaussian_copy_field_f64(
                        child,
                        field as u32,
                        0,
                        actual.as_mut_ptr(),
                        actual.len()
                    ),
                    0
                );
                assert_eq!(
                    actual.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
                    expected.iter().map(|x| x.to_bits()).collect::<Vec<_>>()
                );
            }
            assert_eq!(liblrhsmm_rs_gaussian_release_f64(&mut child), 0);
        }
    }
}

#[test]
fn stream_f64_complete_owners_and_incremental_construction() {
    // SAFETY: Every input is live/initialized/independent; mutation is exclusive; owners released once.
    unsafe {
        let mut stream = null_mut();
        assert_eq!(liblrhsmm_rs_stream_new_f64(2, 2, 3, &mut stream), 0);
        compare_f64(stream, &StreamParameters::<f64>::new(2, 2, 3).unwrap());
        let sentinel = stream;
        let mut failure = sentinel;
        assert_eq!(liblrhsmm_rs_stream_new_f64(1, 0, 1, &mut failure), 3);
        assert_eq!(failure, sentinel);
        assert_eq!(
            liblrhsmm_rs_stream_new_f64(i32::MAX as usize + 1, 1, 1, &mut failure),
            3
        );
        assert_eq!(failure, sentinel);
        assert_eq!(liblrhsmm_rs_stream_new_f64(2, 2, 3, null_mut()), 1);
        assert_eq!(liblrhsmm_rs_stream_layout_f64(null(), null_mut()), 1);

        let mut first = null_mut();
        let mut second = null_mut();
        assert_eq!(liblrhsmm_rs_gaussian_new_f64(1, 1, &mut first), 0);
        assert_eq!(liblrhsmm_rs_gaussian_new_f64(2, 3, &mut second), 0);
        let mut a = GaussianMixtureParameters::<f64>::new(1, 1).unwrap();
        let mut b = GaussianMixtureParameters::<f64>::new(2, 3).unwrap();
        for (handle, native) in [(first, &mut a), (second, &mut b)] {
            for (selector, values) in [
                &mut native.weights,
                &mut native.means,
                &mut native.variances,
                &mut native.variance_floors,
            ]
            .into_iter()
            .enumerate()
            {
                for (i, value) in values.iter_mut().enumerate() {
                    *value = match (selector + i) % 5 {
                        0 => f64::from_bits(0x7ff8000000001234),
                        1 => -0.0,
                        2 => f64::INFINITY,
                        3 => -2.5,
                        _ => 0.125,
                    };
                }
                assert_eq!(
                    liblrhsmm_rs_gaussian_set_field_f64(
                        handle,
                        selector as u32,
                        0,
                        values.as_ptr(),
                        values.len()
                    ),
                    0
                );
            }
        }
        let children = [first.cast_const(), second.cast_const(), first.cast_const()];
        let weight = f64::from_bits(0x7ff8000000004321);
        let mut expected = StreamParameters {
            weight,
            mixtures: vec![a.clone(), b.clone(), a.clone()],
        };
        assert_eq!(
            liblrhsmm_rs_stream_replace_f64(stream, weight, children.as_ptr(), children.len()),
            0
        );
        compare_f64(stream, &expected);
        let mut created = null_mut();
        assert_eq!(
            liblrhsmm_rs_stream_create_f64(weight, children.as_ptr(), 3, &mut created),
            0
        );
        compare_f64(created, &expected);
        let mut clone = null_mut();
        assert_eq!(liblrhsmm_rs_stream_clone_f64(stream, &mut clone), 0);
        assert_eq!(liblrhsmm_rs_stream_set_weight_f64(stream, -0.0), 0);
        expected.weight = -0.0;
        assert_eq!(liblrhsmm_rs_stream_set_emission_f64(stream, 0, second), 0);
        expected.mixtures[0] = b.clone();
        compare_f64(stream, &expected);
        compare_f64(
            clone,
            &StreamParameters {
                weight,
                mixtures: vec![a.clone(), b.clone(), a.clone()],
            },
        );
        let mut returned = null_mut();
        assert_eq!(
            liblrhsmm_rs_stream_get_emission_f64(stream, 0, &mut returned),
            0
        );
        let changed = [42.0_f64];
        assert_eq!(
            liblrhsmm_rs_gaussian_set_field_f64(returned, 1, 0, changed.as_ptr(), 1),
            0
        );
        compare_f64(stream, &expected);
        assert_eq!(liblrhsmm_rs_gaussian_release_f64(&mut returned), 0);
        assert_eq!(liblrhsmm_rs_stream_set_emission_f64(stream, 3, second), 2);
        assert_eq!(liblrhsmm_rs_stream_set_emission_f64(stream, 0, null()), 1);
        failure = sentinel;
        assert_eq!(
            liblrhsmm_rs_stream_get_emission_f64(stream, 3, &mut returned),
            2
        );
        assert!(returned.is_null());
        assert_eq!(
            liblrhsmm_rs_stream_create_f64(weight, null(), 3, &mut failure),
            1
        );
        assert_eq!(failure, sentinel);
        let invalid_children = [first.cast_const(), null()];
        assert_eq!(
            liblrhsmm_rs_stream_replace_f64(stream, 4.0, invalid_children.as_ptr(), 2),
            1
        );
        compare_f64(stream, &expected);
        assert_eq!(
            liblrhsmm_rs_stream_replace_f64(stream, 4.0, null(), i32::MAX as usize + 1),
            2
        );
        compare_f64(stream, &expected);

        let mut builder = null_mut();
        assert_eq!(liblrhsmm_rs_stream_builder_new_f64(2, &mut builder), 0);
        let mut layout = LiblrhsmmRsStreamLayoutF64 {
            weight: 99.0,
            emissions: 99,
        };
        assert_eq!(
            liblrhsmm_rs_stream_builder_layout_f64(builder, &mut layout),
            0
        );
        assert_eq!(
            layout,
            LiblrhsmmRsStreamLayoutF64 {
                weight: 1.0,
                emissions: 2
            }
        );
        let mut present = 99;
        assert_eq!(
            liblrhsmm_rs_stream_builder_has_emission_f64(builder, 0, &mut present),
            0
        );
        assert_eq!(present, 0);
        returned = first;
        assert_eq!(
            liblrhsmm_rs_stream_builder_get_emission_f64(builder, 0, &mut returned),
            3
        );
        assert_eq!(returned, first);
        failure = sentinel;
        assert_eq!(
            liblrhsmm_rs_stream_builder_build_f64(builder, &mut failure),
            3
        );
        assert_eq!(failure, sentinel);
        assert_eq!(
            liblrhsmm_rs_stream_builder_build_f64(builder, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_stream_builder_set_weight_f64(builder, weight),
            0
        );
        assert_eq!(
            liblrhsmm_rs_stream_builder_set_emission_f64(builder, 0, first),
            0
        );
        assert_eq!(
            liblrhsmm_rs_stream_builder_has_emission_f64(builder, 0, &mut present),
            0
        );
        assert_eq!(present, 1);
        assert_eq!(
            liblrhsmm_rs_stream_builder_set_emission_f64(builder, 1, second),
            0
        );
        let mut saved_builder = null_mut();
        assert_eq!(
            liblrhsmm_rs_stream_builder_clone_f64(builder, &mut saved_builder),
            0
        );
        let mut built = null_mut();
        assert_eq!(
            liblrhsmm_rs_stream_builder_build_f64(builder, &mut built),
            0
        );
        let built_expected = StreamParameters {
            weight,
            mixtures: vec![a.clone(), b.clone()],
        };
        compare_f64(built, &built_expected);
        returned = null_mut();
        assert_eq!(
            liblrhsmm_rs_stream_builder_get_emission_f64(builder, 1, &mut returned),
            0
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_set_field_f64(returned, 1, 0, changed.as_ptr(), 1),
            0
        );
        assert_eq!(liblrhsmm_rs_gaussian_release_f64(&mut returned), 0);
        assert_eq!(
            liblrhsmm_rs_stream_builder_clear_emission_f64(builder, 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_stream_builder_clear_emission_f64(builder, 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_stream_builder_build_f64(builder, &mut failure),
            3
        );
        assert_eq!(failure, sentinel);
        present = 99;
        assert_eq!(
            liblrhsmm_rs_stream_builder_has_emission_f64(builder, 2, &mut present),
            2
        );
        assert_eq!(present, 99);
        assert_eq!(
            liblrhsmm_rs_stream_builder_set_emission_f64(builder, 2, first),
            2
        );
        assert_eq!(
            liblrhsmm_rs_stream_builder_clear_emission_f64(builder, 2),
            2
        );
        assert_eq!(
            liblrhsmm_rs_stream_builder_set_emission_f64(builder, 0, null()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_stream_builder_get_emission_f64(builder, 2, &mut returned),
            2
        );
        let mut bad_builder = builder;
        assert_eq!(
            liblrhsmm_rs_stream_builder_new_f64(i32::MAX as usize + 1, &mut bad_builder),
            3
        );
        assert_eq!(bad_builder, builder);
        assert_eq!(liblrhsmm_rs_gaussian_release_f64(&mut first), 0);
        assert_eq!(liblrhsmm_rs_gaussian_release_f64(&mut second), 0);
        assert_eq!(liblrhsmm_rs_stream_builder_release_f64(&mut builder), 0);
        assert!(builder.is_null());
        assert_eq!(liblrhsmm_rs_stream_builder_release_f64(&mut builder), 0);
        compare_f64(built, &built_expected);
        let mut rebuilt = null_mut();
        assert_eq!(
            liblrhsmm_rs_stream_builder_build_f64(saved_builder, &mut rebuilt),
            0
        );
        compare_f64(rebuilt, &built_expected);
        assert_eq!(
            liblrhsmm_rs_stream_builder_release_f64(&mut saved_builder),
            0
        );
        for mut handle in [stream, clone, created, built, rebuilt] {
            assert_eq!(liblrhsmm_rs_stream_release_f64(&mut handle), 0);
            assert!(handle.is_null());
            assert_eq!(liblrhsmm_rs_stream_release_f64(&mut handle), 0);
        }
        assert_eq!(
            liblrhsmm_rs_stream_create_f64(-0.0, null(), 0, &mut stream),
            0
        );
        compare_f64(
            stream,
            &StreamParameters {
                weight: -0.0,
                mixtures: vec![],
            },
        );
        assert_eq!(liblrhsmm_rs_stream_replace_f64(stream, 1.0, null(), 0), 0);
        compare_f64(stream, &StreamParameters::default());
        assert_eq!(liblrhsmm_rs_stream_release_f64(&mut stream), 0);
        assert_eq!(liblrhsmm_rs_stream_new_f64(0, 0, 0, &mut stream), 0);
        compare_f64(stream, &StreamParameters::default());
        assert_eq!(liblrhsmm_rs_stream_release_f64(&mut stream), 0);
        assert_eq!(liblrhsmm_rs_stream_builder_new_f64(0, &mut builder), 0);
        assert_eq!(
            liblrhsmm_rs_stream_builder_build_f64(builder, &mut stream),
            0
        );
        compare_f64(stream, &StreamParameters::default());
        assert_eq!(liblrhsmm_rs_stream_release_f64(&mut stream), 0);
        assert_eq!(liblrhsmm_rs_stream_builder_release_f64(&mut builder), 0);
        assert_eq!(liblrhsmm_rs_stream_release_f64(null_mut()), 1);
        assert_eq!(liblrhsmm_rs_stream_builder_release_f64(null_mut()), 1);
    }
}
