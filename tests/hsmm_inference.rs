use liblrhsmm_rs::{Duration, EmissionTable, HsmmOptions, Model, Segmentation};

fn sample(scenario: usize) -> (Model, Segmentation, EmissionTable, HsmmOptions) {
    let model = Model {
        streams: vec![],
        durations: (0..4)
            .map(|state| Duration {
                mean: 2.5 + state as f32 * 0.25,
                variance: 0.75 + state as f32 * 0.25,
                minimum: if state == 1 { 2 } else { -1 },
                maximum: if scenario == 3 && state == 1 { 3 } else { 7 },
                ..Duration::default()
            })
            .collect(),
    };
    let mut segmentation = Segmentation::new(0, 4);
    segmentation.boundaries = vec![3, 6, 9, 12];
    segmentation.duration_states = vec![0, 1, 2, 3];
    if scenario > 0 {
        for (source, delta, probability) in [(0, 2, 0.2), (1, 0, 0.15), (2, -1, 0.1)] {
            segmentation.add_jump(source, delta, probability).unwrap();
        }
        for state in 0..3 {
            segmentation.add_jump(state, 1, 0.8).unwrap();
        }
    }
    let values = (0..12)
        .flat_map(|frame| {
            (0..4).map(move |state| {
                if scenario == 3 {
                    let desired = [0, 0, 2, 2, 1, 1, 1, 1, 2, 3, 3, 3];
                    return if state == desired[frame] { 0.0 } else { -50.0 };
                }
                let residual = frame as f32 - state as f32 * 3.0;
                -residual * residual * 0.125
            })
        })
        .collect();
    let options = HsmmOptions {
        temperature: if scenario == 2 { 0.5 } else { 1.0 },
        duration_weight: 0.75,
        state_radius: if scenario == 2 { 1 } else { 5 },
        duration_extra: 3,
        duration_extra_factor: 0.5,
    };
    (
        model,
        segmentation,
        EmissionTable::from_log_values(12, 4, values).unwrap(),
        options,
    )
}

#[test]
fn matches_c_forward_backward_scores_and_viterbi_paths_with_jumps() {
    let fixture = include_str!("fixtures/c-hsmm.txt");
    assert_eq!(
        fixture
            .lines()
            .filter(|line| line.starts_with("score "))
            .count(),
        192
    );
    for scenario in 0..4 {
        let (model, segmentation, emissions, options) = sample(scenario);
        let forward = model
            .forward_hsmm(&segmentation, &emissions, options)
            .unwrap();
        let alignment = model
            .viterbi_hsmm(&segmentation, &emissions, options)
            .unwrap();
        let backward = model
            .backward_hsmm(&segmentation, &emissions, options)
            .unwrap();
        let mut path_count = 0;
        for line in fixture.lines() {
            let mut fields = line.split_whitespace();
            let kind = fields.next().unwrap();
            let case: usize = fields.next().unwrap().parse().unwrap();
            if case != scenario {
                continue;
            }
            let index: usize = fields.next().unwrap().parse().unwrap();
            if kind == "score" {
                let begin: f32 = fields.next().unwrap().parse().unwrap();
                let end: f32 = fields.next().unwrap().parse().unwrap();
                let backward_begin: f32 = fields.next().unwrap().parse().unwrap();
                let backward_end: f32 = fields.next().unwrap().parse().unwrap();
                assert_eq!(
                    forward.begin.values()[index],
                    begin,
                    "begin mismatch: {line}"
                );
                assert_eq!(forward.end.values()[index], end, "end mismatch: {line}");
                assert_eq!(
                    backward.begin.values()[index],
                    backward_begin,
                    "backward begin mismatch: {line}"
                );
                assert_eq!(
                    backward.end.values()[index],
                    backward_end,
                    "backward end mismatch: {line}"
                );
            } else {
                assert_eq!(kind, "path");
                let end: usize = fields.next().unwrap().parse().unwrap();
                let state: usize = fields.next().unwrap().parse().unwrap();
                assert_eq!(alignment.occurrences[index].end, end);
                assert_eq!(alignment.occurrences[index].state, state);
                path_count += 1;
            }
        }
        assert_eq!(alignment.occurrences.len(), path_count);
    }
}

#[test]
fn rejects_invalid_durations_and_impossible_paths() {
    let (mut model, mut segmentation, emissions, options) = sample(0);
    model.durations[0].variance = 0.0;
    assert!(
        model
            .forward_hsmm(&segmentation, &emissions, options)
            .is_err()
    );
    model.durations[0].variance = 0.75;
    model.durations[0].maximum = 1;
    assert!(
        model
            .viterbi_hsmm(&segmentation, &emissions, options)
            .is_err()
    );
    segmentation.boundaries[3] = 11;
    assert!(
        model
            .forward_hsmm(&segmentation, &emissions, options)
            .is_err()
    );
}

#[test]
fn zero_temperature_preserves_duration_and_transition_exclusions() {
    let (mut model, mut segmentation, emissions, mut options) = sample(0);
    options.temperature = 0.0;
    model.durations[1].minimum = 13;
    assert!(
        model
            .viterbi_hsmm(&segmentation, &emissions, options)
            .is_err()
    );
    model.durations[1].minimum = 2;
    segmentation.add_jump(0, 1, 0.0).unwrap();
    assert!(
        model
            .viterbi_hsmm(&segmentation, &emissions, options)
            .is_err()
    );
}

#[test]
fn backward_checks_destinations_against_states_instead_of_frames() {
    let model = Model {
        streams: vec![],
        durations: vec![
            Duration {
                mean: 1.0,
                variance: 1.0,
                maximum: 2,
                ..Duration::default()
            };
            4
        ],
    };
    let mut segmentation = Segmentation::new(0, 4);
    segmentation.boundaries = vec![0, 0, 1, 2];
    segmentation.duration_states = vec![0, 1, 2, 3];
    segmentation.add_jump(0, 1, 0.0).unwrap();
    segmentation.add_jump(0, 3, 0.5).unwrap();
    let emissions = EmissionTable::from_log_values(2, 4, vec![0.0; 8]).unwrap();
    let options = HsmmOptions::default();
    let backward = model
        .backward_hsmm(&segmentation, &emissions, options)
        .unwrap();
    assert_eq!(backward.end.get(0, 0), Some(0.5f64.ln() as f32));
    segmentation.add_jump(0, -100, 0.25).unwrap();
    segmentation.add_jump(0, 100, 0.25).unwrap();
    assert_eq!(
        model
            .backward_hsmm(&segmentation, &emissions, options)
            .unwrap(),
        backward
    );
}
