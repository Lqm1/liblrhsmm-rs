use super::*;
#[napi]
pub struct NumericalTableF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::NumericalTableF64>>,
}
#[napi]
impl NumericalTableF64 {
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
impl NumericalTableF64 {
    #[napi(constructor)]
    pub fn binding_new(code: u32) -> napi::Result<Self> {
        (crate::api::NumericalTableF64::new(code)).map(|value| NumericalTableF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
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
    #[napi(js_name = "is_empty")]
    pub fn binding_is_empty(&self) -> napi::Result<bool> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .is_empty())
    }
    #[napi(js_name = "get")]
    pub fn binding_get(&self, index: f64) -> napi::Result<Option<f64>> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .get(checked_usize(index)?))
    }
    #[napi(js_name = "replace")]
    pub fn binding_replace(&self, index: f64, value: f64) -> napi::Result<Option<f64>> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .replace(checked_usize(index)?, value))
    }
    #[napi(js_name = "get_storage")]
    pub fn binding_get_storage(
        &self,
        index: f64,
    ) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        Ok(<napi::bindgen_prelude::Float64Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .get_storage(checked_usize(index)?),
        ))
    }
    #[napi(js_name = "replace_storage")]
    pub fn binding_replace_storage(
        &self,
        index: f64,
        values: napi::bindgen_prelude::Float64Array,
    ) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        let storage_values = values.to_vec();
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .replace_storage(checked_usize(index)?, &storage_values))
        .map(<napi::bindgen_prelude::Float64Array>::from)
    }
    #[napi(js_name = "iter")]
    pub fn binding_iter(&self) -> napi::Result<NumericalIteratorF64> {
        Ok(NumericalIteratorF64 {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .iter(),
            )),
        })
    }
    #[napi(js_name = "values")]
    pub fn binding_values(&self) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        Ok(<napi::bindgen_prelude::Float64Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .values(),
        ))
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(NumericalTableF64 {
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
