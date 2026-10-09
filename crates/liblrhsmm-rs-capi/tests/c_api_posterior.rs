use liblrhsmm_rs_capi::*;
use std::ptr::{null, null_mut};

#[test]
fn sparse_imports_preserve_absent_empty_and_all_weights() {
    unsafe {
        let mut weights = [0.0_f32, -0.0, f32::from_bits(1), 2.0];
        let input = [
            LiblrhsmmRsPosteriorInput {
                present: 0,
                minimum: usize::MAX,
                values: std::ptr::without_provenance(1),
                count: usize::MAX,
            },
            LiblrhsmmRsPosteriorInput {
                present: 1,
                minimum: usize::MAX,
                values: null(),
                count: 0,
            },
            LiblrhsmmRsPosteriorInput {
                present: 1,
                minimum: 3,
                values: weights.as_ptr(),
                count: 4,
            },
        ];
        let (mut durations, mut mixtures) = (null_mut(), null_mut());
        assert_eq!(
            liblrhsmm_rs_duration_occupancies_create(1, 3, input.as_ptr(), 3, &mut durations),
            0
        );
        assert_eq!(
            liblrhsmm_rs_mixture_occupancies_create(1, 3, input.as_ptr(), 3, &mut mixtures),
            0
        );
        let (mut duration_clone, mut mixture_clone) = (null_mut(), null_mut());
        assert_eq!(
            liblrhsmm_rs_duration_occupancies_clone(durations, &mut duration_clone),
            0
        );
        assert_eq!(
            liblrhsmm_rs_mixture_occupancies_clone(mixtures, &mut mixture_clone),
            0
        );
        for state in 0..3 {
            let mut metadata = LiblrhsmmRsPosteriorSlice {
                present: 99,
                minimum: 99,
                values: 99,
            };
            assert_eq!(
                liblrhsmm_rs_duration_occupancies_get_slice(durations, 0, state, &mut metadata),
                0
            );
            assert_eq!(metadata.present, if state == 0 { 0 } else { 1 });
            assert_eq!(metadata.values, if state == 2 { 4 } else { 0 });
            assert_eq!(
                metadata.minimum,
                if state == 0 {
                    0
                } else if state == 1 {
                    usize::MAX
                } else {
                    3
                }
            );
            assert_eq!(
                liblrhsmm_rs_mixture_occupancies_get_slice(mixtures, 0, state, &mut metadata),
                0
            );
            assert_eq!(metadata.present, if state == 0 { 0 } else { 1 });
            assert_eq!(metadata.minimum, 0);
            assert_eq!(
                liblrhsmm_rs_duration_occupancies_copy_values(
                    durations,
                    0,
                    state,
                    metadata.values,
                    null_mut(),
                    0
                ),
                0
            );
            assert_eq!(
                liblrhsmm_rs_mixture_occupancies_copy_values(
                    mixtures,
                    0,
                    state,
                    metadata.values,
                    null_mut(),
                    0
                ),
                0
            );
        }
        let mut copied = [99.0; 4];
        assert_eq!(
            liblrhsmm_rs_duration_occupancies_copy_values(
                durations,
                0,
                2,
                0,
                copied.as_mut_ptr(),
                4
            ),
            0
        );
        assert_eq!(copied.map(f32::to_bits), weights.map(f32::to_bits));
        assert_eq!(
            liblrhsmm_rs_mixture_occupancies_copy_values(mixtures, 0, 2, 0, copied.as_mut_ptr(), 4),
            0
        );
        assert_eq!(copied.map(f32::to_bits), weights.map(f32::to_bits));
        let original = copied;
        assert_eq!(
            liblrhsmm_rs_duration_occupancies_copy_values(
                durations,
                0,
                2,
                1,
                copied.as_mut_ptr(),
                4
            ),
            2
        );
        assert_eq!(copied.map(f32::to_bits), original.map(f32::to_bits));
        assert_eq!(
            liblrhsmm_rs_mixture_occupancies_copy_values(
                mixtures,
                0,
                2,
                usize::MAX,
                copied.as_mut_ptr(),
                1
            ),
            2
        );
        let mut probability = 99.0;
        for (duration, expected) in [
            (2, 0.0),
            (3, 0.0),
            (4, -0.0),
            (5, f32::from_bits(1)),
            (6, 2.0),
            (7, 0.0),
        ] {
            assert_eq!(
                liblrhsmm_rs_duration_occupancies_probability(
                    durations,
                    0,
                    2,
                    duration,
                    &mut probability
                ),
                0
            );
            assert_eq!(probability.to_bits(), expected.to_bits());
        }
        assert_eq!(
            liblrhsmm_rs_duration_occupancies_probability(
                durations,
                0,
                0,
                usize::MAX,
                &mut probability
            ),
            0
        );
        assert_eq!(probability, 0.0);
        let mut layout = LiblrhsmmRsInferenceTableLayout {
            frames: 99,
            states: 99,
            values: 99,
        };
        assert_eq!(
            liblrhsmm_rs_duration_occupancies_layout(durations, &mut layout),
            0
        );
        assert_eq!((layout.frames, layout.states, layout.values), (1, 3, 3));
        assert_eq!(
            liblrhsmm_rs_mixture_occupancies_layout(mixtures, &mut layout),
            0
        );
        assert_eq!((layout.frames, layout.states, layout.values), (1, 3, 3));
        weights[3] = f32::NAN;
        assert_eq!(
            liblrhsmm_rs_duration_occupancies_replace(durations, 1, 3, input.as_ptr(), 3),
            3
        );
        assert_eq!(
            liblrhsmm_rs_mixture_occupancies_replace(mixtures, 1, 3, input.as_ptr(), 3),
            3
        );
        assert_eq!(
            liblrhsmm_rs_duration_occupancies_copy_values(
                durations,
                0,
                2,
                0,
                copied.as_mut_ptr(),
                4
            ),
            0
        );
        assert_eq!(copied.map(f32::to_bits), original.map(f32::to_bits));
        assert_eq!(
            liblrhsmm_rs_mixture_occupancies_copy_values(mixtures, 0, 2, 0, copied.as_mut_ptr(), 4),
            0
        );
        assert_eq!(copied.map(f32::to_bits), original.map(f32::to_bits));
        let mut invalid = input;
        invalid[2].present = 2;
        assert_eq!(
            liblrhsmm_rs_duration_occupancies_replace(durations, 1, 3, invalid.as_ptr(), 3),
            3
        );
        assert_eq!(
            liblrhsmm_rs_mixture_occupancies_replace(mixtures, 1, 3, invalid.as_ptr(), 3),
            3
        );
        invalid[2].present = 1;
        invalid[2].values = null();
        assert_eq!(
            liblrhsmm_rs_duration_occupancies_replace(durations, 1, 3, invalid.as_ptr(), 3),
            1
        );
        assert_eq!(
            liblrhsmm_rs_mixture_occupancies_replace(mixtures, 1, 3, invalid.as_ptr(), 3),
            1
        );
        assert_eq!(
            liblrhsmm_rs_duration_occupancies_replace(durations, 3, 1, input.as_ptr(), 3),
            3
        );
        weights[3] = 2.0;
        assert_eq!(weights[3].to_bits(), original[3].to_bits());
        assert_eq!(
            liblrhsmm_rs_duration_occupancies_replace(durations, 3, 1, input.as_ptr(), 3),
            0
        );
        assert_eq!(
            liblrhsmm_rs_mixture_occupancies_replace(mixtures, 3, 1, input.as_ptr(), 3),
            0
        );
        assert_eq!(liblrhsmm_rs_duration_occupancies_release(&mut durations), 0);
        assert_eq!(liblrhsmm_rs_mixture_occupancies_release(&mut mixtures), 0);
        assert_eq!(
            liblrhsmm_rs_duration_occupancies_layout(duration_clone, &mut layout),
            0
        );
        assert_eq!((layout.frames, layout.states), (1, 3));
        assert_eq!(
            liblrhsmm_rs_mixture_occupancies_copy_values(
                mixture_clone,
                0,
                2,
                0,
                copied.as_mut_ptr(),
                4
            ),
            0
        );
        assert_eq!(copied.map(f32::to_bits), original.map(f32::to_bits));
        assert_eq!(
            liblrhsmm_rs_duration_occupancies_release(&mut duration_clone),
            0
        );
        assert_eq!(
            liblrhsmm_rs_mixture_occupancies_release(&mut mixture_clone),
            0
        );
        let transition_input = [
            LiblrhsmmRsTransitionPosterior {
                present: 1,
                stay: -0.0,
                advance: f32::from_bits(1),
            },
            LiblrhsmmRsTransitionPosterior {
                present: 0,
                stay: f32::NAN,
                advance: f32::INFINITY,
            },
            LiblrhsmmRsTransitionPosterior {
                present: 1,
                stay: 2.0,
                advance: 3.0,
            },
        ];
        let mut transitions = null_mut();
        assert_eq!(
            liblrhsmm_rs_transition_occupancies_create(
                1,
                3,
                transition_input.as_ptr(),
                3,
                &mut transitions
            ),
            0
        );
        let mut transition_clone = null_mut();
        assert_eq!(
            liblrhsmm_rs_transition_occupancies_clone(transitions, &mut transition_clone),
            0
        );
        let mut read = LiblrhsmmRsTransitionPosterior {
            present: 99,
            stay: 99.0,
            advance: 99.0,
        };
        assert_eq!(
            liblrhsmm_rs_transition_occupancies_get(transitions, 0, 0, &mut read),
            0
        );
        assert_eq!(read.stay.to_bits(), (-0.0_f32).to_bits());
        assert_eq!(read.advance.to_bits(), 1);
        assert_eq!(
            liblrhsmm_rs_transition_occupancies_get(transitions, 0, 1, &mut read),
            0
        );
        assert_eq!(read.present, 0);
        let mut invalid = transition_input;
        invalid[2].advance = -1.0;
        assert_eq!(
            liblrhsmm_rs_transition_occupancies_replace(transitions, 1, 3, invalid.as_ptr(), 3),
            3
        );
        assert_eq!(
            liblrhsmm_rs_transition_occupancies_get(transitions, 0, 2, &mut read),
            0
        );
        assert_eq!(read.advance, 3.0);
        assert_eq!(
            liblrhsmm_rs_transition_occupancies_replace(
                transitions,
                3,
                1,
                transition_input.as_ptr(),
                3
            ),
            0
        );
        assert_eq!(
            liblrhsmm_rs_transition_occupancies_layout(transitions, &mut layout),
            0
        );
        assert_eq!((layout.frames, layout.states), (3, 1));
        assert_eq!(
            liblrhsmm_rs_transition_occupancies_release(&mut transitions),
            0
        );
        assert_eq!(
            liblrhsmm_rs_transition_occupancies_get(transition_clone, 0, 2, &mut read),
            0
        );
        assert_eq!(read.stay, 2.0);
        assert_eq!(
            liblrhsmm_rs_transition_occupancies_release(&mut transition_clone),
            0
        );
        assert_eq!(
            liblrhsmm_rs_transition_occupancies_release(&mut transition_clone),
            0
        );
        assert_eq!(liblrhsmm_rs_duration_occupancies_release(null_mut()), 1);
        assert_eq!(liblrhsmm_rs_mixture_occupancies_release(null_mut()), 1);
        assert_eq!(liblrhsmm_rs_transition_occupancies_release(null_mut()), 1);
    }
}

#[path = "common/emissions.rs"]
#[allow(dead_code)]
mod common;
#[path = "common/c_api_emission_inputs.rs"]
mod inputs;

unsafe fn state_values_f32(owner: *const LiblrhsmmRsStateOccupanciesF32) -> Vec<f32> {
    unsafe {
        let mut layout = LiblrhsmmRsInferenceTableLayout {
            frames: 0,
            states: 0,
            values: 0,
        };
        assert_eq!(
            liblrhsmm_rs_state_occupancies_layout_f32(owner, &mut layout),
            0
        );
        let mut values = vec![0.0; layout.values];
        assert_eq!(
            liblrhsmm_rs_state_occupancies_copy_values_f32(
                owner,
                0,
                values.as_mut_ptr(),
                values.len()
            ),
            0
        );
        values
    }
}

#[test]
fn f32_state_imports_full_bits_and_atomic_replacement() {
    unsafe {
        let mut values = [0.0_f32, -0.0, f32::from_bits(1), 2.0];
        let mut owner = null_mut();
        assert_eq!(
            liblrhsmm_rs_state_occupancies_create_f32(2, 2, values.as_ptr(), 4, &mut owner),
            0
        );
        let mut clone = null_mut();
        assert_eq!(
            liblrhsmm_rs_state_occupancies_clone_f32(owner, &mut clone),
            0
        );
        let mut value = 99.0_f32;
        assert_eq!(
            liblrhsmm_rs_state_occupancies_get_f32(owner, 0, 1, &mut value),
            0
        );
        assert_eq!(value.to_bits(), (-0.0_f32).to_bits());
        let mut row = [99.0_f32; 2];
        assert_eq!(
            liblrhsmm_rs_state_occupancies_copy_frame_f32(owner, 1, row.as_mut_ptr(), 2),
            0
        );
        assert_eq!(row.map(f32::to_bits), [1, 2.0_f32.to_bits()]);
        let original = state_values_f32(owner);
        values[3] = f32::NAN;
        assert_eq!(
            liblrhsmm_rs_state_occupancies_replace_f32(owner, 1, 4, values.as_ptr(), 4),
            3
        );
        assert_eq!(
            state_values_f32(owner)
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            original.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
        );
        values[3] = 2.0;
        assert_eq!(
            liblrhsmm_rs_state_occupancies_replace_f32(owner, 1, 4, values.as_ptr(), 4),
            0
        );
        assert_eq!(
            liblrhsmm_rs_state_occupancies_copy_values_f32(owner, 4, null_mut(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_state_occupancies_copy_values_f32(owner, usize::MAX, row.as_mut_ptr(), 1),
            2
        );
        assert_eq!(
            liblrhsmm_rs_state_occupancies_get_f32(owner, 1, 0, &mut value),
            2
        );
        assert_eq!(liblrhsmm_rs_state_occupancies_release_f32(&mut owner), 0);
        assert_eq!(
            state_values_f32(clone)
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            original.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
        );
        assert_eq!(liblrhsmm_rs_state_occupancies_release_f32(&mut clone), 0);
        assert_eq!(liblrhsmm_rs_state_occupancies_release_f32(&mut clone), 0);
    }
}

#[test]
fn f32_all_native_posterior_calculations_and_cache_paths() {
    use liblrhsmm_rs::emission::EmissionPruningOptions;
    use liblrhsmm_rs::mixture_occupancy::MixtureOccupancySettings;
    let (native, observation, segmentation) = common::sample_with_precision::<f32>();
    let settings = liblrhsmm_rs::hsmm::HsmmSettings::<f32> {
        temperature: 0.75,
        ..Default::default()
    };
    let emissions = native
        .emission_log_probabilities(
            &observation,
            &segmentation,
            0.75,
            EmissionPruningOptions::None,
        )
        .unwrap();
    let forward = native
        .forward_hsmm(&segmentation, &emissions, settings)
        .unwrap();
    let backward = native
        .backward_hsmm(&segmentation, &emissions, settings)
        .unwrap();
    let total = forward.end.total_through_state(&backward.end, 0).unwrap();
    let expected = native
        .duration_occupancies(
            &segmentation,
            &emissions,
            &forward,
            &backward,
            total,
            settings,
        )
        .unwrap();
    let expected_state = native
        .hsmm_state_occupancies(&segmentation, &emissions, &expected, settings)
        .unwrap();
    unsafe {
        let (mut model, mut obs, mut seg) = inputs::inputs_f32();
        let mut graph = null_mut();
        assert_eq!(
            liblrhsmm_rs_segmentation_build_transitions_f32(seg, &mut graph),
            0
        );
        let emission_settings = LiblrhsmmRsEmissionSettingsF32 {
            temperature: 0.75,
            pruning: 0,
            radius: 0.0,
            slope: 0.0,
        };
        let mut log = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_emission_log_probabilities_f32(
                model,
                obs,
                seg,
                &emission_settings,
                &mut log
            ),
            0
        );
        let options = LiblrhsmmRsHsmmSettingsF32::from(settings);
        let (mut a, mut b) = (null_mut(), null_mut());
        assert_eq!(
            liblrhsmm_rs_model_forward_hsmm_f32(model, seg, log, &options, &mut a),
            0
        );
        assert_eq!(
            liblrhsmm_rs_model_backward_hsmm_f32(model, seg, log, &options, &mut b),
            0
        );
        let (mut duration, mut cached) = (null_mut(), null_mut());
        assert_eq!(
            liblrhsmm_rs_model_duration_occupancies_f32(
                model,
                seg,
                log,
                a,
                b,
                total,
                &options,
                &mut duration
            ),
            0
        );
        assert_eq!(
            liblrhsmm_rs_model_duration_occupancies_with_graph_f32(
                model,
                graph,
                log,
                a,
                b,
                total,
                &options,
                &mut cached
            ),
            0
        );
        for owner in [duration, cached] {
            for frame in 0..12 {
                for state in 0..3 {
                    let mut layout = LiblrhsmmRsPosteriorSlice {
                        present: 99,
                        minimum: 99,
                        values: 99,
                    };
                    assert_eq!(
                        liblrhsmm_rs_duration_occupancies_get_slice(
                            owner,
                            frame,
                            state,
                            &mut layout
                        ),
                        0
                    );
                    let expected = expected.get(frame, state);
                    assert_eq!(layout.present, u32::from(expected.is_some()));
                    if let Some(expected) = expected {
                        assert_eq!(
                            (layout.minimum, layout.values),
                            (expected.minimum, expected.probabilities.len())
                        );
                        let mut values = vec![0.0; layout.values];
                        assert_eq!(
                            liblrhsmm_rs_duration_occupancies_copy_values(
                                owner,
                                frame,
                                state,
                                0,
                                values.as_mut_ptr(),
                                values.len()
                            ),
                            0
                        );
                        assert_eq!(
                            values.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                            expected
                                .probabilities
                                .iter()
                                .map(|v| v.to_bits())
                                .collect::<Vec<_>>()
                        );
                    }
                }
            }
        }
        let mut state = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_hsmm_state_occupancies_f32(
                model, seg, log, duration, &options, &mut state
            ),
            0
        );
        assert_eq!(
            state_values_f32(state)
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            expected_state
                .values()
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>()
        );
        for mode in [
            MixtureOccupancySettings::Hsmm { state_radius: 0.5 },
            MixtureOccupancySettings::Geometric {
                pruning_slope: 0.34,
            },
        ] {
            for stream in 0..2 {
                let expected = native
                    .mixture_occupancies(
                        &observation,
                        &segmentation,
                        &expected_state,
                        stream,
                        0.75,
                        mode,
                    )
                    .unwrap();
                let mut mixtures = null_mut();
                match mode {
                    MixtureOccupancySettings::Hsmm { state_radius } => assert_eq!(
                        liblrhsmm_rs_model_mixture_occupancies_hsmm_f32(
                            model,
                            obs,
                            seg,
                            state,
                            stream,
                            0.75,
                            state_radius,
                            &mut mixtures
                        ),
                        0
                    ),
                    MixtureOccupancySettings::Geometric { pruning_slope } => assert_eq!(
                        liblrhsmm_rs_model_mixture_occupancies_geometric_f32(
                            model,
                            obs,
                            seg,
                            state,
                            stream,
                            0.75,
                            pruning_slope,
                            &mut mixtures
                        ),
                        0
                    ),
                }
                for frame in 0..12 {
                    for label in 0..3 {
                        let mut layout = LiblrhsmmRsPosteriorSlice {
                            present: 99,
                            minimum: 99,
                            values: 99,
                        };
                        assert_eq!(
                            liblrhsmm_rs_mixture_occupancies_get_slice(
                                mixtures,
                                frame,
                                label,
                                &mut layout
                            ),
                            0
                        );
                        let expected = expected.get(frame, label);
                        assert_eq!(layout.present, u32::from(expected.is_some()));
                        if let Some(expected) = expected {
                            let mut values = vec![0.0; layout.values];
                            assert_eq!(
                                liblrhsmm_rs_mixture_occupancies_copy_values(
                                    mixtures,
                                    frame,
                                    label,
                                    0,
                                    values.as_mut_ptr(),
                                    values.len()
                                ),
                                0
                            );
                            assert_eq!(
                                values.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                                expected.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
                            );
                        }
                    }
                }
                assert_eq!(liblrhsmm_rs_mixture_occupancies_release(&mut mixtures), 0);
            }
        }
        let geo = liblrhsmm_rs::geometric::GeometricSettings {
            temperature: 0.75,
            pruning_slope: 10.0,
        };
        let config = LiblrhsmmRsGeometricSettingsF32::from(geo);
        let expected_a = native
            .forward_geometric(&segmentation, &emissions, geo)
            .unwrap();
        let expected_b = native
            .backward_geometric(&segmentation, &emissions, geo)
            .unwrap();
        let geo_total = expected_a.total_log_probability();
        let expected_state = expected_a
            .geometric_occupancies(&expected_b, geo_total, 10.0)
            .unwrap();
        let expected_transition = native
            .geometric_transition_occupancies(
                &segmentation,
                &emissions,
                &expected_a,
                &expected_b,
                geo_total,
                geo,
            )
            .unwrap();
        let (mut geo_a, mut geo_b, mut geo_state, mut transition) =
            (null_mut(), null_mut(), null_mut(), null_mut());
        assert_eq!(
            liblrhsmm_rs_model_forward_geometric_f32(model, seg, log, &config, &mut geo_a),
            0
        );
        assert_eq!(
            liblrhsmm_rs_model_backward_geometric_f32(model, seg, log, &config, &mut geo_b),
            0
        );
        assert_eq!(
            liblrhsmm_rs_state_scores_geometric_occupancies_f32(
                geo_a,
                geo_b,
                geo_total,
                10.0,
                &mut geo_state
            ),
            0
        );
        assert_eq!(
            state_values_f32(geo_state)
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            expected_state
                .values()
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            liblrhsmm_rs_model_geometric_transition_occupancies_f32(
                model,
                seg,
                log,
                geo_a,
                geo_b,
                geo_total,
                &config,
                &mut transition
            ),
            0
        );
        for frame in 0..12 {
            for label in 0..3 {
                let mut value = LiblrhsmmRsTransitionPosterior {
                    present: 99,
                    stay: 99.0,
                    advance: 99.0,
                };
                assert_eq!(
                    liblrhsmm_rs_transition_occupancies_get(transition, frame, label, &mut value),
                    0
                );
                let expected = expected_transition.get(frame, label);
                assert_eq!(value.present, u32::from(expected.is_some()));
                if let Some(expected) = expected {
                    assert_eq!(value.stay.to_bits(), expected.stay.to_bits());
                    assert_eq!(value.advance.to_bits(), expected.advance.to_bits());
                }
            }
        }
        let mut invalid =
            std::ptr::without_provenance_mut::<LiblrhsmmRsDurationOccupancies>(0x1000);
        assert_eq!(
            liblrhsmm_rs_model_duration_occupancies_f32(
                model,
                seg,
                log,
                a,
                b,
                f32::NAN,
                &options,
                &mut invalid
            ),
            3
        );
        assert_eq!(invalid.addr(), 0x1000);
        assert_eq!(
            liblrhsmm_rs_model_duration_occupancies_with_graph_f32(
                model,
                graph,
                log,
                a,
                b,
                total,
                null(),
                &mut invalid
            ),
            1
        );
        assert_eq!(invalid.addr(), 0x1000);
        assert_eq!(liblrhsmm_rs_model_release_f32(&mut model), 0);
        assert_eq!(liblrhsmm_rs_observation_release_f32(&mut obs), 0);
        assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut seg), 0);
        assert_eq!(liblrhsmm_rs_transition_graph_release_f32(&mut graph), 0);
        assert_eq!(liblrhsmm_rs_log_emissions_release_f32(&mut log), 0);
        assert_eq!(
            state_values_f32(geo_state)
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            expected_state
                .values()
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>()
        );
        assert_eq!(liblrhsmm_rs_state_occupancies_release_f32(&mut state), 0);
        assert_eq!(
            liblrhsmm_rs_state_occupancies_release_f32(&mut geo_state),
            0
        );
        assert_eq!(
            liblrhsmm_rs_transition_occupancies_release(&mut transition),
            0
        );
        assert_eq!(liblrhsmm_rs_duration_occupancies_release(&mut duration), 0);
        assert_eq!(liblrhsmm_rs_duration_occupancies_release(&mut cached), 0);
        assert_eq!(liblrhsmm_rs_hsmm_scores_release_f32(&mut a), 0);
        assert_eq!(liblrhsmm_rs_hsmm_scores_release_f32(&mut b), 0);
        assert_eq!(liblrhsmm_rs_state_scores_release_f32(&mut geo_a), 0);
        assert_eq!(liblrhsmm_rs_state_scores_release_f32(&mut geo_b), 0);
    }
}

unsafe fn state_values_f64(owner: *const LiblrhsmmRsStateOccupanciesF64) -> Vec<f64> {
    unsafe {
        let mut layout = LiblrhsmmRsInferenceTableLayout {
            frames: 0,
            states: 0,
            values: 0,
        };
        assert_eq!(
            liblrhsmm_rs_state_occupancies_layout_f64(owner, &mut layout),
            0
        );
        let mut values = vec![0.0; layout.values];
        assert_eq!(
            liblrhsmm_rs_state_occupancies_copy_values_f64(
                owner,
                0,
                values.as_mut_ptr(),
                values.len()
            ),
            0
        );
        values
    }
}

#[test]
fn f64_state_imports_full_bits_and_atomic_replacement() {
    unsafe {
        let mut values = [0.0_f64, -0.0, f64::from_bits(1), 2.0];
        let mut owner = null_mut();
        assert_eq!(
            liblrhsmm_rs_state_occupancies_create_f64(2, 2, values.as_ptr(), 4, &mut owner),
            0
        );
        let mut clone = null_mut();
        assert_eq!(
            liblrhsmm_rs_state_occupancies_clone_f64(owner, &mut clone),
            0
        );
        let mut value = 99.0_f64;
        assert_eq!(
            liblrhsmm_rs_state_occupancies_get_f64(owner, 0, 1, &mut value),
            0
        );
        assert_eq!(value.to_bits(), (-0.0_f64).to_bits());
        let mut row = [99.0_f64; 2];
        assert_eq!(
            liblrhsmm_rs_state_occupancies_copy_frame_f64(owner, 1, row.as_mut_ptr(), 2),
            0
        );
        assert_eq!(row.map(f64::to_bits), [1, 2.0_f64.to_bits()]);
        let original = state_values_f64(owner);
        values[3] = f64::NAN;
        assert_eq!(
            liblrhsmm_rs_state_occupancies_replace_f64(owner, 1, 4, values.as_ptr(), 4),
            3
        );
        assert_eq!(
            state_values_f64(owner)
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            original.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
        );
        values[3] = 2.0;
        assert_eq!(
            liblrhsmm_rs_state_occupancies_replace_f64(owner, 1, 4, values.as_ptr(), 4),
            0
        );
        assert_eq!(
            liblrhsmm_rs_state_occupancies_copy_values_f64(owner, 4, null_mut(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_state_occupancies_copy_values_f64(owner, usize::MAX, row.as_mut_ptr(), 1),
            2
        );
        assert_eq!(
            liblrhsmm_rs_state_occupancies_get_f64(owner, 1, 0, &mut value),
            2
        );
        assert_eq!(liblrhsmm_rs_state_occupancies_release_f64(&mut owner), 0);
        assert_eq!(
            state_values_f64(clone)
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            original.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
        );
        assert_eq!(liblrhsmm_rs_state_occupancies_release_f64(&mut clone), 0);
        assert_eq!(liblrhsmm_rs_state_occupancies_release_f64(&mut clone), 0);
    }
}

#[test]
fn f64_all_native_posterior_calculations_and_cache_paths() {
    use liblrhsmm_rs::emission::EmissionPruningOptions;
    use liblrhsmm_rs::mixture_occupancy::MixtureOccupancySettings;
    let (native, observation, segmentation) = common::sample_with_precision::<f64>();
    let settings = liblrhsmm_rs::hsmm::HsmmSettings::<f64> {
        temperature: 0.75,
        ..Default::default()
    };
    let emissions = native
        .emission_log_probabilities(
            &observation,
            &segmentation,
            0.75,
            EmissionPruningOptions::None,
        )
        .unwrap();
    let forward = native
        .forward_hsmm(&segmentation, &emissions, settings)
        .unwrap();
    let backward = native
        .backward_hsmm(&segmentation, &emissions, settings)
        .unwrap();
    let total = forward.end.total_through_state(&backward.end, 0).unwrap();
    let expected = native
        .duration_occupancies(
            &segmentation,
            &emissions,
            &forward,
            &backward,
            total,
            settings,
        )
        .unwrap();
    let expected_state = native
        .hsmm_state_occupancies(&segmentation, &emissions, &expected, settings)
        .unwrap();
    unsafe {
        let (mut model, mut obs, mut seg) = inputs::inputs_f64();
        let mut graph = null_mut();
        assert_eq!(
            liblrhsmm_rs_segmentation_build_transitions_f64(seg, &mut graph),
            0
        );
        let emission_settings = LiblrhsmmRsEmissionSettingsF64 {
            temperature: 0.75,
            pruning: 0,
            radius: 0.0,
            slope: 0.0,
        };
        let mut log = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_emission_log_probabilities_f64(
                model,
                obs,
                seg,
                &emission_settings,
                &mut log
            ),
            0
        );
        let options = LiblrhsmmRsHsmmSettingsF64::from(settings);
        let (mut a, mut b) = (null_mut(), null_mut());
        assert_eq!(
            liblrhsmm_rs_model_forward_hsmm_f64(model, seg, log, &options, &mut a),
            0
        );
        assert_eq!(
            liblrhsmm_rs_model_backward_hsmm_f64(model, seg, log, &options, &mut b),
            0
        );
        let (mut duration, mut cached) = (null_mut(), null_mut());
        assert_eq!(
            liblrhsmm_rs_model_duration_occupancies_f64(
                model,
                seg,
                log,
                a,
                b,
                total,
                &options,
                &mut duration
            ),
            0
        );
        assert_eq!(
            liblrhsmm_rs_model_duration_occupancies_with_graph_f64(
                model,
                graph,
                log,
                a,
                b,
                total,
                &options,
                &mut cached
            ),
            0
        );
        for owner in [duration, cached] {
            for frame in 0..12 {
                for state in 0..3 {
                    let mut layout = LiblrhsmmRsPosteriorSlice {
                        present: 99,
                        minimum: 99,
                        values: 99,
                    };
                    assert_eq!(
                        liblrhsmm_rs_duration_occupancies_get_slice(
                            owner,
                            frame,
                            state,
                            &mut layout
                        ),
                        0
                    );
                    let expected = expected.get(frame, state);
                    assert_eq!(layout.present, u32::from(expected.is_some()));
                    if let Some(expected) = expected {
                        assert_eq!(
                            (layout.minimum, layout.values),
                            (expected.minimum, expected.probabilities.len())
                        );
                        let mut values = vec![0.0; layout.values];
                        assert_eq!(
                            liblrhsmm_rs_duration_occupancies_copy_values(
                                owner,
                                frame,
                                state,
                                0,
                                values.as_mut_ptr(),
                                values.len()
                            ),
                            0
                        );
                        assert_eq!(
                            values.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                            expected
                                .probabilities
                                .iter()
                                .map(|v| v.to_bits())
                                .collect::<Vec<_>>()
                        );
                    }
                }
            }
        }
        let mut state = null_mut();
        assert_eq!(
            liblrhsmm_rs_model_hsmm_state_occupancies_f64(
                model, seg, log, duration, &options, &mut state
            ),
            0
        );
        assert_eq!(
            state_values_f64(state)
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            expected_state
                .values()
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>()
        );
        for mode in [
            MixtureOccupancySettings::Hsmm { state_radius: 0.5 },
            MixtureOccupancySettings::Geometric {
                pruning_slope: 0.34,
            },
        ] {
            for stream in 0..2 {
                let expected = native
                    .mixture_occupancies(
                        &observation,
                        &segmentation,
                        &expected_state,
                        stream,
                        0.75,
                        mode,
                    )
                    .unwrap();
                let mut mixtures = null_mut();
                match mode {
                    MixtureOccupancySettings::Hsmm { state_radius } => assert_eq!(
                        liblrhsmm_rs_model_mixture_occupancies_hsmm_f64(
                            model,
                            obs,
                            seg,
                            state,
                            stream,
                            0.75,
                            state_radius,
                            &mut mixtures
                        ),
                        0
                    ),
                    MixtureOccupancySettings::Geometric { pruning_slope } => assert_eq!(
                        liblrhsmm_rs_model_mixture_occupancies_geometric_f64(
                            model,
                            obs,
                            seg,
                            state,
                            stream,
                            0.75,
                            pruning_slope,
                            &mut mixtures
                        ),
                        0
                    ),
                }
                for frame in 0..12 {
                    for label in 0..3 {
                        let mut layout = LiblrhsmmRsPosteriorSlice {
                            present: 99,
                            minimum: 99,
                            values: 99,
                        };
                        assert_eq!(
                            liblrhsmm_rs_mixture_occupancies_get_slice(
                                mixtures,
                                frame,
                                label,
                                &mut layout
                            ),
                            0
                        );
                        let expected = expected.get(frame, label);
                        assert_eq!(layout.present, u32::from(expected.is_some()));
                        if let Some(expected) = expected {
                            let mut values = vec![0.0; layout.values];
                            assert_eq!(
                                liblrhsmm_rs_mixture_occupancies_copy_values(
                                    mixtures,
                                    frame,
                                    label,
                                    0,
                                    values.as_mut_ptr(),
                                    values.len()
                                ),
                                0
                            );
                            assert_eq!(
                                values.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                                expected.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
                            );
                        }
                    }
                }
                assert_eq!(liblrhsmm_rs_mixture_occupancies_release(&mut mixtures), 0);
            }
        }
        let geo = liblrhsmm_rs::geometric::GeometricSettings {
            temperature: 0.75,
            pruning_slope: 10.0,
        };
        let config = LiblrhsmmRsGeometricSettingsF64::from(geo);
        let expected_a = native
            .forward_geometric(&segmentation, &emissions, geo)
            .unwrap();
        let expected_b = native
            .backward_geometric(&segmentation, &emissions, geo)
            .unwrap();
        let geo_total = expected_a.total_log_probability();
        let expected_state = expected_a
            .geometric_occupancies(&expected_b, geo_total, 10.0)
            .unwrap();
        let expected_transition = native
            .geometric_transition_occupancies(
                &segmentation,
                &emissions,
                &expected_a,
                &expected_b,
                geo_total,
                geo,
            )
            .unwrap();
        let (mut geo_a, mut geo_b, mut geo_state, mut transition) =
            (null_mut(), null_mut(), null_mut(), null_mut());
        assert_eq!(
            liblrhsmm_rs_model_forward_geometric_f64(model, seg, log, &config, &mut geo_a),
            0
        );
        assert_eq!(
            liblrhsmm_rs_model_backward_geometric_f64(model, seg, log, &config, &mut geo_b),
            0
        );
        assert_eq!(
            liblrhsmm_rs_state_scores_geometric_occupancies_f64(
                geo_a,
                geo_b,
                geo_total,
                10.0,
                &mut geo_state
            ),
            0
        );
        assert_eq!(
            state_values_f64(geo_state)
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            expected_state
                .values()
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            liblrhsmm_rs_model_geometric_transition_occupancies_f64(
                model,
                seg,
                log,
                geo_a,
                geo_b,
                geo_total,
                &config,
                &mut transition
            ),
            0
        );
        for frame in 0..12 {
            for label in 0..3 {
                let mut value = LiblrhsmmRsTransitionPosterior {
                    present: 99,
                    stay: 99.0,
                    advance: 99.0,
                };
                assert_eq!(
                    liblrhsmm_rs_transition_occupancies_get(transition, frame, label, &mut value),
                    0
                );
                let expected = expected_transition.get(frame, label);
                assert_eq!(value.present, u32::from(expected.is_some()));
                if let Some(expected) = expected {
                    assert_eq!(value.stay.to_bits(), expected.stay.to_bits());
                    assert_eq!(value.advance.to_bits(), expected.advance.to_bits());
                }
            }
        }
        let mut invalid =
            std::ptr::without_provenance_mut::<LiblrhsmmRsDurationOccupancies>(0x1000);
        assert_eq!(
            liblrhsmm_rs_model_duration_occupancies_f64(
                model,
                seg,
                log,
                a,
                b,
                f64::NAN,
                &options,
                &mut invalid
            ),
            3
        );
        assert_eq!(invalid.addr(), 0x1000);
        assert_eq!(
            liblrhsmm_rs_model_duration_occupancies_with_graph_f64(
                model,
                graph,
                log,
                a,
                b,
                total,
                null(),
                &mut invalid
            ),
            1
        );
        assert_eq!(invalid.addr(), 0x1000);
        assert_eq!(liblrhsmm_rs_model_release_f64(&mut model), 0);
        assert_eq!(liblrhsmm_rs_observation_release_f64(&mut obs), 0);
        assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut seg), 0);
        assert_eq!(liblrhsmm_rs_transition_graph_release_f64(&mut graph), 0);
        assert_eq!(liblrhsmm_rs_log_emissions_release_f64(&mut log), 0);
        assert_eq!(
            state_values_f64(geo_state)
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            expected_state
                .values()
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>()
        );
        assert_eq!(liblrhsmm_rs_state_occupancies_release_f64(&mut state), 0);
        assert_eq!(
            liblrhsmm_rs_state_occupancies_release_f64(&mut geo_state),
            0
        );
        assert_eq!(
            liblrhsmm_rs_transition_occupancies_release(&mut transition),
            0
        );
        assert_eq!(liblrhsmm_rs_duration_occupancies_release(&mut duration), 0);
        assert_eq!(liblrhsmm_rs_duration_occupancies_release(&mut cached), 0);
        assert_eq!(liblrhsmm_rs_hsmm_scores_release_f64(&mut a), 0);
        assert_eq!(liblrhsmm_rs_hsmm_scores_release_f64(&mut b), 0);
        assert_eq!(liblrhsmm_rs_state_scores_release_f64(&mut geo_a), 0);
        assert_eq!(liblrhsmm_rs_state_scores_release_f64(&mut geo_b), 0);
    }
}
