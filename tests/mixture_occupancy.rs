use liblrhsmm_rs::{MixtureOccupancyMode, OccupancyTable};
#[path = "common/mixtures.rs"]
mod common;
use common::sample;

#[test]
fn component_posteriors_match_c_for_hsmm_and_hmm_modes() {
    let (model, observation, segmentation, occupancy) = sample();
    let fixture = include_str!("fixtures/c-mixture-occupancy.txt");
    assert_eq!(fixture.lines().count(), 96);
    for temperature in [0.5, 1.0] {
        for mode_index in 0..2 {
            let mode = if mode_index == 0 {
                MixtureOccupancyMode::Hsmm { state_radius: 1.0 }
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
    let mode = MixtureOccupancyMode::Hsmm { state_radius: 1.0 };
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
