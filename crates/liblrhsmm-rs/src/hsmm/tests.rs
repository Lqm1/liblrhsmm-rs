use super::*;

fn compare<T: Scalar>(fixture: &str) {
    let mut count = 0;
    for line in fixture.lines() {
        let fields: Vec<_> = line.split_whitespace().collect();
        let standard = fields[0].parse::<i32>().unwrap();
        let options = HsmmSettings {
            duration_extra: fields[1].parse().unwrap(),
            duration_extra_factor: T::from_f64(fields[2].parse().unwrap()),
            ..Default::default()
        };
        let distribution = DurationParameters {
            mean: T::from_f64(fields[3].parse().unwrap()),
            variance: T::from_f64(fields[4].parse().unwrap()),
            maximum: fields[5].parse().unwrap(),
            ..Default::default()
        };
        let expected = fields[6].parse::<usize>().unwrap();
        assert_eq!(
            duration_limit(standard, &distribution, options),
            expected,
            "{line}"
        );
        count += 1;
    }
    assert_eq!(count, 3024);
}

#[test]
fn duration_limits_match_c_integer_assignments_and_ceiling_order() {
    compare::<f32>(include_str!("../test_data/c-duration-bounds-float.txt"));
    compare::<f64>(include_str!("../test_data/c-duration-bounds-double.txt"));
}
