use liblrhsmm_rs::emission::EmissionPruningOptions;
use liblrhsmm_rs::geometric::GeometricSettings;
use liblrhsmm_rs::numerics::*;
use liblrhsmm_rs::scalar::Scalar;
use liblrhsmm_rs::statistics::ModelSufficientStatistics;

#[path = "common/estimation.rs"]
#[allow(dead_code)]
mod common;

struct Restore {
    log: Vec<f32>,
    exp: Vec<f32>,
    log64: Vec<f64>,
    exp64: Vec<f64>,
}
impl Drop for Restore {
    fn drop(&mut self) {
        for (i, &value) in self.log.iter().enumerate() {
            LOG_ADD_COEFFICIENTS.replace(i, value);
        }
        for (i, &value) in self.exp.iter().enumerate() {
            EXPONENTIAL_COEFFICIENTS.replace(i, value);
        }
        for (i, &value) in self.log64.iter().enumerate() {
            LOG_ADD_COEFFICIENTS_F64.replace(i, value);
        }
        for (i, &value) in self.exp64.iter().enumerate() {
            EXPONENTIAL_COEFFICIENTS_F64.replace(i, value);
        }
    }
}

fn compare<T: Scalar>(fixture: &str, exact: bool, mutate: impl Fn(), helpers: impl Fn() -> [T; 2]) {
    let mut maximum = 0.0_f64;
    let mut count = 0;
    let mut totals = Vec::new();
    for mode in 0..2 {
        if mode == 1 {
            mutate();
        }
        let (model, observation, segmentation) = common::sample_with_precision::<T>();
        let options = GeometricSettings {
            temperature: T::ONE,
            pruning_slope: T::from_f64(0.5),
        };
        let emissions = model
            .emission_log_probabilities(
                &observation,
                &segmentation,
                T::ONE,
                EmissionPruningOptions::Linear {
                    slope: options.pruning_slope,
                },
            )
            .unwrap();
        let forward = model
            .forward_geometric(&segmentation, &emissions, options)
            .unwrap();
        let backward = model
            .backward_geometric(&segmentation, &emissions, options)
            .unwrap();
        let mut statistics = ModelSufficientStatistics::from_model(&model).unwrap();
        let total = statistics
            .estimate_geometric(&model, &observation, &segmentation, options)
            .unwrap();
        totals.push(total.into());
        for line in fixture.lines() {
            let fields: Vec<_> = line.split_whitespace().collect();
            if fields[1].parse::<usize>().unwrap() != mode {
                continue;
            }
            let (actual, start): (Vec<T>, usize) = match fields[0] {
                "helper" => (helpers().to_vec(), 2),
                "scores" => {
                    let index = fields[2].parse::<usize>().unwrap();
                    (vec![forward.values()[index], backward.values()[index]], 3)
                }
                "total" => (vec![total], 2),
                "duration" => {
                    let stat = &statistics.durations[fields[2].parse::<usize>().unwrap()];
                    (vec![stat.value_sum, stat.squared_sum, stat.occupancy], 3)
                }
                "mixture" => {
                    let stat = &statistics.streams[fields[2].parse::<usize>().unwrap()].mixtures
                        [fields[3].parse::<usize>().unwrap()];
                    let mut values = vec![stat.state_occupancy];
                    values.extend(&stat.component_occupancies);
                    for (&sum, &squared) in
                        stat.weighted_sums.iter().zip(&stat.weighted_squared_sums)
                    {
                        values.extend([sum, squared]);
                    }
                    (values, 4)
                }
                _ => panic!("unexpected reference: {line}"),
            };
            assert_eq!(actual.len(), fields.len() - start);
            for (actual, expected) in actual.into_iter().zip(&fields[start..]) {
                let actual: f64 = actual.into();
                let expected: f64 = expected.parse().unwrap();
                if exact {
                    assert_eq!(
                        (actual as f32).to_bits(),
                        (expected as f32).to_bits(),
                        "{line}"
                    );
                }
                maximum = maximum.max((actual - expected).abs() / expected.abs().max(1.0));
                count += 1;
            }
        }
    }
    assert_eq!(count, 258);
    assert_ne!(
        totals[0], totals[1],
        "table changes must reach training likelihoods"
    );
    println!("mutable tables: {count} comparisons; maximum normalized error {maximum}");
    assert!(maximum <= 2e-14);
}

// A separate integration-test process isolates process-global modifications
// from the default-coefficient oracle tests in other binaries.
#[test]
fn changes_reach_inference_and_training_and_preserve_atomic_scalar_bits() {
    let restore = Restore {
        log: log_add_table().collect(),
        exp: exponential_table().collect(),
        log64: log_add_table_f64().collect(),
        exp64: exponential_table_f64().collect(),
    };
    assert_eq!(LOG_ADD_COEFFICIENTS.len(), LOG_ADD_TABLE_SIZE);
    assert!(!LOG_ADD_COEFFICIENTS.is_empty());
    assert!(LOG_ADD_COEFFICIENTS.get(usize::MAX).is_none());
    assert!(
        EXPONENTIAL_COEFFICIENTS_F64
            .replace(usize::MAX, 0.0)
            .is_none()
    );
    compare::<f32>(
        include_str!("../../../tests/fixtures/c-mutable-tables-float.txt"),
        true,
        || {
            for i in 0..7000 {
                assert_eq!(
                    LOG_ADD_COEFFICIENTS
                        .replace(i, (f64::from(restore.log[i]) + 0.015625) as f32)
                        .unwrap()
                        .to_bits(),
                    restore.log[i].to_bits()
                );
                EXPONENTIAL_COEFFICIENTS.replace(i, (f64::from(restore.exp[i]) * 0.875) as f32);
            }
        },
        || [log_add(0.0, 0.0), negative_exponential(-0.125)],
    );
    assert_eq!(log_add_table_f64().collect::<Vec<_>>(), restore.log64);
    assert_eq!(exponential_table_f64().collect::<Vec<_>>(), restore.exp64);
    compare::<f64>(
        include_str!("../../../tests/fixtures/c-mutable-tables-double.txt"),
        false,
        || {
            for i in 0..7000 {
                LOG_ADD_COEFFICIENTS_F64.replace(i, restore.log64[i] + 0.015625);
                EXPONENTIAL_COEFFICIENTS_F64.replace(i, restore.exp64[i] * 0.875);
            }
        },
        || [log_add_f64(0.0, 0.0), negative_exponential_f64(-0.125)],
    );

    let index = 6999;
    let payload = f32::from_bits(0x7f80_0123);
    LOG_ADD_COEFFICIENTS.replace(index, payload);
    assert_eq!(
        LOG_ADD_COEFFICIENTS.get(index).unwrap().to_bits(),
        payload.to_bits()
    );
    assert_eq!(
        log_add_table().next_back().unwrap().to_bits(),
        payload.to_bits()
    );
    LOG_ADD_COEFFICIENTS_F64.replace(index, 0.0);
    std::thread::scope(|scope| {
        scope.spawn(|| {
            for i in 0..100_000 {
                LOG_ADD_COEFFICIENTS_F64
                    .replace(index, f64::from_bits(if i % 2 == 0 { u64::MAX } else { 0 }));
            }
        });
        for _ in 0..100_000 {
            let bits = LOG_ADD_COEFFICIENTS_F64.get(index).unwrap().to_bits();
            assert!(bits == 0 || bits == u64::MAX, "a coefficient must not tear");
        }
    });
    drop(restore);
}
