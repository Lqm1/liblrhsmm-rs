use liblrhsmm_rs::data::SegmentationData;
use liblrhsmm_rs::scalar::Scalar;
use liblrhsmm_rs_capi::*;
use std::ptr::{null, null_mut};

unsafe fn source_f32(expected: &SegmentationData<f32>) -> *mut LiblrhsmmRsSegmentationF32 {
    let rows: Vec<_> = expected
        .output_states
        .iter()
        .map(|row| row.as_ptr())
        .collect();
    let jumps: Vec<Vec<_>> = expected
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
        .map(|group| LiblrhsmmRsJumpSliceF32 {
            jumps: group.as_ptr(),
            count: group.len(),
        })
        .collect();
    let mut source = null_mut();
    assert_eq!(
        unsafe {
            liblrhsmm_rs_segmentation_create_f32(
                rows.len(),
                expected.boundaries.len(),
                expected.boundaries.as_ptr(),
                expected.duration_states.as_ptr(),
                rows.as_ptr(),
                groups.as_ptr(),
                &mut source,
            )
        },
        0
    );
    source
}
unsafe fn compare_f32(
    owner: *const LiblrhsmmRsTransitionGraphF32,
    expected: &SegmentationData<f32>,
) {
    // SAFETY: Live immutable owner and independent initialized buffers.
    unsafe {
        let native = expected.build_transitions().unwrap();
        let mut layout = LiblrhsmmRsSegmentationLayout {
            streams: 99,
            segments: 99,
        };
        assert_eq!(
            liblrhsmm_rs_transition_graph_layout_f32(owner, &mut layout),
            0
        );
        assert_eq!(
            (layout.streams, layout.segments),
            (expected.output_states.len(), expected.boundaries.len())
        );
        for (incoming, groups) in [(true, native.incoming()), (false, native.outgoing())] {
            for (segment, expected_group) in groups.iter().enumerate() {
                let mut count = 99;
                let count_fn = if incoming {
                    liblrhsmm_rs_transition_graph_incoming_count_f32
                } else {
                    liblrhsmm_rs_transition_graph_outgoing_count_f32
                };
                let copy_fn = if incoming {
                    liblrhsmm_rs_transition_graph_copy_incoming_f32
                } else {
                    liblrhsmm_rs_transition_graph_copy_outgoing_f32
                };
                assert_eq!(count_fn(owner, segment, &mut count), 0);
                assert_eq!(count, expected_group.len());
                let mut actual = vec![
                    LiblrhsmmRsJumpF32 {
                        delta: 99,
                        probability: 99.0
                    };
                    count
                ];
                assert_eq!(copy_fn(owner, segment, 0, actual.as_mut_ptr(), count), 0);
                for (actual, expected) in actual.iter().zip(expected_group) {
                    assert_eq!(actual.delta, expected.delta);
                    assert_eq!(actual.probability.to_bits(), expected.probability.to_bits());
                }
                assert_eq!(copy_fn(owner, segment, count, null_mut(), 0), 0);
                for (index, expected) in expected_group.iter().enumerate() {
                    let mut single = LiblrhsmmRsJumpF32 {
                        delta: 99,
                        probability: 99.0,
                    };
                    assert_eq!(copy_fn(owner, segment, index, &mut single, 1), 0);
                    assert_eq!(
                        (single.delta, single.probability.to_bits()),
                        (expected.delta, expected.probability.to_bits())
                    );
                }
            }
        }
        let mut child = null_mut();
        assert_eq!(
            liblrhsmm_rs_transition_graph_get_segmentation_f32(owner, &mut child),
            0
        );
        let mut integers = vec![99; expected.boundaries.len()];
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_boundaries_f32(
                child,
                0,
                integers.as_mut_ptr(),
                integers.len()
            ),
            0
        );
        assert_eq!(integers, expected.boundaries);
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_duration_states_f32(
                child,
                0,
                integers.as_mut_ptr(),
                integers.len()
            ),
            0
        );
        assert_eq!(integers, expected.duration_states);
        for (stream, expected) in expected.output_states.iter().enumerate() {
            assert_eq!(
                liblrhsmm_rs_segmentation_copy_output_states_f32(
                    child,
                    stream,
                    0,
                    integers.as_mut_ptr(),
                    integers.len()
                ),
                0
            );
            assert_eq!(&integers, expected);
        }
        for (segment, expected) in expected.outgoing.iter().enumerate() {
            let mut count = 99;
            assert_eq!(
                liblrhsmm_rs_segmentation_outgoing_count_f32(child, segment, &mut count),
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
                    child,
                    segment,
                    0,
                    jumps.as_mut_ptr(),
                    count
                ),
                0
            );
            for (actual, expected) in jumps.iter().zip(expected) {
                assert_eq!(
                    (actual.delta, actual.probability.to_bits()),
                    (expected.delta, expected.probability.to_bits())
                );
            }
        }
        assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut child), 0);
    }
}

#[test]
fn transition_graph_f32_complete_c_cache_fields_clones_and_errors() {
    // SAFETY: Correct unique handles, independent initialized buffers and explicit lifetime order.
    unsafe {
        let mut expected = SegmentationData::<f32>::new(2, 4);
        expected.boundaries = vec![i32::MIN, 3, 2, i32::MAX];
        expected.duration_states = vec![i32::MAX, -1, 7, i32::MIN];
        expected.output_states = vec![vec![2, -3, 5, i32::MIN], vec![i32::MAX, 8, 1, 9]];
        for (source, delta, weight) in [
            (0, 2, 0.2_f32),
            (1, 0, 0.3),
            (2, -2, 0.4),
            (3, -2, 0.5),
            (0, -100, 0.1),
            (3, 100, 0.1),
        ] {
            expected
                .add_jump(source, delta, f32::from_f32(weight))
                .unwrap();
        }
        for state in 0..4 {
            expected
                .add_jump(state, 1, f32::from_f32(0.6_f32 + state as f32 * 0.1))
                .unwrap();
        }
        let mut source = source_f32(&expected);
        let mut graph = null_mut();
        assert_eq!(
            liblrhsmm_rs_segmentation_build_transitions_f32(source, &mut graph),
            0
        );
        compare_f32(graph, &expected);
        let mut clone = null_mut();
        assert_eq!(
            liblrhsmm_rs_transition_graph_clone_f32(graph, &mut clone),
            0
        );
        // All16 records are unchanged binary32 C output, exactly widened in the f64 case.
        let fixture = include_str!("../../../tests/fixtures/c-transitions.txt");
        assert_eq!(fixture.lines().count(), 16);
        for line in fixture.lines() {
            let fields: Vec<_> = line.split_whitespace().collect();
            let segment: usize = fields[1].parse().unwrap();
            let index: usize = fields[2].parse().unwrap();
            let mut jump = LiblrhsmmRsJumpF32 {
                delta: 99,
                probability: 99.0,
            };
            assert_eq!(
                liblrhsmm_rs_transition_graph_copy_incoming_f32(
                    graph, segment, index, &mut jump, 1
                ),
                0
            );
            assert_eq!(jump.delta, fields[3].parse::<i32>().unwrap());
            assert_eq!(
                jump.probability.to_bits(),
                f32::from_f32(fields[4].parse::<f32>().unwrap()).to_bits()
            );
        }
        assert_eq!(
            liblrhsmm_rs_segmentation_add_jump_f32(source, 0, 0, 0.25),
            0
        );
        let changed = [9_i32; 4];
        assert_eq!(
            liblrhsmm_rs_segmentation_set_boundaries_f32(source, 0, changed.as_ptr(), 4),
            0
        );
        assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut source), 0);
        compare_f32(graph, &expected);
        compare_f32(clone, &expected);
        let mut child = null_mut();
        assert_eq!(
            liblrhsmm_rs_transition_graph_get_segmentation_f32(graph, &mut child),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_output_states_f32(child, 1, 0, changed.as_ptr(), 4),
            0
        );
        assert_eq!(liblrhsmm_rs_segmentation_add_jump_f32(child, 2, 0, 0.75), 0);
        compare_f32(graph, &expected);
        let mut count = 77;
        assert_eq!(
            liblrhsmm_rs_transition_graph_incoming_count_f32(graph, 4, &mut count),
            2
        );
        assert_eq!(count, 77);
        assert_eq!(
            liblrhsmm_rs_transition_graph_outgoing_count_f32(graph, 4, &mut count),
            2
        );
        assert_eq!(count, 77);
        assert_eq!(
            liblrhsmm_rs_transition_graph_incoming_count_f32(graph, 0, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_transition_graph_outgoing_count_f32(graph, 0, null_mut()),
            1
        );
        let mut jumps = [LiblrhsmmRsJumpF32 {
            delta: 77,
            probability: -0.0,
        }; 3];
        for copy_fn in [
            liblrhsmm_rs_transition_graph_copy_incoming_f32,
            liblrhsmm_rs_transition_graph_copy_outgoing_f32,
        ] {
            for (segment, offset, count) in
                [(4, 0, 1), (0, usize::MAX, 1), (0, 0, usize::MAX), (0, 2, 3)]
            {
                assert_eq!(
                    copy_fn(graph, segment, offset, jumps.as_mut_ptr(), count),
                    2
                );
                assert!(
                    jumps.iter().all(|jump| jump.delta == 77
                        && jump.probability.to_bits() == (-0.0_f32).to_bits())
                );
            }
            assert_eq!(copy_fn(graph, 0, 0, null_mut(), 1), 1);
            let misaligned = jumps.as_mut_ptr().cast::<u8>().add(1).cast();
            assert_eq!(copy_fn(graph, 0, 0, misaligned, 1), 1);
        }
        let mut layout = LiblrhsmmRsSegmentationLayout {
            streams: 77,
            segments: 88,
        };
        assert_eq!(
            liblrhsmm_rs_transition_graph_layout_f32(null(), &mut layout),
            1
        );
        assert_eq!((layout.streams, layout.segments), (77, 88));
        assert_eq!(
            liblrhsmm_rs_transition_graph_layout_f32(graph, null_mut()),
            1
        );
        let mut sentinel = graph;
        assert_eq!(
            liblrhsmm_rs_transition_graph_clone_f32(null(), &mut sentinel),
            1
        );
        assert_eq!(sentinel, graph);
        assert_eq!(
            liblrhsmm_rs_transition_graph_clone_f32(graph, null_mut()),
            1
        );
        let mut child_sentinel = child;
        assert_eq!(
            liblrhsmm_rs_transition_graph_get_segmentation_f32(null(), &mut child_sentinel),
            1
        );
        assert_eq!(child_sentinel, child);
        assert_eq!(
            liblrhsmm_rs_transition_graph_get_segmentation_f32(graph, null_mut()),
            1
        );
        for probability in [f32::NAN, f32::INFINITY, -0.5] {
            let mut invalid = expected.clone();
            invalid.outgoing[3][0].probability = probability;
            let mut invalid_source = source_f32(&invalid);
            assert_eq!(
                liblrhsmm_rs_segmentation_build_transitions_f32(invalid_source, &mut sentinel),
                3
            );
            assert_eq!(sentinel, graph);
            assert_eq!(
                liblrhsmm_rs_segmentation_release_f32(&mut invalid_source),
                0
            );
            compare_f32(graph, &expected);
        }
        assert_eq!(
            liblrhsmm_rs_segmentation_build_transitions_f32(null(), &mut sentinel),
            1
        );
        assert_eq!(sentinel, graph);
        assert_eq!(
            liblrhsmm_rs_segmentation_build_transitions_f32(child, null_mut()),
            1
        );
        assert_eq!(liblrhsmm_rs_transition_graph_release_f32(&mut graph), 0);
        assert!(graph.is_null());
        compare_f32(clone, &expected);
        let mut integers = [0; 4];
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_output_states_f32(child, 1, 0, integers.as_mut_ptr(), 4),
            0
        );
        assert_eq!(integers, changed);
        assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut child), 0);
        assert_eq!(liblrhsmm_rs_transition_graph_release_f32(&mut clone), 0);
        assert_eq!(liblrhsmm_rs_transition_graph_release_f32(&mut clone), 0);
        assert_eq!(liblrhsmm_rs_transition_graph_release_f32(null_mut()), 1);
        // Precision-specific bit patterns are preserved in outgoing and derived incoming groups.
        let mut exact = SegmentationData::<f32>::new(0, 3);
        exact.add_jump(0, 2, f32::from_bits(1)).unwrap();
        exact.add_jump(1, 0, -0.0).unwrap();
        exact.add_jump(0, i32::MIN, 0.25).unwrap();
        exact.add_jump(2, i32::MAX, 0.5).unwrap();
        let mut exact_source = source_f32(&exact);
        assert_eq!(
            liblrhsmm_rs_segmentation_build_transitions_f32(exact_source, &mut graph),
            0
        );
        compare_f32(graph, &exact);
        assert_eq!(
            liblrhsmm_rs_transition_graph_clone_f32(graph, &mut clone),
            0
        );
        assert_eq!(liblrhsmm_rs_transition_graph_release_f32(&mut graph), 0);
        assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut exact_source), 0);
        compare_f32(clone, &exact);
        assert_eq!(liblrhsmm_rs_transition_graph_release_f32(&mut clone), 0);
        for streams in [0, 2] {
            let empty = SegmentationData::<f32>::new(streams, 0);
            let mut empty_source = source_f32(&empty);
            assert_eq!(
                liblrhsmm_rs_segmentation_build_transitions_f32(empty_source, &mut graph),
                0
            );
            assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut empty_source), 0);
            compare_f32(graph, &empty);
            assert_eq!(
                liblrhsmm_rs_transition_graph_incoming_count_f32(graph, 0, &mut count),
                2
            );
            assert_eq!(
                liblrhsmm_rs_transition_graph_outgoing_count_f32(graph, 0, &mut count),
                2
            );
            assert_eq!(liblrhsmm_rs_transition_graph_release_f32(&mut graph), 0);
        }
    }
}

unsafe fn source_f64(expected: &SegmentationData<f64>) -> *mut LiblrhsmmRsSegmentationF64 {
    let rows: Vec<_> = expected
        .output_states
        .iter()
        .map(|row| row.as_ptr())
        .collect();
    let jumps: Vec<Vec<_>> = expected
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
        .map(|group| LiblrhsmmRsJumpSliceF64 {
            jumps: group.as_ptr(),
            count: group.len(),
        })
        .collect();
    let mut source = null_mut();
    assert_eq!(
        unsafe {
            liblrhsmm_rs_segmentation_create_f64(
                rows.len(),
                expected.boundaries.len(),
                expected.boundaries.as_ptr(),
                expected.duration_states.as_ptr(),
                rows.as_ptr(),
                groups.as_ptr(),
                &mut source,
            )
        },
        0
    );
    source
}
unsafe fn compare_f64(
    owner: *const LiblrhsmmRsTransitionGraphF64,
    expected: &SegmentationData<f64>,
) {
    // SAFETY: Live immutable owner and independent initialized buffers.
    unsafe {
        let native = expected.build_transitions().unwrap();
        let mut layout = LiblrhsmmRsSegmentationLayout {
            streams: 99,
            segments: 99,
        };
        assert_eq!(
            liblrhsmm_rs_transition_graph_layout_f64(owner, &mut layout),
            0
        );
        assert_eq!(
            (layout.streams, layout.segments),
            (expected.output_states.len(), expected.boundaries.len())
        );
        for (incoming, groups) in [(true, native.incoming()), (false, native.outgoing())] {
            for (segment, expected_group) in groups.iter().enumerate() {
                let mut count = 99;
                let count_fn = if incoming {
                    liblrhsmm_rs_transition_graph_incoming_count_f64
                } else {
                    liblrhsmm_rs_transition_graph_outgoing_count_f64
                };
                let copy_fn = if incoming {
                    liblrhsmm_rs_transition_graph_copy_incoming_f64
                } else {
                    liblrhsmm_rs_transition_graph_copy_outgoing_f64
                };
                assert_eq!(count_fn(owner, segment, &mut count), 0);
                assert_eq!(count, expected_group.len());
                let mut actual = vec![
                    LiblrhsmmRsJumpF64 {
                        delta: 99,
                        probability: 99.0
                    };
                    count
                ];
                assert_eq!(copy_fn(owner, segment, 0, actual.as_mut_ptr(), count), 0);
                for (actual, expected) in actual.iter().zip(expected_group) {
                    assert_eq!(actual.delta, expected.delta);
                    assert_eq!(actual.probability.to_bits(), expected.probability.to_bits());
                }
                assert_eq!(copy_fn(owner, segment, count, null_mut(), 0), 0);
                for (index, expected) in expected_group.iter().enumerate() {
                    let mut single = LiblrhsmmRsJumpF64 {
                        delta: 99,
                        probability: 99.0,
                    };
                    assert_eq!(copy_fn(owner, segment, index, &mut single, 1), 0);
                    assert_eq!(
                        (single.delta, single.probability.to_bits()),
                        (expected.delta, expected.probability.to_bits())
                    );
                }
            }
        }
        let mut child = null_mut();
        assert_eq!(
            liblrhsmm_rs_transition_graph_get_segmentation_f64(owner, &mut child),
            0
        );
        let mut integers = vec![99; expected.boundaries.len()];
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_boundaries_f64(
                child,
                0,
                integers.as_mut_ptr(),
                integers.len()
            ),
            0
        );
        assert_eq!(integers, expected.boundaries);
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_duration_states_f64(
                child,
                0,
                integers.as_mut_ptr(),
                integers.len()
            ),
            0
        );
        assert_eq!(integers, expected.duration_states);
        for (stream, expected) in expected.output_states.iter().enumerate() {
            assert_eq!(
                liblrhsmm_rs_segmentation_copy_output_states_f64(
                    child,
                    stream,
                    0,
                    integers.as_mut_ptr(),
                    integers.len()
                ),
                0
            );
            assert_eq!(&integers, expected);
        }
        for (segment, expected) in expected.outgoing.iter().enumerate() {
            let mut count = 99;
            assert_eq!(
                liblrhsmm_rs_segmentation_outgoing_count_f64(child, segment, &mut count),
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
                    child,
                    segment,
                    0,
                    jumps.as_mut_ptr(),
                    count
                ),
                0
            );
            for (actual, expected) in jumps.iter().zip(expected) {
                assert_eq!(
                    (actual.delta, actual.probability.to_bits()),
                    (expected.delta, expected.probability.to_bits())
                );
            }
        }
        assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut child), 0);
    }
}

#[test]
fn transition_graph_f64_complete_c_cache_fields_clones_and_errors() {
    // SAFETY: Correct unique handles, independent initialized buffers and explicit lifetime order.
    unsafe {
        let mut expected = SegmentationData::<f64>::new(2, 4);
        expected.boundaries = vec![i32::MIN, 3, 2, i32::MAX];
        expected.duration_states = vec![i32::MAX, -1, 7, i32::MIN];
        expected.output_states = vec![vec![2, -3, 5, i32::MIN], vec![i32::MAX, 8, 1, 9]];
        for (source, delta, weight) in [
            (0, 2, 0.2_f32),
            (1, 0, 0.3),
            (2, -2, 0.4),
            (3, -2, 0.5),
            (0, -100, 0.1),
            (3, 100, 0.1),
        ] {
            expected
                .add_jump(source, delta, f64::from_f32(weight))
                .unwrap();
        }
        for state in 0..4 {
            expected
                .add_jump(state, 1, f64::from_f32(0.6_f32 + state as f32 * 0.1))
                .unwrap();
        }
        let mut source = source_f64(&expected);
        let mut graph = null_mut();
        assert_eq!(
            liblrhsmm_rs_segmentation_build_transitions_f64(source, &mut graph),
            0
        );
        compare_f64(graph, &expected);
        let mut clone = null_mut();
        assert_eq!(
            liblrhsmm_rs_transition_graph_clone_f64(graph, &mut clone),
            0
        );
        // All16 records are unchanged binary32 C output, exactly widened in the f64 case.
        let fixture = include_str!("../../../tests/fixtures/c-transitions.txt");
        assert_eq!(fixture.lines().count(), 16);
        for line in fixture.lines() {
            let fields: Vec<_> = line.split_whitespace().collect();
            let segment: usize = fields[1].parse().unwrap();
            let index: usize = fields[2].parse().unwrap();
            let mut jump = LiblrhsmmRsJumpF64 {
                delta: 99,
                probability: 99.0,
            };
            assert_eq!(
                liblrhsmm_rs_transition_graph_copy_incoming_f64(
                    graph, segment, index, &mut jump, 1
                ),
                0
            );
            assert_eq!(jump.delta, fields[3].parse::<i32>().unwrap());
            assert_eq!(
                jump.probability.to_bits(),
                f64::from_f32(fields[4].parse::<f32>().unwrap()).to_bits()
            );
        }
        assert_eq!(
            liblrhsmm_rs_segmentation_add_jump_f64(source, 0, 0, 0.25),
            0
        );
        let changed = [9_i32; 4];
        assert_eq!(
            liblrhsmm_rs_segmentation_set_boundaries_f64(source, 0, changed.as_ptr(), 4),
            0
        );
        assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut source), 0);
        compare_f64(graph, &expected);
        compare_f64(clone, &expected);
        let mut child = null_mut();
        assert_eq!(
            liblrhsmm_rs_transition_graph_get_segmentation_f64(graph, &mut child),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_output_states_f64(child, 1, 0, changed.as_ptr(), 4),
            0
        );
        assert_eq!(liblrhsmm_rs_segmentation_add_jump_f64(child, 2, 0, 0.75), 0);
        compare_f64(graph, &expected);
        let mut count = 77;
        assert_eq!(
            liblrhsmm_rs_transition_graph_incoming_count_f64(graph, 4, &mut count),
            2
        );
        assert_eq!(count, 77);
        assert_eq!(
            liblrhsmm_rs_transition_graph_outgoing_count_f64(graph, 4, &mut count),
            2
        );
        assert_eq!(count, 77);
        assert_eq!(
            liblrhsmm_rs_transition_graph_incoming_count_f64(graph, 0, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_transition_graph_outgoing_count_f64(graph, 0, null_mut()),
            1
        );
        let mut jumps = [LiblrhsmmRsJumpF64 {
            delta: 77,
            probability: -0.0,
        }; 3];
        for copy_fn in [
            liblrhsmm_rs_transition_graph_copy_incoming_f64,
            liblrhsmm_rs_transition_graph_copy_outgoing_f64,
        ] {
            for (segment, offset, count) in
                [(4, 0, 1), (0, usize::MAX, 1), (0, 0, usize::MAX), (0, 2, 3)]
            {
                assert_eq!(
                    copy_fn(graph, segment, offset, jumps.as_mut_ptr(), count),
                    2
                );
                assert!(
                    jumps.iter().all(|jump| jump.delta == 77
                        && jump.probability.to_bits() == (-0.0_f64).to_bits())
                );
            }
            assert_eq!(copy_fn(graph, 0, 0, null_mut(), 1), 1);
            let misaligned = jumps.as_mut_ptr().cast::<u8>().add(1).cast();
            assert_eq!(copy_fn(graph, 0, 0, misaligned, 1), 1);
        }
        let mut layout = LiblrhsmmRsSegmentationLayout {
            streams: 77,
            segments: 88,
        };
        assert_eq!(
            liblrhsmm_rs_transition_graph_layout_f64(null(), &mut layout),
            1
        );
        assert_eq!((layout.streams, layout.segments), (77, 88));
        assert_eq!(
            liblrhsmm_rs_transition_graph_layout_f64(graph, null_mut()),
            1
        );
        let mut sentinel = graph;
        assert_eq!(
            liblrhsmm_rs_transition_graph_clone_f64(null(), &mut sentinel),
            1
        );
        assert_eq!(sentinel, graph);
        assert_eq!(
            liblrhsmm_rs_transition_graph_clone_f64(graph, null_mut()),
            1
        );
        let mut child_sentinel = child;
        assert_eq!(
            liblrhsmm_rs_transition_graph_get_segmentation_f64(null(), &mut child_sentinel),
            1
        );
        assert_eq!(child_sentinel, child);
        assert_eq!(
            liblrhsmm_rs_transition_graph_get_segmentation_f64(graph, null_mut()),
            1
        );
        for probability in [f64::NAN, f64::INFINITY, -0.5] {
            let mut invalid = expected.clone();
            invalid.outgoing[3][0].probability = probability;
            let mut invalid_source = source_f64(&invalid);
            assert_eq!(
                liblrhsmm_rs_segmentation_build_transitions_f64(invalid_source, &mut sentinel),
                3
            );
            assert_eq!(sentinel, graph);
            assert_eq!(
                liblrhsmm_rs_segmentation_release_f64(&mut invalid_source),
                0
            );
            compare_f64(graph, &expected);
        }
        assert_eq!(
            liblrhsmm_rs_segmentation_build_transitions_f64(null(), &mut sentinel),
            1
        );
        assert_eq!(sentinel, graph);
        assert_eq!(
            liblrhsmm_rs_segmentation_build_transitions_f64(child, null_mut()),
            1
        );
        assert_eq!(liblrhsmm_rs_transition_graph_release_f64(&mut graph), 0);
        assert!(graph.is_null());
        compare_f64(clone, &expected);
        let mut integers = [0; 4];
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_output_states_f64(child, 1, 0, integers.as_mut_ptr(), 4),
            0
        );
        assert_eq!(integers, changed);
        assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut child), 0);
        assert_eq!(liblrhsmm_rs_transition_graph_release_f64(&mut clone), 0);
        assert_eq!(liblrhsmm_rs_transition_graph_release_f64(&mut clone), 0);
        assert_eq!(liblrhsmm_rs_transition_graph_release_f64(null_mut()), 1);
        // Precision-specific bit patterns are preserved in outgoing and derived incoming groups.
        let mut exact = SegmentationData::<f64>::new(0, 3);
        exact
            .add_jump(0, 2, f64::from_bits(0x3ff0000000000001))
            .unwrap();
        exact.add_jump(1, 0, -0.0).unwrap();
        exact.add_jump(0, i32::MIN, 0.25).unwrap();
        exact.add_jump(2, i32::MAX, 0.5).unwrap();
        let mut exact_source = source_f64(&exact);
        assert_eq!(
            liblrhsmm_rs_segmentation_build_transitions_f64(exact_source, &mut graph),
            0
        );
        compare_f64(graph, &exact);
        assert_eq!(
            liblrhsmm_rs_transition_graph_clone_f64(graph, &mut clone),
            0
        );
        assert_eq!(liblrhsmm_rs_transition_graph_release_f64(&mut graph), 0);
        assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut exact_source), 0);
        compare_f64(clone, &exact);
        assert_eq!(liblrhsmm_rs_transition_graph_release_f64(&mut clone), 0);
        for streams in [0, 2] {
            let empty = SegmentationData::<f64>::new(streams, 0);
            let mut empty_source = source_f64(&empty);
            assert_eq!(
                liblrhsmm_rs_segmentation_build_transitions_f64(empty_source, &mut graph),
                0
            );
            assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut empty_source), 0);
            compare_f64(graph, &empty);
            assert_eq!(
                liblrhsmm_rs_transition_graph_incoming_count_f64(graph, 0, &mut count),
                2
            );
            assert_eq!(
                liblrhsmm_rs_transition_graph_outgoing_count_f64(graph, 0, &mut count),
                2
            );
            assert_eq!(liblrhsmm_rs_transition_graph_release_f64(&mut graph), 0);
        }
    }
}
