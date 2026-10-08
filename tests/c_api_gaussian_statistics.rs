#![cfg(feature = "c-api")]
use liblrhsmm_rs::c_api::*;
use std::ptr::{null, null_mut};

#[test]
fn statistics_f32_fields_independent_storage_shapes_and_clear() {
    // SAFETY: Independent initialized buffers, exclusive owners and exactly one release per owner.
    unsafe {
        let mut source: *mut LiblrhsmmRsGaussianF32 = null_mut();
        let mut statistics: *mut LiblrhsmmRsGaussianStatisticsF32 = null_mut();
        let mut copy: *mut LiblrhsmmRsGaussianStatisticsF32 = null_mut();
        let mut created: *mut LiblrhsmmRsGaussianStatisticsF32 = null_mut();
        assert_eq!(liblrhsmm_rs_gaussian_new_f32(2, 2, &mut source), 0);
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_from_gaussian_f32(source, &mut statistics),
            0
        );
        assert_eq!(liblrhsmm_rs_gaussian_release_f32(&mut source), 0);
        let mut metadata = LiblrhsmmRsGaussianStatisticsLayoutF32 {
            mixtures: 0,
            dimensions: 0,
            state_occupancy: 99.0,
        };
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_layout_f32(statistics, &mut metadata),
            0
        );
        assert_eq!(
            (
                metadata.mixtures,
                metadata.dimensions,
                metadata.state_occupancy
            ),
            (2, 2, 0.0)
        );
        let sums: [f32; 4] = [-1.0, 2.0, 3.0, 4.0];
        let squares: [f32; 4] = [1.0, 4.0, 9.0, 16.0];
        let occupancy: [f32; 2] = [1.0, 2.0];
        let input = LiblrhsmmRsGaussianStatisticsValuesF32 {
            mixtures: 2,
            dimensions: 2,
            state_occupancy: 3.0,
            weighted_sums: sums.as_ptr(),
            weighted_squared_sums: squares.as_ptr(),
            component_occupancies: occupancy.as_ptr(),
        };
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_replace_f32(statistics, &input),
            0
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_create_f32(&input, &mut created),
            0
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_clone_f32(statistics, &mut copy),
            0
        );
        let special: [f32; 2] = [f32::from_bits(0x7fc12345), -0.0];
        let mut output = [0.0; 2];
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_set_field_f32(statistics, 0, 1, special.as_ptr(), 2),
            0
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_copy_field_f32(
                statistics,
                0,
                1,
                output.as_mut_ptr(),
                2
            ),
            0
        );
        assert_eq!(output.map(f32::to_bits), special.map(f32::to_bits));
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_set_occupancy_f32(statistics, -0.0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_layout_f32(statistics, &mut metadata),
            0
        );
        assert_eq!(metadata.state_occupancy.to_bits(), (-0.0_f32).to_bits());
        for (selector, expected) in [
            (0, sums.as_slice()),
            (1, squares.as_slice()),
            (2, occupancy.as_slice()),
        ] {
            let mut output = vec![99.0; expected.len()];
            assert_eq!(
                liblrhsmm_rs_gaussian_statistics_copy_field_f32(
                    copy,
                    selector,
                    0,
                    output.as_mut_ptr(),
                    output.len()
                ),
                0
            );
            assert_eq!(output, expected);
            assert_eq!(
                liblrhsmm_rs_gaussian_statistics_copy_field_f32(
                    copy,
                    selector,
                    expected.len(),
                    null_mut(),
                    0
                ),
                0
            );
        }
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_set_field_f32(copy, 0, 4, special.as_ptr(), 1),
            2
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_copy_field_f32(copy, 3, 0, null_mut(), 0),
            3
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_set_field_f32(copy, 0, usize::MAX, null(), 0),
            2
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_copy_field_f32(copy, 0, 0, null_mut(), 1),
            1
        );
        let invalid = LiblrhsmmRsGaussianStatisticsValuesF32 {
            dimensions: usize::MAX,
            ..input
        };
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_replace_f32(copy, &invalid),
            2
        );
        assert_eq!(liblrhsmm_rs_gaussian_statistics_clear_f32(statistics), 0);
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_layout_f32(statistics, &mut metadata),
            0
        );
        assert_eq!(metadata.state_occupancy, 0.0);
        for (selector, count) in [(0, 4), (1, 4), (2, 2)] {
            let mut output = vec![99.0; count];
            assert_eq!(
                liblrhsmm_rs_gaussian_statistics_copy_field_f32(
                    statistics,
                    selector,
                    0,
                    output.as_mut_ptr(),
                    count
                ),
                0
            );
            assert_eq!(output, vec![0.0; count]);
        }
        // Full replacement can change shape, with no temporary model allocation.
        let scalar = [7.0];
        let input = LiblrhsmmRsGaussianStatisticsValuesF32 {
            mixtures: 1,
            dimensions: 1,
            state_occupancy: 5.0,
            weighted_sums: scalar.as_ptr(),
            weighted_squared_sums: scalar.as_ptr(),
            component_occupancies: scalar.as_ptr(),
        };
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_replace_f32(statistics, &input),
            0
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_layout_f32(statistics, &mut metadata),
            0
        );
        assert_eq!(
            (
                metadata.mixtures,
                metadata.dimensions,
                metadata.state_occupancy
            ),
            (1, 1, 5.0)
        );
        for slot in [&mut statistics, &mut copy, &mut created] {
            assert_eq!(liblrhsmm_rs_gaussian_statistics_release_f32(slot), 0);
            assert!(slot.is_null());
        }
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_release_f32(&mut statistics),
            0
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_new_f32(1, 2, &mut statistics),
            0
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_release_f32(&mut statistics),
            0
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_new_f32(0, 1, &mut statistics),
            3
        );
        assert!(statistics.is_null());
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_create_f32(null(), &mut statistics),
            1
        );
    }
}
#[test]
fn statistics_f32_updates_original_c_values_and_atomic_errors() {
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
            let mut updated = 99;
            assert_eq!(
                liblrhsmm_rs_gaussian_update_f32(source, statistics, &options, &mut updated),
                0
            );
            assert_eq!(updated, u32::from(scenario != 3));
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
            // Force the final component to overflow after earlier staged component updates.
            assert_eq!(
                liblrhsmm_rs_gaussian_statistics_set_occupancy_f32(statistics, 4.0),
                0
            );
            let occupancy = vec![2.0; components];
            assert_eq!(
                liblrhsmm_rs_gaussian_statistics_set_field_f32(
                    statistics,
                    2,
                    0,
                    occupancy.as_ptr(),
                    components
                ),
                0
            );
            let overflow = [f32::MAX];
            assert_eq!(
                liblrhsmm_rs_gaussian_statistics_set_field_f32(
                    statistics,
                    0,
                    components * 2 - 1,
                    overflow.as_ptr(),
                    1
                ),
                0
            );
            let mut before = Vec::new();
            for (selector, count) in [
                (0, components),
                (1, components * 2),
                (2, components * 2),
                (3, components * 2),
            ] {
                let mut array = vec![0.0; count];
                assert_eq!(
                    liblrhsmm_rs_gaussian_copy_field_f32(
                        source,
                        selector,
                        0,
                        array.as_mut_ptr(),
                        count
                    ),
                    0
                );
                before.push(array);
            }
            updated = 99;
            assert_eq!(
                liblrhsmm_rs_gaussian_update_f32(source, statistics, &options, &mut updated),
                3
            );
            assert_eq!(updated, 99);
            assert_eq!(
                liblrhsmm_rs_gaussian_update_f32(source, statistics, &options, null_mut()),
                1
            );
            options.duration_mode = 99;
            assert_eq!(
                liblrhsmm_rs_gaussian_update_f32(source, statistics, &options, &mut updated),
                3
            );
            assert_eq!(updated, 99);
            for (selector, array) in before.into_iter().enumerate() {
                let mut after = vec![0.0; array.len()];
                assert_eq!(
                    liblrhsmm_rs_gaussian_copy_field_f32(
                        source,
                        selector as u32,
                        0,
                        after.as_mut_ptr(),
                        after.len()
                    ),
                    0
                );
                assert_eq!(after, array);
            }
            assert_eq!(
                liblrhsmm_rs_gaussian_statistics_release_f32(&mut statistics),
                0
            );
            assert_eq!(liblrhsmm_rs_gaussian_release_f32(&mut source), 0);
        }
        assert_eq!(compared, 35);
    }
}

#[test]
fn statistics_f64_fields_independent_storage_shapes_and_clear() {
    // SAFETY: Independent initialized buffers, exclusive owners and exactly one release per owner.
    unsafe {
        let mut source: *mut LiblrhsmmRsGaussianF64 = null_mut();
        let mut statistics: *mut LiblrhsmmRsGaussianStatisticsF64 = null_mut();
        let mut copy: *mut LiblrhsmmRsGaussianStatisticsF64 = null_mut();
        let mut created: *mut LiblrhsmmRsGaussianStatisticsF64 = null_mut();
        assert_eq!(liblrhsmm_rs_gaussian_new_f64(2, 2, &mut source), 0);
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_from_gaussian_f64(source, &mut statistics),
            0
        );
        assert_eq!(liblrhsmm_rs_gaussian_release_f64(&mut source), 0);
        let mut metadata = LiblrhsmmRsGaussianStatisticsLayoutF64 {
            mixtures: 0,
            dimensions: 0,
            state_occupancy: 99.0,
        };
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_layout_f64(statistics, &mut metadata),
            0
        );
        assert_eq!(
            (
                metadata.mixtures,
                metadata.dimensions,
                metadata.state_occupancy
            ),
            (2, 2, 0.0)
        );
        let sums: [f64; 4] = [-1.0, 2.0, 3.0, 4.0];
        let squares: [f64; 4] = [1.0, 4.0, 9.0, 16.0];
        let occupancy: [f64; 2] = [1.0, 2.0];
        let input = LiblrhsmmRsGaussianStatisticsValuesF64 {
            mixtures: 2,
            dimensions: 2,
            state_occupancy: 3.0,
            weighted_sums: sums.as_ptr(),
            weighted_squared_sums: squares.as_ptr(),
            component_occupancies: occupancy.as_ptr(),
        };
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_replace_f64(statistics, &input),
            0
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_create_f64(&input, &mut created),
            0
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_clone_f64(statistics, &mut copy),
            0
        );
        let special: [f64; 2] = [f64::from_bits(0x7ff8123456789abc), -0.0];
        let mut output = [0.0; 2];
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_set_field_f64(statistics, 0, 1, special.as_ptr(), 2),
            0
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_copy_field_f64(
                statistics,
                0,
                1,
                output.as_mut_ptr(),
                2
            ),
            0
        );
        assert_eq!(output.map(f64::to_bits), special.map(f64::to_bits));
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_set_occupancy_f64(statistics, -0.0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_layout_f64(statistics, &mut metadata),
            0
        );
        assert_eq!(metadata.state_occupancy.to_bits(), (-0.0_f64).to_bits());
        for (selector, expected) in [
            (0, sums.as_slice()),
            (1, squares.as_slice()),
            (2, occupancy.as_slice()),
        ] {
            let mut output = vec![99.0; expected.len()];
            assert_eq!(
                liblrhsmm_rs_gaussian_statistics_copy_field_f64(
                    copy,
                    selector,
                    0,
                    output.as_mut_ptr(),
                    output.len()
                ),
                0
            );
            assert_eq!(output, expected);
            assert_eq!(
                liblrhsmm_rs_gaussian_statistics_copy_field_f64(
                    copy,
                    selector,
                    expected.len(),
                    null_mut(),
                    0
                ),
                0
            );
        }
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_set_field_f64(copy, 0, 4, special.as_ptr(), 1),
            2
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_copy_field_f64(copy, 3, 0, null_mut(), 0),
            3
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_set_field_f64(copy, 0, usize::MAX, null(), 0),
            2
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_copy_field_f64(copy, 0, 0, null_mut(), 1),
            1
        );
        let invalid = LiblrhsmmRsGaussianStatisticsValuesF64 {
            dimensions: usize::MAX,
            ..input
        };
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_replace_f64(copy, &invalid),
            2
        );
        assert_eq!(liblrhsmm_rs_gaussian_statistics_clear_f64(statistics), 0);
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_layout_f64(statistics, &mut metadata),
            0
        );
        assert_eq!(metadata.state_occupancy, 0.0);
        for (selector, count) in [(0, 4), (1, 4), (2, 2)] {
            let mut output = vec![99.0; count];
            assert_eq!(
                liblrhsmm_rs_gaussian_statistics_copy_field_f64(
                    statistics,
                    selector,
                    0,
                    output.as_mut_ptr(),
                    count
                ),
                0
            );
            assert_eq!(output, vec![0.0; count]);
        }
        // Full replacement can change shape, with no temporary model allocation.
        let scalar = [7.0];
        let input = LiblrhsmmRsGaussianStatisticsValuesF64 {
            mixtures: 1,
            dimensions: 1,
            state_occupancy: 5.0,
            weighted_sums: scalar.as_ptr(),
            weighted_squared_sums: scalar.as_ptr(),
            component_occupancies: scalar.as_ptr(),
        };
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_replace_f64(statistics, &input),
            0
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_layout_f64(statistics, &mut metadata),
            0
        );
        assert_eq!(
            (
                metadata.mixtures,
                metadata.dimensions,
                metadata.state_occupancy
            ),
            (1, 1, 5.0)
        );
        for slot in [&mut statistics, &mut copy, &mut created] {
            assert_eq!(liblrhsmm_rs_gaussian_statistics_release_f64(slot), 0);
            assert!(slot.is_null());
        }
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_release_f64(&mut statistics),
            0
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_new_f64(1, 2, &mut statistics),
            0
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_release_f64(&mut statistics),
            0
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_new_f64(0, 1, &mut statistics),
            3
        );
        assert!(statistics.is_null());
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_create_f64(null(), &mut statistics),
            1
        );
    }
}
#[test]
fn statistics_f64_updates_original_c_values_and_atomic_errors() {
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
            let mut updated = 99;
            assert_eq!(
                liblrhsmm_rs_gaussian_update_f64(source, statistics, &options, &mut updated),
                0
            );
            assert_eq!(updated, u32::from(scenario != 3));
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
            // Force the final component to overflow after earlier staged component updates.
            assert_eq!(
                liblrhsmm_rs_gaussian_statistics_set_occupancy_f64(statistics, 4.0),
                0
            );
            let occupancy = vec![2.0; components];
            assert_eq!(
                liblrhsmm_rs_gaussian_statistics_set_field_f64(
                    statistics,
                    2,
                    0,
                    occupancy.as_ptr(),
                    components
                ),
                0
            );
            let overflow = [f64::MAX];
            assert_eq!(
                liblrhsmm_rs_gaussian_statistics_set_field_f64(
                    statistics,
                    0,
                    components * 2 - 1,
                    overflow.as_ptr(),
                    1
                ),
                0
            );
            let mut before = Vec::new();
            for (selector, count) in [
                (0, components),
                (1, components * 2),
                (2, components * 2),
                (3, components * 2),
            ] {
                let mut array = vec![0.0; count];
                assert_eq!(
                    liblrhsmm_rs_gaussian_copy_field_f64(
                        source,
                        selector,
                        0,
                        array.as_mut_ptr(),
                        count
                    ),
                    0
                );
                before.push(array);
            }
            updated = 99;
            assert_eq!(
                liblrhsmm_rs_gaussian_update_f64(source, statistics, &options, &mut updated),
                3
            );
            assert_eq!(updated, 99);
            assert_eq!(
                liblrhsmm_rs_gaussian_update_f64(source, statistics, &options, null_mut()),
                1
            );
            options.duration_mode = 99;
            assert_eq!(
                liblrhsmm_rs_gaussian_update_f64(source, statistics, &options, &mut updated),
                3
            );
            assert_eq!(updated, 99);
            for (selector, array) in before.into_iter().enumerate() {
                let mut after = vec![0.0; array.len()];
                assert_eq!(
                    liblrhsmm_rs_gaussian_copy_field_f64(
                        source,
                        selector as u32,
                        0,
                        after.as_mut_ptr(),
                        after.len()
                    ),
                    0
                );
                assert_eq!(after, array);
            }
            assert_eq!(
                liblrhsmm_rs_gaussian_statistics_release_f64(&mut statistics),
                0
            );
            assert_eq!(liblrhsmm_rs_gaussian_release_f64(&mut source), 0);
        }
        assert_eq!(compared, 35);
    }
}
