use liblrhsmm_rs::{
    Duration, GaussianMixture, Model, Observation, ObservationStream, Segmentation, Stream,
};

pub fn sample() -> (Model, Observation, Segmentation) {
    let model = Model {
        streams: [2, 1]
            .into_iter()
            .enumerate()
            .map(|(stream, dimensions)| Stream {
                weight: if stream == 0 { 1.0 } else { 0.5 },
                mixtures: (0..3)
                    .map(|state| {
                        let mut gaussian =
                            GaussianMixture::new(if stream == 0 { 2 } else { 1 }, dimensions)
                                .unwrap();
                        for index in 0..gaussian.means.len() {
                            gaussian.means[index] =
                                state as f32 * 0.5 + index as f32 * 0.25 + stream as f32;
                            gaussian.variances[index] = 0.75 + index as f32 * 0.25;
                        }
                        gaussian
                    })
                    .collect(),
            })
            .collect(),
        durations: vec![
            Duration {
                mean: 4.0,
                variance: 1.0,
                ..Duration::default()
            };
            3
        ],
    };
    // Preserve uniform component weights from both constructors.
    model.validate().unwrap();
    let observation = Observation {
        frames: 12,
        streams: [2, 1]
            .into_iter()
            .enumerate()
            .map(|(stream, dimensions)| ObservationStream {
                dimensions,
                values: (0..12)
                    .flat_map(|time| {
                        (0..dimensions).map(move |dimension| {
                            time as f32 * 0.125 + dimension as f32 * 0.5 + stream as f32
                        })
                    })
                    .collect(),
            })
            .collect(),
    };
    let mut segmentation = Segmentation::new(2, 3);
    segmentation.boundaries = vec![4, 8, 12];
    segmentation.duration_states = vec![0, 1, 2];
    segmentation.output_states = vec![vec![0, 1, 2], vec![2, 1, 0]];
    (model, observation, segmentation)
}
