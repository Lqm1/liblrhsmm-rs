use liblrhsmm_rs::data::SegmentationData;
use liblrhsmm_rs::generation::{
    choose_with_precision, gaussian_with_precision, uniform_with_precision,
};
use liblrhsmm_rs::model::{ModelParameters, StreamLayout};
use rand::{SeedableRng, rngs::SmallRng};

fn draw(index: usize, maximum: i32) -> f64 {
    let values = [
        maximum / 8,
        maximum / 4,
        maximum / 2,
        maximum / 4 * 3,
        maximum / 8 * 7,
        maximum,
    ];
    f64::from(values[index % values.len()]) / f64::from(maximum)
}

fn sample(scenario: usize) -> (ModelParameters<f64>, SegmentationData<f64>) {
    let mut model = ModelParameters::new(
        &[
            StreamLayout {
                emissions: 2,
                components: 2,
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
    for (stream, parameters) in model.streams.iter_mut().enumerate() {
        for (state, gaussian) in parameters.mixtures.iter_mut().enumerate() {
            if stream == 0 {
                gaussian.weights = vec![0.25, 0.75];
            }
            for index in 0..gaussian.means.len() {
                gaussian.means[index] = state as f64 + index as f64 * 0.5 + stream as f64 * 2.0;
                gaussian.variances[index] = if scenario == 0 {
                    0.25 + index as f64 * 0.25
                } else {
                    0.0
                };
            }
        }
    }
    for (state, duration) in model.durations.iter_mut().enumerate() {
        duration.mean = if scenario == 0 {
            2.5 - state as f64
        } else {
            0.0
        };
        duration.variance = if scenario == 0 {
            0.75 - state as f64 * 0.5
        } else {
            0.0
        };
        duration.minimum = 9;
    }
    let mut sequence = SegmentationData::new(2, 2);
    sequence.duration_states = vec![0, 1];
    sequence.output_states = vec![vec![0, 1], vec![1, 0]];
    (model, sequence)
}

#[test]
fn double_generation_matches_both_native_c_random_ranges() {
    let mut compared = 0;
    let mut maximum_error = 0.0_f64;
    for fixture in [
        include_str!("fixtures/c-double-generation-32767.txt"),
        include_str!("fixtures/c-double-generation-2147483647.txt"),
    ] {
        assert_eq!(fixture.lines().count(), 23);
        let maximum: i32 = fixture
            .lines()
            .next()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        for scenario in 0..2 {
            let (model, mut sequence) = sample(scenario);
            let mut draws = 0;
            let observation = model
                .generate_with_uniform(&mut sequence, || {
                    let value = draw(draws, maximum);
                    draws += 1;
                    value
                })
                .unwrap();
            for line in fixture.lines().skip(1) {
                let fields: Vec<_> = line.split_whitespace().collect();
                let index: usize = fields[1].parse().unwrap();
                let actual = match fields[0] {
                    "gaussian" if scenario == 0 => vec![
                        gaussian_with_precision(
                            -0.75,
                            1.25,
                            draw(index, maximum),
                            draw(index + 1, maximum),
                        )
                        .unwrap(),
                    ],
                    "choice" if scenario == 0 => {
                        assert_eq!(
                            choose_with_precision(&[0.25, 0.5, 0.1], draw(index, maximum)).unwrap(),
                            fields[2].parse::<usize>().unwrap()
                        );
                        continue;
                    }
                    "boundaries" if index == scenario => {
                        assert_eq!(
                            sequence.boundaries,
                            fields[2..]
                                .iter()
                                .map(|value| value.parse::<i32>().unwrap())
                                .collect::<Vec<_>>()
                        );
                        continue;
                    }
                    "draws" if index == scenario => {
                        assert_eq!(draws, fields[2].parse::<usize>().unwrap());
                        continue;
                    }
                    "frame" if index == scenario => {
                        let time: usize = fields[2].parse().unwrap();
                        (0..2)
                            .flat_map(|stream| {
                                observation.frame(stream, time).unwrap().iter().copied()
                            })
                            .collect()
                    }
                    _ => continue,
                };
                let start = if fields[0] == "frame" { 3 } else { 2 };
                let expected: Vec<f64> = fields[start..]
                    .iter()
                    .map(|value| value.parse().unwrap())
                    .collect();
                assert_eq!(actual.len(), expected.len());
                for (actual, expected) in actual.into_iter().zip(expected) {
                    assert!(actual.is_finite() && expected.is_finite());
                    maximum_error =
                        maximum_error.max((actual - expected).abs() / expected.abs().max(1.0));
                    compared += 1;
                }
            }
        }
    }
    println!("double generation: {compared} values; maximum normalized error {maximum_error}");
    assert_eq!(compared, 48);
    assert!(maximum_error <= 2e-14);
}

#[test]
fn double_random_draws_and_component_thresholds_retain_precision() {
    let mut rng = SmallRng::seed_from_u64(42);
    let values: Vec<f64> = (0..128).map(|_| uniform_with_precision(&mut rng)).collect();
    assert!(values.iter().all(|value| *value > 0.0 && *value <= 1.0));
    assert!(
        values
            .iter()
            .any(|value| f64::from(*value as f32) != *value)
    );
    assert_eq!(
        choose_with_precision(&[0.5 + f64::EPSILON, 0.5], 0.5).unwrap(),
        0
    );
    assert_eq!(choose_with_precision(&[0.5, 0.5], 0.5).unwrap(), 1);
    assert_eq!(
        gaussian_with_precision(1.0 + f64::EPSILON, 0.0, 0.5, 0.5).unwrap(),
        1.0 + f64::EPSILON
    );
    let (model, mut first) = sample(0);
    let mut second = first.clone();
    assert_eq!(
        model
            .generate(&mut first, &mut SmallRng::seed_from_u64(42))
            .unwrap(),
        model
            .generate(&mut second, &mut SmallRng::seed_from_u64(42))
            .unwrap()
    );
    assert_eq!(first, second);
}

#[test]
fn double_generation_errors_preserve_sequence_after_consuming_draws() {
    let (mut model, mut sequence) = sample(0);
    sequence.boundaries.fill(999);
    let before = sequence.clone();
    let mut draws = 0;
    assert!(
        model
            .generate_with_uniform(&mut sequence, || {
                draws += 1;
                if draws < 6 { 0.5 } else { f64::NAN }
            })
            .is_err()
    );
    assert!(draws >= 6);
    assert_eq!(sequence, before);
    model.durations[0].mean = f64::MAX;
    assert!(model.generate_with_uniform(&mut sequence, || 0.5).is_err());
    assert_eq!(sequence, before);
    for radial in [0.0, -1.0, f64::INFINITY, f64::NAN] {
        assert!(gaussian_with_precision(0.0, 1.0, radial, 0.5).is_err());
    }
}
