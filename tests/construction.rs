#[cfg(feature = "serialization")]
use liblrhsmm_rs::serial::{read_segmentation_set, write_segmentation_set};
use liblrhsmm_rs::{
    Duration, GaussianMixture, Model, ModelBuilder, Segmentation, Stream, StreamBuilder,
    StreamLayout,
};

#[test]
fn flat_start_constructor_matches_c_model_bytes() {
    let model = Model::new(
        &[
            StreamLayout {
                emissions: 2,
                components: 3,
                dimensions: 2,
            },
            StreamLayout {
                emissions: 1,
                components: 2,
                dimensions: 1,
            },
        ],
        3,
    )
    .unwrap();
    #[cfg(feature = "serialization")]
    {
        let reference = include_bytes!("fixtures/c-construction-model.hsmm");
        assert_eq!(model, Model::read_from(reference.as_slice()).unwrap());
        let mut encoded = Vec::new();
        model.write_to(&mut encoded).unwrap();
        assert_eq!(encoded, reference);
    }
    assert_eq!(model.durations, vec![Duration::default(); 3]);
    assert_eq!(model.streams[0].weight, 1.0);
    assert_eq!(model.streams[0].mixtures[0].weights, [1.0 / 3.0; 3]);
}

#[test]
fn incremental_builders_require_filled_slots_and_allow_empty_containers() {
    let mut stream = StreamBuilder::new(2).unwrap();
    assert_eq!(stream.weight, 1.0);
    assert!(stream.clone().build().is_err());
    stream.emissions[0] = Some(GaussianMixture::new(1, 2).unwrap());
    assert!(stream.clone().build().is_err());
    stream.emissions[1] = Some(GaussianMixture::new(3, 2).unwrap());
    let stream = stream.build().unwrap();
    let mut model = ModelBuilder::new(1, 2).unwrap();
    assert!(model.clone().build().is_err());
    model.streams[0] = Some(stream);
    model.durations[0] = Some(Duration::default());
    assert!(model.clone().build().is_err());
    model.durations[1] = Some(Duration::default());
    assert_eq!(
        model.build().unwrap().streams[0].mixtures[1].weights.len(),
        3
    );
    assert_eq!(
        StreamBuilder::new(0).unwrap().build().unwrap(),
        Stream::default()
    );
    assert_eq!(Stream::new(0, 0, 0).unwrap(), Stream::default());
    assert_eq!(
        ModelBuilder::new(0, 0).unwrap().build().unwrap(),
        Model::default()
    );
    assert_eq!(Model::new(&[], 0).unwrap(), Model::default());
    assert!(Stream::new(1, 0, 2).is_err());
    assert!(StreamBuilder::new(usize::MAX).is_err());
    assert!(ModelBuilder::new(usize::MAX, 0).is_err());
    assert!(ModelBuilder::new(0, usize::MAX).is_err());
}

#[test]
fn owned_clones_match_c_copies_and_preserve_independent_storage() {
    let mut gmm = GaussianMixture::new(3, 2).unwrap();
    gmm.weights = vec![0.1, 0.2, 0.7];
    for i in 0..6 {
        gmm.means[i] = i as f32 * 0.5;
        gmm.variances[i] = 1.0 + i as f32 * 0.25;
        gmm.variance_floors[i] = i as f32 * 0.125;
    }
    let copy = gmm.clone();
    gmm.means[0] = 999.0;
    gmm.variance_floors[0] = 998.0;
    gmm.weights[0] = 0.8;
    assert_eq!(copy.means[0], 0.0);
    assert_eq!(copy.variance_floors[0], 0.0);
    assert_eq!(copy.weights[0], 0.1);
    #[cfg(feature = "serialization")]
    {
        let reference = include_bytes!("fixtures/c-copy-gmm.bin");
        assert_eq!(
            copy,
            GaussianMixture::read_from(reference.as_slice()).unwrap()
        );
        let mut encoded = Vec::new();
        copy.write_to(&mut encoded).unwrap();
        assert_eq!(encoded, reference);
    }
    let mut segment = Segmentation::new(2, 3);
    segment.boundaries = vec![2, 4, 6];
    segment.duration_states = vec![3, 4, 5];
    segment.output_states = vec![vec![10, 11, 12], vec![20, 19, 18]];
    segment.add_jump(0, 2, 0.25).unwrap();
    segment.add_jump(0, 1, 0.75).unwrap();
    segment.add_jump(1, 0, 0.1).unwrap();
    segment.add_jump(1, 1, 0.9).unwrap();
    let copy = segment.clone();
    let set = vec![segment.clone(), segment.clone()];
    let mut set_copy = set.clone();
    segment.boundaries[0] = 999;
    segment.output_states[0][0] = 998;
    segment.outgoing[0][0].probability = 0.5;
    assert_eq!(copy.boundaries[0], 2);
    assert_eq!(copy.output_states[0][0], 10);
    assert_eq!(copy.outgoing[0][0].probability, 0.25);
    #[cfg(feature = "serialization")]
    {
        let reference = include_bytes!("fixtures/c-copy-segmentation.bin");
        assert_eq!(copy, Segmentation::read_from(reference.as_slice()).unwrap());
        let mut encoded = Vec::new();
        copy.write_to(&mut encoded).unwrap();
        assert_eq!(encoded, reference);
        let reference = include_bytes!("fixtures/c-copy-segmentation-set.bin");
        assert_eq!(
            set_copy,
            read_segmentation_set(reference.as_slice()).unwrap()
        );
        encoded.clear();
        write_segmentation_set(&set_copy, &mut encoded).unwrap();
        assert_eq!(encoded, reference);
    }
    set_copy[0].outgoing[0][0].probability = 0.125;
    assert_eq!(set[0].outgoing[0][0].probability, 0.25);
    assert_eq!(set_copy[1].outgoing[0][0].probability, 0.25);
}
