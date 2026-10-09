use liblrhsmm_rs::data::{ObservationData, SegmentationData};
use liblrhsmm_rs_capi::*;
use std::ptr::{null, null_mut};
unsafe fn compare_observation_f32(
    owner: *const LiblrhsmmRsObservationF32,
    expected: &ObservationData<f32>,
) {
    // SAFETY: Live readable owner and independent initialized caller buffers.
    unsafe {
        let mut layout = LiblrhsmmRsObservationLayout {
            frames: 99,
            streams: 99,
        };
        assert_eq!(liblrhsmm_rs_observation_layout_f32(owner, &mut layout), 0);
        assert_eq!(layout.frames, expected.frames);
        assert_eq!(layout.streams, expected.streams.len());
        for (index, stream) in expected.streams.iter().enumerate() {
            let mut shape = LiblrhsmmRsObservationStreamLayout {
                frames: 99,
                dimensions: 99,
                values: 99,
            };
            assert_eq!(
                liblrhsmm_rs_observation_get_stream_layout_f32(owner, index, &mut shape),
                0
            );
            assert_eq!(
                shape,
                LiblrhsmmRsObservationStreamLayout {
                    frames: expected.frames,
                    dimensions: stream.dimensions,
                    values: stream.values.len()
                }
            );
            let mut values = vec![99.0; stream.values.len()];
            assert_eq!(
                liblrhsmm_rs_observation_copy_values_f32(
                    owner,
                    index,
                    0,
                    values.as_mut_ptr(),
                    values.len()
                ),
                0
            );
            assert_eq!(
                values
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>(),
                stream
                    .values
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>()
            );
        }
    }
}

unsafe fn compare_observation_f64(
    owner: *const LiblrhsmmRsObservationF64,
    expected: &ObservationData<f64>,
) {
    // SAFETY: Live readable owner and independent initialized caller buffers.
    unsafe {
        let mut layout = LiblrhsmmRsObservationLayout {
            frames: 99,
            streams: 99,
        };
        assert_eq!(liblrhsmm_rs_observation_layout_f64(owner, &mut layout), 0);
        assert_eq!(layout.frames, expected.frames);
        assert_eq!(layout.streams, expected.streams.len());
        for (index, stream) in expected.streams.iter().enumerate() {
            let mut shape = LiblrhsmmRsObservationStreamLayout {
                frames: 99,
                dimensions: 99,
                values: 99,
            };
            assert_eq!(
                liblrhsmm_rs_observation_get_stream_layout_f64(owner, index, &mut shape),
                0
            );
            assert_eq!(
                shape,
                LiblrhsmmRsObservationStreamLayout {
                    frames: expected.frames,
                    dimensions: stream.dimensions,
                    values: stream.values.len()
                }
            );
            let mut values = vec![99.0; stream.values.len()];
            assert_eq!(
                liblrhsmm_rs_observation_copy_values_f64(
                    owner,
                    index,
                    0,
                    values.as_mut_ptr(),
                    values.len()
                ),
                0
            );
            assert_eq!(
                values
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>(),
                stream
                    .values
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>()
            );
        }
    }
}

unsafe fn compare_segmentation_f32(
    owner: *const LiblrhsmmRsSegmentationF32,
    expected: &SegmentationData<f32>,
) {
    // SAFETY: Live readable owner and independent initialized caller buffers.
    unsafe {
        let mut layout = LiblrhsmmRsSegmentationLayout {
            streams: 99,
            segments: 99,
        };
        assert_eq!(liblrhsmm_rs_segmentation_layout_f32(owner, &mut layout), 0);
        assert_eq!(layout.streams, expected.output_states.len());
        assert_eq!(layout.segments, expected.boundaries.len());
        let mut values = vec![99; layout.segments];
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_boundaries_f32(
                owner,
                0,
                values.as_mut_ptr(),
                values.len()
            ),
            0
        );
        assert_eq!(values, expected.boundaries);
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_duration_states_f32(
                owner,
                0,
                values.as_mut_ptr(),
                values.len()
            ),
            0
        );
        assert_eq!(values, expected.duration_states);
        for (stream, states) in expected.output_states.iter().enumerate() {
            assert_eq!(
                liblrhsmm_rs_segmentation_copy_output_states_f32(
                    owner,
                    stream,
                    0,
                    values.as_mut_ptr(),
                    values.len()
                ),
                0
            );
            assert_eq!(&values, states);
        }
        for (state, expected) in expected.outgoing.iter().enumerate() {
            let mut count = 99;
            assert_eq!(
                liblrhsmm_rs_segmentation_outgoing_count_f32(owner, state, &mut count),
                0
            );
            assert_eq!(count, expected.len());
            let mut jumps = vec![
                LiblrhsmmRsJumpF32 {
                    delta: 99,
                    probability: 99.0
                };
                count
            ];
            assert_eq!(
                liblrhsmm_rs_segmentation_copy_outgoing_f32(
                    owner,
                    state,
                    0,
                    jumps.as_mut_ptr(),
                    count
                ),
                0
            );
            for (actual, expected) in jumps.iter().zip(expected) {
                assert_eq!(actual.delta, expected.delta);
                assert_eq!(actual.probability.to_bits(), expected.probability.to_bits());
            }
        }
    }
}

unsafe fn compare_segmentation_f64(
    owner: *const LiblrhsmmRsSegmentationF64,
    expected: &SegmentationData<f64>,
) {
    // SAFETY: Live readable owner and independent initialized caller buffers.
    unsafe {
        let mut layout = LiblrhsmmRsSegmentationLayout {
            streams: 99,
            segments: 99,
        };
        assert_eq!(liblrhsmm_rs_segmentation_layout_f64(owner, &mut layout), 0);
        assert_eq!(layout.streams, expected.output_states.len());
        assert_eq!(layout.segments, expected.boundaries.len());
        let mut values = vec![99; layout.segments];
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_boundaries_f64(
                owner,
                0,
                values.as_mut_ptr(),
                values.len()
            ),
            0
        );
        assert_eq!(values, expected.boundaries);
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_duration_states_f64(
                owner,
                0,
                values.as_mut_ptr(),
                values.len()
            ),
            0
        );
        assert_eq!(values, expected.duration_states);
        for (stream, states) in expected.output_states.iter().enumerate() {
            assert_eq!(
                liblrhsmm_rs_segmentation_copy_output_states_f64(
                    owner,
                    stream,
                    0,
                    values.as_mut_ptr(),
                    values.len()
                ),
                0
            );
            assert_eq!(&values, states);
        }
        for (state, expected) in expected.outgoing.iter().enumerate() {
            let mut count = 99;
            assert_eq!(
                liblrhsmm_rs_segmentation_outgoing_count_f64(owner, state, &mut count),
                0
            );
            assert_eq!(count, expected.len());
            let mut jumps = vec![
                LiblrhsmmRsJumpF64 {
                    delta: 99,
                    probability: 99.0
                };
                count
            ];
            assert_eq!(
                liblrhsmm_rs_segmentation_copy_outgoing_f64(
                    owner,
                    state,
                    0,
                    jumps.as_mut_ptr(),
                    count
                ),
                0
            );
            for (actual, expected) in jumps.iter().zip(expected) {
                assert_eq!(actual.delta, expected.delta);
                assert_eq!(actual.probability.to_bits(), expected.probability.to_bits());
            }
        }
    }
}

use liblrhsmm_rs::data_operations::regroup_with_precision;

unsafe fn sample_f32(
    variant: usize,
) -> (
    *mut LiblrhsmmRsObservationF32,
    *mut LiblrhsmmRsSegmentationF32,
    ObservationData<f32>,
    SegmentationData<f32>,
) {
    // SAFETY: The test supplies live independent arrays and transfers each unique child once.
    unsafe {
        let frames = if variant == 0 {
            8
        } else if variant == 1 {
            3
        } else {
            0
        };
        let dimensions = if variant == 0 { vec![2, 1] } else { vec![1] };
        let mut observation = ObservationData::<f32>::new(frames, &dimensions).unwrap();
        for (stream, data) in observation.streams.iter_mut().enumerate() {
            for (index, value) in data.values.iter_mut().enumerate() {
                *value = if variant == 0 {
                    (index / data.dimensions) as f32 * 10.0
                        + stream as f32 * 100.0
                        + (index % data.dimensions) as f32
                } else {
                    2.0 + index as f32 / 8.0 + f32::EPSILON
                };
            }
        }
        let mut stream_owners = Vec::new();
        for stream in &observation.streams {
            let mut owner = null_mut();
            assert_eq!(
                liblrhsmm_rs_observation_stream_create_f32(
                    stream.dimensions,
                    stream.values.as_ptr(),
                    stream.values.len(),
                    &mut owner
                ),
                0
            );
            stream_owners.push(owner);
        }
        let pointers = stream_owners
            .iter()
            .map(|owner| owner.cast_const())
            .collect::<Vec<_>>();
        let mut observation_owner = null_mut();
        assert_eq!(
            liblrhsmm_rs_observation_create_f32(
                frames,
                pointers.as_ptr(),
                pointers.len(),
                &mut observation_owner
            ),
            0
        );
        for owner in &mut stream_owners {
            assert_eq!(liblrhsmm_rs_observation_stream_release_f32(owner), 0);
        }
        let boundaries = if variant == 0 {
            vec![1, 3, 3, 6, 8]
        } else if variant == 1 {
            vec![0, 1, 3]
        } else {
            vec![]
        };
        let mut segmentation = SegmentationData::<f32>::new(dimensions.len(), boundaries.len());
        segmentation.boundaries = boundaries;
        for index in 0..segmentation.boundaries.len() {
            segmentation.duration_states[index] = 10 + index as i32;
            for stream in 0..dimensions.len() {
                segmentation.output_states[stream][index] = if stream == 0 {
                    20 + index as i32
                } else {
                    40 - index as i32
                };
            }
        }
        if variant == 1 {
            segmentation.add_jump(0, 2, 0.25 + f32::EPSILON).unwrap();
        }
        let outputs = segmentation
            .output_states
            .iter()
            .map(|states| states.as_ptr())
            .collect::<Vec<_>>();
        let jump_rows = segmentation
            .outgoing
            .iter()
            .map(|row| {
                row.iter()
                    .map(|jump| LiblrhsmmRsJumpF32 {
                        delta: jump.delta,
                        probability: jump.probability,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let slices = jump_rows
            .iter()
            .map(|row| LiblrhsmmRsJumpSliceF32 {
                jumps: row.as_ptr(),
                count: row.len(),
            })
            .collect::<Vec<_>>();
        let mut segmentation_owner = null_mut();
        assert_eq!(
            liblrhsmm_rs_segmentation_create_f32(
                dimensions.len(),
                segmentation.boundaries.len(),
                segmentation.boundaries.as_ptr(),
                segmentation.duration_states.as_ptr(),
                outputs.as_ptr(),
                slices.as_ptr(),
                &mut segmentation_owner
            ),
            0
        );
        (
            observation_owner,
            segmentation_owner,
            observation,
            segmentation,
        )
    }
}

#[test]
fn paired_regroup_f32_preserves_complete_native_results_and_ownership() {
    // SAFETY: All arrays and owner slots are independent; releases transfer unique ownership.
    unsafe {
        let (mut first_observation, mut first_segmentation, first_obs, first_seg) = sample_f32(0);
        let (mut second_observation, mut second_segmentation, second_obs, second_seg) =
            sample_f32(1);
        let (mut empty_observation, mut empty_segmentation, empty_obs, empty_seg) = sample_f32(2);
        let observations = [
            first_observation.cast_const(),
            second_observation.cast_const(),
            first_observation.cast_const(),
            empty_observation.cast_const(),
        ];
        let segmentations = [
            first_segmentation.cast_const(),
            second_segmentation.cast_const(),
            first_segmentation.cast_const(),
            empty_segmentation.cast_const(),
        ];
        let expected_observations = [
            first_obs.clone(),
            second_obs.clone(),
            first_obs.clone(),
            empty_obs,
        ];
        let expected_segmentations = [
            first_seg.clone(),
            second_seg.clone(),
            first_seg.clone(),
            empty_seg,
        ];
        let mut observation_set = null_mut();
        let mut segmentation_set = null_mut();
        assert_eq!(
            liblrhsmm_rs_observation_set_create_f32(
                observations.as_ptr(),
                observations.len(),
                &mut observation_set
            ),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_create_f32(
                segmentations.as_ptr(),
                segmentations.len(),
                &mut segmentation_set
            ),
            0
        );
        for unit in [1, 2, 3, 5, 8, usize::MAX] {
            let expected =
                regroup_with_precision(&expected_observations, &expected_segmentations, unit)
                    .unwrap();
            let mut dataset = null_mut();
            let mut copy = null_mut();
            assert_eq!(
                liblrhsmm_rs_regroup_data_f32(
                    observation_set,
                    segmentation_set,
                    unit,
                    &mut dataset
                ),
                0
            );
            let mut length = 99;
            assert_eq!(liblrhsmm_rs_dataset_length_f32(dataset, &mut length), 0);
            assert_eq!(length, expected.observations.len());
            assert_eq!(liblrhsmm_rs_dataset_clone_f32(dataset, &mut copy), 0);
            assert_eq!(liblrhsmm_rs_dataset_release_f32(&mut dataset), 0);
            let mut result_observations = null_mut();
            let mut result_segmentations = null_mut();
            assert_eq!(
                liblrhsmm_rs_dataset_get_observations_f32(copy, &mut result_observations),
                0
            );
            assert_eq!(
                liblrhsmm_rs_dataset_get_segmentations_f32(copy, &mut result_segmentations),
                0
            );
            assert_eq!(liblrhsmm_rs_dataset_release_f32(&mut copy), 0);
            assert_eq!(
                liblrhsmm_rs_observation_set_length_f32(result_observations, &mut length),
                0
            );
            assert_eq!(length, expected.observations.len());
            assert_eq!(
                liblrhsmm_rs_segmentation_set_length_f32(result_segmentations, &mut length),
                0
            );
            assert_eq!(length, expected.segmentations.len());
            for index in 0..length {
                let mut obs = null_mut();
                let mut seg = null_mut();
                assert_eq!(
                    liblrhsmm_rs_observation_set_get_f32(result_observations, index, &mut obs),
                    0
                );
                assert_eq!(
                    liblrhsmm_rs_segmentation_set_get_f32(result_segmentations, index, &mut seg),
                    0
                );
                compare_observation_f32(obs, &expected.observations[index]);
                compare_segmentation_f32(seg, &expected.segmentations[index]);
                assert_eq!(liblrhsmm_rs_observation_release_f32(&mut obs), 0);
                assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut seg), 0);
            }
            assert_eq!(
                liblrhsmm_rs_observation_set_release_f32(&mut result_observations),
                0
            );
            assert_eq!(
                liblrhsmm_rs_segmentation_set_release_f32(&mut result_segmentations),
                0
            );
        }
        let mut sentinel = 0x1000 as *mut LiblrhsmmRsDatasetF32;
        assert_eq!(
            liblrhsmm_rs_regroup_data_f32(observation_set, segmentation_set, 0, &mut sentinel),
            3
        );
        assert_eq!(sentinel as usize, 0x1000);
        assert_eq!(
            liblrhsmm_rs_regroup_data_f32(observation_set, segmentation_set, 2, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_regroup_data_f32(null(), segmentation_set, 2, &mut sentinel),
            1
        );
        assert_eq!(
            liblrhsmm_rs_regroup_data_f32(observation_set, null(), 2, &mut sentinel),
            1
        );
        let mut dataset = null_mut();
        assert_eq!(
            liblrhsmm_rs_regroup_data_f32(observation_set, segmentation_set, 2, &mut dataset),
            0
        );
        assert_eq!(liblrhsmm_rs_dataset_clone_f32(dataset, null_mut()), 1);
        assert_eq!(liblrhsmm_rs_dataset_length_f32(dataset, null_mut()), 1);
        assert_eq!(
            liblrhsmm_rs_dataset_get_observations_f32(dataset, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_dataset_get_segmentations_f32(dataset, null_mut()),
            1
        );
        assert_eq!(liblrhsmm_rs_dataset_clone_f32(null(), &mut sentinel), 1);
        assert_eq!(
            liblrhsmm_rs_dataset_get_observations_f32(null(), &mut observation_set),
            1
        );
        assert_eq!(
            liblrhsmm_rs_dataset_get_segmentations_f32(null(), &mut segmentation_set),
            1
        );
        assert_eq!(sentinel as usize, 0x1000);
        // A valid segmentation with the wrong final frame must fail even late in the set.
        let mut bad = null_mut();
        assert_eq!(
            liblrhsmm_rs_segmentation_clone_f32(second_segmentation, &mut bad),
            0
        );
        let wrong = [0, 1, 2];
        assert_eq!(
            liblrhsmm_rs_segmentation_set_boundaries_f32(bad, 0, wrong.as_ptr(), wrong.len()),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_set_f32(segmentation_set, 1, bad),
            0
        );
        assert_eq!(
            liblrhsmm_rs_regroup_data_f32(observation_set, segmentation_set, 2, &mut sentinel),
            3
        );
        assert_eq!(sentinel as usize, 0x1000);
        assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut bad), 0);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_set_f32(segmentation_set, 1, first_segmentation),
            0
        );
        assert_eq!(
            liblrhsmm_rs_regroup_data_f32(observation_set, segmentation_set, 2, &mut sentinel),
            3
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_replace_f32(segmentation_set, null(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_regroup_data_f32(observation_set, segmentation_set, 2, &mut sentinel),
            3
        );
        assert_eq!(sentinel as usize, 0x1000);
        assert_eq!(
            liblrhsmm_rs_observation_set_release_f32(&mut observation_set),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_release_f32(&mut segmentation_set),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_release_f32(&mut first_observation),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_release_f32(&mut first_segmentation),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_release_f32(&mut second_observation),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_release_f32(&mut second_segmentation),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_release_f32(&mut empty_observation),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_release_f32(&mut empty_segmentation),
            0
        );
        let mut retained = null_mut();
        assert_eq!(
            liblrhsmm_rs_dataset_get_observations_f32(dataset, &mut retained),
            0
        );
        assert_eq!(liblrhsmm_rs_dataset_release_f32(&mut dataset), 0);
        assert!(dataset.is_null());
        assert_eq!(liblrhsmm_rs_dataset_release_f32(&mut dataset), 0);
        assert_eq!(liblrhsmm_rs_dataset_release_f32(null_mut()), 1);
        let mut child = null_mut();
        assert_eq!(
            liblrhsmm_rs_observation_set_get_f32(retained, 0, &mut child),
            0
        );
        compare_observation_f32(
            child,
            &regroup_with_precision(&[first_obs], &[first_seg], 2)
                .unwrap()
                .observations[0],
        );
        assert_eq!(liblrhsmm_rs_observation_set_release_f32(&mut retained), 0);
        assert_eq!(liblrhsmm_rs_observation_release_f32(&mut child), 0);
        assert_eq!(
            liblrhsmm_rs_observation_set_create_f32(null(), 0, &mut observation_set),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_create_f32(null(), 0, &mut segmentation_set),
            0
        );
        assert_eq!(
            liblrhsmm_rs_regroup_data_f32(observation_set, segmentation_set, 1, &mut dataset),
            0
        );
        let mut length = 99;
        assert_eq!(liblrhsmm_rs_dataset_length_f32(dataset, &mut length), 0);
        assert_eq!(length, 0);
        assert_eq!(liblrhsmm_rs_dataset_release_f32(&mut dataset), 0);
        assert_eq!(
            liblrhsmm_rs_observation_set_release_f32(&mut observation_set),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_release_f32(&mut segmentation_set),
            0
        );
    }
}

unsafe fn sample_f64(
    variant: usize,
) -> (
    *mut LiblrhsmmRsObservationF64,
    *mut LiblrhsmmRsSegmentationF64,
    ObservationData<f64>,
    SegmentationData<f64>,
) {
    // SAFETY: The test supplies live independent arrays and transfers each unique child once.
    unsafe {
        let frames = if variant == 0 {
            8
        } else if variant == 1 {
            3
        } else {
            0
        };
        let dimensions = if variant == 0 { vec![2, 1] } else { vec![1] };
        let mut observation = ObservationData::<f64>::new(frames, &dimensions).unwrap();
        for (stream, data) in observation.streams.iter_mut().enumerate() {
            for (index, value) in data.values.iter_mut().enumerate() {
                *value = if variant == 0 {
                    1.0 + (index + stream + 1) as f64 * f64::EPSILON
                } else {
                    2.0 + index as f64 / 8.0 + f64::EPSILON
                };
            }
        }
        let mut stream_owners = Vec::new();
        for stream in &observation.streams {
            let mut owner = null_mut();
            assert_eq!(
                liblrhsmm_rs_observation_stream_create_f64(
                    stream.dimensions,
                    stream.values.as_ptr(),
                    stream.values.len(),
                    &mut owner
                ),
                0
            );
            stream_owners.push(owner);
        }
        let pointers = stream_owners
            .iter()
            .map(|owner| owner.cast_const())
            .collect::<Vec<_>>();
        let mut observation_owner = null_mut();
        assert_eq!(
            liblrhsmm_rs_observation_create_f64(
                frames,
                pointers.as_ptr(),
                pointers.len(),
                &mut observation_owner
            ),
            0
        );
        for owner in &mut stream_owners {
            assert_eq!(liblrhsmm_rs_observation_stream_release_f64(owner), 0);
        }
        let boundaries = if variant == 0 {
            vec![1, 3, 3, 6, 8]
        } else if variant == 1 {
            vec![0, 1, 3]
        } else {
            vec![]
        };
        let mut segmentation = SegmentationData::<f64>::new(dimensions.len(), boundaries.len());
        segmentation.boundaries = boundaries;
        for index in 0..segmentation.boundaries.len() {
            segmentation.duration_states[index] = 10 + index as i32;
            for stream in 0..dimensions.len() {
                segmentation.output_states[stream][index] = if stream == 0 {
                    20 + index as i32
                } else {
                    40 - index as i32
                };
            }
        }
        if variant == 1 {
            segmentation.add_jump(0, 2, 0.25 + f64::EPSILON).unwrap();
        }
        let outputs = segmentation
            .output_states
            .iter()
            .map(|states| states.as_ptr())
            .collect::<Vec<_>>();
        let jump_rows = segmentation
            .outgoing
            .iter()
            .map(|row| {
                row.iter()
                    .map(|jump| LiblrhsmmRsJumpF64 {
                        delta: jump.delta,
                        probability: jump.probability,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let slices = jump_rows
            .iter()
            .map(|row| LiblrhsmmRsJumpSliceF64 {
                jumps: row.as_ptr(),
                count: row.len(),
            })
            .collect::<Vec<_>>();
        let mut segmentation_owner = null_mut();
        assert_eq!(
            liblrhsmm_rs_segmentation_create_f64(
                dimensions.len(),
                segmentation.boundaries.len(),
                segmentation.boundaries.as_ptr(),
                segmentation.duration_states.as_ptr(),
                outputs.as_ptr(),
                slices.as_ptr(),
                &mut segmentation_owner
            ),
            0
        );
        (
            observation_owner,
            segmentation_owner,
            observation,
            segmentation,
        )
    }
}

#[test]
fn paired_regroup_f64_preserves_complete_native_results_and_ownership() {
    // SAFETY: All arrays and owner slots are independent; releases transfer unique ownership.
    unsafe {
        let (mut first_observation, mut first_segmentation, first_obs, first_seg) = sample_f64(0);
        let (mut second_observation, mut second_segmentation, second_obs, second_seg) =
            sample_f64(1);
        let (mut empty_observation, mut empty_segmentation, empty_obs, empty_seg) = sample_f64(2);
        let observations = [
            first_observation.cast_const(),
            second_observation.cast_const(),
            first_observation.cast_const(),
            empty_observation.cast_const(),
        ];
        let segmentations = [
            first_segmentation.cast_const(),
            second_segmentation.cast_const(),
            first_segmentation.cast_const(),
            empty_segmentation.cast_const(),
        ];
        let expected_observations = [
            first_obs.clone(),
            second_obs.clone(),
            first_obs.clone(),
            empty_obs,
        ];
        let expected_segmentations = [
            first_seg.clone(),
            second_seg.clone(),
            first_seg.clone(),
            empty_seg,
        ];
        let mut observation_set = null_mut();
        let mut segmentation_set = null_mut();
        assert_eq!(
            liblrhsmm_rs_observation_set_create_f64(
                observations.as_ptr(),
                observations.len(),
                &mut observation_set
            ),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_create_f64(
                segmentations.as_ptr(),
                segmentations.len(),
                &mut segmentation_set
            ),
            0
        );
        for unit in [1, 2, 3, 5, 8, usize::MAX] {
            let expected =
                regroup_with_precision(&expected_observations, &expected_segmentations, unit)
                    .unwrap();
            let mut dataset = null_mut();
            let mut copy = null_mut();
            assert_eq!(
                liblrhsmm_rs_regroup_data_f64(
                    observation_set,
                    segmentation_set,
                    unit,
                    &mut dataset
                ),
                0
            );
            let mut length = 99;
            assert_eq!(liblrhsmm_rs_dataset_length_f64(dataset, &mut length), 0);
            assert_eq!(length, expected.observations.len());
            assert_eq!(liblrhsmm_rs_dataset_clone_f64(dataset, &mut copy), 0);
            assert_eq!(liblrhsmm_rs_dataset_release_f64(&mut dataset), 0);
            let mut result_observations = null_mut();
            let mut result_segmentations = null_mut();
            assert_eq!(
                liblrhsmm_rs_dataset_get_observations_f64(copy, &mut result_observations),
                0
            );
            assert_eq!(
                liblrhsmm_rs_dataset_get_segmentations_f64(copy, &mut result_segmentations),
                0
            );
            assert_eq!(liblrhsmm_rs_dataset_release_f64(&mut copy), 0);
            assert_eq!(
                liblrhsmm_rs_observation_set_length_f64(result_observations, &mut length),
                0
            );
            assert_eq!(length, expected.observations.len());
            assert_eq!(
                liblrhsmm_rs_segmentation_set_length_f64(result_segmentations, &mut length),
                0
            );
            assert_eq!(length, expected.segmentations.len());
            for index in 0..length {
                let mut obs = null_mut();
                let mut seg = null_mut();
                assert_eq!(
                    liblrhsmm_rs_observation_set_get_f64(result_observations, index, &mut obs),
                    0
                );
                assert_eq!(
                    liblrhsmm_rs_segmentation_set_get_f64(result_segmentations, index, &mut seg),
                    0
                );
                compare_observation_f64(obs, &expected.observations[index]);
                compare_segmentation_f64(seg, &expected.segmentations[index]);
                assert_eq!(liblrhsmm_rs_observation_release_f64(&mut obs), 0);
                assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut seg), 0);
            }
            assert_eq!(
                liblrhsmm_rs_observation_set_release_f64(&mut result_observations),
                0
            );
            assert_eq!(
                liblrhsmm_rs_segmentation_set_release_f64(&mut result_segmentations),
                0
            );
        }
        let mut sentinel = 0x1000 as *mut LiblrhsmmRsDatasetF64;
        assert_eq!(
            liblrhsmm_rs_regroup_data_f64(observation_set, segmentation_set, 0, &mut sentinel),
            3
        );
        assert_eq!(sentinel as usize, 0x1000);
        assert_eq!(
            liblrhsmm_rs_regroup_data_f64(observation_set, segmentation_set, 2, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_regroup_data_f64(null(), segmentation_set, 2, &mut sentinel),
            1
        );
        assert_eq!(
            liblrhsmm_rs_regroup_data_f64(observation_set, null(), 2, &mut sentinel),
            1
        );
        let mut dataset = null_mut();
        assert_eq!(
            liblrhsmm_rs_regroup_data_f64(observation_set, segmentation_set, 2, &mut dataset),
            0
        );
        assert_eq!(liblrhsmm_rs_dataset_clone_f64(dataset, null_mut()), 1);
        assert_eq!(liblrhsmm_rs_dataset_length_f64(dataset, null_mut()), 1);
        assert_eq!(
            liblrhsmm_rs_dataset_get_observations_f64(dataset, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_dataset_get_segmentations_f64(dataset, null_mut()),
            1
        );
        assert_eq!(liblrhsmm_rs_dataset_clone_f64(null(), &mut sentinel), 1);
        assert_eq!(
            liblrhsmm_rs_dataset_get_observations_f64(null(), &mut observation_set),
            1
        );
        assert_eq!(
            liblrhsmm_rs_dataset_get_segmentations_f64(null(), &mut segmentation_set),
            1
        );
        assert_eq!(sentinel as usize, 0x1000);
        // A valid segmentation with the wrong final frame must fail even late in the set.
        let mut bad = null_mut();
        assert_eq!(
            liblrhsmm_rs_segmentation_clone_f64(second_segmentation, &mut bad),
            0
        );
        let wrong = [0, 1, 2];
        assert_eq!(
            liblrhsmm_rs_segmentation_set_boundaries_f64(bad, 0, wrong.as_ptr(), wrong.len()),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_set_f64(segmentation_set, 1, bad),
            0
        );
        assert_eq!(
            liblrhsmm_rs_regroup_data_f64(observation_set, segmentation_set, 2, &mut sentinel),
            3
        );
        assert_eq!(sentinel as usize, 0x1000);
        assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut bad), 0);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_set_f64(segmentation_set, 1, first_segmentation),
            0
        );
        assert_eq!(
            liblrhsmm_rs_regroup_data_f64(observation_set, segmentation_set, 2, &mut sentinel),
            3
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_replace_f64(segmentation_set, null(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_regroup_data_f64(observation_set, segmentation_set, 2, &mut sentinel),
            3
        );
        assert_eq!(sentinel as usize, 0x1000);
        assert_eq!(
            liblrhsmm_rs_observation_set_release_f64(&mut observation_set),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_release_f64(&mut segmentation_set),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_release_f64(&mut first_observation),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_release_f64(&mut first_segmentation),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_release_f64(&mut second_observation),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_release_f64(&mut second_segmentation),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_release_f64(&mut empty_observation),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_release_f64(&mut empty_segmentation),
            0
        );
        let mut retained = null_mut();
        assert_eq!(
            liblrhsmm_rs_dataset_get_observations_f64(dataset, &mut retained),
            0
        );
        assert_eq!(liblrhsmm_rs_dataset_release_f64(&mut dataset), 0);
        assert!(dataset.is_null());
        assert_eq!(liblrhsmm_rs_dataset_release_f64(&mut dataset), 0);
        assert_eq!(liblrhsmm_rs_dataset_release_f64(null_mut()), 1);
        let mut child = null_mut();
        assert_eq!(
            liblrhsmm_rs_observation_set_get_f64(retained, 0, &mut child),
            0
        );
        compare_observation_f64(
            child,
            &regroup_with_precision(&[first_obs], &[first_seg], 2)
                .unwrap()
                .observations[0],
        );
        assert_eq!(liblrhsmm_rs_observation_set_release_f64(&mut retained), 0);
        assert_eq!(liblrhsmm_rs_observation_release_f64(&mut child), 0);
        assert_eq!(
            liblrhsmm_rs_observation_set_create_f64(null(), 0, &mut observation_set),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_create_f64(null(), 0, &mut segmentation_set),
            0
        );
        assert_eq!(
            liblrhsmm_rs_regroup_data_f64(observation_set, segmentation_set, 1, &mut dataset),
            0
        );
        let mut length = 99;
        assert_eq!(liblrhsmm_rs_dataset_length_f64(dataset, &mut length), 0);
        assert_eq!(length, 0);
        assert_eq!(liblrhsmm_rs_dataset_release_f64(&mut dataset), 0);
        assert_eq!(
            liblrhsmm_rs_observation_set_release_f64(&mut observation_set),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_release_f64(&mut segmentation_set),
            0
        );
    }
}
