#![cfg(feature = "c-api")]
use liblrhsmm_rs::c_api::*;
use liblrhsmm_rs::emission::LogEmissionTable;
use liblrhsmm_rs::geometric::StateScoreTable;
use std::ptr::{null, null_mut};

unsafe fn check_log_emissions_f32(
    handle: *const LiblrhsmmRsLogEmissionsF32,
    frames: usize,
    states: usize,
    values: &[f32],
) {
    unsafe {
        let mut layout = LiblrhsmmRsInferenceTableLayout {
            frames: 99,
            states: 99,
            values: 99,
        };
        assert_eq!(
            liblrhsmm_rs_log_emissions_layout_f32(handle, &mut layout),
            0
        );
        assert_eq!(
            (layout.frames, layout.states, layout.values),
            (frames, states, values.len())
        );
        let mut copied = vec![99.0; values.len()];
        assert_eq!(
            liblrhsmm_rs_log_emissions_copy_values_f32(
                handle,
                0,
                copied.as_mut_ptr(),
                copied.len()
            ),
            0
        );
        assert_eq!(
            copied
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            values
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>()
        );
        for frame in 0..frames {
            assert_eq!(
                liblrhsmm_rs_log_emissions_copy_frame_f32(
                    handle,
                    frame,
                    copied.as_mut_ptr(),
                    states
                ),
                0
            );
            assert_eq!(
                copied[..states]
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>(),
                values[frame * states..(frame + 1) * states]
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>()
            );
            for state in 0..states {
                let mut value = 99.0;
                assert_eq!(
                    liblrhsmm_rs_log_emissions_get_f32(handle, frame, state, &mut value),
                    0
                );
                assert_eq!(value.to_bits(), values[frame * states + state].to_bits());
            }
        }
        let mut value = -77.0_f32;
        assert_eq!(
            liblrhsmm_rs_log_emissions_get_f32(handle, frames, 0, &mut value),
            2
        );
        assert_eq!(
            liblrhsmm_rs_log_emissions_get_f32(handle, 0, states, &mut value),
            2
        );
        assert_eq!(value, -77.0);
        assert_eq!(
            liblrhsmm_rs_log_emissions_get_f32(handle, 0, 0, null_mut()),
            1
        );
        for (offset, count) in [(values.len(), 1), (usize::MAX, 1), (0, usize::MAX)] {
            assert_eq!(
                liblrhsmm_rs_log_emissions_copy_values_f32(handle, offset, &mut value, count),
                2
            );
            assert_eq!(value, -77.0);
        }
        assert_eq!(
            liblrhsmm_rs_log_emissions_copy_values_f32(handle, values.len(), null_mut(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_log_emissions_copy_values_f32(handle, 0, null_mut(), 1),
            1
        );
        let misaligned = (&mut value as *mut f32).cast::<u8>().add(1).cast();
        assert_eq!(
            liblrhsmm_rs_log_emissions_copy_values_f32(handle, 0, misaligned, 1),
            1
        );
        assert_eq!(
            liblrhsmm_rs_log_emissions_copy_frame_f32(handle, frames, &mut value, states),
            2
        );
        assert_eq!(
            liblrhsmm_rs_log_emissions_copy_frame_f32(handle, 0, &mut value, states - 1),
            2
        );
        assert_eq!(value, -77.0);
    }
}

unsafe fn check_state_scores_f32(
    handle: *const LiblrhsmmRsStateScoresF32,
    frames: usize,
    states: usize,
    values: &[f32],
) {
    unsafe {
        let mut layout = LiblrhsmmRsInferenceTableLayout {
            frames: 99,
            states: 99,
            values: 99,
        };
        assert_eq!(liblrhsmm_rs_state_scores_layout_f32(handle, &mut layout), 0);
        assert_eq!(
            (layout.frames, layout.states, layout.values),
            (frames, states, values.len())
        );
        let mut copied = vec![99.0; values.len()];
        assert_eq!(
            liblrhsmm_rs_state_scores_copy_values_f32(handle, 0, copied.as_mut_ptr(), copied.len()),
            0
        );
        assert_eq!(
            copied
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            values
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>()
        );
        for frame in 0..frames {
            assert_eq!(
                liblrhsmm_rs_state_scores_copy_frame_f32(
                    handle,
                    frame,
                    copied.as_mut_ptr(),
                    states
                ),
                0
            );
            assert_eq!(
                copied[..states]
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>(),
                values[frame * states..(frame + 1) * states]
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>()
            );
            for state in 0..states {
                let mut value = 99.0;
                assert_eq!(
                    liblrhsmm_rs_state_scores_get_f32(handle, frame, state, &mut value),
                    0
                );
                assert_eq!(value.to_bits(), values[frame * states + state].to_bits());
            }
        }
        let mut value = -77.0_f32;
        assert_eq!(
            liblrhsmm_rs_state_scores_get_f32(handle, frames, 0, &mut value),
            2
        );
        assert_eq!(
            liblrhsmm_rs_state_scores_get_f32(handle, 0, states, &mut value),
            2
        );
        assert_eq!(value, -77.0);
        assert_eq!(
            liblrhsmm_rs_state_scores_get_f32(handle, 0, 0, null_mut()),
            1
        );
        for (offset, count) in [(values.len(), 1), (usize::MAX, 1), (0, usize::MAX)] {
            assert_eq!(
                liblrhsmm_rs_state_scores_copy_values_f32(handle, offset, &mut value, count),
                2
            );
            assert_eq!(value, -77.0);
        }
        assert_eq!(
            liblrhsmm_rs_state_scores_copy_values_f32(handle, values.len(), null_mut(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_state_scores_copy_values_f32(handle, 0, null_mut(), 1),
            1
        );
        let misaligned = (&mut value as *mut f32).cast::<u8>().add(1).cast();
        assert_eq!(
            liblrhsmm_rs_state_scores_copy_values_f32(handle, 0, misaligned, 1),
            1
        );
        assert_eq!(
            liblrhsmm_rs_state_scores_copy_frame_f32(handle, frames, &mut value, states),
            2
        );
        assert_eq!(
            liblrhsmm_rs_state_scores_copy_frame_f32(handle, 0, &mut value, states - 1),
            2
        );
        assert_eq!(value, -77.0);
    }
}

unsafe fn check_emission_densities_f32(
    handle: *const LiblrhsmmRsEmissionDensitiesF32,
    frames: usize,
    states: usize,
    values: &[f32],
) {
    unsafe {
        let mut layout = LiblrhsmmRsInferenceTableLayout {
            frames: 99,
            states: 99,
            values: 99,
        };
        assert_eq!(
            liblrhsmm_rs_emission_densities_layout_f32(handle, &mut layout),
            0
        );
        assert_eq!(
            (layout.frames, layout.states, layout.values),
            (frames, states, values.len())
        );
        let mut copied = vec![99.0; values.len()];
        assert_eq!(
            liblrhsmm_rs_emission_densities_copy_values_f32(
                handle,
                0,
                copied.as_mut_ptr(),
                copied.len()
            ),
            0
        );
        assert_eq!(
            copied
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            values
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>()
        );
        for frame in 0..frames {
            assert_eq!(
                liblrhsmm_rs_emission_densities_copy_frame_f32(
                    handle,
                    frame,
                    copied.as_mut_ptr(),
                    states
                ),
                0
            );
            assert_eq!(
                copied[..states]
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>(),
                values[frame * states..(frame + 1) * states]
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>()
            );
            for state in 0..states {
                let mut value = 99.0;
                assert_eq!(
                    liblrhsmm_rs_emission_densities_get_f32(handle, frame, state, &mut value),
                    0
                );
                assert_eq!(value.to_bits(), values[frame * states + state].to_bits());
            }
        }
        let mut value = -77.0_f32;
        assert_eq!(
            liblrhsmm_rs_emission_densities_get_f32(handle, frames, 0, &mut value),
            2
        );
        assert_eq!(
            liblrhsmm_rs_emission_densities_get_f32(handle, 0, states, &mut value),
            2
        );
        assert_eq!(value, -77.0);
        assert_eq!(
            liblrhsmm_rs_emission_densities_get_f32(handle, 0, 0, null_mut()),
            1
        );
        for (offset, count) in [(values.len(), 1), (usize::MAX, 1), (0, usize::MAX)] {
            assert_eq!(
                liblrhsmm_rs_emission_densities_copy_values_f32(handle, offset, &mut value, count),
                2
            );
            assert_eq!(value, -77.0);
        }
        assert_eq!(
            liblrhsmm_rs_emission_densities_copy_values_f32(handle, values.len(), null_mut(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_emission_densities_copy_values_f32(handle, 0, null_mut(), 1),
            1
        );
        let misaligned = (&mut value as *mut f32).cast::<u8>().add(1).cast();
        assert_eq!(
            liblrhsmm_rs_emission_densities_copy_values_f32(handle, 0, misaligned, 1),
            1
        );
        assert_eq!(
            liblrhsmm_rs_emission_densities_copy_frame_f32(handle, frames, &mut value, states),
            2
        );
        assert_eq!(
            liblrhsmm_rs_emission_densities_copy_frame_f32(handle, 0, &mut value, states - 1),
            2
        );
        assert_eq!(value, -77.0);
    }
}

#[test]
fn tables_f32_complete_fields_bits_copy_lifetimes_and_errors() {
    unsafe {
        let original = [
            0.0_f32,
            -0.0,
            f32::from_bits(1),
            -f32::from_bits(1),
            f32::from_bits(0x3f800001),
            -1e15,
        ];
        let native_log = LogEmissionTable::from_log_values(2, 3, original.to_vec()).unwrap();
        let native_density = native_log.probabilities();
        let raw = [
            f32::from_bits(0x7fc01234),
            -0.0,
            f32::INFINITY,
            f32::NEG_INFINITY,
            f32::from_bits(1),
            f32::from_bits(0x3f800001),
        ];
        let mut input = original;
        let (mut log, mut log_clone, mut scores, mut score_clone, mut densities, mut density_clone) = (
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
        );
        assert_eq!(
            liblrhsmm_rs_log_emissions_create_f32(2, 3, input.as_ptr(), 6, &mut log),
            0
        );
        assert_eq!(
            liblrhsmm_rs_state_scores_create_f32(2, 3, raw.as_ptr(), 6, &mut scores),
            0
        );
        assert_eq!(liblrhsmm_rs_log_emissions_clone_f32(log, &mut log_clone), 0);
        assert_eq!(
            liblrhsmm_rs_state_scores_clone_f32(scores, &mut score_clone),
            0
        );
        assert_eq!(
            liblrhsmm_rs_log_emissions_probabilities_f32(log, &mut densities),
            0
        );
        assert_eq!(
            liblrhsmm_rs_emission_densities_clone_f32(densities, &mut density_clone),
            0
        );
        input.fill(99.0);
        check_log_emissions_f32(log, 2, 3, &original);
        check_state_scores_f32(scores, 2, 3, &raw);
        check_emission_densities_f32(densities, 2, 3, native_density.values());
        assert_eq!(
            liblrhsmm_rs_log_emissions_replace_f32(log, 3, 2, input.as_ptr(), 6),
            0
        );
        assert_eq!(
            liblrhsmm_rs_state_scores_replace_f32(scores, 3, 2, raw.as_ptr(), 6),
            0
        );
        check_log_emissions_f32(log, 3, 2, &input);
        check_state_scores_f32(scores, 3, 2, &raw);
        for (frames, states, count, status) in [
            (0, 3, 0, 3),
            (2, 0, 0, 3),
            (2, 4, 6, 3),
            (usize::MAX, 1, 0, 2),
        ] {
            let mut sentinel_log = log;
            let mut sentinel_scores = scores;
            assert_eq!(
                liblrhsmm_rs_log_emissions_create_f32(
                    frames,
                    states,
                    input.as_ptr(),
                    count,
                    &mut sentinel_log
                ),
                status
            );
            assert_eq!(
                liblrhsmm_rs_state_scores_create_f32(
                    frames,
                    states,
                    input.as_ptr(),
                    count,
                    &mut sentinel_scores
                ),
                status
            );
            assert_eq!(sentinel_log, log);
            assert_eq!(sentinel_scores, scores);
            assert_eq!(
                liblrhsmm_rs_log_emissions_replace_f32(log, frames, states, input.as_ptr(), count),
                status
            );
            assert_eq!(
                liblrhsmm_rs_state_scores_replace_f32(
                    scores,
                    frames,
                    states,
                    input.as_ptr(),
                    count
                ),
                status
            );
        }
        for invalid in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            let mut invalid_values = input;
            invalid_values[5] = invalid;
            let mut sentinel = log;
            assert_eq!(
                liblrhsmm_rs_log_emissions_create_f32(
                    3,
                    2,
                    invalid_values.as_ptr(),
                    6,
                    &mut sentinel
                ),
                3
            );
            assert_eq!(sentinel, log);
            assert_eq!(
                liblrhsmm_rs_log_emissions_replace_f32(log, 3, 2, invalid_values.as_ptr(), 6),
                3
            );
            check_log_emissions_f32(log, 3, 2, &input);
        }
        assert_eq!(
            liblrhsmm_rs_log_emissions_create_f32(2, 3, null(), 6, &mut log),
            1
        );
        assert_eq!(
            liblrhsmm_rs_state_scores_create_f32(2, 3, null(), 6, &mut scores),
            1
        );
        assert_eq!(
            liblrhsmm_rs_log_emissions_probabilities_f32(log, null_mut()),
            1
        );
        assert_eq!(liblrhsmm_rs_log_emissions_release_f32(&mut log), 0);
        assert_eq!(liblrhsmm_rs_state_scores_release_f32(&mut scores), 0);
        assert_eq!(
            liblrhsmm_rs_emission_densities_release_f32(&mut densities),
            0
        );
        check_log_emissions_f32(log_clone, 2, 3, &original);
        check_state_scores_f32(score_clone, 2, 3, &raw);
        check_emission_densities_f32(density_clone, 2, 3, native_density.values());
        assert_eq!(liblrhsmm_rs_log_emissions_release_f32(&mut log_clone), 0);
        assert_eq!(liblrhsmm_rs_state_scores_release_f32(&mut score_clone), 0);
        assert_eq!(
            liblrhsmm_rs_emission_densities_release_f32(&mut density_clone),
            0
        );
        assert_eq!(liblrhsmm_rs_log_emissions_release_f32(&mut log_clone), 0);
        assert_eq!(liblrhsmm_rs_state_scores_release_f32(&mut score_clone), 0);
        assert_eq!(
            liblrhsmm_rs_emission_densities_release_f32(&mut density_clone),
            0
        );
        assert_eq!(liblrhsmm_rs_log_emissions_release_f32(null_mut()), 1);
        assert_eq!(liblrhsmm_rs_state_scores_release_f32(null_mut()), 1);
        assert_eq!(liblrhsmm_rs_emission_densities_release_f32(null_mut()), 1);
        let forward_values = [-0.5_f32, -1.0, -1.5, -2.0, -2.5, -3.0];
        let backward_values = [-3.0_f32, -2.5, -2.0, -1.5, -1.0, -0.5];
        let forward = StateScoreTable::from_values(2, 3, forward_values.to_vec()).unwrap();
        let backward = StateScoreTable::from_values(2, 3, backward_values.to_vec()).unwrap();
        assert_eq!(
            liblrhsmm_rs_state_scores_create_f32(2, 3, forward_values.as_ptr(), 6, &mut scores),
            0
        );
        assert_eq!(
            liblrhsmm_rs_state_scores_create_f32(
                2,
                3,
                backward_values.as_ptr(),
                6,
                &mut score_clone
            ),
            0
        );
        assert_eq!(
            liblrhsmm_rs_log_emissions_create_f32(2, 3, original.as_ptr(), 6, &mut log),
            0
        );
        let mut total = 99.0_f32;
        assert_eq!(liblrhsmm_rs_state_scores_total_f32(scores, &mut total), 0);
        assert_eq!(total.to_bits(), forward.total_log_probability().to_bits());
        for state in 0..3 {
            assert_eq!(
                liblrhsmm_rs_state_scores_total_through_state_f32(
                    scores,
                    score_clone,
                    state,
                    &mut total
                ),
                0
            );
            assert_eq!(
                total.to_bits(),
                forward
                    .total_through_state(&backward, state)
                    .unwrap()
                    .to_bits()
            );
        }
        assert_eq!(
            liblrhsmm_rs_state_scores_backward_total_f32(score_clone, log, &mut total),
            0
        );
        assert_eq!(
            total.to_bits(),
            backward
                .backward_log_probability(&native_log)
                .unwrap()
                .to_bits()
        );
        total = 77.0;
        assert_eq!(
            liblrhsmm_rs_state_scores_total_through_state_f32(scores, score_clone, 3, &mut total),
            3
        );
        assert_eq!(total, 77.0);
        assert_eq!(
            liblrhsmm_rs_log_emissions_replace_f32(log, 3, 2, original.as_ptr(), 6),
            0
        );
        assert_eq!(
            liblrhsmm_rs_state_scores_backward_total_f32(score_clone, log, &mut total),
            3
        );
        assert_eq!(total, 77.0);
        assert_eq!(liblrhsmm_rs_state_scores_total_f32(scores, null_mut()), 1);
        assert_eq!(liblrhsmm_rs_state_scores_release_f32(&mut scores), 0);
        assert_eq!(liblrhsmm_rs_state_scores_release_f32(&mut score_clone), 0);
        assert_eq!(liblrhsmm_rs_log_emissions_release_f32(&mut log), 0);
    }
}

unsafe fn check_log_emissions_f64(
    handle: *const LiblrhsmmRsLogEmissionsF64,
    frames: usize,
    states: usize,
    values: &[f64],
) {
    unsafe {
        let mut layout = LiblrhsmmRsInferenceTableLayout {
            frames: 99,
            states: 99,
            values: 99,
        };
        assert_eq!(
            liblrhsmm_rs_log_emissions_layout_f64(handle, &mut layout),
            0
        );
        assert_eq!(
            (layout.frames, layout.states, layout.values),
            (frames, states, values.len())
        );
        let mut copied = vec![99.0; values.len()];
        assert_eq!(
            liblrhsmm_rs_log_emissions_copy_values_f64(
                handle,
                0,
                copied.as_mut_ptr(),
                copied.len()
            ),
            0
        );
        assert_eq!(
            copied
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            values
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>()
        );
        for frame in 0..frames {
            assert_eq!(
                liblrhsmm_rs_log_emissions_copy_frame_f64(
                    handle,
                    frame,
                    copied.as_mut_ptr(),
                    states
                ),
                0
            );
            assert_eq!(
                copied[..states]
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>(),
                values[frame * states..(frame + 1) * states]
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>()
            );
            for state in 0..states {
                let mut value = 99.0;
                assert_eq!(
                    liblrhsmm_rs_log_emissions_get_f64(handle, frame, state, &mut value),
                    0
                );
                assert_eq!(value.to_bits(), values[frame * states + state].to_bits());
            }
        }
        let mut value = -77.0_f64;
        assert_eq!(
            liblrhsmm_rs_log_emissions_get_f64(handle, frames, 0, &mut value),
            2
        );
        assert_eq!(
            liblrhsmm_rs_log_emissions_get_f64(handle, 0, states, &mut value),
            2
        );
        assert_eq!(value, -77.0);
        assert_eq!(
            liblrhsmm_rs_log_emissions_get_f64(handle, 0, 0, null_mut()),
            1
        );
        for (offset, count) in [(values.len(), 1), (usize::MAX, 1), (0, usize::MAX)] {
            assert_eq!(
                liblrhsmm_rs_log_emissions_copy_values_f64(handle, offset, &mut value, count),
                2
            );
            assert_eq!(value, -77.0);
        }
        assert_eq!(
            liblrhsmm_rs_log_emissions_copy_values_f64(handle, values.len(), null_mut(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_log_emissions_copy_values_f64(handle, 0, null_mut(), 1),
            1
        );
        let misaligned = (&mut value as *mut f64).cast::<u8>().add(1).cast();
        assert_eq!(
            liblrhsmm_rs_log_emissions_copy_values_f64(handle, 0, misaligned, 1),
            1
        );
        assert_eq!(
            liblrhsmm_rs_log_emissions_copy_frame_f64(handle, frames, &mut value, states),
            2
        );
        assert_eq!(
            liblrhsmm_rs_log_emissions_copy_frame_f64(handle, 0, &mut value, states - 1),
            2
        );
        assert_eq!(value, -77.0);
    }
}

unsafe fn check_state_scores_f64(
    handle: *const LiblrhsmmRsStateScoresF64,
    frames: usize,
    states: usize,
    values: &[f64],
) {
    unsafe {
        let mut layout = LiblrhsmmRsInferenceTableLayout {
            frames: 99,
            states: 99,
            values: 99,
        };
        assert_eq!(liblrhsmm_rs_state_scores_layout_f64(handle, &mut layout), 0);
        assert_eq!(
            (layout.frames, layout.states, layout.values),
            (frames, states, values.len())
        );
        let mut copied = vec![99.0; values.len()];
        assert_eq!(
            liblrhsmm_rs_state_scores_copy_values_f64(handle, 0, copied.as_mut_ptr(), copied.len()),
            0
        );
        assert_eq!(
            copied
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            values
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>()
        );
        for frame in 0..frames {
            assert_eq!(
                liblrhsmm_rs_state_scores_copy_frame_f64(
                    handle,
                    frame,
                    copied.as_mut_ptr(),
                    states
                ),
                0
            );
            assert_eq!(
                copied[..states]
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>(),
                values[frame * states..(frame + 1) * states]
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>()
            );
            for state in 0..states {
                let mut value = 99.0;
                assert_eq!(
                    liblrhsmm_rs_state_scores_get_f64(handle, frame, state, &mut value),
                    0
                );
                assert_eq!(value.to_bits(), values[frame * states + state].to_bits());
            }
        }
        let mut value = -77.0_f64;
        assert_eq!(
            liblrhsmm_rs_state_scores_get_f64(handle, frames, 0, &mut value),
            2
        );
        assert_eq!(
            liblrhsmm_rs_state_scores_get_f64(handle, 0, states, &mut value),
            2
        );
        assert_eq!(value, -77.0);
        assert_eq!(
            liblrhsmm_rs_state_scores_get_f64(handle, 0, 0, null_mut()),
            1
        );
        for (offset, count) in [(values.len(), 1), (usize::MAX, 1), (0, usize::MAX)] {
            assert_eq!(
                liblrhsmm_rs_state_scores_copy_values_f64(handle, offset, &mut value, count),
                2
            );
            assert_eq!(value, -77.0);
        }
        assert_eq!(
            liblrhsmm_rs_state_scores_copy_values_f64(handle, values.len(), null_mut(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_state_scores_copy_values_f64(handle, 0, null_mut(), 1),
            1
        );
        let misaligned = (&mut value as *mut f64).cast::<u8>().add(1).cast();
        assert_eq!(
            liblrhsmm_rs_state_scores_copy_values_f64(handle, 0, misaligned, 1),
            1
        );
        assert_eq!(
            liblrhsmm_rs_state_scores_copy_frame_f64(handle, frames, &mut value, states),
            2
        );
        assert_eq!(
            liblrhsmm_rs_state_scores_copy_frame_f64(handle, 0, &mut value, states - 1),
            2
        );
        assert_eq!(value, -77.0);
    }
}

unsafe fn check_emission_densities_f64(
    handle: *const LiblrhsmmRsEmissionDensitiesF64,
    frames: usize,
    states: usize,
    values: &[f64],
) {
    unsafe {
        let mut layout = LiblrhsmmRsInferenceTableLayout {
            frames: 99,
            states: 99,
            values: 99,
        };
        assert_eq!(
            liblrhsmm_rs_emission_densities_layout_f64(handle, &mut layout),
            0
        );
        assert_eq!(
            (layout.frames, layout.states, layout.values),
            (frames, states, values.len())
        );
        let mut copied = vec![99.0; values.len()];
        assert_eq!(
            liblrhsmm_rs_emission_densities_copy_values_f64(
                handle,
                0,
                copied.as_mut_ptr(),
                copied.len()
            ),
            0
        );
        assert_eq!(
            copied
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            values
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>()
        );
        for frame in 0..frames {
            assert_eq!(
                liblrhsmm_rs_emission_densities_copy_frame_f64(
                    handle,
                    frame,
                    copied.as_mut_ptr(),
                    states
                ),
                0
            );
            assert_eq!(
                copied[..states]
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>(),
                values[frame * states..(frame + 1) * states]
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>()
            );
            for state in 0..states {
                let mut value = 99.0;
                assert_eq!(
                    liblrhsmm_rs_emission_densities_get_f64(handle, frame, state, &mut value),
                    0
                );
                assert_eq!(value.to_bits(), values[frame * states + state].to_bits());
            }
        }
        let mut value = -77.0_f64;
        assert_eq!(
            liblrhsmm_rs_emission_densities_get_f64(handle, frames, 0, &mut value),
            2
        );
        assert_eq!(
            liblrhsmm_rs_emission_densities_get_f64(handle, 0, states, &mut value),
            2
        );
        assert_eq!(value, -77.0);
        assert_eq!(
            liblrhsmm_rs_emission_densities_get_f64(handle, 0, 0, null_mut()),
            1
        );
        for (offset, count) in [(values.len(), 1), (usize::MAX, 1), (0, usize::MAX)] {
            assert_eq!(
                liblrhsmm_rs_emission_densities_copy_values_f64(handle, offset, &mut value, count),
                2
            );
            assert_eq!(value, -77.0);
        }
        assert_eq!(
            liblrhsmm_rs_emission_densities_copy_values_f64(handle, values.len(), null_mut(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_emission_densities_copy_values_f64(handle, 0, null_mut(), 1),
            1
        );
        let misaligned = (&mut value as *mut f64).cast::<u8>().add(1).cast();
        assert_eq!(
            liblrhsmm_rs_emission_densities_copy_values_f64(handle, 0, misaligned, 1),
            1
        );
        assert_eq!(
            liblrhsmm_rs_emission_densities_copy_frame_f64(handle, frames, &mut value, states),
            2
        );
        assert_eq!(
            liblrhsmm_rs_emission_densities_copy_frame_f64(handle, 0, &mut value, states - 1),
            2
        );
        assert_eq!(value, -77.0);
    }
}

#[test]
fn tables_f64_complete_fields_bits_copy_lifetimes_and_errors() {
    unsafe {
        let original = [
            0.0_f64,
            -0.0,
            f64::from_bits(1),
            -f64::from_bits(1),
            f64::from_bits(0x3ff0000000000001),
            -1e15,
        ];
        let native_log = LogEmissionTable::from_log_values(2, 3, original.to_vec()).unwrap();
        let native_density = native_log.probabilities();
        let raw = [
            f64::from_bits(0x7ff8000000001234),
            -0.0,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::from_bits(1),
            f64::from_bits(0x3ff0000000000001),
        ];
        let mut input = original;
        let (mut log, mut log_clone, mut scores, mut score_clone, mut densities, mut density_clone) = (
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
        );
        assert_eq!(
            liblrhsmm_rs_log_emissions_create_f64(2, 3, input.as_ptr(), 6, &mut log),
            0
        );
        assert_eq!(
            liblrhsmm_rs_state_scores_create_f64(2, 3, raw.as_ptr(), 6, &mut scores),
            0
        );
        assert_eq!(liblrhsmm_rs_log_emissions_clone_f64(log, &mut log_clone), 0);
        assert_eq!(
            liblrhsmm_rs_state_scores_clone_f64(scores, &mut score_clone),
            0
        );
        assert_eq!(
            liblrhsmm_rs_log_emissions_probabilities_f64(log, &mut densities),
            0
        );
        assert_eq!(
            liblrhsmm_rs_emission_densities_clone_f64(densities, &mut density_clone),
            0
        );
        input.fill(99.0);
        check_log_emissions_f64(log, 2, 3, &original);
        check_state_scores_f64(scores, 2, 3, &raw);
        check_emission_densities_f64(densities, 2, 3, native_density.values());
        assert_eq!(
            liblrhsmm_rs_log_emissions_replace_f64(log, 3, 2, input.as_ptr(), 6),
            0
        );
        assert_eq!(
            liblrhsmm_rs_state_scores_replace_f64(scores, 3, 2, raw.as_ptr(), 6),
            0
        );
        check_log_emissions_f64(log, 3, 2, &input);
        check_state_scores_f64(scores, 3, 2, &raw);
        for (frames, states, count, status) in [
            (0, 3, 0, 3),
            (2, 0, 0, 3),
            (2, 4, 6, 3),
            (usize::MAX, 1, 0, 2),
        ] {
            let mut sentinel_log = log;
            let mut sentinel_scores = scores;
            assert_eq!(
                liblrhsmm_rs_log_emissions_create_f64(
                    frames,
                    states,
                    input.as_ptr(),
                    count,
                    &mut sentinel_log
                ),
                status
            );
            assert_eq!(
                liblrhsmm_rs_state_scores_create_f64(
                    frames,
                    states,
                    input.as_ptr(),
                    count,
                    &mut sentinel_scores
                ),
                status
            );
            assert_eq!(sentinel_log, log);
            assert_eq!(sentinel_scores, scores);
            assert_eq!(
                liblrhsmm_rs_log_emissions_replace_f64(log, frames, states, input.as_ptr(), count),
                status
            );
            assert_eq!(
                liblrhsmm_rs_state_scores_replace_f64(
                    scores,
                    frames,
                    states,
                    input.as_ptr(),
                    count
                ),
                status
            );
        }
        for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let mut invalid_values = input;
            invalid_values[5] = invalid;
            let mut sentinel = log;
            assert_eq!(
                liblrhsmm_rs_log_emissions_create_f64(
                    3,
                    2,
                    invalid_values.as_ptr(),
                    6,
                    &mut sentinel
                ),
                3
            );
            assert_eq!(sentinel, log);
            assert_eq!(
                liblrhsmm_rs_log_emissions_replace_f64(log, 3, 2, invalid_values.as_ptr(), 6),
                3
            );
            check_log_emissions_f64(log, 3, 2, &input);
        }
        assert_eq!(
            liblrhsmm_rs_log_emissions_create_f64(2, 3, null(), 6, &mut log),
            1
        );
        assert_eq!(
            liblrhsmm_rs_state_scores_create_f64(2, 3, null(), 6, &mut scores),
            1
        );
        assert_eq!(
            liblrhsmm_rs_log_emissions_probabilities_f64(log, null_mut()),
            1
        );
        assert_eq!(liblrhsmm_rs_log_emissions_release_f64(&mut log), 0);
        assert_eq!(liblrhsmm_rs_state_scores_release_f64(&mut scores), 0);
        assert_eq!(
            liblrhsmm_rs_emission_densities_release_f64(&mut densities),
            0
        );
        check_log_emissions_f64(log_clone, 2, 3, &original);
        check_state_scores_f64(score_clone, 2, 3, &raw);
        check_emission_densities_f64(density_clone, 2, 3, native_density.values());
        assert_eq!(liblrhsmm_rs_log_emissions_release_f64(&mut log_clone), 0);
        assert_eq!(liblrhsmm_rs_state_scores_release_f64(&mut score_clone), 0);
        assert_eq!(
            liblrhsmm_rs_emission_densities_release_f64(&mut density_clone),
            0
        );
        assert_eq!(liblrhsmm_rs_log_emissions_release_f64(&mut log_clone), 0);
        assert_eq!(liblrhsmm_rs_state_scores_release_f64(&mut score_clone), 0);
        assert_eq!(
            liblrhsmm_rs_emission_densities_release_f64(&mut density_clone),
            0
        );
        assert_eq!(liblrhsmm_rs_log_emissions_release_f64(null_mut()), 1);
        assert_eq!(liblrhsmm_rs_state_scores_release_f64(null_mut()), 1);
        assert_eq!(liblrhsmm_rs_emission_densities_release_f64(null_mut()), 1);
        let forward_values = [-0.5_f64, -1.0, -1.5, -2.0, -2.5, -3.0];
        let backward_values = [-3.0_f64, -2.5, -2.0, -1.5, -1.0, -0.5];
        let forward = StateScoreTable::from_values(2, 3, forward_values.to_vec()).unwrap();
        let backward = StateScoreTable::from_values(2, 3, backward_values.to_vec()).unwrap();
        assert_eq!(
            liblrhsmm_rs_state_scores_create_f64(2, 3, forward_values.as_ptr(), 6, &mut scores),
            0
        );
        assert_eq!(
            liblrhsmm_rs_state_scores_create_f64(
                2,
                3,
                backward_values.as_ptr(),
                6,
                &mut score_clone
            ),
            0
        );
        assert_eq!(
            liblrhsmm_rs_log_emissions_create_f64(2, 3, original.as_ptr(), 6, &mut log),
            0
        );
        let mut total = 99.0_f64;
        assert_eq!(liblrhsmm_rs_state_scores_total_f64(scores, &mut total), 0);
        assert_eq!(total.to_bits(), forward.total_log_probability().to_bits());
        for state in 0..3 {
            assert_eq!(
                liblrhsmm_rs_state_scores_total_through_state_f64(
                    scores,
                    score_clone,
                    state,
                    &mut total
                ),
                0
            );
            assert_eq!(
                total.to_bits(),
                forward
                    .total_through_state(&backward, state)
                    .unwrap()
                    .to_bits()
            );
        }
        assert_eq!(
            liblrhsmm_rs_state_scores_backward_total_f64(score_clone, log, &mut total),
            0
        );
        assert_eq!(
            total.to_bits(),
            backward
                .backward_log_probability(&native_log)
                .unwrap()
                .to_bits()
        );
        total = 77.0;
        assert_eq!(
            liblrhsmm_rs_state_scores_total_through_state_f64(scores, score_clone, 3, &mut total),
            3
        );
        assert_eq!(total, 77.0);
        assert_eq!(
            liblrhsmm_rs_log_emissions_replace_f64(log, 3, 2, original.as_ptr(), 6),
            0
        );
        assert_eq!(
            liblrhsmm_rs_state_scores_backward_total_f64(score_clone, log, &mut total),
            3
        );
        assert_eq!(total, 77.0);
        assert_eq!(liblrhsmm_rs_state_scores_total_f64(scores, null_mut()), 1);
        assert_eq!(liblrhsmm_rs_state_scores_release_f64(&mut scores), 0);
        assert_eq!(liblrhsmm_rs_state_scores_release_f64(&mut score_clone), 0);
        assert_eq!(liblrhsmm_rs_log_emissions_release_f64(&mut log), 0);
    }
}

#[test]
fn tables_f32_totals_from_original_c_score_corpus() {
    let fixture = include_str!("fixtures/c-geometric.txt");
    let mut records = 0;
    for line in fixture.lines().filter(|line| line.starts_with("total ")) {
        let fields: Vec<_> = line.split_whitespace().collect();
        let states: usize = fields[1].parse().unwrap();
        let frames: usize = fields[2].parse().unwrap();
        let state: usize = fields[5].parse().unwrap();
        let mut forward_values = vec![0.0_f32; states * frames];
        let mut backward_values = forward_values.clone();
        let mut imported = 0;
        for source in fixture
            .lines()
            .filter(|source| source.starts_with("scores "))
        {
            let values: Vec<_> = source.split_whitespace().collect();
            if values[1..5] != fields[1..5] {
                continue;
            }
            let index: usize = values[5].parse().unwrap();
            forward_values[index] = values[6].parse().unwrap();
            backward_values[index] = values[7].parse().unwrap();
            imported += 1;
        }
        assert_eq!(imported, frames * states);
        let emissions: Vec<f32> = (0..frames)
            .flat_map(|frame| {
                (0..states).map(move |state| {
                    let residual = frame as f32 - state as f32 * 4.0;
                    -residual * residual * 0.25
                })
            })
            .collect();
        unsafe {
            let (mut forward, mut backward, mut log) = (null_mut(), null_mut(), null_mut());
            assert_eq!(
                liblrhsmm_rs_state_scores_create_f32(
                    frames,
                    states,
                    forward_values.as_ptr(),
                    forward_values.len(),
                    &mut forward
                ),
                0
            );
            assert_eq!(
                liblrhsmm_rs_state_scores_create_f32(
                    frames,
                    states,
                    backward_values.as_ptr(),
                    backward_values.len(),
                    &mut backward
                ),
                0
            );
            assert_eq!(
                liblrhsmm_rs_log_emissions_create_f32(
                    frames,
                    states,
                    emissions.as_ptr(),
                    emissions.len(),
                    &mut log
                ),
                0
            );
            let mut totals = [0.0_f32; 3];
            assert_eq!(
                liblrhsmm_rs_state_scores_total_through_state_f32(
                    forward,
                    backward,
                    state,
                    &mut totals[0]
                ),
                0
            );
            assert_eq!(
                liblrhsmm_rs_state_scores_total_f32(forward, &mut totals[1]),
                0
            );
            assert_eq!(
                liblrhsmm_rs_state_scores_backward_total_f32(backward, log, &mut totals[2]),
                0
            );
            for (actual, expected) in totals.into_iter().zip(&fields[6..]) {
                let expected: f32 = expected.parse().unwrap();
                let actual: f64 = actual.into();
                let expected: f64 = expected.into();
                assert!(
                    (actual - expected).abs() / expected.abs().max(1.0) <= 0.0,
                    "{line}"
                );
            }
            assert_eq!(liblrhsmm_rs_state_scores_release_f32(&mut forward), 0);
            assert_eq!(liblrhsmm_rs_state_scores_release_f32(&mut backward), 0);
            assert_eq!(liblrhsmm_rs_log_emissions_release_f32(&mut log), 0);
        }
        records += 1;
    }
    assert_eq!(records, 14);
}

#[test]
fn tables_f64_totals_from_original_c_score_corpus() {
    let fixture = include_str!("fixtures/c-double-geometric.txt");
    let mut records = 0;
    for line in fixture.lines().filter(|line| line.starts_with("total ")) {
        let fields: Vec<_> = line.split_whitespace().collect();
        let states: usize = fields[1].parse().unwrap();
        let frames: usize = fields[2].parse().unwrap();
        let state: usize = fields[5].parse().unwrap();
        let mut forward_values = vec![0.0_f64; states * frames];
        let mut backward_values = forward_values.clone();
        let mut imported = 0;
        for source in fixture
            .lines()
            .filter(|source| source.starts_with("scores "))
        {
            let values: Vec<_> = source.split_whitespace().collect();
            if values[1..5] != fields[1..5] {
                continue;
            }
            let index: usize = values[5].parse().unwrap();
            forward_values[index] = values[6].parse().unwrap();
            backward_values[index] = values[7].parse().unwrap();
            imported += 1;
        }
        assert_eq!(imported, frames * states);
        let emissions: Vec<f64> = (0..frames)
            .flat_map(|frame| {
                (0..states).map(move |state| {
                    let residual = frame as f64 - state as f64 * 4.0;
                    -residual * residual * 0.25
                })
            })
            .collect();
        unsafe {
            let (mut forward, mut backward, mut log) = (null_mut(), null_mut(), null_mut());
            assert_eq!(
                liblrhsmm_rs_state_scores_create_f64(
                    frames,
                    states,
                    forward_values.as_ptr(),
                    forward_values.len(),
                    &mut forward
                ),
                0
            );
            assert_eq!(
                liblrhsmm_rs_state_scores_create_f64(
                    frames,
                    states,
                    backward_values.as_ptr(),
                    backward_values.len(),
                    &mut backward
                ),
                0
            );
            assert_eq!(
                liblrhsmm_rs_log_emissions_create_f64(
                    frames,
                    states,
                    emissions.as_ptr(),
                    emissions.len(),
                    &mut log
                ),
                0
            );
            let mut totals = [0.0_f64; 3];
            assert_eq!(
                liblrhsmm_rs_state_scores_total_through_state_f64(
                    forward,
                    backward,
                    state,
                    &mut totals[0]
                ),
                0
            );
            assert_eq!(
                liblrhsmm_rs_state_scores_total_f64(forward, &mut totals[1]),
                0
            );
            assert_eq!(
                liblrhsmm_rs_state_scores_backward_total_f64(backward, log, &mut totals[2]),
                0
            );
            for (actual, expected) in totals.into_iter().zip(&fields[6..]) {
                let expected: f64 = expected.parse().unwrap();
                assert!(
                    (actual - expected).abs() / expected.abs().max(1.0) <= 2e-14,
                    "{line}"
                );
            }
            assert_eq!(liblrhsmm_rs_state_scores_release_f64(&mut forward), 0);
            assert_eq!(liblrhsmm_rs_state_scores_release_f64(&mut backward), 0);
            assert_eq!(liblrhsmm_rs_log_emissions_release_f64(&mut log), 0);
        }
        records += 1;
    }
    assert_eq!(records, 14);
}
