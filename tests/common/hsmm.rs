use liblrhsmm_rs::{Duration, EmissionTable, HsmmOptions, Model, Segmentation};

pub fn sample(scenario: usize) -> (Model, Segmentation, EmissionTable, HsmmOptions) {
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
