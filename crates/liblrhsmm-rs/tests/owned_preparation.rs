use liblrhsmm_rs::model::DurationParameters;
use liblrhsmm_rs::preparation::PreparationSettings;
use liblrhsmm_rs::scalar::Scalar;

fn check<T: Scalar>(structures: &str, densities: &str, exact_binary32: bool) {
    let parameters = DurationParameters::<T> {
        mean: T::from_f64(4.25),
        variance: T::from_f64(2.5),
        minimum: 2,
        maximum: 6,
        fixed_mean: 17,
        variance_floor: T::from_f64(0.125),
    };
    for length in [0, 1, 3, 8, 300] {
        for temperature in [0.0, 0.5, 1.0, 1.75] {
            let options = PreparationSettings {
                duration_cache_length: length,
                duration_weight: T::from_f64(0.75),
                temperature: T::from_f64(temperature),
            };
            let borrowed = parameters.prepare(options).unwrap();
            let owned = parameters.clone().prepare_owned(options).unwrap();
            assert_eq!(owned.options(), borrowed.options());
            assert_eq!(owned.parameters(), &parameters);
            assert_eq!(owned.log_normalizer(), borrowed.log_normalizer());
            assert_eq!(
                owned.cached_log_probabilities(),
                borrowed.cached_log_probabilities()
            );
            for frames in 0..=400 {
                assert_eq!(
                    owned.log_probability(frames).unwrap(),
                    borrowed.log_probability(frames).unwrap()
                );
                assert_eq!(
                    owned.probability(frames).unwrap(),
                    borrowed.probability(frames).unwrap()
                );
            }
            assert!(owned.log_probability(usize::MAX).is_err());
            assert!(owned.probability(usize::MAX).is_err());
            let cloned = owned.clone();
            if length != 0 {
                assert_ne!(
                    owned.cached_log_probabilities().as_ptr(),
                    cloned.cached_log_probabilities().as_ptr()
                );
            }
            let mut recovered = owned.into_parameters();
            assert_eq!(recovered, parameters);
            recovered.mean = T::from_f64(99.0);
            assert_eq!(recovered.mean, T::from_f64(99.0));
            assert_eq!(cloned.parameters().mean, T::from_f64(4.25));
            assert_eq!(
                cloned.cached_log_probabilities(),
                borrowed.cached_log_probabilities()
            );
        }
    }
    let owned = parameters
        .clone()
        .prepare_owned(PreparationSettings {
            duration_cache_length: 8,
            duration_weight: T::from_f64(0.5),
            temperature: T::from_f64(0.75),
        })
        .unwrap();
    let mut compared = 0;
    for line in structures.lines() {
        let fields: Vec<_> = line.split_whitespace().collect();
        let actual = match fields[0] {
            "duration" => owned.log_normalizer(),
            "cache" => owned.cached_log_probabilities()[fields[1].parse::<usize>().unwrap()],
            _ => continue,
        };
        let expected: f64 = fields.last().unwrap().parse().unwrap();
        if exact_binary32 {
            assert_eq!(actual, T::from_f64(expected), "{line}");
        }
        let actual: f64 = actual.into();
        assert!(
            (actual - expected).abs() / expected.abs().max(1.0) <= 2e-14,
            "{line}"
        );
        compared += 1;
    }
    assert_eq!(compared, 9);
    let mut count = 0;
    for line in densities
        .lines()
        .filter(|line| line.starts_with("duration "))
    {
        let fields: Vec<_> = line.split_whitespace().collect();
        let frames = fields[1].parse().unwrap();
        let temperature = [0.5, 1.0, 1.75][fields[2].parse::<usize>().unwrap()];
        let owned = parameters
            .clone()
            .prepare_owned(PreparationSettings {
                duration_cache_length: 3,
                duration_weight: T::from_f64(0.75),
                temperature: T::from_f64(temperature),
            })
            .unwrap();
        for (actual, expected) in [
            owned.log_probability(frames).unwrap().into(),
            owned.probability(frames).unwrap(),
        ]
        .into_iter()
        .zip(&fields[4..])
        {
            let expected: f64 = expected.parse().unwrap();
            assert!(
                (actual - expected).abs() / expected.abs().max(1.0) <= 2e-14,
                "{line}"
            );
            count += 1;
        }
    }
    assert_eq!(count, 54);
}

#[test]
fn binary32_owned_cache_and_fallback_retain_borrowed_and_original_c_results() {
    check::<f32>(
        include_str!("../../../tests/fixtures/c-structures-float.txt"),
        include_str!("../../../tests/fixtures/c-precision-float.txt"),
        true,
    );
}

#[test]
fn binary64_owned_cache_and_fallback_retain_borrowed_and_original_c_results() {
    check::<f64>(
        include_str!("../../../tests/fixtures/c-structures-double.txt"),
        include_str!("../../../tests/fixtures/c-precision-double.txt"),
        false,
    );
}

#[test]
fn owned_preparation_rejects_uninitialized_parameters_and_invalid_options() {
    assert!(
        DurationParameters::<f64>::default()
            .prepare_owned(PreparationSettings::default())
            .is_err()
    );
    let parameters = DurationParameters::<f64> {
        variance: 1.0,
        ..Default::default()
    };
    for options in [
        PreparationSettings {
            duration_cache_length: usize::MAX,
            ..Default::default()
        },
        PreparationSettings {
            duration_weight: -1.0,
            ..Default::default()
        },
        PreparationSettings {
            temperature: f64::NAN,
            ..Default::default()
        },
    ] {
        assert!(parameters.clone().prepare_owned(options).is_err());
    }
}
