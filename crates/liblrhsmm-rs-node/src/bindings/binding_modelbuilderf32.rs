use super::*;
#[napi]
pub struct ModelBuilderF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::ModelBuilderF32>>,
}
#[napi]
impl ModelBuilderF32 {
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
impl ModelBuilderF32 {
    #[napi(constructor)]
    pub fn binding_new(streams: f64, durations: f64) -> napi::Result<Self> {
        (crate::api::ModelBuilderF32::new(checked_usize(streams)?, checked_usize(durations)?)).map(
            |value| ModelBuilderF32 {
                inner: std::cell::RefCell::new(Some(value)),
            },
        )
    }
    #[napi(getter, js_name = "streams")]
    pub fn binding_streams(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .streams() as f64)
    }
    #[napi(getter, js_name = "durations")]
    pub fn binding_durations(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .durations() as f64)
    }
    #[napi(js_name = "stream")]
    pub fn binding_stream(&self, index: f64) -> napi::Result<Option<StreamF32>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .stream(checked_usize(index)?))
        .map(|value| StreamF32 {
            inner: std::cell::RefCell::new(Some(value)),
        }))
    }
    #[napi(js_name = "duration")]
    pub fn binding_duration(&self, index: f64) -> napi::Result<Option<DurationF32>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .duration(checked_usize(index)?))
        .map(|value| DurationF32 {
            inner: std::cell::RefCell::new(Some(value)),
        }))
    }
    #[napi(js_name = "has_stream")]
    pub fn binding_has_stream(&self, index: f64) -> napi::Result<bool> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .has_stream(checked_usize(index)?))
    }
    #[napi(js_name = "has_duration")]
    pub fn binding_has_duration(&self, index: f64) -> napi::Result<bool> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .has_duration(checked_usize(index)?))
    }
    #[napi(js_name = "resize")]
    pub fn binding_resize(&self, streams: f64, durations: f64) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .resize(checked_usize(streams)?, checked_usize(durations)?)
    }
    #[napi(js_name = "set_stream")]
    pub fn binding_set_stream(&self, index: f64, value: &StreamF32) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_stream(
            checked_usize(index)?,
            value
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        )
    }
    #[napi(js_name = "set_duration")]
    pub fn binding_set_duration(&self, index: f64, value: &DurationF32) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_duration(
            checked_usize(index)?,
            value
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        )
    }
    #[napi(js_name = "clear_stream")]
    pub fn binding_clear_stream(&self, index: f64) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .clear_stream(checked_usize(index)?)
    }
    #[napi(js_name = "clear_duration")]
    pub fn binding_clear_duration(&self, index: f64) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .clear_duration(checked_usize(index)?)
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(ModelBuilderF32 {
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
    pub fn binding_build(&self) -> napi::Result<ModelF32> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .take()
            .ok_or_else(consumed)?)
        .build())
        .map(|value| ModelF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
}
