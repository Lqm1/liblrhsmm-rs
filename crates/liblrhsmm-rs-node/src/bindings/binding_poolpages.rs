use super::*;
#[napi]
pub struct PoolPages {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::PoolPages>>,
}
#[napi]
impl PoolPages {
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
impl PoolPages {
    #[napi(getter, js_name = "length")]
    pub fn binding_length(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .length() as f64)
    }
    #[napi(js_name = "is_empty")]
    pub fn binding_is_empty(&self) -> napi::Result<bool> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .is_empty())
    }
    #[napi(js_name = "next")]
    pub fn binding_next_value(&self) -> napi::Result<Option<PoolPageUsage>> {
        Ok(((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .next_value())
        .map(|value| PoolPageUsage {
            inner: std::cell::RefCell::new(Some(value)),
        }))
    }
    #[napi(js_name = "next_back")]
    pub fn binding_next_back_value(&self) -> napi::Result<Option<PoolPageUsage>> {
        Ok(((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .next_back_value())
        .map(|value| PoolPageUsage {
            inner: std::cell::RefCell::new(Some(value)),
        }))
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(PoolPages {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .cloned(),
            )),
        })
    }
}
