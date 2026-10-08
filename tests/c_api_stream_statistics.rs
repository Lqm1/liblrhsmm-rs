#![cfg(feature = "c-api")]
use liblrhsmm_rs::c_api::*;
use std::ptr::{null, null_mut};
#[test]
fn stream_f32_updates_original_c_values() {
    // SAFETY: All buffers independent; live owners exclusively mutated; released once.
    unsafe {
        let mut compared = 0;
        for line in include_str!("fixtures/c-updates.txt")
            .lines()
            .filter(|line| line.starts_with("mixture "))
        {
            let fields: Vec<_> = line.split_whitespace().collect();
            let scenario: usize = fields[1].parse().unwrap();
            let component: usize = fields[2].parse().unwrap();
            let components = if scenario == 0 { 1 } else { 2 };
            let mut source: *mut LiblrhsmmRsGaussianF32 = null_mut();
            let mut statistics: *mut LiblrhsmmRsGaussianStatisticsF32 = null_mut();
            assert_eq!(liblrhsmm_rs_gaussian_new_f32(components, 2, &mut source), 0);
            let weights = vec![0.5; components];
            let means = vec![-7.0; components * 2];
            let variances = vec![8.0; components * 2];
            let mut floors = vec![0.0; components * 2];
            let mut sums = vec![0.0; components * 2];
            let mut squares = vec![0.0; components * 2];
            let mut occupancies = vec![0.0; components];
            for k in 0..components {
                sums[2 * k..2 * k + 2].copy_from_slice(&[-3.5, 6.0]);
                squares[2 * k..2 * k + 2].copy_from_slice(&[9.25, 12.0]);
                floors[2 * k + 1] = 0.75;
                occupancies[k] = if scenario == 2 && k == 1 {
                    1e-8
                } else {
                    2.0 + k as f32
                };
            }
            let params = LiblrhsmmRsGaussianParametersF32 {
                mixtures: components,
                dimensions: 2,
                weights: weights.as_ptr(),
                means: means.as_ptr(),
                variances: variances.as_ptr(),
                variance_floors: floors.as_ptr(),
            };
            assert_eq!(liblrhsmm_rs_gaussian_replace_f32(source, &params), 0);
            let values = LiblrhsmmRsGaussianStatisticsValuesF32 {
                mixtures: components,
                dimensions: 2,
                state_occupancy: if scenario == 3 { 1e-8 } else { 4.0 },
                weighted_sums: sums.as_ptr(),
                weighted_squared_sums: squares.as_ptr(),
                component_occupancies: occupancies.as_ptr(),
            };
            assert_eq!(
                liblrhsmm_rs_gaussian_statistics_create_f32(&values, &mut statistics),
                0
            );
            let mut options = LiblrhsmmRsUpdateOptionsF32 {
                duration_mode: 0,
                duration_variance_floor: 0.0,
                emission_variance_floor: 0.0,
            };
            assert_eq!(liblrhsmm_rs_update_options_default_f32(&mut options), 0);
            let mut stream = null_mut();
            let mut stream_statistics = null_mut();
            let children = [source.cast_const()];
            let accumulators = [statistics.cast_const()];
            assert_eq!(
                liblrhsmm_rs_stream_create_f32(1.0, children.as_ptr(), 1, &mut stream),
                0
            );
            assert_eq!(
                liblrhsmm_rs_stream_statistics_create_f32(
                    accumulators.as_ptr(),
                    1,
                    &mut stream_statistics
                ),
                0
            );
            assert_eq!(
                liblrhsmm_rs_stream_update_f32(stream, stream_statistics, &options),
                0
            );
            assert_eq!(liblrhsmm_rs_gaussian_release_f32(&mut source), 0);
            assert_eq!(
                liblrhsmm_rs_stream_get_emission_f32(stream, 0, &mut source),
                0
            );
            let mut w = 0.0;
            let mut m = [0.0; 2];
            let mut v = [0.0; 2];
            assert_eq!(
                liblrhsmm_rs_gaussian_copy_field_f32(source, 0, component, &mut w, 1),
                0
            );
            assert_eq!(
                liblrhsmm_rs_gaussian_copy_field_f32(source, 1, 2 * component, m.as_mut_ptr(), 2),
                0
            );
            assert_eq!(
                liblrhsmm_rs_gaussian_copy_field_f32(source, 2, 2 * component, v.as_mut_ptr(), 2),
                0
            );
            let actual = [w, m[0], v[0], m[1], v[1]];
            for (actual, text) in actual.into_iter().zip(&fields[3..]) {
                let expected: f32 = text.parse().unwrap();
                assert!(
                    (f64::from(actual) - f64::from(expected)).abs()
                        <= 2e-14 * f64::from(expected).abs().max(1.0)
                );
                compared += 1;
            }
            assert_eq!(liblrhsmm_rs_gaussian_release_f32(&mut source), 0);
            assert_eq!(
                liblrhsmm_rs_gaussian_statistics_release_f32(&mut statistics),
                0
            );
            assert_eq!(
                liblrhsmm_rs_stream_statistics_release_f32(&mut stream_statistics),
                0
            );
            assert_eq!(liblrhsmm_rs_stream_release_f32(&mut stream), 0);
        }
        assert_eq!(compared, 35);
    }
}

unsafe fn check_f32(
    owner: *const LiblrhsmmRsStreamStatisticsF32,
    expected: &[(usize, usize, f32, [f32; 3])],
) {
    // SAFETY: Live readable owner and independent outputs; all returned children uniquely released.
    unsafe {
        let mut count = 99;
        assert_eq!(
            liblrhsmm_rs_stream_statistics_count_f32(owner, &mut count),
            0
        );
        assert_eq!(count, expected.len());
        for (index, &(mixtures, dimensions, occupancy, fields)) in expected.iter().enumerate() {
            let mut child = null_mut();
            assert_eq!(
                liblrhsmm_rs_stream_statistics_get_emission_f32(owner, index, &mut child),
                0
            );
            let mut layout = LiblrhsmmRsGaussianStatisticsLayoutF32 {
                mixtures: 99,
                dimensions: 99,
                state_occupancy: 99.0,
            };
            assert_eq!(
                liblrhsmm_rs_gaussian_statistics_layout_f32(child, &mut layout),
                0
            );
            assert_eq!((layout.mixtures, layout.dimensions), (mixtures, dimensions));
            assert_eq!(layout.state_occupancy.to_bits(), occupancy.to_bits());
            for (field, value) in fields.into_iter().enumerate() {
                let length = if field == 2 {
                    mixtures
                } else {
                    mixtures * dimensions
                };
                let mut values = vec![0.0_f32; length];
                assert_eq!(
                    liblrhsmm_rs_gaussian_statistics_copy_field_f32(
                        child,
                        field as u32,
                        0,
                        values.as_mut_ptr(),
                        length
                    ),
                    0
                );
                assert!(values.iter().all(|x| x.to_bits() == value.to_bits()));
            }
            assert_eq!(liblrhsmm_rs_gaussian_statistics_release_f32(&mut child), 0);
        }
    }
}
#[test]
fn stream_f32_complete_statistics_ownership_clear_and_late_update_failure() {
    // SAFETY: Initialized independent buffers; exclusive mutation and unique owner releases.
    unsafe {
        let mut stream = null_mut();
        let mut owner = null_mut();
        assert_eq!(liblrhsmm_rs_stream_new_f32(2, 1, 1, &mut stream), 0);
        assert_eq!(
            liblrhsmm_rs_stream_statistics_from_stream_f32(stream, &mut owner),
            0
        );
        check_f32(owner, &[(1, 1, 0.0, [0.0; 3]); 2]);
        let mut a = null_mut();
        let mut b = null_mut();
        assert_eq!(liblrhsmm_rs_gaussian_statistics_new_f32(1, 1, &mut a), 0);
        assert_eq!(liblrhsmm_rs_gaussian_statistics_new_f32(2, 3, &mut b), 0);
        let special = f32::from_bits(0x7fc01234);
        for (child, mixtures, dimensions) in [(a, 1, 1), (b, 2, 3)] {
            for (field, value) in [special, -0.0, f32::INFINITY].into_iter().enumerate() {
                let count = if field == 2 {
                    mixtures
                } else {
                    mixtures * dimensions
                };
                let values = vec![value; count];
                assert_eq!(
                    liblrhsmm_rs_gaussian_statistics_set_field_f32(
                        child,
                        field as u32,
                        0,
                        values.as_ptr(),
                        count
                    ),
                    0
                );
            }
            assert_eq!(
                liblrhsmm_rs_gaussian_statistics_set_occupancy_f32(child, special),
                0
            );
        }
        let inputs = [a.cast_const(), b.cast_const(), a.cast_const()];
        assert_eq!(
            liblrhsmm_rs_stream_statistics_replace_f32(owner, inputs.as_ptr(), 3),
            0
        );
        let expected = [
            (1, 1, special, [special, -0.0, f32::INFINITY]),
            (2, 3, special, [special, -0.0, f32::INFINITY]),
            (1, 1, special, [special, -0.0, f32::INFINITY]),
        ];
        check_f32(owner, &expected);
        let mut created = null_mut();
        let mut cloned = null_mut();
        assert_eq!(
            liblrhsmm_rs_stream_statistics_create_f32(inputs.as_ptr(), 3, &mut created),
            0
        );
        assert_eq!(
            liblrhsmm_rs_stream_statistics_clone_f32(owner, &mut cloned),
            0
        );
        let bad = [a.cast_const(), null()];
        assert_eq!(
            liblrhsmm_rs_stream_statistics_replace_f32(owner, bad.as_ptr(), 2),
            1
        );
        check_f32(owner, &expected);
        let mut failure = owner;
        assert_eq!(
            liblrhsmm_rs_stream_statistics_create_f32(null(), 1, &mut failure),
            1
        );
        assert_eq!(failure, owner);
        assert_eq!(
            liblrhsmm_rs_stream_statistics_replace_f32(owner, null(), i32::MAX as usize + 1),
            2
        );
        assert_eq!(
            liblrhsmm_rs_stream_statistics_set_emission_f32(owner, 3, a),
            2
        );
        assert_eq!(
            liblrhsmm_rs_stream_statistics_set_emission_f32(owner, 0, null()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_stream_statistics_set_emission_f32(owner, 0, b),
            0
        );
        let mut child = null_mut();
        assert_eq!(
            liblrhsmm_rs_stream_statistics_get_emission_f32(owner, 0, &mut child),
            0
        );
        assert_eq!(liblrhsmm_rs_gaussian_statistics_clear_f32(child), 0);
        assert_eq!(liblrhsmm_rs_gaussian_statistics_release_f32(&mut child), 0);
        check_f32(owner, &[expected[1], expected[1], expected[2]]);
        assert_eq!(liblrhsmm_rs_stream_statistics_clear_f32(owner), 0);
        check_f32(
            owner,
            &[
                (2, 3, 0.0, [0.0; 3]),
                (2, 3, 0.0, [0.0; 3]),
                (1, 1, 0.0, [0.0; 3]),
            ],
        );
        assert_eq!(liblrhsmm_rs_gaussian_statistics_release_f32(&mut a), 0);
        assert_eq!(liblrhsmm_rs_gaussian_statistics_release_f32(&mut b), 0);
        check_f32(created, &expected);
        check_f32(cloned, &expected);
        assert_eq!(liblrhsmm_rs_stream_statistics_release_f32(&mut created), 0);
        assert_eq!(liblrhsmm_rs_stream_statistics_release_f32(&mut cloned), 0);
        assert_eq!(liblrhsmm_rs_stream_statistics_release_f32(&mut owner), 0);
        assert_eq!(
            liblrhsmm_rs_stream_statistics_from_stream_f32(stream, &mut owner),
            0
        );
        let mut options = LiblrhsmmRsUpdateOptionsF32 {
            duration_mode: 0,
            duration_variance_floor: 0.0,
            emission_variance_floor: 0.0,
        };
        assert_eq!(liblrhsmm_rs_update_options_default_f32(&mut options), 0);
        // A valid first emission would change its flat-start mean to 2; invalid second must retain both.
        let mut good = null_mut();
        let mut invalid = null_mut();
        assert_eq!(liblrhsmm_rs_gaussian_statistics_new_f32(1, 1, &mut good), 0);
        let sum = [4.0_f32];
        let square = [10.0_f32];
        let occupancy = [2.0_f32];
        for (field, values) in [sum, square, occupancy].iter().enumerate() {
            assert_eq!(
                liblrhsmm_rs_gaussian_statistics_set_field_f32(
                    good,
                    field as u32,
                    0,
                    values.as_ptr(),
                    1
                ),
                0
            );
        }
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_set_occupancy_f32(good, 2.0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_clone_f32(good, &mut invalid),
            0
        );
        let overflow = [f32::MAX];
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_set_field_f32(invalid, 0, 0, overflow.as_ptr(), 1),
            0
        );
        assert_eq!(
            liblrhsmm_rs_stream_statistics_set_emission_f32(owner, 0, good),
            0
        );
        assert_eq!(
            liblrhsmm_rs_stream_statistics_set_emission_f32(owner, 1, invalid),
            0
        );
        assert_eq!(liblrhsmm_rs_stream_update_f32(stream, owner, &options), 3);
        for index in 0..2 {
            let mut gaussian = null_mut();
            assert_eq!(
                liblrhsmm_rs_stream_get_emission_f32(stream, index, &mut gaussian),
                0
            );
            for (selector, expected) in [1.0_f32, 0.0, 0.0, 0.0].into_iter().enumerate() {
                let mut result = 99.0_f32;
                assert_eq!(
                    liblrhsmm_rs_gaussian_copy_field_f32(
                        gaussian,
                        selector as u32,
                        0,
                        &mut result,
                        1
                    ),
                    0
                );
                assert_eq!(result.to_bits(), expected.to_bits());
            }
            assert_eq!(liblrhsmm_rs_gaussian_release_f32(&mut gaussian), 0);
        }
        assert_eq!(
            liblrhsmm_rs_stream_statistics_set_emission_f32(owner, 1, good),
            0
        );
        assert_eq!(liblrhsmm_rs_stream_update_f32(stream, owner, &options), 0);
        options.duration_mode = 99;
        assert_eq!(liblrhsmm_rs_stream_update_f32(stream, owner, &options), 3);
        assert_eq!(liblrhsmm_rs_stream_update_f32(stream, owner, null()), 1);
        assert_eq!(liblrhsmm_rs_gaussian_statistics_release_f32(&mut good), 0);
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_release_f32(&mut invalid),
            0
        );
        assert_eq!(liblrhsmm_rs_stream_release_f32(&mut stream), 0);
        assert_eq!(liblrhsmm_rs_stream_statistics_clear_f32(owner), 0);
        assert_eq!(
            liblrhsmm_rs_stream_statistics_get_emission_f32(owner, 2, &mut child),
            2
        );
        assert!(child.is_null());
        assert_eq!(liblrhsmm_rs_stream_statistics_release_f32(&mut owner), 0);
        assert!(owner.is_null());
        assert_eq!(liblrhsmm_rs_stream_statistics_release_f32(&mut owner), 0);
        assert_eq!(
            liblrhsmm_rs_stream_statistics_create_f32(null(), 0, &mut owner),
            0
        );
        check_f32(owner, &[]);
        assert_eq!(
            liblrhsmm_rs_stream_statistics_replace_f32(owner, null(), 0),
            0
        );
        assert_eq!(liblrhsmm_rs_stream_new_f32(0, 0, 0, &mut stream), 0);
        options.duration_mode = 0;
        options.emission_variance_floor = -1.0;
        assert_eq!(liblrhsmm_rs_stream_update_f32(stream, owner, &options), 3);
        options.emission_variance_floor = 0.0;
        assert_eq!(liblrhsmm_rs_stream_update_f32(stream, owner, &options), 0);
        assert_eq!(liblrhsmm_rs_stream_release_f32(&mut stream), 0);
        assert_eq!(liblrhsmm_rs_stream_statistics_release_f32(&mut owner), 0);
        assert_eq!(liblrhsmm_rs_stream_statistics_release_f32(null_mut()), 1);
        assert_eq!(
            liblrhsmm_rs_stream_statistics_count_f32(null(), null_mut()),
            1
        );
    }
}
#[test]
fn stream_f64_updates_original_c_values() {
    // SAFETY: All buffers independent; live owners exclusively mutated; released once.
    unsafe {
        let mut compared = 0;
        for line in include_str!("fixtures/c-double-update.txt")
            .lines()
            .filter(|line| line.starts_with("mixture "))
        {
            let fields: Vec<_> = line.split_whitespace().collect();
            let scenario: usize = fields[1].parse().unwrap();
            let component: usize = fields[2].parse().unwrap();
            let components = if scenario == 0 { 1 } else { 2 };
            let mut source: *mut LiblrhsmmRsGaussianF64 = null_mut();
            let mut statistics: *mut LiblrhsmmRsGaussianStatisticsF64 = null_mut();
            assert_eq!(liblrhsmm_rs_gaussian_new_f64(components, 2, &mut source), 0);
            let weights = vec![0.5; components];
            let means = vec![-7.0; components * 2];
            let variances = vec![8.0; components * 2];
            let mut floors = vec![0.0; components * 2];
            let mut sums = vec![0.0; components * 2];
            let mut squares = vec![0.0; components * 2];
            let mut occupancies = vec![0.0; components];
            for k in 0..components {
                sums[2 * k..2 * k + 2].copy_from_slice(&[-3.5, 6.0]);
                squares[2 * k..2 * k + 2].copy_from_slice(&[9.25, 12.0]);
                floors[2 * k + 1] = 0.75;
                occupancies[k] = if scenario == 2 && k == 1 {
                    1e-8
                } else {
                    2.0 + k as f64
                };
            }
            let params = LiblrhsmmRsGaussianParametersF64 {
                mixtures: components,
                dimensions: 2,
                weights: weights.as_ptr(),
                means: means.as_ptr(),
                variances: variances.as_ptr(),
                variance_floors: floors.as_ptr(),
            };
            assert_eq!(liblrhsmm_rs_gaussian_replace_f64(source, &params), 0);
            let values = LiblrhsmmRsGaussianStatisticsValuesF64 {
                mixtures: components,
                dimensions: 2,
                state_occupancy: if scenario == 3 { 1e-8 } else { 4.0 },
                weighted_sums: sums.as_ptr(),
                weighted_squared_sums: squares.as_ptr(),
                component_occupancies: occupancies.as_ptr(),
            };
            assert_eq!(
                liblrhsmm_rs_gaussian_statistics_create_f64(&values, &mut statistics),
                0
            );
            let mut options = LiblrhsmmRsUpdateOptionsF64 {
                duration_mode: 0,
                duration_variance_floor: 0.0,
                emission_variance_floor: 0.0,
            };
            assert_eq!(liblrhsmm_rs_update_options_default_f64(&mut options), 0);
            let mut stream = null_mut();
            let mut stream_statistics = null_mut();
            let children = [source.cast_const()];
            let accumulators = [statistics.cast_const()];
            assert_eq!(
                liblrhsmm_rs_stream_create_f64(1.0, children.as_ptr(), 1, &mut stream),
                0
            );
            assert_eq!(
                liblrhsmm_rs_stream_statistics_create_f64(
                    accumulators.as_ptr(),
                    1,
                    &mut stream_statistics
                ),
                0
            );
            assert_eq!(
                liblrhsmm_rs_stream_update_f64(stream, stream_statistics, &options),
                0
            );
            assert_eq!(liblrhsmm_rs_gaussian_release_f64(&mut source), 0);
            assert_eq!(
                liblrhsmm_rs_stream_get_emission_f64(stream, 0, &mut source),
                0
            );
            let mut w = 0.0;
            let mut m = [0.0; 2];
            let mut v = [0.0; 2];
            assert_eq!(
                liblrhsmm_rs_gaussian_copy_field_f64(source, 0, component, &mut w, 1),
                0
            );
            assert_eq!(
                liblrhsmm_rs_gaussian_copy_field_f64(source, 1, 2 * component, m.as_mut_ptr(), 2),
                0
            );
            assert_eq!(
                liblrhsmm_rs_gaussian_copy_field_f64(source, 2, 2 * component, v.as_mut_ptr(), 2),
                0
            );
            let actual = [w, m[0], v[0], m[1], v[1]];
            for (actual, text) in actual.into_iter().zip(&fields[3..]) {
                let expected: f64 = text.parse().unwrap();
                assert!((actual - expected).abs() <= 2e-14 * expected.abs().max(1.0));
                compared += 1;
            }
            assert_eq!(liblrhsmm_rs_gaussian_release_f64(&mut source), 0);
            assert_eq!(
                liblrhsmm_rs_gaussian_statistics_release_f64(&mut statistics),
                0
            );
            assert_eq!(
                liblrhsmm_rs_stream_statistics_release_f64(&mut stream_statistics),
                0
            );
            assert_eq!(liblrhsmm_rs_stream_release_f64(&mut stream), 0);
        }
        assert_eq!(compared, 35);
    }
}

unsafe fn check_f64(
    owner: *const LiblrhsmmRsStreamStatisticsF64,
    expected: &[(usize, usize, f64, [f64; 3])],
) {
    // SAFETY: Live readable owner and independent outputs; all returned children uniquely released.
    unsafe {
        let mut count = 99;
        assert_eq!(
            liblrhsmm_rs_stream_statistics_count_f64(owner, &mut count),
            0
        );
        assert_eq!(count, expected.len());
        for (index, &(mixtures, dimensions, occupancy, fields)) in expected.iter().enumerate() {
            let mut child = null_mut();
            assert_eq!(
                liblrhsmm_rs_stream_statistics_get_emission_f64(owner, index, &mut child),
                0
            );
            let mut layout = LiblrhsmmRsGaussianStatisticsLayoutF64 {
                mixtures: 99,
                dimensions: 99,
                state_occupancy: 99.0,
            };
            assert_eq!(
                liblrhsmm_rs_gaussian_statistics_layout_f64(child, &mut layout),
                0
            );
            assert_eq!((layout.mixtures, layout.dimensions), (mixtures, dimensions));
            assert_eq!(layout.state_occupancy.to_bits(), occupancy.to_bits());
            for (field, value) in fields.into_iter().enumerate() {
                let length = if field == 2 {
                    mixtures
                } else {
                    mixtures * dimensions
                };
                let mut values = vec![0.0_f64; length];
                assert_eq!(
                    liblrhsmm_rs_gaussian_statistics_copy_field_f64(
                        child,
                        field as u32,
                        0,
                        values.as_mut_ptr(),
                        length
                    ),
                    0
                );
                assert!(values.iter().all(|x| x.to_bits() == value.to_bits()));
            }
            assert_eq!(liblrhsmm_rs_gaussian_statistics_release_f64(&mut child), 0);
        }
    }
}
#[test]
fn stream_f64_complete_statistics_ownership_clear_and_late_update_failure() {
    // SAFETY: Initialized independent buffers; exclusive mutation and unique owner releases.
    unsafe {
        let mut stream = null_mut();
        let mut owner = null_mut();
        assert_eq!(liblrhsmm_rs_stream_new_f64(2, 1, 1, &mut stream), 0);
        assert_eq!(
            liblrhsmm_rs_stream_statistics_from_stream_f64(stream, &mut owner),
            0
        );
        check_f64(owner, &[(1, 1, 0.0, [0.0; 3]); 2]);
        let mut a = null_mut();
        let mut b = null_mut();
        assert_eq!(liblrhsmm_rs_gaussian_statistics_new_f64(1, 1, &mut a), 0);
        assert_eq!(liblrhsmm_rs_gaussian_statistics_new_f64(2, 3, &mut b), 0);
        let special = f64::from_bits(0x7ff8000000001234);
        for (child, mixtures, dimensions) in [(a, 1, 1), (b, 2, 3)] {
            for (field, value) in [special, -0.0, f64::INFINITY].into_iter().enumerate() {
                let count = if field == 2 {
                    mixtures
                } else {
                    mixtures * dimensions
                };
                let values = vec![value; count];
                assert_eq!(
                    liblrhsmm_rs_gaussian_statistics_set_field_f64(
                        child,
                        field as u32,
                        0,
                        values.as_ptr(),
                        count
                    ),
                    0
                );
            }
            assert_eq!(
                liblrhsmm_rs_gaussian_statistics_set_occupancy_f64(child, special),
                0
            );
        }
        let inputs = [a.cast_const(), b.cast_const(), a.cast_const()];
        assert_eq!(
            liblrhsmm_rs_stream_statistics_replace_f64(owner, inputs.as_ptr(), 3),
            0
        );
        let expected = [
            (1, 1, special, [special, -0.0, f64::INFINITY]),
            (2, 3, special, [special, -0.0, f64::INFINITY]),
            (1, 1, special, [special, -0.0, f64::INFINITY]),
        ];
        check_f64(owner, &expected);
        let mut created = null_mut();
        let mut cloned = null_mut();
        assert_eq!(
            liblrhsmm_rs_stream_statistics_create_f64(inputs.as_ptr(), 3, &mut created),
            0
        );
        assert_eq!(
            liblrhsmm_rs_stream_statistics_clone_f64(owner, &mut cloned),
            0
        );
        let bad = [a.cast_const(), null()];
        assert_eq!(
            liblrhsmm_rs_stream_statistics_replace_f64(owner, bad.as_ptr(), 2),
            1
        );
        check_f64(owner, &expected);
        let mut failure = owner;
        assert_eq!(
            liblrhsmm_rs_stream_statistics_create_f64(null(), 1, &mut failure),
            1
        );
        assert_eq!(failure, owner);
        assert_eq!(
            liblrhsmm_rs_stream_statistics_replace_f64(owner, null(), i32::MAX as usize + 1),
            2
        );
        assert_eq!(
            liblrhsmm_rs_stream_statistics_set_emission_f64(owner, 3, a),
            2
        );
        assert_eq!(
            liblrhsmm_rs_stream_statistics_set_emission_f64(owner, 0, null()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_stream_statistics_set_emission_f64(owner, 0, b),
            0
        );
        let mut child = null_mut();
        assert_eq!(
            liblrhsmm_rs_stream_statistics_get_emission_f64(owner, 0, &mut child),
            0
        );
        assert_eq!(liblrhsmm_rs_gaussian_statistics_clear_f64(child), 0);
        assert_eq!(liblrhsmm_rs_gaussian_statistics_release_f64(&mut child), 0);
        check_f64(owner, &[expected[1], expected[1], expected[2]]);
        assert_eq!(liblrhsmm_rs_stream_statistics_clear_f64(owner), 0);
        check_f64(
            owner,
            &[
                (2, 3, 0.0, [0.0; 3]),
                (2, 3, 0.0, [0.0; 3]),
                (1, 1, 0.0, [0.0; 3]),
            ],
        );
        assert_eq!(liblrhsmm_rs_gaussian_statistics_release_f64(&mut a), 0);
        assert_eq!(liblrhsmm_rs_gaussian_statistics_release_f64(&mut b), 0);
        check_f64(created, &expected);
        check_f64(cloned, &expected);
        assert_eq!(liblrhsmm_rs_stream_statistics_release_f64(&mut created), 0);
        assert_eq!(liblrhsmm_rs_stream_statistics_release_f64(&mut cloned), 0);
        assert_eq!(liblrhsmm_rs_stream_statistics_release_f64(&mut owner), 0);
        assert_eq!(
            liblrhsmm_rs_stream_statistics_from_stream_f64(stream, &mut owner),
            0
        );
        let mut options = LiblrhsmmRsUpdateOptionsF64 {
            duration_mode: 0,
            duration_variance_floor: 0.0,
            emission_variance_floor: 0.0,
        };
        assert_eq!(liblrhsmm_rs_update_options_default_f64(&mut options), 0);
        // A valid first emission would change its flat-start mean to 2; invalid second must retain both.
        let mut good = null_mut();
        let mut invalid = null_mut();
        assert_eq!(liblrhsmm_rs_gaussian_statistics_new_f64(1, 1, &mut good), 0);
        let sum = [4.0_f64];
        let square = [10.0_f64];
        let occupancy = [2.0_f64];
        for (field, values) in [sum, square, occupancy].iter().enumerate() {
            assert_eq!(
                liblrhsmm_rs_gaussian_statistics_set_field_f64(
                    good,
                    field as u32,
                    0,
                    values.as_ptr(),
                    1
                ),
                0
            );
        }
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_set_occupancy_f64(good, 2.0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_clone_f64(good, &mut invalid),
            0
        );
        let overflow = [f64::MAX];
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_set_field_f64(invalid, 0, 0, overflow.as_ptr(), 1),
            0
        );
        assert_eq!(
            liblrhsmm_rs_stream_statistics_set_emission_f64(owner, 0, good),
            0
        );
        assert_eq!(
            liblrhsmm_rs_stream_statistics_set_emission_f64(owner, 1, invalid),
            0
        );
        assert_eq!(liblrhsmm_rs_stream_update_f64(stream, owner, &options), 3);
        for index in 0..2 {
            let mut gaussian = null_mut();
            assert_eq!(
                liblrhsmm_rs_stream_get_emission_f64(stream, index, &mut gaussian),
                0
            );
            for (selector, expected) in [1.0_f64, 0.0, 0.0, 0.0].into_iter().enumerate() {
                let mut result = 99.0_f64;
                assert_eq!(
                    liblrhsmm_rs_gaussian_copy_field_f64(
                        gaussian,
                        selector as u32,
                        0,
                        &mut result,
                        1
                    ),
                    0
                );
                assert_eq!(result.to_bits(), expected.to_bits());
            }
            assert_eq!(liblrhsmm_rs_gaussian_release_f64(&mut gaussian), 0);
        }
        assert_eq!(
            liblrhsmm_rs_stream_statistics_set_emission_f64(owner, 1, good),
            0
        );
        assert_eq!(liblrhsmm_rs_stream_update_f64(stream, owner, &options), 0);
        options.duration_mode = 99;
        assert_eq!(liblrhsmm_rs_stream_update_f64(stream, owner, &options), 3);
        assert_eq!(liblrhsmm_rs_stream_update_f64(stream, owner, null()), 1);
        assert_eq!(liblrhsmm_rs_gaussian_statistics_release_f64(&mut good), 0);
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_release_f64(&mut invalid),
            0
        );
        assert_eq!(liblrhsmm_rs_stream_release_f64(&mut stream), 0);
        assert_eq!(liblrhsmm_rs_stream_statistics_clear_f64(owner), 0);
        assert_eq!(
            liblrhsmm_rs_stream_statistics_get_emission_f64(owner, 2, &mut child),
            2
        );
        assert!(child.is_null());
        assert_eq!(liblrhsmm_rs_stream_statistics_release_f64(&mut owner), 0);
        assert!(owner.is_null());
        assert_eq!(liblrhsmm_rs_stream_statistics_release_f64(&mut owner), 0);
        assert_eq!(
            liblrhsmm_rs_stream_statistics_create_f64(null(), 0, &mut owner),
            0
        );
        check_f64(owner, &[]);
        assert_eq!(
            liblrhsmm_rs_stream_statistics_replace_f64(owner, null(), 0),
            0
        );
        assert_eq!(liblrhsmm_rs_stream_new_f64(0, 0, 0, &mut stream), 0);
        options.duration_mode = 0;
        options.emission_variance_floor = -1.0;
        assert_eq!(liblrhsmm_rs_stream_update_f64(stream, owner, &options), 3);
        options.emission_variance_floor = 0.0;
        assert_eq!(liblrhsmm_rs_stream_update_f64(stream, owner, &options), 0);
        assert_eq!(liblrhsmm_rs_stream_release_f64(&mut stream), 0);
        assert_eq!(liblrhsmm_rs_stream_statistics_release_f64(&mut owner), 0);
        assert_eq!(liblrhsmm_rs_stream_statistics_release_f64(null_mut()), 1);
        assert_eq!(
            liblrhsmm_rs_stream_statistics_count_f64(null(), null_mut()),
            1
        );
    }
}
