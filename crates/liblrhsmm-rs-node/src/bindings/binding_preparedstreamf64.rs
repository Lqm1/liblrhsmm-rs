use super::*;
#[napi]
pub struct PreparedStreamF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::PreparedStreamF64>>,
}
#[napi]
impl PreparedStreamF64 {
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
impl PreparedStreamF64 {
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(PreparedStreamF64 {
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
    #[napi(js_name = "parameters")]
    pub fn binding_parameters(&self) -> napi::Result<StreamF64> {
        Ok(StreamF64 {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .parameters(),
            )),
        })
    }
    #[napi(js_name = "into_parameters")]
    pub fn binding_into_parameters(&self) -> napi::Result<StreamF64> {
        Ok(StreamF64 {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow_mut()
                    .map_err(borrowed)?
                    .take()
                    .ok_or_else(consumed)?)
                .into_parameters(),
            )),
        })
    }
    #[napi(js_name = "emission")]
    pub fn binding_emission(&self, state: f64) -> napi::Result<Option<PreparedGaussianF64>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .emission(checked_usize(state)?))
        .map(|value| PreparedGaussianF64 {
            inner: std::cell::RefCell::new(Some(value)),
        }))
    }
}
