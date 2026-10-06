use liblrhsmm_rs::{DurationPosterior, OccupancyTable};
#[path = "common/hsmm.rs"]
mod common;

#[test]
fn duration_and_state_posteriors_match_corrected_c_initialization() {
    let fixture = include_str!("fixtures/corrected-c-duration-occupancy.txt");
    assert_eq!(fixture.lines().count(), 192);
    for scenario in 0..4 {
        let (model, segmentation, emissions, options) = common::sample(scenario);
        let forward = model
            .forward_hsmm(&segmentation, &emissions, options)
            .unwrap();
        let backward = model
            .backward_hsmm(&segmentation, &emissions, options)
            .unwrap();
        let total = forward.end.total_through_state(&backward.end, 0).unwrap();
        let duration = model
            .duration_occupancies(
                &segmentation,
                &emissions,
                &forward,
                &backward,
                total,
                options,
            )
            .unwrap();
        let occupancy = model
            .hsmm_state_occupancies(&segmentation, &emissions, &duration, options)
            .unwrap();
        assert_eq!(occupancy.get(0, 0), Some((1.0f64 / (1.0 + 1e-7)) as f32));
        for line in fixture.lines() {
            let mut fields = line.split_whitespace();
            let case: usize = fields.next().unwrap().parse().unwrap();
            if case != scenario {
                continue;
            }
            let frame: usize = fields.next().unwrap().parse().unwrap();
            let state: usize = fields.next().unwrap().parse().unwrap();
            let minimum: usize = fields.next().unwrap().parse().unwrap();
            let count: usize = fields.next().unwrap().parse().unwrap();
            let expected: Vec<f32> = (0..count)
                .map(|_| fields.next().unwrap().parse().unwrap())
                .collect();
            let state_probability: f32 = fields.next().unwrap().parse().unwrap();
            let actual = duration.get(frame, state);
            assert_eq!(
                actual.is_some(),
                count != 0,
                "slice presence mismatch: {line}"
            );
            if let Some(actual) = actual {
                assert_eq!(actual.minimum, minimum);
                assert_eq!(actual.probabilities, expected, "duration mismatch: {line}");
            }
            assert_eq!(
                occupancy.get(frame, state),
                Some(state_probability),
                "state mismatch: {line}"
            );
        }
    }
}

#[test]
fn invalid_normalizers_and_empty_slices_are_handled() {
    let (mut model, segmentation, emissions, options) = common::sample(0);
    let forward = model
        .forward_hsmm(&segmentation, &emissions, options)
        .unwrap();
    let backward = model
        .backward_hsmm(&segmentation, &emissions, options)
        .unwrap();
    assert!(
        model
            .duration_occupancies(
                &segmentation,
                &emissions,
                &forward,
                &backward,
                f32::NAN,
                options
            )
            .is_err()
    );
    let slice = DurationPosterior {
        minimum: 2,
        probabilities: vec![0.25, 0.75],
    };
    assert_eq!(slice.probability(1), 0.0);
    assert_eq!(slice.probability(2), 0.25);
    assert_eq!(slice.probability(4), 0.0);
    model.durations[0].maximum = 1;
    let duration = model
        .duration_occupancies(&segmentation, &emissions, &forward, &backward, 0.0, options)
        .unwrap();
    assert_eq!(duration.get(0, 0), None);
    assert!(OccupancyTable::from_values(0, 1, vec![]).is_err());
}
