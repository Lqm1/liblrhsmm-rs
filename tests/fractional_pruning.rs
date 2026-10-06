use liblrhsmm_rs::{
    EmissionPruning, HsmmExpectation, HsmmOptions, MixtureOccupancyMode, ModelStatistics,
    PreparationOptions,
};

#[path = "common/emissions.rs"]
mod emissions;
#[path = "common/estimation.rs"]
mod estimation;
#[path = "common/hsmm.rs"]
mod hsmm;
#[path = "common/mixtures.rs"]
mod mixtures;

const RADII: [f32; 4] = [0.25, 0.5, 1.75, 2.5];

#[test]
fn fractional_emission_bounds_match_c_and_include_the_preceding_state() {
    let (model, observation, segmentation) = emissions::sample();
    let prepared = model.prepare(PreparationOptions::default()).unwrap();
    let tables: Vec<_> = RADII
        .iter()
        .map(|&radius| {
            let pruning = EmissionPruning::AroundBoundaries { radius };
            let table = model
                .emission_log_probabilities(&observation, &segmentation, 0.75, pruning)
                .unwrap();
            assert_eq!(
                prepared
                    .emission_log_probabilities(&observation, &segmentation, 0.75, pruning)
                    .unwrap(),
                table
            );
            table
        })
        .collect();
    let fixture = include_str!("fixtures/c-fractional-emissions.txt");
    assert_eq!(fixture.lines().count(), 144);
    for line in fixture.lines() {
        let fields: Vec<_> = line.split_whitespace().collect();
        let scenario: usize = fields[0].parse().unwrap();
        let index: usize = fields[1].parse().unwrap();
        assert_eq!(
            tables[scenario].values()[index],
            fields[2].parse::<f32>().unwrap(),
            "{line}"
        );
    }
    // At frame 5 the center is state 1. Radius 0.5 includes state 0 but excludes 2.
    assert_ne!(tables[1].get(5, 0), Some(liblrhsmm_rs::numerics::LOG_ZERO));
    assert_eq!(tables[1].get(5, 2), Some(liblrhsmm_rs::numerics::LOG_ZERO));
}

#[test]
fn fractional_hsmm_scores_and_paths_match_c() {
    let fixture = include_str!("fixtures/c-fractional-hsmm.txt");
    assert_eq!(
        fixture
            .lines()
            .filter(|line| line.starts_with("score "))
            .count(),
        192
    );
    for (scenario, radius) in RADII.into_iter().enumerate() {
        let (model, segmentation, emissions, mut options) = hsmm::sample(scenario);
        options.state_radius = radius;
        let forward = model
            .forward_hsmm(&segmentation, &emissions, options)
            .unwrap();
        let backward = model
            .backward_hsmm(&segmentation, &emissions, options)
            .unwrap();
        let path = model
            .viterbi_hsmm(&segmentation, &emissions, options)
            .unwrap();
        let prepared = model
            .prepare(PreparationOptions {
                duration_cache_length: 2,
                temperature: options.temperature,
                duration_weight: options.duration_weight,
            })
            .unwrap();
        assert_eq!(
            prepared
                .forward_hsmm(&segmentation, &emissions, options)
                .unwrap(),
            forward
        );
        assert_eq!(
            prepared
                .backward_hsmm(&segmentation, &emissions, options)
                .unwrap(),
            backward
        );
        assert_eq!(
            prepared
                .viterbi_hsmm(&segmentation, &emissions, options)
                .unwrap(),
            path
        );
        let mut path_count = 0;
        for line in fixture.lines() {
            let fields: Vec<_> = line.split_whitespace().collect();
            if fields[1].parse::<usize>().unwrap() != scenario {
                continue;
            }
            let index: usize = fields[2].parse().unwrap();
            if fields[0] == "score" {
                let actual = [
                    forward.begin.values()[index],
                    forward.end.values()[index],
                    backward.begin.values()[index],
                    backward.end.values()[index],
                ];
                let expected: Vec<f32> = fields[3..].iter().map(|v| v.parse().unwrap()).collect();
                assert_eq!(actual.as_slice(), expected, "{line}");
            } else {
                assert_eq!(fields[0], "path");
                assert_eq!(
                    path.occurrences[index].end,
                    fields[3].parse::<usize>().unwrap()
                );
                assert_eq!(
                    path.occurrences[index].state,
                    fields[4].parse::<usize>().unwrap()
                );
                path_count += 1;
            }
        }
        assert_eq!(path.occurrences.len(), path_count);
    }
}

#[test]
fn fractional_duration_and_state_posteriors_match_corrected_c() {
    let fixture = include_str!("fixtures/corrected-c-fractional-durations.txt");
    assert_eq!(fixture.lines().count(), 192);
    for (scenario, radius) in RADII.into_iter().enumerate() {
        let (model, segmentation, emissions, mut options) = hsmm::sample(scenario);
        options.state_radius = radius;
        let forward = model
            .forward_hsmm(&segmentation, &emissions, options)
            .unwrap();
        let backward = model
            .backward_hsmm(&segmentation, &emissions, options)
            .unwrap();
        let total = forward.end.total_through_state(&backward.end, 0).unwrap();
        let durations = model
            .duration_occupancies(
                &segmentation,
                &emissions,
                &forward,
                &backward,
                total,
                options,
            )
            .unwrap();
        let states = model
            .hsmm_state_occupancies(&segmentation, &emissions, &durations, options)
            .unwrap();
        for line in fixture.lines() {
            let fields: Vec<_> = line.split_whitespace().collect();
            if fields[0].parse::<usize>().unwrap() != scenario {
                continue;
            }
            let frame: usize = fields[1].parse().unwrap();
            let state: usize = fields[2].parse().unwrap();
            let minimum: usize = fields[3].parse().unwrap();
            let count: usize = fields[4].parse().unwrap();
            let actual = durations.get(frame, state);
            assert_eq!(actual.is_some(), count != 0, "{line}");
            if let Some(actual) = actual {
                assert_eq!(actual.minimum, minimum);
                let expected: Vec<f32> = fields[5..5 + count]
                    .iter()
                    .map(|v| v.parse().unwrap())
                    .collect();
                assert_eq!(actual.probabilities, expected, "{line}");
            }
            assert_eq!(
                states.get(frame, state),
                Some(fields[5 + count].parse::<f32>().unwrap()),
                "{line}"
            );
        }
    }
}

#[test]
fn fractional_component_posteriors_match_c() {
    let (model, observation, segmentation, occupancy) = mixtures::sample();
    let fixture = include_str!("fixtures/c-fractional-mixtures.txt");
    assert_eq!(fixture.lines().count(), 192);
    for temperature in [0.5, 1.0] {
        for (scenario, radius) in RADII.into_iter().enumerate() {
            let result = model
                .mixture_occupancies(
                    &observation,
                    &segmentation,
                    &occupancy,
                    0,
                    temperature,
                    MixtureOccupancyMode::Hsmm {
                        state_radius: radius,
                    },
                )
                .unwrap();
            for line in fixture.lines() {
                let fields: Vec<_> = line.split_whitespace().collect();
                if fields[0].parse::<usize>().unwrap() != scenario
                    || fields[1].parse::<f32>().unwrap() != temperature
                {
                    continue;
                }
                let frame: usize = fields[2].parse().unwrap();
                let state: usize = fields[3].parse().unwrap();
                let count: usize = fields[4].parse().unwrap();
                let actual = result.get(frame, state);
                assert_eq!(actual.is_some(), count != 0, "{line}");
                if let Some(actual) = actual {
                    let expected: Vec<f32> =
                        fields[5..].iter().map(|v| v.parse().unwrap()).collect();
                    assert_eq!(actual, expected, "{line}");
                }
            }
        }
    }
}

#[test]
fn fractional_estimation_matches_corrected_c_likelihoods_and_all_moments() {
    let (model, observation, segmentation) = estimation::sample();
    let fixture = include_str!("fixtures/corrected-c-fractional-estimation.txt");
    assert_eq!(fixture.lines().count(), 80);
    for (scenario, radius) in RADII.into_iter().enumerate() {
        for temperature in 1..=2 {
            let mut statistics = ModelStatistics::from_model(&model).unwrap();
            let total = statistics
                .estimate_hsmm(
                    &model,
                    &observation,
                    &segmentation,
                    HsmmOptions {
                        state_radius: radius,
                        temperature: temperature as f32 * 0.5,
                        ..HsmmOptions::default()
                    },
                )
                .unwrap();
            for line in fixture.lines() {
                let mut fields = line.split_whitespace();
                let kind = fields.next().unwrap();
                let case: usize = fields.next().unwrap().parse().unwrap();
                let case_temperature: usize = fields.next().unwrap().parse().unwrap();
                if (case, case_temperature) != (scenario, temperature) {
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
                let expected: Vec<f32> = fields.map(|v| v.parse().unwrap()).collect();
                assert_eq!(actual, expected, "{line}");
            }
        }
    }
}

#[test]
fn invalid_radii_fail_in_every_entry_point_without_changing_statistics() {
    let (model, observation, segmentation) = estimation::sample();
    let options = HsmmOptions {
        state_radius: 0.5,
        ..HsmmOptions::default()
    };
    let emissions = model
        .emission_log_probabilities(
            &observation,
            &segmentation,
            1.0,
            EmissionPruning::AroundBoundaries { radius: 0.5 },
        )
        .unwrap();
    let forward = model
        .forward_hsmm(&segmentation, &emissions, options)
        .unwrap();
    let backward = model
        .backward_hsmm(&segmentation, &emissions, options)
        .unwrap();
    let total = forward.end.total_through_state(&backward.end, 0).unwrap();
    let durations = model
        .duration_occupancies(
            &segmentation,
            &emissions,
            &forward,
            &backward,
            total,
            options,
        )
        .unwrap();
    let states = model
        .hsmm_state_occupancies(&segmentation, &emissions, &durations, options)
        .unwrap();
    let mixtures: Vec<_> = (0..model.streams.len())
        .map(|stream| {
            model
                .mixture_occupancies(
                    &observation,
                    &segmentation,
                    &states,
                    stream,
                    1.0,
                    MixtureOccupancyMode::Hsmm { state_radius: 0.5 },
                )
                .unwrap()
        })
        .collect();
    for radius in [-1.0, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        let invalid = HsmmOptions {
            state_radius: radius,
            ..options
        };
        assert!(
            model
                .emission_log_probabilities(
                    &observation,
                    &segmentation,
                    1.0,
                    EmissionPruning::AroundBoundaries { radius }
                )
                .is_err()
        );
        assert!(
            model
                .forward_hsmm(&segmentation, &emissions, invalid)
                .is_err()
        );
        assert!(
            model
                .backward_hsmm(&segmentation, &emissions, invalid)
                .is_err()
        );
        assert!(
            model
                .viterbi_hsmm(&segmentation, &emissions, invalid)
                .is_err()
        );
        assert!(
            model
                .duration_occupancies(
                    &segmentation,
                    &emissions,
                    &forward,
                    &backward,
                    total,
                    invalid
                )
                .is_err()
        );
        assert!(
            model
                .hsmm_state_occupancies(&segmentation, &emissions, &durations, invalid)
                .is_err()
        );
        assert!(
            model
                .mixture_occupancies(
                    &observation,
                    &segmentation,
                    &states,
                    0,
                    1.0,
                    MixtureOccupancyMode::Hsmm {
                        state_radius: radius
                    }
                )
                .is_err()
        );
        let mut statistics = ModelStatistics::from_model(&model).unwrap();
        let previous = statistics.clone();
        assert!(
            statistics
                .collect_hsmm(
                    &model,
                    &observation,
                    &segmentation,
                    HsmmExpectation {
                        durations: &durations,
                        states: &states,
                        mixtures: &mixtures,
                        state_radius: radius
                    }
                )
                .is_err()
        );
        assert_eq!(statistics, previous);
        assert!(
            statistics
                .estimate_hsmm(&model, &observation, &segmentation, invalid)
                .is_err()
        );
        assert_eq!(statistics, previous);
    }
}
