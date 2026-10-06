use liblrhsmm_rs::{
    Duration, GaussianMixture, MixtureOccupancyMode, Model, Observation, ObservationStream,
    OccupancyTable, Segmentation, Stream,
};

fn sample() -> (Model, Observation, Segmentation, OccupancyTable) {
    let mixtures = (0..3)
        .map(|state| {
            let count = [2, 3, 1][state];
            let mut mixture = GaussianMixture::new(count, 2).unwrap();
            for index in 0..mixture.means.len() {
                mixture.means[index] = state as f32 * 0.5 + index as f32 * 0.25;
                mixture.variances[index] = 0.75 + index as f32 * 0.25;
            }
            for component in 0..count {
                mixture.weights[component] =
                    (component as f32 + 1.0) / (count as f32 * (count as f32 + 1.0) / 2.0);
            }
            mixture
        })
        .collect();
    let model = Model {
        streams: vec![Stream {
            weight: 1.0,
            mixtures,
        }],
        durations: vec![
            Duration {
                mean: 4.0,
                variance: 1.0,
                ..Duration::default()
            };
            3
        ],
    };
    let observation = Observation {
        frames: 8,
        streams: vec![ObservationStream {
            dimensions: 2,
            values: (0..8)
                .flat_map(|frame| {
                    (0..2).map(move |dimension| frame as f32 * 0.125 + dimension as f32 * 0.5)
                })
                .collect(),
        }],
    };
    let mut segmentation = Segmentation::new(1, 3);
    segmentation.boundaries = vec![2, 5, 8];
    segmentation.output_states[0] = vec![0, 2, 1];
    let occupancy = OccupancyTable::from_values(
        8,
        3,
        (0..8)
            .flat_map(|frame| {
                (0..3).map(move |state| {
                    if frame % 3 == 0 {
                        0.0
                    } else {
                        0.1 + 0.1 * state as f32 + 0.05 * frame as f32
                    }
                })
            })
            .collect(),
    )
    .unwrap();
    (model, observation, segmentation, occupancy)
}

#[test]
fn component_posteriors_match_c_for_hsmm_and_hmm_modes() {
    let (model, observation, segmentation, occupancy) = sample();
    let fixture = include_str!("fixtures/c-mixture-occupancy.txt");
    assert_eq!(fixture.lines().count(), 96);
    for temperature in [0.5, 1.0] {
        for mode_index in 0..2 {
            let mode = if mode_index == 0 {
                MixtureOccupancyMode::Hsmm { state_radius: 1 }
            } else {
                MixtureOccupancyMode::Geometric { pruning_slope: 0.5 }
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
            assert_eq!(result.frames(), 8);
            assert_eq!(result.states(), 3);
            for line in fixture.lines() {
                let mut fields = line.split_whitespace();
                let expected_mode: usize = fields.next().unwrap().parse().unwrap();
                let expected_temperature: f32 = fields.next().unwrap().parse().unwrap();
                if expected_mode != mode_index || expected_temperature != temperature {
                    continue;
                }
                let frame: usize = fields.next().unwrap().parse().unwrap();
                let state: usize = fields.next().unwrap().parse().unwrap();
                let count: usize = fields.next().unwrap().parse().unwrap();
                let expected: Vec<f32> = fields.map(|value| value.parse().unwrap()).collect();
                assert_eq!(expected.len(), count);
                assert_eq!(
                    result.get(frame, state),
                    if count == 0 {
                        None
                    } else {
                        Some(expected.as_slice())
                    },
                    "C mismatch: {line}"
                );
            }
            assert_eq!(result.get(8, 0), None);
        }
    }
}

#[test]
fn invalid_inputs_fail_before_component_evaluation() {
    let (model, observation, mut segmentation, occupancy) = sample();
    let mode = MixtureOccupancyMode::Hsmm { state_radius: 1 };
    assert!(
        model
            .mixture_occupancies(&observation, &segmentation, &occupancy, 1, 1.0, mode)
            .is_err()
    );
    assert!(
        model
            .mixture_occupancies(&observation, &segmentation, &occupancy, 0, f32::NAN, mode)
            .is_err()
    );
    segmentation.output_states[0][0] = -1;
    assert!(
        model
            .mixture_occupancies(&observation, &segmentation, &occupancy, 0, 1.0, mode)
            .is_err()
    );
    assert!(OccupancyTable::from_values(1, 1, vec![-1.0]).is_err());
    assert!(OccupancyTable::from_values(1, 1, vec![f32::INFINITY]).is_err());
}
