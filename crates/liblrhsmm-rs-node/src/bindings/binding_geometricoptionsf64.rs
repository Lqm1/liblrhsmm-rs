use super::*;
#[napi]
pub struct GeometricOptionsF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::GeometricOptionsF64>>,
}
#[napi]
impl GeometricOptionsF64 {
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
impl GeometricOptionsF64 {
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
    #[napi(getter, js_name = "pruning_slope")]
    pub fn binding_get_pruning_slope(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .pruning_slope)
    }
    #[napi(setter, js_name = "pruning_slope")]
    pub fn binding_set_pruning_slope(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .pruning_slope = value;
        Ok(())
    }
}
#[napi]
impl GeometricOptionsF64 {
    #[napi(constructor)]
    pub fn binding_new() -> napi::Result<Self> {
        Ok(GeometricOptionsF64 {
            inner: std::cell::RefCell::new(Some(crate::api::GeometricOptionsF64::new())),
        })
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(GeometricOptionsF64 {
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
