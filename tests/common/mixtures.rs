use liblrhsmm_rs::{
    Duration, GaussianMixture, Model, Observation, ObservationStream, OccupancyTable, Segmentation,
    Stream,
};

pub fn sample() -> (Model, Observation, Segmentation, OccupancyTable) {
    let mixtures = (0..3)
        .map(|state| {
            let count = [2, 3, 1][state];
            let mut mixture = GaussianMixture::new(count, 2).unwrap();
            for index in 0..mixture.means.len() {
                mixture.means[index] = state as f32 * 0.5 + index as f32 * 0.25;
                mixture.variances[index] = 0.75 + index as f32 * 0.25;
            }
            for component in 0..count {
                mixture.weights[component] =
                    (component as f32 + 1.0) / (count as f32 * (count as f32 + 1.0) / 2.0);
            }
            mixture
        })
        .collect();
    let model = Model {
        streams: vec![Stream {
            weight: 1.0,
            mixtures,
        }],
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
        frames: 8,
        streams: vec![ObservationStream {
            dimensions: 2,
            values: (0..8)
                .flat_map(|frame| {
                    (0..2).map(move |dimension| frame as f32 * 0.125 + dimension as f32 * 0.5)
                })
                .collect(),
        }],
    };
    let mut segmentation = Segmentation::new(1, 3);
    segmentation.boundaries = vec![2, 5, 8];
    segmentation.output_states[0] = vec![0, 2, 1];
    let occupancy = OccupancyTable::from_values(
        8,
        3,
        (0..8)
            .flat_map(|frame| {
                (0..3).map(move |state| {
                    if frame % 3 == 0 {
                        0.0
                    } else {
                        0.1 + 0.1 * state as f32 + 0.05 * frame as f32
                    }
                })
            })
            .collect(),
    )
    .unwrap();
    (model, observation, segmentation, occupancy)
}
