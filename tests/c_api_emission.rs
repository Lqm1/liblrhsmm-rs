#![cfg(feature = "c-api")]
use liblrhsmm_rs::c_api::*;
use liblrhsmm_rs::emission::EmissionPruningOptions;
use liblrhsmm_rs::scalar::Scalar;
use std::ptr::{null, null_mut};
#[path = "common/c_api_emission_inputs.rs"]
mod inputs;
use inputs::{inputs_f32, inputs_f64};
#[path = "common/emissions.rs"]
#[allow(dead_code)]
mod common;

fn reference<T: Scalar>(actual: T, expected: T, tolerance: f64) {
    let (actual, expected): (f64, f64) = (actual.into(), expected.into());
    if tolerance == 0.0 {
        assert_eq!(actual.to_bits(), expected.to_bits());
    } else {
        assert!((actual - expected).abs() / expected.abs().max(1.0) <= tolerance);
    }
}

unsafe fn log_values_f32(owner: *const LiblrhsmmRsLogEmissionsF32) -> Vec<f32> {
    unsafe {
        let mut layout = LiblrhsmmRsInferenceTableLayout {
            frames: 0,
            states: 0,
            values: 0,
        };
        assert_eq!(liblrhsmm_rs_log_emissions_layout_f32(owner, &mut layout), 0);
        assert_eq!((layout.frames, layout.states, layout.values), (12, 3, 36));
        let mut values = vec![0.0; 36];
        assert_eq!(
            liblrhsmm_rs_log_emissions_copy_values_f32(owner, 0, values.as_mut_ptr(), 36),
            0
        );
        values
    }
}
unsafe fn density_values_f32(owner: *const LiblrhsmmRsEmissionDensitiesF32) -> Vec<f32> {
    unsafe {
        let mut layout = LiblrhsmmRsInferenceTableLayout {
            frames: 0,
            states: 0,
            values: 0,
        };
        assert_eq!(
            liblrhsmm_rs_emission_densities_layout_f32(owner, &mut layout),
            0
        );
        assert_eq!((layout.frames, layout.states, layout.values), (12, 3, 36));
        let mut values = vec![0.0; 36];
        assert_eq!(
            liblrhsmm_rs_emission_densities_copy_values_f32(owner, 0, values.as_mut_ptr(), 36),
            0
        );
        values
    }
}

#[test]
fn f32_emission_all_modes_multistream_native_corpora_and_failures() {
    let (native, observation, segmentation) = common::sample_with_precision::<f32>();
    unsafe {
        let (mut model, mut obs, mut seg) = inputs_f32();
        let prep = LiblrhsmmRsDurationPreparationF32 {
            duration_cache_length: 2,
            duration_weight: 1.0,
            temperature: 1.0,
        };
        let mut prepared = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_prepare_f32(model, &prep, &mut prepared),
            0
        );
        let mut settings = LiblrhsmmRsEmissionSettingsF32 {
            temperature: 99.0,
            pruning: 99,
            radius: 99.0,
            slope: 99.0,
        };
        assert_eq!(liblrhsmm_rs_emission_settings_default_f32(&mut settings), 0);
        assert_eq!(settings.temperature, 1.0);
        assert_eq!(settings.pruning, LIBLRHSMM_RS_EMISSION_PRUNING_NONE);
        assert_eq!(settings.radius, 5.0);
        assert_eq!(
            settings.slope,
            liblrhsmm_rs::geometric::GeometricSettings::<f32>::default().pruning_slope
        );
        assert_eq!(liblrhsmm_rs_emission_settings_default_f32(null_mut()), 1);
        assert_eq!(
            liblrhsmm_rs_emission_settings_default_f32(std::ptr::without_provenance_mut(1)),
            1
        );
        let modes = [
            (0, f32::NAN, f32::NAN),
            (1, 0.25, f32::NAN),
            (1, 0.5, f32::NAN),
            (1, 1.75, f32::NAN),
            (1, 2.5, f32::NAN),
            (1, 1.0, f32::NAN),
            (2, f32::NAN, 0.34),
        ];
        for temperature in [0.0, 0.75, 1.75, f32::from_bits(0x3f800001)] {
            for (mode, radius, slope) in modes {
                let options = LiblrhsmmRsEmissionSettingsF32 {
                    temperature,
                    pruning: mode,
                    radius,
                    slope,
                };
                let pruning = EmissionPruningOptions::try_from(options).unwrap();
                let expected = native
                    .emission_log_probabilities(&observation, &segmentation, temperature, pruning)
                    .unwrap();
                let expected_density = expected.probabilities();
                let expected_bits: Vec<_> = expected.values().iter().map(|v| v.to_bits()).collect();
                let density_bits: Vec<_> = expected_density
                    .values()
                    .iter()
                    .map(|v| v.to_bits())
                    .collect();
                let (mut log, mut cached, mut density, mut cached_density) =
                    (null_mut(), null_mut(), null_mut(), null_mut());
                assert_eq!(
                    liblrhsmm_rs_model_emission_log_probabilities_f32(
                        model, obs, seg, &options, &mut log
                    ),
                    0
                );
                assert_eq!(
                    liblrhsmm_rs_prepared_model_emission_log_probabilities_f32(
                        prepared,
                        obs,
                        seg,
                        &options,
                        &mut cached
                    ),
                    0
                );
                assert_eq!(
                    liblrhsmm_rs_model_emission_probabilities_f32(
                        model,
                        obs,
                        seg,
                        &options,
                        &mut density
                    ),
                    0
                );
                assert_eq!(
                    liblrhsmm_rs_prepared_model_emission_probabilities_f32(
                        prepared,
                        obs,
                        seg,
                        &options,
                        &mut cached_density
                    ),
                    0
                );
                for handle in [log, cached] {
                    assert_eq!(
                        log_values_f32(handle)
                            .iter()
                            .map(|v| v.to_bits())
                            .collect::<Vec<_>>(),
                        expected_bits
                    );
                }
                for handle in [density, cached_density] {
                    assert_eq!(
                        density_values_f32(handle)
                            .iter()
                            .map(|v| v.to_bits())
                            .collect::<Vec<_>>(),
                        density_bits
                    );
                }
                assert_eq!(liblrhsmm_rs_log_emissions_release_f32(&mut log), 0);
                assert_eq!(liblrhsmm_rs_log_emissions_release_f32(&mut cached), 0);
                assert_eq!(liblrhsmm_rs_emission_densities_release_f32(&mut density), 0);
                assert_eq!(
                    liblrhsmm_rs_emission_densities_release_f32(&mut cached_density),
                    0
                );
            }
        }
        let mut comparisons = 0;
        for (fixture, fractional) in [
            (include_str!("fixtures/c-emission-tables.txt"), false),
            (include_str!("fixtures/c-fractional-emissions.txt"), true),
        ] {
            for line in fixture.lines() {
                let fields: Vec<_> = line.split_whitespace().collect();
                let scenario: usize = fields[0].parse().unwrap();
                let index: usize = fields[1].parse().unwrap();
                let options = LiblrhsmmRsEmissionSettingsF32 {
                    temperature: 0.75,
                    pruning: if fractional || scenario == 0 { 1 } else { 2 },
                    radius: if fractional {
                        [0.25, 0.5, 1.75, 2.5][scenario]
                    } else {
                        1.0
                    },
                    slope: 0.34,
                };
                let mut table = null_mut();
                assert_eq!(
                    liblrhsmm_rs_prepared_model_emission_log_probabilities_f32(
                        prepared, obs, seg, &options, &mut table
                    ),
                    0
                );
                reference(
                    log_values_f32(table)[index],
                    fields[2].parse::<f32>().unwrap(),
                    0.0,
                );
                comparisons += 1;
                assert_eq!(liblrhsmm_rs_log_emissions_release_f32(&mut table), 0);
            }
        }
        assert_eq!(comparisons, 216);
        for options in [
            LiblrhsmmRsEmissionSettingsF32 {
                pruning: 99,
                ..settings
            },
            LiblrhsmmRsEmissionSettingsF32 {
                temperature: f32::NAN,
                ..settings
            },
            LiblrhsmmRsEmissionSettingsF32 {
                temperature: -0.25,
                ..settings
            },
            LiblrhsmmRsEmissionSettingsF32 {
                pruning: 1,
                radius: f32::INFINITY,
                ..settings
            },
            LiblrhsmmRsEmissionSettingsF32 {
                pruning: 1,
                radius: -0.25,
                ..settings
            },
            LiblrhsmmRsEmissionSettingsF32 {
                pruning: 2,
                slope: f32::NAN,
                ..settings
            },
            LiblrhsmmRsEmissionSettingsF32 {
                pruning: 2,
                slope: -0.25,
                ..settings
            },
        ] {
            let mut log = std::ptr::without_provenance_mut::<LiblrhsmmRsLogEmissionsF32>(0x1000);
            let mut density =
                std::ptr::without_provenance_mut::<LiblrhsmmRsEmissionDensitiesF32>(0x2000);
            assert_eq!(
                liblrhsmm_rs_model_emission_log_probabilities_f32(
                    model, obs, seg, &options, &mut log
                ),
                3
            );
            assert_eq!(
                liblrhsmm_rs_prepared_model_emission_log_probabilities_f32(
                    prepared, obs, seg, &options, &mut log
                ),
                3
            );
            assert_eq!(
                liblrhsmm_rs_model_emission_probabilities_f32(
                    model,
                    obs,
                    seg,
                    &options,
                    &mut density
                ),
                3
            );
            assert_eq!(
                liblrhsmm_rs_prepared_model_emission_probabilities_f32(
                    prepared,
                    obs,
                    seg,
                    &options,
                    &mut density
                ),
                3
            );
            assert_eq!(log.addr(), 0x1000);
            assert_eq!(density.addr(), 0x2000);
        }
        let mut unchanged = std::ptr::without_provenance_mut::<LiblrhsmmRsLogEmissionsF32>(0x1000);
        for (m, o, s, config) in [
            (
                null(),
                obs.cast_const(),
                seg.cast_const(),
                &settings as *const _,
            ),
            (
                model.cast_const(),
                null(),
                seg.cast_const(),
                &settings as *const _,
            ),
            (
                model.cast_const(),
                obs.cast_const(),
                null(),
                &settings as *const _,
            ),
            (
                model.cast_const(),
                obs.cast_const(),
                seg.cast_const(),
                null(),
            ),
        ] {
            assert_eq!(
                liblrhsmm_rs_model_emission_log_probabilities_f32(m, o, s, config, &mut unchanged),
                1
            );
            assert_eq!(unchanged.addr(), 0x1000);
        }
        assert_eq!(
            liblrhsmm_rs_model_emission_log_probabilities_f32(
                model,
                obs,
                seg,
                &settings,
                null_mut()
            ),
            1
        );
        let labels = [-1, 1, 2];
        assert_eq!(
            liblrhsmm_rs_segmentation_set_output_states_f32(seg, 0, 0, labels.as_ptr(), 3),
            0
        );
        assert_eq!(
            liblrhsmm_rs_prepared_model_emission_log_probabilities_f32(
                prepared,
                obs,
                seg,
                &settings,
                &mut unchanged
            ),
            3
        );
        assert_eq!(unchanged.addr(), 0x1000);
        let labels = [0, 1, 2];
        assert_eq!(
            liblrhsmm_rs_segmentation_set_output_states_f32(seg, 0, 0, labels.as_ptr(), 3),
            0
        );
        assert_eq!(liblrhsmm_rs_model_release_f32(&mut model), 0);
        let (mut log, mut density) = (null_mut(), null_mut());
        assert_eq!(
            liblrhsmm_rs_prepared_model_emission_log_probabilities_f32(
                prepared, obs, seg, &settings, &mut log
            ),
            0
        );
        assert_eq!(
            liblrhsmm_rs_prepared_model_emission_probabilities_f32(
                prepared,
                obs,
                seg,
                &settings,
                &mut density
            ),
            0
        );
        assert_eq!(liblrhsmm_rs_prepared_model_release_f32(&mut prepared), 0);
        assert_eq!(liblrhsmm_rs_observation_release_f32(&mut obs), 0);
        assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut seg), 0);
        let expected = native
            .emission_log_probabilities(
                &observation,
                &segmentation,
                1.0,
                EmissionPruningOptions::None,
            )
            .unwrap();
        assert_eq!(
            log_values_f32(log)
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            expected
                .values()
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            density_values_f32(density)
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            expected
                .probabilities()
                .values()
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>()
        );
        assert_eq!(liblrhsmm_rs_log_emissions_release_f32(&mut log), 0);
        assert_eq!(liblrhsmm_rs_emission_densities_release_f32(&mut density), 0);
    }
}

unsafe fn log_values_f64(owner: *const LiblrhsmmRsLogEmissionsF64) -> Vec<f64> {
    unsafe {
        let mut layout = LiblrhsmmRsInferenceTableLayout {
            frames: 0,
            states: 0,
            values: 0,
        };
        assert_eq!(liblrhsmm_rs_log_emissions_layout_f64(owner, &mut layout), 0);
        assert_eq!((layout.frames, layout.states, layout.values), (12, 3, 36));
        let mut values = vec![0.0; 36];
        assert_eq!(
            liblrhsmm_rs_log_emissions_copy_values_f64(owner, 0, values.as_mut_ptr(), 36),
            0
        );
        values
    }
}
unsafe fn density_values_f64(owner: *const LiblrhsmmRsEmissionDensitiesF64) -> Vec<f64> {
    unsafe {
        let mut layout = LiblrhsmmRsInferenceTableLayout {
            frames: 0,
            states: 0,
            values: 0,
        };
        assert_eq!(
            liblrhsmm_rs_emission_densities_layout_f64(owner, &mut layout),
            0
        );
        assert_eq!((layout.frames, layout.states, layout.values), (12, 3, 36));
        let mut values = vec![0.0; 36];
        assert_eq!(
            liblrhsmm_rs_emission_densities_copy_values_f64(owner, 0, values.as_mut_ptr(), 36),
            0
        );
        values
    }
}

#[test]
fn f64_emission_all_modes_multistream_native_corpora_and_failures() {
    let (native, observation, segmentation) = common::sample_with_precision::<f64>();
    unsafe {
        let (mut model, mut obs, mut seg) = inputs_f64();
        let prep = LiblrhsmmRsDurationPreparationF64 {
            duration_cache_length: 2,
            duration_weight: 1.0,
            temperature: 1.0,
        };
        let mut prepared = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_prepare_f64(model, &prep, &mut prepared),
            0
        );
        let mut settings = LiblrhsmmRsEmissionSettingsF64 {
            temperature: 99.0,
            pruning: 99,
            radius: 99.0,
            slope: 99.0,
        };
        assert_eq!(liblrhsmm_rs_emission_settings_default_f64(&mut settings), 0);
        assert_eq!(settings.temperature, 1.0);
        assert_eq!(settings.pruning, LIBLRHSMM_RS_EMISSION_PRUNING_NONE);
        assert_eq!(settings.radius, 5.0);
        assert_eq!(
            settings.slope,
            liblrhsmm_rs::geometric::GeometricSettings::<f64>::default().pruning_slope
        );
        assert_eq!(liblrhsmm_rs_emission_settings_default_f64(null_mut()), 1);
        assert_eq!(
            liblrhsmm_rs_emission_settings_default_f64(std::ptr::without_provenance_mut(1)),
            1
        );
        let modes = [
            (0, f64::NAN, f64::NAN),
            (1, 0.25, f64::NAN),
            (1, 0.5, f64::NAN),
            (1, 1.75, f64::NAN),
            (1, 2.5, f64::NAN),
            (1, 1.0, f64::NAN),
            (2, f64::NAN, 0.34),
        ];
        for temperature in [0.0, 0.75, 1.75, f64::from_bits(0x3ff0000000000001)] {
            for (mode, radius, slope) in modes {
                let options = LiblrhsmmRsEmissionSettingsF64 {
                    temperature,
                    pruning: mode,
                    radius,
                    slope,
                };
                let pruning = EmissionPruningOptions::try_from(options).unwrap();
                let expected = native
                    .emission_log_probabilities(&observation, &segmentation, temperature, pruning)
                    .unwrap();
                let expected_density = expected.probabilities();
                let expected_bits: Vec<_> = expected.values().iter().map(|v| v.to_bits()).collect();
                let density_bits: Vec<_> = expected_density
                    .values()
                    .iter()
                    .map(|v| v.to_bits())
                    .collect();
                let (mut log, mut cached, mut density, mut cached_density) =
                    (null_mut(), null_mut(), null_mut(), null_mut());
                assert_eq!(
                    liblrhsmm_rs_model_emission_log_probabilities_f64(
                        model, obs, seg, &options, &mut log
                    ),
                    0
                );
                assert_eq!(
                    liblrhsmm_rs_prepared_model_emission_log_probabilities_f64(
                        prepared,
                        obs,
                        seg,
                        &options,
                        &mut cached
                    ),
                    0
                );
                assert_eq!(
                    liblrhsmm_rs_model_emission_probabilities_f64(
                        model,
                        obs,
                        seg,
                        &options,
                        &mut density
                    ),
                    0
                );
                assert_eq!(
                    liblrhsmm_rs_prepared_model_emission_probabilities_f64(
                        prepared,
                        obs,
                        seg,
                        &options,
                        &mut cached_density
                    ),
                    0
                );
                for handle in [log, cached] {
                    assert_eq!(
                        log_values_f64(handle)
                            .iter()
                            .map(|v| v.to_bits())
                            .collect::<Vec<_>>(),
                        expected_bits
                    );
                }
                for handle in [density, cached_density] {
                    assert_eq!(
                        density_values_f64(handle)
                            .iter()
                            .map(|v| v.to_bits())
                            .collect::<Vec<_>>(),
                        density_bits
                    );
                }
                assert_eq!(liblrhsmm_rs_log_emissions_release_f64(&mut log), 0);
                assert_eq!(liblrhsmm_rs_log_emissions_release_f64(&mut cached), 0);
                assert_eq!(liblrhsmm_rs_emission_densities_release_f64(&mut density), 0);
                assert_eq!(
                    liblrhsmm_rs_emission_densities_release_f64(&mut cached_density),
                    0
                );
            }
        }
        let mut comparisons = 0;
        for (fixture, fractional) in [
            (include_str!("fixtures/c-double-emissions.txt"), false),
            (
                include_str!("fixtures/c-double-emissions-fractional.txt"),
                true,
            ),
        ] {
            for line in fixture.lines() {
                let fields: Vec<_> = line.split_whitespace().collect();
                let scenario: usize = fields[0].parse().unwrap();
                let index: usize = fields[1].parse().unwrap();
                let options = LiblrhsmmRsEmissionSettingsF64 {
                    temperature: 0.75,
                    pruning: if fractional || scenario == 0 { 1 } else { 2 },
                    radius: if fractional {
                        [0.25, 0.5, 1.75, 2.5][scenario]
                    } else {
                        1.0
                    },
                    slope: 0.34,
                };
                let mut table = null_mut();
                assert_eq!(
                    liblrhsmm_rs_prepared_model_emission_log_probabilities_f64(
                        prepared, obs, seg, &options, &mut table
                    ),
                    0
                );
                reference(
                    log_values_f64(table)[index],
                    fields[2].parse::<f64>().unwrap(),
                    2e-14,
                );
                comparisons += 1;
                assert_eq!(liblrhsmm_rs_log_emissions_release_f64(&mut table), 0);
            }
        }
        assert_eq!(comparisons, 216);
        for options in [
            LiblrhsmmRsEmissionSettingsF64 {
                pruning: 99,
                ..settings
            },
            LiblrhsmmRsEmissionSettingsF64 {
                temperature: f64::NAN,
                ..settings
            },
            LiblrhsmmRsEmissionSettingsF64 {
                temperature: -0.25,
                ..settings
            },
            LiblrhsmmRsEmissionSettingsF64 {
                pruning: 1,
                radius: f64::INFINITY,
                ..settings
            },
            LiblrhsmmRsEmissionSettingsF64 {
                pruning: 1,
                radius: -0.25,
                ..settings
            },
            LiblrhsmmRsEmissionSettingsF64 {
                pruning: 2,
                slope: f64::NAN,
                ..settings
            },
            LiblrhsmmRsEmissionSettingsF64 {
                pruning: 2,
                slope: -0.25,
                ..settings
            },
        ] {
            let mut log = std::ptr::without_provenance_mut::<LiblrhsmmRsLogEmissionsF64>(0x1000);
            let mut density =
                std::ptr::without_provenance_mut::<LiblrhsmmRsEmissionDensitiesF64>(0x2000);
            assert_eq!(
                liblrhsmm_rs_model_emission_log_probabilities_f64(
                    model, obs, seg, &options, &mut log
                ),
                3
            );
            assert_eq!(
                liblrhsmm_rs_prepared_model_emission_log_probabilities_f64(
                    prepared, obs, seg, &options, &mut log
                ),
                3
            );
            assert_eq!(
                liblrhsmm_rs_model_emission_probabilities_f64(
                    model,
                    obs,
                    seg,
                    &options,
                    &mut density
                ),
                3
            );
            assert_eq!(
                liblrhsmm_rs_prepared_model_emission_probabilities_f64(
                    prepared,
                    obs,
                    seg,
                    &options,
                    &mut density
                ),
                3
            );
            assert_eq!(log.addr(), 0x1000);
            assert_eq!(density.addr(), 0x2000);
        }
        let mut unchanged = std::ptr::without_provenance_mut::<LiblrhsmmRsLogEmissionsF64>(0x1000);
        for (m, o, s, config) in [
            (
                null(),
                obs.cast_const(),
                seg.cast_const(),
                &settings as *const _,
            ),
            (
                model.cast_const(),
                null(),
                seg.cast_const(),
                &settings as *const _,
            ),
            (
                model.cast_const(),
                obs.cast_const(),
                null(),
                &settings as *const _,
            ),
            (
                model.cast_const(),
                obs.cast_const(),
                seg.cast_const(),
                null(),
            ),
        ] {
            assert_eq!(
                liblrhsmm_rs_model_emission_log_probabilities_f64(m, o, s, config, &mut unchanged),
                1
            );
            assert_eq!(unchanged.addr(), 0x1000);
        }
        assert_eq!(
            liblrhsmm_rs_model_emission_log_probabilities_f64(
                model,
                obs,
                seg,
                &settings,
                null_mut()
            ),
            1
        );
        let labels = [-1, 1, 2];
        assert_eq!(
            liblrhsmm_rs_segmentation_set_output_states_f64(seg, 0, 0, labels.as_ptr(), 3),
            0
        );
        assert_eq!(
            liblrhsmm_rs_prepared_model_emission_log_probabilities_f64(
                prepared,
                obs,
                seg,
                &settings,
                &mut unchanged
            ),
            3
        );
        assert_eq!(unchanged.addr(), 0x1000);
        let labels = [0, 1, 2];
        assert_eq!(
            liblrhsmm_rs_segmentation_set_output_states_f64(seg, 0, 0, labels.as_ptr(), 3),
            0
        );
        assert_eq!(liblrhsmm_rs_model_release_f64(&mut model), 0);
        let (mut log, mut density) = (null_mut(), null_mut());
        assert_eq!(
            liblrhsmm_rs_prepared_model_emission_log_probabilities_f64(
                prepared, obs, seg, &settings, &mut log
            ),
            0
        );
        assert_eq!(
            liblrhsmm_rs_prepared_model_emission_probabilities_f64(
                prepared,
                obs,
                seg,
                &settings,
                &mut density
            ),
            0
        );
        assert_eq!(liblrhsmm_rs_prepared_model_release_f64(&mut prepared), 0);
        assert_eq!(liblrhsmm_rs_observation_release_f64(&mut obs), 0);
        assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut seg), 0);
        let expected = native
            .emission_log_probabilities(
                &observation,
                &segmentation,
                1.0,
                EmissionPruningOptions::None,
            )
            .unwrap();
        assert_eq!(
            log_values_f64(log)
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            expected
                .values()
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            density_values_f64(density)
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            expected
                .probabilities()
                .values()
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>()
        );
        assert_eq!(liblrhsmm_rs_log_emissions_release_f64(&mut log), 0);
        assert_eq!(liblrhsmm_rs_emission_densities_release_f64(&mut density), 0);
    }
}
