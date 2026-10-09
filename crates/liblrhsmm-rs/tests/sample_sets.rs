#[cfg(feature = "serialization")]
use liblrhsmm_rs::serial::{
    read_observation_set, read_segmentation_set, write_observation_set, write_segmentation_set,
};
use liblrhsmm_rs::{Observation, ObservationSetBuilder, Segmentation, SegmentationSetBuilder};

#[cfg(feature = "serialization")]
#[test]
fn incremental_sample_sets_preserve_c_order_and_serialization() {
    let observation_bytes = include_bytes!("../../../tests/fixtures/c-observation-set.bin");
    let observations = read_observation_set(observation_bytes.as_slice()).unwrap();
    let mut builder = ObservationSetBuilder::new(observations.len()).unwrap();
    assert!(builder.clone().build().is_err());
    for (index, sample) in observations.iter().enumerate().rev() {
        builder.samples[index] = Some(sample.clone());
    }
    let built = builder.build().unwrap();
    assert_eq!(built, observations);
    let mut encoded = Vec::new();
    write_observation_set(&built, &mut encoded).unwrap();
    assert_eq!(encoded, observation_bytes);

    let segmentation_bytes = include_bytes!("../../../tests/fixtures/c-segmentation-set.bin");
    let segmentations = read_segmentation_set(segmentation_bytes.as_slice()).unwrap();
    let mut builder = SegmentationSetBuilder::new(segmentations.len()).unwrap();
    assert!(builder.clone().build().is_err());
    for (index, sample) in segmentations.iter().enumerate().rev() {
        builder.samples[index] = Some(sample.clone());
    }
    let built = builder.build().unwrap();
    assert_eq!(built, segmentations);
    encoded.clear();
    write_segmentation_set(&built, &mut encoded).unwrap();
    assert_eq!(encoded, segmentation_bytes);
}

#[test]
fn sample_builders_reject_unfilled_and_malformed_samples() {
    assert!(
        ObservationSetBuilder::new(0)
            .unwrap()
            .build()
            .unwrap()
            .is_empty()
    );
    assert!(
        SegmentationSetBuilder::new(0)
            .unwrap()
            .build()
            .unwrap()
            .is_empty()
    );
    assert!(ObservationSetBuilder::new(usize::MAX).is_err());
    assert!(SegmentationSetBuilder::new(usize::MAX).is_err());
    let mut observations = vec![Observation::new(3, &[2, 1]).unwrap(); 2];
    observations[0].streams[0].values.pop();
    let mut builder = ObservationSetBuilder::new(1).unwrap();
    builder.samples[0] = Some(observations.remove(0));
    assert!(builder.build().is_err());

    let mut segmentations = vec![Segmentation::new(2, 3); 2];
    segmentations[0].duration_states.pop();
    let mut builder = SegmentationSetBuilder::new(2).unwrap();
    builder.samples[0] = Some(segmentations.remove(0));
    assert!(builder.clone().build().is_err());
    builder.samples[1] = Some(segmentations.remove(0));
    assert!(builder.build().is_err());
}
