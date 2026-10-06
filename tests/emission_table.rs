use liblrhsmm_rs::EmissionPruning;
#[path = "common/emissions.rs"]
mod common;
use common::sample;

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
