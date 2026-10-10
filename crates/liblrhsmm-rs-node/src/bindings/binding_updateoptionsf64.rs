use super::*;
#[napi]
pub struct UpdateOptionsF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::UpdateOptionsF64>>,
}
#[napi]
impl UpdateOptionsF64 {
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
impl UpdateOptionsF64 {
    #[napi(getter, js_name = "duration_mode")]
    pub fn binding_get_duration_mode(&self) -> napi::Result<u32> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .duration_mode)
    }
    #[napi(setter, js_name = "duration_mode")]
    pub fn binding_set_duration_mode(&self, value: u32) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .duration_mode = value;
        Ok(())
    }
    #[napi(getter, js_name = "duration_variance_floor")]
    pub fn binding_get_duration_variance_floor(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .duration_variance_floor)
    }
    #[napi(setter, js_name = "duration_variance_floor")]
    pub fn binding_set_duration_variance_floor(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .duration_variance_floor = value;
        Ok(())
    }
    #[napi(getter, js_name = "emission_variance_floor")]
    pub fn binding_get_emission_variance_floor(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .emission_variance_floor)
    }
    #[napi(setter, js_name = "emission_variance_floor")]
    pub fn binding_set_emission_variance_floor(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .emission_variance_floor = value;
        Ok(())
    }
}
#[napi]
impl UpdateOptionsF64 {
    #[napi(constructor)]
    pub fn binding_new() -> napi::Result<Self> {
        Ok(UpdateOptionsF64 {
            inner: std::cell::RefCell::new(Some(crate::api::UpdateOptionsF64::new())),
        })
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(UpdateOptionsF64 {
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
