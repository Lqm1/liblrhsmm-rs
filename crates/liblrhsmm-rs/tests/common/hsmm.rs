use liblrhsmm_rs::{EmissionTable, HsmmOptions, Model, Segmentation};

pub fn sample(scenario: usize) -> (Model, Segmentation, EmissionTable, HsmmOptions) {
    sample_with_precision::<f32>(scenario)
}

pub fn sample_with_precision<T: liblrhsmm_rs::scalar::Scalar>(
    scenario: usize,
) -> (
    liblrhsmm_rs::model::ModelParameters<T>,
    liblrhsmm_rs::data::SegmentationData<T>,
    liblrhsmm_rs::emission::LogEmissionTable<T>,
    liblrhsmm_rs::hsmm::HsmmSettings<T>,
) {
    let model = liblrhsmm_rs::model::ModelParameters {
        streams: vec![],
        durations: (0..4)
            .map(|state| liblrhsmm_rs::model::DurationParameters {
                mean: T::from_f64(2.5 + state as f64 * 0.25),
                variance: T::from_f64(0.75 + state as f64 * 0.25),
                minimum: if state == 1 { 2 } else { -1 },
                maximum: if scenario == 3 && state == 1 { 3 } else { 7 },
                ..liblrhsmm_rs::model::DurationParameters::default()
            })
            .collect(),
    };
    let mut segmentation = liblrhsmm_rs::data::SegmentationData::new(0, 4);
    segmentation.boundaries = vec![3, 6, 9, 12];
    segmentation.duration_states = vec![0, 1, 2, 3];
    if scenario > 0 {
        for (source, delta, probability) in [(0, 2, 0.2), (1, 0, 0.15), (2, -1, 0.1)] {
            segmentation
                .add_jump(source, delta, T::from_f64(probability))
                .unwrap();
        }
        for state in 0..3 {
            segmentation.add_jump(state, 1, T::from_f64(0.8)).unwrap();
        }
    }
    let values = (0..12)
        .flat_map(|frame| {
            (0..4).map(move |state| {
                if scenario == 3 {
                    let desired = [0, 0, 2, 2, 1, 1, 1, 1, 2, 3, 3, 3];
                    return T::from_f64(if state == desired[frame] { 0.0 } else { -50.0 });
                }
                let residual = frame as f64 - state as f64 * 3.0;
                T::from_f64(-residual * residual * 0.125)
            })
        })
        .collect();
    let options = liblrhsmm_rs::hsmm::HsmmSettings {
        temperature: T::from_f64(if scenario == 2 { 0.5 } else { 1.0 }),
        duration_weight: T::from_f64(0.75),
        state_radius: T::from_f64(if scenario == 2 { 1.0 } else { 5.0 }),
        duration_extra: 3,
        duration_extra_factor: T::from_f64(0.5),
    };
    (
        model,
        segmentation,
        liblrhsmm_rs::emission::LogEmissionTable::from_log_values(12, 4, values).unwrap(),
        options,
    )
}
