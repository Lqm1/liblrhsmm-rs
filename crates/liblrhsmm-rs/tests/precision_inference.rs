use liblrhsmm_rs::data::SegmentationData;
use liblrhsmm_rs::emission::{EmissionPruningOptions, LogEmissionTable};
use liblrhsmm_rs::geometric::GeometricSettings;
use liblrhsmm_rs::model::{DurationParameters, ModelParameters};
use liblrhsmm_rs::preparation::PreparationSettings;

#[path = "common/emissions.rs"]
#[allow(dead_code)]
mod emission_sample;
#[path = "common/hsmm.rs"]
#[allow(dead_code)]
mod hsmm_sample;

fn compare(actual: f64, expected: f64, maximum: &mut f64) {
    assert!(actual.is_finite() && expected.is_finite());
    *maximum = maximum.max((actual - expected).abs() / expected.abs().max(1.0));
}

fn finish(label: &str, count: usize, maximum: f64) {
    println!("{label}: {count} comparisons; maximum normalized error {maximum}");
    assert!(maximum <= 2e-14, "{label}: normalized error {maximum}");
}

#[test]
fn double_geometric_scores_totals_and_paths_match_c() {
    let mut maximum = 0.0;
    let mut count = 0;
    for line in include_str!("../../../tests/fixtures/c-double-geometric.txt").lines() {
        let fields: Vec<_> = line.split_whitespace().collect();
        if fields[0] == "math" {
            let x: f64 = fields[1].parse().unwrap();
            for (actual, text) in [
                liblrhsmm_rs::numerics::log_add_f64(x * 0.125, -x),
                liblrhsmm_rs::numerics::negative_exponential_f64(-x),
                liblrhsmm_rs::numerics::exponential_f64(-x),
            ]
            .into_iter()
            .zip(&fields[2..])
            {
                compare(actual, text.parse().unwrap(), &mut maximum);
                count += 1;
            }
            continue;
        }
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
        let values = (0..frames)
            .flat_map(|frame| {
                (0..states).map(move |state| {
                    let residual = frame as f64 - state as f64 * 4.0;
                    -residual * residual * 0.25
                })
            })
            .collect();
        let emissions = LogEmissionTable::from_log_values(frames, states, values).unwrap();
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
        let path = model
            .viterbi_geometric(&segmentation, &emissions, options)
            .unwrap();
        if fields[0] == "boundary" {
            assert_eq!(path.boundaries[index], fields[6].parse::<usize>().unwrap());
            continue;
        }
        let actual = match fields[0] {
            "scores" => [
                forward.values()[index],
                backward.values()[index],
                path.scores.values()[index],
            ],
            "total" => [
                forward.total_through_state(&backward, index).unwrap(),
                forward.total_log_probability(),
                backward.backward_log_probability(&emissions).unwrap(),
            ],
            _ => panic!("unknown source record"),
        };
        for (actual, text) in actual.into_iter().zip(&fields[6..]) {
            compare(actual, text.parse().unwrap(), &mut maximum);
            count += 1;
        }
    }
    assert_eq!(count, 672);
    finish("double geometric", count, maximum);
}

fn check_hsmm(fixture: &str, fractional: bool) {
    let mut maximum = 0.0;
    let mut count = 0;
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
        let path = model
            .viterbi_hsmm(&segmentation, &emissions, options)
            .unwrap();
        for cache_length in [0, 3, 300] {
            let prepared = model
                .prepare(PreparationSettings {
                    duration_cache_length: cache_length,
                    duration_weight: options.duration_weight,
                    temperature: options.temperature,
                })
                .unwrap();
            let cached_forward = prepared
                .forward_hsmm(&segmentation, &emissions, options)
                .unwrap();
            let cached_backward = prepared
                .backward_hsmm(&segmentation, &emissions, options)
                .unwrap();
            let cached_path = prepared
                .viterbi_hsmm(&segmentation, &emissions, options)
                .unwrap();
            assert_eq!(cached_path, path);
            let mut paths = 0;
            for line in fixture.lines() {
                let fields: Vec<_> = line.split_whitespace().collect();
                if fields[1].parse::<usize>().unwrap() != scenario {
                    continue;
                }
                let index: usize = fields[2].parse().unwrap();
                if fields[0] == "path" {
                    assert_eq!(
                        path.occurrences[index].end,
                        fields[3].parse::<usize>().unwrap()
                    );
                    assert_eq!(
                        path.occurrences[index].state,
                        fields[4].parse::<usize>().unwrap()
                    );
                    paths += 1;
                    continue;
                }
                assert_eq!(fields[0], "score");
                for values in [
                    [
                        forward.begin.values()[index],
                        forward.end.values()[index],
                        backward.begin.values()[index],
                        backward.end.values()[index],
                    ],
                    [
                        cached_forward.begin.values()[index],
                        cached_forward.end.values()[index],
                        cached_backward.begin.values()[index],
                        cached_backward.end.values()[index],
                    ],
                ] {
                    for (actual, text) in values.into_iter().zip(&fields[3..]) {
                        compare(actual, text.parse().unwrap(), &mut maximum);
                        count += 1;
                    }
                }
            }
            assert_eq!(paths, path.occurrences.len());
        }
    }
    assert_eq!(count, 4608);
    finish(
        if fractional {
            "double fractional HSMM"
        } else {
            "double HSMM"
        },
        count,
        maximum,
    );
}

#[test]
fn double_hsmm_scores_paths_and_prepared_calls_match_c() {
    check_hsmm(
        include_str!("../../../tests/fixtures/c-double-hsmm.txt"),
        false,
    );
}

#[test]
fn double_fractional_hsmm_matches_c() {
    check_hsmm(
        include_str!("../../../tests/fixtures/c-double-hsmm-fractional.txt"),
        true,
    );
}

#[test]
fn double_duration_caches_and_fallback_match_c() {
    let duration = DurationParameters::<f64> {
        mean: 4.25,
        variance: 2.5,
        minimum: 2,
        maximum: 6,
        ..Default::default()
    };
    let mut maximum = 0.0;
    for line in include_str!("../../../tests/fixtures/c-double-preparation.txt").lines() {
        let fields: Vec<_> = line.split_whitespace().collect();
        let length = fields[1].parse().unwrap();
        let frames = fields[3].parse().unwrap();
        let prepared = duration
            .prepare(PreparationSettings {
                duration_cache_length: length,
                duration_weight: 0.75,
                temperature: fields[2].parse::<f64>().unwrap() * 0.5,
            })
            .unwrap();
        assert_eq!(prepared.cached_log_probabilities().len(), length);
        compare(
            prepared.log_probability(frames).unwrap(),
            fields[4].parse().unwrap(),
            &mut maximum,
        );
        compare(
            prepared.probability(frames).unwrap(),
            fields[5].parse().unwrap(),
            &mut maximum,
        );
    }
    finish("double duration preparation", 144, maximum);
}

#[test]
fn double_multistream_and_fractional_emissions_match_c() {
    let (model, observation, segmentation) = emission_sample::sample_with_precision::<f64>();
    let prepared = model.prepare(PreparationSettings::default()).unwrap();
    let mut maximum = 0.0;
    let mut count = 0;
    for (fixture, fractional) in [
        (
            include_str!("../../../tests/fixtures/c-double-emissions.txt"),
            false,
        ),
        (
            include_str!("../../../tests/fixtures/c-double-emissions-fractional.txt"),
            true,
        ),
    ] {
        for line in fixture.lines() {
            let fields: Vec<_> = line.split_whitespace().collect();
            let scenario: usize = fields[0].parse().unwrap();
            let index: usize = fields[1].parse().unwrap();
            let pruning = if fractional {
                EmissionPruningOptions::AroundBoundaries {
                    radius: [0.25, 0.5, 1.75, 2.5][scenario],
                }
            } else if scenario == 0 {
                EmissionPruningOptions::AroundBoundaries { radius: 1.0 }
            } else {
                EmissionPruningOptions::Linear { slope: 0.34 }
            };
            let table = model
                .emission_log_probabilities(&observation, &segmentation, 0.75, pruning)
                .unwrap();
            let cached = prepared
                .emission_log_probabilities(&observation, &segmentation, 0.75, pruning)
                .unwrap();
            assert_eq!(table, cached);
            compare(
                table.values()[index],
                fields[2].parse().unwrap(),
                &mut maximum,
            );
            count += 1;
        }
    }
    assert_eq!(count, 216);
    finish("double multistream emissions", count, maximum);
}

#[test]
fn double_preparation_and_pruning_do_not_narrow_configuration() {
    let (model, segmentation, emissions, mut options) =
        hsmm_sample::sample_with_precision::<f64>(0);
    let temperature = 1.0 + f64::EPSILON;
    let prepared = model
        .prepare(PreparationSettings {
            temperature,
            duration_weight: options.duration_weight,
            duration_cache_length: 3,
        })
        .unwrap();
    assert_eq!(prepared.options().temperature, temperature);
    assert!(
        prepared
            .forward_hsmm(&segmentation, &emissions, options)
            .is_err()
    );
    options.temperature = temperature;
    assert!(
        prepared
            .forward_hsmm(&segmentation, &emissions, options)
            .is_ok()
    );
    options.state_radius = f64::NAN;
    assert!(
        model
            .forward_hsmm(&segmentation, &emissions, options)
            .is_err()
    );

    let (model, observation, segmentation) = emission_sample::sample_with_precision::<f64>();
    let before_integer = model
        .emission_log_probabilities(
            &observation,
            &segmentation,
            0.75,
            EmissionPruningOptions::AroundBoundaries {
                radius: 1.0 - f64::EPSILON,
            },
        )
        .unwrap();
    let at_integer = model
        .emission_log_probabilities(
            &observation,
            &segmentation,
            0.75,
            EmissionPruningOptions::AroundBoundaries { radius: 1.0 },
        )
        .unwrap();
    // At frame five, segment one has upper bound 2-epsilon or exactly 2.
    assert_eq!(before_integer.get(5, 2), Some(-1e15));
    assert!(at_integer.get(5, 2).unwrap() > -1e15);
    let mut uninitialized = model.clone();
    uninitialized.durations[1].variance = 0.0;
    assert!(
        uninitialized
            .prepare(PreparationSettings::default())
            .is_err()
    );
    let selected = uninitialized
        .prepare_selected_durations(PreparationSettings::default(), &[0, 2, 2])
        .unwrap();
    assert!(selected.duration(0).is_some());
    assert!(selected.duration(1).is_none());
    assert!(selected.duration(2).is_some());
}

#[test]
fn every_double_hsmm_viterbi_score_matches_corrected_c_output() {
    let mut maximum = 0.0;
    let mut count = 0;
    for (fixture, fractional) in [
        (
            include_str!("../../../tests/fixtures/c-double-hsmm-viterbi.txt"),
            false,
        ),
        (
            include_str!("../../../tests/fixtures/c-double-hsmm-viterbi-fractional.txt"),
            true,
        ),
    ] {
        for scenario in 0..4 {
            let (model, segmentation, emissions, mut options) =
                hsmm_sample::sample_with_precision::<f64>(scenario);
            if fractional {
                options.state_radius = [0.25, 0.5, 1.75, 2.5][scenario];
            }
            let alignment = model
                .viterbi_hsmm(&segmentation, &emissions, options)
                .unwrap();
            let mut entries = 0;
            for line in fixture.lines().filter(|line| line.starts_with("vscore ")) {
                let fields: Vec<_> = line.split_whitespace().collect();
                assert_eq!(fields[1].parse::<usize>().unwrap(), usize::from(fractional));
                if fields[2].parse::<usize>().unwrap() != scenario {
                    continue;
                }
                let index: usize = fields[3].parse().unwrap();
                compare(
                    alignment.scores.begin.values()[index],
                    fields[4].parse().unwrap(),
                    &mut maximum,
                );
                compare(
                    alignment.scores.end.values()[index],
                    fields[5].parse().unwrap(),
                    &mut maximum,
                );
                entries += 1;
                count += 2;
            }
            assert_eq!(entries, 48);
        }
    }
    assert_eq!(count, 768);
    finish("double HSMM Viterbi scores", count, maximum);
}
