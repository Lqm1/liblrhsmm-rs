use super::*;
#[napi]
pub struct PreparedDurationF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::PreparedDurationF32>>,
}
#[napi]
impl PreparedDurationF32 {
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
impl PreparedDurationF32 {
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(PreparedDurationF32 {
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
    pub fn binding_parameters(&self) -> napi::Result<DurationF32> {
        Ok(DurationF32 {
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
    pub fn binding_into_parameters(&self) -> napi::Result<DurationF32> {
        Ok(DurationF32 {
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
    #[napi(js_name = "options")]
    pub fn binding_options(&self) -> napi::Result<PreparationOptionsF32> {
        Ok(PreparationOptionsF32 {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .options(),
            )),
        })
    }
    #[napi(js_name = "log_normalizer")]
    pub fn binding_log_normalizer(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .log_normalizer() as f64)
    }
    #[napi(js_name = "cached_log_probabilities")]
    pub fn binding_cached_log_probabilities(
        &self,
    ) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        Ok(<napi::bindgen_prelude::Float32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .cached_log_probabilities(),
        ))
    }
    #[napi(js_name = "log_probability")]
    pub fn binding_log_probability(&self, frames: f64) -> napi::Result<f64> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .log_probability(checked_usize(frames)?))
        .map(|value| value as f64)
    }
    #[napi(js_name = "probability")]
    pub fn binding_probability(&self, frames: f64) -> napi::Result<f64> {
        (self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .probability(checked_usize(frames)?)
    }
}
