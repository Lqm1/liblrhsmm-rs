use liblrhsmm_rs::StateOccurrence;
use liblrhsmm_rs_capi::*;

use liblrhsmm_rs::data::{JumpParameters, SegmentationData};
use std::ptr::{null, null_mut};

unsafe fn compare_f32(owner: *const LiblrhsmmRsSegmentationF32, expected: &SegmentationData<f32>) {
    // SAFETY: Live readable owner and independent initialized caller buffers.
    unsafe {
        let mut shape = LiblrhsmmRsSegmentationLayout {
            streams: 99,
            segments: 99,
        };
        assert_eq!(liblrhsmm_rs_segmentation_layout_f32(owner, &mut shape), 0);
        assert_eq!(
            shape,
            LiblrhsmmRsSegmentationLayout {
                streams: expected.output_states.len(),
                segments: expected.boundaries.len()
            }
        );
        let mut integers = vec![99; expected.boundaries.len()];
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_boundaries_f32(
                owner,
                0,
                integers.as_mut_ptr(),
                integers.len()
            ),
            0
        );
        assert_eq!(integers, expected.boundaries);
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_duration_states_f32(
                owner,
                0,
                integers.as_mut_ptr(),
                integers.len()
            ),
            0
        );
        assert_eq!(integers, expected.duration_states);
        for (stream, states) in expected.output_states.iter().enumerate() {
            assert_eq!(
                liblrhsmm_rs_segmentation_copy_output_states_f32(
                    owner,
                    stream,
                    0,
                    integers.as_mut_ptr(),
                    integers.len()
                ),
                0
            );
            assert_eq!(&integers, states);
        }
        for (segment, group) in expected.outgoing.iter().enumerate() {
            let mut count = 99;
            assert_eq!(
                liblrhsmm_rs_segmentation_outgoing_count_f32(owner, segment, &mut count),
                0
            );
            assert_eq!(count, group.len());
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
                    segment,
                    0,
                    jumps.as_mut_ptr(),
                    count
                ),
                0
            );
            for (actual, expected) in jumps.iter().zip(group) {
                assert_eq!(actual.delta, expected.delta);
                assert_eq!(actual.probability.to_bits(), expected.probability.to_bits());
            }
        }
    }
}
unsafe fn create_f32(source: &SegmentationData<f32>) -> *mut LiblrhsmmRsSegmentationF32 {
    let rows: Vec<_> = source
        .output_states
        .iter()
        .map(|values| values.as_ptr())
        .collect();
    let jumps: Vec<Vec<_>> = source
        .outgoing
        .iter()
        .map(|group| {
            group
                .iter()
                .map(|jump| LiblrhsmmRsJumpF32 {
                    delta: jump.delta,
                    probability: jump.probability,
                })
                .collect()
        })
        .collect();
    let groups: Vec<_> = jumps
        .iter()
        .map(|values| LiblrhsmmRsJumpSliceF32 {
            jumps: values.as_ptr(),
            count: values.len(),
        })
        .collect();
    let mut owner = null_mut();
    // SAFETY: All borrowed inputs remain live; output starts unowned.
    unsafe {
        assert_eq!(
            liblrhsmm_rs_segmentation_create_f32(
                rows.len(),
                source.boundaries.len(),
                source.boundaries.as_ptr(),
                source.duration_states.as_ptr(),
                rows.as_ptr(),
                groups.as_ptr(),
                &mut owner
            ),
            0
        );
    }
    owner
}

#[test]
fn segmentation_f32_full_bits_fields_copies_mutations_and_atomic_errors() {
    // SAFETY: Independent initialized buffers, live owners, and unique releases.
    unsafe {
        let mut source = SegmentationData::<f32>::new(2, 3);
        source.boundaries = vec![i32::MIN, 5, i32::MAX];
        source.duration_states = vec![-1, i32::MAX, i32::MIN];
        source.output_states = vec![vec![i32::MIN, 4, i32::MAX]; 2];
        let probabilities = [
            0x7fc01234u32,
            1,
            0x80000000,
            0x7f800000,
            0xff800000,
            0x3fa00000,
        ]
        .map(f32::from_bits);
        source.outgoing = vec![
            vec![
                JumpParameters {
                    delta: 2,
                    probability: probabilities[0],
                },
                JumpParameters {
                    delta: -2,
                    probability: probabilities[1],
                },
                JumpParameters {
                    delta: 1,
                    probability: probabilities[2],
                },
            ],
            vec![JumpParameters {
                delta: 1,
                probability: probabilities[3],
            }],
            vec![
                JumpParameters {
                    delta: 0,
                    probability: probabilities[4],
                },
                JumpParameters {
                    delta: 1,
                    probability: probabilities[5],
                },
            ],
        ];
        let original = source.clone();
        let mut expected = original.clone();
        let mut owner = create_f32(&source);
        let mut clone = null_mut();
        let mut zeroed = null_mut();
        let mut resegmented = null_mut();
        assert_eq!(liblrhsmm_rs_segmentation_new_f32(2, 3, &mut zeroed), 0);
        compare_f32(zeroed, &SegmentationData::new(2, 3));
        assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut zeroed), 0);
        assert_eq!(liblrhsmm_rs_segmentation_clone_f32(owner, &mut clone), 0);
        source.boundaries.fill(9);
        source.output_states[0].fill(8);
        source.outgoing[0][0].probability = 1.0;
        drop(source);
        compare_f32(owner, &expected);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_boundaries_f32(owner, 1, [4].as_ptr(), 1),
            0
        );
        expected.boundaries[1] = 4;
        assert_eq!(
            liblrhsmm_rs_segmentation_set_duration_states_f32(owner, 0, [8, i32::MIN].as_ptr(), 2),
            0
        );
        expected.duration_states[..2].copy_from_slice(&[8, i32::MIN]);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_output_states_f32(owner, 0, 1, [7, i32::MAX].as_ptr(), 2),
            0
        );
        expected.output_states[0][1..].copy_from_slice(&[7, i32::MAX]);
        let replacement = [
            LiblrhsmmRsJumpF32 {
                delta: 2,
                probability: probabilities[5],
            },
            LiblrhsmmRsJumpF32 {
                delta: 1,
                probability: 1.0,
            },
        ];
        assert_eq!(
            liblrhsmm_rs_segmentation_set_outgoing_f32(owner, 1, replacement.as_ptr(), 2),
            0
        );
        expected.outgoing[1] = replacement
            .iter()
            .map(|jump| JumpParameters {
                delta: jump.delta,
                probability: jump.probability,
            })
            .collect();
        assert_eq!(
            liblrhsmm_rs_segmentation_add_jump_f32(owner, 2, 1, 0.125),
            0
        );
        expected.add_jump(2, 1, 0.125).unwrap();
        assert_eq!(liblrhsmm_rs_segmentation_add_jump_f32(owner, 0, 0, 0.5), 0);
        expected.add_jump(0, 0, 0.5).unwrap();
        compare_f32(owner, &expected);
        let occurrences = [
            LiblrhsmmRsStateOccurrence { state: 2, end: 3 },
            LiblrhsmmRsStateOccurrence { state: 0, end: 4 },
            LiblrhsmmRsStateOccurrence { state: 2, end: 4 },
        ];
        let path: Vec<_> = occurrences
            .iter()
            .map(|value| StateOccurrence {
                state: value.state,
                end: value.end,
            })
            .collect();
        assert_eq!(
            liblrhsmm_rs_segmentation_resegment_f32(
                owner,
                occurrences.as_ptr(),
                3,
                &mut resegmented
            ),
            0
        );
        compare_f32(resegmented, &expected.resegment(&path).unwrap());
        assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut resegmented), 0);
        let mut integers = [99; 3];
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_boundaries_f32(
                owner,
                usize::MAX,
                integers.as_mut_ptr(),
                1
            ),
            2
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_duration_states_f32(owner, 2, integers.as_mut_ptr(), 2),
            2
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_output_states_f32(owner, 2, 0, integers.as_mut_ptr(), 3),
            2
        );
        assert_eq!(integers, [99; 3]);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_boundaries_f32(owner, 3, [1].as_ptr(), 1),
            2
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_duration_states_f32(owner, 0, null(), 1),
            1
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_output_states_f32(owner, 0, usize::MAX, [1].as_ptr(), 1),
            2
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_boundaries_f32(
                owner,
                0,
                integers.as_mut_ptr().cast::<u8>().add(1).cast(),
                1
            ),
            1
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_duration_states_f32(
                owner,
                0,
                integers.as_mut_ptr(),
                usize::MAX
            ),
            2
        );
        let invalid = [
            LiblrhsmmRsJumpF32 {
                delta: 1,
                probability: 1.0,
            },
            LiblrhsmmRsJumpF32 {
                delta: 1,
                probability: 1.0,
            },
        ];
        assert_eq!(
            liblrhsmm_rs_segmentation_set_outgoing_f32(owner, 1, invalid.as_ptr(), 2),
            3
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_outgoing_f32(owner, 1, null(), 0),
            3
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_outgoing_f32(owner, 1, replacement.as_ptr(), 1),
            3
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_outgoing_f32(owner, 3, replacement.as_ptr(), 2),
            2
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_outgoing_f32(owner, 1, null(), 1),
            1
        );
        let mut jumps = [LiblrhsmmRsJumpF32 {
            delta: 99,
            probability: 99.0,
        }; 2];
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_outgoing_f32(owner, 1, 1, jumps.as_mut_ptr(), 2),
            2
        );
        assert!(
            jumps
                .iter()
                .all(|value| value.delta == 99 && value.probability == 99.0)
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_outgoing_f32(owner, 1, 2, null_mut(), 0),
            0
        );
        let mut count = 99;
        assert_eq!(
            liblrhsmm_rs_segmentation_outgoing_count_f32(owner, 3, &mut count),
            2
        );
        assert_eq!(count, 99);
        assert_eq!(
            liblrhsmm_rs_segmentation_add_jump_f32(owner, 0, 0, f32::NAN),
            3
        );
        assert_eq!(liblrhsmm_rs_segmentation_add_jump_f32(owner, 0, 0, -1.0), 3);
        assert_eq!(liblrhsmm_rs_segmentation_add_jump_f32(owner, 3, 0, 1.0), 3);
        let mut sentinel = owner;
        let wrong = [LiblrhsmmRsStateOccurrence { state: 3, end: 1 }];
        assert_eq!(
            liblrhsmm_rs_segmentation_resegment_f32(owner, wrong.as_ptr(), 1, &mut sentinel),
            3
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_resegment_f32(owner, null(), 1, &mut sentinel),
            1
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_resegment_f32(
                owner,
                occurrences.as_ptr(),
                usize::MAX,
                &mut sentinel
            ),
            2
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_new_f32(i32::MAX as usize + 1, 0, &mut sentinel),
            2
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_new_f32(0, usize::MAX, &mut sentinel),
            2
        );
        assert_eq!(sentinel, owner);
        compare_f32(owner, &expected);
        let end = expected.boundaries.len();
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_boundaries_f32(owner, end, null_mut(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_duration_states_f32(owner, end, null(), 0),
            0
        );
        let bounds = [2, 4];
        let durations = [3, 8];
        let states = [9, 6];
        let rows = [states.as_ptr(), states.as_ptr()];
        let ordinary = [LiblrhsmmRsJumpF32 {
            delta: 1,
            probability: 1.0,
        }];
        let groups = [LiblrhsmmRsJumpSliceF32 {
            jumps: ordinary.as_ptr(),
            count: 1,
        }; 2];
        let bad_groups = [
            groups[0],
            LiblrhsmmRsJumpSliceF32 {
                jumps: invalid.as_ptr(),
                count: 2,
            },
        ];
        assert_eq!(
            liblrhsmm_rs_segmentation_replace_f32(
                owner,
                2,
                2,
                bounds.as_ptr(),
                durations.as_ptr(),
                rows.as_ptr(),
                bad_groups.as_ptr()
            ),
            3
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_replace_f32(
                owner,
                2,
                2,
                bounds.as_ptr(),
                durations.as_ptr(),
                [states.as_ptr(), null()].as_ptr(),
                groups.as_ptr()
            ),
            1
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_create_f32(
                2,
                2,
                bounds.as_ptr(),
                durations.as_ptr(),
                rows.as_ptr(),
                bad_groups.as_ptr(),
                &mut sentinel
            ),
            3
        );
        assert_eq!(sentinel, owner);
        compare_f32(owner, &expected);
        assert_eq!(
            liblrhsmm_rs_segmentation_replace_f32(
                owner,
                2,
                2,
                bounds.as_ptr(),
                durations.as_ptr(),
                rows.as_ptr(),
                groups.as_ptr()
            ),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_output_states_f32(owner, 0, 0, [71].as_ptr(), 1),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_output_states_f32(owner, 1, 0, integers.as_mut_ptr(), 2),
            0
        );
        assert_eq!(&integers[..2], states);
        assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut owner), 0);
        compare_f32(clone, &original);
        assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut clone), 0);
        let empty_rows = [null(), null()];
        assert_eq!(
            liblrhsmm_rs_segmentation_create_f32(
                2,
                0,
                null(),
                null(),
                empty_rows.as_ptr(),
                null(),
                &mut owner
            ),
            0
        );
        compare_f32(owner, &SegmentationData::new(2, 0));
        assert_eq!(
            liblrhsmm_rs_segmentation_replace_f32(owner, 0, 0, null(), null(), null(), null()),
            0
        );
        compare_f32(owner, &SegmentationData::new(0, 0));
        assert_eq!(
            liblrhsmm_rs_segmentation_resegment_f32(owner, null(), 0, &mut resegmented),
            0
        );
        compare_f32(resegmented, &SegmentationData::new(0, 0));
        assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut resegmented), 0);
        assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut owner), 0);
        assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut owner), 0);
        assert_eq!(liblrhsmm_rs_segmentation_release_f32(null_mut()), 1);
    }
}

#[cfg(feature = "serialization")]
#[test]
fn segmentation_f32_original_c_fields_copy_and_resegmentation_fixtures() {
    // SAFETY: Input arrays and all independently owned handles remain live.
    unsafe {
        for bytes in [
            include_bytes!("../../../tests/fixtures/c-segmentation.bin").as_slice(),
            include_bytes!("../../../tests/fixtures/c-copy-segmentation.bin").as_slice(),
        ] {
            let expected = SegmentationData::<f32>::read_from(bytes).unwrap();
            let mut owner = create_f32(&expected);
            let mut clone = null_mut();
            compare_f32(owner, &expected);
            assert_eq!(liblrhsmm_rs_segmentation_clone_f32(owner, &mut clone), 0);
            assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut owner), 0);
            compare_f32(clone, &expected);
            assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut clone), 0);
        }
        let mut source = SegmentationData::<f32>::new(2, 5);
        source.boundaries = vec![1, 3, 3, 6, 8];
        source.duration_states = vec![10, 11, 12, 13, 14];
        source.output_states = vec![vec![20, 21, 22, 23, 24], vec![40, 39, 38, 37, 36]];
        let mut owner = create_f32(&source);
        assert_eq!(liblrhsmm_rs_segmentation_add_jump_f32(owner, 0, 3, 0.25), 0);
        let path = [
            LiblrhsmmRsStateOccurrence { state: 0, end: 2 },
            LiblrhsmmRsStateOccurrence { state: 3, end: 4 },
            LiblrhsmmRsStateOccurrence { state: 3, end: 5 },
            LiblrhsmmRsStateOccurrence { state: 1, end: 8 },
        ];
        let mut resegmented = null_mut();
        assert_eq!(
            liblrhsmm_rs_segmentation_resegment_f32(
                owner,
                path.as_ptr(),
                path.len(),
                &mut resegmented
            ),
            0
        );
        let expected = SegmentationData::<f32>::read_from(
            include_bytes!("../../../tests/fixtures/c-resegmentation.bin").as_slice(),
        )
        .unwrap();
        compare_f32(resegmented, &expected);
        assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut owner), 0);
        compare_f32(resegmented, &expected);
        assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut resegmented), 0);
    }
}

unsafe fn compare_f64(owner: *const LiblrhsmmRsSegmentationF64, expected: &SegmentationData<f64>) {
    // SAFETY: Live readable owner and independent initialized caller buffers.
    unsafe {
        let mut shape = LiblrhsmmRsSegmentationLayout {
            streams: 99,
            segments: 99,
        };
        assert_eq!(liblrhsmm_rs_segmentation_layout_f64(owner, &mut shape), 0);
        assert_eq!(
            shape,
            LiblrhsmmRsSegmentationLayout {
                streams: expected.output_states.len(),
                segments: expected.boundaries.len()
            }
        );
        let mut integers = vec![99; expected.boundaries.len()];
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_boundaries_f64(
                owner,
                0,
                integers.as_mut_ptr(),
                integers.len()
            ),
            0
        );
        assert_eq!(integers, expected.boundaries);
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_duration_states_f64(
                owner,
                0,
                integers.as_mut_ptr(),
                integers.len()
            ),
            0
        );
        assert_eq!(integers, expected.duration_states);
        for (stream, states) in expected.output_states.iter().enumerate() {
            assert_eq!(
                liblrhsmm_rs_segmentation_copy_output_states_f64(
                    owner,
                    stream,
                    0,
                    integers.as_mut_ptr(),
                    integers.len()
                ),
                0
            );
            assert_eq!(&integers, states);
        }
        for (segment, group) in expected.outgoing.iter().enumerate() {
            let mut count = 99;
            assert_eq!(
                liblrhsmm_rs_segmentation_outgoing_count_f64(owner, segment, &mut count),
                0
            );
            assert_eq!(count, group.len());
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
                    segment,
                    0,
                    jumps.as_mut_ptr(),
                    count
                ),
                0
            );
            for (actual, expected) in jumps.iter().zip(group) {
                assert_eq!(actual.delta, expected.delta);
                assert_eq!(actual.probability.to_bits(), expected.probability.to_bits());
            }
        }
    }
}
unsafe fn create_f64(source: &SegmentationData<f64>) -> *mut LiblrhsmmRsSegmentationF64 {
    let rows: Vec<_> = source
        .output_states
        .iter()
        .map(|values| values.as_ptr())
        .collect();
    let jumps: Vec<Vec<_>> = source
        .outgoing
        .iter()
        .map(|group| {
            group
                .iter()
                .map(|jump| LiblrhsmmRsJumpF64 {
                    delta: jump.delta,
                    probability: jump.probability,
                })
                .collect()
        })
        .collect();
    let groups: Vec<_> = jumps
        .iter()
        .map(|values| LiblrhsmmRsJumpSliceF64 {
            jumps: values.as_ptr(),
            count: values.len(),
        })
        .collect();
    let mut owner = null_mut();
    // SAFETY: All borrowed inputs remain live; output starts unowned.
    unsafe {
        assert_eq!(
            liblrhsmm_rs_segmentation_create_f64(
                rows.len(),
                source.boundaries.len(),
                source.boundaries.as_ptr(),
                source.duration_states.as_ptr(),
                rows.as_ptr(),
                groups.as_ptr(),
                &mut owner
            ),
            0
        );
    }
    owner
}

#[test]
fn segmentation_f64_full_bits_fields_copies_mutations_and_atomic_errors() {
    // SAFETY: Independent initialized buffers, live owners, and unique releases.
    unsafe {
        let mut source = SegmentationData::<f64>::new(2, 3);
        source.boundaries = vec![i32::MIN, 5, i32::MAX];
        source.duration_states = vec![-1, i32::MAX, i32::MIN];
        source.output_states = vec![vec![i32::MIN, 4, i32::MAX]; 2];
        let probabilities = [
            0x7ff8000000001234u64,
            1,
            0x8000000000000000,
            0x7ff0000000000000,
            0xfff0000000000000,
            0x3ff4000000000001,
        ]
        .map(f64::from_bits);
        source.outgoing = vec![
            vec![
                JumpParameters {
                    delta: 2,
                    probability: probabilities[0],
                },
                JumpParameters {
                    delta: -2,
                    probability: probabilities[1],
                },
                JumpParameters {
                    delta: 1,
                    probability: probabilities[2],
                },
            ],
            vec![JumpParameters {
                delta: 1,
                probability: probabilities[3],
            }],
            vec![
                JumpParameters {
                    delta: 0,
                    probability: probabilities[4],
                },
                JumpParameters {
                    delta: 1,
                    probability: probabilities[5],
                },
            ],
        ];
        let original = source.clone();
        let mut expected = original.clone();
        let mut owner = create_f64(&source);
        let mut clone = null_mut();
        let mut zeroed = null_mut();
        let mut resegmented = null_mut();
        assert_eq!(liblrhsmm_rs_segmentation_new_f64(2, 3, &mut zeroed), 0);
        compare_f64(zeroed, &SegmentationData::new(2, 3));
        assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut zeroed), 0);
        assert_eq!(liblrhsmm_rs_segmentation_clone_f64(owner, &mut clone), 0);
        source.boundaries.fill(9);
        source.output_states[0].fill(8);
        source.outgoing[0][0].probability = 1.0;
        drop(source);
        compare_f64(owner, &expected);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_boundaries_f64(owner, 1, [4].as_ptr(), 1),
            0
        );
        expected.boundaries[1] = 4;
        assert_eq!(
            liblrhsmm_rs_segmentation_set_duration_states_f64(owner, 0, [8, i32::MIN].as_ptr(), 2),
            0
        );
        expected.duration_states[..2].copy_from_slice(&[8, i32::MIN]);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_output_states_f64(owner, 0, 1, [7, i32::MAX].as_ptr(), 2),
            0
        );
        expected.output_states[0][1..].copy_from_slice(&[7, i32::MAX]);
        let replacement = [
            LiblrhsmmRsJumpF64 {
                delta: 2,
                probability: probabilities[5],
            },
            LiblrhsmmRsJumpF64 {
                delta: 1,
                probability: 1.0,
            },
        ];
        assert_eq!(
            liblrhsmm_rs_segmentation_set_outgoing_f64(owner, 1, replacement.as_ptr(), 2),
            0
        );
        expected.outgoing[1] = replacement
            .iter()
            .map(|jump| JumpParameters {
                delta: jump.delta,
                probability: jump.probability,
            })
            .collect();
        assert_eq!(
            liblrhsmm_rs_segmentation_add_jump_f64(owner, 2, 1, 0.125),
            0
        );
        expected.add_jump(2, 1, 0.125).unwrap();
        assert_eq!(liblrhsmm_rs_segmentation_add_jump_f64(owner, 0, 0, 0.5), 0);
        expected.add_jump(0, 0, 0.5).unwrap();
        compare_f64(owner, &expected);
        let occurrences = [
            LiblrhsmmRsStateOccurrence { state: 2, end: 3 },
            LiblrhsmmRsStateOccurrence { state: 0, end: 4 },
            LiblrhsmmRsStateOccurrence { state: 2, end: 4 },
        ];
        let path: Vec<_> = occurrences
            .iter()
            .map(|value| StateOccurrence {
                state: value.state,
                end: value.end,
            })
            .collect();
        assert_eq!(
            liblrhsmm_rs_segmentation_resegment_f64(
                owner,
                occurrences.as_ptr(),
                3,
                &mut resegmented
            ),
            0
        );
        compare_f64(resegmented, &expected.resegment(&path).unwrap());
        assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut resegmented), 0);
        let mut integers = [99; 3];
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_boundaries_f64(
                owner,
                usize::MAX,
                integers.as_mut_ptr(),
                1
            ),
            2
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_duration_states_f64(owner, 2, integers.as_mut_ptr(), 2),
            2
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_output_states_f64(owner, 2, 0, integers.as_mut_ptr(), 3),
            2
        );
        assert_eq!(integers, [99; 3]);
        assert_eq!(
            liblrhsmm_rs_segmentation_set_boundaries_f64(owner, 3, [1].as_ptr(), 1),
            2
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_duration_states_f64(owner, 0, null(), 1),
            1
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_output_states_f64(owner, 0, usize::MAX, [1].as_ptr(), 1),
            2
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_boundaries_f64(
                owner,
                0,
                integers.as_mut_ptr().cast::<u8>().add(1).cast(),
                1
            ),
            1
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_duration_states_f64(
                owner,
                0,
                integers.as_mut_ptr(),
                usize::MAX
            ),
            2
        );
        let invalid = [
            LiblrhsmmRsJumpF64 {
                delta: 1,
                probability: 1.0,
            },
            LiblrhsmmRsJumpF64 {
                delta: 1,
                probability: 1.0,
            },
        ];
        assert_eq!(
            liblrhsmm_rs_segmentation_set_outgoing_f64(owner, 1, invalid.as_ptr(), 2),
            3
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_outgoing_f64(owner, 1, null(), 0),
            3
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_outgoing_f64(owner, 1, replacement.as_ptr(), 1),
            3
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_outgoing_f64(owner, 3, replacement.as_ptr(), 2),
            2
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_outgoing_f64(owner, 1, null(), 1),
            1
        );
        let mut jumps = [LiblrhsmmRsJumpF64 {
            delta: 99,
            probability: 99.0,
        }; 2];
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_outgoing_f64(owner, 1, 1, jumps.as_mut_ptr(), 2),
            2
        );
        assert!(
            jumps
                .iter()
                .all(|value| value.delta == 99 && value.probability == 99.0)
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_outgoing_f64(owner, 1, 2, null_mut(), 0),
            0
        );
        let mut count = 99;
        assert_eq!(
            liblrhsmm_rs_segmentation_outgoing_count_f64(owner, 3, &mut count),
            2
        );
        assert_eq!(count, 99);
        assert_eq!(
            liblrhsmm_rs_segmentation_add_jump_f64(owner, 0, 0, f64::NAN),
            3
        );
        assert_eq!(liblrhsmm_rs_segmentation_add_jump_f64(owner, 0, 0, -1.0), 3);
        assert_eq!(liblrhsmm_rs_segmentation_add_jump_f64(owner, 3, 0, 1.0), 3);
        let mut sentinel = owner;
        let wrong = [LiblrhsmmRsStateOccurrence { state: 3, end: 1 }];
        assert_eq!(
            liblrhsmm_rs_segmentation_resegment_f64(owner, wrong.as_ptr(), 1, &mut sentinel),
            3
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_resegment_f64(owner, null(), 1, &mut sentinel),
            1
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_resegment_f64(
                owner,
                occurrences.as_ptr(),
                usize::MAX,
                &mut sentinel
            ),
            2
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_new_f64(i32::MAX as usize + 1, 0, &mut sentinel),
            2
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_new_f64(0, usize::MAX, &mut sentinel),
            2
        );
        assert_eq!(sentinel, owner);
        compare_f64(owner, &expected);
        let end = expected.boundaries.len();
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_boundaries_f64(owner, end, null_mut(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_duration_states_f64(owner, end, null(), 0),
            0
        );
        let bounds = [2, 4];
        let durations = [3, 8];
        let states = [9, 6];
        let rows = [states.as_ptr(), states.as_ptr()];
        let ordinary = [LiblrhsmmRsJumpF64 {
            delta: 1,
            probability: 1.0,
        }];
        let groups = [LiblrhsmmRsJumpSliceF64 {
            jumps: ordinary.as_ptr(),
            count: 1,
        }; 2];
        let bad_groups = [
            groups[0],
            LiblrhsmmRsJumpSliceF64 {
                jumps: invalid.as_ptr(),
                count: 2,
            },
        ];
        assert_eq!(
            liblrhsmm_rs_segmentation_replace_f64(
                owner,
                2,
                2,
                bounds.as_ptr(),
                durations.as_ptr(),
                rows.as_ptr(),
                bad_groups.as_ptr()
            ),
            3
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_replace_f64(
                owner,
                2,
                2,
                bounds.as_ptr(),
                durations.as_ptr(),
                [states.as_ptr(), null()].as_ptr(),
                groups.as_ptr()
            ),
            1
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_create_f64(
                2,
                2,
                bounds.as_ptr(),
                durations.as_ptr(),
                rows.as_ptr(),
                bad_groups.as_ptr(),
                &mut sentinel
            ),
            3
        );
        assert_eq!(sentinel, owner);
        compare_f64(owner, &expected);
        assert_eq!(
            liblrhsmm_rs_segmentation_replace_f64(
                owner,
                2,
                2,
                bounds.as_ptr(),
                durations.as_ptr(),
                rows.as_ptr(),
                groups.as_ptr()
            ),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_output_states_f64(owner, 0, 0, [71].as_ptr(), 1),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_output_states_f64(owner, 1, 0, integers.as_mut_ptr(), 2),
            0
        );
        assert_eq!(&integers[..2], states);
        assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut owner), 0);
        compare_f64(clone, &original);
        assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut clone), 0);
        let empty_rows = [null(), null()];
        assert_eq!(
            liblrhsmm_rs_segmentation_create_f64(
                2,
                0,
                null(),
                null(),
                empty_rows.as_ptr(),
                null(),
                &mut owner
            ),
            0
        );
        compare_f64(owner, &SegmentationData::new(2, 0));
        assert_eq!(
            liblrhsmm_rs_segmentation_replace_f64(owner, 0, 0, null(), null(), null(), null()),
            0
        );
        compare_f64(owner, &SegmentationData::new(0, 0));
        assert_eq!(
            liblrhsmm_rs_segmentation_resegment_f64(owner, null(), 0, &mut resegmented),
            0
        );
        compare_f64(resegmented, &SegmentationData::new(0, 0));
        assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut resegmented), 0);
        assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut owner), 0);
        assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut owner), 0);
        assert_eq!(liblrhsmm_rs_segmentation_release_f64(null_mut()), 1);
    }
}

#[cfg(feature = "serialization")]
#[test]
fn segmentation_f64_original_c_fields_copy_and_resegmentation_fixtures() {
    // SAFETY: Input arrays and all independently owned handles remain live.
    unsafe {
        for bytes in [
            include_bytes!("../../../tests/fixtures/c-segmentation.bin").as_slice(),
            include_bytes!("../../../tests/fixtures/c-copy-segmentation.bin").as_slice(),
        ] {
            let expected = SegmentationData::<f64>::read_from(bytes).unwrap();
            let mut owner = create_f64(&expected);
            let mut clone = null_mut();
            compare_f64(owner, &expected);
            assert_eq!(liblrhsmm_rs_segmentation_clone_f64(owner, &mut clone), 0);
            assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut owner), 0);
            compare_f64(clone, &expected);
            assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut clone), 0);
        }
        let mut source = SegmentationData::<f64>::new(2, 5);
        source.boundaries = vec![1, 3, 3, 6, 8];
        source.duration_states = vec![10, 11, 12, 13, 14];
        source.output_states = vec![vec![20, 21, 22, 23, 24], vec![40, 39, 38, 37, 36]];
        let mut owner = create_f64(&source);
        assert_eq!(liblrhsmm_rs_segmentation_add_jump_f64(owner, 0, 3, 0.25), 0);
        let path = [
            LiblrhsmmRsStateOccurrence { state: 0, end: 2 },
            LiblrhsmmRsStateOccurrence { state: 3, end: 4 },
            LiblrhsmmRsStateOccurrence { state: 3, end: 5 },
            LiblrhsmmRsStateOccurrence { state: 1, end: 8 },
        ];
        let mut resegmented = null_mut();
        assert_eq!(
            liblrhsmm_rs_segmentation_resegment_f64(
                owner,
                path.as_ptr(),
                path.len(),
                &mut resegmented
            ),
            0
        );
        let expected = SegmentationData::<f64>::read_from(
            include_bytes!("../../../tests/fixtures/c-resegmentation.bin").as_slice(),
        )
        .unwrap();
        compare_f64(resegmented, &expected);
        assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut owner), 0);
        compare_f64(resegmented, &expected);
        assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut resegmented), 0);
    }
}
