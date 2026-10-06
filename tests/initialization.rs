use liblrhsmm_rs::{
    Duration, GaussianMixture, Model, ModelStatistics, Observation, ObservationStream,
    Segmentation, Stream, UpdateOptions,
};

fn sample() -> (Model, Observation, Segmentation) {
    let model = Model {
        streams: [2, 1]
            .into_iter()
            .map(|dimensions| Stream {
                weight: 1.0,
                mixtures: vec![GaussianMixture::new(1, dimensions).unwrap(); 2],
            })
            .collect(),
        durations: vec![Duration::default(); 2],
    };
    let observation = Observation {
        frames: 6,
        streams: [2, 1]
            .into_iter()
            .enumerate()
            .map(|(stream, dimensions)| ObservationStream {
                dimensions,
                values: (0..6)
                    .flat_map(|time| {
                        (0..dimensions).map(move |dimension| {
                            (time as f32 - 2.0) * 0.25 + dimension as f32 + stream as f32 * 3.0
                        })
                    })
                    .collect(),
            })
            .collect(),
    };
    let mut segmentation = Segmentation::new(2, 3);
    segmentation.boundaries = vec![2, 2, 6];
    segmentation.duration_states = vec![0, 1, 0];
    segmentation.output_states = vec![vec![0, 1, 0], vec![1, 0, 1]];
    (model, observation, segmentation)
}

#[test]
fn aligned_statistics_match_c_with_shared_and_empty_states() {
    let (mut model, observation, segmentation) = sample();
    let mut statistics = ModelStatistics::from_model(&model).unwrap();
    for _ in 0..2 {
        statistics
            .collect_initialization(&model, &observation, &segmentation)
            .unwrap();
    }
    for line in include_str!("fixtures/c-initialization.txt").lines() {
        let mut fields = line.split_whitespace();
        let kind = fields.next().unwrap();
        let index: usize = fields.next().unwrap().parse().unwrap();
        let actual = if kind == "duration" {
            let stat = statistics.durations[index];
            vec![stat.value_sum, stat.squared_sum, stat.occupancy]
        } else {
            assert_eq!(kind, "mixture");
            let state: usize = fields.next().unwrap().parse().unwrap();
            let stat = &statistics.streams[index].mixtures[state];
            let mut values = vec![stat.state_occupancy, stat.component_occupancies[0]];
            for dimension in 0..stat.dimensions {
                values.extend([
                    stat.weighted_sums[dimension],
                    stat.weighted_squared_sums[dimension],
                ]);
            }
            values
        };
        let expected: Vec<f32> = fields.map(|x| x.parse().unwrap()).collect();
        assert_eq!(actual, expected, "{line}");
    }
    model.update(&statistics, UpdateOptions::default()).unwrap();
    assert_eq!(model.durations[0].mean, 3.0);
    assert_eq!(model.durations[0].variance, 1.0);
    assert_eq!(model.streams[0].mixtures[0].means, [0.125, 1.125]);
    // The empty emission state retains its flat-start values.
    assert_eq!(model.streams[0].mixtures[1].means, [0.0, 0.0]);
    statistics.clear();
    assert_eq!(statistics, ModelStatistics::from_model(&model).unwrap());
}

#[test]
fn invalid_initialization_is_atomic() {
    let (mut model, mut observation, mut segmentation) = sample();
    let mut statistics = ModelStatistics::from_model(&model).unwrap();
    let before = statistics.clone();
    segmentation.output_states[1][2] = -1;
    assert!(
        statistics
            .collect_initialization(&model, &observation, &segmentation)
            .is_err()
    );
    assert_eq!(statistics, before);
    segmentation.output_states[1][2] = 1;
    observation.streams[1].values[5] = f32::MAX;
    assert!(
        statistics
            .collect_initialization(&model, &observation, &segmentation)
            .is_err()
    );
    assert_eq!(statistics, before);
    observation.streams[1].values[5] = 3.75;
    model.streams[1].mixtures[1] = GaussianMixture::new(2, 1).unwrap();
    statistics = ModelStatistics::from_model(&model).unwrap();
    let before = statistics.clone();
    assert!(
        statistics
            .collect_initialization(&model, &observation, &segmentation)
            .is_err()
    );
    assert_eq!(statistics, before);
}

#[test]
fn long_duration_square_avoids_c_integer_overflow() {
    let (model, _, _) = sample();
    let observation = Observation {
        frames: 50_000,
        streams: vec![
            ObservationStream {
                dimensions: 2,
                values: vec![0.0; 100_000],
            },
            ObservationStream {
                dimensions: 1,
                values: vec![0.0; 50_000],
            },
        ],
    };
    let mut segmentation = Segmentation::new(2, 1);
    segmentation.boundaries[0] = 50_000;
    let mut statistics = ModelStatistics::from_model(&model).unwrap();
    statistics
        .collect_initialization(&model, &observation, &segmentation)
        .unwrap();
    assert_eq!(statistics.durations[0].squared_sum, 2_500_000_000.0);
}
