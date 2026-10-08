use liblrhsmm_rs::data::{JumpParameters, SegmentationData};
use liblrhsmm_rs::scalar::Scalar;
use liblrhsmm_rs::transitions::{OwnedTransitionGraphParameters, TransitionGraphView};

fn copied_views<T>(view: TransitionGraphView<'_, T>) -> [TransitionGraphView<'_, T>; 2] {
    // This generic body must compile without imposing Copy or Clone on T.
    [view, view]
}
fn jump_bits<T: Scalar>(jumps: &[Vec<JumpParameters<T>>]) -> Vec<Vec<(i32, u64)>> {
    jumps
        .iter()
        .map(|group| {
            group
                .iter()
                .map(|jump| (jump.delta, Into::<f64>::into(jump.probability).to_bits()))
                .collect()
        })
        .collect()
}
fn source<T: Scalar>() -> SegmentationData<T> {
    let mut segmentation = SegmentationData::new(2, 4);
    segmentation.boundaries = vec![i32::MIN, 3, 2, i32::MAX];
    segmentation.duration_states = vec![i32::MAX, -1, 7, i32::MIN];
    segmentation.output_states = vec![vec![2, -3, 5, i32::MIN], vec![i32::MAX, 8, 1, 9]];
    for (source, delta, probability) in [
        (0, 2, 0.2_f32),
        (1, 0, 0.3),
        (2, -2, 0.4),
        (3, -2, 0.5),
        (0, -100, 0.1),
        (3, 100, 0.1),
    ] {
        segmentation
            .add_jump(source, delta, T::from_f32(probability))
            .unwrap();
    }
    for state in 0..4 {
        segmentation
            .add_jump(state, 1, T::from_f32(0.6_f32 + state as f32 * 0.1))
            .unwrap();
    }
    segmentation
}
fn compare<T: Scalar>(
    owner: &OwnedTransitionGraphParameters<T>,
    expected: &SegmentationData<T>,
    incoming: &[Vec<JumpParameters<T>>],
) {
    assert_eq!(owner.segmentation(), expected);
    assert_eq!(jump_bits(owner.outgoing()), jump_bits(&expected.outgoing));
    assert_eq!(jump_bits(owner.incoming()), jump_bits(incoming));
    for view in copied_views(owner.as_view()) {
        assert!(std::ptr::eq(view.segmentation(), owner.segmentation()));
        assert_eq!(view.incoming().as_ptr(), owner.incoming().as_ptr());
        assert_eq!(view.outgoing().as_ptr(), owner.outgoing().as_ptr());
        for (cached, borrowed) in owner.incoming().iter().zip(view.incoming()) {
            assert_eq!(cached.as_ptr(), borrowed.as_ptr());
        }
    }
}
fn verify<T: Scalar>() {
    let mut source = source::<T>();
    let expected = source.clone();
    let borrowed = source.build_transitions().unwrap();
    let incoming = borrowed.incoming().to_vec();
    let cache_address = borrowed.incoming().as_ptr();
    let first_group = borrowed.incoming()[0].as_ptr();
    for view in copied_views(borrowed.as_view()) {
        assert_eq!(view.incoming().as_ptr(), cache_address);
        assert!(std::ptr::eq(view.segmentation(), &source));
    }
    let fixture = include_str!("fixtures/c-transitions.txt");
    assert_eq!(fixture.lines().count(), 16);
    for line in fixture.lines() {
        let fields: Vec<_> = line.split_whitespace().collect();
        let state = fields[1].parse::<usize>().unwrap();
        let index = fields[2].parse::<usize>().unwrap();
        assert_eq!(
            incoming[state][index].delta,
            fields[3].parse::<i32>().unwrap()
        );
        // The unchanged binary32 C corpus is exactly widened for the f64 test.
        let probability = T::from_f32(fields[4].parse::<f32>().unwrap());
        assert_eq!(
            Into::<f64>::into(incoming[state][index].probability).to_bits(),
            Into::<f64>::into(probability).to_bits()
        );
    }
    let snapshot = borrowed.to_owned();
    let view_snapshot = borrowed.as_view().to_owned();
    assert_ne!(snapshot.incoming().as_ptr(), cache_address);
    assert_ne!(snapshot.incoming()[0].as_ptr(), first_group);
    assert_ne!(
        snapshot.segmentation().boundaries.as_ptr(),
        source.boundaries.as_ptr()
    );
    assert_ne!(
        view_snapshot.incoming().as_ptr(),
        snapshot.incoming().as_ptr()
    );
    let moved = borrowed.into_owned();
    assert_eq!(moved.incoming().as_ptr(), cache_address);
    assert_eq!(moved.incoming()[0].as_ptr(), first_group);
    source.boundaries.fill(9);
    source.duration_states.fill(2);
    source.output_states[0].fill(4);
    source.add_jump(0, 0, T::ONE).unwrap();
    drop(source);
    for owner in [&snapshot, &view_snapshot, &moved] {
        compare(owner, &expected, &incoming);
    }
    let cloned = snapshot.clone();
    assert_ne!(cloned.incoming().as_ptr(), snapshot.incoming().as_ptr());
    assert_ne!(
        cloned.incoming()[0].as_ptr(),
        snapshot.incoming()[0].as_ptr()
    );
    assert_ne!(
        cloned.segmentation().boundaries.as_ptr(),
        snapshot.segmentation().boundaries.as_ptr()
    );
    drop(snapshot);
    compare(&cloned, &expected, &incoming);
    let editable = cloned.into_segmentation();
    let boundaries = editable.boundaries.as_ptr();
    let durations = editable.duration_states.as_ptr();
    let states = editable.output_states[0].as_ptr();
    let outgoing = editable.outgoing[0].as_ptr();
    let owned = editable.build_transitions_owned().unwrap();
    assert_eq!(owned.segmentation().boundaries.as_ptr(), boundaries);
    assert_eq!(owned.segmentation().duration_states.as_ptr(), durations);
    assert_eq!(owned.segmentation().output_states[0].as_ptr(), states);
    assert_eq!(owned.outgoing()[0].as_ptr(), outgoing);
    compare(&owned, &expected, &incoming);
    let cache_pointer = owned.incoming().as_ptr();
    for _ in 0..20 {
        assert_eq!(owned.as_view().incoming().as_ptr(), cache_pointer);
    }
    let mut recovered = owned.into_segmentation();
    assert_eq!(recovered.boundaries.as_ptr(), boundaries);
    assert_eq!(recovered.duration_states.as_ptr(), durations);
    assert_eq!(recovered.output_states[0].as_ptr(), states);
    assert_eq!(recovered.outgoing[0].as_ptr(), outgoing);
    recovered.add_jump(0, 0, T::ONE).unwrap();
    compare(&moved, &expected, &incoming);
    compare(&view_snapshot, &expected, &incoming);
}

#[test]
fn owned_f32_transitions_preserve_c_cache_order_fields_and_lifetimes() {
    verify::<f32>();
}
#[test]
fn owned_f64_transitions_preserve_widened_c_cache_fields_and_lifetimes() {
    verify::<f64>();
}

#[test]
fn owned_double_jumps_keep_full_precision_signed_zero_and_extreme_offsets() {
    let mut source = SegmentationData::<f64>::new(0, 2);
    let probability = f64::from_bits(0x3ff0000000000001);
    source.add_jump(0, 0, probability).unwrap();
    source.add_jump(0, 1, -0.0).unwrap();
    source.add_jump(0, i32::MIN, 0.5).unwrap();
    source.add_jump(1, i32::MAX, 0.5).unwrap();
    let expected = source.clone();
    let incoming = source.build_transitions().unwrap().incoming().to_vec();
    let owner = source.build_transitions_owned().unwrap();
    assert_eq!(
        owner.incoming()[0][0].probability.to_bits(),
        probability.to_bits()
    );
    assert_eq!(
        owner.incoming()[1][0].probability.to_bits(),
        (-0.0_f64).to_bits()
    );
    assert_eq!(owner.outgoing()[0][1].delta, i32::MIN);
    let clone = owner.as_view().to_owned();
    drop(owner);
    compare(&clone, &expected, &incoming);
}

#[test]
fn owned_transitions_support_empty_graphs_and_native_validation_errors() {
    fn verify<T: Scalar>() {
        for streams in [0, 2] {
            let owner = SegmentationData::<T>::new(streams, 0)
                .build_transitions_owned()
                .unwrap();
            assert!(owner.incoming().is_empty());
            assert!(owner.outgoing().is_empty());
            assert_eq!(owner.as_view().segmentation().output_states.len(), streams);
            assert_eq!(
                owner.clone().into_segmentation(),
                owner.segmentation().clone()
            );
        }
        for probability in [f64::NAN, f64::INFINITY, -1.0] {
            let mut source = SegmentationData::<T>::new(0, 2);
            source.outgoing[0][0].probability = T::from_f64(probability);
            let borrowed_error = source.build_transitions().unwrap_err();
            assert_eq!(
                source.build_transitions_owned().unwrap_err(),
                borrowed_error
            );
        }
        let mut source = SegmentationData::<T>::new(1, 2);
        source.duration_states.pop();
        assert_eq!(
            source.clone().build_transitions_owned().unwrap_err(),
            source.build_transitions().unwrap_err()
        );
        source = SegmentationData::new(1, 2);
        source.outgoing[0].clear();
        assert_eq!(
            source.clone().build_transitions_owned().unwrap_err(),
            source.build_transitions().unwrap_err()
        );
    }
    verify::<f32>();
    verify::<f64>();
}
