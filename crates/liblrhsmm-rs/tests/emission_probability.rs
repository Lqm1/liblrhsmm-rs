use liblrhsmm_rs::GaussianMixture;

fn distribution(mixtures: usize) -> GaussianMixture {
    let mut gmm = GaussianMixture::new(mixtures, 2).unwrap();
    for component in 0..mixtures {
        let component_value = component as f32;
        gmm.weights[component] =
            (component_value + 1.0) / (mixtures as f32 * (mixtures as f32 + 1.0) / 2.0);
        gmm.means[component * 2] = 1.25 + component_value;
        gmm.means[component * 2 + 1] = -2.5 - component_value;
        gmm.variances[component * 2] = 0.75 + component_value;
        gmm.variances[component * 2 + 1] = 2.5 + component_value;
    }
    gmm
}

#[test]
fn matches_c_single_and_multiple_component_likelihoods() {
    let observations = [[0.0, 0.0], [1.25, -2.5], [-3.0, 4.0], [100.0, -100.0]];
    assert_eq!(
        include_str!("../../../tests/fixtures/c-emissions.txt")
            .lines()
            .count(),
        24
    );
    for line in include_str!("../../../tests/fixtures/c-emissions.txt").lines() {
        let mut fields = line.split_whitespace();
        let mixtures: usize = fields.next().unwrap().parse().unwrap();
        let index: usize = fields.next().unwrap().parse().unwrap();
        let temperature: f32 = fields.next().unwrap().parse().unwrap();
        let expected: Vec<f32> = fields.map(|value| value.parse().unwrap()).collect();
        let gmm = distribution(mixtures);
        let prepared = gmm.prepare().unwrap();
        let mut actual = vec![
            prepared
                .log_probability(&observations[index], temperature)
                .unwrap(),
            prepared
                .probability(&observations[index], temperature)
                .unwrap() as f32,
        ];
        for component in 0..mixtures {
            actual.push(
                prepared
                    .component_log_probability(&observations[index], component)
                    .unwrap(),
            );
            actual.push(
                prepared
                    .component_probability(&observations[index], component)
                    .unwrap() as f32,
            );
        }
        assert_eq!(actual, expected, "C likelihood mismatch: {line}");
    }
}

#[test]
fn rejects_uninitialized_variances_and_invalid_inputs() {
    let mut gmm = GaussianMixture::new(1, 2).unwrap();
    assert!(gmm.prepare().is_err());
    gmm.variances.fill(1.0);
    let prepared = gmm.prepare().unwrap();
    assert!(prepared.log_probability(&[0.0], 1.0).is_err());
    assert!(prepared.log_probability(&[f32::NAN, 0.0], 1.0).is_err());
    assert!(prepared.log_probability(&[0.0, 0.0], -1.0).is_err());
    assert!(prepared.component_log_probability(&[0.0, 0.0], 1).is_err());
    assert_eq!(
        prepared
            .log_probability(&[f32::MAX, f32::MAX], 0.0)
            .unwrap(),
        0.0
    );
}

#[test]
fn single_component_weight_retains_the_original_convention() {
    let mut gmm = distribution(1);
    let expected = gmm
        .prepare()
        .unwrap()
        .log_probability(&[0.0, 0.0], 1.0)
        .unwrap();
    gmm.weights[0] = 0.25;
    assert_eq!(
        gmm.prepare()
            .unwrap()
            .log_probability(&[0.0, 0.0], 1.0)
            .unwrap(),
        expected
    );
}
