use liblrhsmm_rs::{
    Duration, GaussianMixture, Model, Observation, ObservationStream, Segmentation, Stream,
};

pub fn sample() -> (Model, Observation, Segmentation) {
    let mut streams = Vec::new();
    for stream in 0..2 {
        let mut distributions = Vec::new();
        for state in 0..3 {
            let mut gmm = GaussianMixture::new(
                if stream == 0 { 2 } else { 1 },
                if stream == 0 { 2 } else { 1 },
            )
            .unwrap();
            for index in 0..gmm.means.len() {
                gmm.means[index] = state as f32 * 0.5 + index as f32 * 0.25;
                gmm.variances[index] = 0.75 + index as f32 * 0.25;
            }
            distributions.push(gmm);
        }
        streams.push(Stream {
            weight: if stream == 0 { 0.5 } else { 1.25 },
            mixtures: distributions,
        });
    }
    let model = Model {
        streams,
        durations: vec![
            Duration {
                mean: 4.0,
                variance: 1.0,
                ..Duration::default()
            };
            3
        ],
    };
    let observation = Observation {
        frames: 12,
        streams: (0..2)
            .map(|stream| {
                let dimensions = if stream == 0 { 2 } else { 1 };
                ObservationStream {
                    dimensions,
                    values: (0..12)
                        .flat_map(|frame| {
                            (0..dimensions)
                                .map(move |dimension| frame as f32 * 0.125 + dimension as f32 * 0.5)
                        })
                        .collect(),
                }
            })
            .collect(),
    };
    let mut segmentation = Segmentation::new(2, 3);
    segmentation.boundaries = vec![4, 8, 12];
    segmentation.duration_states = vec![0, 1, 2];
    segmentation.output_states = vec![vec![0, 1, 2], vec![2, 1, 0]];
    (model, observation, segmentation)
}
