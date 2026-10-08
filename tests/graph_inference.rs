use liblrhsmm_rs::duration_occupancy::DurationOccupancies;
use liblrhsmm_rs::emission::LogEmissionTable;
use liblrhsmm_rs::hsmm::{HsmmPath, HsmmScoreTables};
use liblrhsmm_rs::preparation::PreparationSettings;
use liblrhsmm_rs::scalar::Scalar;
use liblrhsmm_rs::transitions::TransitionGraphView;

#[path = "common/hsmm.rs"]
#[allow(dead_code)]
mod common;

fn score_bits<T: Scalar>(scores: &HsmmScoreTables<T>) -> Vec<u64> {
    scores
        .begin
        .values()
        .iter()
        .chain(scores.end.values())
        .map(|value| Into::<f64>::into(*value).to_bits())
        .collect()
}
fn path_bits<T: Scalar>(path: &HsmmPath<T>) -> (Vec<liblrhsmm_rs::StateOccurrence>, u64, Vec<u64>) {
    (
        path.occurrences.clone(),
        Into::<f64>::into(path.log_probability).to_bits(),
        score_bits(&path.scores),
    )
}
fn duration_bits(duration: &DurationOccupancies) -> Vec<Option<(usize, Vec<u32>)>> {
    duration
        .values()
        .iter()
        .map(|value| {
            value.as_ref().map(|value| {
                (
                    value.minimum,
                    value.probabilities.iter().map(|p| p.to_bits()).collect(),
                )
            })
        })
        .collect()
}
fn verify<T: Scalar>() {
    for scenario in 0..4 {
        let (model, mut source, emissions, base) = common::sample_with_precision::<T>(scenario);
        let expected = source.clone();
        let owned = source.clone().build_transitions_owned().unwrap();
        source.boundaries.fill(0);
        source.duration_states.fill(-1);
        source.add_jump(0, 0, T::ONE).unwrap();
        drop(source);
        let borrowed = expected.build_transitions().unwrap();
        let cache = owned.incoming().as_ptr();
        let group = owned.incoming()[1].as_ptr();
        for options in [
            base,
            liblrhsmm_rs::hsmm::HsmmSettings {
                temperature: T::ZERO,
                ..base
            },
            liblrhsmm_rs::hsmm::HsmmSettings {
                temperature: T::from_f64(1.75),
                state_radius: T::from_f64(0.5),
                ..base
            },
        ] {
            let expected_forward = model.forward_hsmm(&expected, &emissions, options).unwrap();
            let expected_backward = model.backward_hsmm(&expected, &emissions, options).unwrap();
            let expected_path = model
                .viterbi_hsmm(&expected, &emissions, options)
                .map(|path| path_bits(&path));
            let total = expected_forward
                .end
                .total_through_state(&expected_backward.end, 0)
                .unwrap();
            let expected_duration = model
                .duration_occupancies(
                    &expected,
                    &emissions,
                    &expected_forward,
                    &expected_backward,
                    total,
                    options,
                )
                .unwrap();
            for graph in [borrowed.as_view(), owned.as_view()] {
                for _ in 0..3 {
                    let forward = model
                        .forward_hsmm_with_graph(graph, &emissions, options)
                        .unwrap();
                    let backward = model
                        .backward_hsmm_with_graph(graph, &emissions, options)
                        .unwrap();
                    assert_eq!(score_bits(&forward), score_bits(&expected_forward));
                    assert_eq!(score_bits(&backward), score_bits(&expected_backward));
                    assert_eq!(
                        model
                            .viterbi_hsmm_with_graph(graph, &emissions, options)
                            .map(|path| path_bits(&path)),
                        expected_path
                    );
                    let duration = model
                        .duration_occupancies_with_graph(
                            graph, &emissions, &forward, &backward, total, options,
                        )
                        .unwrap();
                    assert_eq!(duration_bits(&duration), duration_bits(&expected_duration));
                    assert_eq!(duration.frames(), expected_duration.frames());
                    assert_eq!(duration.states(), expected_duration.states());
                }
                for cache_length in [0, 2, 300] {
                    let preparation = PreparationSettings {
                        temperature: options.temperature,
                        duration_weight: options.duration_weight,
                        duration_cache_length: cache_length,
                    };
                    let prepared = model.prepare(preparation).unwrap();
                    assert_eq!(
                        score_bits(
                            &prepared
                                .forward_hsmm_with_graph(graph, &emissions, options)
                                .unwrap()
                        ),
                        score_bits(
                            &prepared
                                .forward_hsmm(&expected, &emissions, options)
                                .unwrap()
                        )
                    );
                    assert_eq!(
                        score_bits(
                            &prepared
                                .backward_hsmm_with_graph(graph, &emissions, options)
                                .unwrap()
                        ),
                        score_bits(
                            &prepared
                                .backward_hsmm(&expected, &emissions, options)
                                .unwrap()
                        )
                    );
                    assert_eq!(
                        prepared
                            .viterbi_hsmm_with_graph(graph, &emissions, options)
                            .map(|path| path_bits(&path)),
                        prepared
                            .viterbi_hsmm(&expected, &emissions, options)
                            .map(|path| path_bits(&path))
                    );
                    let owned_model = model.clone().prepare_owned(preparation).unwrap();
                    assert_eq!(
                        score_bits(
                            &owned_model
                                .forward_hsmm(&expected, &emissions, options)
                                .unwrap()
                        ),
                        score_bits(
                            &prepared
                                .forward_hsmm(&expected, &emissions, options)
                                .unwrap()
                        )
                    );
                    assert_eq!(
                        score_bits(
                            &owned_model
                                .backward_hsmm(&expected, &emissions, options)
                                .unwrap()
                        ),
                        score_bits(
                            &prepared
                                .backward_hsmm(&expected, &emissions, options)
                                .unwrap()
                        )
                    );
                    assert_eq!(
                        owned_model
                            .viterbi_hsmm(&expected, &emissions, options)
                            .map(|path| path_bits(&path)),
                        prepared
                            .viterbi_hsmm(&expected, &emissions, options)
                            .map(|path| path_bits(&path))
                    );
                    {
                        assert_eq!(
                            score_bits(
                                &owned_model
                                    .forward_hsmm_with_graph(graph, &emissions, options)
                                    .unwrap()
                            ),
                            score_bits(
                                &prepared
                                    .forward_hsmm_with_graph(graph, &emissions, options)
                                    .unwrap()
                            )
                        );
                        assert_eq!(
                            score_bits(
                                &owned_model
                                    .backward_hsmm_with_graph(graph, &emissions, options)
                                    .unwrap()
                            ),
                            score_bits(
                                &prepared
                                    .backward_hsmm_with_graph(graph, &emissions, options)
                                    .unwrap()
                            )
                        );
                        assert_eq!(
                            owned_model
                                .viterbi_hsmm_with_graph(graph, &emissions, options)
                                .map(|path| path_bits(&path)),
                            prepared
                                .viterbi_hsmm_with_graph(graph, &emissions, options)
                                .map(|path| path_bits(&path))
                        );
                    }
                }
            }
        }
        assert_eq!(owned.incoming().as_ptr(), cache);
        assert_eq!(owned.incoming()[1].as_ptr(), group);
        assert_eq!(owned.segmentation(), &expected);
        assert_eq!(owned.outgoing(), borrowed.outgoing());
    }
}

#[test]
fn f32_graph_inference_reuses_both_graph_kinds_and_prepared_models() {
    verify::<f32>();
}
#[test]
fn f64_graph_inference_reuses_both_graph_kinds_and_prepared_models() {
    verify::<f64>();
}

fn verify_errors<T: Scalar>(graph: TransitionGraphView<'_, T>) {
    let (mut model, _, emissions, options) = common::sample_with_precision::<T>(0);
    let source = graph.segmentation();
    let forward = model.forward_hsmm(source, &emissions, options).unwrap();
    let backward = model.backward_hsmm(source, &emissions, options).unwrap();
    for changed in [
        liblrhsmm_rs::hsmm::HsmmSettings {
            temperature: T::from_f64(f64::NAN),
            ..options
        },
        liblrhsmm_rs::hsmm::HsmmSettings {
            duration_weight: T::from_f64(-1.0),
            ..options
        },
        liblrhsmm_rs::hsmm::HsmmSettings {
            state_radius: T::from_f64(-0.5),
            ..options
        },
        liblrhsmm_rs::hsmm::HsmmSettings {
            duration_extra_factor: T::from_f64(f64::INFINITY),
            ..options
        },
    ] {
        assert_eq!(
            model
                .forward_hsmm_with_graph(graph, &emissions, changed)
                .unwrap_err(),
            model.forward_hsmm(source, &emissions, changed).unwrap_err()
        );
        assert_eq!(
            model
                .backward_hsmm_with_graph(graph, &emissions, changed)
                .unwrap_err(),
            model
                .backward_hsmm(source, &emissions, changed)
                .unwrap_err()
        );
        assert_eq!(
            model
                .viterbi_hsmm_with_graph(graph, &emissions, changed)
                .unwrap_err(),
            model.viterbi_hsmm(source, &emissions, changed).unwrap_err()
        );
        assert_eq!(
            model
                .duration_occupancies_with_graph(
                    graph,
                    &emissions,
                    &forward,
                    &backward,
                    T::ZERO,
                    changed
                )
                .unwrap_err(),
            model
                .duration_occupancies(source, &emissions, &forward, &backward, T::ZERO, changed)
                .unwrap_err()
        );
    }
    let mismatched = LogEmissionTable::from_log_values(12, 3, vec![T::ZERO; 36]).unwrap();
    assert_eq!(
        model
            .forward_hsmm_with_graph(graph, &mismatched, options)
            .unwrap_err(),
        model
            .forward_hsmm(source, &mismatched, options)
            .unwrap_err()
    );
    assert_eq!(
        model
            .duration_occupancies_with_graph(
                graph,
                &mismatched,
                &forward,
                &backward,
                T::from_f64(f64::NAN),
                options
            )
            .unwrap_err()
            .0,
        "duration occupancy score dimensions differ"
    );
    assert_eq!(
        model
            .duration_occupancies_with_graph(
                graph,
                &emissions,
                &forward,
                &backward,
                T::from_f64(f64::NAN),
                options
            )
            .unwrap_err()
            .0,
        "duration occupancy normalizer must be finite"
    );
    let prepared = model
        .prepare(PreparationSettings {
            temperature: options.temperature,
            duration_weight: options.duration_weight,
            duration_cache_length: 2,
        })
        .unwrap();
    for changed in [
        liblrhsmm_rs::hsmm::HsmmSettings {
            temperature: T::ZERO,
            ..options
        },
        liblrhsmm_rs::hsmm::HsmmSettings {
            duration_weight: T::ZERO,
            ..options
        },
    ] {
        assert_eq!(
            prepared
                .forward_hsmm_with_graph(graph, &emissions, changed)
                .unwrap_err(),
            prepared
                .forward_hsmm(source, &emissions, changed)
                .unwrap_err()
        );
        assert_eq!(
            prepared
                .backward_hsmm_with_graph(graph, &emissions, changed)
                .unwrap_err(),
            prepared
                .backward_hsmm(source, &emissions, changed)
                .unwrap_err()
        );
        assert_eq!(
            prepared
                .viterbi_hsmm_with_graph(graph, &emissions, changed)
                .unwrap_err(),
            prepared
                .viterbi_hsmm(source, &emissions, changed)
                .unwrap_err()
        );
    }
    let owned = model
        .clone()
        .prepare_owned(PreparationSettings {
            temperature: options.temperature,
            duration_weight: options.duration_weight,
            duration_cache_length: 2,
        })
        .unwrap();
    let changed = liblrhsmm_rs::hsmm::HsmmSettings {
        temperature: T::ZERO,
        ..options
    };
    let expected_error = prepared
        .forward_hsmm(source, &emissions, changed)
        .unwrap_err();
    assert_eq!(
        owned.forward_hsmm(source, &emissions, changed).unwrap_err(),
        expected_error
    );
    assert_eq!(
        owned
            .backward_hsmm(source, &emissions, changed)
            .unwrap_err(),
        expected_error
    );
    assert_eq!(
        owned.viterbi_hsmm(source, &emissions, changed).unwrap_err(),
        expected_error
    );
    assert_eq!(
        owned
            .forward_hsmm_with_graph(graph, &emissions, changed)
            .unwrap_err(),
        expected_error
    );
    assert_eq!(
        owned
            .backward_hsmm_with_graph(graph, &emissions, changed)
            .unwrap_err(),
        expected_error
    );
    assert_eq!(
        owned
            .viterbi_hsmm_with_graph(graph, &emissions, changed)
            .unwrap_err(),
        expected_error
    );
    let partial = model
        .clone()
        .prepare_owned_selected_durations(
            PreparationSettings {
                temperature: options.temperature,
                duration_weight: options.duration_weight,
                duration_cache_length: 2,
            },
            &[0, 1],
        )
        .unwrap();
    for error in [
        partial
            .forward_hsmm_with_graph(graph, &emissions, options)
            .unwrap_err(),
        partial
            .backward_hsmm_with_graph(graph, &emissions, options)
            .unwrap_err(),
        partial
            .viterbi_hsmm_with_graph(graph, &emissions, options)
            .unwrap_err(),
    ] {
        assert_eq!(error.0, "prepared duration state is out of range");
    }
    model.durations.clear();
    assert_eq!(
        model
            .forward_hsmm_with_graph(graph, &emissions, options)
            .unwrap_err(),
        model.forward_hsmm(source, &emissions, options).unwrap_err()
    );
    assert_eq!(
        model
            .backward_hsmm_with_graph(graph, &emissions, options)
            .unwrap_err(),
        model
            .backward_hsmm(source, &emissions, options)
            .unwrap_err()
    );
    assert_eq!(
        model
            .viterbi_hsmm_with_graph(graph, &emissions, options)
            .unwrap_err(),
        model.viterbi_hsmm(source, &emissions, options).unwrap_err()
    );
}

#[test]
fn graph_inference_preserves_validation_errors_and_legacy_error_order() {
    let (_, source, _, _) = common::sample_with_precision::<f32>(0);
    let graph = source.build_transitions().unwrap();
    verify_errors(graph.as_view());
    let (_, source, _, _) = common::sample_with_precision::<f64>(0);
    let graph = source.build_transitions_owned().unwrap();
    verify_errors(graph.as_view());
    // Legacy occupancy validates scores and preparation before topology, as before.
    let (model, mut source, emissions, options) = common::sample(0);
    let forward = model.forward_hsmm(&source, &emissions, options).unwrap();
    let backward = model.backward_hsmm(&source, &emissions, options).unwrap();
    source.outgoing[0][0].probability = f32::NAN;
    assert_eq!(
        model
            .duration_occupancies(&source, &emissions, &forward, &backward, f32::NAN, options)
            .unwrap_err()
            .0,
        "duration occupancy normalizer must be finite"
    );
    let changed = liblrhsmm_rs::hsmm::HsmmSettings {
        duration_weight: -1.0,
        ..options
    };
    assert_eq!(
        model
            .duration_occupancies(&source, &emissions, &forward, &backward, 0.0, changed)
            .unwrap_err()
            .0,
        "HSMM weights and duration factor must be finite and nonnegative"
    );
    assert_eq!(
        model
            .forward_hsmm(&source, &emissions, changed)
            .unwrap_err()
            .0,
        "transition weights must be finite and nonnegative"
    );
}

fn verify_c_corpus<T: Scalar>(fixture: &str, fractional: bool, tolerance: f64) {
    let mut comparisons = 0;
    for scenario in 0..4 {
        let (model, source, emissions, mut options) = common::sample_with_precision::<T>(scenario);
        if fractional {
            options.state_radius = T::from_f64([0.25, 0.5, 1.75, 2.5][scenario]);
        }
        let graph = source.build_transitions_owned().unwrap();
        let forward = model
            .forward_hsmm_with_graph(graph.as_view(), &emissions, options)
            .unwrap();
        let backward = model
            .backward_hsmm_with_graph(graph.as_view(), &emissions, options)
            .unwrap();
        let path = model
            .viterbi_hsmm_with_graph(graph.as_view(), &emissions, options)
            .unwrap();
        let mut paths = 0;
        for line in fixture.lines() {
            let fields: Vec<_> = line.split_whitespace().collect();
            if fields[1].parse::<usize>().unwrap() != scenario {
                continue;
            }
            let index: usize = fields[2].parse().unwrap();
            if fields[0] == "path" {
                assert_eq!(
                    path.occurrences[index].end,
                    fields[3].parse::<usize>().unwrap()
                );
                assert_eq!(
                    path.occurrences[index].state,
                    fields[4].parse::<usize>().unwrap()
                );
                paths += 1;
            } else {
                assert_eq!(fields[0], "score");
                for (actual, reference) in [
                    forward.begin.values()[index],
                    forward.end.values()[index],
                    backward.begin.values()[index],
                    backward.end.values()[index],
                ]
                .into_iter()
                .zip(&fields[3..])
                {
                    let expected: f64 = T::from_f64(reference.parse().unwrap()).into();
                    let actual: f64 = actual.into();
                    if tolerance == 0.0 {
                        assert_eq!(actual.to_bits(), expected.to_bits(), "{line}");
                    } else {
                        assert!(
                            (actual - expected).abs() / expected.abs().max(1.0) <= tolerance,
                            "{line}"
                        );
                    }
                    comparisons += 1;
                }
            }
        }
        assert_eq!(paths, path.occurrences.len());
    }
    assert_eq!(comparisons, 768);
}

#[test]
fn graph_inference_matches_original_float_double_and_fractional_c_corpora() {
    verify_c_corpus::<f32>(include_str!("fixtures/c-hsmm.txt"), false, 0.0);
    verify_c_corpus::<f64>(include_str!("fixtures/c-double-hsmm.txt"), false, 2e-14);
    verify_c_corpus::<f64>(
        include_str!("fixtures/c-double-hsmm-fractional.txt"),
        true,
        2e-14,
    );
    let fixture = include_str!("fixtures/corrected-c-duration-occupancy.txt");
    assert_eq!(fixture.lines().count(), 192);
    let mut records = 0;
    for scenario in 0..4 {
        let (model, source, emissions, options) = common::sample(scenario);
        let graph = source.build_transitions_owned().unwrap();
        let forward = model
            .forward_hsmm_with_graph(graph.as_view(), &emissions, options)
            .unwrap();
        let backward = model
            .backward_hsmm_with_graph(graph.as_view(), &emissions, options)
            .unwrap();
        let total = forward.end.total_through_state(&backward.end, 0).unwrap();
        let duration = model
            .duration_occupancies_with_graph(
                graph.as_view(),
                &emissions,
                &forward,
                &backward,
                total,
                options,
            )
            .unwrap();
        for line in fixture.lines() {
            let fields: Vec<_> = line.split_whitespace().collect();
            if fields[0].parse::<usize>().unwrap() != scenario {
                continue;
            }
            let frame: usize = fields[1].parse().unwrap();
            let state: usize = fields[2].parse().unwrap();
            let minimum: usize = fields[3].parse().unwrap();
            let count: usize = fields[4].parse().unwrap();
            let actual = duration.get(frame, state);
            assert_eq!(actual.is_some(), count != 0);
            if let Some(actual) = actual {
                assert_eq!(actual.minimum, minimum);
                assert_eq!(actual.probabilities.len(), count);
                for (actual, expected) in actual.probabilities.iter().zip(&fields[5..5 + count]) {
                    assert_eq!(
                        actual.to_bits(),
                        expected.parse::<f32>().unwrap().to_bits(),
                        "{line}"
                    );
                }
            }
            records += 1;
        }
    }
    assert_eq!(records, 192);
}
