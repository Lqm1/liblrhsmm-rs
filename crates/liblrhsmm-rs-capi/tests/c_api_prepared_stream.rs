use liblrhsmm_rs_capi::*;
use std::ptr::{null, null_mut};

#[test]
fn prepared_stream_f32_original_c_values_snapshot_and_errors() {
    // SAFETY: Initialized independent buffers; exclusive owners; all releases occur once.
    unsafe {
        let mut source: *mut LiblrhsmmRsGaussianF32 = null_mut();
        let mut prepared: *mut LiblrhsmmRsPreparedGaussianF32 = null_mut();
        assert_eq!(liblrhsmm_rs_gaussian_new_f32(1, 2, &mut source), 0);
        assert_eq!(prepare_through_stream_f32(source, &mut prepared), 3);
        assert!(prepared.is_null());
        let weights: [f32; 1] = [1.0];
        let means: [f32; 2] = [1.25, -2.5];
        let variances: [f32; 2] = [2.5, 0.75];
        let floors: [f32; 2] = [0.1, 0.2];
        let input = LiblrhsmmRsGaussianParametersF32 {
            mixtures: 1,
            dimensions: 2,
            weights: weights.as_ptr(),
            means: means.as_ptr(),
            variances: variances.as_ptr(),
            variance_floors: floors.as_ptr(),
        };
        assert_eq!(liblrhsmm_rs_gaussian_replace_f32(source, &input), 0);
        assert_eq!(
            liblrhsmm_rs_gaussian_increase_mixtures_f32(source, 5, 0.2),
            0
        );
        assert_eq!(prepare_through_stream_f32(source, &mut prepared), 0);
        let mut editable: *mut LiblrhsmmRsGaussianF32 = null_mut();
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_parameters_f32(prepared, &mut editable),
            0
        );
        let mut cloned: *mut LiblrhsmmRsPreparedGaussianF32 = null_mut();
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_clone_f32(prepared, &mut cloned),
            0
        );
        let changed: [f32; 1] = [999.0];
        assert_eq!(
            liblrhsmm_rs_gaussian_set_field_f32(source, 1, 0, changed.as_ptr(), 1),
            0
        );
        assert_eq!(liblrhsmm_rs_gaussian_release_f32(&mut source), 0);
        assert_eq!(
            liblrhsmm_rs_gaussian_set_field_f32(editable, 1, 0, changed.as_ptr(), 1),
            0
        );
        assert_eq!(liblrhsmm_rs_gaussian_release_f32(&mut editable), 0);
        let mut length = 0;
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_cache_length_f32(prepared, &mut length),
            0
        );
        assert_eq!(length, 5);
        let mut cache = [0.0; 5];
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_cache_copy_f32(prepared, 0, cache.as_mut_ptr(), 5),
            0
        );
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_cache_copy_f32(prepared, 5, null_mut(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_cache_copy_f32(prepared, usize::MAX, null_mut(), 0),
            2
        );
        let mut result: f32 = 77.0;
        let mut probability = 77.0_f64;
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_cache_copy_f32(prepared, 5, &mut result, 1),
            2
        );
        assert_eq!(result, 77.0);
        let mut compared = 0;
        for line in include_str!("../../../tests/fixtures/c-precision-float.txt").lines() {
            let fields: Vec<_> = line.split_whitespace().collect();
            if !["component", "output"].contains(&fields[0]) {
                continue;
            }
            let index: usize = fields[1].parse().unwrap();
            let secondary: usize = fields[2].parse().unwrap();
            // The C oracle constructs its observations from double expressions then casts.
            let observation: [f32; 2] = [
                (index as f64 * 0.3 - 0.5) as f32,
                (index as f64 * (-0.2) + 0.75) as f32,
            ];
            if fields[0] == "component" {
                assert_eq!(
                    liblrhsmm_rs_prepared_gaussian_component_log_probability_f32(
                        prepared,
                        observation.as_ptr(),
                        2,
                        secondary,
                        &mut result
                    ),
                    0
                );
                assert_eq!(
                    liblrhsmm_rs_prepared_gaussian_component_probability_f32(
                        prepared,
                        observation.as_ptr(),
                        2,
                        secondary,
                        &mut probability
                    ),
                    0
                );
            } else {
                let temperature: [f32; 3] = [0.5, 1.0, 1.75];
                assert_eq!(
                    liblrhsmm_rs_prepared_gaussian_log_probability_f32(
                        prepared,
                        observation.as_ptr(),
                        2,
                        temperature[secondary],
                        &mut result
                    ),
                    0
                );
                assert_eq!(
                    liblrhsmm_rs_prepared_gaussian_probability_f32(
                        prepared,
                        observation.as_ptr(),
                        2,
                        temperature[secondary],
                        &mut probability
                    ),
                    0
                );
            }
            let target: f32 = fields[3].parse().unwrap();
            let target_probability: f64 = fields[4].parse().unwrap();
            assert!(
                (f64::from(result) - f64::from(target)).abs()
                    <= 2e-14 * f64::from(target).abs().max(1.0)
            );
            assert!(
                (probability - target_probability).abs()
                    <= 2e-14 * target_probability.abs().max(1.0)
            );
            compared += 2;
        }
        assert_eq!(compared, 80);
        let observation: [f32; 2] = [0.0, 0.0];
        let mut clone_result = 0.0;
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_log_probability_f32(
                cloned,
                observation.as_ptr(),
                2,
                1.0,
                &mut clone_result
            ),
            0
        );
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_log_probability_f32(
                prepared,
                observation.as_ptr(),
                2,
                1.0,
                &mut result
            ),
            0
        );
        assert_eq!(clone_result.to_bits(), result.to_bits());
        result = 77.0;
        probability = 77.0;
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_component_log_probability_f32(
                prepared,
                observation.as_ptr(),
                2,
                5,
                &mut result
            ),
            3
        );
        assert_eq!(result, 77.0);
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_log_probability_f32(
                prepared,
                null(),
                0,
                1.0,
                &mut result
            ),
            3
        );
        assert_eq!(result, 77.0);
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_log_probability_f32(
                prepared,
                observation.as_ptr(),
                2,
                -1.0,
                &mut result
            ),
            3
        );
        assert_eq!(result, 77.0);
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_probability_f32(
                prepared,
                observation.as_ptr(),
                2,
                f32::NAN,
                &mut probability
            ),
            3
        );
        assert_eq!(probability, 77.0);
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_log_probability_f32(
                prepared,
                null(),
                2,
                1.0,
                &mut result
            ),
            1
        );
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_log_probability_f32(
                prepared,
                null(),
                usize::MAX,
                1.0,
                &mut result
            ),
            2
        );
        assert_eq!(liblrhsmm_rs_prepared_gaussian_release_f32(&mut cloned), 0);
        assert_eq!(liblrhsmm_rs_prepared_gaussian_release_f32(&mut prepared), 0);
        assert!(prepared.is_null());
        assert_eq!(liblrhsmm_rs_prepared_gaussian_release_f32(&mut prepared), 0);
        // Independent original C cached-normalizer fixture, distinct from splitting.
        let weights: [f32; 3] = [1.0 / 3.0; 3];
        let means: [f32; 6] = [0.0; 6];
        let variances: [f32; 6] = [0.75, 1.25, 2.5, 3.75, 0.125, 0.5];
        let floors: [f32; 6] = [0.0; 6];
        let input = LiblrhsmmRsGaussianParametersF32 {
            mixtures: 3,
            dimensions: 2,
            weights: weights.as_ptr(),
            means: means.as_ptr(),
            variances: variances.as_ptr(),
            variance_floors: floors.as_ptr(),
        };
        assert_eq!(liblrhsmm_rs_gaussian_create_f32(&input, &mut source), 0);
        assert_eq!(prepare_through_stream_f32(source, &mut prepared), 0);
        let mut cache = [0.0; 3];
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_cache_copy_f32(prepared, 0, cache.as_mut_ptr(), 3),
            0
        );
        for line in include_str!("../../../tests/fixtures/c-structures-float.txt")
            .lines()
            .filter(|line| line.starts_with("gaussian "))
        {
            let fields: Vec<_> = line.split_whitespace().collect();
            let index: usize = fields[1].parse().unwrap();
            let expected: f32 = fields.last().unwrap().parse().unwrap();
            assert!(
                (f64::from(cache[index]) - f64::from(expected)).abs()
                    <= 2e-14 * f64::from(expected).abs().max(1.0)
            );
        }
        assert_eq!(liblrhsmm_rs_gaussian_release_f32(&mut source), 0);
        assert_eq!(liblrhsmm_rs_prepared_gaussian_release_f32(&mut prepared), 0);
    }
}

#[test]
fn prepared_stream_f64_original_c_values_snapshot_and_errors() {
    // SAFETY: Initialized independent buffers; exclusive owners; all releases occur once.
    unsafe {
        let mut source: *mut LiblrhsmmRsGaussianF64 = null_mut();
        let mut prepared: *mut LiblrhsmmRsPreparedGaussianF64 = null_mut();
        assert_eq!(liblrhsmm_rs_gaussian_new_f64(1, 2, &mut source), 0);
        assert_eq!(prepare_through_stream_f64(source, &mut prepared), 3);
        assert!(prepared.is_null());
        let weights: [f64; 1] = [1.0];
        let means: [f64; 2] = [1.25, -2.5];
        let variances: [f64; 2] = [2.5, 0.75];
        let floors: [f64; 2] = [0.1, 0.2];
        let input = LiblrhsmmRsGaussianParametersF64 {
            mixtures: 1,
            dimensions: 2,
            weights: weights.as_ptr(),
            means: means.as_ptr(),
            variances: variances.as_ptr(),
            variance_floors: floors.as_ptr(),
        };
        assert_eq!(liblrhsmm_rs_gaussian_replace_f64(source, &input), 0);
        assert_eq!(
            liblrhsmm_rs_gaussian_increase_mixtures_f64(source, 5, 0.2),
            0
        );
        assert_eq!(prepare_through_stream_f64(source, &mut prepared), 0);
        let mut editable: *mut LiblrhsmmRsGaussianF64 = null_mut();
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_parameters_f64(prepared, &mut editable),
            0
        );
        let mut cloned: *mut LiblrhsmmRsPreparedGaussianF64 = null_mut();
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_clone_f64(prepared, &mut cloned),
            0
        );
        let changed: [f64; 1] = [999.0];
        assert_eq!(
            liblrhsmm_rs_gaussian_set_field_f64(source, 1, 0, changed.as_ptr(), 1),
            0
        );
        assert_eq!(liblrhsmm_rs_gaussian_release_f64(&mut source), 0);
        assert_eq!(
            liblrhsmm_rs_gaussian_set_field_f64(editable, 1, 0, changed.as_ptr(), 1),
            0
        );
        assert_eq!(liblrhsmm_rs_gaussian_release_f64(&mut editable), 0);
        let mut length = 0;
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_cache_length_f64(prepared, &mut length),
            0
        );
        assert_eq!(length, 5);
        let mut cache = [0.0; 5];
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_cache_copy_f64(prepared, 0, cache.as_mut_ptr(), 5),
            0
        );
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_cache_copy_f64(prepared, 5, null_mut(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_cache_copy_f64(prepared, usize::MAX, null_mut(), 0),
            2
        );
        let mut result: f64 = 77.0;
        let mut probability = 77.0_f64;
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_cache_copy_f64(prepared, 5, &mut result, 1),
            2
        );
        assert_eq!(result, 77.0);
        let mut compared = 0;
        for line in include_str!("../../../tests/fixtures/c-precision-double.txt").lines() {
            let fields: Vec<_> = line.split_whitespace().collect();
            if !["component", "output"].contains(&fields[0]) {
                continue;
            }
            let index: usize = fields[1].parse().unwrap();
            let secondary: usize = fields[2].parse().unwrap();
            // The C oracle constructs its observations from double expressions then casts.
            let observation: [f64; 2] =
                [(index as f64 * 0.3 - 0.5), (index as f64 * (-0.2) + 0.75)];
            if fields[0] == "component" {
                assert_eq!(
                    liblrhsmm_rs_prepared_gaussian_component_log_probability_f64(
                        prepared,
                        observation.as_ptr(),
                        2,
                        secondary,
                        &mut result
                    ),
                    0
                );
                assert_eq!(
                    liblrhsmm_rs_prepared_gaussian_component_probability_f64(
                        prepared,
                        observation.as_ptr(),
                        2,
                        secondary,
                        &mut probability
                    ),
                    0
                );
            } else {
                let temperature: [f64; 3] = [0.5, 1.0, 1.75];
                assert_eq!(
                    liblrhsmm_rs_prepared_gaussian_log_probability_f64(
                        prepared,
                        observation.as_ptr(),
                        2,
                        temperature[secondary],
                        &mut result
                    ),
                    0
                );
                assert_eq!(
                    liblrhsmm_rs_prepared_gaussian_probability_f64(
                        prepared,
                        observation.as_ptr(),
                        2,
                        temperature[secondary],
                        &mut probability
                    ),
                    0
                );
            }
            let target: f64 = fields[3].parse().unwrap();
            let target_probability: f64 = fields[4].parse().unwrap();
            assert!((result - target).abs() <= 2e-14 * target.abs().max(1.0));
            assert!(
                (probability - target_probability).abs()
                    <= 2e-14 * target_probability.abs().max(1.0)
            );
            compared += 2;
        }
        assert_eq!(compared, 80);
        let observation: [f64; 2] = [0.0, 0.0];
        let mut clone_result = 0.0;
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_log_probability_f64(
                cloned,
                observation.as_ptr(),
                2,
                1.0,
                &mut clone_result
            ),
            0
        );
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_log_probability_f64(
                prepared,
                observation.as_ptr(),
                2,
                1.0,
                &mut result
            ),
            0
        );
        assert_eq!(clone_result.to_bits(), result.to_bits());
        result = 77.0;
        probability = 77.0;
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_component_log_probability_f64(
                prepared,
                observation.as_ptr(),
                2,
                5,
                &mut result
            ),
            3
        );
        assert_eq!(result, 77.0);
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_log_probability_f64(
                prepared,
                null(),
                0,
                1.0,
                &mut result
            ),
            3
        );
        assert_eq!(result, 77.0);
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_log_probability_f64(
                prepared,
                observation.as_ptr(),
                2,
                -1.0,
                &mut result
            ),
            3
        );
        assert_eq!(result, 77.0);
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_probability_f64(
                prepared,
                observation.as_ptr(),
                2,
                f64::NAN,
                &mut probability
            ),
            3
        );
        assert_eq!(probability, 77.0);
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_log_probability_f64(
                prepared,
                null(),
                2,
                1.0,
                &mut result
            ),
            1
        );
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_log_probability_f64(
                prepared,
                null(),
                usize::MAX,
                1.0,
                &mut result
            ),
            2
        );
        assert_eq!(liblrhsmm_rs_prepared_gaussian_release_f64(&mut cloned), 0);
        assert_eq!(liblrhsmm_rs_prepared_gaussian_release_f64(&mut prepared), 0);
        assert!(prepared.is_null());
        assert_eq!(liblrhsmm_rs_prepared_gaussian_release_f64(&mut prepared), 0);
        // Independent original C cached-normalizer fixture, distinct from splitting.
        let weights: [f64; 3] = [1.0 / 3.0; 3];
        let means: [f64; 6] = [0.0; 6];
        let variances: [f64; 6] = [0.75, 1.25, 2.5, 3.75, 0.125, 0.5];
        let floors: [f64; 6] = [0.0; 6];
        let input = LiblrhsmmRsGaussianParametersF64 {
            mixtures: 3,
            dimensions: 2,
            weights: weights.as_ptr(),
            means: means.as_ptr(),
            variances: variances.as_ptr(),
            variance_floors: floors.as_ptr(),
        };
        assert_eq!(liblrhsmm_rs_gaussian_create_f64(&input, &mut source), 0);
        assert_eq!(prepare_through_stream_f64(source, &mut prepared), 0);
        let mut cache = [0.0; 3];
        assert_eq!(
            liblrhsmm_rs_prepared_gaussian_cache_copy_f64(prepared, 0, cache.as_mut_ptr(), 3),
            0
        );
        for line in include_str!("../../../tests/fixtures/c-structures-double.txt")
            .lines()
            .filter(|line| line.starts_with("gaussian "))
        {
            let fields: Vec<_> = line.split_whitespace().collect();
            let index: usize = fields[1].parse().unwrap();
            let expected: f64 = fields.last().unwrap().parse().unwrap();
            assert!((cache[index] - expected).abs() <= 2e-14 * expected.abs().max(1.0));
        }
        assert_eq!(liblrhsmm_rs_gaussian_release_f64(&mut source), 0);
        assert_eq!(liblrhsmm_rs_prepared_gaussian_release_f64(&mut prepared), 0);
    }
}

unsafe fn prepare_through_stream_f32(
    source: *const LiblrhsmmRsGaussianF32,
    output: *mut *mut LiblrhsmmRsPreparedGaussianF32,
) -> u32 {
    // SAFETY: Caller provides a live source and independent initialized output; all local owners are released.
    unsafe {
        let mut stream = null_mut();
        let mut prepared = null_mut();
        let mut copy = null_mut();
        let mut editable = null_mut();
        let children = [source];
        assert_eq!(
            liblrhsmm_rs_stream_create_f32(0.75, children.as_ptr(), 1, &mut stream),
            0
        );
        let result = liblrhsmm_rs_stream_prepare_f32(stream, &mut prepared);
        if result == 0 {
            let mut layout = LiblrhsmmRsStreamLayoutF32 {
                weight: 99.0,
                emissions: 99,
            };
            assert_eq!(
                liblrhsmm_rs_prepared_stream_layout_f32(prepared, &mut layout),
                0
            );
            assert_eq!(
                layout,
                LiblrhsmmRsStreamLayoutF32 {
                    weight: 0.75,
                    emissions: 1
                }
            );
            assert_eq!(
                liblrhsmm_rs_prepared_stream_parameters_f32(prepared, &mut editable),
                0
            );
            assert_eq!(liblrhsmm_rs_stream_layout_f32(editable, &mut layout), 0);
            assert_eq!(layout.weight, 0.75);
            assert_eq!(
                liblrhsmm_rs_prepared_stream_clone_f32(prepared, &mut copy),
                0
            );
            assert_eq!(
                liblrhsmm_rs_prepared_stream_get_emission_f32(copy, 1, output),
                2
            );
            assert_eq!(
                liblrhsmm_rs_prepared_stream_get_emission_f32(copy, 0, output),
                0
            );
            assert_eq!(liblrhsmm_rs_prepared_stream_layout_f32(copy, null_mut()), 1);
            assert_eq!(
                liblrhsmm_rs_prepared_stream_parameters_f32(copy, null_mut()),
                1
            );
            assert_eq!(
                liblrhsmm_rs_prepared_stream_get_emission_f32(copy, 0, null_mut()),
                1
            );
            assert_eq!(liblrhsmm_rs_stream_release_f32(&mut editable), 0);
            assert_eq!(liblrhsmm_rs_prepared_stream_release_f32(&mut prepared), 0);
            assert_eq!(liblrhsmm_rs_prepared_stream_release_f32(&mut copy), 0);
        }
        assert!(prepared.is_null());
        assert_eq!(liblrhsmm_rs_stream_release_f32(&mut stream), 0);
        assert_eq!(liblrhsmm_rs_prepared_stream_release_f32(&mut prepared), 0);
        assert_eq!(liblrhsmm_rs_prepared_stream_release_f32(null_mut()), 1);
        result
    }
}

unsafe fn prepare_through_stream_f64(
    source: *const LiblrhsmmRsGaussianF64,
    output: *mut *mut LiblrhsmmRsPreparedGaussianF64,
) -> u32 {
    // SAFETY: Caller provides a live source and independent initialized output; all local owners are released.
    unsafe {
        let mut stream = null_mut();
        let mut prepared = null_mut();
        let mut copy = null_mut();
        let mut editable = null_mut();
        let children = [source];
        assert_eq!(
            liblrhsmm_rs_stream_create_f64(0.75, children.as_ptr(), 1, &mut stream),
            0
        );
        let result = liblrhsmm_rs_stream_prepare_f64(stream, &mut prepared);
        if result == 0 {
            let mut layout = LiblrhsmmRsStreamLayoutF64 {
                weight: 99.0,
                emissions: 99,
            };
            assert_eq!(
                liblrhsmm_rs_prepared_stream_layout_f64(prepared, &mut layout),
                0
            );
            assert_eq!(
                layout,
                LiblrhsmmRsStreamLayoutF64 {
                    weight: 0.75,
                    emissions: 1
                }
            );
            assert_eq!(
                liblrhsmm_rs_prepared_stream_parameters_f64(prepared, &mut editable),
                0
            );
            assert_eq!(liblrhsmm_rs_stream_layout_f64(editable, &mut layout), 0);
            assert_eq!(layout.weight, 0.75);
            assert_eq!(
                liblrhsmm_rs_prepared_stream_clone_f64(prepared, &mut copy),
                0
            );
            assert_eq!(
                liblrhsmm_rs_prepared_stream_get_emission_f64(copy, 1, output),
                2
            );
            assert_eq!(
                liblrhsmm_rs_prepared_stream_get_emission_f64(copy, 0, output),
                0
            );
            assert_eq!(liblrhsmm_rs_prepared_stream_layout_f64(copy, null_mut()), 1);
            assert_eq!(
                liblrhsmm_rs_prepared_stream_parameters_f64(copy, null_mut()),
                1
            );
            assert_eq!(
                liblrhsmm_rs_prepared_stream_get_emission_f64(copy, 0, null_mut()),
                1
            );
            assert_eq!(liblrhsmm_rs_stream_release_f64(&mut editable), 0);
            assert_eq!(liblrhsmm_rs_prepared_stream_release_f64(&mut prepared), 0);
            assert_eq!(liblrhsmm_rs_prepared_stream_release_f64(&mut copy), 0);
        }
        assert!(prepared.is_null());
        assert_eq!(liblrhsmm_rs_stream_release_f64(&mut stream), 0);
        assert_eq!(liblrhsmm_rs_prepared_stream_release_f64(&mut prepared), 0);
        assert_eq!(liblrhsmm_rs_prepared_stream_release_f64(null_mut()), 1);
        result
    }
}
