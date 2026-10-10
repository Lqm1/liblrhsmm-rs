use super::*;
#[napi]
pub struct StreamStatisticsF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::StreamStatisticsF64>>,
}
#[napi]
impl StreamStatisticsF64 {
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
impl StreamStatisticsF64 {
    #[napi(constructor)]
    pub fn binding_new() -> napi::Result<Self> {
        Ok(StreamStatisticsF64 {
            inner: std::cell::RefCell::new(Some(crate::api::StreamStatisticsF64::new())),
        })
    }
    #[napi(js_name = "from_stream")]
    pub fn binding_from_stream(value: &StreamF64) -> napi::Result<Self> {
        (crate::api::StreamStatisticsF64::from_stream(
            value
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| StreamStatisticsF64 {
            inner: std::cell::RefCell::new(Some(value)),
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
    #[napi(js_name = "emission")]
    pub fn binding_emission(&self, index: f64) -> napi::Result<Option<GaussianStatisticsF64>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .emission(checked_usize(index)?))
        .map(|value| GaussianStatisticsF64 {
            inner: std::cell::RefCell::new(Some(value)),
        }))
    }
    #[napi(js_name = "set_emission")]
    pub fn binding_set_emission(
        &self,
        index: f64,
        value: &GaussianStatisticsF64,
    ) -> napi::Result<()> {
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
    #[napi(js_name = "replace")]
    pub fn binding_replace(&self, values: &GaussianStatisticsCollectionF64) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .replace(
            values
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
        Ok(StreamStatisticsF64 {
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
}
