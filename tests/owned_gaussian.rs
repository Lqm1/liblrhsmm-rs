use liblrhsmm_rs::model::GaussianMixtureParameters;
use liblrhsmm_rs::scalar::Scalar;

fn bits<T: Scalar>(value: T) -> u64 {
    let value: f64 = value.into();
    value.to_bits()
}
fn check<T: Scalar>() {
    for components in [1, 2, 5] {
        for dimensions in [1, 2, 4] {
            let mut source = GaussianMixtureParameters::<T>::new(components, dimensions).unwrap();
            for (index, mean) in source.means.iter_mut().enumerate() {
                *mean = T::from_f64(index as f64 * 0.3 - 1.25);
            }
            for (index, variance) in source.variances.iter_mut().enumerate() {
                *variance = T::from_f64(0.5 + index as f64 * 0.2);
            }
            source.weights[0] = T::ZERO;
            let borrowed = source.prepare().unwrap();
            let owned = source.clone().prepare_owned().unwrap();
            assert_eq!(
                owned
                    .cached_log_normalizers()
                    .iter()
                    .copied()
                    .map(bits)
                    .collect::<Vec<_>>(),
                borrowed
                    .cached_log_normalizers()
                    .iter()
                    .copied()
                    .map(bits)
                    .collect::<Vec<_>>()
            );
            for value in [-1000.0, -0.0, 0.25, 3.75, 1000.0] {
                let observation = vec![T::from_f64(value); dimensions];
                for component in 0..components {
                    assert_eq!(
                        bits(
                            owned
                                .component_log_probability(&observation, component)
                                .unwrap()
                        ),
                        bits(
                            borrowed
                                .component_log_probability(&observation, component)
                                .unwrap()
                        )
                    );
                    assert_eq!(
                        owned
                            .component_probability(&observation, component)
                            .unwrap()
                            .to_bits(),
                        borrowed
                            .component_probability(&observation, component)
                            .unwrap()
                            .to_bits()
                    );
                }
                for temperature in [0.0, 0.5, 1.0, 1.75] {
                    let temperature = T::from_f64(temperature);
                    assert_eq!(
                        bits(owned.log_probability(&observation, temperature).unwrap()),
                        bits(borrowed.log_probability(&observation, temperature).unwrap())
                    );
                    assert_eq!(
                        owned
                            .probability(&observation, temperature)
                            .unwrap()
                            .to_bits(),
                        borrowed
                            .probability(&observation, temperature)
                            .unwrap()
                            .to_bits()
                    );
                }
            }
            let cloned = owned.clone();
            source.means.fill(T::from_f64(999.0));
            drop(source);
            let observation = vec![T::ZERO; dimensions];
            assert_eq!(
                bits(cloned.log_probability(&observation, T::ONE).unwrap()),
                bits(owned.log_probability(&observation, T::ONE).unwrap())
            );
            let original = owned.parameters().clone();
            let mut recovered = owned.into_parameters();
            assert_eq!(recovered, original);
            recovered.means.fill(T::from_f64(-999.0));
            assert_eq!(cloned.parameters(), &original);
            assert!(cloned.log_probability(&[], T::ONE).is_err());
            assert!(
                cloned
                    .component_probability(&observation, components)
                    .is_err()
            );
            assert!(cloned.probability(&observation, T::from_f64(-1.0)).is_err());
        }
    }
}
#[test]
fn binary32_owned_gaussians_match_borrowed_kernels_and_preserve_snapshots() {
    check::<f32>();
}
#[test]
fn binary64_owned_gaussians_match_borrowed_kernels_and_preserve_snapshots() {
    check::<f64>();
}
#[test]
fn owned_gaussians_reject_the_same_uninitialized_and_invalid_parameters() {
    let mut parameters = GaussianMixtureParameters::<f64>::new(1, 1).unwrap();
    assert!(parameters.clone().prepare_owned().is_err());
    for variance in [-1.0, f64::INFINITY, f64::NAN] {
        parameters.variances[0] = variance;
        assert!(parameters.clone().prepare_owned().is_err());
    }
    parameters.variances[0] = 1.0;
    parameters.means[0] = f64::NAN;
    assert!(parameters.clone().prepare_owned().is_err());
    parameters.means[0] = 0.0;
    parameters.weights[0] = -1.0;
    assert!(parameters.prepare_owned().is_err());
}
