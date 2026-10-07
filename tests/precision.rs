use liblrhsmm_rs::data::{JumpParameters, ObservationData, SegmentationData};
use liblrhsmm_rs::model::{
    DurationParameters, GaussianMixtureParameters, ModelParameters, ModelParametersBuilder,
    StreamLayout, StreamParametersBuilder,
};
use liblrhsmm_rs::scalar::Scalar;

fn check_source<T: Scalar>(fixture: &str) {
    let duration = DurationParameters::<T> {
        mean: T::from_f64(4.25),
        variance: T::from_f64(2.5),
        minimum: 2,
        maximum: 6,
        ..Default::default()
    };
    let mut mixture = GaussianMixtureParameters::<T>::new(1, 2).unwrap();
    mixture.means = vec![T::from_f64(1.25), T::from_f64(-2.5)];
    mixture.variances = vec![T::from_f64(2.5), T::from_f64(0.75)];
    mixture.variance_floors = vec![T::from_f64(0.1), T::from_f64(0.2)];
    mixture.increase_mixtures(5, T::from_f64(0.2)).unwrap();
    let prepared = mixture.prepare().unwrap();
    let temperatures = [0.5, 1.0, 1.75];
    let mut maximum = 0.0f64;
    let mut compared = 0;
    for line in fixture.lines() {
        let mut fields = line.split_whitespace();
        let operation = fields.next().unwrap();
        let index: usize = fields.next().unwrap().parse().unwrap();
        let secondary: usize = fields.next().unwrap().parse().unwrap();
        let expected: Vec<f64> = fields.map(|value| value.parse().unwrap()).collect();
        let observation = [
            T::from_f64(index as f64 * 0.3 - 0.5),
            T::from_f64(index as f64 * -0.2 + 0.75),
        ];
        let actual = match operation {
            "duration" => {
                let weight = T::from_f64(0.75);
                let temperature = T::from_f64(temperatures[secondary]);
                vec![
                    duration
                        .log_probability(T::from_f64(index as f64), weight)
                        .unwrap()
                        .into(),
                    duration
                        .annealed_log_probability(index, weight, temperature)
                        .unwrap()
                        .into(),
                    duration.probability(index, weight, temperature).unwrap(),
                ]
            }
            "mixture" => {
                let mut values = vec![mixture.weights[index].into()];
                for dimension in 0..2 {
                    let offset = index * 2 + dimension;
                    values.extend([
                        mixture.means[offset].into(),
                        mixture.variances[offset].into(),
                        mixture.variance_floors[offset].into(),
                    ]);
                }
                values
            }
            "component" => vec![
                prepared
                    .component_log_probability(&observation, secondary)
                    .unwrap()
                    .into(),
                prepared
                    .component_probability(&observation, secondary)
                    .unwrap(),
            ],
            "output" => {
                let temperature = T::from_f64(temperatures[secondary]);
                vec![
                    prepared
                        .log_probability(&observation, temperature)
                        .unwrap()
                        .into(),
                    prepared.probability(&observation, temperature).unwrap(),
                ]
            }
            _ => panic!("unknown reference operation"),
        };
        assert_eq!(actual.len(), expected.len());
        for (actual, expected) in actual.into_iter().zip(expected) {
            assert!(actual.is_finite());
            let error = (actual - expected).abs() / expected.abs().max(1.0);
            maximum = maximum.max(error);
            compared += 1;
        }
    }
    assert_eq!(compared, 196);
    println!(
        "{}: {compared} source values; maximum normalized error {maximum}",
        std::any::type_name::<T>()
    );
    assert!(
        maximum <= 2e-14,
        "maximum normalized source error {maximum}"
    );
}

#[test]
fn binary32_model_operations_and_densities_match_c() {
    check_source::<f32>(include_str!("fixtures/c-precision-float.txt"));
}

#[test]
fn binary64_model_operations_and_densities_match_c() {
    check_source::<f64>(include_str!("fixtures/c-precision-double.txt"));
}

#[test]
fn binary64_owned_models_and_data_retain_full_parameter_precision() {
    let precise = 1.0 + f64::EPSILON;
    let mut mixture = GaussianMixtureParameters::<f64>::new(1, 1).unwrap();
    mixture.means[0] = precise;
    mixture.variances[0] = 1.0;
    mixture.variance_floors[0] = precise;
    let mut stream = StreamParametersBuilder::<f64>::new(1).unwrap();
    stream.weight = precise;
    stream.emissions[0] = Some(mixture);
    let mut builder = ModelParametersBuilder::<f64>::new(1, 1).unwrap();
    builder.streams[0] = Some(stream.build().unwrap());
    builder.durations[0] = Some(DurationParameters {
        mean: precise,
        variance: 1.0,
        ..Default::default()
    });
    let model = builder.build().unwrap();
    let mut copy = model.clone();
    copy.streams[0].mixtures[0].means[0] = f64::MAX;
    assert_eq!(model.streams[0].mixtures[0].means[0], precise);
    assert_eq!(model.streams[0].weight, precise);
    assert_eq!(model.durations[0].mean, precise);
    assert_eq!(model.streams[0].mixtures[0].variance_floors[0], precise);
    assert!(
        ModelParametersBuilder::<f64>::new(1, 1)
            .unwrap()
            .build()
            .is_err()
    );
    assert!(
        StreamParametersBuilder::<f64>::new(1)
            .unwrap()
            .build()
            .is_err()
    );
    let flat = ModelParameters::<f64>::new(
        &[StreamLayout {
            emissions: 2,
            components: 3,
            dimensions: 1,
        }],
        2,
    )
    .unwrap();
    assert_eq!(flat.streams[0].mixtures[0].weights, [1.0 / 3.0; 3]);
    let mut observation = ObservationData::<f64>::new(2, &[1]).unwrap();
    observation.streams[0].values = vec![precise, f64::MAX];
    observation.validate().unwrap();
    assert_eq!(observation.frame(0, 0), Some([precise].as_slice()));
    let mut segmentation = SegmentationData::<f64>::new(1, 2);
    segmentation.outgoing[0] = vec![
        JumpParameters {
            delta: 2,
            probability: precise,
        },
        JumpParameters {
            delta: 1,
            probability: 0.5,
        },
    ];
    segmentation.validate().unwrap();
    assert_eq!(segmentation.outgoing[0][0].probability, precise);
}
