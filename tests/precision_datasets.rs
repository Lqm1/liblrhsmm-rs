use liblrhsmm_rs::StateOccurrence;
use liblrhsmm_rs::data::{JumpParameters, ObservationData, SegmentationData};
use liblrhsmm_rs::data_operations::{SampleSetBuilder, regroup_with_precision as regroup_data};

fn sample() -> (ObservationData<f64>, SegmentationData<f64>) {
    let mut observation = ObservationData::<f64>::new(8, &[2, 1]).unwrap();
    for (stream, data) in observation.streams.iter_mut().enumerate() {
        for (index, value) in data.values.iter_mut().enumerate() {
            *value = 1.0 + ((index + stream + 1) as f64) * f64::EPSILON;
        }
    }
    let mut segmentation = SegmentationData::<f64>::new(2, 5);
    segmentation.boundaries = vec![1, 3, 3, 6, 8];
    segmentation.duration_states = vec![10, 11, 12, 13, 14];
    segmentation.output_states = vec![vec![20, 21, 22, 23, 24], vec![40, 39, 38, 37, 36]];
    (observation, segmentation)
}

#[test]
fn double_regrouping_matches_every_corrected_c_record() {
    let (observation, segmentation) = sample();
    let fixture = include_str!("fixtures/corrected-c-double-datasets.txt");
    assert_eq!(fixture.lines().count(), 150);
    for unit in [1, 2, 3, 5, 8] {
        let dataset = regroup_data(
            std::slice::from_ref(&observation),
            std::slice::from_ref(&segmentation),
            unit,
        )
        .unwrap();
        for line in fixture.lines() {
            let fields: Vec<_> = line.split_whitespace().collect();
            if fields[1].parse::<usize>().unwrap() != unit {
                continue;
            }
            let index = fields[2].parse::<usize>().unwrap();
            match fields[0] {
                "count" => assert_eq!(dataset.observations.len(), index),
                "value" => {
                    let stream = fields[3].parse::<usize>().unwrap();
                    let element = fields[4].parse::<usize>().unwrap();
                    assert_eq!(
                        dataset.observations[index].streams[stream].values[element].to_bits(),
                        fields[5].parse::<f64>().unwrap().to_bits(),
                        "{line}"
                    );
                }
                "state" => {
                    let state = fields[3].parse::<usize>().unwrap();
                    let segmentation = &dataset.segmentations[index];
                    let actual = [
                        segmentation.boundaries[state],
                        segmentation.duration_states[state],
                        segmentation.output_states[0][state],
                        segmentation.output_states[1][state],
                    ];
                    let expected: Vec<i32> = fields[4..]
                        .iter()
                        .map(|value| value.parse().unwrap())
                        .collect();
                    assert_eq!(actual.as_slice(), expected, "{line}");
                }
                _ => panic!("unexpected record: {line}"),
            }
        }
    }
}

#[test]
fn double_regrouping_preserves_every_bit_and_original_samples() {
    let (observation, segmentation) = sample();
    let before = (observation.clone(), segmentation.clone());
    for unit in [1, 2, 3, 5, 8, usize::MAX] {
        let dataset = regroup_data(
            std::slice::from_ref(&observation),
            std::slice::from_ref(&segmentation),
            unit,
        )
        .unwrap();
        assert_eq!(dataset.observations.len(), 5usize.div_ceil(unit));
        for stream in 0..2 {
            let reconstructed: Vec<_> = dataset
                .observations
                .iter()
                .flat_map(|sample| {
                    sample.streams[stream]
                        .values
                        .iter()
                        .map(|value| value.to_bits())
                })
                .collect();
            assert_eq!(
                reconstructed,
                observation.streams[stream]
                    .values
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>()
            );
        }
        let durations: Vec<_> = dataset
            .segmentations
            .iter()
            .flat_map(|sample| sample.duration_states.iter().copied())
            .collect();
        assert_eq!(durations, segmentation.duration_states);
        assert!(
            dataset
                .segmentations
                .iter()
                .flat_map(|sample| &sample.outgoing)
                .flatten()
                .all(|jump| jump.probability == 1.0)
        );
    }
    assert_eq!((observation, segmentation), before);
}

#[test]
fn double_builders_own_samples_and_resegmentation_resets_jumps() {
    let mut observation = ObservationData::<f64>::new(1, &[1]).unwrap();
    observation.streams[0].values[0] = 1.0 + f64::EPSILON;
    let mut observations = SampleSetBuilder::new(2).unwrap();
    observations.samples[1] = Some(observation.clone());
    assert!(observations.clone().build().is_err());
    observations.samples[0] = Some(observation.clone());
    let mut built = observations.build().unwrap();
    built[0].streams[0].values[0] = 7.0;
    assert_eq!(built[1], observation);

    let mut source = SegmentationData::<f64>::new(1, 2);
    source.boundaries = vec![1, 2];
    source.duration_states = vec![3, 4];
    source.output_states = vec![vec![5, 6]];
    source.outgoing[0] = vec![JumpParameters {
        delta: 1,
        probability: 1.0 - f64::EPSILON,
    }];
    let before = source.clone();
    let mut segments = SampleSetBuilder::new(1).unwrap();
    segments.samples[0] = Some(source.clone());
    assert_eq!(segments.build().unwrap()[0], source);
    let path = [
        StateOccurrence { state: 1, end: 1 },
        StateOccurrence { state: 0, end: 2 },
    ];
    let remapped = source.resegment(&path).unwrap();
    assert_eq!(remapped.duration_states, [4, 3]);
    assert_eq!(remapped.output_states[0], [6, 5]);
    assert!(
        remapped
            .outgoing
            .iter()
            .flatten()
            .all(|jump| jump.probability == 1.0)
    );
    assert!(
        source
            .resegment(&[StateOccurrence { state: 2, end: 1 }])
            .is_err()
    );
    assert_eq!(source, before);
}
