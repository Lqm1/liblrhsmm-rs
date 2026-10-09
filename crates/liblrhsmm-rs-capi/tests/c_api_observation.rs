use liblrhsmm_rs_capi::*;
use std::ptr::{null, null_mut};

#[test]
fn observation_f32_full_shape_bits_copies_frames_and_atomic_errors() {
    // SAFETY: Independent initialized buffers and live uniquely released owners.
    unsafe {
        let mut first = null_mut();
        let mut second = null_mut();
        let mut owner = null_mut();
        let mut clone = null_mut();
        let mut child = null_mut();
        let mut zeroed = null_mut();
        let bits = [
            0x7fc01234u32,
            0x80000000,
            0x7f800000,
            0x3fa00000,
            0xff800000,
            1,
        ];
        let values = bits.map(f32::from_bits);
        let dimensions = [2, 3];
        assert_eq!(
            liblrhsmm_rs_observation_new_f32(2, dimensions.as_ptr(), 2, &mut zeroed),
            0
        );
        let mut copied = [99.0; 6];
        for (stream, &dimension) in dimensions.iter().enumerate() {
            let count = 2 * dimension;
            assert_eq!(
                liblrhsmm_rs_observation_copy_values_f32(
                    zeroed,
                    stream,
                    0,
                    copied.as_mut_ptr(),
                    count
                ),
                0
            );
            assert!(copied[..count].iter().all(|value| value.to_bits() == 0));
        }
        assert_eq!(liblrhsmm_rs_observation_release_f32(&mut zeroed), 0);
        assert_eq!(
            liblrhsmm_rs_observation_stream_create_f32(2, values.as_ptr(), 4, &mut first),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_create_f32(3, values.as_ptr(), 6, &mut second),
            0
        );
        let handles = [first.cast_const(), second.cast_const()];
        assert_eq!(
            liblrhsmm_rs_observation_create_f32(2, handles.as_ptr(), 2, &mut owner),
            0
        );
        assert_eq!(liblrhsmm_rs_observation_clone_f32(owner, &mut clone), 0);
        let mut layout = LiblrhsmmRsObservationLayout {
            frames: 99,
            streams: 99,
        };
        let mut stream_layout = LiblrhsmmRsObservationStreamLayout {
            frames: 99,
            dimensions: 99,
            values: 99,
        };
        assert_eq!(liblrhsmm_rs_observation_layout_f32(owner, &mut layout), 0);
        assert_eq!(
            layout,
            LiblrhsmmRsObservationLayout {
                frames: 2,
                streams: 2
            }
        );
        for (stream, &dimension) in dimensions.iter().enumerate() {
            assert_eq!(
                liblrhsmm_rs_observation_get_stream_layout_f32(owner, stream, &mut stream_layout),
                0
            );
            assert_eq!(
                stream_layout,
                LiblrhsmmRsObservationStreamLayout {
                    frames: 2,
                    dimensions: dimension,
                    values: dimension * 2
                }
            );
            assert_eq!(
                liblrhsmm_rs_observation_copy_values_f32(
                    owner,
                    stream,
                    0,
                    copied.as_mut_ptr(),
                    dimension * 2
                ),
                0
            );
            assert_eq!(
                copied[..dimension * 2]
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>(),
                bits[..dimension * 2]
            );
            assert_eq!(
                liblrhsmm_rs_observation_copy_frame_f32(
                    owner,
                    stream,
                    1,
                    copied.as_mut_ptr(),
                    dimension
                ),
                0
            );
            assert_eq!(
                copied[..dimension]
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>(),
                bits[dimension..dimension * 2]
            );
        }
        // A returned child and whole clone outlive every source without sharing mutations.
        assert_eq!(
            liblrhsmm_rs_observation_get_stream_f32(owner, 1, &mut child),
            0
        );
        let row = [4.0, -0.0, 9.0];
        assert_eq!(
            liblrhsmm_rs_observation_stream_set_values_f32(second, 0, row.as_ptr(), 3),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_copy_values_f32(owner, 1, 0, copied.as_mut_ptr(), 6),
            0
        );
        assert_eq!(copied.map(f32::to_bits), bits);
        assert_eq!(
            liblrhsmm_rs_observation_set_frame_f32(owner, 1, 1, row.as_ptr(), 3),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_values_f32(owner, 1, 0, row.as_ptr(), 3),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_copy_frame_f32(owner, 1, 0, copied.as_mut_ptr(), 3),
            0
        );
        assert_eq!(
            copied[..3]
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            row.map(f32::to_bits)
        );
        assert_eq!(liblrhsmm_rs_observation_set_stream_f32(owner, 0, second), 0);
        assert_eq!(
            liblrhsmm_rs_observation_get_stream_layout_f32(owner, 0, &mut stream_layout),
            0
        );
        assert_eq!(stream_layout.dimensions, 3);
        let before = copied;
        assert_eq!(
            liblrhsmm_rs_observation_copy_values_f32(owner, 1, 0, copied.as_mut_ptr(), 6),
            0
        );
        let owner_bits = copied.map(f32::to_bits);
        assert_eq!(
            liblrhsmm_rs_observation_stream_replace_f32(first, 1, row.as_ptr(), 3),
            0
        );
        assert_eq!(liblrhsmm_rs_observation_set_stream_f32(owner, 1, first), 3);
        assert_eq!(
            liblrhsmm_rs_observation_replace_f32(owner, 2, &first.cast_const(), 1),
            3
        );
        assert_eq!(liblrhsmm_rs_observation_replace_f32(owner, 2, null(), 0), 3);
        assert_eq!(
            liblrhsmm_rs_observation_replace_f32(owner, i32::MAX as usize + 1, handles.as_ptr(), 2),
            3
        );
        assert_eq!(liblrhsmm_rs_observation_set_stream_f32(owner, 2, second), 2);
        assert_eq!(
            liblrhsmm_rs_observation_set_values_f32(owner, 1, usize::MAX, row.as_ptr(), 1),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_values_f32(owner, 1, 6, row.as_ptr(), 1),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_values_f32(owner, 1, 0, null(), 1),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_frame_f32(owner, 1, 0, row.as_ptr(), 2),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_frame_f32(owner, 1, 2, row.as_ptr(), 3),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_copy_values_f32(owner, 1, 0, copied.as_mut_ptr(), 6),
            0
        );
        assert_eq!(copied.map(f32::to_bits), owner_bits);
        copied = before;
        assert_eq!(
            liblrhsmm_rs_observation_copy_frame_f32(owner, 1, 2, copied.as_mut_ptr(), 3),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_copy_frame_f32(owner, 1, 0, copied.as_mut_ptr(), 2),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_copy_values_f32(owner, 2, 0, copied.as_mut_ptr(), 3),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_copy_values_f32(owner, 1, 0, copied.as_mut_ptr(), usize::MAX),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_copy_values_f32(
                owner,
                1,
                0,
                copied.as_mut_ptr().cast::<u8>().add(1).cast(),
                1
            ),
            1
        );
        assert_eq!(copied.map(f32::to_bits), before.map(f32::to_bits));
        let sentinel_layout = layout;
        assert_eq!(liblrhsmm_rs_observation_layout_f32(null(), &mut layout), 1);
        assert_eq!(layout, sentinel_layout);
        let sentinel_stream_layout = stream_layout;
        assert_eq!(
            liblrhsmm_rs_observation_get_stream_layout_f32(owner, 2, &mut stream_layout),
            2
        );
        assert_eq!(stream_layout, sentinel_stream_layout);
        let mut sentinel = owner;
        assert_eq!(
            liblrhsmm_rs_observation_new_f32(2, null(), 1, &mut sentinel),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_new_f32(2, [0].as_ptr(), 1, &mut sentinel),
            3
        );
        assert_eq!(
            liblrhsmm_rs_observation_new_f32(2, [i32::MAX as usize + 1].as_ptr(), 1, &mut sentinel),
            3
        );
        assert_eq!(
            liblrhsmm_rs_observation_new_f32(2, dimensions.as_ptr(), usize::MAX, &mut sentinel),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_create_f32(2, [null()].as_ptr(), 1, &mut sentinel),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_create_f32(2, handles.as_ptr(), 2, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_get_stream_f32(owner, 2, &mut child),
            2
        );
        assert_eq!(sentinel, owner);
        assert_eq!(
            liblrhsmm_rs_observation_copy_values_f32(owner, 1, 6, null_mut(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_values_f32(owner, 1, 6, null(), 0),
            0
        );
        // Repeated child references are independently copied, including a complete reshape.
        let repeated = [
            second.cast_const(),
            second.cast_const(),
            second.cast_const(),
        ];
        assert_eq!(
            liblrhsmm_rs_observation_replace_f32(owner, 2, repeated.as_ptr(), 3),
            0
        );
        assert_eq!(liblrhsmm_rs_observation_layout_f32(owner, &mut layout), 0);
        assert_eq!(
            layout,
            LiblrhsmmRsObservationLayout {
                frames: 2,
                streams: 3
            }
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_values_f32(owner, 0, 0, [12.0].as_ptr(), 1),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_copy_values_f32(owner, 1, 0, copied.as_mut_ptr(), 1),
            0
        );
        assert_eq!(copied[0].to_bits(), row[0].to_bits());
        assert_eq!(liblrhsmm_rs_observation_release_f32(&mut owner), 0);
        assert_eq!(liblrhsmm_rs_observation_stream_release_f32(&mut first), 0);
        assert_eq!(liblrhsmm_rs_observation_stream_release_f32(&mut second), 0);
        assert_eq!(
            liblrhsmm_rs_observation_copy_values_f32(clone, 1, 0, copied.as_mut_ptr(), 6),
            0
        );
        assert_eq!(copied.map(f32::to_bits), bits);
        assert_eq!(liblrhsmm_rs_observation_release_f32(&mut clone), 0);
        assert_eq!(
            liblrhsmm_rs_observation_stream_copy_values_f32(child, 0, copied.as_mut_ptr(), 6),
            0
        );
        assert_eq!(copied.map(f32::to_bits), bits);
        assert_eq!(liblrhsmm_rs_observation_stream_release_f32(&mut child), 0);
        assert_eq!(
            liblrhsmm_rs_observation_new_f32(0, dimensions.as_ptr(), 2, &mut owner),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_copy_values_f32(owner, 0, 0, null_mut(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_copy_frame_f32(owner, 0, 0, copied.as_mut_ptr(), 2),
            2
        );
        assert_eq!(liblrhsmm_rs_observation_release_f32(&mut owner), 0);
        assert_eq!(liblrhsmm_rs_observation_release_f32(&mut owner), 0);
        assert_eq!(liblrhsmm_rs_observation_release_f32(null_mut()), 1);
        #[cfg(feature = "serialization")]
        {
            let original = liblrhsmm_rs::data::ObservationData::<f32>::read_from(
                include_bytes!("../../../tests/fixtures/c-observation.bin").as_slice(),
            )
            .unwrap();
            let mut children = Vec::new();
            for stream in &original.streams {
                let mut temporary = null_mut();
                assert_eq!(
                    liblrhsmm_rs_observation_stream_create_f32(
                        stream.dimensions,
                        stream.values.as_ptr(),
                        stream.values.len(),
                        &mut temporary
                    ),
                    0
                );
                children.push(temporary);
            }
            let inputs: Vec<_> = children.iter().map(|owner| owner.cast_const()).collect();
            assert_eq!(
                liblrhsmm_rs_observation_create_f32(
                    original.frames,
                    inputs.as_ptr(),
                    inputs.len(),
                    &mut owner
                ),
                0
            );
            for child in &mut children {
                assert_eq!(liblrhsmm_rs_observation_stream_release_f32(child), 0);
            }
            for (stream, expected) in original.streams.iter().enumerate() {
                let mut result = vec![99.0; expected.values.len()];
                assert_eq!(
                    liblrhsmm_rs_observation_copy_values_f32(
                        owner,
                        stream,
                        0,
                        result.as_mut_ptr(),
                        result.len()
                    ),
                    0
                );
                assert_eq!(
                    result
                        .iter()
                        .map(|value| value.to_bits())
                        .collect::<Vec<_>>(),
                    expected
                        .values
                        .iter()
                        .map(|value| value.to_bits())
                        .collect::<Vec<_>>()
                );
            }
            assert_eq!(liblrhsmm_rs_observation_release_f32(&mut owner), 0);
        }
    }
}

#[test]
fn observation_f64_full_shape_bits_copies_frames_and_atomic_errors() {
    // SAFETY: Independent initialized buffers and live uniquely released owners.
    unsafe {
        let mut first = null_mut();
        let mut second = null_mut();
        let mut owner = null_mut();
        let mut clone = null_mut();
        let mut child = null_mut();
        let mut zeroed = null_mut();
        let bits = [
            0x7ff8000000001234u64,
            0x8000000000000000,
            0x7ff0000000000000,
            0x3ff4000000000001,
            0xfff0000000000000,
            1,
        ];
        let values = bits.map(f64::from_bits);
        let dimensions = [2, 3];
        assert_eq!(
            liblrhsmm_rs_observation_new_f64(2, dimensions.as_ptr(), 2, &mut zeroed),
            0
        );
        let mut copied = [99.0; 6];
        for (stream, &dimension) in dimensions.iter().enumerate() {
            let count = 2 * dimension;
            assert_eq!(
                liblrhsmm_rs_observation_copy_values_f64(
                    zeroed,
                    stream,
                    0,
                    copied.as_mut_ptr(),
                    count
                ),
                0
            );
            assert!(copied[..count].iter().all(|value| value.to_bits() == 0));
        }
        assert_eq!(liblrhsmm_rs_observation_release_f64(&mut zeroed), 0);
        assert_eq!(
            liblrhsmm_rs_observation_stream_create_f64(2, values.as_ptr(), 4, &mut first),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_create_f64(3, values.as_ptr(), 6, &mut second),
            0
        );
        let handles = [first.cast_const(), second.cast_const()];
        assert_eq!(
            liblrhsmm_rs_observation_create_f64(2, handles.as_ptr(), 2, &mut owner),
            0
        );
        assert_eq!(liblrhsmm_rs_observation_clone_f64(owner, &mut clone), 0);
        let mut layout = LiblrhsmmRsObservationLayout {
            frames: 99,
            streams: 99,
        };
        let mut stream_layout = LiblrhsmmRsObservationStreamLayout {
            frames: 99,
            dimensions: 99,
            values: 99,
        };
        assert_eq!(liblrhsmm_rs_observation_layout_f64(owner, &mut layout), 0);
        assert_eq!(
            layout,
            LiblrhsmmRsObservationLayout {
                frames: 2,
                streams: 2
            }
        );
        for (stream, &dimension) in dimensions.iter().enumerate() {
            assert_eq!(
                liblrhsmm_rs_observation_get_stream_layout_f64(owner, stream, &mut stream_layout),
                0
            );
            assert_eq!(
                stream_layout,
                LiblrhsmmRsObservationStreamLayout {
                    frames: 2,
                    dimensions: dimension,
                    values: dimension * 2
                }
            );
            assert_eq!(
                liblrhsmm_rs_observation_copy_values_f64(
                    owner,
                    stream,
                    0,
                    copied.as_mut_ptr(),
                    dimension * 2
                ),
                0
            );
            assert_eq!(
                copied[..dimension * 2]
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>(),
                bits[..dimension * 2]
            );
            assert_eq!(
                liblrhsmm_rs_observation_copy_frame_f64(
                    owner,
                    stream,
                    1,
                    copied.as_mut_ptr(),
                    dimension
                ),
                0
            );
            assert_eq!(
                copied[..dimension]
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>(),
                bits[dimension..dimension * 2]
            );
        }
        // A returned child and whole clone outlive every source without sharing mutations.
        assert_eq!(
            liblrhsmm_rs_observation_get_stream_f64(owner, 1, &mut child),
            0
        );
        let row = [4.0, -0.0, 9.0];
        assert_eq!(
            liblrhsmm_rs_observation_stream_set_values_f64(second, 0, row.as_ptr(), 3),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_copy_values_f64(owner, 1, 0, copied.as_mut_ptr(), 6),
            0
        );
        assert_eq!(copied.map(f64::to_bits), bits);
        assert_eq!(
            liblrhsmm_rs_observation_set_frame_f64(owner, 1, 1, row.as_ptr(), 3),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_values_f64(owner, 1, 0, row.as_ptr(), 3),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_copy_frame_f64(owner, 1, 0, copied.as_mut_ptr(), 3),
            0
        );
        assert_eq!(
            copied[..3]
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            row.map(f64::to_bits)
        );
        assert_eq!(liblrhsmm_rs_observation_set_stream_f64(owner, 0, second), 0);
        assert_eq!(
            liblrhsmm_rs_observation_get_stream_layout_f64(owner, 0, &mut stream_layout),
            0
        );
        assert_eq!(stream_layout.dimensions, 3);
        let before = copied;
        assert_eq!(
            liblrhsmm_rs_observation_copy_values_f64(owner, 1, 0, copied.as_mut_ptr(), 6),
            0
        );
        let owner_bits = copied.map(f64::to_bits);
        assert_eq!(
            liblrhsmm_rs_observation_stream_replace_f64(first, 1, row.as_ptr(), 3),
            0
        );
        assert_eq!(liblrhsmm_rs_observation_set_stream_f64(owner, 1, first), 3);
        assert_eq!(
            liblrhsmm_rs_observation_replace_f64(owner, 2, &first.cast_const(), 1),
            3
        );
        assert_eq!(liblrhsmm_rs_observation_replace_f64(owner, 2, null(), 0), 3);
        assert_eq!(
            liblrhsmm_rs_observation_replace_f64(owner, i32::MAX as usize + 1, handles.as_ptr(), 2),
            3
        );
        assert_eq!(liblrhsmm_rs_observation_set_stream_f64(owner, 2, second), 2);
        assert_eq!(
            liblrhsmm_rs_observation_set_values_f64(owner, 1, usize::MAX, row.as_ptr(), 1),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_values_f64(owner, 1, 6, row.as_ptr(), 1),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_values_f64(owner, 1, 0, null(), 1),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_frame_f64(owner, 1, 0, row.as_ptr(), 2),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_frame_f64(owner, 1, 2, row.as_ptr(), 3),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_copy_values_f64(owner, 1, 0, copied.as_mut_ptr(), 6),
            0
        );
        assert_eq!(copied.map(f64::to_bits), owner_bits);
        copied = before;
        assert_eq!(
            liblrhsmm_rs_observation_copy_frame_f64(owner, 1, 2, copied.as_mut_ptr(), 3),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_copy_frame_f64(owner, 1, 0, copied.as_mut_ptr(), 2),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_copy_values_f64(owner, 2, 0, copied.as_mut_ptr(), 3),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_copy_values_f64(owner, 1, 0, copied.as_mut_ptr(), usize::MAX),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_copy_values_f64(
                owner,
                1,
                0,
                copied.as_mut_ptr().cast::<u8>().add(1).cast(),
                1
            ),
            1
        );
        assert_eq!(copied.map(f64::to_bits), before.map(f64::to_bits));
        let sentinel_layout = layout;
        assert_eq!(liblrhsmm_rs_observation_layout_f64(null(), &mut layout), 1);
        assert_eq!(layout, sentinel_layout);
        let sentinel_stream_layout = stream_layout;
        assert_eq!(
            liblrhsmm_rs_observation_get_stream_layout_f64(owner, 2, &mut stream_layout),
            2
        );
        assert_eq!(stream_layout, sentinel_stream_layout);
        let mut sentinel = owner;
        assert_eq!(
            liblrhsmm_rs_observation_new_f64(2, null(), 1, &mut sentinel),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_new_f64(2, [0].as_ptr(), 1, &mut sentinel),
            3
        );
        assert_eq!(
            liblrhsmm_rs_observation_new_f64(2, [i32::MAX as usize + 1].as_ptr(), 1, &mut sentinel),
            3
        );
        assert_eq!(
            liblrhsmm_rs_observation_new_f64(2, dimensions.as_ptr(), usize::MAX, &mut sentinel),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_create_f64(2, [null()].as_ptr(), 1, &mut sentinel),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_create_f64(2, handles.as_ptr(), 2, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_get_stream_f64(owner, 2, &mut child),
            2
        );
        assert_eq!(sentinel, owner);
        assert_eq!(
            liblrhsmm_rs_observation_copy_values_f64(owner, 1, 6, null_mut(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_values_f64(owner, 1, 6, null(), 0),
            0
        );
        // Repeated child references are independently copied, including a complete reshape.
        let repeated = [
            second.cast_const(),
            second.cast_const(),
            second.cast_const(),
        ];
        assert_eq!(
            liblrhsmm_rs_observation_replace_f64(owner, 2, repeated.as_ptr(), 3),
            0
        );
        assert_eq!(liblrhsmm_rs_observation_layout_f64(owner, &mut layout), 0);
        assert_eq!(
            layout,
            LiblrhsmmRsObservationLayout {
                frames: 2,
                streams: 3
            }
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_values_f64(owner, 0, 0, [12.0].as_ptr(), 1),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_copy_values_f64(owner, 1, 0, copied.as_mut_ptr(), 1),
            0
        );
        assert_eq!(copied[0].to_bits(), row[0].to_bits());
        assert_eq!(liblrhsmm_rs_observation_release_f64(&mut owner), 0);
        assert_eq!(liblrhsmm_rs_observation_stream_release_f64(&mut first), 0);
        assert_eq!(liblrhsmm_rs_observation_stream_release_f64(&mut second), 0);
        assert_eq!(
            liblrhsmm_rs_observation_copy_values_f64(clone, 1, 0, copied.as_mut_ptr(), 6),
            0
        );
        assert_eq!(copied.map(f64::to_bits), bits);
        assert_eq!(liblrhsmm_rs_observation_release_f64(&mut clone), 0);
        assert_eq!(
            liblrhsmm_rs_observation_stream_copy_values_f64(child, 0, copied.as_mut_ptr(), 6),
            0
        );
        assert_eq!(copied.map(f64::to_bits), bits);
        assert_eq!(liblrhsmm_rs_observation_stream_release_f64(&mut child), 0);
        assert_eq!(
            liblrhsmm_rs_observation_new_f64(0, dimensions.as_ptr(), 2, &mut owner),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_copy_values_f64(owner, 0, 0, null_mut(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_copy_frame_f64(owner, 0, 0, copied.as_mut_ptr(), 2),
            2
        );
        assert_eq!(liblrhsmm_rs_observation_release_f64(&mut owner), 0);
        assert_eq!(liblrhsmm_rs_observation_release_f64(&mut owner), 0);
        assert_eq!(liblrhsmm_rs_observation_release_f64(null_mut()), 1);
        #[cfg(feature = "serialization")]
        {
            let original = liblrhsmm_rs::data::ObservationData::<f64>::read_from(
                include_bytes!("../../../tests/fixtures/c-observation.bin").as_slice(),
            )
            .unwrap();
            let mut children = Vec::new();
            for stream in &original.streams {
                let mut temporary = null_mut();
                assert_eq!(
                    liblrhsmm_rs_observation_stream_create_f64(
                        stream.dimensions,
                        stream.values.as_ptr(),
                        stream.values.len(),
                        &mut temporary
                    ),
                    0
                );
                children.push(temporary);
            }
            let inputs: Vec<_> = children.iter().map(|owner| owner.cast_const()).collect();
            assert_eq!(
                liblrhsmm_rs_observation_create_f64(
                    original.frames,
                    inputs.as_ptr(),
                    inputs.len(),
                    &mut owner
                ),
                0
            );
            for child in &mut children {
                assert_eq!(liblrhsmm_rs_observation_stream_release_f64(child), 0);
            }
            for (stream, expected) in original.streams.iter().enumerate() {
                let mut result = vec![99.0; expected.values.len()];
                assert_eq!(
                    liblrhsmm_rs_observation_copy_values_f64(
                        owner,
                        stream,
                        0,
                        result.as_mut_ptr(),
                        result.len()
                    ),
                    0
                );
                assert_eq!(
                    result
                        .iter()
                        .map(|value| value.to_bits())
                        .collect::<Vec<_>>(),
                    expected
                        .values
                        .iter()
                        .map(|value| value.to_bits())
                        .collect::<Vec<_>>()
                );
            }
            assert_eq!(liblrhsmm_rs_observation_release_f64(&mut owner), 0);
        }
    }
}
