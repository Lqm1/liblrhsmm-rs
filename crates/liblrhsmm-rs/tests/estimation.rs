use liblrhsmm_rs::{GeometricOptions, HsmmOptions, ModelStatistics, UpdateOptions};
#[path = "common/estimation.rs"]
mod common;
use common::sample;

fn round_trip(model: liblrhsmm_rs::Model) -> liblrhsmm_rs::Model {
    #[cfg(feature = "serialization")]
    {
        let mut bytes = Vec::new();
        model.write_to(&mut bytes).unwrap();
        let decoded = liblrhsmm_rs::Model::read_from(bytes.as_slice()).unwrap();
        assert_eq!(decoded, model);
        decoded
    }
    #[cfg(not(feature = "serialization"))]
    {
        model
    }
}

#[test]
fn both_expectation_steps_match_c_statistics() {
    let (model, observation, segmentation) = sample();
    let fixture = include_str!("../../../tests/fixtures/corrected-c-estimation.txt");
    assert_eq!(fixture.lines().count(), 40);
    for mode in 0..2 {
        for temperature in 1..=2 {
            let mut statistics = ModelStatistics::from_model(&model).unwrap();
            let value = temperature as f32 * 0.5;
            let total = if mode == 0 {
                statistics
                    .estimate_hsmm(
                        &model,
                        &observation,
                        &segmentation,
                        HsmmOptions {
                            temperature: value,
                            state_radius: 1.0,
                            ..HsmmOptions::default()
                        },
                    )
                    .unwrap()
            } else {
                statistics
                    .estimate_geometric(
                        &model,
                        &observation,
                        &segmentation,
                        GeometricOptions {
                            temperature: value,
                            pruning_slope: 0.5,
                        },
                    )
                    .unwrap()
            };
            for line in fixture.lines() {
                let mut fields = line.split_whitespace();
                let kind = fields.next().unwrap();
                let case_mode: usize = fields.next().unwrap().parse().unwrap();
                let case_temperature: usize = fields.next().unwrap().parse().unwrap();
                if (case_mode, case_temperature) != (mode, temperature) {
                    continue;
                }
                let actual = match kind {
                    "total" => vec![total],
                    "duration" => {
                        let state: usize = fields.next().unwrap().parse().unwrap();
                        let stat = statistics.durations[state];
                        vec![stat.value_sum, stat.squared_sum, stat.occupancy]
                    }
                    "mixture" => {
                        let stream: usize = fields.next().unwrap().parse().unwrap();
                        let state: usize = fields.next().unwrap().parse().unwrap();
                        let stat = &statistics.streams[stream].mixtures[state];
                        let mut values = vec![stat.state_occupancy];
                        values.extend(&stat.component_occupancies);
                        for index in 0..stat.weighted_sums.len() {
                            values.extend([
                                stat.weighted_sums[index],
                                stat.weighted_squared_sums[index],
                            ]);
                        }
                        values
                    }
                    _ => panic!("unknown reference record"),
                };
                let expected: Vec<f32> = fields.map(|x| x.parse().unwrap()).collect();
                assert_eq!(actual, expected, "{line}");
            }
        }
    }
}

#[test]
fn expectation_failure_does_not_modify_prior_statistics() {
    let (model, mut observation, segmentation) = sample();
    let mut statistics = ModelStatistics::from_model(&model).unwrap();
    statistics
        .estimate_hsmm(&model, &observation, &segmentation, HsmmOptions::default())
        .unwrap();
    let before = statistics.clone();
    observation.streams[1].values[11] = f32::NAN;
    assert!(
        statistics
            .estimate_hsmm(&model, &observation, &segmentation, HsmmOptions::default())
            .is_err()
    );
    assert_eq!(statistics, before);
    assert!(
        statistics
            .estimate_geometric(
                &model,
                &observation,
                &segmentation,
                GeometricOptions::default()
            )
            .is_err()
    );
    assert_eq!(statistics, before);
}

#[test]
fn estimation_update_serialization_and_inference_are_connected() {
    let (mut model, observation, segmentation) = sample();
    for _ in 0..2 {
        let mut statistics = ModelStatistics::from_model(&model).unwrap();
        assert!(
            statistics
                .estimate_hsmm(&model, &observation, &segmentation, HsmmOptions::default())
                .unwrap()
                .is_finite()
        );
        model.update(&statistics, UpdateOptions::default()).unwrap();
    }
    let decoded = round_trip(model);
    let emissions = decoded
        .emission_log_probabilities(
            &observation,
            &segmentation,
            1.0,
            liblrhsmm_rs::EmissionPruning::None,
        )
        .unwrap();
    let alignment = decoded
        .viterbi_hsmm(&segmentation, &emissions, HsmmOptions::default())
        .unwrap();
    assert_eq!(
        alignment.occurrences.last().unwrap().end,
        observation.frames
    );
    assert!(alignment.log_probability.is_finite());
}

#[test]
fn geometric_learning_round_trip_and_temperature_changes() {
    let (mut model, observation, segmentation) = sample();
    let options = GeometricOptions {
        temperature: 1.0,
        pruning_slope: 0.5,
    };
    for _ in 0..2 {
        let mut statistics = ModelStatistics::from_model(&model).unwrap();
        assert!(
            statistics
                .estimate_geometric(&model, &observation, &segmentation, options)
                .unwrap()
                .is_finite()
        );
        model
            .update(
                &statistics,
                UpdateOptions {
                    duration_mode: liblrhsmm_rs::DurationMode::Geometric,
                    ..UpdateOptions::default()
                },
            )
            .unwrap();
    }
    let decoded = round_trip(model);
    let emissions = decoded
        .emission_log_probabilities(
            &observation,
            &segmentation,
            1.0,
            liblrhsmm_rs::EmissionPruning::Linear { slope: 0.5 },
        )
        .unwrap();
    let alignment = decoded
        .viterbi_geometric(&segmentation, &emissions, options)
        .unwrap();
    assert!(alignment.log_probability.is_finite());

    let mut baseline = ModelStatistics::from_model(&decoded).unwrap();
    let expected = baseline
        .estimate_hsmm(
            &decoded,
            &observation,
            &segmentation,
            HsmmOptions::default(),
        )
        .unwrap();
    let mut alternate = ModelStatistics::from_model(&decoded).unwrap();
    alternate
        .estimate_hsmm(
            &decoded,
            &observation,
            &segmentation,
            HsmmOptions {
                temperature: 0.5,
                ..HsmmOptions::default()
            },
        )
        .unwrap();
    alternate.clear();
    let actual = alternate
        .estimate_hsmm(
            &decoded,
            &observation,
            &segmentation,
            HsmmOptions::default(),
        )
        .unwrap();
    assert_eq!(actual, expected);
    assert_eq!(alternate, baseline);
}
