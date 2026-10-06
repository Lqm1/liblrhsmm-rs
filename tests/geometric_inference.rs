use liblrhsmm_rs::{Duration, EmissionTable, GeometricOptions, Model, Segmentation, numerics};

fn sample(states: usize, frames: usize) -> (Model, Segmentation, EmissionTable) {
    let model = Model {
        streams: vec![],
        durations: (0..states)
            .map(|state| Duration {
                mean: [0.5, 2.75, 5.0][state],
                variance: 1.0,
                ..Duration::default()
            })
            .collect(),
    };
    let mut segmentation = Segmentation::new(0, states);
    segmentation.duration_states = (0..states as i32).collect();
    let values = (0..frames)
        .flat_map(|frame| {
            (0..states).map(move |state| {
                let residual = frame as f32 - state as f32 * 4.0;
                -residual * residual * 0.25
            })
        })
        .collect();
    (
        model,
        segmentation,
        EmissionTable::from_log_values(frames, states, values).unwrap(),
    )
}

#[test]
fn matches_original_forward_backward_and_viterbi() {
    let fixture = include_str!("fixtures/c-geometric.txt");
    assert_eq!(fixture.lines().count(), 238);
    for line in fixture.lines() {
        let mut fields = line.split_whitespace();
        let kind = fields.next().unwrap();
        if kind == "math" {
            let x: f32 = fields.next().unwrap().parse().unwrap();
            let expected: Vec<f32> = fields.map(|value| value.parse().unwrap()).collect();
            assert_eq!(
                [
                    numerics::log_add(x * 0.125, -x),
                    numerics::negative_exponential(-x),
                    numerics::exponential(-x)
                ]
                .as_slice(),
                expected
            );
            continue;
        }
        let states: usize = fields.next().unwrap().parse().unwrap();
        let frames: usize = fields.next().unwrap().parse().unwrap();
        let temperature: f32 = fields.next().unwrap().parse().unwrap();
        let pruning_slope: f32 = fields.next().unwrap().parse().unwrap();
        let index: usize = fields.next().unwrap().parse().unwrap();
        let (model, segmentation, emissions) = sample(states, frames);
        let options = GeometricOptions {
            temperature,
            pruning_slope,
        };
        let alignment = model
            .viterbi_geometric(&segmentation, &emissions, options)
            .unwrap();
        if kind == "boundary" {
            let expected: usize = fields.next().unwrap().parse().unwrap();
            assert_eq!(
                alignment.boundaries[index], expected,
                "boundary mismatch: {line}"
            );
        } else {
            let expected: Vec<f32> = fields.map(|value| value.parse().unwrap()).collect();
            let forward = model
                .forward_geometric(&segmentation, &emissions, options)
                .unwrap();
            let backward = model
                .backward_geometric(&segmentation, &emissions, options)
                .unwrap();
            let actual = if kind == "scores" {
                [
                    forward.values()[index],
                    backward.values()[index],
                    alignment.scores.values()[index],
                ]
            } else {
                assert_eq!(kind, "total");
                [
                    forward.total_through_state(&backward, index).unwrap(),
                    forward.total_log_probability(),
                    backward.backward_log_probability(&emissions).unwrap(),
                ]
            };
            assert_eq!(actual.as_slice(), expected, "score mismatch: {line}");
        }
    }
}

#[test]
fn reports_impossible_paths_and_invalid_duration_states() {
    let (model, mut segmentation, emissions) = sample(3, 1);
    assert!(
        model
            .viterbi_geometric(
                &segmentation,
                &emissions,
                GeometricOptions {
                    pruning_slope: 1.0,
                    ..GeometricOptions::default()
                }
            )
            .is_err()
    );
    segmentation.duration_states[0] = -1;
    assert!(
        model
            .forward_geometric(&segmentation, &emissions, GeometricOptions::default())
            .is_err()
    );
    assert!(EmissionTable::from_log_values(0, 1, vec![]).is_err());
    assert!(EmissionTable::from_log_values(1, 1, vec![f32::NAN]).is_err());
}
