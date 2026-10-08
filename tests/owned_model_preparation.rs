use liblrhsmm_rs::emission::EmissionPruningOptions;
use liblrhsmm_rs::model::{
    DurationParameters, GaussianMixtureParameters, ModelParameters, StreamParameters,
};
use liblrhsmm_rs::preparation::PreparationSettings;
use liblrhsmm_rs::scalar::Scalar;

#[path = "common/emissions.rs"]
#[allow(dead_code)]
mod emissions;

fn bits<T: Scalar>(value: T) -> u64 {
    let value: f64 = value.into();
    value.to_bits()
}

fn stream_snapshot<T: Scalar>() {
    let mut source = StreamParameters::<T> {
        weight: T::from_f64(0.75),
        mixtures: Vec::new(),
    };
    for (components, dimensions) in [(1, 1), (2, 3), (5, 2)] {
        let mut distribution = GaussianMixtureParameters::new(components, dimensions).unwrap();
        for (index, mean) in distribution.means.iter_mut().enumerate() {
            *mean = T::from_f64(index as f64 * 0.25 - 1.5);
        }
        for (index, variance) in distribution.variances.iter_mut().enumerate() {
            *variance = T::from_f64(0.5 + index as f64 * 0.1);
        }
        distribution.variance_floors.fill(T::from_f64(-0.0));
        source.mixtures.push(distribution);
    }
    let original = source.clone();
    let pointer = source.mixtures[0].means.as_ptr();
    let owned = source.clone().prepare_owned().unwrap();
    for (state, distribution) in source.mixtures.iter().enumerate() {
        let expected = distribution.prepare().unwrap();
        let actual = owned.emission(state).unwrap();
        assert_eq!(actual.parameters(), distribution);
        assert_eq!(
            actual.cached_log_normalizers(),
            expected.cached_log_normalizers()
        );
        assert_eq!(
            actual.cached_log_normalizers().as_ptr(),
            owned
                .emission(state)
                .unwrap()
                .cached_log_normalizers()
                .as_ptr()
        );
        for value in [-1000.0, -0.0, 0.25, 3.75, 1000.0] {
            let observation = vec![T::from_f64(value); distribution.dimensions];
            for temperature in [0.0, 0.5, 1.0, 1.75] {
                let temperature = T::from_f64(temperature);
                assert_eq!(
                    bits(actual.log_probability(&observation, temperature).unwrap()),
                    bits(expected.log_probability(&observation, temperature).unwrap())
                );
                assert_eq!(
                    actual
                        .probability(&observation, temperature)
                        .unwrap()
                        .to_bits(),
                    expected
                        .probability(&observation, temperature)
                        .unwrap()
                        .to_bits()
                );
            }
            for component in 0..distribution.weights.len() {
                assert_eq!(
                    bits(
                        actual
                            .component_log_probability(&observation, component)
                            .unwrap()
                    ),
                    bits(
                        expected
                            .component_log_probability(&observation, component)
                            .unwrap()
                    )
                );
                assert_eq!(
                    actual
                        .component_probability(&observation, component)
                        .unwrap()
                        .to_bits(),
                    expected
                        .component_probability(&observation, component)
                        .unwrap()
                        .to_bits()
                );
            }
        }
        let child = actual.to_owned();
        assert_eq!(child.parameters(), distribution);
        assert_eq!(
            child.cached_log_normalizers(),
            actual.cached_log_normalizers()
        );
        assert_ne!(
            child.cached_log_normalizers().as_ptr(),
            actual.cached_log_normalizers().as_ptr()
        );
        assert!(actual.log_probability(&[], T::ONE).is_err());
        assert!(
            actual
                .component_probability(
                    &vec![T::ZERO; distribution.dimensions],
                    distribution.weights.len()
                )
                .is_err()
        );
    }
    assert!(owned.emission(3).is_none());
    assert_eq!(owned.parameters(), &original);
    let cloned = owned.clone();
    source.mixtures[0].means.fill(T::from_f64(99.0));
    drop(source);
    assert_eq!(cloned.parameters(), &original);
    let retained_pointer = owned.parameters().mixtures[0].means.as_ptr();
    let cache_pointer = owned.emission(0).unwrap().cached_log_normalizers().as_ptr();
    let mut recovered = owned.into_parameters();
    assert_eq!(recovered.mixtures[0].means.as_ptr(), retained_pointer);
    assert_ne!(recovered.mixtures[0].means.as_ptr(), pointer);
    assert_eq!(recovered, original);
    recovered.mixtures[0].means.fill(T::from_f64(-99.0));
    assert_eq!(cloned.parameters(), &original);
    assert_ne!(
        cloned
            .emission(0)
            .unwrap()
            .cached_log_normalizers()
            .as_ptr(),
        cache_pointer
    );
    assert!(
        StreamParameters::<T>::default()
            .prepare_owned()
            .unwrap()
            .emission(0)
            .is_none()
    );
    for weight in [-1.0, f64::NAN, f64::INFINITY] {
        let mut invalid = original.clone();
        invalid.weight = T::from_f64(weight);
        assert!(invalid.prepare_owned().is_err());
    }
    let mut invalid = original;
    invalid.mixtures[2].variances[0] = T::ZERO;
    assert!(invalid.prepare_owned().is_err());
}

fn model_snapshot<T: Scalar>(fixture: &str) {
    let (mut source, observation, segmentation) = emissions::sample_with_precision::<T>();
    for length in [0, 2, 300] {
        let options = PreparationSettings {
            duration_cache_length: length,
            duration_weight: T::from_f64(0.75),
            temperature: T::from_f64(0.5),
        };
        let borrowed = source.prepare(options).unwrap();
        let owned = source.clone().prepare_owned(options).unwrap();
        assert_eq!(owned.model(), &source);
        assert_eq!(owned.options(), options);
        for stream in 0..2 {
            for state in 0..3 {
                let actual = owned.emission(stream, state).unwrap();
                let expected = borrowed.emission(stream, state).unwrap();
                assert_eq!(actual.parameters(), &source.streams[stream].mixtures[state]);
                assert_eq!(
                    actual.cached_log_normalizers(),
                    expected.cached_log_normalizers()
                );
            }
        }
        assert!(owned.emission(2, 0).is_none());
        assert!(owned.emission(0, 3).is_none());
        assert!(owned.duration(3).is_none());
        for state in 0..3 {
            let actual = owned.duration(state).unwrap();
            let expected = borrowed.duration(state).unwrap();
            assert_eq!(actual.parameters(), &source.durations[state]);
            assert_eq!(actual.options(), expected.options());
            assert_eq!(
                bits(actual.log_normalizer()),
                bits(expected.log_normalizer())
            );
            assert_eq!(
                actual.cached_log_probabilities(),
                expected.cached_log_probabilities()
            );
            for frames in [0, 1, 2, 4, 299, 300, 32767, i32::MAX as usize] {
                assert_eq!(
                    bits(actual.log_probability(frames).unwrap()),
                    bits(expected.log_probability(frames).unwrap())
                );
                assert_eq!(
                    actual.probability(frames).unwrap().to_bits(),
                    expected.probability(frames).unwrap().to_bits()
                );
            }
            assert!(actual.log_probability(i32::MAX as usize + 1).is_err());
            let copied = actual.to_owned();
            assert_eq!(copied.parameters(), actual.parameters());
            assert_eq!(
                copied.cached_log_probabilities(),
                actual.cached_log_probabilities()
            );
            if length != 0 {
                assert_ne!(
                    copied.cached_log_probabilities().as_ptr(),
                    actual.cached_log_probabilities().as_ptr()
                );
            }
        }
        let pruning = [
            EmissionPruningOptions::None,
            EmissionPruningOptions::AroundBoundaries { radius: T::ONE },
            EmissionPruningOptions::Linear {
                slope: T::from_f64(0.34),
            },
        ];
        for pruning in pruning {
            for temperature in [T::ZERO, T::from_f64(0.75), T::ONE] {
                assert_eq!(
                    owned
                        .emission_log_probabilities(
                            &observation,
                            &segmentation,
                            temperature,
                            pruning
                        )
                        .unwrap(),
                    borrowed
                        .emission_log_probabilities(
                            &observation,
                            &segmentation,
                            temperature,
                            pruning
                        )
                        .unwrap()
                );
                assert_eq!(
                    owned
                        .emission_probabilities(&observation, &segmentation, temperature, pruning)
                        .unwrap(),
                    borrowed
                        .emission_probabilities(&observation, &segmentation, temperature, pruning)
                        .unwrap()
                );
            }
        }
        let tables = [
            owned
                .emission_log_probabilities(
                    &observation,
                    &segmentation,
                    T::from_f64(0.75),
                    pruning[1],
                )
                .unwrap(),
            owned
                .emission_log_probabilities(
                    &observation,
                    &segmentation,
                    T::from_f64(0.75),
                    pruning[2],
                )
                .unwrap(),
        ];
        let mut compared = 0;
        for line in fixture.lines() {
            let fields: Vec<_> = line.split_whitespace().collect();
            let table: usize = fields[0].parse().unwrap();
            let index: usize = fields[1].parse().unwrap();
            let expected: f64 = T::from_f64(fields[2].parse::<f64>().unwrap()).into();
            assert!(
                (Into::<f64>::into(tables[table].values()[index]) - expected).abs()
                    <= 2e-14 * expected.abs().max(1.0),
                "{line}"
            );
            compared += 1;
        }
        assert_eq!(compared, 72);
        assert!(
            owned
                .emission_log_probabilities(
                    &observation,
                    &segmentation,
                    T::from_f64(-1.0),
                    pruning[0]
                )
                .is_err()
        );
    }
    let original = source.clone();
    let prepared = source
        .clone()
        .prepare_owned(PreparationSettings::default())
        .unwrap();
    let cloned = prepared.clone();
    source.streams.clear();
    source.durations.clear();
    drop(source);
    assert_eq!(cloned.model(), &original);
    assert_eq!(
        cloned
            .emission_log_probabilities(
                &observation,
                &segmentation,
                T::ONE,
                EmissionPruningOptions::None
            )
            .unwrap(),
        prepared
            .emission_log_probabilities(
                &observation,
                &segmentation,
                T::ONE,
                EmissionPruningOptions::None
            )
            .unwrap()
    );
    let pointer = prepared.model().streams[0].mixtures[0].means.as_ptr();
    let mut recovered = prepared.into_model();
    assert_eq!(recovered.streams[0].mixtures[0].means.as_ptr(), pointer);
    assert_eq!(recovered, original);
    recovered.streams[0].mixtures[0]
        .means
        .fill(T::from_f64(99.0));
    assert_eq!(cloned.model(), &original);
    recovered = original.clone();
    recovered.durations.push(DurationParameters::default());
    assert!(
        recovered
            .clone()
            .prepare_owned(PreparationSettings::default())
            .is_err()
    );
    let selected = recovered
        .clone()
        .prepare_owned_selected_durations(PreparationSettings::default(), &[0, 2, 0])
        .unwrap();
    assert!(selected.duration(0).is_some());
    assert!(selected.duration(1).is_none());
    assert!(selected.duration(2).is_some());
    assert!(selected.duration(3).is_none());
    assert!(
        recovered
            .clone()
            .prepare_owned_selected_durations(PreparationSettings::default(), &[3])
            .is_err()
    );
    assert!(
        recovered
            .clone()
            .prepare_owned_selected_durations(PreparationSettings::default(), &[4])
            .is_err()
    );
    let emissions_only = recovered
        .prepare_owned_selected_durations(PreparationSettings::default(), &[])
        .unwrap();
    assert!((0..4).all(|state| emissions_only.duration(state).is_none()));
    assert_eq!(
        emissions_only
            .emission_log_probabilities(
                &observation,
                &segmentation,
                T::ONE,
                EmissionPruningOptions::None
            )
            .unwrap(),
        cloned
            .emission_log_probabilities(
                &observation,
                &segmentation,
                T::ONE,
                EmissionPruningOptions::None
            )
            .unwrap()
    );
    assert!(
        ModelParameters::<T>::default()
            .prepare_owned(PreparationSettings::default())
            .unwrap()
            .emission(0, 0)
            .is_none()
    );
    for invalid in [
        PreparationSettings {
            temperature: T::from_f64(-1.0),
            ..PreparationSettings::default()
        },
        PreparationSettings {
            duration_weight: T::from_f64(f64::NAN),
            ..PreparationSettings::default()
        },
        PreparationSettings {
            duration_cache_length: i32::MAX as usize + 1,
            ..PreparationSettings::default()
        },
    ] {
        assert!(
            original
                .clone()
                .prepare_owned_selected_durations(invalid, &[])
                .is_err()
        );
    }
}

#[test]
fn owned_stream_f32_keeps_parameters_caches_and_density_results() {
    stream_snapshot::<f32>();
}
#[test]
fn owned_stream_f64_keeps_parameters_caches_and_density_results() {
    stream_snapshot::<f64>();
}
#[test]
fn owned_model_f32_keeps_caches_and_matches_original_c_tables() {
    model_snapshot::<f32>(include_str!("fixtures/c-emission-tables.txt"));
}
#[test]
fn owned_model_f64_keeps_caches_and_matches_original_c_tables() {
    model_snapshot::<f64>(include_str!("fixtures/c-double-emissions.txt"));
}

#[test]
#[cfg(feature = "serialization")]
fn owned_historical_models_prepare_selected_states_in_both_precisions() {
    fn check<T: Scalar>() {
        let source = ModelParameters::<T>::read_from(
            include_bytes!("fixtures/cmu-arctic-all-speakers.hsmm").as_slice(),
        )
        .unwrap();
        let selected: Vec<_> = source
            .durations
            .iter()
            .enumerate()
            .filter_map(|(index, duration)| (duration.variance > T::ZERO).then_some(index))
            .collect();
        assert_eq!(selected.len(), 205);
        let owned = source
            .clone()
            .prepare_owned_selected_durations(PreparationSettings::default(), &selected)
            .unwrap();
        assert_eq!(owned.model(), &source);
        assert!((0..215).all(|index| owned.duration(index).is_some() == selected.contains(&index)));
        let borrowed = source
            .prepare_selected_durations(PreparationSettings::default(), &selected)
            .unwrap();
        for (stream, parameters) in source.streams.iter().enumerate() {
            for state in 0..parameters.mixtures.len() {
                assert_eq!(
                    owned
                        .emission(stream, state)
                        .unwrap()
                        .cached_log_normalizers(),
                    borrowed
                        .emission(stream, state)
                        .unwrap()
                        .cached_log_normalizers()
                );
            }
        }
        for state in selected {
            assert_eq!(
                owned.duration(state).unwrap().cached_log_probabilities(),
                borrowed.duration(state).unwrap().cached_log_probabilities()
            );
        }
    }
    check::<f32>();
    check::<f64>();
}
