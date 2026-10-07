use liblrhsmm_rs::data::{ObservationData, SegmentationData};
use liblrhsmm_rs::emission::LogEmissionTable;
use liblrhsmm_rs::geometric::GeometricSettings;
use liblrhsmm_rs::hsmm::HsmmSettings;
use liblrhsmm_rs::mixture_occupancy::MixtureOccupancySettings;
use liblrhsmm_rs::model::{
    DurationParameters, GaussianMixtureParameters, ModelParameters, StreamLayout,
};
use liblrhsmm_rs::statistics::{
    DurationSufficientStatistics, GaussianSufficientStatistics, ModelSufficientStatistics,
};
use liblrhsmm_rs::update::{DurationMode, UpdateSettings};

#[test]
fn double_training_repeats_and_rejects_invalid_observations_atomically() {
    for geometric in [false, true] {
        let (mut model, observation, segmentation) =
            estimation_sample::sample_with_precision::<f64>();
        for _ in 0..3 {
            let mut statistics = ModelSufficientStatistics::from_model(&model).unwrap();
            let estimate = |statistics: &mut ModelSufficientStatistics<f64>,
                            observation: &ObservationData<f64>| {
                if geometric {
                    statistics.estimate_geometric(
                        &model,
                        observation,
                        &segmentation,
                        GeometricSettings {
                            pruning_slope: 0.5,
                            ..Default::default()
                        },
                    )
                } else {
                    statistics.estimate_hsmm(
                        &model,
                        observation,
                        &segmentation,
                        HsmmSettings::default(),
                    )
                }
            };
            assert!(estimate(&mut statistics, &observation).unwrap().is_finite());
            let before = statistics.clone();
            let mut invalid = observation.clone();
            *invalid
                .streams
                .last_mut()
                .unwrap()
                .values
                .last_mut()
                .unwrap() = f64::NAN;
            assert!(estimate(&mut statistics, &invalid).is_err());
            assert_eq!(statistics, before);
            model
                .update(
                    &statistics,
                    UpdateSettings {
                        duration_mode: if geometric {
                            DurationMode::Geometric
                        } else {
                            DurationMode::Normal
                        },
                        ..Default::default()
                    },
                )
                .unwrap();
        }
        let emissions = model
            .emission_log_probabilities(
                &observation,
                &segmentation,
                1.0,
                liblrhsmm_rs::emission::EmissionPruningOptions::None,
            )
            .unwrap();
        if geometric {
            let path = model
                .viterbi_geometric(
                    &segmentation,
                    &emissions,
                    GeometricSettings {
                        pruning_slope: 0.5,
                        ..Default::default()
                    },
                )
                .unwrap();
            assert!(path.log_probability.is_finite());
            assert_eq!(path.boundaries.last(), Some(&observation.frames));
        } else {
            let path = model
                .viterbi_hsmm(&segmentation, &emissions, HsmmSettings::default())
                .unwrap();
            assert!(path.log_probability.is_finite());
            assert_eq!(path.occurrences.last().unwrap().end, observation.frames);
        }
    }
}

#[test]
fn double_fixed_duration_mean_uses_residual_variance_and_failed_update_is_atomic() {
    let mut duration = DurationParameters::<f64> {
        fixed_mean: 4,
        ..Default::default()
    };
    duration
        .update(
            &DurationSufficientStatistics {
                value_sum: 6.0,
                squared_sum: 20.0,
                occupancy: 2.0,
            },
            UpdateSettings::default(),
        )
        .unwrap();
    assert_eq!((duration.mean, duration.variance), (4.0, 2.0));

    let (mut model, observation, segmentation) = estimation_sample::sample_with_precision::<f64>();
    let mut statistics = ModelSufficientStatistics::from_model(&model).unwrap();
    statistics
        .estimate_hsmm(&model, &observation, &segmentation, HsmmSettings::default())
        .unwrap();
    statistics.durations.last_mut().unwrap().value_sum = 0.0;
    statistics.durations.last_mut().unwrap().occupancy = 1.0;
    let before = model.clone();
    assert!(
        model
            .update(
                &statistics,
                UpdateSettings {
                    duration_mode: DurationMode::Geometric,
                    ..Default::default()
                }
            )
            .is_err()
    );
    assert_eq!(model, before);
}

#[path = "common/estimation.rs"]
#[allow(dead_code)]
mod estimation_sample;
#[path = "common/hsmm.rs"]
#[allow(dead_code)]
mod hsmm_sample;
#[path = "common/mixtures.rs"]
#[allow(dead_code)]
mod mixture_sample;

#[derive(Default)]
struct Comparison {
    count: usize,
    maximum: f64,
}
impl Comparison {
    fn check(&mut self, actual: f64, expected: f64) {
        assert!(actual.is_finite() && expected.is_finite());
        self.maximum = self
            .maximum
            .max((actual - expected).abs() / expected.abs().max(1.0));
        self.count += 1;
    }
    fn finish(&self, label: &str) {
        println!(
            "{label}: {} values; maximum normalized error {}",
            self.count, self.maximum
        );
        assert!(self.count > 0);
        assert!(self.maximum <= 2e-14, "{label}: {}", self.maximum);
    }
}

#[test]
fn double_geometric_state_and_float_transition_posteriors_match_c() {
    let mut comparison = Comparison::default();
    let mut transitions = 0;
    for line in include_str!("fixtures/c-double-geometric-posteriors.txt")
        .lines()
        .filter(|line| line.starts_with("occupancy "))
    {
        let fields: Vec<_> = line.split_whitespace().collect();
        let states: usize = fields[1].parse().unwrap();
        let frames: usize = fields[2].parse().unwrap();
        let index: usize = fields[5].parse().unwrap();
        let model = ModelParameters::<f64> {
            streams: vec![],
            durations: (0..states)
                .map(|state| DurationParameters {
                    mean: [0.5, 2.75, 5.0][state],
                    variance: 1.0,
                    ..Default::default()
                })
                .collect(),
        };
        let mut segmentation = SegmentationData::<f64>::new(0, states);
        segmentation.duration_states = (0..states as i32).collect();
        let emissions = LogEmissionTable::from_log_values(
            frames,
            states,
            (0..frames)
                .flat_map(|frame| {
                    (0..states).map(move |state| {
                        let residual = frame as f64 - state as f64 * 4.0;
                        -residual * residual * 0.25
                    })
                })
                .collect(),
        )
        .unwrap();
        let options = GeometricSettings {
            temperature: fields[3].parse().unwrap(),
            pruning_slope: fields[4].parse().unwrap(),
        };
        let forward = model
            .forward_geometric(&segmentation, &emissions, options)
            .unwrap();
        let backward = model
            .backward_geometric(&segmentation, &emissions, options)
            .unwrap();
        let total = backward.backward_log_probability(&emissions).unwrap();
        let occupancy = forward
            .geometric_occupancies(&backward, total, options.pruning_slope)
            .unwrap();
        let posterior = model
            .geometric_transition_occupancies(
                &segmentation,
                &emissions,
                &forward,
                &backward,
                total,
                options,
            )
            .unwrap();
        comparison.check(occupancy.values()[index], fields[6].parse().unwrap());
        let present: usize = fields[7].parse().unwrap();
        match posterior.values()[index] {
            Some(value) => {
                assert_eq!(present, 1);
                assert_eq!(
                    value.stay.to_bits(),
                    fields[8].parse::<f32>().unwrap().to_bits()
                );
                assert_eq!(
                    value.advance.to_bits(),
                    fields[9].parse::<f32>().unwrap().to_bits()
                );
                transitions += 2;
            }
            None => assert_eq!(present, 0),
        }
    }
    assert_eq!(comparison.count, 146);
    assert!(transitions > 0);
    comparison.finish("double geometric state occupancies");
}

#[test]
fn double_duration_posteriors_keep_float_slices_and_double_state_marginals() {
    let mut comparison = Comparison::default();
    let mut probabilities = 0;
    for (fixture, fractional) in [
        (
            include_str!("fixtures/c-double-duration-posteriors.txt"),
            false,
        ),
        (
            include_str!("fixtures/c-double-duration-posteriors-fractional.txt"),
            true,
        ),
    ] {
        for scenario in 0..4 {
            let (model, segmentation, emissions, mut options) =
                hsmm_sample::sample_with_precision::<f64>(scenario);
            if fractional {
                options.state_radius = [0.25, 0.5, 1.75, 2.5][scenario];
            }
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
                let frame = fields[1].parse().unwrap();
                let state = fields[2].parse().unwrap();
                let minimum: usize = fields[3].parse().unwrap();
                let count: usize = fields[4].parse().unwrap();
                if count == 0 {
                    assert!(durations.get(frame, state).is_none());
                } else {
                    let actual = durations.get(frame, state).unwrap();
                    assert_eq!(actual.minimum, minimum);
                    let expected: Vec<f32> = fields[5..5 + count]
                        .iter()
                        .map(|value| value.parse().unwrap())
                        .collect();
                    assert_eq!(actual.probabilities, expected, "{line}");
                    probabilities += count;
                }
                comparison.check(
                    states.get(frame, state).unwrap(),
                    fields[5 + count].parse().unwrap(),
                );
            }
        }
    }
    assert_eq!(comparison.count, 384);
    assert!(probabilities > 0);
    println!("double duration slices: {probabilities} exact binary32 probabilities");
    comparison.finish("double HSMM state marginals");
}

#[test]
fn double_mixture_posteriors_preserve_source_float_rounding() {
    let (model, observation, segmentation, occupancy) =
        mixture_sample::sample_with_precision::<f64>();
    let mut count = 0;
    for (fixture, fractional) in [
        (
            include_str!("fixtures/c-double-mixture-posteriors.txt"),
            false,
        ),
        (
            include_str!("fixtures/c-double-mixture-posteriors-fractional.txt"),
            true,
        ),
    ] {
        for temperature in [0.5, 1.0] {
            for scenario in 0..if fractional { 4 } else { 2 } {
                let mode = if fractional {
                    MixtureOccupancySettings::Hsmm {
                        state_radius: [0.25, 0.5, 1.75, 2.5][scenario],
                    }
                } else if scenario == 0 {
                    MixtureOccupancySettings::Hsmm { state_radius: 1.0 }
                } else {
                    MixtureOccupancySettings::Geometric { pruning_slope: 0.5 }
                };
                let result = model
                    .mixture_occupancies(
                        &observation,
                        &segmentation,
                        &occupancy,
                        0,
                        temperature,
                        mode,
                    )
                    .unwrap();
                for line in fixture.lines() {
                    let fields: Vec<_> = line.split_whitespace().collect();
                    if fields[0].parse::<usize>().unwrap() != scenario
                        || fields[1].parse::<f64>().unwrap() != temperature
                    {
                        continue;
                    }
                    let frame = fields[2].parse().unwrap();
                    let state = fields[3].parse().unwrap();
                    let components: usize = fields[4].parse().unwrap();
                    if components == 0 {
                        assert!(result.get(frame, state).is_none());
                    } else {
                        let expected: Vec<f32> = fields[5..]
                            .iter()
                            .map(|value| value.parse().unwrap())
                            .collect();
                        assert_eq!(expected.len(), components);
                        assert_eq!(result.get(frame, state).unwrap(), expected, "{line}");
                        count += components;
                    }
                }
            }
        }
    }
    assert!(count > 0);
    println!("double mixture slices: {count} exact binary32 probabilities");
}

#[test]
fn double_expectation_likelihoods_and_all_moments_match_corrected_c() {
    let (model, observation, segmentation) = estimation_sample::sample_with_precision::<f64>();
    let mut comparison = Comparison::default();
    for (fixture, fractional) in [
        (include_str!("fixtures/c-double-estimation.txt"), false),
        (
            include_str!("fixtures/c-double-estimation-fractional.txt"),
            true,
        ),
    ] {
        for mode in 0..if fractional { 4 } else { 2 } {
            for temperature in 1..=2 {
                let mut statistics = ModelSufficientStatistics::from_model(&model).unwrap();
                let value = temperature as f64 * 0.5;
                let total = if fractional || mode == 0 {
                    statistics
                        .estimate_hsmm(
                            &model,
                            &observation,
                            &segmentation,
                            HsmmSettings {
                                temperature: value,
                                state_radius: if fractional {
                                    [0.25, 0.5, 1.75, 2.5][mode]
                                } else {
                                    1.0
                                },
                                ..Default::default()
                            },
                        )
                        .unwrap()
                } else {
                    statistics
                        .estimate_geometric(
                            &model,
                            &observation,
                            &segmentation,
                            GeometricSettings {
                                temperature: value,
                                pruning_slope: 0.5,
                            },
                        )
                        .unwrap()
                };
                for line in fixture.lines() {
                    let mut fields = line.split_whitespace();
                    let kind = fields.next().unwrap();
                    if fields.next().unwrap().parse::<usize>().unwrap() != mode
                        || fields.next().unwrap().parse::<usize>().unwrap() != temperature
                    {
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
                        _ => panic!("unknown source record"),
                    };
                    let expected: Vec<f64> = fields.map(|value| value.parse().unwrap()).collect();
                    assert_eq!(actual.len(), expected.len());
                    for (actual, expected) in actual.into_iter().zip(expected) {
                        comparison.check(actual, expected);
                    }
                }
            }
        }
    }
    comparison.finish("double expectation likelihoods and statistics");
}

#[test]
fn double_maximization_matches_c() {
    let mut comparison = Comparison::default();
    for line in include_str!("fixtures/c-double-update.txt").lines() {
        let mut fields = line.split_whitespace();
        let kind = fields.next().unwrap();
        let scenario: usize = fields.next().unwrap().parse().unwrap();
        let actual = if kind == "duration" {
            let mut duration = DurationParameters::<f64> {
                mean: 7.0,
                variance: 3.0,
                variance_floor: if scenario == 2 { 2.5 } else { 0.0 },
                ..Default::default()
            };
            let statistics = DurationSufficientStatistics {
                value_sum: if scenario == 3 { 1.5 } else { 9.0 },
                squared_sum: 29.0,
                occupancy: if scenario == 4 { 1e-8 } else { 3.0 },
            };
            duration
                .update(
                    &statistics,
                    UpdateSettings {
                        duration_mode: if scenario == 3 {
                            DurationMode::Geometric
                        } else {
                            DurationMode::Normal
                        },
                        ..Default::default()
                    },
                )
                .unwrap();
            vec![duration.mean, duration.variance]
        } else {
            assert_eq!(kind, "mixture");
            let component: usize = fields.next().unwrap().parse().unwrap();
            let mut gaussian =
                GaussianMixtureParameters::<f64>::new(if scenario == 0 { 1 } else { 2 }, 2)
                    .unwrap();
            gaussian.weights.fill(0.5);
            gaussian.means.fill(-7.0);
            gaussian.variances.fill(8.0);
            let mut statistics = GaussianSufficientStatistics::from_gaussian(&gaussian).unwrap();
            statistics.state_occupancy = if scenario == 3 { 1e-8 } else { 4.0 };
            for k in 0..gaussian.weights.len() {
                statistics.component_occupancies[k] = if scenario == 2 && k == 1 {
                    1e-8
                } else {
                    2.0 + k as f64
                };
                statistics.weighted_sums[k * 2..k * 2 + 2].copy_from_slice(&[-3.5, 6.0]);
                statistics.weighted_squared_sums[k * 2..k * 2 + 2].copy_from_slice(&[9.25, 12.0]);
                gaussian.variance_floors[k * 2 + 1] = 0.75;
            }
            gaussian
                .update(&statistics, UpdateSettings::default())
                .unwrap();
            let index = component * 2;
            vec![
                gaussian.weights[component],
                gaussian.means[index],
                gaussian.variances[index],
                gaussian.means[index + 1],
                gaussian.variances[index + 1],
            ]
        };
        let expected: Vec<f64> = fields.map(|value| value.parse().unwrap()).collect();
        assert_eq!(actual.len(), expected.len());
        for (actual, expected) in actual.into_iter().zip(expected) {
            comparison.check(actual, expected);
        }
    }
    comparison.finish("double maximization");
}

#[test]
fn double_aligned_initialization_matches_c_with_empty_and_shared_states() {
    let model = ModelParameters::<f64>::new(
        &[
            StreamLayout {
                emissions: 2,
                components: 1,
                dimensions: 2,
            },
            StreamLayout {
                emissions: 2,
                components: 1,
                dimensions: 1,
            },
        ],
        2,
    )
    .unwrap();
    let mut observation = ObservationData::<f64>::new(6, &[2, 1]).unwrap();
    for (stream, data) in observation.streams.iter_mut().enumerate() {
        for frame in 0..6 {
            for dimension in 0..data.dimensions {
                data.values[frame * data.dimensions + dimension] =
                    (frame as f64 - 2.0) * 0.25 + dimension as f64 + stream as f64 * 3.0;
            }
        }
    }
    let mut segmentation = SegmentationData::<f64>::new(2, 3);
    segmentation.boundaries = vec![2, 2, 6];
    segmentation.duration_states = vec![0, 1, 0];
    segmentation.output_states = vec![vec![0, 1, 0], vec![1, 0, 1]];
    let mut statistics = ModelSufficientStatistics::from_model(&model).unwrap();
    for _ in 0..2 {
        statistics
            .collect_initialization(&model, &observation, &segmentation)
            .unwrap();
    }
    let mut comparison = Comparison::default();
    for line in include_str!("fixtures/c-double-initialization.txt").lines() {
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
        let expected: Vec<f64> = fields.map(|value| value.parse().unwrap()).collect();
        assert_eq!(actual.len(), expected.len());
        for (actual, expected) in actual.into_iter().zip(expected) {
            comparison.check(actual, expected);
        }
    }
    comparison.finish("double aligned initialization");
    statistics.clear();
    assert_eq!(
        statistics,
        ModelSufficientStatistics::from_model(&model).unwrap()
    );
}
