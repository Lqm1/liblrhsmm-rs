use super::*;
#[napi]
pub struct MemoryPool {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::MemoryPool>>,
}
#[napi]
impl MemoryPool {
    #[napi]
    pub fn close(&self) -> napi::Result<()> {
        *self.inner.try_borrow_mut().map_err(borrowed)? = None;
        Ok(())
    }
    #[napi]
    pub fn free(&self) -> napi::Result<()> {
        self.close()
    }
    #[napi(getter, js_name = "is_closed")]
    pub fn is_closed(&self) -> napi::Result<bool> {
        Ok(self.inner.try_borrow().map_err(borrowed)?.is_none())
    }
}
#[napi]
impl MemoryPool {
    #[napi(constructor)]
    pub fn binding_new(page_size: f64) -> napi::Result<Self> {
        (crate::api::MemoryPool::new(checked_usize(page_size)?)).map(|value| MemoryPool {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(getter, js_name = "page_size")]
    pub fn binding_page_size(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .page_size() as f64)
    }
    #[napi(getter, js_name = "page_count")]
    pub fn binding_page_count(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .page_count() as f64)
    }
    #[napi(getter, js_name = "total_capacity")]
    pub fn binding_total_capacity(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .total_capacity() as f64)
    }
    #[napi(getter, js_name = "allocated_bytes")]
    pub fn binding_allocated_bytes(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .allocated_bytes() as f64)
    }
    #[napi(js_name = "pages")]
    pub fn binding_pages(&self) -> napi::Result<PoolPages> {
        Ok(PoolPages {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .pages(),
            )),
        })
    }
    #[napi(js_name = "page")]
    pub fn binding_page(&self, index: f64) -> napi::Result<Option<PoolPageUsage>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .page(checked_usize(index)?))
        .map(|value| PoolPageUsage {
            inner: std::cell::RefCell::new(Some(value)),
        }))
    }
    #[napi(js_name = "allocate")]
    pub fn binding_allocate(&self, size: f64) -> napi::Result<PoolAllocation> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .allocate(checked_usize(size)?))
        .map(|value| PoolAllocation {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "allocate_zeroed")]
    pub fn binding_allocate_zeroed(&self, number: f64, size: f64) -> napi::Result<PoolAllocation> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .allocate_zeroed(checked_usize(number)?, checked_usize(size)?))
        .map(|value| PoolAllocation {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "get")]
    pub fn binding_get(
        &self,
        allocation: &PoolAllocation,
    ) -> napi::Result<Option<napi::bindgen_prelude::Uint8Array>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .get(
            allocation
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(<napi::bindgen_prelude::Uint8Array>::from))
    }
    #[napi(js_name = "get_range")]
    pub fn binding_get_range(
        &self,
        allocation: &PoolAllocation,
        offset: f64,
        length: f64,
    ) -> napi::Result<napi::bindgen_prelude::Uint8Array> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .get_range(
            allocation
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            checked_usize(offset)?,
            checked_usize(length)?,
        ))
        .map(<napi::bindgen_prelude::Uint8Array>::from)
    }
    #[napi(js_name = "set_bytes")]
    pub fn binding_set_bytes(
        &self,
        allocation: &PoolAllocation,
        offset: f64,
        values: napi::bindgen_prelude::Uint8Array,
    ) -> napi::Result<()> {
        let storage_values = values.to_vec();
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_bytes(
            allocation
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            checked_usize(offset)?,
            &storage_values,
        )
    }
}
