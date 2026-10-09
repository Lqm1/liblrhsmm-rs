use liblrhsmm_rs::model::GaussianMixtureParameters;
use liblrhsmm_rs::scalar::Scalar;
use liblrhsmm_rs::statistics::GaussianSufficientStatistics;
fn check<T: Scalar>() {
    for (mixtures, dimensions) in [(1, 1), (2, 3), (5, 4)] {
        let direct = GaussianSufficientStatistics::<T>::new(mixtures, dimensions).unwrap();
        let source = GaussianMixtureParameters::<T>::new(mixtures, dimensions).unwrap();
        assert_eq!(
            direct,
            GaussianSufficientStatistics::from_gaussian(&source).unwrap()
        );
        assert_eq!(direct.weighted_sums.len(), mixtures * dimensions);
    }
    for (mixtures, dimensions) in [
        (0, 1),
        (1, 0),
        (usize::MAX, 1),
        (1, usize::MAX),
        (i32::MAX as usize, i32::MAX as usize),
    ] {
        assert!(GaussianSufficientStatistics::<T>::new(mixtures, dimensions).is_err());
    }
}
#[test]
fn direct_statistics_allocation_matches_native_model_shapes_in_both_precisions() {
    check::<f32>();
    check::<f64>();
}
