//! Paged byte storage with checked allocation handles and native accounting.
use super::bytes::{byte_input, byte_range};
use super::{boundary, range, scalar_result};
use crate::ModelError;
use crate::memory_pool::{MemoryPool, PoolAllocation};

/// Independent native pool owner. Allocation handles identify regions but do not keep pages alive.
pub struct LiblrhsmmRsMemoryPool {
    pool: MemoryPool,
}
/// Native region identity. Cloning shares one region; release does not reclaim its bytes.
pub struct LiblrhsmmRsPoolAllocation {
    allocation: PoolAllocation,
}
/// Complete native pool accounting, including its initial empty page.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiblrhsmmRsPoolLayout {
    pub page_size: usize,
    pub pages: usize,
    pub total_capacity: usize,
    pub allocated_bytes: usize,
}
/// Requested capacity and used bytes of one page, independent of allocator capacity.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiblrhsmmRsPoolPage {
    pub used_bytes: usize,
    pub capacity: usize,
}

/// Create a native pool, including a zero-capacity initial page when requested.
/// # Safety
/// Output is independent aligned writable owner storage holding no live owner.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_memory_pool_new(
    page_size: usize,
    output: *mut *mut LiblrhsmmRsMemoryPool,
) -> u32 {
    // SAFETY: Caller supplies independent owner output storage.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsMemoryPool {
                pool: MemoryPool::new(page_size)?,
            })))
        })
    }
}

/// Copy complete pool accounting. Errors preserve the initialized descriptor.
/// # Safety
/// Pool is a live owner; output is independent aligned writable storage.
/// Reads cannot overlap pool mutation or release.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_memory_pool_layout(
    pool: *const LiblrhsmmRsMemoryPool,
    output: *mut LiblrhsmmRsPoolLayout,
) -> u32 {
    if let Err(status) = range(pool, 1) {
        return status;
    }
    // SAFETY: Caller provides a live readable pool and independent output.
    unsafe {
        scalar_result(output, || {
            let pool = &(*pool).pool;
            Ok(LiblrhsmmRsPoolLayout {
                page_size: pool.page_size(),
                pages: pool.page_count(),
                total_capacity: pool.total_capacity(),
                allocated_bytes: pool.allocated_bytes(),
            })
        })
    }
}

/// Copy one page's native usage in allocation order. Invalid indices return two.
/// # Safety
/// Pool is live and readable; output is independent aligned writable storage.
/// Reads cannot overlap pool mutation or release.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_memory_pool_page(
    pool: *const LiblrhsmmRsMemoryPool,
    index: usize,
    output: *mut LiblrhsmmRsPoolPage,
) -> u32 {
    if let Err(status) = range(pool, 1) {
        return status;
    }
    // SAFETY: Caller provides a live readable pool.
    let pool = unsafe { &(*pool).pool };
    if index >= pool.page_count() {
        return 2;
    }
    // SAFETY: Caller provides independent writable output storage.
    unsafe {
        scalar_result(output, || {
            let page = pool
                .pages()
                .nth(index)
                .ok_or(ModelError("pool page is missing"))?;
            Ok(LiblrhsmmRsPoolPage {
                used_bytes: page.used_bytes,
                capacity: page.capacity,
            })
        })
    }
}

/// Allocate native initialized bytes. Existing regions remain valid across page growth.
/// # Safety
/// Pool is a live exclusively writable owner; output is independent aligned
/// writable owner storage holding no live owner. Allocation cannot overlap any
/// other pool access or release. Output validation precedes pool mutation.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_memory_pool_allocate(
    pool: *mut LiblrhsmmRsMemoryPool,
    size: usize,
    output: *mut *mut LiblrhsmmRsPoolAllocation,
) -> u32 {
    if let Err(status) = range(pool, 1) {
        return status;
    }
    // SAFETY: Caller provides exclusive pool access and independent owner output.
    unsafe {
        scalar_result(output, || {
            let allocation = (*pool).pool.allocate(size)?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsPoolAllocation {
                allocation,
            })))
        })
    }
}

/// Allocate number*size native zeroed bytes, rejecting multiplication overflow.
/// # Safety
/// Pool is a live exclusively writable owner; output is independent aligned
/// writable owner storage holding no live owner. No other pool access or release
/// overlaps this call. Output validation precedes pool mutation.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_memory_pool_allocate_zeroed(
    pool: *mut LiblrhsmmRsMemoryPool,
    number: usize,
    size: usize,
    output: *mut *mut LiblrhsmmRsPoolAllocation,
) -> u32 {
    if let Err(status) = range(pool, 1) {
        return status;
    }
    // SAFETY: Caller provides exclusive pool access and independent owner output.
    unsafe {
        scalar_result(output, || {
            let allocation = (*pool).pool.allocate_zeroed(number, size)?;
            Ok(Box::into_raw(Box::new(LiblrhsmmRsPoolAllocation {
                allocation,
            })))
        })
    }
}

/// Copy one checked region interval. A handle from another pool returns three.
/// # Safety
/// Pool and allocation are live readable owners. Output is independent writable
/// storage for count bytes and cannot alias owner storage. Reads cannot overlap
/// mutation or release. Empty copies permit null output at valid offsets.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_memory_pool_copy_bytes(
    pool: *const LiblrhsmmRsMemoryPool,
    allocation: *const LiblrhsmmRsPoolAllocation,
    offset: usize,
    output: *mut u8,
    count: usize,
) -> u32 {
    for result in [
        range(pool, 1),
        range(allocation, 1),
        byte_range(output.cast_const(), count),
    ] {
        if let Err(status) = result {
            return status;
        }
    }
    let Some(end) = offset.checked_add(count) else {
        return 2;
    };
    // SAFETY: Caller supplies readable owners with no overlapping mutation/release.
    let Some(region) = (unsafe { &(*pool).pool }).get(unsafe { &(*allocation).allocation }) else {
        return 3;
    };
    let Some(bytes) = region.get(offset..end) else {
        return 2;
    };
    if count != 0 {
        // SAFETY: Independent initialized source and valid writable caller buffer.
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), output, count);
        }
    }
    0
}

/// Replace one checked region interval. Errors retain all bytes and accounting.
/// # Safety
/// Pool is a live exclusively writable owner; allocation is a live readable owner.
/// Input is independent initialized readable storage for count bytes and cannot
/// alias pool storage. No other pool access or owner release overlaps this call.
/// Empty replacements permit null input at valid offsets.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_memory_pool_set_bytes(
    pool: *mut LiblrhsmmRsMemoryPool,
    allocation: *const LiblrhsmmRsPoolAllocation,
    offset: usize,
    input: *const u8,
    count: usize,
) -> u32 {
    for result in [
        range(pool, 1),
        range(allocation, 1),
        byte_range(input, count),
    ] {
        if let Err(status) = result {
            return status;
        }
    }
    let Some(end) = offset.checked_add(count) else {
        return 2;
    };
    // SAFETY: Caller supplies exclusive pool access and a readable allocation owner.
    let Some(region) = (unsafe { &mut (*pool).pool }).get_mut(unsafe { &(*allocation).allocation })
    else {
        return 3;
    };
    let Some(bytes) = region.get_mut(offset..end) else {
        return 2;
    };
    // SAFETY: Caller input is initialized, readable and independent of pool storage.
    bytes.copy_from_slice(unsafe { byte_input(input, count) });
    0
}

/// Copy the allocation length. Valid even after its original pool is released.
/// # Safety
/// Allocation is a live readable owner; output is independent aligned writable
/// usize storage. Reads cannot overlap allocation release.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_pool_allocation_length(
    allocation: *const LiblrhsmmRsPoolAllocation,
    output: *mut usize,
) -> u32 {
    if let Err(status) = range(allocation, 1) {
        return status;
    }
    // SAFETY: Caller supplies readable allocation and independent output.
    unsafe { scalar_result(output, || Ok((*allocation).allocation.len())) }
}

/// Clone the native identity, sharing the same region without copying its bytes.
/// # Safety
/// Allocation is a live readable owner; output is independent aligned writable
/// owner storage holding no live owner. Reads cannot overlap allocation release.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_pool_allocation_clone(
    allocation: *const LiblrhsmmRsPoolAllocation,
    output: *mut *mut LiblrhsmmRsPoolAllocation,
) -> u32 {
    if let Err(status) = range(allocation, 1) {
        return status;
    }
    // SAFETY: Caller supplies readable allocation and independent output.
    unsafe {
        scalar_result(output, || {
            Ok(Box::into_raw(Box::new(LiblrhsmmRsPoolAllocation {
                allocation: (*allocation).allocation.clone(),
            })))
        })
    }
}

unsafe fn release<T: std::panic::RefUnwindSafe>(owner: *mut *mut T) -> u32 {
    if let Err(status) = range(owner.cast_const(), 1) {
        return status;
    }
    // SAFETY: Public caller requires exclusive initialized unique-owner storage.
    let pointer = unsafe { owner.read() };
    if pointer.is_null() {
        return 0;
    }
    if let Err(status) = range(pointer.cast_const(), 1) {
        return status;
    }
    match boundary(|| {
        // SAFETY: Caller transfers unique ownership; slot is cleared before drop.
        unsafe {
            owner.write(std::ptr::null_mut());
            drop(Box::from_raw(pointer));
        }
        Ok(())
    }) {
        Ok(()) => 0,
        Err(status) => status,
    }
}

/// Release the allocation identity and clear its slot. Pool bytes are not reclaimed.
/// # Safety
/// Slot is aligned initialized exclusively writable storage containing a unique
/// live allocation owner or null. Release cannot overlap any allocation access.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_pool_allocation_release(
    owner: *mut *mut LiblrhsmmRsPoolAllocation,
) -> u32 {
    // SAFETY: Caller supplies initialized exclusive unique ownership.
    unsafe { release(owner) }
}

/// Release all pool pages and clear its slot. Remaining allocation handles keep
/// metadata only; another pool cannot use their regions.
/// # Safety
/// Slot is aligned initialized exclusively writable storage containing a unique
/// live pool owner or null. Release cannot overlap any pool access.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn liblrhsmm_rs_memory_pool_release(
    owner: *mut *mut LiblrhsmmRsMemoryPool,
) -> u32 {
    // SAFETY: Caller supplies initialized exclusive unique ownership.
    unsafe { release(owner) }
}
