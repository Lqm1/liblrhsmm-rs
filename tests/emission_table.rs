use liblrhsmm_rs::{
    Duration, EmissionPruning, GaussianMixture, Model, Observation, ObservationStream,
    Segmentation, Stream,
};

fn sample() -> (Model, Observation, Segmentation) {
    let mut streams = Vec::new();
    for stream in 0..2 {
        let mut distributions = Vec::new();
        for state in 0..3 {
            let mut gmm = GaussianMixture::new(
                if stream == 0 { 2 } else { 1 },
                if stream == 0 { 2 } else { 1 },
            )
            .unwrap();
            for index in 0..gmm.means.len() {
                gmm.means[index] = state as f32 * 0.5 + index as f32 * 0.25;
                gmm.variances[index] = 0.75 + index as f32 * 0.25;
            }
            distributions.push(gmm);
        }
        streams.push(Stream {
            weight: if stream == 0 { 0.5 } else { 1.25 },
            mixtures: distributions,
        });
    }
    let model = Model {
        streams,
        durations: vec![
            Duration {
                mean: 4.0,
                variance: 1.0,
                ..Duration::default()
            };
            3
        ],
    };
    let observation = Observation {
        frames: 12,
        streams: (0..2)
            .map(|stream| {
                let dimensions = if stream == 0 { 2 } else { 1 };
                ObservationStream {
                    dimensions,
                    values: (0..12)
                        .flat_map(|frame| {
                            (0..dimensions)
                                .map(move |dimension| frame as f32 * 0.125 + dimension as f32 * 0.5)
                        })
                        .collect(),
                }
            })
            .collect(),
    };
    let mut segmentation = Segmentation::new(2, 3);
    segmentation.boundaries = vec![4, 8, 12];
    segmentation.duration_states = vec![0, 1, 2];
    segmentation.output_states = vec![vec![0, 1, 2], vec![2, 1, 0]];
    (model, observation, segmentation)
}

#[test]
fn multistream_tables_match_c_with_both_pruning_conventions() {
    let (model, observation, segmentation) = sample();
    assert_eq!(
        include_str!("fixtures/c-emission-tables.txt")
            .lines()
            .count(),
        72
    );
    let tables = [
        model
            .emission_log_probabilities(
                &observation,
                &segmentation,
                0.75,
                EmissionPruning::AroundBoundaries { radius: 1 },
            )
            .unwrap(),
        model
            .emission_log_probabilities(
                &observation,
                &segmentation,
                0.75,
                EmissionPruning::Linear { slope: 0.34 },
            )
            .unwrap(),
    ];
    for line in include_str!("fixtures/c-emission-tables.txt").lines() {
        let mut fields = line.split_whitespace();
        let table: usize = fields.next().unwrap().parse().unwrap();
        let index: usize = fields.next().unwrap().parse().unwrap();
        let expected: f32 = fields.next().unwrap().parse().unwrap();
        assert_eq!(
            tables[table].values()[index],
            expected,
            "C mismatch: {line}"
        );
    }
    assert_eq!(tables[0].frames(), 12);
    assert_eq!(tables[0].states(), 3);
    assert_eq!(tables[0].get(12, 0), None);
    assert_eq!(tables[0].get(0, 3), None);
    let unpruned = model
        .emission_log_probabilities(&observation, &segmentation, 0.75, EmissionPruning::None)
        .unwrap();
    // The inclusive segment boundary evaluates state 2; the linear band excludes it.
    assert_eq!(tables[0].get(4, 2), unpruned.get(4, 2));
    assert_eq!(tables[1].get(4, 2), Some(-1e15));
}

#[test]
fn non_log_emissions_match_c_including_pruned_underflow() {
    let (model, observation, segmentation) = sample();
    let result = model
        .emission_probabilities(
            &observation,
            &segmentation,
            0.75,
            EmissionPruning::AroundBoundaries { radius: 1 },
        )
        .unwrap();
    assert_eq!(result.frames(), 12);
    assert_eq!(result.states(), 3);
    assert_eq!(result.values().len(), 36);
    for line in include_str!("fixtures/c-probability-forms.txt")
        .lines()
        .filter(|line| line.starts_with("emission"))
    {
        let mut fields = line.split_whitespace();
        assert_eq!(fields.next(), Some("emission"));
        let index: usize = fields.next().unwrap().parse().unwrap();
        let expected: f32 = fields.next().unwrap().parse().unwrap();
        assert_eq!(result.get(index / 3, index % 3), Some(expected), "{line}");
    }
    // C's approximate exponential clamps negative scores, including pruned pairs.
    assert!(result.get(0, 2).unwrap() > 0.0);
    assert!(result.get(12, 0).is_none());
    assert!(result.get(0, 3).is_none());
    let log = model
        .emission_log_probabilities(
            &observation,
            &segmentation,
            0.75,
            EmissionPruning::AroundBoundaries { radius: 1 },
        )
        .unwrap();
    assert_eq!(log.probabilities(), result);
}

#[test]
fn invalid_state_mapping_and_boundaries_are_errors() {
    let (model, observation, mut segmentation) = sample();
    segmentation.output_states[0][0] = -1;
    assert!(
        model
            .emission_log_probabilities(&observation, &segmentation, 1.0, EmissionPruning::None)
            .is_err()
    );
    segmentation.output_states[0][0] = 0;
    segmentation.boundaries[2] = 11;
    assert!(
        model
            .emission_log_probabilities(
                &observation,
                &segmentation,
                1.0,
                EmissionPruning::AroundBoundaries { radius: 1 }
            )
            .is_err()
    );
    assert!(
        model
            .emission_log_probabilities(
                &observation,
                &segmentation,
                1.0,
                EmissionPruning::Linear { slope: -0.1 }
            )
            .is_err()
    );
}
