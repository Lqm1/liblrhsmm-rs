#![cfg(feature = "c-api")]
use liblrhsmm_rs::c_api::*;
use liblrhsmm_rs::model::GaussianMixtureParameters;
use std::ptr::{null, null_mut};

#[test]
fn gaussian_f32_owners_fields_native_transformations_and_c_oracle() {
    // SAFETY: Independent initialized buffers; exclusive mutation; every owner is released once.
    unsafe {
        let mut owner: *mut LiblrhsmmRsGaussianF32 = null_mut();
        assert_eq!(liblrhsmm_rs_gaussian_new_f32(0, 2, &mut owner), 3);
        assert!(owner.is_null());
        assert_eq!(liblrhsmm_rs_gaussian_new_f32(1, 2, &mut owner), 0);
        let mut layout = LiblrhsmmRsGaussianLayout {
            mixtures: 0,
            dimensions: 0,
        };
        assert_eq!(liblrhsmm_rs_gaussian_layout_f32(owner, &mut layout), 0);
        assert_eq!(
            layout,
            LiblrhsmmRsGaussianLayout {
                mixtures: 1,
                dimensions: 2
            }
        );
        let weights: [f32; 1] = [1.0];
        let means: [f32; 2] = [1.25, -2.5];
        let variances: [f32; 2] = [2.5, 0.75];
        let floors: [f32; 2] = [0.1, 0.2];
        let descriptor = LiblrhsmmRsGaussianParametersF32 {
            mixtures: 1,
            dimensions: 2,
            weights: weights.as_ptr(),
            means: means.as_ptr(),
            variances: variances.as_ptr(),
            variance_floors: floors.as_ptr(),
        };
        assert_eq!(liblrhsmm_rs_gaussian_replace_f32(owner, &descriptor), 0);
        let mut copy: *mut LiblrhsmmRsGaussianF32 = null_mut();
        assert_eq!(liblrhsmm_rs_gaussian_clone_f32(owner, &mut copy), 0);
        assert_eq!(
            liblrhsmm_rs_gaussian_increase_mixtures_f32(owner, 5, 0.2),
            0
        );
        let mut native = GaussianMixtureParameters::<f32>::new(1, 2).unwrap();
        native.means.copy_from_slice(&means);
        native.variances.copy_from_slice(&variances);
        native.variance_floors.copy_from_slice(&floors);
        native.increase_mixtures(5, 0.2).unwrap();
        let mut fields = Vec::new();
        for (selector, expected) in [
            &native.weights,
            &native.means,
            &native.variances,
            &native.variance_floors,
        ]
        .into_iter()
        .enumerate()
        {
            let mut values = vec![99.0; expected.len()];
            assert_eq!(
                liblrhsmm_rs_gaussian_copy_field_f32(
                    owner,
                    selector as u32,
                    0,
                    values.as_mut_ptr(),
                    values.len()
                ),
                0
            );
            assert_eq!(
                values.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
                expected.iter().map(|x| x.to_bits()).collect::<Vec<_>>()
            );
            assert_eq!(
                liblrhsmm_rs_gaussian_copy_field_f32(
                    owner,
                    selector as u32,
                    expected.len(),
                    null_mut(),
                    0
                ),
                0
            );
            fields.push(values);
        }
        let mut compared = 0;
        for line in include_str!("fixtures/c-precision-float.txt")
            .lines()
            .filter(|line| line.starts_with("mixture "))
        {
            let mut tokens = line.split_whitespace();
            tokens.next();
            let index: usize = tokens.next().unwrap().parse().unwrap();
            tokens.next();
            let expected: Vec<f32> = tokens.map(|v| v.parse().unwrap()).collect();
            let actual = [
                fields[0][index],
                fields[1][2 * index],
                fields[2][2 * index],
                fields[3][2 * index],
                fields[1][2 * index + 1],
                fields[2][2 * index + 1],
                fields[3][2 * index + 1],
            ];
            for (actual, expected) in actual.into_iter().zip(expected) {
                assert!(
                    (f64::from(actual) - f64::from(expected)).abs()
                        <= 2e-14 * f64::from(expected).abs().max(1.0)
                );
                compared += 1;
            }
        }
        assert_eq!(compared, 35);
        let special = [f32::from_bits(0x7fc12345), -0.0];
        assert_eq!(
            liblrhsmm_rs_gaussian_set_field_f32(copy, 1, 0, special.as_ptr(), 2),
            0
        );
        let mut values = [0.0; 2];
        assert_eq!(
            liblrhsmm_rs_gaussian_copy_field_f32(copy, 1, 0, values.as_mut_ptr(), 2),
            0
        );
        assert_eq!(values.map(f32::to_bits), special.map(f32::to_bits));
        assert_eq!(
            liblrhsmm_rs_gaussian_set_field_f32(copy, 1, 1, special.as_ptr(), 2),
            2
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_set_field_f32(copy, 1, usize::MAX, null(), 0),
            2
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_set_field_f32(copy, 4, 0, null(), 0),
            3
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_copy_field_f32(copy, 1, 0, null_mut(), 1),
            1
        );
        assert_eq!(liblrhsmm_rs_gaussian_increase_mixtures_f32(copy, 2, 0.2), 3);
        assert_eq!(
            liblrhsmm_rs_gaussian_copy_field_f32(copy, 1, 0, values.as_mut_ptr(), 2),
            0
        );
        assert_eq!(values.map(f32::to_bits), special.map(f32::to_bits));
        let bad = LiblrhsmmRsGaussianParametersF32 {
            dimensions: usize::MAX,
            ..descriptor
        };
        assert_eq!(liblrhsmm_rs_gaussian_replace_f32(copy, &bad), 2);
        assert_eq!(liblrhsmm_rs_gaussian_layout_f32(copy, &mut layout), 0);
        assert_eq!(layout.dimensions, 2);
        assert_eq!(liblrhsmm_rs_gaussian_replace_f32(copy, &descriptor), 0);
        let handles = [copy as *const LiblrhsmmRsGaussianF32, copy];
        let mut combined: *mut LiblrhsmmRsGaussianF32 = null_mut();
        assert_eq!(
            liblrhsmm_rs_gaussian_combine_f32(handles.as_ptr(), 2, &mut combined),
            0
        );
        assert_eq!(liblrhsmm_rs_gaussian_layout_f32(combined, &mut layout), 0);
        assert_eq!(
            layout,
            LiblrhsmmRsGaussianLayout {
                mixtures: 1,
                dimensions: 4
            }
        );
        let mut combined_values = [0.0; 4];
        assert_eq!(
            liblrhsmm_rs_gaussian_copy_field_f32(combined, 3, 0, combined_values.as_mut_ptr(), 4),
            0
        );
        assert_eq!(combined_values, [0.1, 0.2, 0.1, 0.2]);
        let mut failure: *mut LiblrhsmmRsGaussianF32 = null_mut();
        assert_eq!(
            liblrhsmm_rs_gaussian_combine_f32(null(), 0, &mut failure),
            3
        );
        assert!(failure.is_null());
        assert_eq!(
            liblrhsmm_rs_gaussian_create_f32(&descriptor, &mut failure),
            0
        );
        assert_eq!(liblrhsmm_rs_gaussian_release_f32(&mut failure), 0);
        let mut duration: *mut LiblrhsmmRsDurationF32 = null_mut();
        let duration_descriptor = LiblrhsmmRsDurationParametersF32 {
            mean: 2.0,
            variance: 3.0,
            minimum: -1,
            maximum: -1,
            fixed_mean: -1,
            variance_floor: 0.4,
        };
        assert_eq!(
            liblrhsmm_rs_duration_create_f32(&duration_descriptor, &mut duration),
            0
        );
        let durations = [duration as *const LiblrhsmmRsDurationF32];
        assert_eq!(
            liblrhsmm_rs_gaussian_from_durations_f32(durations.as_ptr(), 1, &mut failure),
            0
        );
        let mut scalar = 99.0;
        for (selector, expected) in [(0, 1.0), (1, 2.0), (2, 3.0), (3, 0.0)] {
            assert_eq!(
                liblrhsmm_rs_gaussian_copy_field_f32(failure, selector, 0, &mut scalar, 1),
                0
            );
            assert_eq!(scalar, expected);
        }
        assert_eq!(liblrhsmm_rs_duration_release_f32(&mut duration), 0);
        for slot in [&mut owner, &mut copy, &mut combined, &mut failure] {
            assert_eq!(liblrhsmm_rs_gaussian_release_f32(slot), 0);
            assert!(slot.is_null());
        }
        assert_eq!(liblrhsmm_rs_gaussian_release_f32(&mut owner), 0);
        assert_eq!(liblrhsmm_rs_gaussian_new_f32(1, 2, null_mut()), 1);
        assert_eq!(liblrhsmm_rs_gaussian_create_f32(null(), &mut owner), 1);
    }
}

#[test]
fn gaussian_f64_owners_fields_native_transformations_and_c_oracle() {
    // SAFETY: Independent initialized buffers; exclusive mutation; every owner is released once.
    unsafe {
        let mut owner: *mut LiblrhsmmRsGaussianF64 = null_mut();
        assert_eq!(liblrhsmm_rs_gaussian_new_f64(0, 2, &mut owner), 3);
        assert!(owner.is_null());
        assert_eq!(liblrhsmm_rs_gaussian_new_f64(1, 2, &mut owner), 0);
        let mut layout = LiblrhsmmRsGaussianLayout {
            mixtures: 0,
            dimensions: 0,
        };
        assert_eq!(liblrhsmm_rs_gaussian_layout_f64(owner, &mut layout), 0);
        assert_eq!(
            layout,
            LiblrhsmmRsGaussianLayout {
                mixtures: 1,
                dimensions: 2
            }
        );
        let weights: [f64; 1] = [1.0];
        let means: [f64; 2] = [1.25, -2.5];
        let variances: [f64; 2] = [2.5, 0.75];
        let floors: [f64; 2] = [0.1, 0.2];
        let descriptor = LiblrhsmmRsGaussianParametersF64 {
            mixtures: 1,
            dimensions: 2,
            weights: weights.as_ptr(),
            means: means.as_ptr(),
            variances: variances.as_ptr(),
            variance_floors: floors.as_ptr(),
        };
        assert_eq!(liblrhsmm_rs_gaussian_replace_f64(owner, &descriptor), 0);
        let mut copy: *mut LiblrhsmmRsGaussianF64 = null_mut();
        assert_eq!(liblrhsmm_rs_gaussian_clone_f64(owner, &mut copy), 0);
        assert_eq!(
            liblrhsmm_rs_gaussian_increase_mixtures_f64(owner, 5, 0.2),
            0
        );
        let mut native = GaussianMixtureParameters::<f64>::new(1, 2).unwrap();
        native.means.copy_from_slice(&means);
        native.variances.copy_from_slice(&variances);
        native.variance_floors.copy_from_slice(&floors);
        native.increase_mixtures(5, 0.2).unwrap();
        let mut fields = Vec::new();
        for (selector, expected) in [
            &native.weights,
            &native.means,
            &native.variances,
            &native.variance_floors,
        ]
        .into_iter()
        .enumerate()
        {
            let mut values = vec![99.0; expected.len()];
            assert_eq!(
                liblrhsmm_rs_gaussian_copy_field_f64(
                    owner,
                    selector as u32,
                    0,
                    values.as_mut_ptr(),
                    values.len()
                ),
                0
            );
            assert_eq!(
                values.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
                expected.iter().map(|x| x.to_bits()).collect::<Vec<_>>()
            );
            assert_eq!(
                liblrhsmm_rs_gaussian_copy_field_f64(
                    owner,
                    selector as u32,
                    expected.len(),
                    null_mut(),
                    0
                ),
                0
            );
            fields.push(values);
        }
        let mut compared = 0;
        for line in include_str!("fixtures/c-precision-double.txt")
            .lines()
            .filter(|line| line.starts_with("mixture "))
        {
            let mut tokens = line.split_whitespace();
            tokens.next();
            let index: usize = tokens.next().unwrap().parse().unwrap();
            tokens.next();
            let expected: Vec<f64> = tokens.map(|v| v.parse().unwrap()).collect();
            let actual = [
                fields[0][index],
                fields[1][2 * index],
                fields[2][2 * index],
                fields[3][2 * index],
                fields[1][2 * index + 1],
                fields[2][2 * index + 1],
                fields[3][2 * index + 1],
            ];
            for (actual, expected) in actual.into_iter().zip(expected) {
                assert!((actual - expected).abs() <= 2e-14 * expected.abs().max(1.0));
                compared += 1;
            }
        }
        assert_eq!(compared, 35);
        let special = [f64::from_bits(0x7ff8123456789abc), -0.0];
        assert_eq!(
            liblrhsmm_rs_gaussian_set_field_f64(copy, 1, 0, special.as_ptr(), 2),
            0
        );
        let mut values = [0.0; 2];
        assert_eq!(
            liblrhsmm_rs_gaussian_copy_field_f64(copy, 1, 0, values.as_mut_ptr(), 2),
            0
        );
        assert_eq!(values.map(f64::to_bits), special.map(f64::to_bits));
        assert_eq!(
            liblrhsmm_rs_gaussian_set_field_f64(copy, 1, 1, special.as_ptr(), 2),
            2
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_set_field_f64(copy, 1, usize::MAX, null(), 0),
            2
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_set_field_f64(copy, 4, 0, null(), 0),
            3
        );
        assert_eq!(
            liblrhsmm_rs_gaussian_copy_field_f64(copy, 1, 0, null_mut(), 1),
            1
        );
        assert_eq!(liblrhsmm_rs_gaussian_increase_mixtures_f64(copy, 2, 0.2), 3);
        assert_eq!(
            liblrhsmm_rs_gaussian_copy_field_f64(copy, 1, 0, values.as_mut_ptr(), 2),
            0
        );
        assert_eq!(values.map(f64::to_bits), special.map(f64::to_bits));
        let bad = LiblrhsmmRsGaussianParametersF64 {
            dimensions: usize::MAX,
            ..descriptor
        };
        assert_eq!(liblrhsmm_rs_gaussian_replace_f64(copy, &bad), 2);
        assert_eq!(liblrhsmm_rs_gaussian_layout_f64(copy, &mut layout), 0);
        assert_eq!(layout.dimensions, 2);
        assert_eq!(liblrhsmm_rs_gaussian_replace_f64(copy, &descriptor), 0);
        let handles = [copy as *const LiblrhsmmRsGaussianF64, copy];
        let mut combined: *mut LiblrhsmmRsGaussianF64 = null_mut();
        assert_eq!(
            liblrhsmm_rs_gaussian_combine_f64(handles.as_ptr(), 2, &mut combined),
            0
        );
        assert_eq!(liblrhsmm_rs_gaussian_layout_f64(combined, &mut layout), 0);
        assert_eq!(
            layout,
            LiblrhsmmRsGaussianLayout {
                mixtures: 1,
                dimensions: 4
            }
        );
        let mut combined_values = [0.0; 4];
        assert_eq!(
            liblrhsmm_rs_gaussian_copy_field_f64(combined, 3, 0, combined_values.as_mut_ptr(), 4),
            0
        );
        assert_eq!(combined_values, [0.1, 0.2, 0.1, 0.2]);
        let mut failure: *mut LiblrhsmmRsGaussianF64 = null_mut();
        assert_eq!(
            liblrhsmm_rs_gaussian_combine_f64(null(), 0, &mut failure),
            3
        );
        assert!(failure.is_null());
        assert_eq!(
            liblrhsmm_rs_gaussian_create_f64(&descriptor, &mut failure),
            0
        );
        assert_eq!(liblrhsmm_rs_gaussian_release_f64(&mut failure), 0);
        let mut duration: *mut LiblrhsmmRsDurationF64 = null_mut();
        let duration_descriptor = LiblrhsmmRsDurationParametersF64 {
            mean: 2.0,
            variance: 3.0,
            minimum: -1,
            maximum: -1,
            fixed_mean: -1,
            variance_floor: 0.4,
        };
        assert_eq!(
            liblrhsmm_rs_duration_create_f64(&duration_descriptor, &mut duration),
            0
        );
        let durations = [duration as *const LiblrhsmmRsDurationF64];
        assert_eq!(
            liblrhsmm_rs_gaussian_from_durations_f64(durations.as_ptr(), 1, &mut failure),
            0
        );
        let mut scalar = 99.0;
        for (selector, expected) in [(0, 1.0), (1, 2.0), (2, 3.0), (3, 0.0)] {
            assert_eq!(
                liblrhsmm_rs_gaussian_copy_field_f64(failure, selector, 0, &mut scalar, 1),
                0
            );
            assert_eq!(scalar, expected);
        }
        assert_eq!(liblrhsmm_rs_duration_release_f64(&mut duration), 0);
        for slot in [&mut owner, &mut copy, &mut combined, &mut failure] {
            assert_eq!(liblrhsmm_rs_gaussian_release_f64(slot), 0);
            assert!(slot.is_null());
        }
        assert_eq!(liblrhsmm_rs_gaussian_release_f64(&mut owner), 0);
        assert_eq!(liblrhsmm_rs_gaussian_new_f64(1, 2, null_mut()), 1);
        assert_eq!(liblrhsmm_rs_gaussian_create_f64(null(), &mut owner), 1);
    }
}
