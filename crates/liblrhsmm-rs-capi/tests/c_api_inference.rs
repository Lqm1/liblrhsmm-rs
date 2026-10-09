use liblrhsmm_rs::geometric::{GeometricPath, GeometricSettings, StateScoreTable};
use liblrhsmm_rs::hsmm::{HsmmPath, HsmmScoreTables};
use liblrhsmm_rs_capi::*;
use std::ptr::{null, null_mut};
#[path = "common/hsmm.rs"]
#[allow(dead_code)]
mod common;

unsafe fn table_f32(owner: *const LiblrhsmmRsStateScoresF32) -> StateScoreTable<f32> {
    unsafe {
        let mut layout = LiblrhsmmRsInferenceTableLayout {
            frames: 0,
            states: 0,
            values: 0,
        };
        assert_eq!(liblrhsmm_rs_state_scores_layout_f32(owner, &mut layout), 0);
        let mut values = vec![0.0; layout.values];
        assert_eq!(
            liblrhsmm_rs_state_scores_copy_values_f32(owner, 0, values.as_mut_ptr(), values.len()),
            0
        );
        StateScoreTable::from_values(layout.frames, layout.states, values).unwrap()
    }
}
fn bits_f32(table: &StateScoreTable<f32>) -> Vec<u32> {
    table.values().iter().map(|v| v.to_bits()).collect()
}
unsafe fn scores_f32(owner: *const LiblrhsmmRsHsmmScoresF32) -> HsmmScoreTables<f32> {
    unsafe {
        let (mut begin, mut end) = (null_mut(), null_mut());
        assert_eq!(liblrhsmm_rs_hsmm_scores_get_begin_f32(owner, &mut begin), 0);
        assert_eq!(liblrhsmm_rs_hsmm_scores_get_end_f32(owner, &mut end), 0);
        let result = HsmmScoreTables {
            begin: table_f32(begin),
            end: table_f32(end),
        };
        assert_eq!(liblrhsmm_rs_state_scores_release_f32(&mut begin), 0);
        assert_eq!(liblrhsmm_rs_state_scores_release_f32(&mut end), 0);
        result
    }
}
fn same_scores_f32(actual: &HsmmScoreTables<f32>, expected: &HsmmScoreTables<f32>) {
    assert_eq!(bits_f32(&actual.begin), bits_f32(&expected.begin));
    assert_eq!(bits_f32(&actual.end), bits_f32(&expected.end));
}
unsafe fn hsmm_path_f32(owner: *const LiblrhsmmRsHsmmPathF32, expected: &HsmmPath<f32>) {
    unsafe {
        let mut layout = LiblrhsmmRsPathLayout {
            frames: 0,
            states: 0,
            occurrences: 0,
        };
        assert_eq!(liblrhsmm_rs_hsmm_path_layout_f32(owner, &mut layout), 0);
        assert_eq!(
            (layout.frames, layout.states, layout.occurrences),
            (
                expected.scores.begin.frames(),
                expected.scores.begin.states(),
                expected.occurrences.len()
            )
        );
        let mut likelihood = 0.0_f32;
        assert_eq!(
            liblrhsmm_rs_hsmm_path_log_probability_f32(owner, &mut likelihood),
            0
        );
        assert_eq!(likelihood.to_bits(), expected.log_probability.to_bits());
        let sentinel = LiblrhsmmRsStateOccurrence {
            state: usize::MAX,
            end: usize::MAX,
        };
        let mut occurrences = vec![sentinel; layout.occurrences];
        assert_eq!(
            liblrhsmm_rs_hsmm_path_copy_occurrences_f32(
                owner,
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
            expected
                .occurrences
                .iter()
                .map(|v| (v.state, v.end))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            liblrhsmm_rs_hsmm_path_copy_occurrences_f32(owner, occurrences.len(), null_mut(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_hsmm_path_copy_occurrences_f32(
                owner,
                usize::MAX,
                occurrences.as_mut_ptr(),
                1
            ),
            2
        );
        assert_eq!(
            liblrhsmm_rs_hsmm_path_copy_occurrences_f32(owner, 0, null_mut(), 1),
            1
        );
        assert_eq!(
            liblrhsmm_rs_hsmm_path_copy_occurrences_f32(
                owner,
                0,
                std::ptr::without_provenance_mut(1),
                1
            ),
            1
        );
        let mut child = null_mut();
        assert_eq!(liblrhsmm_rs_hsmm_path_get_scores_f32(owner, &mut child), 0);
        same_scores_f32(&scores_f32(child), &expected.scores);
        assert_eq!(liblrhsmm_rs_hsmm_scores_release_f32(&mut child), 0);
    }
}
unsafe fn geometric_path_f32(
    owner: *const LiblrhsmmRsGeometricPathF32,
    expected: &GeometricPath<f32>,
) {
    unsafe {
        let mut layout = LiblrhsmmRsPathLayout {
            frames: 0,
            states: 0,
            occurrences: 0,
        };
        assert_eq!(
            liblrhsmm_rs_geometric_path_layout_f32(owner, &mut layout),
            0
        );
        assert_eq!(
            (layout.frames, layout.states, layout.occurrences),
            (
                expected.states.len(),
                expected.boundaries.len(),
                expected.boundaries.len()
            )
        );
        let (mut states, mut boundaries) = (
            vec![usize::MAX; layout.frames],
            vec![usize::MAX; layout.states],
        );
        assert_eq!(
            liblrhsmm_rs_geometric_path_copy_states_f32(
                owner,
                0,
                states.as_mut_ptr(),
                states.len()
            ),
            0
        );
        assert_eq!(
            liblrhsmm_rs_geometric_path_copy_boundaries_f32(
                owner,
                0,
                boundaries.as_mut_ptr(),
                boundaries.len()
            ),
            0
        );
        assert_eq!(states, expected.states);
        assert_eq!(boundaries, expected.boundaries);
        let original = states.clone();
        assert_eq!(
            liblrhsmm_rs_geometric_path_copy_states_f32(
                owner,
                states.len(),
                states.as_mut_ptr(),
                1
            ),
            2
        );
        assert_eq!(states, original);
        assert_eq!(
            liblrhsmm_rs_geometric_path_copy_states_f32(owner, states.len(), null_mut(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_geometric_path_copy_boundaries_f32(owner, boundaries.len(), null_mut(), 0),
            0
        );
        let mut likelihood = 0.0_f32;
        assert_eq!(
            liblrhsmm_rs_geometric_path_log_probability_f32(owner, &mut likelihood),
            0
        );
        assert_eq!(likelihood.to_bits(), expected.log_probability.to_bits());
        let mut child = null_mut();
        assert_eq!(
            liblrhsmm_rs_geometric_path_get_scores_f32(owner, &mut child),
            0
        );
        assert_eq!(bits_f32(&table_f32(child)), bits_f32(&expected.scores));
        assert_eq!(liblrhsmm_rs_state_scores_release_f32(&mut child), 0);
    }
}

#[test]
fn inference_f32_complete_results_all_model_paths_and_independent_children() {
    for scenario in 0..4 {
        let (native, segmentation, emissions, settings) =
            common::sample_with_precision::<f32>(scenario);
        unsafe {
            let (mut model, mut seg, mut log, mut graph) =
                (null_mut(), null_mut(), null_mut(), null_mut());
            let mut durations = Vec::new();
            for value in &native.durations {
                let params = LiblrhsmmRsDurationParametersF32 {
                    mean: value.mean,
                    variance: value.variance,
                    minimum: value.minimum,
                    maximum: value.maximum,
                    fixed_mean: value.fixed_mean,
                    variance_floor: value.variance_floor,
                };
                let mut duration = null_mut();
                assert_eq!(liblrhsmm_rs_duration_create_f32(&params, &mut duration), 0);
                durations.push(duration);
            }
            let borrowed: Vec<_> = durations.iter().map(|v| v.cast_const()).collect();
            assert_eq!(
                liblrhsmm_rs_model_create_f32(
                    null(),
                    0,
                    borrowed.as_ptr(),
                    borrowed.len(),
                    &mut model
                ),
                0
            );
            for duration in &mut durations {
                assert_eq!(liblrhsmm_rs_duration_release_f32(duration), 0);
            }
            let mut jumps: Vec<Vec<LiblrhsmmRsJumpF32>> = segmentation
                .outgoing
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|v| LiblrhsmmRsJumpF32 {
                            delta: v.delta,
                            probability: v.probability,
                        })
                        .collect()
                })
                .collect();
            let slices: Vec<_> = jumps
                .iter()
                .map(|v| LiblrhsmmRsJumpSliceF32 {
                    jumps: v.as_ptr(),
                    count: v.len(),
                })
                .collect();
            assert_eq!(
                liblrhsmm_rs_segmentation_create_f32(
                    0,
                    4,
                    segmentation.boundaries.as_ptr(),
                    segmentation.duration_states.as_ptr(),
                    null(),
                    slices.as_ptr(),
                    &mut seg
                ),
                0
            );
            jumps.clear();
            assert_eq!(
                liblrhsmm_rs_segmentation_build_transitions_f32(seg, &mut graph),
                0
            );
            assert_eq!(
                liblrhsmm_rs_log_emissions_create_f32(
                    emissions.frames(),
                    emissions.states(),
                    emissions.values().as_ptr(),
                    emissions.values().len(),
                    &mut log
                ),
                0
            );
            let mut options = LiblrhsmmRsHsmmSettingsF32 {
                temperature: 99.0,
                duration_weight: 99.0,
                state_radius: 99.0,
                duration_extra: 99,
                duration_extra_factor: 99.0,
            };
            assert_eq!(liblrhsmm_rs_hsmm_settings_default_f32(&mut options), 0);
            assert_eq!(
                liblrhsmm_rs::hsmm::HsmmSettings::<f32>::from(options),
                Default::default()
            );
            options = settings.into();
            assert_eq!(liblrhsmm_rs_hsmm_settings_default_f32(null_mut()), 1);
            let mut geo = LiblrhsmmRsGeometricSettingsF32 {
                temperature: 99.0,
                pruning_slope: 99.0,
            };
            assert_eq!(liblrhsmm_rs_geometric_settings_default_f32(&mut geo), 0);
            assert_eq!(GeometricSettings::<f32>::from(geo), Default::default());
            assert_eq!(liblrhsmm_rs_geometric_settings_default_f32(null_mut()), 1);
            geo.pruning_slope = 10.0;

            let expected_forward = native
                .forward_hsmm(&segmentation, &emissions, settings)
                .unwrap();
            let expected_backward = native
                .backward_hsmm(&segmentation, &emissions, settings)
                .unwrap();
            let expected_path = native.viterbi_hsmm(&segmentation, &emissions, settings);
            for cache_length in [0, 2, 300] {
                let prep = LiblrhsmmRsDurationPreparationF32 {
                    duration_cache_length: cache_length,
                    duration_weight: settings.duration_weight,
                    temperature: settings.temperature,
                };
                let mut prepared = null_mut();
                assert_eq!(
                    liblrhsmm_rs_model_prepare_f32(model, &prep, &mut prepared),
                    0
                );
                let mut forward = null_mut();
                let mut backward = null_mut();
                for (function, expected) in [
                    (
                        liblrhsmm_rs_model_forward_hsmm_f32
                            as unsafe extern "C" fn(
                                *const LiblrhsmmRsModelF32,
                                *const LiblrhsmmRsSegmentationF32,
                                *const LiblrhsmmRsLogEmissionsF32,
                                *const LiblrhsmmRsHsmmSettingsF32,
                                *mut *mut LiblrhsmmRsHsmmScoresF32,
                            ) -> u32,
                        &expected_forward,
                    ),
                    (
                        liblrhsmm_rs_model_backward_hsmm_f32 as _,
                        &expected_backward,
                    ),
                ] {
                    let mut result = null_mut();
                    assert_eq!(function(model, seg, log, &options, &mut result), 0);
                    same_scores_f32(&scores_f32(result), expected);
                    assert_eq!(liblrhsmm_rs_hsmm_scores_release_f32(&mut result), 0);
                }
                assert_eq!(
                    liblrhsmm_rs_model_forward_hsmm_with_graph_f32(
                        model,
                        graph,
                        log,
                        &options,
                        &mut forward
                    ),
                    0
                );
                assert_eq!(
                    liblrhsmm_rs_model_backward_hsmm_with_graph_f32(
                        model,
                        graph,
                        log,
                        &options,
                        &mut backward
                    ),
                    0
                );
                same_scores_f32(&scores_f32(forward), &expected_forward);
                same_scores_f32(&scores_f32(backward), &expected_backward);
                for (function, expected) in [
                    (
                        liblrhsmm_rs_prepared_model_forward_hsmm_f32
                            as unsafe extern "C" fn(
                                *const LiblrhsmmRsPreparedModelF32,
                                *const LiblrhsmmRsSegmentationF32,
                                *const LiblrhsmmRsLogEmissionsF32,
                                *const LiblrhsmmRsHsmmSettingsF32,
                                *mut *mut LiblrhsmmRsHsmmScoresF32,
                            ) -> u32,
                        &expected_forward,
                    ),
                    (
                        liblrhsmm_rs_prepared_model_backward_hsmm_f32 as _,
                        &expected_backward,
                    ),
                ] {
                    let mut result = null_mut();
                    assert_eq!(function(prepared, seg, log, &options, &mut result), 0);
                    same_scores_f32(&scores_f32(result), expected);
                    assert_eq!(liblrhsmm_rs_hsmm_scores_release_f32(&mut result), 0);
                }
                for (function, expected) in [
                    (
                        liblrhsmm_rs_prepared_model_forward_hsmm_with_graph_f32
                            as unsafe extern "C" fn(
                                *const LiblrhsmmRsPreparedModelF32,
                                *const LiblrhsmmRsTransitionGraphF32,
                                *const LiblrhsmmRsLogEmissionsF32,
                                *const LiblrhsmmRsHsmmSettingsF32,
                                *mut *mut LiblrhsmmRsHsmmScoresF32,
                            ) -> u32,
                        &expected_forward,
                    ),
                    (
                        liblrhsmm_rs_prepared_model_backward_hsmm_with_graph_f32 as _,
                        &expected_backward,
                    ),
                ] {
                    let mut result = null_mut();
                    assert_eq!(function(prepared, graph, log, &options, &mut result), 0);
                    same_scores_f32(&scores_f32(result), expected);
                    assert_eq!(liblrhsmm_rs_hsmm_scores_release_f32(&mut result), 0);
                }
                let mut clone = null_mut();
                assert_eq!(liblrhsmm_rs_hsmm_scores_clone_f32(forward, &mut clone), 0);
                let (mut begin, mut end) = (null_mut(), null_mut());
                assert_eq!(liblrhsmm_rs_hsmm_scores_get_begin_f32(clone, &mut begin), 0);
                assert_eq!(liblrhsmm_rs_hsmm_scores_get_end_f32(clone, &mut end), 0);
                let mut imported = null_mut();
                assert_eq!(
                    liblrhsmm_rs_hsmm_scores_create_f32(begin, end, &mut imported),
                    0
                );
                assert_eq!(liblrhsmm_rs_state_scores_release_f32(&mut begin), 0);
                assert_eq!(liblrhsmm_rs_state_scores_release_f32(&mut end), 0);
                assert_eq!(liblrhsmm_rs_hsmm_scores_release_f32(&mut clone), 0);
                same_scores_f32(&scores_f32(imported), &expected_forward);
                assert_eq!(liblrhsmm_rs_hsmm_scores_release_f32(&mut imported), 0);

                {
                    let mut path = null_mut();
                    let status =
                        liblrhsmm_rs_model_viterbi_hsmm_f32(model, seg, log, &options, &mut path);
                    assert_eq!(status, if expected_path.is_ok() { 0 } else { 3 });
                    if let Ok(expected) = &expected_path {
                        hsmm_path_f32(path, expected);
                    }
                    assert_eq!(liblrhsmm_rs_hsmm_path_release_f32(&mut path), 0);
                }
                let mut retained = null_mut();
                let status = liblrhsmm_rs_model_viterbi_hsmm_with_graph_f32(
                    model,
                    graph,
                    log,
                    &options,
                    &mut retained,
                );
                assert_eq!(status, if expected_path.is_ok() { 0 } else { 3 });
                {
                    let mut path = null_mut();
                    let status = liblrhsmm_rs_prepared_model_viterbi_hsmm_f32(
                        prepared, seg, log, &options, &mut path,
                    );
                    assert_eq!(status, if expected_path.is_ok() { 0 } else { 3 });
                    if let Ok(expected) = &expected_path {
                        hsmm_path_f32(path, expected);
                    }
                    assert_eq!(liblrhsmm_rs_hsmm_path_release_f32(&mut path), 0);
                }
                let mut cached_path = null_mut();
                let status = liblrhsmm_rs_prepared_model_viterbi_hsmm_with_graph_f32(
                    prepared,
                    graph,
                    log,
                    &options,
                    &mut cached_path,
                );
                assert_eq!(status, if expected_path.is_ok() { 0 } else { 3 });
                assert_eq!(liblrhsmm_rs_prepared_model_release_f32(&mut prepared), 0);
                if let Ok(expected) = &expected_path {
                    hsmm_path_f32(retained, expected);
                    hsmm_path_f32(cached_path, expected);
                    let mut clone = null_mut();
                    assert_eq!(liblrhsmm_rs_hsmm_path_clone_f32(retained, &mut clone), 0);
                    assert_eq!(liblrhsmm_rs_hsmm_path_release_f32(&mut retained), 0);
                    hsmm_path_f32(clone, expected);
                    assert_eq!(liblrhsmm_rs_hsmm_path_release_f32(&mut clone), 0);
                }
                assert_eq!(liblrhsmm_rs_hsmm_path_release_f32(&mut retained), 0);
                assert_eq!(liblrhsmm_rs_hsmm_path_release_f32(&mut cached_path), 0);
                assert_eq!(liblrhsmm_rs_hsmm_scores_release_f32(&mut forward), 0);
                assert_eq!(liblrhsmm_rs_hsmm_scores_release_f32(&mut backward), 0);
            }
            let native_geo = GeometricSettings::from(geo);
            for (function, expected) in [
                (
                    liblrhsmm_rs_model_forward_geometric_f32
                        as unsafe extern "C" fn(
                            *const LiblrhsmmRsModelF32,
                            *const LiblrhsmmRsSegmentationF32,
                            *const LiblrhsmmRsLogEmissionsF32,
                            *const LiblrhsmmRsGeometricSettingsF32,
                            *mut *mut LiblrhsmmRsStateScoresF32,
                        ) -> u32,
                    native
                        .forward_geometric(&segmentation, &emissions, native_geo)
                        .unwrap(),
                ),
                (
                    liblrhsmm_rs_model_backward_geometric_f32 as _,
                    native
                        .backward_geometric(&segmentation, &emissions, native_geo)
                        .unwrap(),
                ),
            ] {
                let mut result = null_mut();
                assert_eq!(function(model, seg, log, &geo, &mut result), 0);
                assert_eq!(bits_f32(&table_f32(result)), bits_f32(&expected));
                assert_eq!(liblrhsmm_rs_state_scores_release_f32(&mut result), 0);
            }
            let expected = native
                .viterbi_geometric(&segmentation, &emissions, native_geo)
                .unwrap();
            let mut path = null_mut();
            assert_eq!(
                liblrhsmm_rs_model_viterbi_geometric_f32(model, seg, log, &geo, &mut path),
                0
            );
            let mut cloned = null_mut();
            assert_eq!(liblrhsmm_rs_geometric_path_clone_f32(path, &mut cloned), 0);
            assert_eq!(liblrhsmm_rs_geometric_path_release_f32(&mut path), 0);
            let mut stale = std::ptr::without_provenance_mut::<LiblrhsmmRsHsmmScoresF32>(0x1000);
            let invalid = LiblrhsmmRsHsmmSettingsF32 {
                temperature: f32::NAN,
                ..options
            };
            assert_eq!(
                liblrhsmm_rs_model_forward_hsmm_f32(model, seg, log, &invalid, &mut stale),
                3
            );
            assert_eq!(stale.addr(), 0x1000);
            assert_eq!(
                liblrhsmm_rs_model_forward_hsmm_f32(null(), seg, log, &options, &mut stale),
                1
            );
            assert_eq!(
                liblrhsmm_rs_model_forward_hsmm_f32(model, seg, log, null(), &mut stale),
                1
            );
            assert_eq!(
                liblrhsmm_rs_model_forward_hsmm_f32(model, seg, log, &options, null_mut()),
                1
            );
            assert_eq!(liblrhsmm_rs_model_release_f32(&mut model), 0);
            assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut seg), 0);
            assert_eq!(liblrhsmm_rs_transition_graph_release_f32(&mut graph), 0);
            assert_eq!(liblrhsmm_rs_log_emissions_release_f32(&mut log), 0);
            geometric_path_f32(cloned, &expected);
            assert_eq!(liblrhsmm_rs_geometric_path_release_f32(&mut cloned), 0);
            assert_eq!(liblrhsmm_rs_geometric_path_release_f32(&mut cloned), 0);
            assert_eq!(liblrhsmm_rs_geometric_path_release_f32(null_mut()), 1);
            assert_eq!(liblrhsmm_rs_hsmm_path_release_f32(null_mut()), 1);
            assert_eq!(liblrhsmm_rs_hsmm_scores_release_f32(null_mut()), 1);
        }
    }
}

unsafe fn table_f64(owner: *const LiblrhsmmRsStateScoresF64) -> StateScoreTable<f64> {
    unsafe {
        let mut layout = LiblrhsmmRsInferenceTableLayout {
            frames: 0,
            states: 0,
            values: 0,
        };
        assert_eq!(liblrhsmm_rs_state_scores_layout_f64(owner, &mut layout), 0);
        let mut values = vec![0.0; layout.values];
        assert_eq!(
            liblrhsmm_rs_state_scores_copy_values_f64(owner, 0, values.as_mut_ptr(), values.len()),
            0
        );
        StateScoreTable::from_values(layout.frames, layout.states, values).unwrap()
    }
}
fn bits_f64(table: &StateScoreTable<f64>) -> Vec<u64> {
    table.values().iter().map(|v| v.to_bits()).collect()
}
unsafe fn scores_f64(owner: *const LiblrhsmmRsHsmmScoresF64) -> HsmmScoreTables<f64> {
    unsafe {
        let (mut begin, mut end) = (null_mut(), null_mut());
        assert_eq!(liblrhsmm_rs_hsmm_scores_get_begin_f64(owner, &mut begin), 0);
        assert_eq!(liblrhsmm_rs_hsmm_scores_get_end_f64(owner, &mut end), 0);
        let result = HsmmScoreTables {
            begin: table_f64(begin),
            end: table_f64(end),
        };
        assert_eq!(liblrhsmm_rs_state_scores_release_f64(&mut begin), 0);
        assert_eq!(liblrhsmm_rs_state_scores_release_f64(&mut end), 0);
        result
    }
}
fn same_scores_f64(actual: &HsmmScoreTables<f64>, expected: &HsmmScoreTables<f64>) {
    assert_eq!(bits_f64(&actual.begin), bits_f64(&expected.begin));
    assert_eq!(bits_f64(&actual.end), bits_f64(&expected.end));
}
unsafe fn hsmm_path_f64(owner: *const LiblrhsmmRsHsmmPathF64, expected: &HsmmPath<f64>) {
    unsafe {
        let mut layout = LiblrhsmmRsPathLayout {
            frames: 0,
            states: 0,
            occurrences: 0,
        };
        assert_eq!(liblrhsmm_rs_hsmm_path_layout_f64(owner, &mut layout), 0);
        assert_eq!(
            (layout.frames, layout.states, layout.occurrences),
            (
                expected.scores.begin.frames(),
                expected.scores.begin.states(),
                expected.occurrences.len()
            )
        );
        let mut likelihood = 0.0_f64;
        assert_eq!(
            liblrhsmm_rs_hsmm_path_log_probability_f64(owner, &mut likelihood),
            0
        );
        assert_eq!(likelihood.to_bits(), expected.log_probability.to_bits());
        let sentinel = LiblrhsmmRsStateOccurrence {
            state: usize::MAX,
            end: usize::MAX,
        };
        let mut occurrences = vec![sentinel; layout.occurrences];
        assert_eq!(
            liblrhsmm_rs_hsmm_path_copy_occurrences_f64(
                owner,
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
            expected
                .occurrences
                .iter()
                .map(|v| (v.state, v.end))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            liblrhsmm_rs_hsmm_path_copy_occurrences_f64(owner, occurrences.len(), null_mut(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_hsmm_path_copy_occurrences_f64(
                owner,
                usize::MAX,
                occurrences.as_mut_ptr(),
                1
            ),
            2
        );
        assert_eq!(
            liblrhsmm_rs_hsmm_path_copy_occurrences_f64(owner, 0, null_mut(), 1),
            1
        );
        assert_eq!(
            liblrhsmm_rs_hsmm_path_copy_occurrences_f64(
                owner,
                0,
                std::ptr::without_provenance_mut(1),
                1
            ),
            1
        );
        let mut child = null_mut();
        assert_eq!(liblrhsmm_rs_hsmm_path_get_scores_f64(owner, &mut child), 0);
        same_scores_f64(&scores_f64(child), &expected.scores);
        assert_eq!(liblrhsmm_rs_hsmm_scores_release_f64(&mut child), 0);
    }
}
unsafe fn geometric_path_f64(
    owner: *const LiblrhsmmRsGeometricPathF64,
    expected: &GeometricPath<f64>,
) {
    unsafe {
        let mut layout = LiblrhsmmRsPathLayout {
            frames: 0,
            states: 0,
            occurrences: 0,
        };
        assert_eq!(
            liblrhsmm_rs_geometric_path_layout_f64(owner, &mut layout),
            0
        );
        assert_eq!(
            (layout.frames, layout.states, layout.occurrences),
            (
                expected.states.len(),
                expected.boundaries.len(),
                expected.boundaries.len()
            )
        );
        let (mut states, mut boundaries) = (
            vec![usize::MAX; layout.frames],
            vec![usize::MAX; layout.states],
        );
        assert_eq!(
            liblrhsmm_rs_geometric_path_copy_states_f64(
                owner,
                0,
                states.as_mut_ptr(),
                states.len()
            ),
            0
        );
        assert_eq!(
            liblrhsmm_rs_geometric_path_copy_boundaries_f64(
                owner,
                0,
                boundaries.as_mut_ptr(),
                boundaries.len()
            ),
            0
        );
        assert_eq!(states, expected.states);
        assert_eq!(boundaries, expected.boundaries);
        let original = states.clone();
        assert_eq!(
            liblrhsmm_rs_geometric_path_copy_states_f64(
                owner,
                states.len(),
                states.as_mut_ptr(),
                1
            ),
            2
        );
        assert_eq!(states, original);
        assert_eq!(
            liblrhsmm_rs_geometric_path_copy_states_f64(owner, states.len(), null_mut(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_geometric_path_copy_boundaries_f64(owner, boundaries.len(), null_mut(), 0),
            0
        );
        let mut likelihood = 0.0_f64;
        assert_eq!(
            liblrhsmm_rs_geometric_path_log_probability_f64(owner, &mut likelihood),
            0
        );
        assert_eq!(likelihood.to_bits(), expected.log_probability.to_bits());
        let mut child = null_mut();
        assert_eq!(
            liblrhsmm_rs_geometric_path_get_scores_f64(owner, &mut child),
            0
        );
        assert_eq!(bits_f64(&table_f64(child)), bits_f64(&expected.scores));
        assert_eq!(liblrhsmm_rs_state_scores_release_f64(&mut child), 0);
    }
}

#[test]
fn inference_f64_complete_results_all_model_paths_and_independent_children() {
    for scenario in 0..4 {
        let (native, segmentation, emissions, settings) =
            common::sample_with_precision::<f64>(scenario);
        unsafe {
            let (mut model, mut seg, mut log, mut graph) =
                (null_mut(), null_mut(), null_mut(), null_mut());
            let mut durations = Vec::new();
            for value in &native.durations {
                let params = LiblrhsmmRsDurationParametersF64 {
                    mean: value.mean,
                    variance: value.variance,
                    minimum: value.minimum,
                    maximum: value.maximum,
                    fixed_mean: value.fixed_mean,
                    variance_floor: value.variance_floor,
                };
                let mut duration = null_mut();
                assert_eq!(liblrhsmm_rs_duration_create_f64(&params, &mut duration), 0);
                durations.push(duration);
            }
            let borrowed: Vec<_> = durations.iter().map(|v| v.cast_const()).collect();
            assert_eq!(
                liblrhsmm_rs_model_create_f64(
                    null(),
                    0,
                    borrowed.as_ptr(),
                    borrowed.len(),
                    &mut model
                ),
                0
            );
            for duration in &mut durations {
                assert_eq!(liblrhsmm_rs_duration_release_f64(duration), 0);
            }
            let mut jumps: Vec<Vec<LiblrhsmmRsJumpF64>> = segmentation
                .outgoing
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|v| LiblrhsmmRsJumpF64 {
                            delta: v.delta,
                            probability: v.probability,
                        })
                        .collect()
                })
                .collect();
            let slices: Vec<_> = jumps
                .iter()
                .map(|v| LiblrhsmmRsJumpSliceF64 {
                    jumps: v.as_ptr(),
                    count: v.len(),
                })
                .collect();
            assert_eq!(
                liblrhsmm_rs_segmentation_create_f64(
                    0,
                    4,
                    segmentation.boundaries.as_ptr(),
                    segmentation.duration_states.as_ptr(),
                    null(),
                    slices.as_ptr(),
                    &mut seg
                ),
                0
            );
            jumps.clear();
            assert_eq!(
                liblrhsmm_rs_segmentation_build_transitions_f64(seg, &mut graph),
                0
            );
            assert_eq!(
                liblrhsmm_rs_log_emissions_create_f64(
                    emissions.frames(),
                    emissions.states(),
                    emissions.values().as_ptr(),
                    emissions.values().len(),
                    &mut log
                ),
                0
            );
            let mut options = LiblrhsmmRsHsmmSettingsF64 {
                temperature: 99.0,
                duration_weight: 99.0,
                state_radius: 99.0,
                duration_extra: 99,
                duration_extra_factor: 99.0,
            };
            assert_eq!(liblrhsmm_rs_hsmm_settings_default_f64(&mut options), 0);
            assert_eq!(
                liblrhsmm_rs::hsmm::HsmmSettings::<f64>::from(options),
                Default::default()
            );
            options = settings.into();
            assert_eq!(liblrhsmm_rs_hsmm_settings_default_f64(null_mut()), 1);
            let mut geo = LiblrhsmmRsGeometricSettingsF64 {
                temperature: 99.0,
                pruning_slope: 99.0,
            };
            assert_eq!(liblrhsmm_rs_geometric_settings_default_f64(&mut geo), 0);
            assert_eq!(GeometricSettings::<f64>::from(geo), Default::default());
            assert_eq!(liblrhsmm_rs_geometric_settings_default_f64(null_mut()), 1);
            geo.pruning_slope = 10.0;

            let expected_forward = native
                .forward_hsmm(&segmentation, &emissions, settings)
                .unwrap();
            let expected_backward = native
                .backward_hsmm(&segmentation, &emissions, settings)
                .unwrap();
            let expected_path = native.viterbi_hsmm(&segmentation, &emissions, settings);
            for cache_length in [0, 2, 300] {
                let prep = LiblrhsmmRsDurationPreparationF64 {
                    duration_cache_length: cache_length,
                    duration_weight: settings.duration_weight,
                    temperature: settings.temperature,
                };
                let mut prepared = null_mut();
                assert_eq!(
                    liblrhsmm_rs_model_prepare_f64(model, &prep, &mut prepared),
                    0
                );
                let mut forward = null_mut();
                let mut backward = null_mut();
                for (function, expected) in [
                    (
                        liblrhsmm_rs_model_forward_hsmm_f64
                            as unsafe extern "C" fn(
                                *const LiblrhsmmRsModelF64,
                                *const LiblrhsmmRsSegmentationF64,
                                *const LiblrhsmmRsLogEmissionsF64,
                                *const LiblrhsmmRsHsmmSettingsF64,
                                *mut *mut LiblrhsmmRsHsmmScoresF64,
                            ) -> u32,
                        &expected_forward,
                    ),
                    (
                        liblrhsmm_rs_model_backward_hsmm_f64 as _,
                        &expected_backward,
                    ),
                ] {
                    let mut result = null_mut();
                    assert_eq!(function(model, seg, log, &options, &mut result), 0);
                    same_scores_f64(&scores_f64(result), expected);
                    assert_eq!(liblrhsmm_rs_hsmm_scores_release_f64(&mut result), 0);
                }
                assert_eq!(
                    liblrhsmm_rs_model_forward_hsmm_with_graph_f64(
                        model,
                        graph,
                        log,
                        &options,
                        &mut forward
                    ),
                    0
                );
                assert_eq!(
                    liblrhsmm_rs_model_backward_hsmm_with_graph_f64(
                        model,
                        graph,
                        log,
                        &options,
                        &mut backward
                    ),
                    0
                );
                same_scores_f64(&scores_f64(forward), &expected_forward);
                same_scores_f64(&scores_f64(backward), &expected_backward);
                for (function, expected) in [
                    (
                        liblrhsmm_rs_prepared_model_forward_hsmm_f64
                            as unsafe extern "C" fn(
                                *const LiblrhsmmRsPreparedModelF64,
                                *const LiblrhsmmRsSegmentationF64,
                                *const LiblrhsmmRsLogEmissionsF64,
                                *const LiblrhsmmRsHsmmSettingsF64,
                                *mut *mut LiblrhsmmRsHsmmScoresF64,
                            ) -> u32,
                        &expected_forward,
                    ),
                    (
                        liblrhsmm_rs_prepared_model_backward_hsmm_f64 as _,
                        &expected_backward,
                    ),
                ] {
                    let mut result = null_mut();
                    assert_eq!(function(prepared, seg, log, &options, &mut result), 0);
                    same_scores_f64(&scores_f64(result), expected);
                    assert_eq!(liblrhsmm_rs_hsmm_scores_release_f64(&mut result), 0);
                }
                for (function, expected) in [
                    (
                        liblrhsmm_rs_prepared_model_forward_hsmm_with_graph_f64
                            as unsafe extern "C" fn(
                                *const LiblrhsmmRsPreparedModelF64,
                                *const LiblrhsmmRsTransitionGraphF64,
                                *const LiblrhsmmRsLogEmissionsF64,
                                *const LiblrhsmmRsHsmmSettingsF64,
                                *mut *mut LiblrhsmmRsHsmmScoresF64,
                            ) -> u32,
                        &expected_forward,
                    ),
                    (
                        liblrhsmm_rs_prepared_model_backward_hsmm_with_graph_f64 as _,
                        &expected_backward,
                    ),
                ] {
                    let mut result = null_mut();
                    assert_eq!(function(prepared, graph, log, &options, &mut result), 0);
                    same_scores_f64(&scores_f64(result), expected);
                    assert_eq!(liblrhsmm_rs_hsmm_scores_release_f64(&mut result), 0);
                }
                let mut clone = null_mut();
                assert_eq!(liblrhsmm_rs_hsmm_scores_clone_f64(forward, &mut clone), 0);
                let (mut begin, mut end) = (null_mut(), null_mut());
                assert_eq!(liblrhsmm_rs_hsmm_scores_get_begin_f64(clone, &mut begin), 0);
                assert_eq!(liblrhsmm_rs_hsmm_scores_get_end_f64(clone, &mut end), 0);
                let mut imported = null_mut();
                assert_eq!(
                    liblrhsmm_rs_hsmm_scores_create_f64(begin, end, &mut imported),
                    0
                );
                assert_eq!(liblrhsmm_rs_state_scores_release_f64(&mut begin), 0);
                assert_eq!(liblrhsmm_rs_state_scores_release_f64(&mut end), 0);
                assert_eq!(liblrhsmm_rs_hsmm_scores_release_f64(&mut clone), 0);
                same_scores_f64(&scores_f64(imported), &expected_forward);
                assert_eq!(liblrhsmm_rs_hsmm_scores_release_f64(&mut imported), 0);

                {
                    let mut path = null_mut();
                    let status =
                        liblrhsmm_rs_model_viterbi_hsmm_f64(model, seg, log, &options, &mut path);
                    assert_eq!(status, if expected_path.is_ok() { 0 } else { 3 });
                    if let Ok(expected) = &expected_path {
                        hsmm_path_f64(path, expected);
                    }
                    assert_eq!(liblrhsmm_rs_hsmm_path_release_f64(&mut path), 0);
                }
                let mut retained = null_mut();
                let status = liblrhsmm_rs_model_viterbi_hsmm_with_graph_f64(
                    model,
                    graph,
                    log,
                    &options,
                    &mut retained,
                );
                assert_eq!(status, if expected_path.is_ok() { 0 } else { 3 });
                {
                    let mut path = null_mut();
                    let status = liblrhsmm_rs_prepared_model_viterbi_hsmm_f64(
                        prepared, seg, log, &options, &mut path,
                    );
                    assert_eq!(status, if expected_path.is_ok() { 0 } else { 3 });
                    if let Ok(expected) = &expected_path {
                        hsmm_path_f64(path, expected);
                    }
                    assert_eq!(liblrhsmm_rs_hsmm_path_release_f64(&mut path), 0);
                }
                let mut cached_path = null_mut();
                let status = liblrhsmm_rs_prepared_model_viterbi_hsmm_with_graph_f64(
                    prepared,
                    graph,
                    log,
                    &options,
                    &mut cached_path,
                );
                assert_eq!(status, if expected_path.is_ok() { 0 } else { 3 });
                assert_eq!(liblrhsmm_rs_prepared_model_release_f64(&mut prepared), 0);
                if let Ok(expected) = &expected_path {
                    hsmm_path_f64(retained, expected);
                    hsmm_path_f64(cached_path, expected);
                    let mut clone = null_mut();
                    assert_eq!(liblrhsmm_rs_hsmm_path_clone_f64(retained, &mut clone), 0);
                    assert_eq!(liblrhsmm_rs_hsmm_path_release_f64(&mut retained), 0);
                    hsmm_path_f64(clone, expected);
                    assert_eq!(liblrhsmm_rs_hsmm_path_release_f64(&mut clone), 0);
                }
                assert_eq!(liblrhsmm_rs_hsmm_path_release_f64(&mut retained), 0);
                assert_eq!(liblrhsmm_rs_hsmm_path_release_f64(&mut cached_path), 0);
                assert_eq!(liblrhsmm_rs_hsmm_scores_release_f64(&mut forward), 0);
                assert_eq!(liblrhsmm_rs_hsmm_scores_release_f64(&mut backward), 0);
            }
            let native_geo = GeometricSettings::from(geo);
            for (function, expected) in [
                (
                    liblrhsmm_rs_model_forward_geometric_f64
                        as unsafe extern "C" fn(
                            *const LiblrhsmmRsModelF64,
                            *const LiblrhsmmRsSegmentationF64,
                            *const LiblrhsmmRsLogEmissionsF64,
                            *const LiblrhsmmRsGeometricSettingsF64,
                            *mut *mut LiblrhsmmRsStateScoresF64,
                        ) -> u32,
                    native
                        .forward_geometric(&segmentation, &emissions, native_geo)
                        .unwrap(),
                ),
                (
                    liblrhsmm_rs_model_backward_geometric_f64 as _,
                    native
                        .backward_geometric(&segmentation, &emissions, native_geo)
                        .unwrap(),
                ),
            ] {
                let mut result = null_mut();
                assert_eq!(function(model, seg, log, &geo, &mut result), 0);
                assert_eq!(bits_f64(&table_f64(result)), bits_f64(&expected));
                assert_eq!(liblrhsmm_rs_state_scores_release_f64(&mut result), 0);
            }
            let expected = native
                .viterbi_geometric(&segmentation, &emissions, native_geo)
                .unwrap();
            let mut path = null_mut();
            assert_eq!(
                liblrhsmm_rs_model_viterbi_geometric_f64(model, seg, log, &geo, &mut path),
                0
            );
            let mut cloned = null_mut();
            assert_eq!(liblrhsmm_rs_geometric_path_clone_f64(path, &mut cloned), 0);
            assert_eq!(liblrhsmm_rs_geometric_path_release_f64(&mut path), 0);
            let mut stale = std::ptr::without_provenance_mut::<LiblrhsmmRsHsmmScoresF64>(0x1000);
            let invalid = LiblrhsmmRsHsmmSettingsF64 {
                temperature: f64::NAN,
                ..options
            };
            assert_eq!(
                liblrhsmm_rs_model_forward_hsmm_f64(model, seg, log, &invalid, &mut stale),
                3
            );
            assert_eq!(stale.addr(), 0x1000);
            assert_eq!(
                liblrhsmm_rs_model_forward_hsmm_f64(null(), seg, log, &options, &mut stale),
                1
            );
            assert_eq!(
                liblrhsmm_rs_model_forward_hsmm_f64(model, seg, log, null(), &mut stale),
                1
            );
            assert_eq!(
                liblrhsmm_rs_model_forward_hsmm_f64(model, seg, log, &options, null_mut()),
                1
            );
            assert_eq!(liblrhsmm_rs_model_release_f64(&mut model), 0);
            assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut seg), 0);
            assert_eq!(liblrhsmm_rs_transition_graph_release_f64(&mut graph), 0);
            assert_eq!(liblrhsmm_rs_log_emissions_release_f64(&mut log), 0);
            geometric_path_f64(cloned, &expected);
            assert_eq!(liblrhsmm_rs_geometric_path_release_f64(&mut cloned), 0);
            assert_eq!(liblrhsmm_rs_geometric_path_release_f64(&mut cloned), 0);
            assert_eq!(liblrhsmm_rs_geometric_path_release_f64(null_mut()), 1);
            assert_eq!(liblrhsmm_rs_hsmm_path_release_f64(null_mut()), 1);
            assert_eq!(liblrhsmm_rs_hsmm_scores_release_f64(null_mut()), 1);
        }
    }
}

#[test]
fn f32_score_pair_import_errors_preserve_outputs_and_scalar_bits() {
    unsafe {
        let raw = [
            f32::from_bits(0x7fc01234),
            -0.0,
            f32::INFINITY,
            f32::NEG_INFINITY,
        ];
        let (mut begin, mut end, mut other) = (null_mut(), null_mut(), null_mut());
        assert_eq!(
            liblrhsmm_rs_state_scores_create_f32(2, 2, raw.as_ptr(), 4, &mut begin),
            0
        );
        assert_eq!(
            liblrhsmm_rs_state_scores_create_f32(2, 2, raw.as_ptr(), 4, &mut end),
            0
        );
        assert_eq!(
            liblrhsmm_rs_state_scores_create_f32(1, 4, raw.as_ptr(), 4, &mut other),
            0
        );
        let mut untouched = std::ptr::without_provenance_mut::<LiblrhsmmRsHsmmScoresF32>(0x1000);
        assert_eq!(
            liblrhsmm_rs_hsmm_scores_create_f32(begin, other, &mut untouched),
            3
        );
        assert_eq!(untouched.addr(), 0x1000);
        assert_eq!(
            liblrhsmm_rs_hsmm_scores_create_f32(null(), end, &mut untouched),
            1
        );
        assert_eq!(
            liblrhsmm_rs_hsmm_scores_create_f32(begin, end, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_hsmm_scores_create_f32(begin, end, std::ptr::without_provenance_mut(1)),
            1
        );
        let mut owner = null_mut();
        assert_eq!(
            liblrhsmm_rs_hsmm_scores_create_f32(begin, end, &mut owner),
            0
        );
        assert_eq!(liblrhsmm_rs_state_scores_release_f32(&mut begin), 0);
        assert_eq!(liblrhsmm_rs_state_scores_release_f32(&mut end), 0);
        assert_eq!(liblrhsmm_rs_state_scores_release_f32(&mut other), 0);
        assert_eq!(liblrhsmm_rs_hsmm_scores_get_begin_f32(owner, null_mut()), 1);
        assert_eq!(
            liblrhsmm_rs_hsmm_scores_get_end_f32(owner, std::ptr::without_provenance_mut(1)),
            1
        );
        let mut child = null_mut();
        assert_eq!(liblrhsmm_rs_hsmm_scores_get_end_f32(owner, &mut child), 0);
        assert_eq!(liblrhsmm_rs_hsmm_scores_release_f32(&mut owner), 0);
        let mut values = [0.0_f32; 4];
        assert_eq!(
            liblrhsmm_rs_state_scores_copy_values_f32(child, 0, values.as_mut_ptr(), 4),
            0
        );
        assert_eq!(values.map(f32::to_bits), raw.map(f32::to_bits));
        assert_eq!(liblrhsmm_rs_state_scores_release_f32(&mut child), 0);
        assert_eq!(liblrhsmm_rs_hsmm_scores_release_f32(&mut owner), 0);
    }
}

#[test]
fn f64_score_pair_import_errors_preserve_outputs_and_scalar_bits() {
    unsafe {
        let raw = [
            f64::from_bits(0x7ff8000000001234),
            -0.0,
            f64::INFINITY,
            f64::NEG_INFINITY,
        ];
        let (mut begin, mut end, mut other) = (null_mut(), null_mut(), null_mut());
        assert_eq!(
            liblrhsmm_rs_state_scores_create_f64(2, 2, raw.as_ptr(), 4, &mut begin),
            0
        );
        assert_eq!(
            liblrhsmm_rs_state_scores_create_f64(2, 2, raw.as_ptr(), 4, &mut end),
            0
        );
        assert_eq!(
            liblrhsmm_rs_state_scores_create_f64(1, 4, raw.as_ptr(), 4, &mut other),
            0
        );
        let mut untouched = std::ptr::without_provenance_mut::<LiblrhsmmRsHsmmScoresF64>(0x1000);
        assert_eq!(
            liblrhsmm_rs_hsmm_scores_create_f64(begin, other, &mut untouched),
            3
        );
        assert_eq!(untouched.addr(), 0x1000);
        assert_eq!(
            liblrhsmm_rs_hsmm_scores_create_f64(null(), end, &mut untouched),
            1
        );
        assert_eq!(
            liblrhsmm_rs_hsmm_scores_create_f64(begin, end, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_hsmm_scores_create_f64(begin, end, std::ptr::without_provenance_mut(1)),
            1
        );
        let mut owner = null_mut();
        assert_eq!(
            liblrhsmm_rs_hsmm_scores_create_f64(begin, end, &mut owner),
            0
        );
        assert_eq!(liblrhsmm_rs_state_scores_release_f64(&mut begin), 0);
        assert_eq!(liblrhsmm_rs_state_scores_release_f64(&mut end), 0);
        assert_eq!(liblrhsmm_rs_state_scores_release_f64(&mut other), 0);
        assert_eq!(liblrhsmm_rs_hsmm_scores_get_begin_f64(owner, null_mut()), 1);
        assert_eq!(
            liblrhsmm_rs_hsmm_scores_get_end_f64(owner, std::ptr::without_provenance_mut(1)),
            1
        );
        let mut child = null_mut();
        assert_eq!(liblrhsmm_rs_hsmm_scores_get_end_f64(owner, &mut child), 0);
        assert_eq!(liblrhsmm_rs_hsmm_scores_release_f64(&mut owner), 0);
        let mut values = [0.0_f64; 4];
        assert_eq!(
            liblrhsmm_rs_state_scores_copy_values_f64(child, 0, values.as_mut_ptr(), 4),
            0
        );
        assert_eq!(values.map(f64::to_bits), raw.map(f64::to_bits));
        assert_eq!(liblrhsmm_rs_state_scores_release_f64(&mut child), 0);
        assert_eq!(liblrhsmm_rs_hsmm_scores_release_f64(&mut owner), 0);
    }
}
