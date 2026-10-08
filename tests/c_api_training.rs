#![cfg(feature = "c-api")]
use liblrhsmm_rs::c_api::*;
use liblrhsmm_rs::expectation::{GeometricPosteriorExpectation, HsmmPosteriorExpectation};
use liblrhsmm_rs::occupancy::StateOccupancyTable;
use liblrhsmm_rs::statistics::ModelSufficientStatistics;
use liblrhsmm_rs::{
    DurationOccupancies, DurationPosterior, MixtureOccupancies, TransitionOccupancies,
    TransitionPosterior,
};
use std::ptr::{null, null_mut};
#[path = "common/emissions.rs"]
#[allow(dead_code)]
mod common;
#[path = "common/c_api_emission_inputs.rs"]
mod inputs;

fn native_bits_f32(statistics: &ModelSufficientStatistics<f32>) -> Vec<u64> {
    let mut result = vec![
        statistics.streams.len() as u64,
        statistics.durations.len() as u64,
    ];
    for stream in &statistics.streams {
        result.push(stream.mixtures.len() as u64);
        for gaussian in &stream.mixtures {
            result.extend([
                gaussian.component_occupancies.len() as u64,
                gaussian.dimensions as u64,
                gaussian.state_occupancy.to_bits() as u64,
            ]);
            for values in [
                &gaussian.weighted_sums,
                &gaussian.weighted_squared_sums,
                &gaussian.component_occupancies,
            ] {
                result.extend(values.iter().map(|v| v.to_bits() as u64));
            }
        }
    }
    for duration in &statistics.durations {
        result.extend([
            duration.value_sum.to_bits() as u64,
            duration.squared_sum.to_bits() as u64,
            duration.occupancy.to_bits() as u64,
        ]);
    }
    result
}
unsafe fn c_bits_f32(statistics: *const LiblrhsmmRsModelStatisticsF32) -> Vec<u64> {
    unsafe {
        let mut layout = LiblrhsmmRsModelLayout {
            streams: 0,
            durations: 0,
        };
        assert_eq!(
            liblrhsmm_rs_model_statistics_layout_f32(statistics, &mut layout),
            0
        );
        let mut result = vec![layout.streams as u64, layout.durations as u64];
        for index in 0..layout.streams {
            let mut stream = null_mut();
            assert_eq!(
                liblrhsmm_rs_model_statistics_get_stream_f32(statistics, index, &mut stream),
                0
            );
            let mut count = 0;
            assert_eq!(
                liblrhsmm_rs_stream_statistics_count_f32(stream, &mut count),
                0
            );
            result.push(count as u64);
            for state in 0..count {
                let mut gaussian = null_mut();
                assert_eq!(
                    liblrhsmm_rs_stream_statistics_get_emission_f32(stream, state, &mut gaussian),
                    0
                );
                let mut shape = LiblrhsmmRsGaussianStatisticsLayoutF32 {
                    mixtures: 0,
                    dimensions: 0,
                    state_occupancy: 0.0,
                };
                assert_eq!(
                    liblrhsmm_rs_gaussian_statistics_layout_f32(gaussian, &mut shape),
                    0
                );
                result.extend([
                    shape.mixtures as u64,
                    shape.dimensions as u64,
                    shape.state_occupancy.to_bits() as u64,
                ]);
                for field in 0..3 {
                    let mut values = vec![
                        0.0;
                        if field == 2 {
                            shape.mixtures
                        } else {
                            shape.mixtures * shape.dimensions
                        }
                    ];
                    assert_eq!(
                        liblrhsmm_rs_gaussian_statistics_copy_field_f32(
                            gaussian,
                            field,
                            0,
                            values.as_mut_ptr(),
                            values.len()
                        ),
                        0
                    );
                    result.extend(values.iter().map(|v| v.to_bits() as u64));
                }
                assert_eq!(
                    liblrhsmm_rs_gaussian_statistics_release_f32(&mut gaussian),
                    0
                );
            }
            assert_eq!(liblrhsmm_rs_stream_statistics_release_f32(&mut stream), 0);
        }
        for index in 0..layout.durations {
            let mut duration = null_mut();
            assert_eq!(
                liblrhsmm_rs_model_statistics_get_duration_f32(statistics, index, &mut duration),
                0
            );
            let mut values = LiblrhsmmRsDurationStatisticsValuesF32 {
                value_sum: 0.0,
                squared_sum: 0.0,
                occupancy: 0.0,
            };
            assert_eq!(
                liblrhsmm_rs_duration_statistics_get_f32(duration, &mut values),
                0
            );
            result.extend([
                values.value_sum.to_bits() as u64,
                values.squared_sum.to_bits() as u64,
                values.occupancy.to_bits() as u64,
            ]);
            assert_eq!(
                liblrhsmm_rs_duration_statistics_release_f32(&mut duration),
                0
            );
        }
        result
    }
}

#[test]
fn f32_collectors_preserve_complete_external_posteriors_and_atomic_failures() {
    let (native, observation, segmentation) = common::sample_with_precision::<f32>();
    let mut expected = ModelSufficientStatistics::from_model(&native).unwrap();
    let mut duration_values = vec![None; 36];
    duration_values[3] = Some(DurationPosterior {
        minimum: 2,
        probabilities: vec![2.0, 3.0],
    });
    let duration = DurationOccupancies::from_values(12, 3, duration_values).unwrap();
    let mut transition_values = vec![None; 36];
    transition_values[3] = Some(TransitionPosterior {
        stay: 1.5,
        advance: 2.5,
    });
    let transition = TransitionOccupancies::from_values(12, 3, transition_values).unwrap();
    let mut state_values = vec![0.0; 36];
    state_values[3] = 4.0;
    let state = StateOccupancyTable::from_values(12, 3, state_values.clone()).unwrap();
    let mixtures: Vec<_> = [vec![2.0, 3.0], vec![5.0]]
        .into_iter()
        .map(|weights| {
            let mut values = vec![None; 36];
            values[3] = Some(weights);
            MixtureOccupancies::from_values(12, 3, values).unwrap()
        })
        .collect();
    unsafe {
        let (mut model, mut obs, mut seg) = inputs::inputs_f32();
        let mut statistics = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_statistics_from_model_f32(model, &mut statistics),
            0
        );
        let weights = [2.0, 3.0];
        let empty = LiblrhsmmRsPosteriorInput {
            present: 0,
            minimum: 0,
            values: null(),
            count: 0,
        };
        let mut posterior_inputs = [empty; 36];
        posterior_inputs[3] = LiblrhsmmRsPosteriorInput {
            present: 1,
            minimum: 2,
            values: weights.as_ptr(),
            count: 2,
        };
        let (mut durations, mut transitions, mut states) = (null_mut(), null_mut(), null_mut());
        assert_eq!(
            liblrhsmm_rs_duration_occupancies_create(
                12,
                3,
                posterior_inputs.as_ptr(),
                36,
                &mut durations
            ),
            0
        );
        let mut arrivals = [LiblrhsmmRsTransitionPosterior {
            present: 0,
            stay: 0.0,
            advance: 0.0,
        }; 36];
        arrivals[3] = LiblrhsmmRsTransitionPosterior {
            present: 1,
            stay: 1.5,
            advance: 2.5,
        };
        assert_eq!(
            liblrhsmm_rs_transition_occupancies_create(
                12,
                3,
                arrivals.as_ptr(),
                36,
                &mut transitions
            ),
            0
        );
        assert_eq!(
            liblrhsmm_rs_state_occupancies_create_f32(
                12,
                3,
                state_values.as_ptr(),
                36,
                &mut states
            ),
            0
        );
        let mut components = [null_mut(); 2];
        assert_eq!(
            liblrhsmm_rs_mixture_occupancies_create(
                12,
                3,
                posterior_inputs.as_ptr(),
                36,
                &mut components[0]
            ),
            0
        );
        let second_weights = [5.0];
        posterior_inputs[3].values = second_weights.as_ptr();
        posterior_inputs[3].count = 1;
        assert_eq!(
            liblrhsmm_rs_mixture_occupancies_create(
                12,
                3,
                posterior_inputs.as_ptr(),
                36,
                &mut components[1]
            ),
            0
        );
        let borrowed = components.map(|value| value.cast_const());
        for _ in 0..2 {
            expected
                .collect_hsmm(
                    &native,
                    &observation,
                    &segmentation,
                    HsmmPosteriorExpectation {
                        durations: &duration,
                        states: &state,
                        mixtures: &mixtures,
                        state_radius: 1.0,
                    },
                )
                .unwrap();
            assert_eq!(
                liblrhsmm_rs_model_statistics_collect_hsmm_f32(
                    statistics,
                    model,
                    obs,
                    seg,
                    durations,
                    states,
                    borrowed.as_ptr(),
                    2,
                    1.0
                ),
                0
            );
            assert_eq!(c_bits_f32(statistics), native_bits_f32(&expected));
        }
        let before = c_bits_f32(statistics);
        assert_eq!(
            liblrhsmm_rs_model_statistics_collect_hsmm_f32(
                statistics,
                model,
                obs,
                seg,
                durations,
                states,
                borrowed.as_ptr(),
                1,
                1.0
            ),
            3
        );
        assert_eq!(
            liblrhsmm_rs_model_statistics_collect_hsmm_f32(
                statistics,
                model,
                obs,
                seg,
                durations,
                states,
                null(),
                2,
                1.0
            ),
            1
        );
        let invalid = [borrowed[0], null()];
        assert_eq!(
            liblrhsmm_rs_model_statistics_collect_hsmm_f32(
                statistics,
                model,
                obs,
                seg,
                durations,
                states,
                invalid.as_ptr(),
                2,
                1.0
            ),
            1
        );
        assert_eq!(
            liblrhsmm_rs_model_statistics_collect_hsmm_f32(
                statistics,
                model,
                obs,
                seg,
                durations,
                states,
                borrowed.as_ptr(),
                2,
                f32::NAN
            ),
            3
        );
        assert_eq!(c_bits_f32(statistics), before);
        assert_eq!(liblrhsmm_rs_model_statistics_clear_f32(statistics), 0);
        expected.clear();
        for _ in 0..2 {
            expected
                .collect_geometric(
                    &native,
                    &observation,
                    &segmentation,
                    GeometricPosteriorExpectation {
                        transitions: &transition,
                        states: &state,
                        mixtures: &mixtures,
                        pruning_slope: 1.0,
                    },
                )
                .unwrap();
            assert_eq!(
                liblrhsmm_rs_model_statistics_collect_geometric_f32(
                    statistics,
                    model,
                    obs,
                    seg,
                    transitions,
                    states,
                    borrowed.as_ptr(),
                    2,
                    1.0
                ),
                0
            );
            assert_eq!(c_bits_f32(statistics), native_bits_f32(&expected));
        }
        let before = c_bits_f32(statistics);
        assert_eq!(
            liblrhsmm_rs_model_statistics_collect_geometric_f32(
                statistics,
                model,
                obs,
                seg,
                transitions,
                states,
                borrowed.as_ptr(),
                1,
                1.0
            ),
            3
        );
        assert_eq!(
            liblrhsmm_rs_model_statistics_collect_geometric_f32(
                statistics,
                model,
                obs,
                seg,
                transitions,
                states,
                invalid.as_ptr(),
                2,
                1.0
            ),
            1
        );
        assert_eq!(
            liblrhsmm_rs_model_statistics_collect_geometric_f32(
                statistics,
                model,
                obs,
                seg,
                transitions,
                states,
                borrowed.as_ptr(),
                2,
                f32::NAN
            ),
            3
        );
        assert_eq!(c_bits_f32(statistics), before);
        for component in &mut components {
            assert_eq!(liblrhsmm_rs_mixture_occupancies_release(component), 0);
        }
        assert_eq!(liblrhsmm_rs_duration_occupancies_release(&mut durations), 0);
        assert_eq!(
            liblrhsmm_rs_transition_occupancies_release(&mut transitions),
            0
        );
        assert_eq!(liblrhsmm_rs_state_occupancies_release_f32(&mut states), 0);
        assert_eq!(liblrhsmm_rs_model_release_f32(&mut model), 0);
        assert_eq!(liblrhsmm_rs_observation_release_f32(&mut obs), 0);
        assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut seg), 0);
        assert_eq!(c_bits_f32(statistics), before);
        assert_eq!(
            liblrhsmm_rs_model_statistics_release_f32(&mut statistics),
            0
        );
    }
}

#[test]
fn f32_initialization_estimation_update_codec_and_inference_are_connected() {
    let (mut native, observation, segmentation) = common::sample_with_precision::<f32>();
    unsafe {
        let (mut model, mut obs, mut seg) = inputs::inputs_f32();
        let specifications = [
            LiblrhsmmRsStreamSpecification {
                emissions: 3,
                components: 1,
                dimensions: 2,
            },
            LiblrhsmmRsStreamSpecification {
                emissions: 3,
                components: 1,
                dimensions: 1,
            },
        ];
        let layouts = specifications.map(|v| liblrhsmm_rs::model::StreamLayout {
            emissions: v.emissions,
            components: v.components,
            dimensions: v.dimensions,
        });
        let flat = liblrhsmm_rs::model::ModelParameters::<f32>::new(&layouts, 3).unwrap();
        let mut expected = ModelSufficientStatistics::from_model(&flat).unwrap();
        let (mut initial, mut initial_statistics) = (null_mut(), null_mut());
        assert_eq!(
            liblrhsmm_rs_model_new_f32(specifications.as_ptr(), 2, 3, &mut initial),
            0
        );
        assert_eq!(
            liblrhsmm_rs_model_statistics_from_model_f32(initial, &mut initial_statistics),
            0
        );
        for _ in 0..2 {
            expected
                .collect_initialization(&flat, &observation, &segmentation)
                .unwrap();
            assert_eq!(
                liblrhsmm_rs_model_statistics_collect_initialization_f32(
                    initial_statistics,
                    initial,
                    obs,
                    seg
                ),
                0
            );
            assert_eq!(c_bits_f32(initial_statistics), native_bits_f32(&expected));
        }
        let before = c_bits_f32(initial_statistics);
        assert_eq!(
            liblrhsmm_rs_model_statistics_collect_initialization_f32(
                initial_statistics,
                initial,
                obs,
                null()
            ),
            1
        );
        assert_eq!(c_bits_f32(initial_statistics), before);
        assert_eq!(
            liblrhsmm_rs_model_statistics_release_f32(&mut initial_statistics),
            0
        );
        assert_eq!(liblrhsmm_rs_model_release_f32(&mut initial), 0);
        let mut statistics = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_statistics_from_model_f32(model, &mut statistics),
            0
        );
        let mut expected = ModelSufficientStatistics::from_model(&native).unwrap();
        for temperature in [0.5, 0.75, 1.75, f32::from_bits(1.0_f32.to_bits() + 1)] {
            let hsmm = liblrhsmm_rs::hsmm::HsmmSettings {
                temperature,
                duration_weight: 0.8,
                state_radius: 0.5,
                duration_extra: 1,
                duration_extra_factor: 1.25,
            };
            let geometric = liblrhsmm_rs::geometric::GeometricSettings {
                temperature,
                pruning_slope: 0.34,
            };
            for mode in 0..2 {
                assert_eq!(liblrhsmm_rs_model_statistics_clear_f32(statistics), 0);
                expected.clear();
                let hsmm_options = LiblrhsmmRsHsmmSettingsF32::from(hsmm);
                let geometric_options = LiblrhsmmRsGeometricSettingsF32::from(geometric);
                let total = if mode == 0 {
                    expected
                        .estimate_hsmm(&native, &observation, &segmentation, hsmm)
                        .unwrap()
                } else {
                    expected
                        .estimate_geometric(&native, &observation, &segmentation, geometric)
                        .unwrap()
                };
                let mut actual = 99.0_f32;
                if mode == 0 {
                    assert_eq!(
                        liblrhsmm_rs_model_statistics_estimate_hsmm_f32(
                            statistics,
                            model,
                            obs,
                            seg,
                            &hsmm_options,
                            &mut actual
                        ),
                        0
                    );
                } else {
                    assert_eq!(
                        liblrhsmm_rs_model_statistics_estimate_geometric_f32(
                            statistics,
                            model,
                            obs,
                            seg,
                            &geometric_options,
                            &mut actual
                        ),
                        0
                    );
                }
                assert_eq!(actual.to_bits(), total.to_bits());
                assert_eq!(c_bits_f32(statistics), native_bits_f32(&expected));
                let before = c_bits_f32(statistics);
                assert_eq!(
                    liblrhsmm_rs_model_statistics_estimate_hsmm_f32(
                        statistics,
                        model,
                        obs,
                        seg,
                        &hsmm_options,
                        null_mut()
                    ),
                    1
                );
                assert_eq!(
                    liblrhsmm_rs_model_statistics_estimate_geometric_f32(
                        statistics,
                        model,
                        obs,
                        seg,
                        &geometric_options,
                        null_mut()
                    ),
                    1
                );
                let invalid_hsmm = LiblrhsmmRsHsmmSettingsF32 {
                    temperature: f32::NAN,
                    ..hsmm_options
                };
                let invalid_geometric = LiblrhsmmRsGeometricSettingsF32 {
                    temperature: f32::NAN,
                    ..geometric_options
                };
                actual = 99.0;
                assert_eq!(
                    liblrhsmm_rs_model_statistics_estimate_hsmm_f32(
                        statistics,
                        model,
                        obs,
                        seg,
                        &invalid_hsmm,
                        &mut actual
                    ),
                    3
                );
                assert_eq!(
                    liblrhsmm_rs_model_statistics_estimate_geometric_f32(
                        statistics,
                        model,
                        obs,
                        seg,
                        &invalid_geometric,
                        &mut actual
                    ),
                    3
                );
                assert_eq!(actual, 99.0);
                assert_eq!(c_bits_f32(statistics), before);
            }
        }
        let mut update = LiblrhsmmRsUpdateOptionsF32 {
            duration_mode: 0,
            duration_variance_floor: 0.0,
            emission_variance_floor: 0.0,
        };
        assert_eq!(liblrhsmm_rs_update_options_default_f32(&mut update), 0);
        native
            .update(&expected, liblrhsmm_rs::update::UpdateSettings::default())
            .unwrap();
        assert_eq!(liblrhsmm_rs_model_update_f32(model, statistics, &update), 0);
        #[cfg(feature = "serialization")]
        {
            let mut bytes = null_mut();
            assert_eq!(
                liblrhsmm_rs_model_write_bytes_f32(
                    model,
                    LIBLRHSMM_RS_ENCODING_WITH_VARIANCE_FLOORS,
                    &mut bytes
                ),
                0
            );
            let mut length = 0;
            assert_eq!(liblrhsmm_rs_bytes_length(bytes, &mut length), 0);
            let mut actual = vec![0; length];
            assert_eq!(
                liblrhsmm_rs_bytes_copy(bytes, 0, actual.as_mut_ptr(), length),
                0
            );
            let mut expected_bytes = Vec::new();
            native.write_to(&mut expected_bytes).unwrap();
            assert_eq!(actual, expected_bytes);
            let mut limits = LiblrhsmmRsDecodeLimits {
                max_array_entries: 0,
            };
            assert_eq!(liblrhsmm_rs_decode_limits_default(&mut limits), 0);
            let mut decoded = null_mut();
            assert_eq!(
                liblrhsmm_rs_model_read_bytes_f32(actual.as_ptr(), length, &limits, &mut decoded),
                0
            );
            assert_eq!(liblrhsmm_rs_bytes_release(&mut bytes), 0);
            assert_eq!(liblrhsmm_rs_model_release_f32(&mut model), 0);
            model = decoded;
            native =
                liblrhsmm_rs::model::ModelParameters::<f32>::read_from(actual.as_slice()).unwrap();
        }
        let options = LiblrhsmmRsEmissionSettingsF32 {
            temperature: 1.0,
            pruning: 0,
            radius: 0.0,
            slope: 0.0,
        };
        let mut log = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_emission_log_probabilities_f32(model, obs, seg, &options, &mut log),
            0
        );
        let mut hsmm = LiblrhsmmRsHsmmSettingsF32 {
            temperature: 0.0,
            duration_weight: 0.0,
            state_radius: 0.0,
            duration_extra: 0,
            duration_extra_factor: 0.0,
        };
        assert_eq!(liblrhsmm_rs_hsmm_settings_default_f32(&mut hsmm), 0);
        let mut path = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_viterbi_hsmm_f32(model, seg, log, &hsmm, &mut path),
            0
        );
        let native_log = native
            .emission_log_probabilities(
                &observation,
                &segmentation,
                1.0,
                liblrhsmm_rs::emission::EmissionPruningOptions::None,
            )
            .unwrap();
        let native_path = native
            .viterbi_hsmm(
                &segmentation,
                &native_log,
                liblrhsmm_rs::hsmm::HsmmSettings::default(),
            )
            .unwrap();
        let mut probability = 0.0_f32;
        assert_eq!(
            liblrhsmm_rs_hsmm_path_log_probability_f32(path, &mut probability),
            0
        );
        assert_eq!(probability.to_bits(), native_path.log_probability.to_bits());
        let mut occurrences =
            vec![LiblrhsmmRsStateOccurrence { state: 0, end: 0 }; native_path.occurrences.len()];
        assert_eq!(
            liblrhsmm_rs_hsmm_path_copy_occurrences_f32(
                path,
                0,
                occurrences.as_mut_ptr(),
                occurrences.len()
            ),
            0
        );
        assert_eq!(
            occurrences
                .iter()
                .map(|v| (v.state, v.end))
                .collect::<Vec<_>>(),
            native_path
                .occurrences
                .iter()
                .map(|v| (v.state, v.end))
                .collect::<Vec<_>>()
        );
        assert_eq!(liblrhsmm_rs_hsmm_path_release_f32(&mut path), 0);
        assert_eq!(liblrhsmm_rs_log_emissions_release_f32(&mut log), 0);
        assert_eq!(
            liblrhsmm_rs_model_statistics_release_f32(&mut statistics),
            0
        );
        assert_eq!(liblrhsmm_rs_model_release_f32(&mut model), 0);
        assert_eq!(liblrhsmm_rs_observation_release_f32(&mut obs), 0);
        assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut seg), 0);
    }
}

fn native_bits_f64(statistics: &ModelSufficientStatistics<f64>) -> Vec<u64> {
    let mut result = vec![
        statistics.streams.len() as u64,
        statistics.durations.len() as u64,
    ];
    for stream in &statistics.streams {
        result.push(stream.mixtures.len() as u64);
        for gaussian in &stream.mixtures {
            result.extend([
                gaussian.component_occupancies.len() as u64,
                gaussian.dimensions as u64,
                gaussian.state_occupancy.to_bits(),
            ]);
            for values in [
                &gaussian.weighted_sums,
                &gaussian.weighted_squared_sums,
                &gaussian.component_occupancies,
            ] {
                result.extend(values.iter().map(|v| v.to_bits()));
            }
        }
    }
    for duration in &statistics.durations {
        result.extend([
            duration.value_sum.to_bits(),
            duration.squared_sum.to_bits(),
            duration.occupancy.to_bits(),
        ]);
    }
    result
}
unsafe fn c_bits_f64(statistics: *const LiblrhsmmRsModelStatisticsF64) -> Vec<u64> {
    unsafe {
        let mut layout = LiblrhsmmRsModelLayout {
            streams: 0,
            durations: 0,
        };
        assert_eq!(
            liblrhsmm_rs_model_statistics_layout_f64(statistics, &mut layout),
            0
        );
        let mut result = vec![layout.streams as u64, layout.durations as u64];
        for index in 0..layout.streams {
            let mut stream = null_mut();
            assert_eq!(
                liblrhsmm_rs_model_statistics_get_stream_f64(statistics, index, &mut stream),
                0
            );
            let mut count = 0;
            assert_eq!(
                liblrhsmm_rs_stream_statistics_count_f64(stream, &mut count),
                0
            );
            result.push(count as u64);
            for state in 0..count {
                let mut gaussian = null_mut();
                assert_eq!(
                    liblrhsmm_rs_stream_statistics_get_emission_f64(stream, state, &mut gaussian),
                    0
                );
                let mut shape = LiblrhsmmRsGaussianStatisticsLayoutF64 {
                    mixtures: 0,
                    dimensions: 0,
                    state_occupancy: 0.0,
                };
                assert_eq!(
                    liblrhsmm_rs_gaussian_statistics_layout_f64(gaussian, &mut shape),
                    0
                );
                result.extend([
                    shape.mixtures as u64,
                    shape.dimensions as u64,
                    shape.state_occupancy.to_bits(),
                ]);
                for field in 0..3 {
                    let mut values = vec![
                        0.0;
                        if field == 2 {
                            shape.mixtures
                        } else {
                            shape.mixtures * shape.dimensions
                        }
                    ];
                    assert_eq!(
                        liblrhsmm_rs_gaussian_statistics_copy_field_f64(
                            gaussian,
                            field,
                            0,
                            values.as_mut_ptr(),
                            values.len()
                        ),
                        0
                    );
                    result.extend(values.iter().map(|v| v.to_bits()));
                }
                assert_eq!(
                    liblrhsmm_rs_gaussian_statistics_release_f64(&mut gaussian),
                    0
                );
            }
            assert_eq!(liblrhsmm_rs_stream_statistics_release_f64(&mut stream), 0);
        }
        for index in 0..layout.durations {
            let mut duration = null_mut();
            assert_eq!(
                liblrhsmm_rs_model_statistics_get_duration_f64(statistics, index, &mut duration),
                0
            );
            let mut values = LiblrhsmmRsDurationStatisticsValuesF64 {
                value_sum: 0.0,
                squared_sum: 0.0,
                occupancy: 0.0,
            };
            assert_eq!(
                liblrhsmm_rs_duration_statistics_get_f64(duration, &mut values),
                0
            );
            result.extend([
                values.value_sum.to_bits(),
                values.squared_sum.to_bits(),
                values.occupancy.to_bits(),
            ]);
            assert_eq!(
                liblrhsmm_rs_duration_statistics_release_f64(&mut duration),
                0
            );
        }
        result
    }
}

#[test]
fn f64_collectors_preserve_complete_external_posteriors_and_atomic_failures() {
    let (native, observation, segmentation) = common::sample_with_precision::<f64>();
    let mut expected = ModelSufficientStatistics::from_model(&native).unwrap();
    let mut duration_values = vec![None; 36];
    duration_values[3] = Some(DurationPosterior {
        minimum: 2,
        probabilities: vec![2.0, 3.0],
    });
    let duration = DurationOccupancies::from_values(12, 3, duration_values).unwrap();
    let mut transition_values = vec![None; 36];
    transition_values[3] = Some(TransitionPosterior {
        stay: 1.5,
        advance: 2.5,
    });
    let transition = TransitionOccupancies::from_values(12, 3, transition_values).unwrap();
    let mut state_values = vec![0.0; 36];
    state_values[3] = 4.0;
    let state = StateOccupancyTable::from_values(12, 3, state_values.clone()).unwrap();
    let mixtures: Vec<_> = [vec![2.0, 3.0], vec![5.0]]
        .into_iter()
        .map(|weights| {
            let mut values = vec![None; 36];
            values[3] = Some(weights);
            MixtureOccupancies::from_values(12, 3, values).unwrap()
        })
        .collect();
    unsafe {
        let (mut model, mut obs, mut seg) = inputs::inputs_f64();
        let mut statistics = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_statistics_from_model_f64(model, &mut statistics),
            0
        );
        let weights = [2.0, 3.0];
        let empty = LiblrhsmmRsPosteriorInput {
            present: 0,
            minimum: 0,
            values: null(),
            count: 0,
        };
        let mut posterior_inputs = [empty; 36];
        posterior_inputs[3] = LiblrhsmmRsPosteriorInput {
            present: 1,
            minimum: 2,
            values: weights.as_ptr(),
            count: 2,
        };
        let (mut durations, mut transitions, mut states) = (null_mut(), null_mut(), null_mut());
        assert_eq!(
            liblrhsmm_rs_duration_occupancies_create(
                12,
                3,
                posterior_inputs.as_ptr(),
                36,
                &mut durations
            ),
            0
        );
        let mut arrivals = [LiblrhsmmRsTransitionPosterior {
            present: 0,
            stay: 0.0,
            advance: 0.0,
        }; 36];
        arrivals[3] = LiblrhsmmRsTransitionPosterior {
            present: 1,
            stay: 1.5,
            advance: 2.5,
        };
        assert_eq!(
            liblrhsmm_rs_transition_occupancies_create(
                12,
                3,
                arrivals.as_ptr(),
                36,
                &mut transitions
            ),
            0
        );
        assert_eq!(
            liblrhsmm_rs_state_occupancies_create_f64(
                12,
                3,
                state_values.as_ptr(),
                36,
                &mut states
            ),
            0
        );
        let mut components = [null_mut(); 2];
        assert_eq!(
            liblrhsmm_rs_mixture_occupancies_create(
                12,
                3,
                posterior_inputs.as_ptr(),
                36,
                &mut components[0]
            ),
            0
        );
        let second_weights = [5.0];
        posterior_inputs[3].values = second_weights.as_ptr();
        posterior_inputs[3].count = 1;
        assert_eq!(
            liblrhsmm_rs_mixture_occupancies_create(
                12,
                3,
                posterior_inputs.as_ptr(),
                36,
                &mut components[1]
            ),
            0
        );
        let borrowed = components.map(|value| value.cast_const());
        for _ in 0..2 {
            expected
                .collect_hsmm(
                    &native,
                    &observation,
                    &segmentation,
                    HsmmPosteriorExpectation {
                        durations: &duration,
                        states: &state,
                        mixtures: &mixtures,
                        state_radius: 1.0,
                    },
                )
                .unwrap();
            assert_eq!(
                liblrhsmm_rs_model_statistics_collect_hsmm_f64(
                    statistics,
                    model,
                    obs,
                    seg,
                    durations,
                    states,
                    borrowed.as_ptr(),
                    2,
                    1.0
                ),
                0
            );
            assert_eq!(c_bits_f64(statistics), native_bits_f64(&expected));
        }
        let before = c_bits_f64(statistics);
        assert_eq!(
            liblrhsmm_rs_model_statistics_collect_hsmm_f64(
                statistics,
                model,
                obs,
                seg,
                durations,
                states,
                borrowed.as_ptr(),
                1,
                1.0
            ),
            3
        );
        assert_eq!(
            liblrhsmm_rs_model_statistics_collect_hsmm_f64(
                statistics,
                model,
                obs,
                seg,
                durations,
                states,
                null(),
                2,
                1.0
            ),
            1
        );
        let invalid = [borrowed[0], null()];
        assert_eq!(
            liblrhsmm_rs_model_statistics_collect_hsmm_f64(
                statistics,
                model,
                obs,
                seg,
                durations,
                states,
                invalid.as_ptr(),
                2,
                1.0
            ),
            1
        );
        assert_eq!(
            liblrhsmm_rs_model_statistics_collect_hsmm_f64(
                statistics,
                model,
                obs,
                seg,
                durations,
                states,
                borrowed.as_ptr(),
                2,
                f64::NAN
            ),
            3
        );
        assert_eq!(c_bits_f64(statistics), before);
        assert_eq!(liblrhsmm_rs_model_statistics_clear_f64(statistics), 0);
        expected.clear();
        for _ in 0..2 {
            expected
                .collect_geometric(
                    &native,
                    &observation,
                    &segmentation,
                    GeometricPosteriorExpectation {
                        transitions: &transition,
                        states: &state,
                        mixtures: &mixtures,
                        pruning_slope: 1.0,
                    },
                )
                .unwrap();
            assert_eq!(
                liblrhsmm_rs_model_statistics_collect_geometric_f64(
                    statistics,
                    model,
                    obs,
                    seg,
                    transitions,
                    states,
                    borrowed.as_ptr(),
                    2,
                    1.0
                ),
                0
            );
            assert_eq!(c_bits_f64(statistics), native_bits_f64(&expected));
        }
        let before = c_bits_f64(statistics);
        assert_eq!(
            liblrhsmm_rs_model_statistics_collect_geometric_f64(
                statistics,
                model,
                obs,
                seg,
                transitions,
                states,
                borrowed.as_ptr(),
                1,
                1.0
            ),
            3
        );
        assert_eq!(
            liblrhsmm_rs_model_statistics_collect_geometric_f64(
                statistics,
                model,
                obs,
                seg,
                transitions,
                states,
                invalid.as_ptr(),
                2,
                1.0
            ),
            1
        );
        assert_eq!(
            liblrhsmm_rs_model_statistics_collect_geometric_f64(
                statistics,
                model,
                obs,
                seg,
                transitions,
                states,
                borrowed.as_ptr(),
                2,
                f64::NAN
            ),
            3
        );
        assert_eq!(c_bits_f64(statistics), before);
        for component in &mut components {
            assert_eq!(liblrhsmm_rs_mixture_occupancies_release(component), 0);
        }
        assert_eq!(liblrhsmm_rs_duration_occupancies_release(&mut durations), 0);
        assert_eq!(
            liblrhsmm_rs_transition_occupancies_release(&mut transitions),
            0
        );
        assert_eq!(liblrhsmm_rs_state_occupancies_release_f64(&mut states), 0);
        assert_eq!(liblrhsmm_rs_model_release_f64(&mut model), 0);
        assert_eq!(liblrhsmm_rs_observation_release_f64(&mut obs), 0);
        assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut seg), 0);
        assert_eq!(c_bits_f64(statistics), before);
        assert_eq!(
            liblrhsmm_rs_model_statistics_release_f64(&mut statistics),
            0
        );
    }
}

#[test]
fn f64_initialization_estimation_update_codec_and_inference_are_connected() {
    let (mut native, observation, segmentation) = common::sample_with_precision::<f64>();
    unsafe {
        let (mut model, mut obs, mut seg) = inputs::inputs_f64();
        let specifications = [
            LiblrhsmmRsStreamSpecification {
                emissions: 3,
                components: 1,
                dimensions: 2,
            },
            LiblrhsmmRsStreamSpecification {
                emissions: 3,
                components: 1,
                dimensions: 1,
            },
        ];
        let layouts = specifications.map(|v| liblrhsmm_rs::model::StreamLayout {
            emissions: v.emissions,
            components: v.components,
            dimensions: v.dimensions,
        });
        let flat = liblrhsmm_rs::model::ModelParameters::<f64>::new(&layouts, 3).unwrap();
        let mut expected = ModelSufficientStatistics::from_model(&flat).unwrap();
        let (mut initial, mut initial_statistics) = (null_mut(), null_mut());
        assert_eq!(
            liblrhsmm_rs_model_new_f64(specifications.as_ptr(), 2, 3, &mut initial),
            0
        );
        assert_eq!(
            liblrhsmm_rs_model_statistics_from_model_f64(initial, &mut initial_statistics),
            0
        );
        for _ in 0..2 {
            expected
                .collect_initialization(&flat, &observation, &segmentation)
                .unwrap();
            assert_eq!(
                liblrhsmm_rs_model_statistics_collect_initialization_f64(
                    initial_statistics,
                    initial,
                    obs,
                    seg
                ),
                0
            );
            assert_eq!(c_bits_f64(initial_statistics), native_bits_f64(&expected));
        }
        let before = c_bits_f64(initial_statistics);
        assert_eq!(
            liblrhsmm_rs_model_statistics_collect_initialization_f64(
                initial_statistics,
                initial,
                obs,
                null()
            ),
            1
        );
        assert_eq!(c_bits_f64(initial_statistics), before);
        assert_eq!(
            liblrhsmm_rs_model_statistics_release_f64(&mut initial_statistics),
            0
        );
        assert_eq!(liblrhsmm_rs_model_release_f64(&mut initial), 0);
        let mut statistics = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_statistics_from_model_f64(model, &mut statistics),
            0
        );
        let mut expected = ModelSufficientStatistics::from_model(&native).unwrap();
        for temperature in [0.5, 0.75, 1.75, f64::from_bits(1.0_f64.to_bits() + 1)] {
            let hsmm = liblrhsmm_rs::hsmm::HsmmSettings {
                temperature,
                duration_weight: 0.8,
                state_radius: 0.5,
                duration_extra: 1,
                duration_extra_factor: 1.25,
            };
            let geometric = liblrhsmm_rs::geometric::GeometricSettings {
                temperature,
                pruning_slope: 0.34,
            };
            for mode in 0..2 {
                assert_eq!(liblrhsmm_rs_model_statistics_clear_f64(statistics), 0);
                expected.clear();
                let hsmm_options = LiblrhsmmRsHsmmSettingsF64::from(hsmm);
                let geometric_options = LiblrhsmmRsGeometricSettingsF64::from(geometric);
                let total = if mode == 0 {
                    expected
                        .estimate_hsmm(&native, &observation, &segmentation, hsmm)
                        .unwrap()
                } else {
                    expected
                        .estimate_geometric(&native, &observation, &segmentation, geometric)
                        .unwrap()
                };
                let mut actual = 99.0_f64;
                if mode == 0 {
                    assert_eq!(
                        liblrhsmm_rs_model_statistics_estimate_hsmm_f64(
                            statistics,
                            model,
                            obs,
                            seg,
                            &hsmm_options,
                            &mut actual
                        ),
                        0
                    );
                } else {
                    assert_eq!(
                        liblrhsmm_rs_model_statistics_estimate_geometric_f64(
                            statistics,
                            model,
                            obs,
                            seg,
                            &geometric_options,
                            &mut actual
                        ),
                        0
                    );
                }
                assert_eq!(actual.to_bits(), total.to_bits());
                assert_eq!(c_bits_f64(statistics), native_bits_f64(&expected));
                let before = c_bits_f64(statistics);
                assert_eq!(
                    liblrhsmm_rs_model_statistics_estimate_hsmm_f64(
                        statistics,
                        model,
                        obs,
                        seg,
                        &hsmm_options,
                        null_mut()
                    ),
                    1
                );
                assert_eq!(
                    liblrhsmm_rs_model_statistics_estimate_geometric_f64(
                        statistics,
                        model,
                        obs,
                        seg,
                        &geometric_options,
                        null_mut()
                    ),
                    1
                );
                let invalid_hsmm = LiblrhsmmRsHsmmSettingsF64 {
                    temperature: f64::NAN,
                    ..hsmm_options
                };
                let invalid_geometric = LiblrhsmmRsGeometricSettingsF64 {
                    temperature: f64::NAN,
                    ..geometric_options
                };
                actual = 99.0;
                assert_eq!(
                    liblrhsmm_rs_model_statistics_estimate_hsmm_f64(
                        statistics,
                        model,
                        obs,
                        seg,
                        &invalid_hsmm,
                        &mut actual
                    ),
                    3
                );
                assert_eq!(
                    liblrhsmm_rs_model_statistics_estimate_geometric_f64(
                        statistics,
                        model,
                        obs,
                        seg,
                        &invalid_geometric,
                        &mut actual
                    ),
                    3
                );
                assert_eq!(actual, 99.0);
                assert_eq!(c_bits_f64(statistics), before);
            }
        }
        let mut update = LiblrhsmmRsUpdateOptionsF64 {
            duration_mode: 0,
            duration_variance_floor: 0.0,
            emission_variance_floor: 0.0,
        };
        assert_eq!(liblrhsmm_rs_update_options_default_f64(&mut update), 0);
        native
            .update(&expected, liblrhsmm_rs::update::UpdateSettings::default())
            .unwrap();
        assert_eq!(liblrhsmm_rs_model_update_f64(model, statistics, &update), 0);
        #[cfg(feature = "serialization")]
        {
            let mut bytes = null_mut();
            assert_eq!(
                liblrhsmm_rs_model_write_bytes_f64(
                    model,
                    LIBLRHSMM_RS_ENCODING_WITH_VARIANCE_FLOORS,
                    &mut bytes
                ),
                0
            );
            let mut length = 0;
            assert_eq!(liblrhsmm_rs_bytes_length(bytes, &mut length), 0);
            let mut actual = vec![0; length];
            assert_eq!(
                liblrhsmm_rs_bytes_copy(bytes, 0, actual.as_mut_ptr(), length),
                0
            );
            let mut expected_bytes = Vec::new();
            native.write_to(&mut expected_bytes).unwrap();
            assert_eq!(actual, expected_bytes);
            let mut limits = LiblrhsmmRsDecodeLimits {
                max_array_entries: 0,
            };
            assert_eq!(liblrhsmm_rs_decode_limits_default(&mut limits), 0);
            let mut decoded = null_mut();
            assert_eq!(
                liblrhsmm_rs_model_read_bytes_f64(actual.as_ptr(), length, &limits, &mut decoded),
                0
            );
            assert_eq!(liblrhsmm_rs_bytes_release(&mut bytes), 0);
            assert_eq!(liblrhsmm_rs_model_release_f64(&mut model), 0);
            model = decoded;
            native =
                liblrhsmm_rs::model::ModelParameters::<f64>::read_from(actual.as_slice()).unwrap();
        }
        let options = LiblrhsmmRsEmissionSettingsF64 {
            temperature: 1.0,
            pruning: 0,
            radius: 0.0,
            slope: 0.0,
        };
        let mut log = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_emission_log_probabilities_f64(model, obs, seg, &options, &mut log),
            0
        );
        let mut hsmm = LiblrhsmmRsHsmmSettingsF64 {
            temperature: 0.0,
            duration_weight: 0.0,
            state_radius: 0.0,
            duration_extra: 0,
            duration_extra_factor: 0.0,
        };
        assert_eq!(liblrhsmm_rs_hsmm_settings_default_f64(&mut hsmm), 0);
        let mut path = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_viterbi_hsmm_f64(model, seg, log, &hsmm, &mut path),
            0
        );
        let native_log = native
            .emission_log_probabilities(
                &observation,
                &segmentation,
                1.0,
                liblrhsmm_rs::emission::EmissionPruningOptions::None,
            )
            .unwrap();
        let native_path = native
            .viterbi_hsmm(
                &segmentation,
                &native_log,
                liblrhsmm_rs::hsmm::HsmmSettings::default(),
            )
            .unwrap();
        let mut probability = 0.0_f64;
        assert_eq!(
            liblrhsmm_rs_hsmm_path_log_probability_f64(path, &mut probability),
            0
        );
        assert_eq!(probability.to_bits(), native_path.log_probability.to_bits());
        let mut occurrences =
            vec![LiblrhsmmRsStateOccurrence { state: 0, end: 0 }; native_path.occurrences.len()];
        assert_eq!(
            liblrhsmm_rs_hsmm_path_copy_occurrences_f64(
                path,
                0,
                occurrences.as_mut_ptr(),
                occurrences.len()
            ),
            0
        );
        assert_eq!(
            occurrences
                .iter()
                .map(|v| (v.state, v.end))
                .collect::<Vec<_>>(),
            native_path
                .occurrences
                .iter()
                .map(|v| (v.state, v.end))
                .collect::<Vec<_>>()
        );
        assert_eq!(liblrhsmm_rs_hsmm_path_release_f64(&mut path), 0);
        assert_eq!(liblrhsmm_rs_log_emissions_release_f64(&mut log), 0);
        assert_eq!(
            liblrhsmm_rs_model_statistics_release_f64(&mut statistics),
            0
        );
        assert_eq!(liblrhsmm_rs_model_release_f64(&mut model), 0);
        assert_eq!(liblrhsmm_rs_observation_release_f64(&mut obs), 0);
        assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut seg), 0);
    }
}
