use liblrhsmm_rs_capi::*;
use std::ptr::{null, null_mut};

#[test]
fn observation_stream_f32_full_bits_frames_deep_copies_and_errors() {
    // SAFETY: Independent initialized scalar buffers; all owners live and uniquely released.
    unsafe {
        let mut owner = null_mut();
        let mut clone = null_mut();
        let mut zeroed = null_mut();
        let mut values = [
            f32::from_bits(0x7fc01234),
            -0.0,
            f32::INFINITY,
            1.25,
            f32::NEG_INFINITY,
            f32::from_bits(1),
        ];
        let expected = values.map(f32::to_bits);
        assert_eq!(
            liblrhsmm_rs_observation_stream_new_f32(2, 3, &mut zeroed),
            0
        );
        let mut result = [99.0; 6];
        assert_eq!(
            liblrhsmm_rs_observation_stream_copy_values_f32(zeroed, 0, result.as_mut_ptr(), 6),
            0
        );
        assert!(
            result
                .iter()
                .all(|value| value.to_bits() == (0.0_f32).to_bits())
        );
        assert_eq!(liblrhsmm_rs_observation_stream_release_f32(&mut zeroed), 0);
        assert_eq!(
            liblrhsmm_rs_observation_stream_create_f32(3, values.as_ptr(), 6, &mut owner),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_clone_f32(owner, &mut clone),
            0
        );
        values.fill(99.0);
        let mut layout = LiblrhsmmRsObservationStreamLayout {
            frames: 99,
            dimensions: 99,
            values: 99,
        };
        assert_eq!(
            liblrhsmm_rs_observation_stream_layout_f32(owner, &mut layout),
            0
        );
        assert_eq!(
            layout,
            LiblrhsmmRsObservationStreamLayout {
                frames: 2,
                dimensions: 3,
                values: 6
            }
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_copy_values_f32(owner, 0, result.as_mut_ptr(), 6),
            0
        );
        assert_eq!(result.map(f32::to_bits), expected);
        for frame in 0..2 {
            let mut row = [99.0; 3];
            assert_eq!(
                liblrhsmm_rs_observation_stream_copy_frame_f32(owner, frame, row.as_mut_ptr(), 3),
                0
            );
            assert_eq!(row.map(f32::to_bits), expected[frame * 3..frame * 3 + 3]);
        }
        let row = [3.5, -0.0, -2.25];
        assert_eq!(
            liblrhsmm_rs_observation_stream_set_frame_f32(owner, 1, row.as_ptr(), 3),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_copy_frame_f32(owner, 1, result.as_mut_ptr(), 3),
            0
        );
        assert_eq!(
            result[..3]
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            row.map(f32::to_bits)
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_set_values_f32(owner, 1, row.as_ptr(), 3),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_copy_values_f32(owner, 1, result.as_mut_ptr(), 3),
            0
        );
        assert_eq!(
            result[..3]
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            row.map(f32::to_bits)
        );
        // Full replacement can resize both shape and storage without borrowing input.
        let changed = [1.0, 2.0, 3.0, 4.0];
        assert_eq!(
            liblrhsmm_rs_observation_stream_replace_f32(owner, 2, changed.as_ptr(), 4),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_layout_f32(owner, &mut layout),
            0
        );
        assert_eq!(
            layout,
            LiblrhsmmRsObservationStreamLayout {
                frames: 2,
                dimensions: 2,
                values: 4
            }
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_replace_f32(owner, 3, changed.as_ptr(), 4),
            3
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_replace_f32(owner, 0, null(), 0),
            3
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_replace_f32(owner, 2, null(), 4),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_replace_f32(owner, 2, changed.as_ptr(), usize::MAX),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_set_values_f32(owner, usize::MAX, changed.as_ptr(), 1),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_set_values_f32(owner, 3, changed.as_ptr(), 2),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_set_frame_f32(owner, 0, changed.as_ptr(), 1),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_set_frame_f32(owner, 2, changed.as_ptr(), 2),
            2
        );
        let misaligned = changed.as_ptr().cast::<u8>().wrapping_add(1).cast();
        assert_eq!(
            liblrhsmm_rs_observation_stream_set_values_f32(owner, 0, misaligned, 1),
            1
        );
        let mut retained = [99.0; 4];
        assert_eq!(
            liblrhsmm_rs_observation_stream_copy_values_f32(owner, 0, retained.as_mut_ptr(), 4),
            0
        );
        assert_eq!(retained, changed);
        assert_eq!(
            liblrhsmm_rs_observation_stream_copy_values_f32(owner, 4, null_mut(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_set_values_f32(owner, 4, null(), 0),
            0
        );
        retained.fill(99.0);
        assert_eq!(
            liblrhsmm_rs_observation_stream_copy_values_f32(owner, 4, retained.as_mut_ptr(), 1),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_copy_frame_f32(owner, 2, retained.as_mut_ptr(), 2),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_copy_frame_f32(owner, 0, retained.as_mut_ptr(), 3),
            2
        );
        assert_eq!(retained, [99.0; 4]);
        let old_layout = layout;
        assert_eq!(
            liblrhsmm_rs_observation_stream_layout_f32(null(), &mut layout),
            1
        );
        assert_eq!(layout, old_layout);
        assert_eq!(
            liblrhsmm_rs_observation_stream_layout_f32(owner, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_new_f32(i32::MAX as usize + 1, 1, &mut zeroed),
            3
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_new_f32(1, 0, &mut zeroed),
            3
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_create_f32(1, null(), 1, &mut zeroed),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_clone_f32(null(), &mut zeroed),
            1
        );
        assert!(zeroed.is_null());
        assert_eq!(liblrhsmm_rs_observation_stream_release_f32(&mut owner), 0);
        assert_eq!(
            liblrhsmm_rs_observation_stream_copy_values_f32(clone, 0, result.as_mut_ptr(), 6),
            0
        );
        assert_eq!(result.map(f32::to_bits), expected);
        assert_eq!(liblrhsmm_rs_observation_stream_release_f32(&mut clone), 0);
        assert_eq!(
            liblrhsmm_rs_observation_stream_create_f32(3, null(), 0, &mut owner),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_layout_f32(owner, &mut layout),
            0
        );
        assert_eq!(
            layout,
            LiblrhsmmRsObservationStreamLayout {
                frames: 0,
                dimensions: 3,
                values: 0
            }
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_copy_frame_f32(owner, 0, result.as_mut_ptr(), 3),
            2
        );
        assert_eq!(liblrhsmm_rs_observation_stream_release_f32(&mut owner), 0);
        assert_eq!(liblrhsmm_rs_observation_stream_release_f32(&mut owner), 0);
        assert_eq!(liblrhsmm_rs_observation_stream_release_f32(null_mut()), 1);
        #[cfg(feature = "serialization")]
        {
            // Each original C observation feature bit survives this stream boundary unchanged.
            let original = liblrhsmm_rs::data::ObservationData::<f32>::read_from(
                include_bytes!("../../../tests/fixtures/c-observation.bin").as_slice(),
            )
            .unwrap();
            for stream in original.streams {
                assert_eq!(
                    liblrhsmm_rs_observation_stream_create_f32(
                        stream.dimensions,
                        stream.values.as_ptr(),
                        stream.values.len(),
                        &mut owner
                    ),
                    0
                );
                let mut copied = vec![99.0; stream.values.len()];
                assert_eq!(
                    liblrhsmm_rs_observation_stream_copy_values_f32(
                        owner,
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
                    stream
                        .values
                        .iter()
                        .map(|value| value.to_bits())
                        .collect::<Vec<_>>()
                );
                assert_eq!(
                    liblrhsmm_rs_observation_stream_layout_f32(owner, &mut layout),
                    0
                );
                assert_eq!(layout.frames, original.frames);
                assert_eq!(liblrhsmm_rs_observation_stream_release_f32(&mut owner), 0);
            }
        }
    }
}

#[test]
fn observation_stream_f64_full_bits_frames_deep_copies_and_errors() {
    // SAFETY: Independent initialized scalar buffers; all owners live and uniquely released.
    unsafe {
        let mut owner = null_mut();
        let mut clone = null_mut();
        let mut zeroed = null_mut();
        let mut values = [
            f64::from_bits(0x7ff8000000001234),
            -0.0,
            f64::INFINITY,
            1.2500000000000002,
            f64::NEG_INFINITY,
            f64::from_bits(1),
        ];
        let expected = values.map(f64::to_bits);
        assert_eq!(
            liblrhsmm_rs_observation_stream_new_f64(2, 3, &mut zeroed),
            0
        );
        let mut result = [99.0; 6];
        assert_eq!(
            liblrhsmm_rs_observation_stream_copy_values_f64(zeroed, 0, result.as_mut_ptr(), 6),
            0
        );
        assert!(
            result
                .iter()
                .all(|value| value.to_bits() == (0.0_f64).to_bits())
        );
        assert_eq!(liblrhsmm_rs_observation_stream_release_f64(&mut zeroed), 0);
        assert_eq!(
            liblrhsmm_rs_observation_stream_create_f64(3, values.as_ptr(), 6, &mut owner),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_clone_f64(owner, &mut clone),
            0
        );
        values.fill(99.0);
        let mut layout = LiblrhsmmRsObservationStreamLayout {
            frames: 99,
            dimensions: 99,
            values: 99,
        };
        assert_eq!(
            liblrhsmm_rs_observation_stream_layout_f64(owner, &mut layout),
            0
        );
        assert_eq!(
            layout,
            LiblrhsmmRsObservationStreamLayout {
                frames: 2,
                dimensions: 3,
                values: 6
            }
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_copy_values_f64(owner, 0, result.as_mut_ptr(), 6),
            0
        );
        assert_eq!(result.map(f64::to_bits), expected);
        for frame in 0..2 {
            let mut row = [99.0; 3];
            assert_eq!(
                liblrhsmm_rs_observation_stream_copy_frame_f64(owner, frame, row.as_mut_ptr(), 3),
                0
            );
            assert_eq!(row.map(f64::to_bits), expected[frame * 3..frame * 3 + 3]);
        }
        let row = [3.5, -0.0, -2.25];
        assert_eq!(
            liblrhsmm_rs_observation_stream_set_frame_f64(owner, 1, row.as_ptr(), 3),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_copy_frame_f64(owner, 1, result.as_mut_ptr(), 3),
            0
        );
        assert_eq!(
            result[..3]
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            row.map(f64::to_bits)
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_set_values_f64(owner, 1, row.as_ptr(), 3),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_copy_values_f64(owner, 1, result.as_mut_ptr(), 3),
            0
        );
        assert_eq!(
            result[..3]
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            row.map(f64::to_bits)
        );
        // Full replacement can resize both shape and storage without borrowing input.
        let changed = [1.0, 2.0, 3.0, 4.0];
        assert_eq!(
            liblrhsmm_rs_observation_stream_replace_f64(owner, 2, changed.as_ptr(), 4),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_layout_f64(owner, &mut layout),
            0
        );
        assert_eq!(
            layout,
            LiblrhsmmRsObservationStreamLayout {
                frames: 2,
                dimensions: 2,
                values: 4
            }
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_replace_f64(owner, 3, changed.as_ptr(), 4),
            3
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_replace_f64(owner, 0, null(), 0),
            3
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_replace_f64(owner, 2, null(), 4),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_replace_f64(owner, 2, changed.as_ptr(), usize::MAX),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_set_values_f64(owner, usize::MAX, changed.as_ptr(), 1),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_set_values_f64(owner, 3, changed.as_ptr(), 2),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_set_frame_f64(owner, 0, changed.as_ptr(), 1),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_set_frame_f64(owner, 2, changed.as_ptr(), 2),
            2
        );
        let misaligned = changed.as_ptr().cast::<u8>().wrapping_add(1).cast();
        assert_eq!(
            liblrhsmm_rs_observation_stream_set_values_f64(owner, 0, misaligned, 1),
            1
        );
        let mut retained = [99.0; 4];
        assert_eq!(
            liblrhsmm_rs_observation_stream_copy_values_f64(owner, 0, retained.as_mut_ptr(), 4),
            0
        );
        assert_eq!(retained, changed);
        assert_eq!(
            liblrhsmm_rs_observation_stream_copy_values_f64(owner, 4, null_mut(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_set_values_f64(owner, 4, null(), 0),
            0
        );
        retained.fill(99.0);
        assert_eq!(
            liblrhsmm_rs_observation_stream_copy_values_f64(owner, 4, retained.as_mut_ptr(), 1),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_copy_frame_f64(owner, 2, retained.as_mut_ptr(), 2),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_copy_frame_f64(owner, 0, retained.as_mut_ptr(), 3),
            2
        );
        assert_eq!(retained, [99.0; 4]);
        let old_layout = layout;
        assert_eq!(
            liblrhsmm_rs_observation_stream_layout_f64(null(), &mut layout),
            1
        );
        assert_eq!(layout, old_layout);
        assert_eq!(
            liblrhsmm_rs_observation_stream_layout_f64(owner, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_new_f64(i32::MAX as usize + 1, 1, &mut zeroed),
            3
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_new_f64(1, 0, &mut zeroed),
            3
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_create_f64(1, null(), 1, &mut zeroed),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_clone_f64(null(), &mut zeroed),
            1
        );
        assert!(zeroed.is_null());
        assert_eq!(liblrhsmm_rs_observation_stream_release_f64(&mut owner), 0);
        assert_eq!(
            liblrhsmm_rs_observation_stream_copy_values_f64(clone, 0, result.as_mut_ptr(), 6),
            0
        );
        assert_eq!(result.map(f64::to_bits), expected);
        assert_eq!(liblrhsmm_rs_observation_stream_release_f64(&mut clone), 0);
        assert_eq!(
            liblrhsmm_rs_observation_stream_create_f64(3, null(), 0, &mut owner),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_layout_f64(owner, &mut layout),
            0
        );
        assert_eq!(
            layout,
            LiblrhsmmRsObservationStreamLayout {
                frames: 0,
                dimensions: 3,
                values: 0
            }
        );
        assert_eq!(
            liblrhsmm_rs_observation_stream_copy_frame_f64(owner, 0, result.as_mut_ptr(), 3),
            2
        );
        assert_eq!(liblrhsmm_rs_observation_stream_release_f64(&mut owner), 0);
        assert_eq!(liblrhsmm_rs_observation_stream_release_f64(&mut owner), 0);
        assert_eq!(liblrhsmm_rs_observation_stream_release_f64(null_mut()), 1);
        #[cfg(feature = "serialization")]
        {
            // Each original C observation feature bit survives this stream boundary unchanged.
            let original = liblrhsmm_rs::data::ObservationData::<f64>::read_from(
                include_bytes!("../../../tests/fixtures/c-observation.bin").as_slice(),
            )
            .unwrap();
            for stream in original.streams {
                assert_eq!(
                    liblrhsmm_rs_observation_stream_create_f64(
                        stream.dimensions,
                        stream.values.as_ptr(),
                        stream.values.len(),
                        &mut owner
                    ),
                    0
                );
                let mut copied = vec![99.0; stream.values.len()];
                assert_eq!(
                    liblrhsmm_rs_observation_stream_copy_values_f64(
                        owner,
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
                    stream
                        .values
                        .iter()
                        .map(|value| value.to_bits())
                        .collect::<Vec<_>>()
                );
                assert_eq!(
                    liblrhsmm_rs_observation_stream_layout_f64(owner, &mut layout),
                    0
                );
                assert_eq!(layout.frames, original.frames);
                assert_eq!(liblrhsmm_rs_observation_stream_release_f64(&mut owner), 0);
            }
        }
    }
}
