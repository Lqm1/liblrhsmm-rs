use liblrhsmm_rs::MemoryPool;
use liblrhsmm_rs_capi::*;

use std::ptr::{null, null_mut};

unsafe fn layout(pool: *const LiblrhsmmRsMemoryPool) -> LiblrhsmmRsPoolLayout {
    let mut result = LiblrhsmmRsPoolLayout {
        page_size: 99,
        pages: 99,
        total_capacity: 99,
        allocated_bytes: 99,
    };
    // SAFETY: Test owns a live readable pool and independent initialized output.
    assert_eq!(
        unsafe { liblrhsmm_rs_memory_pool_layout(pool, &mut result) },
        0
    );
    result
}

#[test]
fn complete_pool_growth_regions_and_native_accounting_match_original_c() {
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
    let mut reference = include_str!("../../../tests/fixtures/c-memory-pool.txt").lines();
    // SAFETY: Test keeps unique live owners, independent buffers and exclusive mutation.
    unsafe {
        for (scenario, page_size) in [8, 0].into_iter().enumerate() {
            let mut native = MemoryPool::new(page_size).unwrap();
            let mut pool = null_mut();
            assert_eq!(liblrhsmm_rs_memory_pool_new(page_size, &mut pool), 0);
            let mut allocations = Vec::new();
            let mut last_top = 0;
            let mut last_page = 1;
            for (index, (zeroed, number, size)) in requests.into_iter().enumerate() {
                let allocation = if zeroed {
                    native.allocate_zeroed(number, size).unwrap()
                } else {
                    native.allocate(number * size).unwrap()
                };
                let mut owner = null_mut();
                let status = if zeroed {
                    liblrhsmm_rs_memory_pool_allocate_zeroed(pool, number, size, &mut owner)
                } else {
                    liblrhsmm_rs_memory_pool_allocate(pool, number * size, &mut owner)
                };
                assert_eq!(status, 0);
                let mut length = 99;
                assert_eq!(liblrhsmm_rs_pool_allocation_length(owner, &mut length), 0);
                assert_eq!(length, allocation.len());
                let mut initial = vec![99; length];
                assert_eq!(
                    liblrhsmm_rs_memory_pool_copy_bytes(
                        pool,
                        owner,
                        0,
                        initial.as_mut_ptr(),
                        length
                    ),
                    0
                );
                assert_eq!(initial, native.get(&allocation).unwrap());
                let values = vec![index as u8 + 1; length];
                assert_eq!(
                    liblrhsmm_rs_memory_pool_set_bytes(pool, owner, 0, values.as_ptr(), length),
                    0
                );
                native
                    .get_mut(&allocation)
                    .unwrap()
                    .copy_from_slice(&values);
                let shape = layout(pool);
                assert_eq!(
                    shape,
                    LiblrhsmmRsPoolLayout {
                        page_size,
                        pages: native.page_count(),
                        total_capacity: native.total_capacity(),
                        allocated_bytes: native.allocated_bytes()
                    }
                );
                for (page, expected) in native.pages().enumerate() {
                    let mut actual = LiblrhsmmRsPoolPage {
                        used_bytes: 99,
                        capacity: 99,
                    };
                    assert_eq!(liblrhsmm_rs_memory_pool_page(pool, page, &mut actual), 0);
                    assert_eq!(
                        actual,
                        LiblrhsmmRsPoolPage {
                            used_bytes: expected.used_bytes,
                            capacity: expected.capacity
                        }
                    );
                }
                let offset = if last_page == shape.pages {
                    last_top
                } else {
                    0
                };
                last_top = offset + length;
                last_page = shape.pages;
                assert_eq!(
                    reference.next().unwrap(),
                    format!(
                        "A {scenario} {index} {} {} {last_top} {offset} {length} 1",
                        shape.pages, shape.total_capacity
                    )
                );
                allocations.push(owner);
            }
            for (index, owner) in allocations.iter_mut().enumerate() {
                let mut length = 0;
                assert_eq!(liblrhsmm_rs_pool_allocation_length(*owner, &mut length), 0);
                let mut copy = vec![99; length];
                assert_eq!(
                    liblrhsmm_rs_memory_pool_copy_bytes(pool, *owner, 0, copy.as_mut_ptr(), length),
                    0
                );
                let checksum = copy.iter().map(|value| usize::from(*value)).sum::<usize>();
                assert_eq!(
                    reference.next().unwrap(),
                    format!("V {scenario} {index} {checksum}")
                );
                assert_eq!(liblrhsmm_rs_pool_allocation_release(owner), 0);
                assert!(owner.is_null());
            }
            assert_eq!(liblrhsmm_rs_memory_pool_release(&mut pool), 0);
            assert!(pool.is_null());
        }
    }
    assert!(reference.next().is_none());
}

#[test]
fn shared_region_identity_and_failures_preserve_bytes_outputs_and_accounting() {
    // SAFETY: Unique owner slots, independent buffers and exclusive pool mutations.
    unsafe {
        let mut pool = null_mut();
        let mut other = null_mut();
        let mut owner = null_mut();
        let mut clone = null_mut();
        assert_eq!(liblrhsmm_rs_memory_pool_new(4, &mut pool), 0);
        assert_eq!(liblrhsmm_rs_memory_pool_new(4, &mut other), 0);
        assert_eq!(liblrhsmm_rs_memory_pool_allocate(pool, 3, &mut owner), 0);
        assert_eq!(liblrhsmm_rs_pool_allocation_clone(owner, &mut clone), 0);
        assert_eq!(
            liblrhsmm_rs_memory_pool_set_bytes(pool, clone, 0, [1, 3, 5].as_ptr(), 3),
            0
        );
        let mut values = [99; 3];
        assert_eq!(
            liblrhsmm_rs_memory_pool_copy_bytes(pool, owner, 0, values.as_mut_ptr(), 3),
            0
        );
        assert_eq!(values, [1, 3, 5]);
        let before = layout(pool);
        assert_eq!(liblrhsmm_rs_memory_pool_allocate(pool, 100, null_mut()), 1);
        assert_eq!(
            liblrhsmm_rs_memory_pool_allocate_zeroed(pool, 100, 100, null_mut()),
            1
        );
        let mut sentinel = owner;
        for size in [usize::MAX, isize::MAX as usize + 1] {
            assert_eq!(
                liblrhsmm_rs_memory_pool_allocate(pool, size, &mut sentinel),
                3
            );
            assert_eq!(sentinel, owner);
        }
        assert_eq!(
            liblrhsmm_rs_memory_pool_allocate_zeroed(pool, usize::MAX, 2, &mut sentinel),
            3
        );
        assert_eq!(sentinel, owner);
        assert_eq!(layout(pool), before);
        let mut page = LiblrhsmmRsPoolPage {
            used_bytes: 71,
            capacity: 71,
        };
        assert_eq!(liblrhsmm_rs_memory_pool_page(pool, 99, &mut page), 2);
        assert_eq!(
            page,
            LiblrhsmmRsPoolPage {
                used_bytes: 71,
                capacity: 71
            }
        );
        assert_eq!(
            liblrhsmm_rs_memory_pool_copy_bytes(other, owner, 0, values.as_mut_ptr(), 3),
            3
        );
        assert_eq!(
            liblrhsmm_rs_memory_pool_set_bytes(other, owner, 0, [9, 9, 9].as_ptr(), 3),
            3
        );
        for offset in [2, usize::MAX] {
            values = [71; 3];
            assert_eq!(
                liblrhsmm_rs_memory_pool_copy_bytes(pool, owner, offset, values.as_mut_ptr(), 3),
                2
            );
            assert_eq!(values, [71; 3]);
            assert_eq!(
                liblrhsmm_rs_memory_pool_set_bytes(pool, owner, offset, [9, 9, 9].as_ptr(), 3),
                2
            );
        }
        assert_eq!(
            liblrhsmm_rs_memory_pool_copy_bytes(pool, owner, 3, null_mut(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_memory_pool_set_bytes(pool, owner, 3, null(), 0),
            0
        );
        assert_eq!(
            liblrhsmm_rs_memory_pool_copy_bytes(pool, owner, 4, null_mut(), 0),
            2
        );
        assert_eq!(
            liblrhsmm_rs_memory_pool_copy_bytes(pool, owner, 0, null_mut(), 1),
            1
        );
        assert_eq!(
            liblrhsmm_rs_memory_pool_set_bytes(pool, owner, 0, null(), 1),
            1
        );
        assert_eq!(
            liblrhsmm_rs_memory_pool_copy_bytes(pool, owner, 0, null_mut(), usize::MAX),
            2
        );
        assert_eq!(liblrhsmm_rs_pool_allocation_length(owner, null_mut()), 1);
        assert_eq!(liblrhsmm_rs_pool_allocation_clone(owner, null_mut()), 1);
        assert_eq!(liblrhsmm_rs_memory_pool_layout(pool, null_mut()), 1);
        assert_eq!(liblrhsmm_rs_memory_pool_new(usize::MAX, &mut pool), 3);
        assert_eq!(layout(pool), before);
        assert_eq!(
            liblrhsmm_rs_memory_pool_copy_bytes(pool, clone, 0, values.as_mut_ptr(), 3),
            0
        );
        assert_eq!(values, [1, 3, 5]);
        assert_eq!(liblrhsmm_rs_pool_allocation_release(&mut owner), 0);
        assert_eq!(liblrhsmm_rs_memory_pool_release(&mut pool), 0);
        let mut length = 99;
        assert_eq!(liblrhsmm_rs_pool_allocation_length(clone, &mut length), 0);
        assert_eq!(length, 3);
        assert_eq!(
            liblrhsmm_rs_memory_pool_copy_bytes(other, clone, 0, values.as_mut_ptr(), 3),
            3
        );
        let mut retained = null_mut();
        assert_eq!(liblrhsmm_rs_pool_allocation_clone(clone, &mut retained), 0);
        assert_eq!(liblrhsmm_rs_pool_allocation_release(&mut clone), 0);
        assert_eq!(liblrhsmm_rs_pool_allocation_release(&mut retained), 0);
        assert_eq!(liblrhsmm_rs_pool_allocation_release(&mut retained), 0);
        assert_eq!(liblrhsmm_rs_pool_allocation_release(null_mut()), 1);
        assert_eq!(liblrhsmm_rs_memory_pool_release(&mut pool), 0);
        assert_eq!(liblrhsmm_rs_memory_pool_release(null_mut()), 1);
        assert_eq!(liblrhsmm_rs_memory_pool_release(&mut other), 0);
    }
}
