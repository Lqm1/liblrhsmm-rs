use liblrhsmm_rs::data::{ObservationData, ObservationStreamData, SegmentationData};
use liblrhsmm_rs::model::{
    DurationParameters, GaussianMixtureParameters, ModelParameters, StreamParameters,
};
use liblrhsmm_rs::scalar::Scalar;
use liblrhsmm_rs::{Model, Observation, Segmentation};

pub fn sample() -> (Model, Observation, Segmentation) {
    sample_with_precision::<f32>()
}

pub fn sample_with_precision<T: Scalar>()
-> (ModelParameters<T>, ObservationData<T>, SegmentationData<T>) {
    let model = ModelParameters {
        streams: [2, 1]
            .into_iter()
            .enumerate()
            .map(|(stream, dimensions)| StreamParameters {
                weight: T::from_f64(if stream == 0 { 1.0 } else { 0.5 }),
                mixtures: (0..3)
                    .map(|state| {
                        let mut gaussian = GaussianMixtureParameters::new(
                            if stream == 0 { 2 } else { 1 },
                            dimensions,
                        )
                        .unwrap();
                        for index in 0..gaussian.means.len() {
                            gaussian.means[index] = T::from_f64(
                                state as f64 * 0.5 + index as f64 * 0.25 + stream as f64,
                            );
                            gaussian.variances[index] = T::from_f64(0.75 + index as f64 * 0.25);
                        }
                        gaussian
                    })
                    .collect(),
            })
            .collect(),
        durations: vec![
            DurationParameters {
                mean: T::from_f64(4.0),
                variance: T::ONE,
                ..DurationParameters::default()
            };
            3
        ],
    };
    // Preserve uniform component weights from both constructors.
    model.validate().unwrap();
    let observation = ObservationData {
        frames: 12,
        streams: [2, 1]
            .into_iter()
            .enumerate()
            .map(|(stream, dimensions)| ObservationStreamData {
                dimensions,
                values: (0..12)
                    .flat_map(|time| {
                        (0..dimensions).map(move |dimension| {
                            T::from_f64(
                                time as f64 * 0.125 + dimension as f64 * 0.5 + stream as f64,
                            )
                        })
                    })
                    .collect(),
            })
            .collect(),
    };
    let mut segmentation = SegmentationData::new(2, 3);
    segmentation.boundaries = vec![4, 8, 12];
    segmentation.duration_states = vec![0, 1, 2];
    segmentation.output_states = vec![vec![0, 1, 2], vec![2, 1, 0]];
    (model, observation, segmentation)
}
