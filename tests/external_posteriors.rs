use liblrhsmm_rs::expectation::{GeometricPosteriorExpectation, HsmmPosteriorExpectation};
use liblrhsmm_rs::occupancy::StateOccupancyTable;
use liblrhsmm_rs::scalar::Scalar;
use liblrhsmm_rs::statistics::ModelSufficientStatistics;
use liblrhsmm_rs::{
    DurationOccupancies, DurationPosterior, MixtureOccupancies, TransitionOccupancies,
    TransitionPosterior,
};
#[path = "common/estimation.rs"]
#[allow(dead_code)]
mod common;

fn collect_external<T: Scalar>() {
    let (model, observation, segmentation) = common::sample_with_precision::<T>();
    let frames = observation.frames;
    let states = segmentation.boundaries.len();
    let index = states; // Frame one, state zero.
    let mut duration_values = vec![None; frames * states];
    duration_values[index] = Some(DurationPosterior {
        minimum: 2,
        probabilities: vec![2.0, 3.0],
    });
    let durations = DurationOccupancies::from_values(frames, states, duration_values).unwrap();
    let mut transition_values = vec![None; frames * states];
    transition_values[index] = Some(TransitionPosterior {
        stay: 1.5,
        advance: 2.5,
    });
    let transitions =
        TransitionOccupancies::from_values(frames, states, transition_values).unwrap();
    let mut state_values = vec![T::ZERO; frames * states];
    state_values[index] = T::from_f64(4.0);
    let occupancy = StateOccupancyTable::from_values(frames, states, state_values).unwrap();
    let mixtures: Vec<_> = [vec![2.0, 3.0], vec![5.0]]
        .into_iter()
        .map(|weights| {
            let mut values = vec![None; frames * states];
            values[index] = Some(weights);
            MixtureOccupancies::from_values(frames, states, values).unwrap()
        })
        .collect();

    for geometric in [false, true] {
        let mut statistics = ModelSufficientStatistics::from_model(&model).unwrap();
        if geometric {
            statistics
                .collect_geometric(
                    &model,
                    &observation,
                    &segmentation,
                    GeometricPosteriorExpectation {
                        transitions: &transitions,
                        states: &occupancy,
                        mixtures: &mixtures,
                        pruning_slope: T::from_f64(0.5),
                    },
                )
                .unwrap();
        } else {
            statistics
                .collect_hsmm(
                    &model,
                    &observation,
                    &segmentation,
                    HsmmPosteriorExpectation {
                        durations: &durations,
                        states: &occupancy,
                        mixtures: &mixtures,
                        state_radius: T::ONE,
                    },
                )
                .unwrap();
        }
        let duration = statistics.durations[0];
        assert_eq!(
            duration.value_sum,
            T::from_f64(if geometric { 2.5 } else { 13.0 })
        );
        assert_eq!(
            duration.occupancy,
            T::from_f64(if geometric { 4.0 } else { 5.0 })
        );
        assert_eq!(
            duration.squared_sum,
            T::from_f64(if geometric { 0.0 } else { 35.0 })
        );
        for (stream_index, weights) in [vec![2.0, 3.0], vec![5.0]].iter().enumerate() {
            let mapped = segmentation.output_states[stream_index][0] as usize;
            let distribution = &statistics.streams[stream_index].mixtures[mapped];
            assert_eq!(distribution.state_occupancy, T::from_f64(4.0));
            for (component, &weight) in weights.iter().enumerate() {
                assert_eq!(
                    distribution.component_occupancies[component],
                    T::from_f64(weight)
                );
                for (dimension, &feature) in observation
                    .frame(stream_index, 1)
                    .unwrap()
                    .iter()
                    .enumerate()
                {
                    let offset = component * distribution.dimensions + dimension;
                    assert_eq!(
                        distribution.weighted_sums[offset],
                        T::from_f64(weight) * feature
                    );
                    assert_eq!(
                        distribution.weighted_squared_sums[offset],
                        T::from_f64(weight) * feature * feature
                    );
                }
            }
        }
        // Imported mixture cardinalities are validated against the model before mutation.
        let mut invalid = mixtures.clone();
        invalid[0] =
            MixtureOccupancies::from_values(frames, states, vec![Some(vec![1.0]); frames * states])
                .unwrap();
        let before = statistics.clone();
        assert!(
            statistics
                .collect_hsmm(
                    &model,
                    &observation,
                    &segmentation,
                    HsmmPosteriorExpectation {
                        durations: &durations,
                        states: &occupancy,
                        mixtures: &invalid,
                        state_radius: T::ONE,
                    }
                )
                .is_err()
        );
        assert_eq!(statistics, before);
    }
}

#[test]
fn external_binary32_posteriors_reach_both_collectors() {
    collect_external::<f32>();
}

#[test]
fn external_binary64_posteriors_retain_float_slices() {
    collect_external::<f64>();
}

#[test]
fn invalid_shapes_weights_and_duration_overflow_are_rejected() {
    for (frames, states) in [(0, 1), (1, 0), (2, 1), (usize::MAX, 2)] {
        assert!(DurationOccupancies::from_values(frames, states, vec![None]).is_err());
        assert!(MixtureOccupancies::from_values(frames, states, vec![None]).is_err());
        assert!(TransitionOccupancies::from_values(frames, states, vec![None]).is_err());
    }
    for value in [-1.0, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        assert!(
            DurationOccupancies::from_values(
                1,
                1,
                vec![Some(DurationPosterior {
                    minimum: 1,
                    probabilities: vec![value],
                })]
            )
            .is_err()
        );
        assert!(MixtureOccupancies::from_values(1, 1, vec![Some(vec![value])]).is_err());
        for posterior in [
            TransitionPosterior {
                stay: value,
                advance: 0.0,
            },
            TransitionPosterior {
                stay: 0.0,
                advance: value,
            },
        ] {
            assert!(TransitionOccupancies::from_values(1, 1, vec![Some(posterior)]).is_err());
        }
    }
    assert!(
        DurationOccupancies::from_values(
            1,
            1,
            vec![Some(DurationPosterior {
                minimum: usize::MAX,
                probabilities: vec![1.0, 2.0],
            })]
        )
        .is_err()
    );
}

#[test]
fn imports_preserve_absent_empty_slices_and_signed_zero() {
    let values = vec![
        None,
        Some(DurationPosterior {
            minimum: usize::MAX,
            probabilities: vec![],
        }),
        Some(DurationPosterior {
            minimum: usize::MAX,
            probabilities: vec![-0.0],
        }),
    ];
    let duration = DurationOccupancies::from_values(1, 3, values.clone()).unwrap();
    assert_eq!(duration.values(), values);
    assert_eq!(
        duration
            .get(0, 2)
            .unwrap()
            .probability(usize::MAX)
            .to_bits(),
        (-0.0_f32).to_bits()
    );
    let mixture =
        MixtureOccupancies::from_values(1, 3, vec![None, Some(vec![]), Some(vec![-0.0])]).unwrap();
    assert_eq!(mixture.get(0, 0), None);
    assert_eq!(mixture.get(0, 1), Some([].as_slice()));
    assert_eq!(
        mixture.get(0, 2).unwrap()[0].to_bits(),
        (-0.0_f32).to_bits()
    );
    let transition = TransitionOccupancies::from_values(
        1,
        1,
        vec![Some(TransitionPosterior {
            stay: -0.0,
            advance: 2.0,
        })],
    )
    .unwrap();
    assert_eq!(
        transition.get(0, 0).unwrap().stay.to_bits(),
        (-0.0_f32).to_bits()
    );
    assert_eq!(transition.get(1, 0), None);
}
