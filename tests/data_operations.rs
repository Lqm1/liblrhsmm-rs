use liblrhsmm_rs::{Jump, Observation, Segmentation, StateOccurrence, regroup_data};

fn sample() -> (Observation, Segmentation) {
    let mut observation = Observation::new(8, &[2, 1]).unwrap();
    for time in 0..8 {
        for (stream, data) in observation.streams.iter_mut().enumerate() {
            for dimension in 0..data.dimensions {
                data.values[time * data.dimensions + dimension] =
                    time as f32 * 10.0 + stream as f32 * 100.0 + dimension as f32;
            }
        }
    }
    let mut segmentation = Segmentation::new(2, 5);
    segmentation.boundaries = vec![1, 3, 3, 6, 8];
    segmentation.duration_states = vec![10, 11, 12, 13, 14];
    segmentation.output_states = vec![vec![20, 21, 22, 23, 24], vec![40, 39, 38, 37, 36]];
    (observation, segmentation)
}

#[test]
fn partitions_match_corrected_c_and_preserve_all_frames() {
    let (observation, segmentation) = sample();
    let dataset = regroup_data(
        std::slice::from_ref(&observation),
        std::slice::from_ref(&segmentation),
        2,
    )
    .unwrap();
    assert_eq!(dataset.observations.len(), 3);
    #[cfg(feature = "serialization")]
    {
        let observations: [&[u8]; 3] = [
            include_bytes!("fixtures/corrected-c-regroup-0-observation.bin"),
            include_bytes!("fixtures/corrected-c-regroup-1-observation.bin"),
            include_bytes!("fixtures/corrected-c-regroup-2-observation.bin"),
        ];
        let segmentations: [&[u8]; 3] = [
            include_bytes!("fixtures/corrected-c-regroup-0-segmentation.bin"),
            include_bytes!("fixtures/corrected-c-regroup-1-segmentation.bin"),
            include_bytes!("fixtures/corrected-c-regroup-2-segmentation.bin"),
        ];
        for index in 0..3 {
            assert_eq!(
                dataset.observations[index],
                Observation::read_from(observations[index]).unwrap()
            );
            assert_eq!(
                dataset.segmentations[index],
                Segmentation::read_from(segmentations[index]).unwrap()
            );
            let mut encoded = Vec::new();
            dataset.observations[index].write_to(&mut encoded).unwrap();
            assert_eq!(encoded, observations[index]);
            encoded.clear();
            dataset.segmentations[index].write_to(&mut encoded).unwrap();
            assert_eq!(encoded, segmentations[index]);
        }
    }
    for unit_size in [1, 2, 3, 5, 8, usize::MAX] {
        let dataset = regroup_data(
            std::slice::from_ref(&observation),
            std::slice::from_ref(&segmentation),
            unit_size,
        )
        .unwrap();
        assert_eq!(dataset.observations.len(), 5usize.div_ceil(unit_size));
        for stream in 0..2 {
            let reconstructed: Vec<_> = dataset
                .observations
                .iter()
                .flat_map(|sample| sample.streams[stream].values.iter().copied())
                .collect();
            assert_eq!(reconstructed, observation.streams[stream].values);
        }
        assert_eq!(
            dataset
                .segmentations
                .iter()
                .flat_map(|seg| &seg.duration_states)
                .copied()
                .collect::<Vec<_>>(),
            segmentation.duration_states
        );
    }
}

#[test]
fn resegmentation_matches_c_for_skips_and_repeated_states() {
    let (_, mut source) = sample();
    source.add_jump(0, 3, 0.25).unwrap();
    let path = [
        StateOccurrence { state: 0, end: 2 },
        StateOccurrence { state: 3, end: 4 },
        StateOccurrence { state: 3, end: 5 },
        StateOccurrence { state: 1, end: 8 },
    ];
    let actual = source.resegment(&path).unwrap();
    #[cfg(feature = "serialization")]
    {
        let reference = include_bytes!("fixtures/c-resegmentation.bin");
        assert_eq!(
            actual,
            Segmentation::read_from(reference.as_slice()).unwrap()
        );
        let mut encoded = Vec::new();
        actual.write_to(&mut encoded).unwrap();
        assert_eq!(encoded, reference);
    }
    assert!(actual.outgoing.iter().all(|jumps| jumps
        == &[Jump {
            delta: 1,
            probability: 1.0
        }]));
    let before = source.clone();
    assert!(
        source
            .resegment(&[StateOccurrence { state: 5, end: 2 }])
            .is_err()
    );
    assert!(
        source
            .resegment(&[
                StateOccurrence { state: 0, end: 2 },
                StateOccurrence { state: 1, end: 1 }
            ])
            .is_err()
    );
    assert_eq!(source, before);
    assert!(source.resegment(&[]).unwrap().boundaries.is_empty());
}

#[test]
fn invalid_and_empty_datasets_are_checked() {
    let (observation, mut segmentation) = sample();
    assert!(regroup_data(&[], &[], 0).is_err());
    assert!(regroup_data(std::slice::from_ref(&observation), &[], 2).is_err());
    segmentation.boundaries[4] = 7;
    assert!(
        regroup_data(
            std::slice::from_ref(&observation),
            std::slice::from_ref(&segmentation),
            2
        )
        .is_err()
    );
    assert!(regroup_data(&[], &[], 1).unwrap().observations.is_empty());
    assert!(
        regroup_data(
            &[Observation::new(0, &[1]).unwrap()],
            &[Segmentation::new(1, 0)],
            1
        )
        .unwrap()
        .observations
        .is_empty()
    );
    assert!(Observation::new(1, &[]).is_err());
    assert!(Observation::new(1, &[0]).is_err());
    let dataset = regroup_data(
        &[observation.clone(), observation],
        &[sample().1, sample().1],
        3,
    )
    .unwrap();
    assert_eq!(dataset.observations.len(), 4);
}
