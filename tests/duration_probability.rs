use liblrhsmm_rs::{Duration, numerics::LOG_ZERO};

fn sample() -> Duration {
    Duration {
        mean: 4.25,
        variance: 2.5,
        minimum: 2,
        maximum: 6,
        ..Duration::default()
    }
}

#[test]
fn annealed_duration_log_and_density_match_c() {
    let duration = sample();
    let fixture = include_str!("fixtures/c-probability-forms.txt");
    assert_eq!(fixture.lines().count(), 54);
    for line in fixture.lines().filter(|line| line.starts_with("duration")) {
        let mut fields = line.split_whitespace();
        assert_eq!(fields.next(), Some("duration"));
        let temperature: usize = fields.next().unwrap().parse().unwrap();
        let frames: usize = fields.next().unwrap().parse().unwrap();
        let log: f32 = fields.next().unwrap().parse().unwrap();
        let density: f32 = fields.next().unwrap().parse().unwrap();
        assert_eq!(
            duration
                .annealed_log_probability(frames, 0.75, temperature as f32 * 0.5)
                .unwrap(),
            log,
            "{line}"
        );
        assert_eq!(
            duration
                .probability(frames, 0.75, temperature as f32 * 0.5)
                .unwrap(),
            f64::from(density),
            "{line}"
        );
    }
    // Maximum duration constrains searches, not the standalone density.
    assert!(duration.probability(8, 0.75, 1.0).unwrap() > 0.0);
}

#[test]
fn zero_temperature_retains_minimum_and_invalid_inputs_are_errors() {
    let duration = sample();
    assert_eq!(
        duration.annealed_log_probability(1, 0.75, 0.0).unwrap(),
        LOG_ZERO
    );
    assert_eq!(
        duration.annealed_log_probability(3, 0.75, 0.0).unwrap(),
        0.0
    );
    assert!(
        duration
            .annealed_log_probability(usize::MAX, 0.75, 1.0)
            .is_err()
    );
    assert!(
        duration
            .annealed_log_probability(3, 0.75, f32::NAN)
            .is_err()
    );
    assert!(duration.annealed_log_probability(3, 0.75, -1.0).is_err());
    assert!(duration.probability(3, f32::NAN, 1.0).is_err());
    assert!(Duration::default().probability(3, 1.0, 1.0).is_err());
}
