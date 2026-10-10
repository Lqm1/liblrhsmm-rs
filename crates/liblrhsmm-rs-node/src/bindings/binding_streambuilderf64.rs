use super::*;
#[napi]
pub struct StreamBuilderF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::StreamBuilderF64>>,
}
#[napi]
impl StreamBuilderF64 {
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
impl StreamBuilderF64 {
    #[napi(constructor)]
    pub fn binding_new(emissions: f64) -> napi::Result<Self> {
        (crate::api::StreamBuilderF64::new(checked_usize(emissions)?)).map(|value| {
            StreamBuilderF64 {
                inner: std::cell::RefCell::new(Some(value)),
            }
        })
    }
    #[napi(getter, js_name = "emissions")]
    pub fn binding_emissions(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .emissions() as f64)
    }
    #[napi(js_name = "weight")]
    pub fn binding_weight(&self) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        Ok(<napi::bindgen_prelude::Float64Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .weight(),
        ))
    }
    #[napi(js_name = "set_weight")]
    pub fn binding_set_weight(
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
        .set_weight(&storage_values)
    }
    #[napi(js_name = "emission")]
    pub fn binding_emission(&self, index: f64) -> napi::Result<Option<GaussianF64>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .emission(checked_usize(index)?))
        .map(|value| GaussianF64 {
            inner: std::cell::RefCell::new(Some(value)),
        }))
    }
    #[napi(js_name = "has_emission")]
    pub fn binding_has_emission(&self, index: f64) -> napi::Result<bool> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .has_emission(checked_usize(index)?))
    }
    #[napi(js_name = "resize")]
    pub fn binding_resize(&self, emissions: f64) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .resize(checked_usize(emissions)?)
    }
    #[napi(js_name = "set_emission")]
    pub fn binding_set_emission(&self, index: f64, value: &GaussianF64) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_emission(
            checked_usize(index)?,
            value
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        )
    }
    #[napi(js_name = "clear_emission")]
    pub fn binding_clear_emission(&self, index: f64) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .clear_emission(checked_usize(index)?)
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(StreamBuilderF64 {
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
    #[napi(js_name = "build")]
    pub fn binding_build(&self) -> napi::Result<StreamF64> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .take()
            .ok_or_else(consumed)?)
        .build())
        .map(|value| StreamF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
}
