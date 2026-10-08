#![cfg(feature = "c-api")]
use liblrhsmm_rs::c_api::*;
use std::ptr::{null, null_mut};

#[test]
fn model_f32_statistics_ownership_and_late_update_failure() {
    // SAFETY: Independent initialized buffers and live, uniquely released owners.
    unsafe {
        let specification = LiblrhsmmRsStreamSpecification {
            emissions: 1,
            components: 1,
            dimensions: 1,
        };
        let mut model = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_new_f32(&specification, 1, 2, &mut model),
            0
        );
        let snapshot = |model| {
            let mut result = Vec::new();
            let mut stream = null_mut();
            let mut gaussian = null_mut();
            assert_eq!(liblrhsmm_rs_model_get_stream_f32(model, 0, &mut stream), 0);
            let mut layout = LiblrhsmmRsStreamLayoutF32 {
                weight: 0.0,
                emissions: 0,
            };
            assert_eq!(liblrhsmm_rs_stream_layout_f32(stream, &mut layout), 0);
            result.push(layout.weight.to_bits() as u64);
            result.push(layout.emissions as u64);
            assert_eq!(
                liblrhsmm_rs_stream_get_emission_f32(stream, 0, &mut gaussian),
                0
            );
            for field in 0..4 {
                let mut value: f32 = 0.0;
                assert_eq!(
                    liblrhsmm_rs_gaussian_copy_field_f32(gaussian, field, 0, &mut value, 1),
                    0
                );
                result.push(value.to_bits() as u64);
            }
            assert_eq!(liblrhsmm_rs_gaussian_release_f32(&mut gaussian), 0);
            assert_eq!(liblrhsmm_rs_stream_release_f32(&mut stream), 0);
            for index in 0..2 {
                let mut duration = null_mut();
                assert_eq!(
                    liblrhsmm_rs_model_get_duration_f32(model, index, &mut duration),
                    0
                );
                let mut parameters = LiblrhsmmRsDurationParametersF32 {
                    mean: 0.0,
                    variance: 0.0,
                    minimum: 0,
                    maximum: 0,
                    fixed_mean: 0,
                    variance_floor: 0.0,
                };
                assert_eq!(liblrhsmm_rs_duration_get_f32(duration, &mut parameters), 0);
                result.extend([
                    parameters.mean.to_bits() as u64,
                    parameters.variance.to_bits() as u64,
                    parameters.minimum as u64,
                    parameters.maximum as u64,
                    parameters.fixed_mean as u64,
                    parameters.variance_floor.to_bits() as u64,
                ]);
                assert_eq!(liblrhsmm_rs_duration_release_f32(&mut duration), 0);
            }
            result
        };
        let before = snapshot(model);
        let mut statistics = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_statistics_from_model_f32(model, &mut statistics),
            0
        );
        let mut layout = LiblrhsmmRsModelLayout {
            streams: 99,
            durations: 99,
        };
        assert_eq!(
            liblrhsmm_rs_model_statistics_layout_f32(statistics, &mut layout),
            0
        );
        assert_eq!(
            layout,
            LiblrhsmmRsModelLayout {
                streams: 1,
                durations: 2
            }
        );
        let mut stream = null_mut();
        let mut gaussian = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_statistics_get_stream_f32(statistics, 0, &mut stream),
            0
        );
        assert_eq!(
            liblrhsmm_rs_stream_statistics_get_emission_f32(stream, 0, &mut gaussian),
            0
        );
        let sums = [4.0];
        let squares = [10.0];
        let occupancies = [2.0];
        let values = LiblrhsmmRsGaussianStatisticsValuesF32 {
            mixtures: 1,
            dimensions: 1,
            state_occupancy: 2.0,
            weighted_sums: sums.as_ptr(),
            weighted_squared_sums: squares.as_ptr(),
            component_occupancies: occupancies.as_ptr(),
        };
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_replace_f32(gaussian, &values),
            0
        );
        assert_eq!(
            liblrhsmm_rs_stream_statistics_set_emission_f32(stream, 0, gaussian),
            0
        );
        assert_eq!(
            liblrhsmm_rs_model_statistics_set_stream_f32(statistics, 0, stream),
            0
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_release_f32(&mut gaussian),
            0
        );
        assert_eq!(liblrhsmm_rs_stream_statistics_release_f32(&mut stream), 0);
        let good = LiblrhsmmRsDurationStatisticsValuesF32 {
            value_sum: 6.0,
            squared_sum: 20.0,
            occupancy: 2.0,
        };
        let bad = LiblrhsmmRsDurationStatisticsValuesF32 {
            value_sum: f32::MAX,
            squared_sum: 20.0,
            occupancy: 2.0,
        };
        let mut duration = null_mut();
        assert_eq!(
            liblrhsmm_rs_duration_statistics_create_f32(&good, &mut duration),
            0
        );
        assert_eq!(
            liblrhsmm_rs_model_statistics_set_duration_f32(statistics, 0, duration),
            0
        );
        assert_eq!(liblrhsmm_rs_duration_statistics_set_f32(duration, &bad), 0);
        assert_eq!(
            liblrhsmm_rs_model_statistics_set_duration_f32(statistics, 1, duration),
            0
        );
        assert_eq!(
            liblrhsmm_rs_duration_statistics_release_f32(&mut duration),
            0
        );
        let mut options = LiblrhsmmRsUpdateOptionsF32 {
            duration_mode: 0,
            duration_variance_floor: 0.05,
            emission_variance_floor: 0.01,
        };
        // The final duration overflows only after the stream and first duration have been staged.
        assert_eq!(
            liblrhsmm_rs_model_update_f32(model, statistics, &options),
            3
        );
        assert_eq!(snapshot(model), before);
        assert_eq!(
            liblrhsmm_rs_model_statistics_get_duration_f32(statistics, 1, &mut duration),
            0
        );
        let mut observed = good;
        assert_eq!(
            liblrhsmm_rs_duration_statistics_get_f32(duration, &mut observed),
            0
        );
        assert_eq!(observed, bad);
        assert_eq!(liblrhsmm_rs_duration_statistics_set_f32(duration, &good), 0);
        assert_eq!(
            liblrhsmm_rs_model_statistics_set_duration_f32(statistics, 1, duration),
            0
        );
        assert_eq!(
            liblrhsmm_rs_duration_statistics_release_f32(&mut duration),
            0
        );
        let mut copy = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_statistics_clone_f32(statistics, &mut copy),
            0
        );
        assert_eq!(liblrhsmm_rs_model_statistics_clear_f32(statistics), 0);
        assert_eq!(liblrhsmm_rs_model_update_f32(model, copy, &options), 0);
        let after = snapshot(model);
        assert_eq!(after[3], (2.0_f32).to_bits() as u64);
        assert_eq!(after[6], (3.0_f32).to_bits() as u64);
        assert_eq!(after[12], (3.0_f32).to_bits() as u64);
        assert_ne!(after, before);
        assert_eq!(
            liblrhsmm_rs_model_update_f32(model, statistics, &options),
            0
        );
        assert_eq!(snapshot(model), after);
        options.duration_mode = 99;
        assert_eq!(liblrhsmm_rs_model_update_f32(model, copy, &options), 3);
        assert_eq!(liblrhsmm_rs_model_update_f32(model, copy, null()), 1);
        assert_eq!(snapshot(model), after);
        // Constructor copies repeated children; source mutation and release cannot alter copies.
        assert_eq!(
            liblrhsmm_rs_model_statistics_get_stream_f32(copy, 0, &mut stream),
            0
        );
        assert_eq!(
            liblrhsmm_rs_model_statistics_get_duration_f32(copy, 0, &mut duration),
            0
        );
        let unusual = LiblrhsmmRsDurationStatisticsValuesF32 {
            value_sum: f32::from_bits(0x7fc01234),
            squared_sum: -0.0,
            occupancy: f32::INFINITY,
        };
        assert_eq!(
            liblrhsmm_rs_duration_statistics_set_f32(duration, &unusual),
            0
        );
        let streams = [stream.cast_const(), stream.cast_const()];
        let durations = [duration.cast_const(), duration.cast_const()];
        let mut assembled = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_statistics_create_f32(
                streams.as_ptr(),
                2,
                durations.as_ptr(),
                2,
                &mut assembled
            ),
            0
        );
        assert_eq!(liblrhsmm_rs_stream_statistics_clear_f32(stream), 0);
        assert_eq!(liblrhsmm_rs_duration_statistics_clear_f32(duration), 0);
        assert_eq!(liblrhsmm_rs_stream_statistics_release_f32(&mut stream), 0);
        assert_eq!(
            liblrhsmm_rs_duration_statistics_release_f32(&mut duration),
            0
        );
        for index in 0..2 {
            assert_eq!(
                liblrhsmm_rs_model_statistics_get_duration_f32(assembled, index, &mut duration),
                0
            );
            assert_eq!(
                liblrhsmm_rs_duration_statistics_get_f32(duration, &mut observed),
                0
            );
            assert_eq!(observed.value_sum.to_bits(), unusual.value_sum.to_bits());
            assert_eq!(
                observed.squared_sum.to_bits(),
                unusual.squared_sum.to_bits()
            );
            assert_eq!(observed.occupancy.to_bits(), unusual.occupancy.to_bits());
            assert_eq!(
                liblrhsmm_rs_duration_statistics_release_f32(&mut duration),
                0
            );
            assert_eq!(
                liblrhsmm_rs_model_statistics_get_stream_f32(assembled, index, &mut stream),
                0
            );
            assert_eq!(
                liblrhsmm_rs_stream_statistics_get_emission_f32(stream, 0, &mut gaussian),
                0
            );
            let mut sum: f32 = 0.0;
            assert_eq!(
                liblrhsmm_rs_gaussian_statistics_copy_field_f32(gaussian, 0, 0, &mut sum, 1),
                0
            );
            assert_eq!(sum, 4.0);
            assert_eq!(
                liblrhsmm_rs_gaussian_statistics_release_f32(&mut gaussian),
                0
            );
            assert_eq!(liblrhsmm_rs_stream_statistics_release_f32(&mut stream), 0);
        }
        let invalid = [null()];
        assert_eq!(
            liblrhsmm_rs_model_statistics_replace_f32(assembled, null(), 0, invalid.as_ptr(), 1),
            1
        );
        assert_eq!(
            liblrhsmm_rs_model_statistics_layout_f32(assembled, &mut layout),
            0
        );
        assert_eq!(
            layout,
            LiblrhsmmRsModelLayout {
                streams: 2,
                durations: 2
            }
        );
        let mut sentinel = copy;
        assert_eq!(
            liblrhsmm_rs_model_statistics_get_duration_f32(assembled, 2, &mut duration),
            2
        );
        assert!(duration.is_null());
        assert_eq!(
            liblrhsmm_rs_model_statistics_clone_f32(null(), &mut sentinel),
            1
        );
        assert_eq!(sentinel, copy);
        assert_eq!(
            liblrhsmm_rs_model_statistics_set_duration_f32(assembled, 2, null()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_model_statistics_set_stream_f32(assembled, 2, null()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_model_statistics_replace_f32(assembled, null(), 0, null(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_model_statistics_layout_f32(assembled, &mut layout),
            0
        );
        assert_eq!(
            layout,
            LiblrhsmmRsModelLayout {
                streams: 0,
                durations: 0
            }
        );
        assert_eq!(liblrhsmm_rs_model_update_f32(model, assembled, &options), 3);
        assert_eq!(snapshot(model), after);
        assert_eq!(liblrhsmm_rs_model_release_f32(&mut model), 0);
        for slot in [&mut statistics, &mut copy, &mut assembled] {
            assert_eq!(liblrhsmm_rs_model_statistics_release_f32(slot), 0);
            assert!(slot.is_null());
            assert_eq!(liblrhsmm_rs_model_statistics_release_f32(slot), 0);
        }
    }
}

#[test]
fn model_f64_statistics_ownership_and_late_update_failure() {
    // SAFETY: Independent initialized buffers and live, uniquely released owners.
    unsafe {
        let specification = LiblrhsmmRsStreamSpecification {
            emissions: 1,
            components: 1,
            dimensions: 1,
        };
        let mut model = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_new_f64(&specification, 1, 2, &mut model),
            0
        );
        let snapshot = |model| {
            let mut result = Vec::new();
            let mut stream = null_mut();
            let mut gaussian = null_mut();
            assert_eq!(liblrhsmm_rs_model_get_stream_f64(model, 0, &mut stream), 0);
            let mut layout = LiblrhsmmRsStreamLayoutF64 {
                weight: 0.0,
                emissions: 0,
            };
            assert_eq!(liblrhsmm_rs_stream_layout_f64(stream, &mut layout), 0);
            result.push(layout.weight.to_bits());
            result.push(layout.emissions as u64);
            assert_eq!(
                liblrhsmm_rs_stream_get_emission_f64(stream, 0, &mut gaussian),
                0
            );
            for field in 0..4 {
                let mut value: f64 = 0.0;
                assert_eq!(
                    liblrhsmm_rs_gaussian_copy_field_f64(gaussian, field, 0, &mut value, 1),
                    0
                );
                result.push(value.to_bits());
            }
            assert_eq!(liblrhsmm_rs_gaussian_release_f64(&mut gaussian), 0);
            assert_eq!(liblrhsmm_rs_stream_release_f64(&mut stream), 0);
            for index in 0..2 {
                let mut duration = null_mut();
                assert_eq!(
                    liblrhsmm_rs_model_get_duration_f64(model, index, &mut duration),
                    0
                );
                let mut parameters = LiblrhsmmRsDurationParametersF64 {
                    mean: 0.0,
                    variance: 0.0,
                    minimum: 0,
                    maximum: 0,
                    fixed_mean: 0,
                    variance_floor: 0.0,
                };
                assert_eq!(liblrhsmm_rs_duration_get_f64(duration, &mut parameters), 0);
                result.extend([
                    parameters.mean.to_bits(),
                    parameters.variance.to_bits(),
                    parameters.minimum as u64,
                    parameters.maximum as u64,
                    parameters.fixed_mean as u64,
                    parameters.variance_floor.to_bits(),
                ]);
                assert_eq!(liblrhsmm_rs_duration_release_f64(&mut duration), 0);
            }
            result
        };
        let before = snapshot(model);
        let mut statistics = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_statistics_from_model_f64(model, &mut statistics),
            0
        );
        let mut layout = LiblrhsmmRsModelLayout {
            streams: 99,
            durations: 99,
        };
        assert_eq!(
            liblrhsmm_rs_model_statistics_layout_f64(statistics, &mut layout),
            0
        );
        assert_eq!(
            layout,
            LiblrhsmmRsModelLayout {
                streams: 1,
                durations: 2
            }
        );
        let mut stream = null_mut();
        let mut gaussian = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_statistics_get_stream_f64(statistics, 0, &mut stream),
            0
        );
        assert_eq!(
            liblrhsmm_rs_stream_statistics_get_emission_f64(stream, 0, &mut gaussian),
            0
        );
        let sums = [4.0];
        let squares = [10.0];
        let occupancies = [2.0];
        let values = LiblrhsmmRsGaussianStatisticsValuesF64 {
            mixtures: 1,
            dimensions: 1,
            state_occupancy: 2.0,
            weighted_sums: sums.as_ptr(),
            weighted_squared_sums: squares.as_ptr(),
            component_occupancies: occupancies.as_ptr(),
        };
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_replace_f64(gaussian, &values),
            0
        );
        assert_eq!(
            liblrhsmm_rs_stream_statistics_set_emission_f64(stream, 0, gaussian),
            0
        );
        assert_eq!(
            liblrhsmm_rs_model_statistics_set_stream_f64(statistics, 0, stream),
            0
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_statistics_release_f64(&mut gaussian),
            0
        );
        assert_eq!(liblrhsmm_rs_stream_statistics_release_f64(&mut stream), 0);
        let good = LiblrhsmmRsDurationStatisticsValuesF64 {
            value_sum: 6.0,
            squared_sum: 20.0,
            occupancy: 2.0,
        };
        let bad = LiblrhsmmRsDurationStatisticsValuesF64 {
            value_sum: f64::MAX,
            squared_sum: 20.0,
            occupancy: 2.0,
        };
        let mut duration = null_mut();
        assert_eq!(
            liblrhsmm_rs_duration_statistics_create_f64(&good, &mut duration),
            0
        );
        assert_eq!(
            liblrhsmm_rs_model_statistics_set_duration_f64(statistics, 0, duration),
            0
        );
        assert_eq!(liblrhsmm_rs_duration_statistics_set_f64(duration, &bad), 0);
        assert_eq!(
            liblrhsmm_rs_model_statistics_set_duration_f64(statistics, 1, duration),
            0
        );
        assert_eq!(
            liblrhsmm_rs_duration_statistics_release_f64(&mut duration),
            0
        );
        let mut options = LiblrhsmmRsUpdateOptionsF64 {
            duration_mode: 0,
            duration_variance_floor: 0.05,
            emission_variance_floor: 0.01,
        };
        // The final duration overflows only after the stream and first duration have been staged.
        assert_eq!(
            liblrhsmm_rs_model_update_f64(model, statistics, &options),
            3
        );
        assert_eq!(snapshot(model), before);
        assert_eq!(
            liblrhsmm_rs_model_statistics_get_duration_f64(statistics, 1, &mut duration),
            0
        );
        let mut observed = good;
        assert_eq!(
            liblrhsmm_rs_duration_statistics_get_f64(duration, &mut observed),
            0
        );
        assert_eq!(observed, bad);
        assert_eq!(liblrhsmm_rs_duration_statistics_set_f64(duration, &good), 0);
        assert_eq!(
            liblrhsmm_rs_model_statistics_set_duration_f64(statistics, 1, duration),
            0
        );
        assert_eq!(
            liblrhsmm_rs_duration_statistics_release_f64(&mut duration),
            0
        );
        let mut copy = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_statistics_clone_f64(statistics, &mut copy),
            0
        );
        assert_eq!(liblrhsmm_rs_model_statistics_clear_f64(statistics), 0);
        assert_eq!(liblrhsmm_rs_model_update_f64(model, copy, &options), 0);
        let after = snapshot(model);
        assert_eq!(after[3], (2.0_f64).to_bits());
        assert_eq!(after[6], (3.0_f64).to_bits());
        assert_eq!(after[12], (3.0_f64).to_bits());
        assert_ne!(after, before);
        assert_eq!(
            liblrhsmm_rs_model_update_f64(model, statistics, &options),
            0
        );
        assert_eq!(snapshot(model), after);
        options.duration_mode = 99;
        assert_eq!(liblrhsmm_rs_model_update_f64(model, copy, &options), 3);
        assert_eq!(liblrhsmm_rs_model_update_f64(model, copy, null()), 1);
        assert_eq!(snapshot(model), after);
        // Constructor copies repeated children; source mutation and release cannot alter copies.
        assert_eq!(
            liblrhsmm_rs_model_statistics_get_stream_f64(copy, 0, &mut stream),
            0
        );
        assert_eq!(
            liblrhsmm_rs_model_statistics_get_duration_f64(copy, 0, &mut duration),
            0
        );
        let unusual = LiblrhsmmRsDurationStatisticsValuesF64 {
            value_sum: f64::from_bits(0x7ff8000000001234),
            squared_sum: -0.0,
            occupancy: f64::INFINITY,
        };
        assert_eq!(
            liblrhsmm_rs_duration_statistics_set_f64(duration, &unusual),
            0
        );
        let streams = [stream.cast_const(), stream.cast_const()];
        let durations = [duration.cast_const(), duration.cast_const()];
        let mut assembled = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_statistics_create_f64(
                streams.as_ptr(),
                2,
                durations.as_ptr(),
                2,
                &mut assembled
            ),
            0
        );
        assert_eq!(liblrhsmm_rs_stream_statistics_clear_f64(stream), 0);
        assert_eq!(liblrhsmm_rs_duration_statistics_clear_f64(duration), 0);
        assert_eq!(liblrhsmm_rs_stream_statistics_release_f64(&mut stream), 0);
        assert_eq!(
            liblrhsmm_rs_duration_statistics_release_f64(&mut duration),
            0
        );
        for index in 0..2 {
            assert_eq!(
                liblrhsmm_rs_model_statistics_get_duration_f64(assembled, index, &mut duration),
                0
            );
            assert_eq!(
                liblrhsmm_rs_duration_statistics_get_f64(duration, &mut observed),
                0
            );
            assert_eq!(observed.value_sum.to_bits(), unusual.value_sum.to_bits());
            assert_eq!(
                observed.squared_sum.to_bits(),
                unusual.squared_sum.to_bits()
            );
            assert_eq!(observed.occupancy.to_bits(), unusual.occupancy.to_bits());
            assert_eq!(
                liblrhsmm_rs_duration_statistics_release_f64(&mut duration),
                0
            );
            assert_eq!(
                liblrhsmm_rs_model_statistics_get_stream_f64(assembled, index, &mut stream),
                0
            );
            assert_eq!(
                liblrhsmm_rs_stream_statistics_get_emission_f64(stream, 0, &mut gaussian),
                0
            );
            let mut sum: f64 = 0.0;
            assert_eq!(
                liblrhsmm_rs_gaussian_statistics_copy_field_f64(gaussian, 0, 0, &mut sum, 1),
                0
            );
            assert_eq!(sum, 4.0);
            assert_eq!(
                liblrhsmm_rs_gaussian_statistics_release_f64(&mut gaussian),
                0
            );
            assert_eq!(liblrhsmm_rs_stream_statistics_release_f64(&mut stream), 0);
        }
        let invalid = [null()];
        assert_eq!(
            liblrhsmm_rs_model_statistics_replace_f64(assembled, null(), 0, invalid.as_ptr(), 1),
            1
        );
        assert_eq!(
            liblrhsmm_rs_model_statistics_layout_f64(assembled, &mut layout),
            0
        );
        assert_eq!(
            layout,
            LiblrhsmmRsModelLayout {
                streams: 2,
                durations: 2
            }
        );
        let mut sentinel = copy;
        assert_eq!(
            liblrhsmm_rs_model_statistics_get_duration_f64(assembled, 2, &mut duration),
            2
        );
        assert!(duration.is_null());
        assert_eq!(
            liblrhsmm_rs_model_statistics_clone_f64(null(), &mut sentinel),
            1
        );
        assert_eq!(sentinel, copy);
        assert_eq!(
            liblrhsmm_rs_model_statistics_set_duration_f64(assembled, 2, null()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_model_statistics_set_stream_f64(assembled, 2, null()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_model_statistics_replace_f64(assembled, null(), 0, null(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_model_statistics_layout_f64(assembled, &mut layout),
            0
        );
        assert_eq!(
            layout,
            LiblrhsmmRsModelLayout {
                streams: 0,
                durations: 0
            }
        );
        assert_eq!(liblrhsmm_rs_model_update_f64(model, assembled, &options), 3);
        assert_eq!(snapshot(model), after);
        assert_eq!(liblrhsmm_rs_model_release_f64(&mut model), 0);
        for slot in [&mut statistics, &mut copy, &mut assembled] {
            assert_eq!(liblrhsmm_rs_model_statistics_release_f64(slot), 0);
            assert!(slot.is_null());
            assert_eq!(liblrhsmm_rs_model_statistics_release_f64(slot), 0);
        }
    }
}
#[test]
fn model_f32_updates_original_c_values() {
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
            let mut model = null_mut();
            let mut model_statistics = null_mut();
            let streams = [stream.cast_const()];
            let accumulators = [stream_statistics.cast_const()];
            assert_eq!(
                liblrhsmm_rs_model_create_f32(streams.as_ptr(), 1, null(), 0, &mut model),
                0
            );
            assert_eq!(
                liblrhsmm_rs_model_statistics_create_f32(
                    accumulators.as_ptr(),
                    1,
                    null(),
                    0,
                    &mut model_statistics
                ),
                0
            );
            assert_eq!(
                liblrhsmm_rs_model_update_f32(model, model_statistics, &options),
                0
            );
            assert_eq!(liblrhsmm_rs_stream_release_f32(&mut stream), 0);
            assert_eq!(liblrhsmm_rs_model_get_stream_f32(model, 0, &mut stream), 0);
            assert_eq!(liblrhsmm_rs_model_release_f32(&mut model), 0);
            assert_eq!(
                liblrhsmm_rs_model_statistics_release_f32(&mut model_statistics),
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

        let mut duration_compared = 0;
        for line in include_str!("fixtures/c-updates.txt")
            .lines()
            .filter(|line| line.starts_with("duration "))
        {
            let fields: Vec<_> = line.split_whitespace().collect();
            let scenario: usize = fields[1].parse().unwrap();
            let parameters = LiblrhsmmRsDurationParametersF32 {
                mean: 7.0,
                variance: 3.0,
                minimum: 2,
                maximum: 6,
                fixed_mean: -1,
                variance_floor: if scenario == 2 { 2.5 } else { 0.0 },
            };
            let values = LiblrhsmmRsDurationStatisticsValuesF32 {
                value_sum: if scenario == 3 { 1.5 } else { 9.0 },
                squared_sum: 29.0,
                occupancy: if scenario == 4 { 1e-8 } else { 3.0 },
            };
            let options = LiblrhsmmRsUpdateOptionsF32 {
                duration_mode: u32::from(scenario == 3),
                duration_variance_floor: 0.05,
                emission_variance_floor: 0.01,
            };
            let mut duration = null_mut();
            let mut statistics = null_mut();
            let mut model = null_mut();
            let mut model_statistics = null_mut();
            assert_eq!(
                liblrhsmm_rs_duration_create_f32(&parameters, &mut duration),
                0
            );
            assert_eq!(
                liblrhsmm_rs_duration_statistics_create_f32(&values, &mut statistics),
                0
            );
            let durations = [duration.cast_const()];
            let accumulators = [statistics.cast_const()];
            assert_eq!(
                liblrhsmm_rs_model_create_f32(null(), 0, durations.as_ptr(), 1, &mut model),
                0
            );
            assert_eq!(
                liblrhsmm_rs_model_statistics_create_f32(
                    null(),
                    0,
                    accumulators.as_ptr(),
                    1,
                    &mut model_statistics
                ),
                0
            );
            assert_eq!(liblrhsmm_rs_duration_release_f32(&mut duration), 0);
            assert_eq!(
                liblrhsmm_rs_duration_statistics_release_f32(&mut statistics),
                0
            );
            assert_eq!(
                liblrhsmm_rs_model_update_f32(model, model_statistics, &options),
                0
            );
            assert_eq!(
                liblrhsmm_rs_model_get_duration_f32(model, 0, &mut duration),
                0
            );
            let mut actual = parameters;
            assert_eq!(liblrhsmm_rs_duration_get_f32(duration, &mut actual), 0);
            assert_eq!(
                (actual.minimum, actual.maximum, actual.fixed_mean),
                (2, 6, -1)
            );
            assert_eq!(
                actual.variance_floor.to_bits(),
                parameters.variance_floor.to_bits()
            );
            for (actual, expected) in [actual.mean, actual.variance].into_iter().zip(&fields[2..]) {
                let expected: f32 = expected.parse().unwrap();
                assert_eq!(actual.to_bits(), expected.to_bits(), "{line}");
                duration_compared += 1;
            }
            assert_eq!(liblrhsmm_rs_duration_release_f32(&mut duration), 0);
            assert_eq!(liblrhsmm_rs_model_release_f32(&mut model), 0);
            assert_eq!(
                liblrhsmm_rs_model_statistics_release_f32(&mut model_statistics),
                0
            );
        }
        assert_eq!(duration_compared, 10);
    }
}

#[test]
fn model_f64_updates_original_c_values() {
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
            let mut model = null_mut();
            let mut model_statistics = null_mut();
            let streams = [stream.cast_const()];
            let accumulators = [stream_statistics.cast_const()];
            assert_eq!(
                liblrhsmm_rs_model_create_f64(streams.as_ptr(), 1, null(), 0, &mut model),
                0
            );
            assert_eq!(
                liblrhsmm_rs_model_statistics_create_f64(
                    accumulators.as_ptr(),
                    1,
                    null(),
                    0,
                    &mut model_statistics
                ),
                0
            );
            assert_eq!(
                liblrhsmm_rs_model_update_f64(model, model_statistics, &options),
                0
            );
            assert_eq!(liblrhsmm_rs_stream_release_f64(&mut stream), 0);
            assert_eq!(liblrhsmm_rs_model_get_stream_f64(model, 0, &mut stream), 0);
            assert_eq!(liblrhsmm_rs_model_release_f64(&mut model), 0);
            assert_eq!(
                liblrhsmm_rs_model_statistics_release_f64(&mut model_statistics),
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

        let mut duration_compared = 0;
        for line in include_str!("fixtures/c-double-update.txt")
            .lines()
            .filter(|line| line.starts_with("duration "))
        {
            let fields: Vec<_> = line.split_whitespace().collect();
            let scenario: usize = fields[1].parse().unwrap();
            let parameters = LiblrhsmmRsDurationParametersF64 {
                mean: 7.0,
                variance: 3.0,
                minimum: 2,
                maximum: 6,
                fixed_mean: -1,
                variance_floor: if scenario == 2 { 2.5 } else { 0.0 },
            };
            let values = LiblrhsmmRsDurationStatisticsValuesF64 {
                value_sum: if scenario == 3 { 1.5 } else { 9.0 },
                squared_sum: 29.0,
                occupancy: if scenario == 4 { 1e-8 } else { 3.0 },
            };
            let options = LiblrhsmmRsUpdateOptionsF64 {
                duration_mode: u32::from(scenario == 3),
                duration_variance_floor: 0.05,
                emission_variance_floor: 0.01,
            };
            let mut duration = null_mut();
            let mut statistics = null_mut();
            let mut model = null_mut();
            let mut model_statistics = null_mut();
            assert_eq!(
                liblrhsmm_rs_duration_create_f64(&parameters, &mut duration),
                0
            );
            assert_eq!(
                liblrhsmm_rs_duration_statistics_create_f64(&values, &mut statistics),
                0
            );
            let durations = [duration.cast_const()];
            let accumulators = [statistics.cast_const()];
            assert_eq!(
                liblrhsmm_rs_model_create_f64(null(), 0, durations.as_ptr(), 1, &mut model),
                0
            );
            assert_eq!(
                liblrhsmm_rs_model_statistics_create_f64(
                    null(),
                    0,
                    accumulators.as_ptr(),
                    1,
                    &mut model_statistics
                ),
                0
            );
            assert_eq!(liblrhsmm_rs_duration_release_f64(&mut duration), 0);
            assert_eq!(
                liblrhsmm_rs_duration_statistics_release_f64(&mut statistics),
                0
            );
            assert_eq!(
                liblrhsmm_rs_model_update_f64(model, model_statistics, &options),
                0
            );
            assert_eq!(
                liblrhsmm_rs_model_get_duration_f64(model, 0, &mut duration),
                0
            );
            let mut actual = parameters;
            assert_eq!(liblrhsmm_rs_duration_get_f64(duration, &mut actual), 0);
            assert_eq!(
                (actual.minimum, actual.maximum, actual.fixed_mean),
                (2, 6, -1)
            );
            assert_eq!(
                actual.variance_floor.to_bits(),
                parameters.variance_floor.to_bits()
            );
            for (actual, expected) in [actual.mean, actual.variance].into_iter().zip(&fields[2..]) {
                let expected: f64 = expected.parse().unwrap();
                assert!(
                    (actual - expected).abs() <= 2e-14 * expected.abs().max(1.0),
                    "{line}"
                );
                duration_compared += 1;
            }
            assert_eq!(liblrhsmm_rs_duration_release_f64(&mut duration), 0);
            assert_eq!(liblrhsmm_rs_model_release_f64(&mut model), 0);
            assert_eq!(
                liblrhsmm_rs_model_statistics_release_f64(&mut model_statistics),
                0
            );
        }
        assert_eq!(duration_compared, 10);
    }
}
