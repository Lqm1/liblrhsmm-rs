use liblrhsmm_rs::{Model, Observation, Segmentation};

pub fn sample() -> (Model, Observation, Segmentation) {
    sample_with_precision::<f32>()
}

pub fn sample_with_precision<T: liblrhsmm_rs::scalar::Scalar>() -> (
    liblrhsmm_rs::model::ModelParameters<T>,
    liblrhsmm_rs::data::ObservationData<T>,
    liblrhsmm_rs::data::SegmentationData<T>,
) {
    let mut streams = Vec::new();
    for stream in 0..2 {
        let mut distributions = Vec::new();
        for state in 0..3 {
            let mut gmm = liblrhsmm_rs::model::GaussianMixtureParameters::new(
                if stream == 0 { 2 } else { 1 },
                if stream == 0 { 2 } else { 1 },
            )
            .unwrap();
            for index in 0..gmm.means.len() {
                gmm.means[index] = T::from_f64(state as f64 * 0.5 + index as f64 * 0.25);
                gmm.variances[index] = T::from_f64(0.75 + index as f64 * 0.25);
            }
            distributions.push(gmm);
        }
        streams.push(liblrhsmm_rs::model::StreamParameters {
            weight: T::from_f64(if stream == 0 { 0.5 } else { 1.25 }),
            mixtures: distributions,
        });
    }
    let model = liblrhsmm_rs::model::ModelParameters {
        streams,
        durations: vec![
            liblrhsmm_rs::model::DurationParameters {
                mean: T::from_f64(4.0),
                variance: T::ONE,
                ..liblrhsmm_rs::model::DurationParameters::default()
            };
            3
        ],
    };
    let observation = liblrhsmm_rs::data::ObservationData {
        frames: 12,
        streams: (0..2)
            .map(|stream| {
                let dimensions = if stream == 0 { 2 } else { 1 };
                liblrhsmm_rs::data::ObservationStreamData {
                    dimensions,
                    values: (0..12)
                        .flat_map(|frame| {
                            (0..dimensions).map(move |dimension| {
                                T::from_f64(frame as f64 * 0.125 + dimension as f64 * 0.5)
                            })
                        })
                        .collect(),
                }
            })
            .collect(),
    };
    let mut segmentation = liblrhsmm_rs::data::SegmentationData::new(2, 3);
    segmentation.boundaries = vec![4, 8, 12];
    segmentation.duration_states = vec![0, 1, 2];
    segmentation.output_states = vec![vec![0, 1, 2], vec![2, 1, 0]];
    (model, observation, segmentation)
}
