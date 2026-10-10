use super::*;
#[napi]
pub struct EmissionPruningF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::EmissionPruningF64>>,
}
#[napi]
impl EmissionPruningF64 {
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
impl EmissionPruningF64 {
    #[napi(getter, js_name = "mode")]
    pub fn binding_get_mode(&self) -> napi::Result<u32> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .mode)
    }
    #[napi(setter, js_name = "mode")]
    pub fn binding_set_mode(&self, value: u32) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .mode = value;
        Ok(())
    }
    #[napi(getter, js_name = "parameter")]
    pub fn binding_get_parameter(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .parameter)
    }
    #[napi(setter, js_name = "parameter")]
    pub fn binding_set_parameter(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .parameter = value;
        Ok(())
    }
}
#[napi]
impl EmissionPruningF64 {
    #[napi(constructor)]
    pub fn binding_new(mode: u32, parameter: f64) -> napi::Result<Self> {
        Ok(EmissionPruningF64 {
            inner: std::cell::RefCell::new(Some(crate::api::EmissionPruningF64::new(
                mode, parameter,
            ))),
        })
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(EmissionPruningF64 {
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
