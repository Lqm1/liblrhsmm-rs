use liblrhsmm_rs::data::{ObservationData, ObservationStreamData, SegmentationData};
use liblrhsmm_rs::model::{
    DurationParameters, GaussianMixtureParameters, ModelParameters, StreamParameters,
};
use liblrhsmm_rs::occupancy::StateOccupancyTable;
use liblrhsmm_rs::scalar::Scalar;
use liblrhsmm_rs::{Model, Observation, OccupancyTable, Segmentation};

pub fn sample() -> (Model, Observation, Segmentation, OccupancyTable) {
    sample_with_precision::<f32>()
}

type MixtureSample<T> = (
    ModelParameters<T>,
    ObservationData<T>,
    SegmentationData<T>,
    StateOccupancyTable<T>,
);
pub fn sample_with_precision<T: Scalar>() -> MixtureSample<T> {
    let mixtures = (0..3)
        .map(|state| {
            let count = [2, 3, 1][state];
            let mut mixture = GaussianMixtureParameters::new(count, 2).unwrap();
            for index in 0..mixture.means.len() {
                mixture.means[index] = T::from_f64(state as f64 * 0.5 + index as f64 * 0.25);
                mixture.variances[index] = T::from_f64(0.75 + index as f64 * 0.25);
            }
            for component in 0..count {
                mixture.weights[component] = T::from_f64(
                    (component as f64 + 1.0) / (count as f64 * (count as f64 + 1.0) / 2.0),
                );
            }
            mixture
        })
        .collect();
    let model = ModelParameters {
        streams: vec![StreamParameters {
            weight: T::ONE,
            mixtures,
        }],
        durations: vec![
            DurationParameters {
                mean: T::from_f64(4.0),
                variance: T::ONE,
                ..DurationParameters::default()
            };
            3
        ],
    };
    let observation = ObservationData {
        frames: 8,
        streams: vec![ObservationStreamData {
            dimensions: 2,
            values: (0..8)
                .flat_map(|frame| {
                    (0..2).map(move |dimension| {
                        T::from_f64(frame as f64 * 0.125 + dimension as f64 * 0.5)
                    })
                })
                .collect(),
        }],
    };
    let mut segmentation = SegmentationData::new(1, 3);
    segmentation.boundaries = vec![2, 5, 8];
    segmentation.output_states[0] = vec![0, 2, 1];
    let occupancy = StateOccupancyTable::from_values(
        8,
        3,
        (0..8)
            .flat_map(|frame| {
                (0..3).map(move |state| {
                    if frame % 3 == 0 {
                        T::ZERO
                    } else {
                        T::from_f64(0.1)
                            + T::from_f64(0.1) * T::from_f64(state as f64)
                            + T::from_f64(0.05) * T::from_f64(frame as f64)
                    }
                })
            })
            .collect(),
    )
    .unwrap();
    (model, observation, segmentation, occupancy)
}
