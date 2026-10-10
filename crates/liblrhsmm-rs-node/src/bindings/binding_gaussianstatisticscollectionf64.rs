use super::*;
#[napi]
pub struct GaussianStatisticsCollectionF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::GaussianStatisticsCollectionF64>>,
}
#[napi]
impl GaussianStatisticsCollectionF64 {
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
impl GaussianStatisticsCollectionF64 {
    #[napi(constructor)]
    pub fn binding_new() -> napi::Result<Self> {
        Ok(GaussianStatisticsCollectionF64 {
            inner: std::cell::RefCell::new(
                Some(crate::api::GaussianStatisticsCollectionF64::new()),
            ),
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
    #[napi(js_name = "push")]
    pub fn binding_push(&self, value: &GaussianStatisticsF64) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .push(
            value
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        );
        Ok(())
    }
    #[napi(js_name = "get")]
    pub fn binding_get(&self, index: f64) -> napi::Result<Option<GaussianStatisticsF64>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .get(checked_usize(index)?))
        .map(|value| GaussianStatisticsF64 {
            inner: std::cell::RefCell::new(Some(value)),
        }))
    }
    #[napi(js_name = "replace")]
    pub fn binding_replace(&self, index: f64, value: &GaussianStatisticsF64) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .replace(
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
    pub fn binding_clear(&self) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .clear();
        Ok(())
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(GaussianStatisticsCollectionF64 {
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
