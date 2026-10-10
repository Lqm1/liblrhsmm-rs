use super::*;
#[napi]
pub struct HsmmPathF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::HsmmPathF64>>,
}
#[napi]
impl HsmmPathF64 {
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
impl HsmmPathF64 {
    #[napi(constructor)]
    pub fn binding_new(
        occurrences: napi::bindgen_prelude::Uint32Array,
        log_probability: napi::bindgen_prelude::Float64Array,
        scores: &HsmmScoresF64,
    ) -> napi::Result<Self> {
        let storage_occurrences = occurrences.to_vec();
        let storage_log_probability = log_probability.to_vec();
        (crate::api::HsmmPathF64::new(
            &storage_occurrences,
            &storage_log_probability,
            scores
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| HsmmPathF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "occurrences")]
    pub fn binding_occurrences(&self) -> napi::Result<napi::bindgen_prelude::Uint32Array> {
        Ok(<napi::bindgen_prelude::Uint32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .occurrences(),
        ))
    }
    #[napi(js_name = "set_occurrences")]
    pub fn binding_set_occurrences(
        &self,
        values: napi::bindgen_prelude::Uint32Array,
    ) -> napi::Result<()> {
        let storage_values = values.to_vec();
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_occurrences(&storage_values)
    }
    #[napi(js_name = "log_probability")]
    pub fn binding_log_probability(&self) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        Ok(<napi::bindgen_prelude::Float64Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .log_probability(),
        ))
    }
    #[napi(js_name = "set_log_probability")]
    pub fn binding_set_log_probability(
        &self,
        values: napi::bindgen_prelude::Float64Array,
    ) -> napi::Result<()> {
        let storage_values = values.to_vec();
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_log_probability(&storage_values)
    }
    #[napi(js_name = "scores")]
    pub fn binding_scores(&self) -> napi::Result<HsmmScoresF64> {
        Ok(HsmmScoresF64 {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .scores(),
            )),
        })
    }
    #[napi(js_name = "set_scores")]
    pub fn binding_set_scores(&self, value: &HsmmScoresF64) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_scores(
            value
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        );
        Ok(())
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(HsmmPathF64 {
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
