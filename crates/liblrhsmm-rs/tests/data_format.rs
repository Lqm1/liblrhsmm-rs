#![cfg(feature = "serialization")]

use liblrhsmm_rs::{Jump, Observation, Segmentation, serial::*};
const OBSERVATION: &[u8] = include_bytes!("../../../tests/fixtures/c-observation.bin");
const SEGMENTATION: &[u8] = include_bytes!("../../../tests/fixtures/c-segmentation.bin");
const OBSERVATION_SET: &[u8] = include_bytes!("../../../tests/fixtures/c-observation-set.bin");
const SEGMENTATION_SET: &[u8] = include_bytes!("../../../tests/fixtures/c-segmentation-set.bin");

#[test]
fn preserves_c_observations_and_stream_order() {
    let observation = Observation::read_from(OBSERVATION).unwrap();
    assert_eq!(observation.frames, 3);
    assert_eq!(observation.frame(0, 0), Some([-0.25, 0.75].as_slice()));
    assert_eq!(observation.frame(1, 2), Some([10.25].as_slice()));
    assert_eq!(observation.frame(1, 3), None);
    let mut output = Vec::new();
    observation.write_to(&mut output).unwrap();
    assert_eq!(output, OBSERVATION);
    let samples = read_observation_set(OBSERVATION_SET).unwrap();
    assert_eq!(samples, vec![observation.clone(), observation]);
    output.clear();
    write_observation_set(&samples, &mut output).unwrap();
    assert_eq!(output, OBSERVATION_SET);
}

#[test]
fn preserves_c_segmentation_and_nested_malformed_headers() {
    let segmentation = Segmentation::read_from(SEGMENTATION).unwrap();
    assert_eq!(segmentation.boundaries, [2, 4, 6]);
    assert_eq!(segmentation.output_states[1], [2, 1, 0]);
    assert_eq!(
        segmentation.outgoing[0],
        [
            Jump {
                delta: 2,
                probability: 0.25
            },
            Jump {
                delta: 1,
                probability: 0.75
            }
        ]
    );
    assert_eq!(segmentation.outgoing[2][1].delta, -1);
    let mut output = Vec::new();
    segmentation.write_to(&mut output).unwrap();
    assert_eq!(output, SEGMENTATION);
    let samples = read_segmentation_set(SEGMENTATION_SET).unwrap();
    assert_eq!(samples, vec![segmentation.clone(), segmentation]);
    output.clear();
    write_segmentation_set(&samples, &mut output).unwrap();
    assert_eq!(output, SEGMENTATION_SET);
}

#[test]
fn rejects_corrupted_lengths_and_missing_jump_termination() {
    for length in 0..SEGMENTATION.len() {
        assert!(Segmentation::read_from(&SEGMENTATION[..length]).is_err());
    }
    let mut segmentation = Segmentation::read_from(SEGMENTATION).unwrap();
    segmentation.outgoing[0].pop();
    assert!(segmentation.write_to(Vec::new()).is_err());
    segmentation.outgoing[0].clear();
    assert!(segmentation.validate().is_err());
    let mut observation = Observation::read_from(OBSERVATION).unwrap();
    observation.streams[0].values.pop();
    assert!(observation.write_to(Vec::new()).is_err());
}
