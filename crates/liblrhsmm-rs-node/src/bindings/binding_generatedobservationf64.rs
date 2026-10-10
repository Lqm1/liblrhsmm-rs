use super::*;
#[napi]
pub struct GeneratedObservationF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::GeneratedObservationF64>>,
}
#[napi]
impl GeneratedObservationF64 {
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
impl GeneratedObservationF64 {
    #[napi(getter, js_name = "draws_used")]
    pub fn binding_draws_used(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .draws_used() as f64)
    }
    #[napi(getter, js_name = "random_words_used")]
    pub fn binding_random_words_used(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .random_words_used() as f64)
    }
    #[napi(js_name = "observation")]
    pub fn binding_observation(&self) -> napi::Result<ObservationF64> {
        Ok(ObservationF64 {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .observation(),
            )),
        })
    }
    #[napi(js_name = "into_observation")]
    pub fn binding_into_observation(&self) -> napi::Result<ObservationF64> {
        Ok(ObservationF64 {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow_mut()
                    .map_err(borrowed)?
                    .take()
                    .ok_or_else(consumed)?)
                .into_observation(),
            )),
        })
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(GeneratedObservationF64 {
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
