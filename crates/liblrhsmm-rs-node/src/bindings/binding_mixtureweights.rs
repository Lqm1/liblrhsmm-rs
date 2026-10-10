use super::*;
#[napi]
pub struct MixtureWeights {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::MixtureWeights>>,
}
#[napi]
impl MixtureWeights {
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
impl MixtureWeights {
    #[napi(constructor)]
    pub fn binding_new(values: napi::bindgen_prelude::Float32Array) -> napi::Result<Self> {
        let storage_values = values.to_vec();
        Ok(MixtureWeights {
            inner: std::cell::RefCell::new(Some(crate::api::MixtureWeights::new(&storage_values))),
        })
    }
    #[napi(js_name = "values")]
    pub fn binding_values(&self) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        Ok(<napi::bindgen_prelude::Float32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .values(),
        ))
    }
    #[napi(js_name = "replace")]
    pub fn binding_replace(&self, values: napi::bindgen_prelude::Float32Array) -> napi::Result<()> {
        let storage_values = values.to_vec();
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .replace(&storage_values);
        Ok(())
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(MixtureWeights {
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
