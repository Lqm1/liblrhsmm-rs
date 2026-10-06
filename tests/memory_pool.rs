use liblrhsmm_rs::MemoryPool;

#[test]
fn page_growth_and_zeroed_regions_match_c() {
    let requests = [
        (false, 1, 0),
        (false, 1, 3),
        (true, 1, 5),
        (false, 1, 1),
        (true, 3, 4),
        (true, 0, 17),
        (false, 1, 2),
        (true, 2, 3),
    ];
    let mut reference = include_str!("fixtures/c-memory-pool.txt").lines();
    for (scenario, page_size) in [8, 0].into_iter().enumerate() {
        let mut pool = MemoryPool::new(page_size).unwrap();
        assert_eq!(pool.page_size(), page_size);
        assert_eq!(pool.page_count(), 1);
        assert_eq!(pool.total_capacity(), page_size);
        let mut allocations = Vec::new();
        let mut last_top = 0;
        let mut last_page = 1;
        let mut allocated_bytes = 0;
        for (index, (zeroed, number, size)) in requests.into_iter().enumerate() {
            let allocation = if zeroed {
                pool.allocate_zeroed(number, size).unwrap()
            } else {
                pool.allocate(number * size).unwrap()
            };
            let length = number * size;
            assert_eq!(allocation.len(), length);
            assert_eq!(allocation.is_empty(), length == 0);
            let data = pool.get_mut(&allocation).unwrap();
            assert!(data.iter().all(|&value| value == 0));
            data.fill(index as u8 + 1);
            let offset = if last_page == pool.page_count() {
                last_top
            } else {
                0
            };
            last_top = offset + length;
            last_page = pool.page_count();
            allocated_bytes += length;
            assert_eq!(pool.allocated_bytes(), allocated_bytes);
            assert_eq!(
                reference.next().unwrap(),
                format!(
                    "A {scenario} {index} {} {} {last_top} {offset} {length} 1",
                    pool.page_count(),
                    pool.total_capacity()
                )
            );
            allocations.push(allocation);
        }
        for (index, allocation) in allocations.iter().enumerate() {
            let checksum: usize = pool
                .get(allocation)
                .unwrap()
                .iter()
                .map(|&v| v as usize)
                .sum();
            assert_eq!(
                reference.next().unwrap(),
                format!("V {scenario} {index} {checksum}")
            );
        }
    }
    assert!(reference.next().is_none());
}

#[test]
fn failed_allocations_preserve_pages_contents_and_accounting() {
    let mut pool = MemoryPool::new(8).unwrap();
    let allocation = pool.allocate(3).unwrap();
    pool.get_mut(&allocation)
        .unwrap()
        .copy_from_slice(&[2, 4, 8]);
    assert!(pool.allocate_zeroed(usize::MAX, 2).is_err());
    assert!(pool.allocate(usize::MAX).is_err());
    assert!(pool.allocate(isize::MAX as usize + 1).is_err());
    assert_eq!(pool.page_count(), 1);
    assert_eq!(pool.total_capacity(), 8);
    assert_eq!(pool.allocated_bytes(), 3);
    assert_eq!(pool.get(&allocation).unwrap(), [2, 4, 8]);
    assert!(MemoryPool::new(usize::MAX).is_err());
}

#[test]
fn handles_belong_to_their_pool_and_clones_share_one_region() {
    let mut pool = MemoryPool::new(4).unwrap();
    let allocation = pool.allocate(3).unwrap();
    let copy = allocation.clone();
    pool.get_mut(&copy).unwrap().copy_from_slice(&[1, 3, 5]);
    assert_eq!(pool.get(&allocation).unwrap(), [1, 3, 5]);
    let mut other = MemoryPool::new(4).unwrap();
    assert!(other.get(&allocation).is_none());
    assert!(other.get_mut(&allocation).is_none());
    let empty = pool.allocate(0).unwrap();
    assert_eq!(pool.get(&empty).unwrap(), []);
    assert!(other.get(&empty).is_none());
    drop(pool);
    assert!(other.get(&copy).is_none());
}
