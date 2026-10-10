use super::*;
#[napi]
pub struct PreparedGaussianF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::PreparedGaussianF32>>,
}
#[napi]
impl PreparedGaussianF32 {
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
impl PreparedGaussianF32 {
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(PreparedGaussianF32 {
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
    pub fn binding_parameters(&self) -> napi::Result<GaussianF32> {
        Ok(GaussianF32 {
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
    pub fn binding_into_parameters(&self) -> napi::Result<GaussianF32> {
        Ok(GaussianF32 {
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
    #[napi(js_name = "cached_log_normalizers")]
    pub fn binding_cached_log_normalizers(
        &self,
    ) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        Ok(<napi::bindgen_prelude::Float32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .cached_log_normalizers(),
        ))
    }
    #[napi(js_name = "component_log_probability")]
    pub fn binding_component_log_probability(
        &self,
        observation: napi::bindgen_prelude::Float32Array,
        component: f64,
    ) -> napi::Result<f64> {
        let storage_observation = observation.to_vec();
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .component_log_probability(&storage_observation, checked_usize(component)?))
        .map(|value| value as f64)
    }
    #[napi(js_name = "component_probability")]
    pub fn binding_component_probability(
        &self,
        observation: napi::bindgen_prelude::Float32Array,
        component: f64,
    ) -> napi::Result<f64> {
        let storage_observation = observation.to_vec();
        (self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .component_probability(&storage_observation, checked_usize(component)?)
    }
    #[napi(js_name = "log_probability")]
    pub fn binding_log_probability(
        &self,
        observation: napi::bindgen_prelude::Float32Array,
        temperature: f64,
    ) -> napi::Result<f64> {
        let storage_observation = observation.to_vec();
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .log_probability(&storage_observation, temperature as f32))
        .map(|value| value as f64)
    }
    #[napi(js_name = "probability")]
    pub fn binding_probability(
        &self,
        observation: napi::bindgen_prelude::Float32Array,
        temperature: f64,
    ) -> napi::Result<f64> {
        let storage_observation = observation.to_vec();
        (self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .probability(&storage_observation, temperature as f32)
    }
}
