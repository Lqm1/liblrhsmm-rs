use super::*;
#[napi]
pub struct DurationPosterior {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::DurationPosterior>>,
}
#[napi]
impl DurationPosterior {
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
impl DurationPosterior {
    #[napi(constructor)]
    pub fn binding_new(
        minimum: f64,
        probabilities: napi::bindgen_prelude::Float32Array,
    ) -> napi::Result<Self> {
        let storage_probabilities = probabilities.to_vec();
        Ok(DurationPosterior {
            inner: std::cell::RefCell::new(Some(crate::api::DurationPosterior::new(
                checked_usize(minimum)?,
                &storage_probabilities,
            ))),
        })
    }
    #[napi(getter, js_name = "minimum")]
    pub fn binding_minimum(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .minimum() as f64)
    }
    #[napi(setter, js_name = "minimum")]
    pub fn binding_set_minimum(&self, value: f64) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_minimum(checked_usize(value)?);
        Ok(())
    }
    #[napi(js_name = "probabilities")]
    pub fn binding_probabilities(&self) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        Ok(<napi::bindgen_prelude::Float32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .probabilities(),
        ))
    }
    #[napi(js_name = "set_probabilities")]
    pub fn binding_set_probabilities(
        &self,
        values: napi::bindgen_prelude::Float32Array,
    ) -> napi::Result<()> {
        let storage_values = values.to_vec();
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_probabilities(&storage_values);
        Ok(())
    }
    #[napi(js_name = "probability")]
    pub fn binding_probability(&self, duration: f64) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .probability(checked_usize(duration)?) as f64)
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(DurationPosterior {
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
