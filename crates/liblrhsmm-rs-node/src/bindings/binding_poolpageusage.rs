use super::*;
#[napi]
pub struct PoolPageUsage {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::PoolPageUsage>>,
}
#[napi]
impl PoolPageUsage {
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
impl PoolPageUsage {
    #[napi(getter, js_name = "used_bytes")]
    pub fn binding_get_used_bytes(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .used_bytes as f64)
    }
    #[napi(setter, js_name = "used_bytes")]
    pub fn binding_set_used_bytes(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .used_bytes = checked_usize(value)?;
        Ok(())
    }
    #[napi(getter, js_name = "capacity")]
    pub fn binding_get_capacity(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .capacity as f64)
    }
    #[napi(setter, js_name = "capacity")]
    pub fn binding_set_capacity(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .capacity = checked_usize(value)?;
        Ok(())
    }
}
