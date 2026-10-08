#![cfg(feature = "c-api")]
use liblrhsmm_rs::c_api::*;
use liblrhsmm_rs::model::DurationParameters;
use liblrhsmm_rs::preparation::PreparationSettings;
use std::ptr::{null, null_mut};

#[test]
fn prepared_duration_f32_cache_source_and_independent_lifetime() {
    // SAFETY: All storage is independent and initialized; each owner is released once.
    unsafe {
        let descriptor = LiblrhsmmRsDurationParametersF32 {
            mean: 4.25,
            variance: 2.5,
            minimum: 2,
            maximum: 6,
            fixed_mean: 17,
            variance_floor: 0.125,
        };
        let native = DurationParameters::<f32> {
            mean: 4.25,
            variance: 2.5,
            minimum: 2,
            maximum: 6,
            fixed_mean: 17,
            variance_floor: 0.125,
        };
        let mut owner = null_mut();
        assert_eq!(liblrhsmm_rs_duration_create_f32(&descriptor, &mut owner), 0);
        let mut options = LiblrhsmmRsDurationPreparationF32 {
            duration_cache_length: 0,
            duration_weight: 0.0,
            temperature: 0.0,
        };
        assert_eq!(
            liblrhsmm_rs_duration_preparation_default_f32(&mut options),
            0
        );
        assert_eq!(
            options,
            LiblrhsmmRsDurationPreparationF32 {
                duration_cache_length: 300,
                duration_weight: 1.0,
                temperature: 1.0
            }
        );
        options = LiblrhsmmRsDurationPreparationF32 {
            duration_cache_length: 8,
            duration_weight: 0.5,
            temperature: 0.75,
        };
        let mut prepared = null_mut();
        assert_eq!(
            liblrhsmm_rs_duration_prepare_f32(owner, &options, &mut prepared),
            0
        );
        let mut fetched_parameters = descriptor;
        let mut fetched_options = options;
        assert_eq!(
            liblrhsmm_rs_prepared_duration_parameters_f32(prepared, &mut fetched_parameters),
            0
        );
        assert_eq!(fetched_parameters, descriptor);
        assert_eq!(
            liblrhsmm_rs_prepared_duration_options_f32(prepared, &mut fetched_options),
            0
        );
        assert_eq!(fetched_options, options);
        let expected = native
            .prepare(PreparationSettings {
                duration_cache_length: 8,
                duration_weight: 0.5,
                temperature: 0.75,
            })
            .unwrap();
        let mut count = 99;
        assert_eq!(
            liblrhsmm_rs_prepared_duration_cache_length_f32(prepared, &mut count),
            0
        );
        assert_eq!(count, 8);
        let mut values = [71.0; 8];
        assert_eq!(
            liblrhsmm_rs_prepared_duration_cache_copy_f32(prepared, 0, values.as_mut_ptr(), 8),
            0
        );
        assert_eq!(values, expected.cached_log_probabilities());
        let mut normalizer = 71.0;
        assert_eq!(
            liblrhsmm_rs_prepared_duration_log_normalizer_f32(prepared, &mut normalizer),
            0
        );
        assert_eq!(normalizer.to_bits(), expected.log_normalizer().to_bits());
        let fixture = include_str!("fixtures/c-structures-float.txt");
        let mut compared = 0;
        for line in fixture.lines() {
            let fields: Vec<_> = line.split_whitespace().collect();
            let actual = match fields[0] {
                "duration" => normalizer,
                "cache" => values[fields[1].parse::<usize>().unwrap()],
                _ => continue,
            };
            let expected: f64 = fields.last().unwrap().parse().unwrap();
            let actual = f64::from(actual);
            assert!(
                (actual - expected).abs() / expected.abs().max(1.0) <= 2e-14,
                "{line}"
            );
            compared += 1;
        }
        assert_eq!(compared, 9);
        let mut copied = null_mut();
        assert_eq!(
            liblrhsmm_rs_prepared_duration_clone_f32(prepared, &mut copied),
            0
        );
        let mut edited = descriptor;
        edited.mean = 99.0;
        edited.minimum = 99;
        assert_eq!(liblrhsmm_rs_duration_set_f32(owner, &edited), 0);
        assert_eq!(liblrhsmm_rs_duration_release_f32(&mut owner), 0);
        assert_eq!(liblrhsmm_rs_prepared_duration_release_f32(&mut prepared), 0);
        assert!(prepared.is_null());
        assert_eq!(liblrhsmm_rs_prepared_duration_release_f32(&mut prepared), 0);
        for frames in 0..=400 {
            let mut log = 71.0;
            let mut density = 71.0;
            assert_eq!(
                liblrhsmm_rs_prepared_duration_log_probability_f32(copied, frames, &mut log),
                0
            );
            assert_eq!(
                log.to_bits(),
                expected.log_probability(frames).unwrap().to_bits()
            );
            assert_eq!(
                liblrhsmm_rs_prepared_duration_probability_f32(copied, frames, &mut density),
                0
            );
            assert_eq!(
                density.to_bits(),
                expected.probability(frames).unwrap().to_bits()
            );
        }
        assert_eq!(
            liblrhsmm_rs_prepared_duration_parameters_f32(copied, &mut fetched_parameters),
            0
        );
        assert_eq!(fetched_parameters, descriptor);
        assert_eq!(
            liblrhsmm_rs_prepared_duration_cache_copy_f32(copied, 8, null_mut(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_prepared_duration_cache_copy_f32(copied, 9, null_mut(), 0),
            2
        );
        assert_eq!(
            liblrhsmm_rs_prepared_duration_cache_copy_f32(
                copied,
                usize::MAX,
                values.as_mut_ptr(),
                1
            ),
            2
        );
        values.fill(71.0);
        assert_eq!(
            liblrhsmm_rs_prepared_duration_cache_copy_f32(copied, 7, values.as_mut_ptr(), 2),
            2
        );
        assert_eq!(values, [71.0; 8]);
        assert_eq!(
            liblrhsmm_rs_prepared_duration_cache_copy_f32(copied, 0, null_mut(), 1),
            1
        );
        let misaligned = values.as_mut_ptr().cast::<u8>().wrapping_add(1).cast();
        assert_eq!(
            liblrhsmm_rs_prepared_duration_cache_copy_f32(copied, 0, misaligned, 1),
            1
        );
        assert_eq!(values, [71.0; 8]);
        let mut density_result = 71.0;
        let mut result = 71.0;
        assert_eq!(
            liblrhsmm_rs_prepared_duration_log_probability_f32(copied, usize::MAX, &mut result),
            3
        );
        assert_eq!(result, 71.0);
        assert_eq!(
            liblrhsmm_rs_prepared_duration_log_probability_f32(copied, 3, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_prepared_duration_probability_f32(copied, usize::MAX, &mut density_result),
            3
        );
        assert_eq!(density_result, 71.0);
        assert_eq!(
            liblrhsmm_rs_prepared_duration_options_f32(copied, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_prepared_duration_parameters_f32(copied, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_prepared_duration_log_normalizer_f32(copied, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_prepared_duration_cache_length_f32(copied, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_prepared_duration_clone_f32(copied, null_mut()),
            1
        );
        assert_eq!(liblrhsmm_rs_prepared_duration_release_f32(null_mut()), 1);
        assert_eq!(liblrhsmm_rs_duration_preparation_default_f32(null_mut()), 1);
        assert_eq!(
            liblrhsmm_rs_duration_prepare_f32(null(), &options, &mut prepared),
            1
        );
        assert_eq!(liblrhsmm_rs_prepared_duration_release_f32(&mut copied), 0);
        assert!(copied.is_null());
        // Empty preparations still retain their normalizer and safe fallback.
        assert_eq!(liblrhsmm_rs_duration_create_f32(&descriptor, &mut owner), 0);
        options.duration_cache_length = 0;
        options.temperature = 0.0;
        assert_eq!(
            liblrhsmm_rs_duration_prepare_f32(owner, &options, &mut prepared),
            0
        );
        assert_eq!(
            liblrhsmm_rs_prepared_duration_cache_copy_f32(prepared, 0, null_mut(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_prepared_duration_log_probability_f32(prepared, 1, &mut normalizer),
            0
        );
        assert_eq!(normalizer, -1e15);
        assert_eq!(
            liblrhsmm_rs_prepared_duration_log_probability_f32(prepared, 3, &mut normalizer),
            0
        );
        assert_eq!(normalizer, 0.0);
        assert_eq!(liblrhsmm_rs_prepared_duration_release_f32(&mut prepared), 0);
        options.duration_cache_length = usize::MAX;
        assert_eq!(
            liblrhsmm_rs_duration_prepare_f32(owner, &options, &mut prepared),
            3
        );
        assert!(prepared.is_null());
        assert_eq!(
            liblrhsmm_rs_duration_prepare_f32(owner, null(), &mut prepared),
            1
        );
        assert_eq!(
            liblrhsmm_rs_duration_prepare_f32(owner, &options, null_mut()),
            1
        );
        assert_eq!(liblrhsmm_rs_duration_release_f32(&mut owner), 0);
    }
}

#[test]
fn prepared_duration_f64_cache_source_and_independent_lifetime() {
    // SAFETY: All storage is independent and initialized; each owner is released once.
    unsafe {
        let descriptor = LiblrhsmmRsDurationParametersF64 {
            mean: 4.25,
            variance: 2.5,
            minimum: 2,
            maximum: 6,
            fixed_mean: 17,
            variance_floor: 0.125,
        };
        let native = DurationParameters::<f64> {
            mean: 4.25,
            variance: 2.5,
            minimum: 2,
            maximum: 6,
            fixed_mean: 17,
            variance_floor: 0.125,
        };
        let mut owner = null_mut();
        assert_eq!(liblrhsmm_rs_duration_create_f64(&descriptor, &mut owner), 0);
        let mut options = LiblrhsmmRsDurationPreparationF64 {
            duration_cache_length: 0,
            duration_weight: 0.0,
            temperature: 0.0,
        };
        assert_eq!(
            liblrhsmm_rs_duration_preparation_default_f64(&mut options),
            0
        );
        assert_eq!(
            options,
            LiblrhsmmRsDurationPreparationF64 {
                duration_cache_length: 300,
                duration_weight: 1.0,
                temperature: 1.0
            }
        );
        options = LiblrhsmmRsDurationPreparationF64 {
            duration_cache_length: 8,
            duration_weight: 0.5,
            temperature: 0.75,
        };
        let mut prepared = null_mut();
        assert_eq!(
            liblrhsmm_rs_duration_prepare_f64(owner, &options, &mut prepared),
            0
        );
        let mut fetched_parameters = descriptor;
        let mut fetched_options = options;
        assert_eq!(
            liblrhsmm_rs_prepared_duration_parameters_f64(prepared, &mut fetched_parameters),
            0
        );
        assert_eq!(fetched_parameters, descriptor);
        assert_eq!(
            liblrhsmm_rs_prepared_duration_options_f64(prepared, &mut fetched_options),
            0
        );
        assert_eq!(fetched_options, options);
        let expected = native
            .prepare(PreparationSettings {
                duration_cache_length: 8,
                duration_weight: 0.5,
                temperature: 0.75,
            })
            .unwrap();
        let mut count = 99;
        assert_eq!(
            liblrhsmm_rs_prepared_duration_cache_length_f64(prepared, &mut count),
            0
        );
        assert_eq!(count, 8);
        let mut values = [71.0; 8];
        assert_eq!(
            liblrhsmm_rs_prepared_duration_cache_copy_f64(prepared, 0, values.as_mut_ptr(), 8),
            0
        );
        assert_eq!(values, expected.cached_log_probabilities());
        let mut normalizer = 71.0;
        assert_eq!(
            liblrhsmm_rs_prepared_duration_log_normalizer_f64(prepared, &mut normalizer),
            0
        );
        assert_eq!(normalizer.to_bits(), expected.log_normalizer().to_bits());
        let fixture = include_str!("fixtures/c-structures-double.txt");
        let mut compared = 0;
        for line in fixture.lines() {
            let fields: Vec<_> = line.split_whitespace().collect();
            let actual = match fields[0] {
                "duration" => normalizer,
                "cache" => values[fields[1].parse::<usize>().unwrap()],
                _ => continue,
            };
            let expected: f64 = fields.last().unwrap().parse().unwrap();
            assert!(
                (actual - expected).abs() / expected.abs().max(1.0) <= 2e-14,
                "{line}"
            );
            compared += 1;
        }
        assert_eq!(compared, 9);
        let mut copied = null_mut();
        assert_eq!(
            liblrhsmm_rs_prepared_duration_clone_f64(prepared, &mut copied),
            0
        );
        let mut edited = descriptor;
        edited.mean = 99.0;
        edited.minimum = 99;
        assert_eq!(liblrhsmm_rs_duration_set_f64(owner, &edited), 0);
        assert_eq!(liblrhsmm_rs_duration_release_f64(&mut owner), 0);
        assert_eq!(liblrhsmm_rs_prepared_duration_release_f64(&mut prepared), 0);
        assert!(prepared.is_null());
        assert_eq!(liblrhsmm_rs_prepared_duration_release_f64(&mut prepared), 0);
        for frames in 0..=400 {
            let mut log = 71.0;
            let mut density = 71.0;
            assert_eq!(
                liblrhsmm_rs_prepared_duration_log_probability_f64(copied, frames, &mut log),
                0
            );
            assert_eq!(
                log.to_bits(),
                expected.log_probability(frames).unwrap().to_bits()
            );
            assert_eq!(
                liblrhsmm_rs_prepared_duration_probability_f64(copied, frames, &mut density),
                0
            );
            assert_eq!(
                density.to_bits(),
                expected.probability(frames).unwrap().to_bits()
            );
        }
        assert_eq!(
            liblrhsmm_rs_prepared_duration_parameters_f64(copied, &mut fetched_parameters),
            0
        );
        assert_eq!(fetched_parameters, descriptor);
        assert_eq!(
            liblrhsmm_rs_prepared_duration_cache_copy_f64(copied, 8, null_mut(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_prepared_duration_cache_copy_f64(copied, 9, null_mut(), 0),
            2
        );
        assert_eq!(
            liblrhsmm_rs_prepared_duration_cache_copy_f64(
                copied,
                usize::MAX,
                values.as_mut_ptr(),
                1
            ),
            2
        );
        values.fill(71.0);
        assert_eq!(
            liblrhsmm_rs_prepared_duration_cache_copy_f64(copied, 7, values.as_mut_ptr(), 2),
            2
        );
        assert_eq!(values, [71.0; 8]);
        assert_eq!(
            liblrhsmm_rs_prepared_duration_cache_copy_f64(copied, 0, null_mut(), 1),
            1
        );
        let misaligned = values.as_mut_ptr().cast::<u8>().wrapping_add(1).cast();
        assert_eq!(
            liblrhsmm_rs_prepared_duration_cache_copy_f64(copied, 0, misaligned, 1),
            1
        );
        assert_eq!(values, [71.0; 8]);
        let mut density_result = 71.0;
        let mut result = 71.0;
        assert_eq!(
            liblrhsmm_rs_prepared_duration_log_probability_f64(copied, usize::MAX, &mut result),
            3
        );
        assert_eq!(result, 71.0);
        assert_eq!(
            liblrhsmm_rs_prepared_duration_log_probability_f64(copied, 3, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_prepared_duration_probability_f64(copied, usize::MAX, &mut density_result),
            3
        );
        assert_eq!(density_result, 71.0);
        assert_eq!(
            liblrhsmm_rs_prepared_duration_options_f64(copied, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_prepared_duration_parameters_f64(copied, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_prepared_duration_log_normalizer_f64(copied, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_prepared_duration_cache_length_f64(copied, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_prepared_duration_clone_f64(copied, null_mut()),
            1
        );
        assert_eq!(liblrhsmm_rs_prepared_duration_release_f64(null_mut()), 1);
        assert_eq!(liblrhsmm_rs_duration_preparation_default_f64(null_mut()), 1);
        assert_eq!(
            liblrhsmm_rs_duration_prepare_f64(null(), &options, &mut prepared),
            1
        );
        assert_eq!(liblrhsmm_rs_prepared_duration_release_f64(&mut copied), 0);
        assert!(copied.is_null());
        // Empty preparations still retain their normalizer and safe fallback.
        assert_eq!(liblrhsmm_rs_duration_create_f64(&descriptor, &mut owner), 0);
        options.duration_cache_length = 0;
        options.temperature = 0.0;
        assert_eq!(
            liblrhsmm_rs_duration_prepare_f64(owner, &options, &mut prepared),
            0
        );
        assert_eq!(
            liblrhsmm_rs_prepared_duration_cache_copy_f64(prepared, 0, null_mut(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_prepared_duration_log_probability_f64(prepared, 1, &mut normalizer),
            0
        );
        assert_eq!(normalizer, -1e15);
        assert_eq!(
            liblrhsmm_rs_prepared_duration_log_probability_f64(prepared, 3, &mut normalizer),
            0
        );
        assert_eq!(normalizer, 0.0);
        assert_eq!(liblrhsmm_rs_prepared_duration_release_f64(&mut prepared), 0);
        options.duration_cache_length = usize::MAX;
        assert_eq!(
            liblrhsmm_rs_duration_prepare_f64(owner, &options, &mut prepared),
            3
        );
        assert!(prepared.is_null());
        assert_eq!(
            liblrhsmm_rs_duration_prepare_f64(owner, null(), &mut prepared),
            1
        );
        assert_eq!(
            liblrhsmm_rs_duration_prepare_f64(owner, &options, null_mut()),
            1
        );
        assert_eq!(liblrhsmm_rs_duration_release_f64(&mut owner), 0);
    }
}
