//! Append-only byte storage with the original liblrhsmm page growth policy.
//!
//! Allocation handles remain valid as the pool grows. Access uses borrowed byte
//! slices rather than untyped pointers; all exposed bytes are initialized.

use std::sync::Arc;

use crate::ModelError;

#[derive(Debug)]
struct Page {
    bytes: Vec<u8>,
    capacity: usize,
}

impl Page {
    fn new(capacity: usize) -> Result<Self, ModelError> {
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(capacity)
            .map_err(|_| ModelError("memory pool page allocation failed"))?;
        Ok(Self { bytes, capacity })
    }
}

/// An allocation owned by one memory pool. Cloning the handle shares its region.
/// Dropping a handle does not reclaim the region; dropping the pool frees pages.
#[derive(Debug, Clone)]
pub struct PoolAllocation {
    owner: Arc<()>,
    page: usize,
    offset: usize,
    length: usize,
}

impl PoolAllocation {
    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

/// Paged byte arena corresponding to `lrh_mempool`.
///
/// Pages grow by `page_size`, except that an oversized request gets its own
/// larger page. Only the last page accepts new allocations, as in C. Capacities
/// report the requested page sizes, not allocator-specific vector capacities.
#[derive(Debug)]
pub struct MemoryPool {
    owner: Arc<()>,
    pages: Vec<Page>,
    page_size: usize,
    total_capacity: usize,
    allocated_bytes: usize,
}

/// Read-only accounting for one pool page, in allocation order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageUsage {
    pub used_bytes: usize,
    pub capacity: usize,
}

impl MemoryPool {
    /// Create one page, including when `page_size` is zero.
    pub fn new(page_size: usize) -> Result<Self, ModelError> {
        let page = Page::new(page_size)?;
        let mut pages = Vec::new();
        pages
            .try_reserve_exact(1)
            .map_err(|_| ModelError("memory pool page list allocation failed"))?;
        pages.push(page);
        Ok(Self {
            owner: Arc::new(()),
            pages,
            page_size,
            total_capacity: page_size,
            allocated_bytes: 0,
        })
    }

    pub fn page_size(&self) -> usize {
        self.page_size
    }

    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    /// Inspect every page without exposing owning pointers or unused bytes.
    pub fn pages(&self) -> impl ExactSizeIterator<Item = PageUsage> + '_ {
        self.pages.iter().map(|page| PageUsage {
            used_bytes: page.bytes.len(),
            capacity: page.capacity,
        })
    }

    pub fn total_capacity(&self) -> usize {
        self.total_capacity
    }

    pub fn allocated_bytes(&self) -> usize {
        self.allocated_bytes
    }

    /// Allocate a region. Unlike C's uninitialized `malloc`, bytes start at zero.
    /// Previous allocations and accounting remain unchanged on failure.
    pub fn allocate(&mut self, size: usize) -> Result<PoolAllocation, ModelError> {
        let allocated_bytes = self
            .allocated_bytes
            .checked_add(size)
            .ok_or(ModelError("memory pool allocation size overflow"))?;
        let last = &self.pages[self.pages.len() - 1];
        if size > last.capacity - last.bytes.len() {
            let capacity = self.page_size.max(size);
            let total_capacity = self
                .total_capacity
                .checked_add(capacity)
                .ok_or(ModelError("memory pool capacity overflow"))?;
            let page = Page::new(capacity)?;
            self.pages
                .try_reserve(1)
                .map_err(|_| ModelError("memory pool page list allocation failed"))?;
            self.pages.push(page);
            self.total_capacity = total_capacity;
        }
        let page_index = self.pages.len() - 1;
        let page = &mut self.pages[page_index];
        let offset = page.bytes.len();
        // Capacity was checked or reserved above, so resize cannot reallocate.
        page.bytes.resize(offset + size, 0);
        self.allocated_bytes = allocated_bytes;
        Ok(PoolAllocation {
            owner: Arc::clone(&self.owner),
            page: page_index,
            offset,
            length: size,
        })
    }

    /// Allocate `number * size` initialized bytes, rejecting multiplication overflow.
    pub fn allocate_zeroed(
        &mut self,
        number: usize,
        size: usize,
    ) -> Result<PoolAllocation, ModelError> {
        let length = number
            .checked_mul(size)
            .ok_or(ModelError("memory pool element count overflow"))?;
        self.allocate(length)
    }

    /// Borrow a region, or return `None` for a handle from a different pool.
    pub fn get(&self, allocation: &PoolAllocation) -> Option<&[u8]> {
        if !Arc::ptr_eq(&self.owner, &allocation.owner) {
            return None;
        }
        self.pages[allocation.page]
            .bytes
            .get(allocation.offset..allocation.offset + allocation.length)
    }

    /// Mutably borrow a region, or reject a handle from a different pool.
    pub fn get_mut(&mut self, allocation: &PoolAllocation) -> Option<&mut [u8]> {
        if !Arc::ptr_eq(&self.owner, &allocation.owner) {
            return None;
        }
        self.pages[allocation.page]
            .bytes
            .get_mut(allocation.offset..allocation.offset + allocation.length)
    }
}
