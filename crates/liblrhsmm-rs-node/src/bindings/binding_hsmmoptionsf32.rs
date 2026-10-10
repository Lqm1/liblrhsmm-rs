use super::*;
#[napi]
pub struct HsmmOptionsF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::HsmmOptionsF32>>,
}
#[napi]
impl HsmmOptionsF32 {
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
impl HsmmOptionsF32 {
    #[napi(getter, js_name = "temperature")]
    pub fn binding_get_temperature(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .temperature as f64)
    }
    #[napi(setter, js_name = "temperature")]
    pub fn binding_set_temperature(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .temperature = value as f32;
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
            .duration_weight as f64)
    }
    #[napi(setter, js_name = "duration_weight")]
    pub fn binding_set_duration_weight(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .duration_weight = value as f32;
        Ok(())
    }
    #[napi(getter, js_name = "state_radius")]
    pub fn binding_get_state_radius(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .state_radius as f64)
    }
    #[napi(setter, js_name = "state_radius")]
    pub fn binding_set_state_radius(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .state_radius = value as f32;
        Ok(())
    }
    #[napi(getter, js_name = "duration_extra")]
    pub fn binding_get_duration_extra(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .duration_extra as f64)
    }
    #[napi(setter, js_name = "duration_extra")]
    pub fn binding_set_duration_extra(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .duration_extra = checked_usize(value)?;
        Ok(())
    }
    #[napi(getter, js_name = "duration_extra_factor")]
    pub fn binding_get_duration_extra_factor(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .duration_extra_factor as f64)
    }
    #[napi(setter, js_name = "duration_extra_factor")]
    pub fn binding_set_duration_extra_factor(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .duration_extra_factor = value as f32;
        Ok(())
    }
}
#[napi]
impl HsmmOptionsF32 {
    #[napi(constructor)]
    pub fn binding_new() -> napi::Result<Self> {
        Ok(HsmmOptionsF32 {
            inner: std::cell::RefCell::new(Some(crate::api::HsmmOptionsF32::new())),
        })
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(HsmmOptionsF32 {
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
