use liblrhsmm_rs::memory_pool::MemoryPool;
use liblrhsmm_rs::model::{ModelParameters, StreamLayout};
use liblrhsmm_rs::preparation::PreparationSettings;
use liblrhsmm_rs::scalar::Scalar;

fn check_cache<T: Scalar>(fixture: &str, exact_binary32: bool) {
    let mut model = ModelParameters::<T>::new(
        &[StreamLayout {
            emissions: 1,
            components: 3,
            dimensions: 2,
        }],
        1,
    )
    .unwrap();
    model.streams[0].mixtures[0].variances = [0.75, 1.25, 2.5, 3.75, 0.125, 0.5]
        .into_iter()
        .map(T::from_f64)
        .collect();
    model.durations[0].mean = T::from_f64(4.25);
    model.durations[0].variance = T::from_f64(2.5);
    model.durations[0].minimum = 2;
    model.durations[0].maximum = 6;
    let prepared = model
        .prepare(PreparationSettings {
            duration_cache_length: 8,
            temperature: T::from_f64(0.75),
            duration_weight: T::from_f64(0.5),
        })
        .unwrap();
    let gaussian = prepared.emission(0, 0).unwrap();
    let duration = prepared.duration(0).unwrap();
    let mut count = 0;
    let mut maximum = 0.0_f64;
    for line in fixture.lines() {
        let fields: Vec<_> = line.split_whitespace().collect();
        let actual = match fields[0] {
            "gaussian" => gaussian.cached_log_normalizers()[fields[1].parse::<usize>().unwrap()],
            "duration" => duration.log_normalizer(),
            "cache" => duration.cached_log_probabilities()[fields[1].parse::<usize>().unwrap()],
            "page" => continue,
            _ => panic!("unexpected record: {line}"),
        };
        let actual: f64 = actual.into();
        let expected: f64 = fields.last().unwrap().parse().unwrap();
        if exact_binary32 {
            assert_eq!(
                (actual as f32).to_bits(),
                (expected as f32).to_bits(),
                "{line}"
            );
        }
        maximum = maximum.max((actual - expected).abs() / expected.abs().max(1.0));
        count += 1;
    }
    assert_eq!(count, 12);
    println!("prepared cache metadata: {count} values; maximum normalized error {maximum}");
    assert!(maximum <= 2e-14);
}

#[test]
fn prepared_constants_and_lookup_storage_match_both_c_precisions() {
    check_cache::<f32>(include_str!("fixtures/c-structures-float.txt"), true);
    check_cache::<f64>(include_str!("fixtures/c-structures-double.txt"), false);
}

#[test]
fn every_pool_page_matches_the_c_linked_list_and_keeps_exact_length() {
    let fixture = include_str!("fixtures/c-structures-double.txt");
    let mut pool = MemoryPool::new(8).unwrap();
    for (step, size) in [3, 6, 12, 0, 2].into_iter().enumerate() {
        pool.allocate_zeroed(1, size).unwrap();
        let mut pages = pool.pages();
        assert_eq!(pages.len(), pool.page_count());
        for line in fixture.lines().filter(|line| line.starts_with("page ")) {
            let fields: Vec<_> = line.split_whitespace().collect();
            if fields[1].parse::<usize>().unwrap() != step {
                continue;
            }
            let remaining = pages.len();
            let page = pages.next().unwrap();
            assert_eq!(page.used_bytes, fields[3].parse::<usize>().unwrap());
            assert_eq!(page.capacity, fields[4].parse::<usize>().unwrap());
            assert_eq!(pages.len(), remaining - 1);
        }
        assert!(pages.next().is_none());
        assert_eq!(
            pool.pages().map(|page| page.capacity).sum::<usize>(),
            pool.total_capacity()
        );
        assert_eq!(
            pool.pages().map(|page| page.used_bytes).sum::<usize>(),
            pool.allocated_bytes()
        );
    }
}
