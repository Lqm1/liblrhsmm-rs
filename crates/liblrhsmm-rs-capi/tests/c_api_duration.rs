use liblrhsmm_rs::model::DurationParameters;
use liblrhsmm_rs_capi::*;
use std::ptr::{null, null_mut};

#[test]
fn duration_f32_ownership_fields_and_original_c_densities() {
    // SAFETY: Every pointer identifies independent initialized storage; each owner is released once.
    unsafe {
        let mut descriptor = LiblrhsmmRsDurationParametersF32 {
            mean: 99.0,
            variance: 99.0,
            minimum: 99,
            maximum: 99,
            fixed_mean: 99,
            variance_floor: 99.0,
        };
        assert_eq!(liblrhsmm_rs_duration_default_f32(&mut descriptor), 0);
        assert_eq!(
            descriptor,
            LiblrhsmmRsDurationParametersF32 {
                mean: 0.0,
                variance: 0.0,
                minimum: -1,
                maximum: -1,
                fixed_mean: -1,
                variance_floor: 0.0
            }
        );
        let mut owner: *mut LiblrhsmmRsDurationF32 = null_mut();
        assert_eq!(liblrhsmm_rs_duration_create_f32(&descriptor, &mut owner), 0);
        let mut density = 71.0;
        assert_eq!(
            liblrhsmm_rs_duration_probability_f32(owner, 3, 1.0, 1.0, &mut density),
            3
        );
        assert_eq!(density, 71.0);
        descriptor = LiblrhsmmRsDurationParametersF32 {
            mean: 4.25,
            variance: 2.5,
            minimum: 2,
            maximum: 6,
            fixed_mean: 17,
            variance_floor: 0.125,
        };
        assert_eq!(liblrhsmm_rs_duration_set_f32(owner, &descriptor), 0);
        let native = DurationParameters::<f32> {
            mean: descriptor.mean,
            variance: descriptor.variance,
            minimum: descriptor.minimum,
            maximum: descriptor.maximum,
            fixed_mean: descriptor.fixed_mean,
            variance_floor: descriptor.variance_floor,
        };
        let fixture = include_str!("../../../tests/fixtures/c-precision-float.txt");
        let mut compared = 0;
        for line in fixture.lines().filter(|line| line.starts_with("duration ")) {
            let fields: Vec<_> = line.split_whitespace().collect();
            let frames: usize = fields[1].parse().unwrap();
            let temperature = [0.5, 1.0, 1.75][fields[2].parse::<usize>().unwrap()];
            let mut log = 71.0;
            let mut annealed = 71.0;
            assert_eq!(
                liblrhsmm_rs_duration_log_probability_f32(owner, frames as f32, 0.75, &mut log),
                0
            );
            assert_eq!(
                liblrhsmm_rs_duration_annealed_log_probability_f32(
                    owner,
                    frames,
                    0.75,
                    temperature,
                    &mut annealed
                ),
                0
            );
            assert_eq!(
                liblrhsmm_rs_duration_probability_f32(
                    owner,
                    frames,
                    0.75,
                    temperature,
                    &mut density
                ),
                0
            );
            assert_eq!(
                log.to_bits(),
                native
                    .log_probability(frames as f32, 0.75)
                    .unwrap()
                    .to_bits()
            );
            assert_eq!(
                annealed.to_bits(),
                native
                    .annealed_log_probability(frames, 0.75, temperature)
                    .unwrap()
                    .to_bits()
            );
            assert_eq!(
                density.to_bits(),
                native
                    .probability(frames, 0.75, temperature)
                    .unwrap()
                    .to_bits()
            );
            for (actual, expected) in [log as f64, annealed as f64, density]
                .into_iter()
                .zip(&fields[3..])
            {
                let expected: f64 = expected.parse().unwrap();
                assert!(
                    (actual - expected).abs() / expected.abs().max(1.0) <= 2e-14,
                    "{line}"
                );
                compared += 1;
            }
        }
        assert_eq!(compared, 81);
        assert_eq!(
            liblrhsmm_rs_duration_probability_f32(owner, 8, 0.75, 1.0, &mut density),
            0
        );
        assert!(density > 0.0);
        let mut copy = null_mut();
        assert_eq!(liblrhsmm_rs_duration_clone_f32(owner, &mut copy), 0);
        descriptor.mean = 8.0;
        descriptor.variance_floor = f32::from_bits(0x7fc01234);
        assert_eq!(liblrhsmm_rs_duration_set_f32(copy, &descriptor), 0);
        let mut fetched = descriptor;
        assert_eq!(liblrhsmm_rs_duration_get_f32(copy, &mut fetched), 0);
        assert_eq!(
            fetched.variance_floor.to_bits(),
            descriptor.variance_floor.to_bits()
        );
        assert_eq!(liblrhsmm_rs_duration_get_f32(owner, &mut fetched), 0);
        assert_eq!(fetched.mean, 4.25);
        assert_eq!(fetched.fixed_mean, 17);
        assert_eq!(fetched.variance_floor, 0.125);
        assert_eq!(liblrhsmm_rs_duration_set_f32(owner, null()), 1);
        assert_eq!(liblrhsmm_rs_duration_clone_f32(owner, null_mut()), 1);
        assert_eq!(liblrhsmm_rs_duration_create_f32(&descriptor, null_mut()), 1);
        assert_eq!(liblrhsmm_rs_duration_create_f32(null(), &mut copy), 1);
        assert_eq!(liblrhsmm_rs_duration_get_f32(null(), &mut fetched), 1);
        assert_eq!(liblrhsmm_rs_duration_get_f32(owner, null_mut()), 1);
        assert_eq!(liblrhsmm_rs_duration_release_f32(null_mut()), 1);
        assert_eq!(liblrhsmm_rs_duration_default_f32(null_mut()), 1);
        let mut output = 71.0;
        assert_eq!(
            liblrhsmm_rs_duration_annealed_log_probability_f32(
                owner,
                usize::MAX,
                0.75,
                1.0,
                &mut output
            ),
            3
        );
        assert_eq!(output, 71.0);
        assert_eq!(
            liblrhsmm_rs_duration_annealed_log_probability_f32(owner, 3, 0.75, -1.0, &mut output),
            3
        );
        assert_eq!(output, 71.0);
        assert_eq!(
            liblrhsmm_rs_duration_log_probability_f32(owner, f32::NAN, 0.75, &mut output),
            3
        );
        assert_eq!(output, 71.0);
        assert_eq!(
            liblrhsmm_rs_duration_probability_f32(owner, 3, f32::NAN, 1.0, &mut density),
            3
        );
        assert_eq!(
            liblrhsmm_rs_duration_probability_f32(owner, 3, 0.75, 1.0, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_duration_annealed_log_probability_f32(owner, 1, 0.75, 0.0, &mut output),
            0
        );
        assert_eq!(output, -1e15);
        assert_eq!(
            liblrhsmm_rs_duration_annealed_log_probability_f32(owner, 3, 0.75, 0.0, &mut output),
            0
        );
        assert_eq!(output, 0.0);
        assert_eq!(liblrhsmm_rs_duration_release_f32(&mut owner), 0);
        assert!(owner.is_null());
        assert_eq!(liblrhsmm_rs_duration_release_f32(&mut owner), 0);
        assert_eq!(liblrhsmm_rs_duration_get_f32(copy, &mut fetched), 0);
        assert_eq!(fetched.mean, 8.0);
        assert_eq!(liblrhsmm_rs_duration_release_f32(&mut copy), 0);
        assert!(copy.is_null());
    }
}

#[test]
fn duration_f64_ownership_fields_and_original_c_densities() {
    // SAFETY: Every pointer identifies independent initialized storage; each owner is released once.
    unsafe {
        let mut descriptor = LiblrhsmmRsDurationParametersF64 {
            mean: 99.0,
            variance: 99.0,
            minimum: 99,
            maximum: 99,
            fixed_mean: 99,
            variance_floor: 99.0,
        };
        assert_eq!(liblrhsmm_rs_duration_default_f64(&mut descriptor), 0);
        assert_eq!(
            descriptor,
            LiblrhsmmRsDurationParametersF64 {
                mean: 0.0,
                variance: 0.0,
                minimum: -1,
                maximum: -1,
                fixed_mean: -1,
                variance_floor: 0.0
            }
        );
        let mut owner: *mut LiblrhsmmRsDurationF64 = null_mut();
        assert_eq!(liblrhsmm_rs_duration_create_f64(&descriptor, &mut owner), 0);
        let mut density = 71.0;
        assert_eq!(
            liblrhsmm_rs_duration_probability_f64(owner, 3, 1.0, 1.0, &mut density),
            3
        );
        assert_eq!(density, 71.0);
        descriptor = LiblrhsmmRsDurationParametersF64 {
            mean: 4.25,
            variance: 2.5,
            minimum: 2,
            maximum: 6,
            fixed_mean: 17,
            variance_floor: 0.125,
        };
        assert_eq!(liblrhsmm_rs_duration_set_f64(owner, &descriptor), 0);
        let native = DurationParameters::<f64> {
            mean: descriptor.mean,
            variance: descriptor.variance,
            minimum: descriptor.minimum,
            maximum: descriptor.maximum,
            fixed_mean: descriptor.fixed_mean,
            variance_floor: descriptor.variance_floor,
        };
        let fixture = include_str!("../../../tests/fixtures/c-precision-double.txt");
        let mut compared = 0;
        for line in fixture.lines().filter(|line| line.starts_with("duration ")) {
            let fields: Vec<_> = line.split_whitespace().collect();
            let frames: usize = fields[1].parse().unwrap();
            let temperature = [0.5, 1.0, 1.75][fields[2].parse::<usize>().unwrap()];
            let mut log = 71.0;
            let mut annealed = 71.0;
            assert_eq!(
                liblrhsmm_rs_duration_log_probability_f64(owner, frames as f64, 0.75, &mut log),
                0
            );
            assert_eq!(
                liblrhsmm_rs_duration_annealed_log_probability_f64(
                    owner,
                    frames,
                    0.75,
                    temperature,
                    &mut annealed
                ),
                0
            );
            assert_eq!(
                liblrhsmm_rs_duration_probability_f64(
                    owner,
                    frames,
                    0.75,
                    temperature,
                    &mut density
                ),
                0
            );
            assert_eq!(
                log.to_bits(),
                native
                    .log_probability(frames as f64, 0.75)
                    .unwrap()
                    .to_bits()
            );
            assert_eq!(
                annealed.to_bits(),
                native
                    .annealed_log_probability(frames, 0.75, temperature)
                    .unwrap()
                    .to_bits()
            );
            assert_eq!(
                density.to_bits(),
                native
                    .probability(frames, 0.75, temperature)
                    .unwrap()
                    .to_bits()
            );
            for (actual, expected) in [log, annealed, density].into_iter().zip(&fields[3..]) {
                let expected: f64 = expected.parse().unwrap();
                assert!(
                    (actual - expected).abs() / expected.abs().max(1.0) <= 2e-14,
                    "{line}"
                );
                compared += 1;
            }
        }
        assert_eq!(compared, 81);
        assert_eq!(
            liblrhsmm_rs_duration_probability_f64(owner, 8, 0.75, 1.0, &mut density),
            0
        );
        assert!(density > 0.0);
        let mut copy = null_mut();
        assert_eq!(liblrhsmm_rs_duration_clone_f64(owner, &mut copy), 0);
        descriptor.mean = 8.0;
        descriptor.variance_floor = f64::from_bits(0x7ff8000000001234);
        assert_eq!(liblrhsmm_rs_duration_set_f64(copy, &descriptor), 0);
        let mut fetched = descriptor;
        assert_eq!(liblrhsmm_rs_duration_get_f64(copy, &mut fetched), 0);
        assert_eq!(
            fetched.variance_floor.to_bits(),
            descriptor.variance_floor.to_bits()
        );
        assert_eq!(liblrhsmm_rs_duration_get_f64(owner, &mut fetched), 0);
        assert_eq!(fetched.mean, 4.25);
        assert_eq!(fetched.fixed_mean, 17);
        assert_eq!(fetched.variance_floor, 0.125);
        assert_eq!(liblrhsmm_rs_duration_set_f64(owner, null()), 1);
        assert_eq!(liblrhsmm_rs_duration_clone_f64(owner, null_mut()), 1);
        assert_eq!(liblrhsmm_rs_duration_create_f64(&descriptor, null_mut()), 1);
        assert_eq!(liblrhsmm_rs_duration_create_f64(null(), &mut copy), 1);
        assert_eq!(liblrhsmm_rs_duration_get_f64(null(), &mut fetched), 1);
        assert_eq!(liblrhsmm_rs_duration_get_f64(owner, null_mut()), 1);
        assert_eq!(liblrhsmm_rs_duration_release_f64(null_mut()), 1);
        assert_eq!(liblrhsmm_rs_duration_default_f64(null_mut()), 1);
        let mut output = 71.0;
        assert_eq!(
            liblrhsmm_rs_duration_annealed_log_probability_f64(
                owner,
                usize::MAX,
                0.75,
                1.0,
                &mut output
            ),
            3
        );
        assert_eq!(output, 71.0);
        assert_eq!(
            liblrhsmm_rs_duration_annealed_log_probability_f64(owner, 3, 0.75, -1.0, &mut output),
            3
        );
        assert_eq!(output, 71.0);
        assert_eq!(
            liblrhsmm_rs_duration_log_probability_f64(owner, f64::NAN, 0.75, &mut output),
            3
        );
        assert_eq!(output, 71.0);
        assert_eq!(
            liblrhsmm_rs_duration_probability_f64(owner, 3, f64::NAN, 1.0, &mut density),
            3
        );
        assert_eq!(
            liblrhsmm_rs_duration_probability_f64(owner, 3, 0.75, 1.0, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_duration_annealed_log_probability_f64(owner, 1, 0.75, 0.0, &mut output),
            0
        );
        assert_eq!(output, -1e15);
        assert_eq!(
            liblrhsmm_rs_duration_annealed_log_probability_f64(owner, 3, 0.75, 0.0, &mut output),
            0
        );
        assert_eq!(output, 0.0);
        assert_eq!(liblrhsmm_rs_duration_release_f64(&mut owner), 0);
        assert!(owner.is_null());
        assert_eq!(liblrhsmm_rs_duration_release_f64(&mut owner), 0);
        assert_eq!(liblrhsmm_rs_duration_get_f64(copy, &mut fetched), 0);
        assert_eq!(fetched.mean, 8.0);
        assert_eq!(liblrhsmm_rs_duration_release_f64(&mut copy), 0);
        assert!(copy.is_null());
    }
}
