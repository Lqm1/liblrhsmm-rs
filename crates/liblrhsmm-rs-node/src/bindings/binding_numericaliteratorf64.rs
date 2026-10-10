use super::*;
#[napi]
pub struct NumericalIteratorF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::NumericalIteratorF64>>,
}
#[napi]
impl NumericalIteratorF64 {
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
impl NumericalIteratorF64 {
    #[napi(getter, js_name = "length")]
    pub fn binding_length(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .length() as f64)
    }
    #[napi(js_name = "next")]
    pub fn binding_next_value(&self) -> napi::Result<Option<f64>> {
        Ok((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .next_value())
    }
    #[napi(js_name = "next_back")]
    pub fn binding_next_back_value(&self) -> napi::Result<Option<f64>> {
        Ok((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .next_back_value())
    }
    #[napi(js_name = "next_storage")]
    pub fn binding_next_storage(&self) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        Ok(<napi::bindgen_prelude::Float64Array>::from(
            (self
                .inner
                .try_borrow_mut()
                .map_err(borrowed)?
                .as_mut()
                .ok_or_else(consumed)?)
            .next_storage(),
        ))
    }
    #[napi(js_name = "next_back_storage")]
    pub fn binding_next_back_storage(&self) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        Ok(<napi::bindgen_prelude::Float64Array>::from(
            (self
                .inner
                .try_borrow_mut()
                .map_err(borrowed)?
                .as_mut()
                .ok_or_else(consumed)?)
            .next_back_storage(),
        ))
    }
}
