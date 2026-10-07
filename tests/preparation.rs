use liblrhsmm_rs::{Duration, EmissionPruning, HsmmOptions, Model, PreparationOptions};

#[path = "common/emissions.rs"]
mod emissions;
#[path = "common/hsmm.rs"]
mod hsmm;

#[test]
fn selected_durations_skip_unused_uninitialized_states_and_retain_inference() {
    let (mut model, segmentation, emissions, options) = hsmm::sample(0);
    let preparation = PreparationOptions {
        temperature: options.temperature,
        duration_weight: options.duration_weight,
        ..PreparationOptions::default()
    };
    let expected = model
        .viterbi_hsmm(&segmentation, &emissions, options)
        .unwrap();
    model.durations.push(Duration {
        mean: 0.0,
        variance: 0.0,
        ..Duration::default()
    });
    assert!(model.prepare(PreparationOptions::default()).is_err());
    let selected = model
        .prepare_selected_durations(preparation, &[0, 1, 2, 3, 0])
        .unwrap();
    assert!(selected.duration(4).is_none());
    assert_eq!(
        selected
            .viterbi_hsmm(&segmentation, &emissions, options)
            .unwrap(),
        expected
    );
    let emissions_only = model.prepare_selected_durations(preparation, &[]).unwrap();
    assert!(emissions_only.duration(0).is_none());
    assert!(
        emissions_only
            .viterbi_hsmm(&segmentation, &emissions, options)
            .is_err()
    );
    assert!(
        model
            .prepare_selected_durations(PreparationOptions::default(), &[4])
            .is_err()
    );
    assert!(
        model
            .prepare_selected_durations(PreparationOptions::default(), &[5])
            .is_err()
    );
}

#[test]
fn bundled_historical_model_prepares_all_initialized_durations() {
    let model =
        Model::read_from(include_bytes!("fixtures/cmu-arctic-all-speakers.hsmm").as_slice())
            .unwrap();
    let selected: Vec<_> = model
        .durations
        .iter()
        .enumerate()
        .filter_map(|(index, duration)| (duration.variance > 0.0).then_some(index))
        .collect();
    assert_eq!(selected.len(), 205);
    let prepared = model
        .prepare_selected_durations(PreparationOptions::default(), &selected)
        .unwrap();
    assert_eq!(model.durations.len(), 215);
    for index in 0..215 {
        assert_eq!(
            prepared.duration(index).is_some(),
            selected.contains(&index)
        );
    }
    assert!(model.prepare(PreparationOptions::default()).is_err());
}

#[test]
fn prepared_duration_cache_and_fallback_match_c() {
    let duration = Duration {
        mean: 4.25,
        variance: 2.5,
        minimum: 2,
        maximum: 6,
        ..Duration::default()
    };
    let mut reference = include_str!("fixtures/c-preparation.txt").lines();
    assert_eq!(PreparationOptions::default().duration_cache_length, 300);
    for length in [0, 3, 8, 300] {
        for temperature in 1..=2 {
            let options = PreparationOptions {
                duration_cache_length: length,
                duration_weight: 0.75,
                temperature: temperature as f32 * 0.5,
            };
            let prepared = duration.prepare(options).unwrap();
            assert_eq!(prepared.options(), options);
            assert_eq!(prepared.cached_log_probabilities().len(), length);
            for frame in 0..=8 {
                let line = reference.next().unwrap();
                let fields: Vec<_> = line.split_whitespace().collect();
                assert_eq!(fields[0], "D");
                assert_eq!(fields[1].parse::<usize>().unwrap(), length);
                assert_eq!(fields[2].parse::<usize>().unwrap(), temperature);
                assert_eq!(fields[3].parse::<usize>().unwrap(), frame);
                assert_eq!(
                    prepared.log_probability(frame).unwrap(),
                    fields[4].parse::<f32>().unwrap(),
                    "{line}"
                );
                assert_eq!(
                    prepared.probability(frame).unwrap(),
                    fields[5].parse::<f32>().unwrap() as f64,
                    "{line}"
                );
                assert_eq!(
                    prepared.log_probability(frame).unwrap(),
                    duration
                        .annealed_log_probability(frame, 0.75, options.temperature)
                        .unwrap()
                );
            }
        }
    }
    assert!(reference.next().is_none());
}

#[test]
fn whole_model_preparation_reuses_emissions_and_matches_c_tables() {
    let (model, observation, segmentation) = emissions::sample();
    let prepared = model.prepare(PreparationOptions::default()).unwrap();
    assert!(std::ptr::eq(prepared.model(), &model));
    assert_eq!(prepared.options(), PreparationOptions::default());
    assert!(prepared.duration(3).is_none());
    assert!(prepared.emission(2, 0).is_none());
    assert!(prepared.emission(0, 3).is_none());
    assert_eq!(
        prepared
            .emission(0, 0)
            .unwrap()
            .log_probability(&[0.0, 0.5], 0.75)
            .unwrap(),
        model.streams[0].mixtures[0]
            .prepare()
            .unwrap()
            .log_probability(&[0.0, 0.5], 0.75)
            .unwrap()
    );
    let tables = [
        prepared
            .emission_log_probabilities(
                &observation,
                &segmentation,
                0.75,
                EmissionPruning::AroundBoundaries { radius: 1.0 },
            )
            .unwrap(),
        prepared
            .emission_log_probabilities(
                &observation,
                &segmentation,
                0.75,
                EmissionPruning::Linear { slope: 0.34 },
            )
            .unwrap(),
    ];
    for line in include_str!("fixtures/c-emission-tables.txt").lines() {
        let fields: Vec<_> = line.split_whitespace().collect();
        let table: usize = fields[0].parse().unwrap();
        let index: usize = fields[1].parse().unwrap();
        assert_eq!(
            tables[table].values()[index],
            fields[2].parse::<f32>().unwrap(),
            "{line}"
        );
    }
    assert_eq!(
        prepared
            .emission_probabilities(
                &observation,
                &segmentation,
                0.75,
                EmissionPruning::AroundBoundaries { radius: 1.0 }
            )
            .unwrap(),
        tables[0].probabilities()
    );
    // An independent cache can coexist without mutating this preparation.
    let other = model
        .prepare(PreparationOptions {
            temperature: 0.5,
            ..PreparationOptions::default()
        })
        .unwrap();
    assert_eq!(
        other.duration(0).unwrap().log_probability(4).unwrap() * 2.0,
        prepared.duration(0).unwrap().log_probability(4).unwrap()
    );
}

#[test]
fn prepared_hsmm_inference_matches_c_with_short_and_empty_caches() {
    for scenario in 0..4 {
        let (model, segmentation, emissions, options) = hsmm::sample(scenario);
        let expected_forward = model
            .forward_hsmm(&segmentation, &emissions, options)
            .unwrap();
        let expected_backward = model
            .backward_hsmm(&segmentation, &emissions, options)
            .unwrap();
        let expected_path = model
            .viterbi_hsmm(&segmentation, &emissions, options)
            .unwrap();
        for cache_length in [0, 2, 300] {
            let prepared = model
                .prepare(PreparationOptions {
                    temperature: options.temperature,
                    duration_weight: options.duration_weight,
                    duration_cache_length: cache_length,
                })
                .unwrap();
            let forward = prepared
                .forward_hsmm(&segmentation, &emissions, options)
                .unwrap();
            let backward = prepared
                .backward_hsmm(&segmentation, &emissions, options)
                .unwrap();
            assert_eq!(forward, expected_forward);
            assert_eq!(backward, expected_backward);
            assert_eq!(
                prepared
                    .viterbi_hsmm(&segmentation, &emissions, options)
                    .unwrap(),
                expected_path
            );
            for line in include_str!("fixtures/c-hsmm.txt").lines() {
                let fields: Vec<_> = line.split_whitespace().collect();
                if fields[0] != "score" || fields[1].parse::<usize>().unwrap() != scenario {
                    continue;
                }
                let index: usize = fields[2].parse().unwrap();
                assert_eq!(
                    forward.begin.values()[index],
                    fields[3].parse::<f32>().unwrap()
                );
                assert_eq!(
                    forward.end.values()[index],
                    fields[4].parse::<f32>().unwrap()
                );
                assert_eq!(
                    backward.begin.values()[index],
                    fields[5].parse::<f32>().unwrap()
                );
                assert_eq!(
                    backward.end.values()[index],
                    fields[6].parse::<f32>().unwrap()
                );
            }
            let changed = HsmmOptions {
                temperature: options.temperature * 0.5,
                ..options
            };
            assert!(
                prepared
                    .forward_hsmm(&segmentation, &emissions, changed)
                    .is_err()
            );
            assert!(
                prepared
                    .backward_hsmm(&segmentation, &emissions, changed)
                    .is_err()
            );
            assert!(
                prepared
                    .viterbi_hsmm(&segmentation, &emissions, changed)
                    .is_err()
            );
        }
    }
}

#[test]
fn preparation_rejects_invalid_inputs_and_retains_zero_temperature_constraints() {
    let (mut model, observation, mut segmentation) = emissions::sample();
    let prepared = model.prepare(PreparationOptions::default()).unwrap();
    segmentation.output_states[0][0] = 999;
    assert!(
        prepared
            .emission_log_probabilities(&observation, &segmentation, 1.0, EmissionPruning::None)
            .is_err()
    );
    model.durations[0].variance = 0.0;
    assert!(model.prepare(PreparationOptions::default()).is_err());
    assert!(
        Model::default()
            .prepare(PreparationOptions::default())
            .is_ok()
    );
    let duration = Duration {
        mean: 3.0,
        variance: 1.0,
        minimum: 2,
        ..Duration::default()
    };
    for options in [
        PreparationOptions {
            duration_cache_length: usize::MAX,
            ..PreparationOptions::default()
        },
        PreparationOptions {
            temperature: f32::NAN,
            ..PreparationOptions::default()
        },
        PreparationOptions {
            duration_weight: -1.0,
            ..PreparationOptions::default()
        },
    ] {
        assert!(duration.prepare(options).is_err());
    }
    let prepared = duration
        .prepare(PreparationOptions {
            temperature: 0.0,
            duration_cache_length: 3,
            ..PreparationOptions::default()
        })
        .unwrap();
    assert_eq!(
        prepared.log_probability(1).unwrap(),
        liblrhsmm_rs::numerics::LOG_ZERO
    );
    assert_eq!(prepared.log_probability(2).unwrap(), 0.0);
    assert_eq!(prepared.log_probability(8).unwrap(), 0.0);
    assert!(prepared.log_probability(usize::MAX).is_err());
    // C compares floating frame counts for positive temperatures. Retain rounding
    // at the binary32 integer precision boundary, both in and beyond the cache.
    let large = Duration {
        mean: 16_777_216.0,
        minimum: 16_777_217,
        ..duration
    };
    let prepared = large
        .prepare(PreparationOptions {
            duration_cache_length: 0,
            ..PreparationOptions::default()
        })
        .unwrap();
    assert_eq!(
        prepared.log_probability(16_777_216).unwrap(),
        large
            .annealed_log_probability(16_777_216, 1.0, 1.0)
            .unwrap()
    );
}
