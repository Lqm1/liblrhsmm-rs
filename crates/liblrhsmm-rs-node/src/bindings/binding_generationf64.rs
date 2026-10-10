use super::*;
#[napi]
pub struct GenerationF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::GenerationF64>>,
}
#[napi]
impl GenerationF64 {
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
impl GenerationF64 {
    #[napi(constructor)]
    pub fn binding_new(model: &ModelF64, sequence: &SegmentationF64) -> napi::Result<Self> {
        Ok(GenerationF64 {
            inner: std::cell::RefCell::new(Some(crate::api::GenerationF64::new(
                model
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?,
                sequence
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?,
            ))),
        })
    }
    #[napi(js_name = "model")]
    pub fn binding_model(&self) -> napi::Result<ModelF64> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .model())
        .map(|value| ModelF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "sequence")]
    pub fn binding_sequence(&self) -> napi::Result<SegmentationF64> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .sequence())
        .map(|value| SegmentationF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "set_model")]
    pub fn binding_set_model(&self, model: &ModelF64) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .set_model(
            model
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        )
    }
    #[napi(js_name = "set_sequence")]
    pub fn binding_set_sequence(&self, sequence: &SegmentationF64) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .set_sequence(
            sequence
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        )
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .cloned())
        .map(|value| GenerationF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "generate_with_uniform")]
    pub fn binding_generate_with_uniform(
        &self,
        #[napi(ts_arg_type = "() => number")] callback: napi::bindgen_prelude::Unknown<'_>,
        env: napi::Env,
    ) -> napi::Result<GeneratedObservationF64> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .generate_with_uniform(&crate::callbacks::Function::new(env, callback)?))
        .map(|value| GeneratedObservationF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "generate_with_random_words")]
    pub fn binding_generate_with_random_words(
        &self,
        #[napi(ts_arg_type = "() => number")] callback: napi::bindgen_prelude::Unknown<'_>,
        env: napi::Env,
    ) -> napi::Result<GeneratedObservationF64> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .generate_with_random_words(&crate::callbacks::Function::new(env, callback)?))
        .map(|value| GeneratedObservationF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
}
