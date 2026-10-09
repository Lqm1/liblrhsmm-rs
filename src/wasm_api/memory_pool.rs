use crate::memory_pool::{
    MemoryPool as NativeMemoryPool, PageUsage, PoolAllocation as NativeAllocation,
};
use wasm_bindgen::prelude::*;

fn error(value: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&value.to_string())
}

/// A copied page descriptor, independent of the owning pool.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct PoolPageUsage {
    pub used_bytes: usize,
    pub capacity: usize,
}
impl From<PageUsage> for PoolPageUsage {
    fn from(value: PageUsage) -> Self {
        Self {
            used_bytes: value.used_bytes,
            capacity: value.capacity,
        }
    }
}

/// An exact-size snapshot of page usage in native allocation order.
#[wasm_bindgen]
#[derive(Clone)]
pub struct PoolPages {
    inner: std::vec::IntoIter<PageUsage>,
}
impl Iterator for PoolPages {
    type Item = PoolPageUsage;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(Into::into)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
impl DoubleEndedIterator for PoolPages {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(Into::into)
    }
}
impl ExactSizeIterator for PoolPages {}
impl std::iter::FusedIterator for PoolPages {}
#[wasm_bindgen]
impl PoolPages {
    #[wasm_bindgen(getter)]
    pub fn length(&self) -> usize {
        self.inner.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.len() == 0
    }
    #[wasm_bindgen(js_name = next)]
    pub fn next_value(&mut self) -> Option<PoolPageUsage> {
        Iterator::next(self)
    }
    #[wasm_bindgen(js_name = next_back)]
    pub fn next_back_value(&mut self) -> Option<PoolPageUsage> {
        DoubleEndedIterator::next_back(self)
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}

/// Shared region identity; releasing this handle does not reclaim pool bytes.
#[wasm_bindgen]
#[derive(Clone)]
pub struct PoolAllocation {
    inner: NativeAllocation,
}
#[wasm_bindgen]
impl PoolAllocation {
    #[wasm_bindgen(getter)]
    pub fn length(&self) -> usize {
        self.inner.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}

/// Paged byte storage using the native growth policy and ownership checks.
#[wasm_bindgen]
pub struct MemoryPool {
    inner: NativeMemoryPool,
}
#[wasm_bindgen]
impl MemoryPool {
    #[wasm_bindgen(constructor)]
    pub fn new(page_size: usize) -> Result<Self, JsValue> {
        NativeMemoryPool::new(page_size)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    #[wasm_bindgen(getter)]
    pub fn page_size(&self) -> usize {
        self.inner.page_size()
    }
    #[wasm_bindgen(getter)]
    pub fn page_count(&self) -> usize {
        self.inner.page_count()
    }
    #[wasm_bindgen(getter)]
    pub fn total_capacity(&self) -> usize {
        self.inner.total_capacity()
    }
    #[wasm_bindgen(getter)]
    pub fn allocated_bytes(&self) -> usize {
        self.inner.allocated_bytes()
    }
    pub fn pages(&self) -> PoolPages {
        PoolPages {
            inner: self.inner.pages().collect::<Vec<_>>().into_iter(),
        }
    }
    pub fn page(&self, index: usize) -> Option<PoolPageUsage> {
        self.inner.pages().nth(index).map(Into::into)
    }
    pub fn allocate(&mut self, size: usize) -> Result<PoolAllocation, JsValue> {
        self.inner
            .allocate(size)
            .map(|inner| PoolAllocation { inner })
            .map_err(error)
    }
    pub fn allocate_zeroed(
        &mut self,
        number: usize,
        size: usize,
    ) -> Result<PoolAllocation, JsValue> {
        self.inner
            .allocate_zeroed(number, size)
            .map(|inner| PoolAllocation { inner })
            .map_err(error)
    }
    /// Return an independent byte copy, or absence for another pool's handle.
    pub fn get(&self, allocation: &PoolAllocation) -> Option<Vec<u8>> {
        self.inner.get(&allocation.inner).map(<[u8]>::to_vec)
    }
    pub fn get_range(
        &self,
        allocation: &PoolAllocation,
        offset: usize,
        length: usize,
    ) -> Result<Vec<u8>, JsValue> {
        let bytes = self
            .inner
            .get(&allocation.inner)
            .ok_or_else(|| error("allocation belongs to another memory pool"))?;
        let end = offset
            .checked_add(length)
            .ok_or_else(|| error("memory pool range overflow"))?;
        bytes
            .get(offset..end)
            .map(<[u8]>::to_vec)
            .ok_or_else(|| error("memory pool range out of bounds"))
    }
    /// Native mutable-slice access through an atomic checked byte replacement.
    pub fn set_bytes(
        &mut self,
        allocation: &PoolAllocation,
        offset: usize,
        values: &[u8],
    ) -> Result<(), JsValue> {
        let bytes = self
            .inner
            .get_mut(&allocation.inner)
            .ok_or_else(|| error("allocation belongs to another memory pool"))?;
        let end = offset
            .checked_add(values.len())
            .ok_or_else(|| error("memory pool range overflow"))?;
        let target = bytes
            .get_mut(offset..end)
            .ok_or_else(|| error("memory pool range out of bounds"))?;
        target.copy_from_slice(values);
        Ok(())
    }
}
