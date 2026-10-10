use super::*;
#[napi]
pub struct HsmmScoresF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::HsmmScoresF64>>,
}
#[napi]
impl HsmmScoresF64 {
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
impl HsmmScoresF64 {
    #[napi(constructor)]
    pub fn binding_new(begin: &StateScoresF64, end: &StateScoresF64) -> napi::Result<Self> {
        Ok(HsmmScoresF64 {
            inner: std::cell::RefCell::new(Some(crate::api::HsmmScoresF64::new(
                begin
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?,
                end.inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?,
            ))),
        })
    }
    #[napi(js_name = "begin")]
    pub fn binding_begin(&self) -> napi::Result<StateScoresF64> {
        Ok(StateScoresF64 {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .begin(),
            )),
        })
    }
    #[napi(js_name = "end")]
    pub fn binding_end(&self) -> napi::Result<StateScoresF64> {
        Ok(StateScoresF64 {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .end(),
            )),
        })
    }
    #[napi(js_name = "set_begin")]
    pub fn binding_set_begin(&self, value: &StateScoresF64) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_begin(
            value
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        );
        Ok(())
    }
    #[napi(js_name = "set_end")]
    pub fn binding_set_end(&self, value: &StateScoresF64) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_end(
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
        Ok(HsmmScoresF64 {
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
