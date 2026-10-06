use liblrhsmm_rs::{Duration, GaussianMixture};

#[test]
fn transformations_and_duration_density_match_c() {
    let duration = Duration {
        mean: 4.25,
        variance: 2.5,
        minimum: 2,
        maximum: 6,
        ..Duration::default()
    };
    let mut mixture = GaussianMixture::new(1, 2).unwrap();
    mixture.means = vec![1.25, -2.5];
    mixture.variances = vec![2.5, 0.75];
    mixture.variance_floors = vec![0.1, 0.2];
    mixture.increase_mixtures(5, 0.2).unwrap();
    for line in include_str!("fixtures/c-parameters.txt").lines() {
        let mut fields = line.split_whitespace();
        let kind = fields.next().unwrap();
        let index: usize = fields.next().unwrap().parse().unwrap();
        let expected: Vec<f32> = fields.map(|value| value.parse().unwrap()).collect();
        let actual = match kind {
            "duration" => vec![duration.log_probability(index as f32, 0.75).unwrap()],
            "mixture" => {
                let mut values = vec![mixture.weights[index]];
                for dimension in 0..2 {
                    let offset = index * 2 + dimension;
                    values.extend([
                        mixture.means[offset],
                        mixture.variances[offset],
                        mixture.variance_floors[offset],
                    ]);
                }
                values
            }
            _ => panic!("unknown oracle record"),
        };
        assert_eq!(actual, expected, "C mismatch: {line}");
    }
}

#[test]
fn combination_preserves_dimension_order_and_floors() {
    let mut first = GaussianMixture::new(1, 2).unwrap();
    first.means = vec![2.0, 3.0];
    first.variances = vec![4.0, 5.0];
    first.variance_floors = vec![0.1, 0.2];
    let mut second = GaussianMixture::new(1, 1).unwrap();
    second.means[0] = 6.0;
    second.variances[0] = 7.0;
    second.variance_floors[0] = 0.3;
    let combined = GaussianMixture::combine(&[first, second]).unwrap();
    assert_eq!(combined.means, [2.0, 3.0, 6.0]);
    assert_eq!(combined.variances, [4.0, 5.0, 7.0]);
    assert_eq!(combined.variance_floors, [0.1, 0.2, 0.3]);
    assert_eq!(combined.weights, [1.0]);
    let durations = [Duration {
        mean: 2.0,
        variance: 3.0,
        variance_floor: 0.4,
        ..Duration::default()
    }];
    let combined = GaussianMixture::from_durations(&durations).unwrap();
    assert_eq!(combined.means, [2.0]);
    assert_eq!(combined.variances, [3.0]);
    assert_eq!(combined.variance_floors, [0.0]); // Upstream does not copy duration floors.
    assert!(GaussianMixture::combine(&[GaussianMixture::new(2, 1).unwrap()]).is_err());
    assert!(GaussianMixture::combine(&[]).is_err());
}

#[test]
fn invalid_split_is_rejected_without_changing_parameters() {
    let mut mixture = GaussianMixture::new(1, 1).unwrap();
    mixture.variances[0] = -1.0;
    let before = mixture.clone();
    assert!(mixture.increase_mixtures(2, 0.2).is_err());
    assert_eq!(mixture, before);
    assert!(Duration::default().log_probability(3.0, 1.0).is_err());
}
