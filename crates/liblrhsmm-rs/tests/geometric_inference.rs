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
    let fixture = include_str!("../../../tests/fixtures/c-geometric.txt");
    assert_eq!(fixture.lines().count(), 384);
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
        } else if kind == "occupancy" {
            let expected: f32 = fields.next().unwrap().parse().unwrap();
            let present: usize = fields.next().unwrap().parse().unwrap();
            let stay: f32 = fields.next().unwrap().parse().unwrap();
            let advance: f32 = fields.next().unwrap().parse().unwrap();
            let forward = model
                .forward_geometric(&segmentation, &emissions, options)
                .unwrap();
            let backward = model
                .backward_geometric(&segmentation, &emissions, options)
                .unwrap();
            let total = backward.backward_log_probability(&emissions).unwrap();
            let occupancy = forward
                .geometric_occupancies(&backward, total, pruning_slope)
                .unwrap();
            let transitions = model
                .geometric_transition_occupancies(
                    &segmentation,
                    &emissions,
                    &forward,
                    &backward,
                    total,
                    options,
                )
                .unwrap();
            assert_eq!(
                occupancy.values()[index],
                expected,
                "occupancy mismatch: {line}"
            );
            let actual = transitions.values()[index];
            assert_eq!(actual.is_some(), present != 0);
            if let Some(actual) = actual {
                assert_eq!(
                    [actual.stay, actual.advance],
                    [stay, advance],
                    "transition mismatch: {line}"
                );
            }
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

#[test]
fn occupancy_rejects_mismatched_dimensions_and_nonfinite_normalizers() {
    let (model, segmentation, emissions) = sample(3, 12);
    let options = GeometricOptions {
        temperature: 1.0,
        pruning_slope: 1.0,
    };
    let forward = model
        .forward_geometric(&segmentation, &emissions, options)
        .unwrap();
    let backward = model
        .backward_geometric(&segmentation, &emissions, options)
        .unwrap();
    assert!(
        forward
            .geometric_occupancies(&backward, f32::NAN, 1.0)
            .is_err()
    );
    assert!(forward.geometric_occupancies(&backward, 0.0, -1.0).is_err());
    let (short_model, short_segmentation, short_emissions) = sample(1, 1);
    let short = short_model
        .forward_geometric(&short_segmentation, &short_emissions, options)
        .unwrap();
    assert!(forward.geometric_occupancies(&short, 0.0, 1.0).is_err());
    assert!(
        model
            .geometric_transition_occupancies(
                &segmentation,
                &short_emissions,
                &forward,
                &backward,
                0.0,
                options
            )
            .is_err()
    );
    let total = backward.backward_log_probability(&emissions).unwrap();
    let occupancy = forward
        .geometric_occupancies(&backward, total, 1.0)
        .unwrap();
    let transitions = model
        .geometric_transition_occupancies(
            &segmentation,
            &emissions,
            &forward,
            &backward,
            total,
            options,
        )
        .unwrap();
    assert_eq!(occupancy.get(0, 0), Some(0.0));
    assert_eq!(occupancy.get(11, 2), Some(0.0));
    assert_eq!(transitions.get(0, 0), None);
    assert_eq!(transitions.get(11, 2), None);
    assert_eq!(occupancy.get(12, 0), None);
}
