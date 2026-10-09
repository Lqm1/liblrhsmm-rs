use liblrhsmm_rs::{
    Duration, GaussianMixture, Model, Segmentation, Stream,
    generation::{choose_component, gaussian_sample},
};
use rand::{SeedableRng, rngs::SmallRng};

fn draw(index: usize) -> f32 {
    let maximum = i32::MAX;
    let values = [
        maximum / 8,
        maximum / 4,
        maximum / 2,
        maximum / 4 * 3,
        maximum / 8 * 7,
        maximum,
    ];
    values[index % values.len()] as f32 / maximum as f32
}
fn sample(scenario: usize) -> (Model, Segmentation) {
    let model = Model {
        streams: [2, 1]
            .into_iter()
            .enumerate()
            .map(|(stream, dimensions)| Stream {
                weight: 1.0,
                mixtures: (0..2)
                    .map(|state| {
                        let mut g =
                            GaussianMixture::new(if stream == 0 { 2 } else { 1 }, dimensions)
                                .unwrap();
                        if stream == 0 {
                            g.weights = vec![0.25, 0.75];
                        }
                        for index in 0..g.means.len() {
                            g.means[index] =
                                state as f32 + index as f32 * 0.5 + stream as f32 * 2.0;
                            g.variances[index] = if scenario == 0 {
                                0.25 + index as f32 * 0.25
                            } else {
                                0.0
                            };
                        }
                        g
                    })
                    .collect(),
            })
            .collect(),
        durations: (0..2)
            .map(|state| Duration {
                mean: if scenario == 0 {
                    2.5 - state as f32
                } else {
                    0.0
                },
                variance: if scenario == 0 {
                    0.75 - state as f32 * 0.5
                } else {
                    0.0
                },
                minimum: 9,
                ..Duration::default()
            })
            .collect(),
    };
    let mut sequence = Segmentation::new(2, 2);
    sequence.duration_states = vec![0, 1];
    sequence.output_states = vec![vec![0, 1], vec![1, 0]];
    (model, sequence)
}

#[test]
fn generation_and_scalar_samplers_match_controlled_c_random_draws() {
    let fixture = include_str!("../../../tests/fixtures/c-generation.txt");
    for scenario in 0..2 {
        let (model, mut sequence) = sample(scenario);
        let mut draws = 0;
        let observation = model
            .generate_with_uniform(&mut sequence, || {
                let value = draw(draws);
                draws += 1;
                value
            })
            .unwrap();
        for line in fixture.lines() {
            let mut fields = line.split_whitespace();
            let kind = fields.next().unwrap();
            let index: usize = fields.next().unwrap().parse().unwrap();
            match kind {
                "gaussian" if scenario == 0 => {
                    let expected: f32 = fields.next().unwrap().parse().unwrap();
                    assert_eq!(
                        gaussian_sample(-0.75, 1.25, draw(index), draw(index + 1)).unwrap(),
                        expected,
                        "{line}"
                    );
                }
                "choice" if scenario == 0 => {
                    let expected: usize = fields.next().unwrap().parse().unwrap();
                    assert_eq!(
                        choose_component(&[0.25, 0.5, 0.1], draw(index)).unwrap(),
                        expected,
                        "{line}"
                    );
                }
                "boundaries" if index == scenario => {
                    let expected: Vec<i32> = fields.map(|x| x.parse().unwrap()).collect();
                    assert_eq!(sequence.boundaries, expected);
                }
                "frame" if index == scenario => {
                    let time: usize = fields.next().unwrap().parse().unwrap();
                    let actual: Vec<_> = (0..2)
                        .flat_map(|stream| observation.frame(stream, time).unwrap().iter().copied())
                        .collect();
                    let expected: Vec<f32> = fields.map(|x| x.parse().unwrap()).collect();
                    assert_eq!(actual, expected, "{line}");
                }
                "draws" if index == scenario => {
                    assert_eq!(draws, fields.next().unwrap().parse::<usize>().unwrap())
                }
                _ => {}
            }
        }
    }
}

#[test]
fn generation_errors_preserve_sequence_and_prevalidate_models() {
    let (mut model, mut sequence) = sample(0);
    sequence.boundaries.fill(999);
    let before = sequence.clone();
    assert!(model.generate_with_uniform(&mut sequence, || 0.0).is_err());
    assert_eq!(sequence, before);
    let mut draws = 0;
    assert!(
        model
            .generate_with_uniform(&mut sequence, || {
                draws += 1;
                if draws < 6 { 0.5 } else { f32::NAN }
            })
            .is_err()
    );
    assert_eq!(sequence, before);
    model.streams[1].mixtures[0].variances[0] = -1.0;
    draws = 0;
    assert!(
        model
            .generate_with_uniform(&mut sequence, || {
                draws += 1;
                0.5
            })
            .is_err()
    );
    assert_eq!(draws, 0);
    assert_eq!(sequence, before);
    assert!(
        model
            .generate_with_uniform(&mut Segmentation::new(2, 0), || 0.5)
            .is_err()
    );
}

#[test]
fn seeded_rng_generation_is_repeatable_and_rejects_overflow() {
    let (mut model, mut first) = sample(0);
    let mut second = first.clone();
    let observation = model
        .generate(&mut first, &mut SmallRng::seed_from_u64(42))
        .unwrap();
    let repeated = model
        .generate(&mut second, &mut SmallRng::seed_from_u64(42))
        .unwrap();
    assert_eq!(first, second);
    assert_eq!(observation, repeated);
    assert!(
        observation
            .streams
            .iter()
            .flat_map(|stream| &stream.values)
            .all(|x| x.is_finite())
    );
    model.durations[0].mean = f32::MAX;
    let before = first.clone();
    assert!(model.generate_with_uniform(&mut first, || 0.5).is_err());
    assert_eq!(first, before);
}

#[test]
fn sampler_domains_and_duration_rounding_are_explicit() {
    assert_eq!(gaussian_sample(3.0, 0.0, 0.25, 1.0).unwrap(), 3.0);
    for radial in [0.0, -0.5, 1.1, f32::NAN] {
        assert!(gaussian_sample(0.0, 1.0, radial, 0.5).is_err());
    }
    assert!(gaussian_sample(0.0, -1.0, 0.5, 0.5).is_err());
    assert!(gaussian_sample(0.0, 1.0, 0.5, 1.1).is_err());
    assert!(choose_component(&[], 0.5).is_err());
    assert!(choose_component(&[1.0, -0.5], 0.5).is_err());
    assert!(choose_component(&[1.0], f32::NAN).is_err());
    assert_eq!(choose_component(&[0.0, 0.0], 0.0).unwrap(), 1);
    let (mut model, mut sequence) = sample(1);
    for duration in &mut model.durations {
        duration.mean = 1.5;
    }
    let observation = model.generate_with_uniform(&mut sequence, || 0.5).unwrap();
    assert_eq!(sequence.boundaries, [2, 4]);
    assert_eq!(observation.frames, 4);
}
