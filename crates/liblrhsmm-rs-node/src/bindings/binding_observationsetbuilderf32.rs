use super::*;
#[napi]
pub struct ObservationSetBuilderF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::ObservationSetBuilderF32>>,
}
#[napi]
impl ObservationSetBuilderF32 {
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
impl ObservationSetBuilderF32 {
    #[napi(constructor)]
    pub fn binding_new(samples: f64) -> napi::Result<Self> {
        (crate::api::ObservationSetBuilderF32::new(checked_usize(samples)?)).map(|value| {
            ObservationSetBuilderF32 {
                inner: std::cell::RefCell::new(Some(value)),
            }
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
    #[napi(js_name = "has")]
    pub fn binding_has(&self, index: f64) -> napi::Result<bool> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .has(checked_usize(index)?))
    }
    #[napi(js_name = "get")]
    pub fn binding_get(&self, index: f64) -> napi::Result<Option<ObservationF32>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .get(checked_usize(index)?))
        .map(|value| ObservationF32 {
            inner: std::cell::RefCell::new(Some(value)),
        }))
    }
    #[napi(js_name = "set")]
    pub fn binding_set(&self, index: f64, value: &ObservationF32) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set(
            checked_usize(index)?,
            value
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        )
    }
    #[napi(js_name = "clear")]
    pub fn binding_clear(&self, index: f64) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .clear(checked_usize(index)?)
    }
    #[napi(js_name = "resize")]
    pub fn binding_resize(&self, count: f64) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .resize(checked_usize(count)?)
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(ObservationSetBuilderF32 {
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
    pub fn binding_build(&self) -> napi::Result<ObservationCollectionF32> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .take()
            .ok_or_else(consumed)?)
        .build())
        .map(|value| ObservationCollectionF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
}
