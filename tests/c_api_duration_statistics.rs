#![cfg(feature = "c-api")]
use liblrhsmm_rs::c_api::*;
use liblrhsmm_rs::model::DurationParameters;
use liblrhsmm_rs::statistics::DurationSufficientStatistics;
use liblrhsmm_rs::update::{DurationMode, UpdateSettings};
use std::ptr::{null, null_mut};

#[test]
fn statistics_f32_all_fields_bits_ownership_and_clear() {
    // SAFETY: Independent initialized descriptors and exclusive, uniquely released owners.
    unsafe {
        let mut values = LiblrhsmmRsDurationStatisticsValuesF32 {
            value_sum: 99.0,
            squared_sum: 99.0,
            occupancy: 99.0,
        };
        assert_eq!(liblrhsmm_rs_duration_statistics_default_f32(&mut values), 0);
        assert_eq!(
            values,
            LiblrhsmmRsDurationStatisticsValuesF32 {
                value_sum: 0.0,
                squared_sum: 0.0,
                occupancy: 0.0
            }
        );
        let mut parameters = LiblrhsmmRsDurationParametersF32 {
            mean: 0.0,
            variance: 0.0,
            minimum: 0,
            maximum: 0,
            fixed_mean: 0,
            variance_floor: 0.0,
        };
        assert_eq!(liblrhsmm_rs_duration_default_f32(&mut parameters), 0);
        let mut duration = null_mut();
        assert_eq!(
            liblrhsmm_rs_duration_create_f32(&parameters, &mut duration),
            0
        );
        let mut statistics = null_mut();
        assert_eq!(
            liblrhsmm_rs_duration_statistics_from_duration_f32(duration, &mut statistics),
            0
        );
        assert_eq!(liblrhsmm_rs_duration_release_f32(&mut duration), 0);
        values = LiblrhsmmRsDurationStatisticsValuesF32 {
            value_sum: 6.0,
            squared_sum: 20.0,
            occupancy: 2.0,
        };
        assert_eq!(
            liblrhsmm_rs_duration_statistics_set_f32(statistics, &values),
            0
        );
        let mut copy = null_mut();
        assert_eq!(
            liblrhsmm_rs_duration_statistics_clone_f32(statistics, &mut copy),
            0
        );
        assert_eq!(liblrhsmm_rs_duration_statistics_clear_f32(statistics), 0);
        let mut fetched = values;
        assert_eq!(
            liblrhsmm_rs_duration_statistics_get_f32(statistics, &mut fetched),
            0
        );
        assert_eq!(
            fetched,
            LiblrhsmmRsDurationStatisticsValuesF32 {
                value_sum: 0.0,
                squared_sum: 0.0,
                occupancy: 0.0
            }
        );
        assert_eq!(
            liblrhsmm_rs_duration_statistics_get_f32(copy, &mut fetched),
            0
        );
        assert_eq!(fetched, values);
        values.value_sum = f32::from_bits(0x7fc01234);
        values.squared_sum = -1.0;
        values.occupancy = f32::NEG_INFINITY;
        assert_eq!(
            liblrhsmm_rs_duration_statistics_set_f32(statistics, &values),
            0
        );
        assert_eq!(
            liblrhsmm_rs_duration_statistics_get_f32(statistics, &mut fetched),
            0
        );
        assert_eq!(fetched.value_sum.to_bits(), values.value_sum.to_bits());
        assert_eq!(fetched.squared_sum.to_bits(), values.squared_sum.to_bits());
        assert_eq!(fetched.occupancy.to_bits(), values.occupancy.to_bits());
        let mut direct = null_mut();
        assert_eq!(
            liblrhsmm_rs_duration_statistics_create_f32(&values, &mut direct),
            0
        );
        assert_eq!(
            liblrhsmm_rs_duration_statistics_get_f32(direct, &mut fetched),
            0
        );
        assert_eq!(fetched.value_sum.to_bits(), values.value_sum.to_bits());
        assert_eq!(
            liblrhsmm_rs_duration_statistics_create_f32(null(), &mut direct),
            1
        );
        assert_eq!(
            liblrhsmm_rs_duration_statistics_create_f32(&values, null_mut()),
            1
        );
        let misaligned = (&values as *const LiblrhsmmRsDurationStatisticsValuesF32)
            .cast::<u8>()
            .wrapping_add(1)
            .cast();
        assert_eq!(
            liblrhsmm_rs_duration_statistics_create_f32(misaligned, &mut direct),
            1
        );
        assert_eq!(
            liblrhsmm_rs_duration_statistics_get_f32(copy, null_mut()),
            1
        );
        assert_eq!(liblrhsmm_rs_duration_statistics_set_f32(copy, null()), 1);
        assert_eq!(
            liblrhsmm_rs_duration_statistics_clone_f32(copy, null_mut()),
            1
        );
        assert_eq!(liblrhsmm_rs_duration_statistics_default_f32(null_mut()), 1);
        assert_eq!(
            liblrhsmm_rs_duration_statistics_from_duration_f32(null(), &mut statistics),
            1
        );
        assert_eq!(liblrhsmm_rs_duration_statistics_clear_f32(null_mut()), 1);
        assert_eq!(liblrhsmm_rs_duration_statistics_release_f32(null_mut()), 1);
        assert_eq!(
            liblrhsmm_rs_duration_statistics_release_f32(&mut statistics),
            0
        );
        assert!(statistics.is_null());
        assert_eq!(
            liblrhsmm_rs_duration_statistics_release_f32(&mut statistics),
            0
        );
        assert_eq!(
            liblrhsmm_rs_duration_statistics_get_f32(copy, &mut fetched),
            0
        );
        assert_eq!(fetched.value_sum, 6.0);
        assert_eq!(liblrhsmm_rs_duration_statistics_release_f32(&mut copy), 0);
        assert_eq!(liblrhsmm_rs_duration_statistics_clear_f32(direct), 0);
        assert_eq!(liblrhsmm_rs_duration_statistics_release_f32(&mut direct), 0);
    }
}

#[test]
fn update_f32_source_results_flags_floors_and_failure_atomicity() {
    // SAFETY: Independent initialized descriptors, result storage and uniquely owned handles.
    unsafe {
        let mut options = LiblrhsmmRsUpdateOptionsF32 {
            duration_mode: 99,
            duration_variance_floor: 99.0,
            emission_variance_floor: 99.0,
        };
        assert_eq!(liblrhsmm_rs_update_options_default_f32(&mut options), 0);
        assert_eq!(
            options,
            LiblrhsmmRsUpdateOptionsF32 {
                duration_mode: LIBLRHSMM_RS_DURATION_NORMAL,
                duration_variance_floor: 0.05,
                emission_variance_floor: 0.01
            }
        );
        let mut duration = null_mut();
        let mut statistics = null_mut();
        let mut values = LiblrhsmmRsDurationStatisticsValuesF32 {
            value_sum: 9.0,
            squared_sum: 29.0,
            occupancy: 3.0,
        };
        assert_eq!(
            liblrhsmm_rs_duration_statistics_create_f32(&values, &mut statistics),
            0
        );
        let mut parameters = LiblrhsmmRsDurationParametersF32 {
            mean: 7.0,
            variance: 3.0,
            minimum: 2,
            maximum: 6,
            fixed_mean: -1,
            variance_floor: 0.0,
        };
        assert_eq!(
            liblrhsmm_rs_duration_create_f32(&parameters, &mut duration),
            0
        );
        let fixture = include_str!("fixtures/c-updates.txt");
        let mut compared = 0;
        for line in fixture.lines().filter(|line| line.starts_with("duration ")) {
            let fields: Vec<_> = line.split_whitespace().collect();
            let scenario: usize = fields[1].parse().unwrap();
            parameters.mean = 7.0;
            parameters.variance = 3.0;
            parameters.variance_floor = if scenario == 2 { 2.5 } else { 0.0 };
            values.value_sum = if scenario == 3 { 1.5 } else { 9.0 };
            values.occupancy = if scenario == 4 { 1e-8 } else { 3.0 };
            options.duration_mode = if scenario == 3 {
                LIBLRHSMM_RS_DURATION_GEOMETRIC
            } else {
                LIBLRHSMM_RS_DURATION_NORMAL
            };
            assert_eq!(liblrhsmm_rs_duration_set_f32(duration, &parameters), 0);
            assert_eq!(
                liblrhsmm_rs_duration_statistics_set_f32(statistics, &values),
                0
            );
            let mut expected = DurationParameters::<f32> {
                mean: parameters.mean,
                variance: parameters.variance,
                minimum: parameters.minimum,
                maximum: parameters.maximum,
                fixed_mean: parameters.fixed_mean,
                variance_floor: parameters.variance_floor,
            };
            let changed = expected
                .update(
                    &DurationSufficientStatistics {
                        value_sum: values.value_sum,
                        squared_sum: values.squared_sum,
                        occupancy: values.occupancy,
                    },
                    UpdateSettings {
                        duration_mode: if scenario == 3 {
                            DurationMode::Geometric
                        } else {
                            DurationMode::Normal
                        },
                        ..Default::default()
                    },
                )
                .unwrap();
            let mut updated = 99;
            assert_eq!(
                liblrhsmm_rs_duration_update_f32(duration, statistics, &options, &mut updated),
                0
            );
            assert_eq!(updated, u32::from(changed));
            assert_eq!(liblrhsmm_rs_duration_get_f32(duration, &mut parameters), 0);
            assert_eq!(parameters.mean.to_bits(), expected.mean.to_bits());
            assert_eq!(parameters.variance.to_bits(), expected.variance.to_bits());
            assert_eq!(parameters.minimum, 2);
            assert_eq!(parameters.maximum, 6);
            for (actual, expected) in [parameters.mean, parameters.variance]
                .into_iter()
                .zip(&fields[2..])
            {
                assert_eq!(
                    actual.to_bits(),
                    expected.parse::<f32>().unwrap().to_bits(),
                    "{line}"
                );
                compared += 1;
            }
        }
        assert_eq!(compared, 10);
        options.duration_mode = LIBLRHSMM_RS_DURATION_NORMAL;
        parameters.fixed_mean = 4;
        parameters.variance_floor = 0.0;
        values = LiblrhsmmRsDurationStatisticsValuesF32 {
            value_sum: 6.0,
            squared_sum: 20.0,
            occupancy: 2.0,
        };
        assert_eq!(liblrhsmm_rs_duration_set_f32(duration, &parameters), 0);
        assert_eq!(
            liblrhsmm_rs_duration_statistics_set_f32(statistics, &values),
            0
        );
        let mut updated = 99;
        assert_eq!(
            liblrhsmm_rs_duration_update_f32(duration, statistics, &options, &mut updated),
            0
        );
        assert_eq!(updated, 1);
        assert_eq!(liblrhsmm_rs_duration_get_f32(duration, &mut parameters), 0);
        assert_eq!((parameters.mean, parameters.variance), (4.0, 2.0));
        let before = parameters;
        for bad in [
            LiblrhsmmRsUpdateOptionsF32 {
                duration_mode: 99,
                ..options
            },
            LiblrhsmmRsUpdateOptionsF32 {
                duration_variance_floor: -1.0,
                ..options
            },
            LiblrhsmmRsUpdateOptionsF32 {
                emission_variance_floor: f32::NAN,
                ..options
            },
        ] {
            updated = 99;
            assert_eq!(
                liblrhsmm_rs_duration_update_f32(duration, statistics, &bad, &mut updated),
                3
            );
            assert_eq!(updated, 99);
            assert_eq!(liblrhsmm_rs_duration_get_f32(duration, &mut parameters), 0);
            assert_eq!(parameters, before);
        }
        assert_eq!(
            liblrhsmm_rs_duration_update_f32(duration, statistics, &options, null_mut()),
            1
        );
        assert_eq!(liblrhsmm_rs_duration_get_f32(duration, &mut parameters), 0);
        assert_eq!(parameters, before);
        assert_eq!(
            liblrhsmm_rs_duration_update_f32(duration, null(), &options, &mut updated),
            1
        );
        assert_eq!(
            liblrhsmm_rs_duration_update_f32(duration, statistics, null(), &mut updated),
            1
        );
        assert_eq!(
            liblrhsmm_rs_duration_update_f32(null_mut(), statistics, &options, &mut updated),
            1
        );
        assert_eq!(liblrhsmm_rs_update_options_default_f32(null_mut()), 1);
        for bad in [
            LiblrhsmmRsDurationStatisticsValuesF32 {
                value_sum: f32::NAN,
                ..values
            },
            LiblrhsmmRsDurationStatisticsValuesF32 {
                squared_sum: -1.0,
                ..values
            },
            LiblrhsmmRsDurationStatisticsValuesF32 {
                occupancy: -1.0,
                ..values
            },
            LiblrhsmmRsDurationStatisticsValuesF32 {
                value_sum: f32::MAX,
                squared_sum: f32::MAX,
                occupancy: 1e-6,
            },
        ] {
            assert_eq!(
                liblrhsmm_rs_duration_statistics_set_f32(statistics, &bad),
                0
            );
            updated = 99;
            assert_eq!(
                liblrhsmm_rs_duration_update_f32(duration, statistics, &options, &mut updated),
                3
            );
            assert_eq!(updated, 99);
            assert_eq!(liblrhsmm_rs_duration_get_f32(duration, &mut parameters), 0);
            assert_eq!(parameters, before);
        }
        options.duration_mode = LIBLRHSMM_RS_DURATION_GEOMETRIC;
        values.value_sum = 0.0;
        assert_eq!(
            liblrhsmm_rs_duration_statistics_set_f32(statistics, &values),
            0
        );
        assert_eq!(
            liblrhsmm_rs_duration_update_f32(duration, statistics, &options, &mut updated),
            3
        );
        assert_eq!(liblrhsmm_rs_duration_get_f32(duration, &mut parameters), 0);
        assert_eq!(parameters, before);
        assert_eq!(
            liblrhsmm_rs_duration_statistics_release_f32(&mut statistics),
            0
        );
        assert_eq!(liblrhsmm_rs_duration_release_f32(&mut duration), 0);
    }
}

#[test]
fn statistics_f64_all_fields_bits_ownership_and_clear() {
    // SAFETY: Independent initialized descriptors and exclusive, uniquely released owners.
    unsafe {
        let mut values = LiblrhsmmRsDurationStatisticsValuesF64 {
            value_sum: 99.0,
            squared_sum: 99.0,
            occupancy: 99.0,
        };
        assert_eq!(liblrhsmm_rs_duration_statistics_default_f64(&mut values), 0);
        assert_eq!(
            values,
            LiblrhsmmRsDurationStatisticsValuesF64 {
                value_sum: 0.0,
                squared_sum: 0.0,
                occupancy: 0.0
            }
        );
        let mut parameters = LiblrhsmmRsDurationParametersF64 {
            mean: 0.0,
            variance: 0.0,
            minimum: 0,
            maximum: 0,
            fixed_mean: 0,
            variance_floor: 0.0,
        };
        assert_eq!(liblrhsmm_rs_duration_default_f64(&mut parameters), 0);
        let mut duration = null_mut();
        assert_eq!(
            liblrhsmm_rs_duration_create_f64(&parameters, &mut duration),
            0
        );
        let mut statistics = null_mut();
        assert_eq!(
            liblrhsmm_rs_duration_statistics_from_duration_f64(duration, &mut statistics),
            0
        );
        assert_eq!(liblrhsmm_rs_duration_release_f64(&mut duration), 0);
        values = LiblrhsmmRsDurationStatisticsValuesF64 {
            value_sum: 6.0,
            squared_sum: 20.0,
            occupancy: 2.0,
        };
        assert_eq!(
            liblrhsmm_rs_duration_statistics_set_f64(statistics, &values),
            0
        );
        let mut copy = null_mut();
        assert_eq!(
            liblrhsmm_rs_duration_statistics_clone_f64(statistics, &mut copy),
            0
        );
        assert_eq!(liblrhsmm_rs_duration_statistics_clear_f64(statistics), 0);
        let mut fetched = values;
        assert_eq!(
            liblrhsmm_rs_duration_statistics_get_f64(statistics, &mut fetched),
            0
        );
        assert_eq!(
            fetched,
            LiblrhsmmRsDurationStatisticsValuesF64 {
                value_sum: 0.0,
                squared_sum: 0.0,
                occupancy: 0.0
            }
        );
        assert_eq!(
            liblrhsmm_rs_duration_statistics_get_f64(copy, &mut fetched),
            0
        );
        assert_eq!(fetched, values);
        values.value_sum = f64::from_bits(0x7ff8000000001234);
        values.squared_sum = -1.0;
        values.occupancy = f64::NEG_INFINITY;
        assert_eq!(
            liblrhsmm_rs_duration_statistics_set_f64(statistics, &values),
            0
        );
        assert_eq!(
            liblrhsmm_rs_duration_statistics_get_f64(statistics, &mut fetched),
            0
        );
        assert_eq!(fetched.value_sum.to_bits(), values.value_sum.to_bits());
        assert_eq!(fetched.squared_sum.to_bits(), values.squared_sum.to_bits());
        assert_eq!(fetched.occupancy.to_bits(), values.occupancy.to_bits());
        let mut direct = null_mut();
        assert_eq!(
            liblrhsmm_rs_duration_statistics_create_f64(&values, &mut direct),
            0
        );
        assert_eq!(
            liblrhsmm_rs_duration_statistics_get_f64(direct, &mut fetched),
            0
        );
        assert_eq!(fetched.value_sum.to_bits(), values.value_sum.to_bits());
        assert_eq!(
            liblrhsmm_rs_duration_statistics_create_f64(null(), &mut direct),
            1
        );
        assert_eq!(
            liblrhsmm_rs_duration_statistics_create_f64(&values, null_mut()),
            1
        );
        let misaligned = (&values as *const LiblrhsmmRsDurationStatisticsValuesF64)
            .cast::<u8>()
            .wrapping_add(1)
            .cast();
        assert_eq!(
            liblrhsmm_rs_duration_statistics_create_f64(misaligned, &mut direct),
            1
        );
        assert_eq!(
            liblrhsmm_rs_duration_statistics_get_f64(copy, null_mut()),
            1
        );
        assert_eq!(liblrhsmm_rs_duration_statistics_set_f64(copy, null()), 1);
        assert_eq!(
            liblrhsmm_rs_duration_statistics_clone_f64(copy, null_mut()),
            1
        );
        assert_eq!(liblrhsmm_rs_duration_statistics_default_f64(null_mut()), 1);
        assert_eq!(
            liblrhsmm_rs_duration_statistics_from_duration_f64(null(), &mut statistics),
            1
        );
        assert_eq!(liblrhsmm_rs_duration_statistics_clear_f64(null_mut()), 1);
        assert_eq!(liblrhsmm_rs_duration_statistics_release_f64(null_mut()), 1);
        assert_eq!(
            liblrhsmm_rs_duration_statistics_release_f64(&mut statistics),
            0
        );
        assert!(statistics.is_null());
        assert_eq!(
            liblrhsmm_rs_duration_statistics_release_f64(&mut statistics),
            0
        );
        assert_eq!(
            liblrhsmm_rs_duration_statistics_get_f64(copy, &mut fetched),
            0
        );
        assert_eq!(fetched.value_sum, 6.0);
        assert_eq!(liblrhsmm_rs_duration_statistics_release_f64(&mut copy), 0);
        assert_eq!(liblrhsmm_rs_duration_statistics_clear_f64(direct), 0);
        assert_eq!(liblrhsmm_rs_duration_statistics_release_f64(&mut direct), 0);
    }
}

#[test]
fn update_f64_source_results_flags_floors_and_failure_atomicity() {
    // SAFETY: Independent initialized descriptors, result storage and uniquely owned handles.
    unsafe {
        let mut options = LiblrhsmmRsUpdateOptionsF64 {
            duration_mode: 99,
            duration_variance_floor: 99.0,
            emission_variance_floor: 99.0,
        };
        assert_eq!(liblrhsmm_rs_update_options_default_f64(&mut options), 0);
        assert_eq!(
            options,
            LiblrhsmmRsUpdateOptionsF64 {
                duration_mode: LIBLRHSMM_RS_DURATION_NORMAL,
                duration_variance_floor: 0.05,
                emission_variance_floor: 0.01
            }
        );
        let mut duration = null_mut();
        let mut statistics = null_mut();
        let mut values = LiblrhsmmRsDurationStatisticsValuesF64 {
            value_sum: 9.0,
            squared_sum: 29.0,
            occupancy: 3.0,
        };
        assert_eq!(
            liblrhsmm_rs_duration_statistics_create_f64(&values, &mut statistics),
            0
        );
        let mut parameters = LiblrhsmmRsDurationParametersF64 {
            mean: 7.0,
            variance: 3.0,
            minimum: 2,
            maximum: 6,
            fixed_mean: -1,
            variance_floor: 0.0,
        };
        assert_eq!(
            liblrhsmm_rs_duration_create_f64(&parameters, &mut duration),
            0
        );
        let fixture = include_str!("fixtures/c-double-update.txt");
        let mut compared = 0;
        for line in fixture.lines().filter(|line| line.starts_with("duration ")) {
            let fields: Vec<_> = line.split_whitespace().collect();
            let scenario: usize = fields[1].parse().unwrap();
            parameters.mean = 7.0;
            parameters.variance = 3.0;
            parameters.variance_floor = if scenario == 2 { 2.5 } else { 0.0 };
            values.value_sum = if scenario == 3 { 1.5 } else { 9.0 };
            values.occupancy = if scenario == 4 { 1e-8 } else { 3.0 };
            options.duration_mode = if scenario == 3 {
                LIBLRHSMM_RS_DURATION_GEOMETRIC
            } else {
                LIBLRHSMM_RS_DURATION_NORMAL
            };
            assert_eq!(liblrhsmm_rs_duration_set_f64(duration, &parameters), 0);
            assert_eq!(
                liblrhsmm_rs_duration_statistics_set_f64(statistics, &values),
                0
            );
            let mut expected = DurationParameters::<f64> {
                mean: parameters.mean,
                variance: parameters.variance,
                minimum: parameters.minimum,
                maximum: parameters.maximum,
                fixed_mean: parameters.fixed_mean,
                variance_floor: parameters.variance_floor,
            };
            let changed = expected
                .update(
                    &DurationSufficientStatistics {
                        value_sum: values.value_sum,
                        squared_sum: values.squared_sum,
                        occupancy: values.occupancy,
                    },
                    UpdateSettings {
                        duration_mode: if scenario == 3 {
                            DurationMode::Geometric
                        } else {
                            DurationMode::Normal
                        },
                        ..Default::default()
                    },
                )
                .unwrap();
            let mut updated = 99;
            assert_eq!(
                liblrhsmm_rs_duration_update_f64(duration, statistics, &options, &mut updated),
                0
            );
            assert_eq!(updated, u32::from(changed));
            assert_eq!(liblrhsmm_rs_duration_get_f64(duration, &mut parameters), 0);
            assert_eq!(parameters.mean.to_bits(), expected.mean.to_bits());
            assert_eq!(parameters.variance.to_bits(), expected.variance.to_bits());
            assert_eq!(parameters.minimum, 2);
            assert_eq!(parameters.maximum, 6);
            for (actual, expected) in [parameters.mean, parameters.variance]
                .into_iter()
                .zip(&fields[2..])
            {
                let expected: f64 = expected.parse().unwrap();
                assert!(
                    (actual - expected).abs() / expected.abs().max(1.0) <= 2e-14,
                    "{line}"
                );
                compared += 1;
            }
        }
        assert_eq!(compared, 10);
        options.duration_mode = LIBLRHSMM_RS_DURATION_NORMAL;
        parameters.fixed_mean = 4;
        parameters.variance_floor = 0.0;
        values = LiblrhsmmRsDurationStatisticsValuesF64 {
            value_sum: 6.0,
            squared_sum: 20.0,
            occupancy: 2.0,
        };
        assert_eq!(liblrhsmm_rs_duration_set_f64(duration, &parameters), 0);
        assert_eq!(
            liblrhsmm_rs_duration_statistics_set_f64(statistics, &values),
            0
        );
        let mut updated = 99;
        assert_eq!(
            liblrhsmm_rs_duration_update_f64(duration, statistics, &options, &mut updated),
            0
        );
        assert_eq!(updated, 1);
        assert_eq!(liblrhsmm_rs_duration_get_f64(duration, &mut parameters), 0);
        assert_eq!((parameters.mean, parameters.variance), (4.0, 2.0));
        let before = parameters;
        for bad in [
            LiblrhsmmRsUpdateOptionsF64 {
                duration_mode: 99,
                ..options
            },
            LiblrhsmmRsUpdateOptionsF64 {
                duration_variance_floor: -1.0,
                ..options
            },
            LiblrhsmmRsUpdateOptionsF64 {
                emission_variance_floor: f64::NAN,
                ..options
            },
        ] {
            updated = 99;
            assert_eq!(
                liblrhsmm_rs_duration_update_f64(duration, statistics, &bad, &mut updated),
                3
            );
            assert_eq!(updated, 99);
            assert_eq!(liblrhsmm_rs_duration_get_f64(duration, &mut parameters), 0);
            assert_eq!(parameters, before);
        }
        assert_eq!(
            liblrhsmm_rs_duration_update_f64(duration, statistics, &options, null_mut()),
            1
        );
        assert_eq!(liblrhsmm_rs_duration_get_f64(duration, &mut parameters), 0);
        assert_eq!(parameters, before);
        assert_eq!(
            liblrhsmm_rs_duration_update_f64(duration, null(), &options, &mut updated),
            1
        );
        assert_eq!(
            liblrhsmm_rs_duration_update_f64(duration, statistics, null(), &mut updated),
            1
        );
        assert_eq!(
            liblrhsmm_rs_duration_update_f64(null_mut(), statistics, &options, &mut updated),
            1
        );
        assert_eq!(liblrhsmm_rs_update_options_default_f64(null_mut()), 1);
        for bad in [
            LiblrhsmmRsDurationStatisticsValuesF64 {
                value_sum: f64::NAN,
                ..values
            },
            LiblrhsmmRsDurationStatisticsValuesF64 {
                squared_sum: -1.0,
                ..values
            },
            LiblrhsmmRsDurationStatisticsValuesF64 {
                occupancy: -1.0,
                ..values
            },
            LiblrhsmmRsDurationStatisticsValuesF64 {
                value_sum: f64::MAX,
                squared_sum: f64::MAX,
                occupancy: 1e-6,
            },
        ] {
            assert_eq!(
                liblrhsmm_rs_duration_statistics_set_f64(statistics, &bad),
                0
            );
            updated = 99;
            assert_eq!(
                liblrhsmm_rs_duration_update_f64(duration, statistics, &options, &mut updated),
                3
            );
            assert_eq!(updated, 99);
            assert_eq!(liblrhsmm_rs_duration_get_f64(duration, &mut parameters), 0);
            assert_eq!(parameters, before);
        }
        options.duration_mode = LIBLRHSMM_RS_DURATION_GEOMETRIC;
        values.value_sum = 0.0;
        assert_eq!(
            liblrhsmm_rs_duration_statistics_set_f64(statistics, &values),
            0
        );
        assert_eq!(
            liblrhsmm_rs_duration_update_f64(duration, statistics, &options, &mut updated),
            3
        );
        assert_eq!(liblrhsmm_rs_duration_get_f64(duration, &mut parameters), 0);
        assert_eq!(parameters, before);
        assert_eq!(
            liblrhsmm_rs_duration_statistics_release_f64(&mut statistics),
            0
        );
        assert_eq!(liblrhsmm_rs_duration_release_f64(&mut duration), 0);
    }
}
