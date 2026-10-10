use super::*;
#[napi]
pub struct PreparationOptionsF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::PreparationOptionsF64>>,
}
#[napi]
impl PreparationOptionsF64 {
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
impl PreparationOptionsF64 {
    #[napi(getter, js_name = "duration_cache_length")]
    pub fn binding_get_duration_cache_length(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .duration_cache_length as f64)
    }
    #[napi(setter, js_name = "duration_cache_length")]
    pub fn binding_set_duration_cache_length(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .duration_cache_length = checked_usize(value)?;
        Ok(())
    }
    #[napi(getter, js_name = "duration_weight")]
    pub fn binding_get_duration_weight(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .duration_weight)
    }
    #[napi(setter, js_name = "duration_weight")]
    pub fn binding_set_duration_weight(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .duration_weight = value;
        Ok(())
    }
    #[napi(getter, js_name = "temperature")]
    pub fn binding_get_temperature(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .temperature)
    }
    #[napi(setter, js_name = "temperature")]
    pub fn binding_set_temperature(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .temperature = value;
        Ok(())
    }
}
#[napi]
impl PreparationOptionsF64 {
    #[napi(constructor)]
    pub fn binding_new() -> napi::Result<Self> {
        Ok(PreparationOptionsF64 {
            inner: std::cell::RefCell::new(Some(crate::api::PreparationOptionsF64::new())),
        })
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(PreparationOptionsF64 {
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
