#![cfg(feature = "c-api")]
use liblrhsmm_rs::c_api::*;
use liblrhsmm_rs::data::{ObservationData, SegmentationData};
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

unsafe fn make_observation_f32(
    variant: usize,
) -> (*mut LiblrhsmmRsObservationF32, ObservationData<f32>) {
    // SAFETY: Test maintains exclusive live owners and independent initialized arrays.
    unsafe {
        let frames = if variant == 0 { 3 } else { 2 };
        let dimensions = if variant == 0 { vec![2, 1] } else { vec![1] };
        let mut expected = ObservationData::<f32>::new(frames, &dimensions).unwrap();
        let mut owners = Vec::new();
        for (stream, values) in expected.streams.iter_mut().enumerate() {
            for (index, value) in values.values.iter_mut().enumerate() {
                *value = 1.0 + (index + stream + variant) as f32 / 8.0 + f32::EPSILON;
            }
            let mut owner = null_mut();
            assert_eq!(
                liblrhsmm_rs_observation_stream_create_f32(
                    values.dimensions,
                    values.values.as_ptr(),
                    values.values.len(),
                    &mut owner
                ),
                0
            );
            owners.push(owner);
        }
        let pointers = owners
            .iter()
            .map(|owner| owner.cast_const())
            .collect::<Vec<_>>();
        let mut owner = null_mut();
        assert_eq!(
            liblrhsmm_rs_observation_create_f32(
                frames,
                pointers.as_ptr(),
                pointers.len(),
                &mut owner
            ),
            0
        );
        for child in &mut owners {
            assert_eq!(liblrhsmm_rs_observation_stream_release_f32(child), 0);
        }
        (owner, expected)
    }
}

#[test]
fn observation_f32_complete_sets_missing_slots_and_atomic_errors() {
    // SAFETY: Test keeps unique live owners, independent outputs and exclusive mutations.
    unsafe {
        let (mut first, first_value) = make_observation_f32(0);
        let (mut second, second_value) = make_observation_f32(1);
        let mut builder = null_mut();
        let mut builder_copy = null_mut();
        let mut built = null_mut();
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_new_f32(3, &mut builder),
            0
        );
        let mut length = 99;
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_length_f32(builder, &mut length),
            0
        );
        assert_eq!(length, 3);
        let mut presence = 99;
        let mut child = 0x1000 as *mut LiblrhsmmRsObservationF32;
        let mut sentinel = 0x1000 as *mut LiblrhsmmRsObservationSetF32;
        for index in 0..3 {
            assert_eq!(
                liblrhsmm_rs_observation_set_builder_has_sample_f32(builder, index, &mut presence),
                0
            );
            assert_eq!(presence, 0);
            assert_eq!(
                liblrhsmm_rs_observation_set_builder_get_sample_f32(builder, index, &mut child),
                3
            );
            assert_eq!(child, 0x1000 as *mut LiblrhsmmRsObservationF32);
        }
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_build_f32(builder, &mut sentinel),
            3
        );
        assert_eq!(sentinel, 0x1000 as *mut LiblrhsmmRsObservationSetF32);
        for (index, source) in [first, second, first].into_iter().enumerate() {
            assert_eq!(
                liblrhsmm_rs_observation_set_builder_set_sample_f32(builder, index, source),
                0
            );
        }
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_clone_f32(builder, &mut builder_copy),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_clear_sample_f32(builder, 1),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_has_sample_f32(builder, 1, &mut presence),
            0
        );
        assert_eq!(presence, 0);
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_get_sample_f32(builder_copy, 1, &mut child),
            0
        );
        compare_observation_f32(child, &second_value);
        assert_eq!(liblrhsmm_rs_observation_release_f32(&mut child), 0);
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_resize_f32(builder, 4),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_has_sample_f32(builder, 3, &mut presence),
            0
        );
        assert_eq!(presence, 0);
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_resize_f32(builder, 1),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_build_f32(builder, &mut built),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_length_f32(built, &mut length),
            0
        );
        assert_eq!(length, 1);
        assert_eq!(liblrhsmm_rs_observation_set_release_f32(&mut built), 0);
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_build_f32(builder_copy, &mut built),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_length_f32(built, &mut length),
            0
        );
        assert_eq!(length, 3);
        for (index, expected) in [&first_value, &second_value, &first_value]
            .into_iter()
            .enumerate()
        {
            assert_eq!(
                liblrhsmm_rs_observation_set_get_f32(built, index, &mut child),
                0
            );
            compare_observation_f32(child, expected);
            assert_eq!(liblrhsmm_rs_observation_release_f32(&mut child), 0);
        }
        assert_eq!(liblrhsmm_rs_observation_set_set_f32(built, 1, first), 0);
        assert_eq!(liblrhsmm_rs_observation_set_append_f32(built, second), 0);
        for (index, expected) in [&first_value, &first_value, &first_value, &second_value]
            .into_iter()
            .enumerate()
        {
            assert_eq!(
                liblrhsmm_rs_observation_set_get_f32(built, index, &mut child),
                0
            );
            compare_observation_f32(child, expected);
            assert_eq!(liblrhsmm_rs_observation_release_f32(&mut child), 0);
        }
        assert_eq!(
            liblrhsmm_rs_observation_set_length_f32(built, &mut length),
            0
        );
        assert_eq!(length, 4);
        let inputs = [first.cast_const(), second.cast_const(), first.cast_const()];
        assert_eq!(
            liblrhsmm_rs_observation_set_replace_f32(built, inputs.as_ptr(), inputs.len()),
            0
        );
        let mut copied = null_mut();
        let mut created = null_mut();
        assert_eq!(
            liblrhsmm_rs_observation_set_clone_f32(built, &mut copied),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_create_f32(inputs.as_ptr(), inputs.len(), &mut created),
            0
        );
        let bad = [first.cast_const(), null()];
        sentinel = created;
        assert_eq!(
            liblrhsmm_rs_observation_set_create_f32(bad.as_ptr(), 2, &mut sentinel),
            1
        );
        assert_eq!(sentinel, created);
        assert_eq!(
            liblrhsmm_rs_observation_set_replace_f32(built, bad.as_ptr(), 2),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_create_f32(null(), usize::MAX, &mut sentinel),
            2
        );
        assert_eq!(sentinel, created);
        assert_eq!(
            liblrhsmm_rs_observation_set_create_f32(null(), 1, &mut sentinel),
            1
        );
        assert_eq!(liblrhsmm_rs_observation_set_set_f32(built, 99, first), 2);
        assert_eq!(liblrhsmm_rs_observation_set_set_f32(built, 0, null()), 1);
        assert_eq!(liblrhsmm_rs_observation_set_append_f32(built, null()), 1);
        child = 0x1000 as *mut LiblrhsmmRsObservationF32;
        assert_eq!(
            liblrhsmm_rs_observation_set_get_f32(built, 99, &mut child),
            2
        );
        assert_eq!(child, 0x1000 as *mut LiblrhsmmRsObservationF32);
        assert_eq!(
            liblrhsmm_rs_observation_set_get_f32(built, 0, null_mut()),
            1
        );
        assert_eq!(liblrhsmm_rs_observation_set_clone_f32(built, null_mut()), 1);
        assert_eq!(
            liblrhsmm_rs_observation_set_length_f32(built, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_resize_f32(builder_copy, i32::MAX as usize + 1),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_length_f32(builder_copy, &mut length),
            0
        );
        assert_eq!(length, 3);
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_has_sample_f32(builder_copy, 99, &mut presence),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_has_sample_f32(builder_copy, 0, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_set_sample_f32(builder_copy, 0, null()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_clear_sample_f32(builder_copy, 99),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_build_f32(builder_copy, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_clone_f32(builder_copy, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_length_f32(builder_copy, null_mut()),
            1
        );
        let mut builder_sentinel = builder;
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_new_f32(
                i32::MAX as usize + 1,
                &mut builder_sentinel
            ),
            3
        );
        assert_eq!(builder_sentinel, builder);
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_new_f32(usize::MAX, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_get_f32(copied, 1, &mut child),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_release_f32(&mut builder),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_release_f32(&mut builder_copy),
            0
        );
        assert_eq!(liblrhsmm_rs_observation_release_f32(&mut first), 0);
        assert_eq!(liblrhsmm_rs_observation_release_f32(&mut second), 0);
        assert_eq!(liblrhsmm_rs_observation_set_release_f32(&mut copied), 0);
        compare_observation_f32(child, &second_value);
        assert_eq!(liblrhsmm_rs_observation_release_f32(&mut child), 0);
        for set in [built, created] {
            for (index, expected) in [&first_value, &second_value, &first_value]
                .into_iter()
                .enumerate()
            {
                assert_eq!(
                    liblrhsmm_rs_observation_set_get_f32(set, index, &mut child),
                    0
                );
                compare_observation_f32(child, expected);
                assert_eq!(liblrhsmm_rs_observation_release_f32(&mut child), 0);
            }
        }
        assert_eq!(
            liblrhsmm_rs_observation_set_replace_f32(built, null(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_length_f32(built, &mut length),
            0
        );
        assert_eq!(length, 0);
        assert_eq!(liblrhsmm_rs_observation_set_release_f32(&mut built), 0);
        assert_eq!(liblrhsmm_rs_observation_set_release_f32(&mut created), 0);
        assert_eq!(liblrhsmm_rs_observation_set_release_f32(&mut created), 0);
        assert_eq!(liblrhsmm_rs_observation_set_release_f32(null_mut()), 1);
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_release_f32(&mut builder_copy),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_release_f32(null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_new_f32(0, &mut builder),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_build_f32(builder, &mut built),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_length_f32(built, &mut length),
            0
        );
        assert_eq!(length, 0);
        assert_eq!(liblrhsmm_rs_observation_set_release_f32(&mut built), 0);
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_release_f32(&mut builder),
            0
        );
    }
}
unsafe fn make_observation_f64(
    variant: usize,
) -> (*mut LiblrhsmmRsObservationF64, ObservationData<f64>) {
    // SAFETY: Test maintains exclusive live owners and independent initialized arrays.
    unsafe {
        let frames = if variant == 0 { 3 } else { 2 };
        let dimensions = if variant == 0 { vec![2, 1] } else { vec![1] };
        let mut expected = ObservationData::<f64>::new(frames, &dimensions).unwrap();
        let mut owners = Vec::new();
        for (stream, values) in expected.streams.iter_mut().enumerate() {
            for (index, value) in values.values.iter_mut().enumerate() {
                *value = 1.0 + (index + stream + variant) as f64 / 8.0 + f64::EPSILON;
            }
            let mut owner = null_mut();
            assert_eq!(
                liblrhsmm_rs_observation_stream_create_f64(
                    values.dimensions,
                    values.values.as_ptr(),
                    values.values.len(),
                    &mut owner
                ),
                0
            );
            owners.push(owner);
        }
        let pointers = owners
            .iter()
            .map(|owner| owner.cast_const())
            .collect::<Vec<_>>();
        let mut owner = null_mut();
        assert_eq!(
            liblrhsmm_rs_observation_create_f64(
                frames,
                pointers.as_ptr(),
                pointers.len(),
                &mut owner
            ),
            0
        );
        for child in &mut owners {
            assert_eq!(liblrhsmm_rs_observation_stream_release_f64(child), 0);
        }
        (owner, expected)
    }
}

#[test]
fn observation_f64_complete_sets_missing_slots_and_atomic_errors() {
    // SAFETY: Test keeps unique live owners, independent outputs and exclusive mutations.
    unsafe {
        let (mut first, first_value) = make_observation_f64(0);
        let (mut second, second_value) = make_observation_f64(1);
        let mut builder = null_mut();
        let mut builder_copy = null_mut();
        let mut built = null_mut();
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_new_f64(3, &mut builder),
            0
        );
        let mut length = 99;
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_length_f64(builder, &mut length),
            0
        );
        assert_eq!(length, 3);
        let mut presence = 99;
        let mut child = 0x1000 as *mut LiblrhsmmRsObservationF64;
        let mut sentinel = 0x1000 as *mut LiblrhsmmRsObservationSetF64;
        for index in 0..3 {
            assert_eq!(
                liblrhsmm_rs_observation_set_builder_has_sample_f64(builder, index, &mut presence),
                0
            );
            assert_eq!(presence, 0);
            assert_eq!(
                liblrhsmm_rs_observation_set_builder_get_sample_f64(builder, index, &mut child),
                3
            );
            assert_eq!(child, 0x1000 as *mut LiblrhsmmRsObservationF64);
        }
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_build_f64(builder, &mut sentinel),
            3
        );
        assert_eq!(sentinel, 0x1000 as *mut LiblrhsmmRsObservationSetF64);
        for (index, source) in [first, second, first].into_iter().enumerate() {
            assert_eq!(
                liblrhsmm_rs_observation_set_builder_set_sample_f64(builder, index, source),
                0
            );
        }
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_clone_f64(builder, &mut builder_copy),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_clear_sample_f64(builder, 1),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_has_sample_f64(builder, 1, &mut presence),
            0
        );
        assert_eq!(presence, 0);
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_get_sample_f64(builder_copy, 1, &mut child),
            0
        );
        compare_observation_f64(child, &second_value);
        assert_eq!(liblrhsmm_rs_observation_release_f64(&mut child), 0);
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_resize_f64(builder, 4),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_has_sample_f64(builder, 3, &mut presence),
            0
        );
        assert_eq!(presence, 0);
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_resize_f64(builder, 1),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_build_f64(builder, &mut built),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_length_f64(built, &mut length),
            0
        );
        assert_eq!(length, 1);
        assert_eq!(liblrhsmm_rs_observation_set_release_f64(&mut built), 0);
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_build_f64(builder_copy, &mut built),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_length_f64(built, &mut length),
            0
        );
        assert_eq!(length, 3);
        for (index, expected) in [&first_value, &second_value, &first_value]
            .into_iter()
            .enumerate()
        {
            assert_eq!(
                liblrhsmm_rs_observation_set_get_f64(built, index, &mut child),
                0
            );
            compare_observation_f64(child, expected);
            assert_eq!(liblrhsmm_rs_observation_release_f64(&mut child), 0);
        }
        assert_eq!(liblrhsmm_rs_observation_set_set_f64(built, 1, first), 0);
        assert_eq!(liblrhsmm_rs_observation_set_append_f64(built, second), 0);
        for (index, expected) in [&first_value, &first_value, &first_value, &second_value]
            .into_iter()
            .enumerate()
        {
            assert_eq!(
                liblrhsmm_rs_observation_set_get_f64(built, index, &mut child),
                0
            );
            compare_observation_f64(child, expected);
            assert_eq!(liblrhsmm_rs_observation_release_f64(&mut child), 0);
        }
        assert_eq!(
            liblrhsmm_rs_observation_set_length_f64(built, &mut length),
            0
        );
        assert_eq!(length, 4);
        let inputs = [first.cast_const(), second.cast_const(), first.cast_const()];
        assert_eq!(
            liblrhsmm_rs_observation_set_replace_f64(built, inputs.as_ptr(), inputs.len()),
            0
        );
        let mut copied = null_mut();
        let mut created = null_mut();
        assert_eq!(
            liblrhsmm_rs_observation_set_clone_f64(built, &mut copied),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_create_f64(inputs.as_ptr(), inputs.len(), &mut created),
            0
        );
        let bad = [first.cast_const(), null()];
        sentinel = created;
        assert_eq!(
            liblrhsmm_rs_observation_set_create_f64(bad.as_ptr(), 2, &mut sentinel),
            1
        );
        assert_eq!(sentinel, created);
        assert_eq!(
            liblrhsmm_rs_observation_set_replace_f64(built, bad.as_ptr(), 2),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_create_f64(null(), usize::MAX, &mut sentinel),
            2
        );
        assert_eq!(sentinel, created);
        assert_eq!(
            liblrhsmm_rs_observation_set_create_f64(null(), 1, &mut sentinel),
            1
        );
        assert_eq!(liblrhsmm_rs_observation_set_set_f64(built, 99, first), 2);
        assert_eq!(liblrhsmm_rs_observation_set_set_f64(built, 0, null()), 1);
        assert_eq!(liblrhsmm_rs_observation_set_append_f64(built, null()), 1);
        child = 0x1000 as *mut LiblrhsmmRsObservationF64;
        assert_eq!(
            liblrhsmm_rs_observation_set_get_f64(built, 99, &mut child),
            2
        );
        assert_eq!(child, 0x1000 as *mut LiblrhsmmRsObservationF64);
        assert_eq!(
            liblrhsmm_rs_observation_set_get_f64(built, 0, null_mut()),
            1
        );
        assert_eq!(liblrhsmm_rs_observation_set_clone_f64(built, null_mut()), 1);
        assert_eq!(
            liblrhsmm_rs_observation_set_length_f64(built, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_resize_f64(builder_copy, i32::MAX as usize + 1),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_length_f64(builder_copy, &mut length),
            0
        );
        assert_eq!(length, 3);
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_has_sample_f64(builder_copy, 99, &mut presence),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_has_sample_f64(builder_copy, 0, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_set_sample_f64(builder_copy, 0, null()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_clear_sample_f64(builder_copy, 99),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_build_f64(builder_copy, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_clone_f64(builder_copy, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_length_f64(builder_copy, null_mut()),
            1
        );
        let mut builder_sentinel = builder;
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_new_f64(
                i32::MAX as usize + 1,
                &mut builder_sentinel
            ),
            3
        );
        assert_eq!(builder_sentinel, builder);
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_new_f64(usize::MAX, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_get_f64(copied, 1, &mut child),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_release_f64(&mut builder),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_release_f64(&mut builder_copy),
            0
        );
        assert_eq!(liblrhsmm_rs_observation_release_f64(&mut first), 0);
        assert_eq!(liblrhsmm_rs_observation_release_f64(&mut second), 0);
        assert_eq!(liblrhsmm_rs_observation_set_release_f64(&mut copied), 0);
        compare_observation_f64(child, &second_value);
        assert_eq!(liblrhsmm_rs_observation_release_f64(&mut child), 0);
        for set in [built, created] {
            for (index, expected) in [&first_value, &second_value, &first_value]
                .into_iter()
                .enumerate()
            {
                assert_eq!(
                    liblrhsmm_rs_observation_set_get_f64(set, index, &mut child),
                    0
                );
                compare_observation_f64(child, expected);
                assert_eq!(liblrhsmm_rs_observation_release_f64(&mut child), 0);
            }
        }
        assert_eq!(
            liblrhsmm_rs_observation_set_replace_f64(built, null(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_length_f64(built, &mut length),
            0
        );
        assert_eq!(length, 0);
        assert_eq!(liblrhsmm_rs_observation_set_release_f64(&mut built), 0);
        assert_eq!(liblrhsmm_rs_observation_set_release_f64(&mut created), 0);
        assert_eq!(liblrhsmm_rs_observation_set_release_f64(&mut created), 0);
        assert_eq!(liblrhsmm_rs_observation_set_release_f64(null_mut()), 1);
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_release_f64(&mut builder_copy),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_release_f64(null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_new_f64(0, &mut builder),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_build_f64(builder, &mut built),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_length_f64(built, &mut length),
            0
        );
        assert_eq!(length, 0);
        assert_eq!(liblrhsmm_rs_observation_set_release_f64(&mut built), 0);
        assert_eq!(
            liblrhsmm_rs_observation_set_builder_release_f64(&mut builder),
            0
        );
    }
}
unsafe fn make_segmentation_f32(
    variant: usize,
) -> (*mut LiblrhsmmRsSegmentationF32, SegmentationData<f32>) {
    // SAFETY: Test maintains exclusive live owners and independent initialized arrays.
    unsafe {
        let streams = if variant == 0 { 2 } else { 1 };
        let segments = if variant == 0 { 3 } else { 2 };
        let mut expected = SegmentationData::<f32>::new(streams, segments);
        expected.boundaries = (1..=segments as i32).collect();
        expected.duration_states = (0..segments).map(|index| 10 + index as i32).collect();
        for (stream, states) in expected.output_states.iter_mut().enumerate() {
            *states = (0..segments)
                .map(|index| 20 + stream as i32 * 10 + index as i32)
                .collect();
        }
        expected.outgoing[0][0].probability = 0.5 + f32::EPSILON;
        expected.outgoing[0].insert(
            0,
            liblrhsmm_rs::data::JumpParameters {
                delta: 2,
                probability: 0.25,
            },
        );
        let rows = expected
            .output_states
            .iter()
            .map(|row| row.as_ptr())
            .collect::<Vec<_>>();
        let jumps = expected
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
        let slices = jumps
            .iter()
            .map(|row| LiblrhsmmRsJumpSliceF32 {
                jumps: row.as_ptr(),
                count: row.len(),
            })
            .collect::<Vec<_>>();
        let mut owner = null_mut();
        assert_eq!(
            liblrhsmm_rs_segmentation_create_f32(
                streams,
                segments,
                expected.boundaries.as_ptr(),
                expected.duration_states.as_ptr(),
                rows.as_ptr(),
                slices.as_ptr(),
                &mut owner
            ),
            0
        );
        (owner, expected)
    }
}

#[test]
fn segmentation_f32_complete_sets_missing_slots_and_atomic_errors() {
    // SAFETY: Test keeps unique live owners, independent outputs and exclusive mutations.
    unsafe {
        let (mut first, first_value) = make_segmentation_f32(0);
        let (mut second, second_value) = make_segmentation_f32(1);
        let mut builder = null_mut();
        let mut builder_copy = null_mut();
        let mut built = null_mut();
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_new_f32(3, &mut builder),
            0
        );
        let mut length = 99;
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_length_f32(builder, &mut length),
            0
        );
        assert_eq!(length, 3);
        let mut presence = 99;
        let mut child = 0x1000 as *mut LiblrhsmmRsSegmentationF32;
        let mut sentinel = 0x1000 as *mut LiblrhsmmRsSegmentationSetF32;
        for index in 0..3 {
            assert_eq!(
                liblrhsmm_rs_segmentation_set_builder_has_sample_f32(builder, index, &mut presence),
                0
            );
            assert_eq!(presence, 0);
            assert_eq!(
                liblrhsmm_rs_segmentation_set_builder_get_sample_f32(builder, index, &mut child),
                3
            );
            assert_eq!(child, 0x1000 as *mut LiblrhsmmRsSegmentationF32);
        }
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_build_f32(builder, &mut sentinel),
            3
        );
        assert_eq!(sentinel, 0x1000 as *mut LiblrhsmmRsSegmentationSetF32);
        for (index, source) in [first, second, first].into_iter().enumerate() {
            assert_eq!(
                liblrhsmm_rs_segmentation_set_builder_set_sample_f32(builder, index, source),
                0
            );
        }
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_clone_f32(builder, &mut builder_copy),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_clear_sample_f32(builder, 1),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_has_sample_f32(builder, 1, &mut presence),
            0
        );
        assert_eq!(presence, 0);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_get_sample_f32(builder_copy, 1, &mut child),
            0
        );
        compare_segmentation_f32(child, &second_value);
        assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut child), 0);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_resize_f32(builder, 4),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_has_sample_f32(builder, 3, &mut presence),
            0
        );
        assert_eq!(presence, 0);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_resize_f32(builder, 1),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_build_f32(builder, &mut built),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_length_f32(built, &mut length),
            0
        );
        assert_eq!(length, 1);
        assert_eq!(liblrhsmm_rs_segmentation_set_release_f32(&mut built), 0);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_build_f32(builder_copy, &mut built),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_length_f32(built, &mut length),
            0
        );
        assert_eq!(length, 3);
        for (index, expected) in [&first_value, &second_value, &first_value]
            .into_iter()
            .enumerate()
        {
            assert_eq!(
                liblrhsmm_rs_segmentation_set_get_f32(built, index, &mut child),
                0
            );
            compare_segmentation_f32(child, expected);
            assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut child), 0);
        }
        assert_eq!(liblrhsmm_rs_segmentation_set_set_f32(built, 1, first), 0);
        assert_eq!(liblrhsmm_rs_segmentation_set_append_f32(built, second), 0);
        for (index, expected) in [&first_value, &first_value, &first_value, &second_value]
            .into_iter()
            .enumerate()
        {
            assert_eq!(
                liblrhsmm_rs_segmentation_set_get_f32(built, index, &mut child),
                0
            );
            compare_segmentation_f32(child, expected);
            assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut child), 0);
        }
        assert_eq!(
            liblrhsmm_rs_segmentation_set_length_f32(built, &mut length),
            0
        );
        assert_eq!(length, 4);
        let inputs = [first.cast_const(), second.cast_const(), first.cast_const()];
        assert_eq!(
            liblrhsmm_rs_segmentation_set_replace_f32(built, inputs.as_ptr(), inputs.len()),
            0
        );
        let mut copied = null_mut();
        let mut created = null_mut();
        assert_eq!(
            liblrhsmm_rs_segmentation_set_clone_f32(built, &mut copied),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_create_f32(inputs.as_ptr(), inputs.len(), &mut created),
            0
        );
        let bad = [first.cast_const(), null()];
        sentinel = created;
        assert_eq!(
            liblrhsmm_rs_segmentation_set_create_f32(bad.as_ptr(), 2, &mut sentinel),
            1
        );
        assert_eq!(sentinel, created);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_replace_f32(built, bad.as_ptr(), 2),
            1
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_create_f32(null(), usize::MAX, &mut sentinel),
            2
        );
        assert_eq!(sentinel, created);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_create_f32(null(), 1, &mut sentinel),
            1
        );
        assert_eq!(liblrhsmm_rs_segmentation_set_set_f32(built, 99, first), 2);
        assert_eq!(liblrhsmm_rs_segmentation_set_set_f32(built, 0, null()), 1);
        assert_eq!(liblrhsmm_rs_segmentation_set_append_f32(built, null()), 1);
        child = 0x1000 as *mut LiblrhsmmRsSegmentationF32;
        assert_eq!(
            liblrhsmm_rs_segmentation_set_get_f32(built, 99, &mut child),
            2
        );
        assert_eq!(child, 0x1000 as *mut LiblrhsmmRsSegmentationF32);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_get_f32(built, 0, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_clone_f32(built, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_length_f32(built, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_resize_f32(builder_copy, i32::MAX as usize + 1),
            2
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_length_f32(builder_copy, &mut length),
            0
        );
        assert_eq!(length, 3);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_has_sample_f32(builder_copy, 99, &mut presence),
            2
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_has_sample_f32(builder_copy, 0, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_set_sample_f32(builder_copy, 0, null()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_clear_sample_f32(builder_copy, 99),
            2
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_build_f32(builder_copy, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_clone_f32(builder_copy, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_length_f32(builder_copy, null_mut()),
            1
        );
        let mut builder_sentinel = builder;
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_new_f32(
                i32::MAX as usize + 1,
                &mut builder_sentinel
            ),
            3
        );
        assert_eq!(builder_sentinel, builder);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_new_f32(usize::MAX, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_get_f32(copied, 1, &mut child),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_release_f32(&mut builder),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_release_f32(&mut builder_copy),
            0
        );
        assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut first), 0);
        assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut second), 0);
        assert_eq!(liblrhsmm_rs_segmentation_set_release_f32(&mut copied), 0);
        compare_segmentation_f32(child, &second_value);
        assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut child), 0);
        for set in [built, created] {
            for (index, expected) in [&first_value, &second_value, &first_value]
                .into_iter()
                .enumerate()
            {
                assert_eq!(
                    liblrhsmm_rs_segmentation_set_get_f32(set, index, &mut child),
                    0
                );
                compare_segmentation_f32(child, expected);
                assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut child), 0);
            }
        }
        assert_eq!(
            liblrhsmm_rs_segmentation_set_replace_f32(built, null(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_length_f32(built, &mut length),
            0
        );
        assert_eq!(length, 0);
        assert_eq!(liblrhsmm_rs_segmentation_set_release_f32(&mut built), 0);
        assert_eq!(liblrhsmm_rs_segmentation_set_release_f32(&mut created), 0);
        assert_eq!(liblrhsmm_rs_segmentation_set_release_f32(&mut created), 0);
        assert_eq!(liblrhsmm_rs_segmentation_set_release_f32(null_mut()), 1);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_release_f32(&mut builder_copy),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_release_f32(null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_new_f32(0, &mut builder),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_build_f32(builder, &mut built),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_length_f32(built, &mut length),
            0
        );
        assert_eq!(length, 0);
        assert_eq!(liblrhsmm_rs_segmentation_set_release_f32(&mut built), 0);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_release_f32(&mut builder),
            0
        );
    }
}
unsafe fn make_segmentation_f64(
    variant: usize,
) -> (*mut LiblrhsmmRsSegmentationF64, SegmentationData<f64>) {
    // SAFETY: Test maintains exclusive live owners and independent initialized arrays.
    unsafe {
        let streams = if variant == 0 { 2 } else { 1 };
        let segments = if variant == 0 { 3 } else { 2 };
        let mut expected = SegmentationData::<f64>::new(streams, segments);
        expected.boundaries = (1..=segments as i32).collect();
        expected.duration_states = (0..segments).map(|index| 10 + index as i32).collect();
        for (stream, states) in expected.output_states.iter_mut().enumerate() {
            *states = (0..segments)
                .map(|index| 20 + stream as i32 * 10 + index as i32)
                .collect();
        }
        expected.outgoing[0][0].probability = 0.5 + f64::EPSILON;
        expected.outgoing[0].insert(
            0,
            liblrhsmm_rs::data::JumpParameters {
                delta: 2,
                probability: 0.25,
            },
        );
        let rows = expected
            .output_states
            .iter()
            .map(|row| row.as_ptr())
            .collect::<Vec<_>>();
        let jumps = expected
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
        let slices = jumps
            .iter()
            .map(|row| LiblrhsmmRsJumpSliceF64 {
                jumps: row.as_ptr(),
                count: row.len(),
            })
            .collect::<Vec<_>>();
        let mut owner = null_mut();
        assert_eq!(
            liblrhsmm_rs_segmentation_create_f64(
                streams,
                segments,
                expected.boundaries.as_ptr(),
                expected.duration_states.as_ptr(),
                rows.as_ptr(),
                slices.as_ptr(),
                &mut owner
            ),
            0
        );
        (owner, expected)
    }
}

#[test]
fn segmentation_f64_complete_sets_missing_slots_and_atomic_errors() {
    // SAFETY: Test keeps unique live owners, independent outputs and exclusive mutations.
    unsafe {
        let (mut first, first_value) = make_segmentation_f64(0);
        let (mut second, second_value) = make_segmentation_f64(1);
        let mut builder = null_mut();
        let mut builder_copy = null_mut();
        let mut built = null_mut();
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_new_f64(3, &mut builder),
            0
        );
        let mut length = 99;
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_length_f64(builder, &mut length),
            0
        );
        assert_eq!(length, 3);
        let mut presence = 99;
        let mut child = 0x1000 as *mut LiblrhsmmRsSegmentationF64;
        let mut sentinel = 0x1000 as *mut LiblrhsmmRsSegmentationSetF64;
        for index in 0..3 {
            assert_eq!(
                liblrhsmm_rs_segmentation_set_builder_has_sample_f64(builder, index, &mut presence),
                0
            );
            assert_eq!(presence, 0);
            assert_eq!(
                liblrhsmm_rs_segmentation_set_builder_get_sample_f64(builder, index, &mut child),
                3
            );
            assert_eq!(child, 0x1000 as *mut LiblrhsmmRsSegmentationF64);
        }
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_build_f64(builder, &mut sentinel),
            3
        );
        assert_eq!(sentinel, 0x1000 as *mut LiblrhsmmRsSegmentationSetF64);
        for (index, source) in [first, second, first].into_iter().enumerate() {
            assert_eq!(
                liblrhsmm_rs_segmentation_set_builder_set_sample_f64(builder, index, source),
                0
            );
        }
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_clone_f64(builder, &mut builder_copy),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_clear_sample_f64(builder, 1),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_has_sample_f64(builder, 1, &mut presence),
            0
        );
        assert_eq!(presence, 0);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_get_sample_f64(builder_copy, 1, &mut child),
            0
        );
        compare_segmentation_f64(child, &second_value);
        assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut child), 0);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_resize_f64(builder, 4),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_has_sample_f64(builder, 3, &mut presence),
            0
        );
        assert_eq!(presence, 0);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_resize_f64(builder, 1),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_build_f64(builder, &mut built),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_length_f64(built, &mut length),
            0
        );
        assert_eq!(length, 1);
        assert_eq!(liblrhsmm_rs_segmentation_set_release_f64(&mut built), 0);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_build_f64(builder_copy, &mut built),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_length_f64(built, &mut length),
            0
        );
        assert_eq!(length, 3);
        for (index, expected) in [&first_value, &second_value, &first_value]
            .into_iter()
            .enumerate()
        {
            assert_eq!(
                liblrhsmm_rs_segmentation_set_get_f64(built, index, &mut child),
                0
            );
            compare_segmentation_f64(child, expected);
            assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut child), 0);
        }
        assert_eq!(liblrhsmm_rs_segmentation_set_set_f64(built, 1, first), 0);
        assert_eq!(liblrhsmm_rs_segmentation_set_append_f64(built, second), 0);
        for (index, expected) in [&first_value, &first_value, &first_value, &second_value]
            .into_iter()
            .enumerate()
        {
            assert_eq!(
                liblrhsmm_rs_segmentation_set_get_f64(built, index, &mut child),
                0
            );
            compare_segmentation_f64(child, expected);
            assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut child), 0);
        }
        assert_eq!(
            liblrhsmm_rs_segmentation_set_length_f64(built, &mut length),
            0
        );
        assert_eq!(length, 4);
        let inputs = [first.cast_const(), second.cast_const(), first.cast_const()];
        assert_eq!(
            liblrhsmm_rs_segmentation_set_replace_f64(built, inputs.as_ptr(), inputs.len()),
            0
        );
        let mut copied = null_mut();
        let mut created = null_mut();
        assert_eq!(
            liblrhsmm_rs_segmentation_set_clone_f64(built, &mut copied),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_create_f64(inputs.as_ptr(), inputs.len(), &mut created),
            0
        );
        let bad = [first.cast_const(), null()];
        sentinel = created;
        assert_eq!(
            liblrhsmm_rs_segmentation_set_create_f64(bad.as_ptr(), 2, &mut sentinel),
            1
        );
        assert_eq!(sentinel, created);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_replace_f64(built, bad.as_ptr(), 2),
            1
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_create_f64(null(), usize::MAX, &mut sentinel),
            2
        );
        assert_eq!(sentinel, created);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_create_f64(null(), 1, &mut sentinel),
            1
        );
        assert_eq!(liblrhsmm_rs_segmentation_set_set_f64(built, 99, first), 2);
        assert_eq!(liblrhsmm_rs_segmentation_set_set_f64(built, 0, null()), 1);
        assert_eq!(liblrhsmm_rs_segmentation_set_append_f64(built, null()), 1);
        child = 0x1000 as *mut LiblrhsmmRsSegmentationF64;
        assert_eq!(
            liblrhsmm_rs_segmentation_set_get_f64(built, 99, &mut child),
            2
        );
        assert_eq!(child, 0x1000 as *mut LiblrhsmmRsSegmentationF64);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_get_f64(built, 0, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_clone_f64(built, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_length_f64(built, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_resize_f64(builder_copy, i32::MAX as usize + 1),
            2
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_length_f64(builder_copy, &mut length),
            0
        );
        assert_eq!(length, 3);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_has_sample_f64(builder_copy, 99, &mut presence),
            2
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_has_sample_f64(builder_copy, 0, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_set_sample_f64(builder_copy, 0, null()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_clear_sample_f64(builder_copy, 99),
            2
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_build_f64(builder_copy, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_clone_f64(builder_copy, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_length_f64(builder_copy, null_mut()),
            1
        );
        let mut builder_sentinel = builder;
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_new_f64(
                i32::MAX as usize + 1,
                &mut builder_sentinel
            ),
            3
        );
        assert_eq!(builder_sentinel, builder);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_new_f64(usize::MAX, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_get_f64(copied, 1, &mut child),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_release_f64(&mut builder),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_release_f64(&mut builder_copy),
            0
        );
        assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut first), 0);
        assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut second), 0);
        assert_eq!(liblrhsmm_rs_segmentation_set_release_f64(&mut copied), 0);
        compare_segmentation_f64(child, &second_value);
        assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut child), 0);
        for set in [built, created] {
            for (index, expected) in [&first_value, &second_value, &first_value]
                .into_iter()
                .enumerate()
            {
                assert_eq!(
                    liblrhsmm_rs_segmentation_set_get_f64(set, index, &mut child),
                    0
                );
                compare_segmentation_f64(child, expected);
                assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut child), 0);
            }
        }
        assert_eq!(
            liblrhsmm_rs_segmentation_set_replace_f64(built, null(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_length_f64(built, &mut length),
            0
        );
        assert_eq!(length, 0);
        assert_eq!(liblrhsmm_rs_segmentation_set_release_f64(&mut built), 0);
        assert_eq!(liblrhsmm_rs_segmentation_set_release_f64(&mut created), 0);
        assert_eq!(liblrhsmm_rs_segmentation_set_release_f64(&mut created), 0);
        assert_eq!(liblrhsmm_rs_segmentation_set_release_f64(null_mut()), 1);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_release_f64(&mut builder_copy),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_release_f64(null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_new_f64(0, &mut builder),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_build_f64(builder, &mut built),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_length_f64(built, &mut length),
            0
        );
        assert_eq!(length, 0);
        assert_eq!(liblrhsmm_rs_segmentation_set_release_f64(&mut built), 0);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_builder_release_f64(&mut builder),
            0
        );
    }
}
