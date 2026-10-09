use liblrhsmm_rs::{
    Duration, DurationMode, DurationStatistics, GaussianMixture, GaussianStatistics, Model,
    ModelStatistics, Stream, UpdateOptions,
};

#[test]
fn empty_stream_updates_validate_options_in_both_precisions() {
    use liblrhsmm_rs::model::StreamParameters;
    use liblrhsmm_rs::scalar::Scalar;
    use liblrhsmm_rs::statistics::StreamSufficientStatistics;
    use liblrhsmm_rs::update::UpdateSettings;
    fn check<T: Scalar>() {
        let mut stream = StreamParameters::<T>::default();
        let statistics = StreamSufficientStatistics::from_stream(&stream).unwrap();
        let before = stream.clone();
        for value in [-1.0, f64::NAN, f64::INFINITY] {
            for duration in [false, true] {
                let mut options = UpdateSettings::default();
                if duration {
                    options.duration_variance_floor = T::from_f64(value);
                } else {
                    options.emission_variance_floor = T::from_f64(value);
                }
                assert!(stream.update(&statistics, options).is_err());
                assert_eq!(stream, before);
            }
        }
        stream
            .update(&statistics, UpdateSettings::default())
            .unwrap();
        assert_eq!(stream, before);
    }
    check::<f32>();
    check::<f64>();
}

#[test]
fn maximization_matches_unmodified_c() {
    for line in include_str!("../../../tests/fixtures/c-updates.txt").lines() {
        let mut fields = line.split_whitespace();
        let kind = fields.next().unwrap();
        let scenario: usize = fields.next().unwrap().parse().unwrap();
        if kind == "duration" {
            let mut duration = Duration {
                mean: 7.0,
                variance: 3.0,
                variance_floor: if scenario == 2 { 2.5 } else { 0.0 },
                ..Duration::default()
            };
            let statistics = DurationStatistics {
                value_sum: if scenario == 3 { 1.5 } else { 9.0 },
                squared_sum: 29.0,
                occupancy: if scenario == 4 { 1e-8 } else { 3.0 },
            };
            let options = UpdateOptions {
                duration_mode: if scenario == 3 {
                    DurationMode::Geometric
                } else {
                    DurationMode::Normal
                },
                ..UpdateOptions::default()
            };
            duration.update(&statistics, options).unwrap();
            let expected: Vec<f32> = fields.map(|x| x.parse().unwrap()).collect();
            assert_eq!(
                [duration.mean, duration.variance].as_slice(),
                expected,
                "{line}"
            );
        } else {
            assert_eq!(kind, "mixture");
            let component: usize = fields.next().unwrap().parse().unwrap();
            let mut gaussian = GaussianMixture::new(if scenario == 0 { 1 } else { 2 }, 2).unwrap();
            gaussian.weights.fill(0.5);
            gaussian.means.fill(-7.0);
            gaussian.variances.fill(8.0);
            let mut statistics = GaussianStatistics::from_gaussian(&gaussian).unwrap();
            statistics.state_occupancy = if scenario == 3 { 1e-8 } else { 4.0 };
            for k in 0..gaussian.weights.len() {
                statistics.component_occupancies[k] = if scenario == 2 && k == 1 {
                    1e-8
                } else {
                    2.0 + k as f32
                };
                statistics.weighted_sums[k * 2..k * 2 + 2].copy_from_slice(&[-3.5, 6.0]);
                statistics.weighted_squared_sums[k * 2..k * 2 + 2].copy_from_slice(&[9.25, 12.0]);
                gaussian.variance_floors[k * 2 + 1] = 0.75;
            }
            gaussian
                .update(&statistics, UpdateOptions::default())
                .unwrap();
            let index = component * 2;
            let actual = [
                gaussian.weights[component],
                gaussian.means[index],
                gaussian.variances[index],
                gaussian.means[index + 1],
                gaussian.variances[index + 1],
            ];
            let expected: Vec<f32> = fields.map(|x| x.parse().unwrap()).collect();
            assert_eq!(actual.as_slice(), expected, "{line}");
        }
    }
}

#[test]
fn fixed_mean_variance_uses_squared_residuals() {
    // Observed durations 2 and 4, constrained mean 4: ((2-4)^2 + (4-4)^2)/2 = 2.
    let mut duration = Duration {
        fixed_mean: 4,
        ..Duration::default()
    };
    duration
        .update(
            &DurationStatistics {
                value_sum: 6.0,
                squared_sum: 20.0,
                occupancy: 2.0,
            },
            UpdateOptions::default(),
        )
        .unwrap();
    assert_eq!(duration.mean, 4.0);
    assert_eq!(duration.variance, 2.0);
    duration.variance_floor = 3.0;
    duration
        .update(
            &DurationStatistics {
                value_sum: 6.0,
                squared_sum: 20.0,
                occupancy: 2.0,
            },
            UpdateOptions::default(),
        )
        .unwrap();
    assert_eq!(duration.variance, 3.0);
}

#[test]
fn failed_model_update_preserves_all_parameters() {
    let mut model = Model {
        streams: vec![Stream {
            weight: 1.0,
            mixtures: vec![GaussianMixture::new(1, 1).unwrap()],
        }],
        durations: vec![Duration::default()],
    };
    let mut statistics = ModelStatistics::from_model(&model).unwrap();
    let gaussian = &mut statistics.streams[0].mixtures[0];
    gaussian.state_occupancy = 1.0;
    gaussian.component_occupancies[0] = 1.0;
    gaussian.weighted_sums[0] = 2.0;
    gaussian.weighted_squared_sums[0] = 5.0;
    statistics.durations[0].occupancy = 1.0;
    let before = model.clone();
    let geometric = UpdateOptions {
        duration_mode: DurationMode::Geometric,
        ..UpdateOptions::default()
    };
    assert!(model.update(&statistics, geometric).is_err());
    assert_eq!(model, before);
    statistics.durations[0].value_sum = 0.5;
    model.update(&statistics, geometric).unwrap();
    assert_eq!(model.streams[0].mixtures[0].means, [2.0]);
    assert_eq!(model.streams[0].mixtures[0].variances, [1.0]);
    assert_eq!(model.durations[0].mean, 2.0);
    statistics.clear();
    assert_eq!(statistics, ModelStatistics::from_model(&model).unwrap());
    statistics.streams[0].mixtures[0].weighted_sums.clear();
    let before = model.clone();
    assert!(model.update(&statistics, UpdateOptions::default()).is_err());
    assert_eq!(model, before);
}
