use super::*;
#[napi]
pub struct TransitionGraphF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::TransitionGraphF32>>,
}
#[napi]
impl TransitionGraphF32 {
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
impl TransitionGraphF32 {
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(TransitionGraphF32 {
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
    #[napi(js_name = "segmentation")]
    pub fn binding_segmentation(&self) -> napi::Result<SegmentationF32> {
        Ok(SegmentationF32 {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .segmentation(),
            )),
        })
    }
    #[napi(js_name = "into_segmentation")]
    pub fn binding_into_segmentation(&self) -> napi::Result<SegmentationF32> {
        Ok(SegmentationF32 {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow_mut()
                    .map_err(borrowed)?
                    .take()
                    .ok_or_else(consumed)?)
                .into_segmentation(),
            )),
        })
    }
    #[napi(js_name = "incoming")]
    pub fn binding_incoming(&self, state: f64) -> napi::Result<Option<JumpGroupF32>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .incoming(checked_usize(state)?))
        .map(|value| JumpGroupF32 {
            inner: std::cell::RefCell::new(Some(value)),
        }))
    }
    #[napi(js_name = "outgoing")]
    pub fn binding_outgoing(&self, state: f64) -> napi::Result<Option<JumpGroupF32>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .outgoing(checked_usize(state)?))
        .map(|value| JumpGroupF32 {
            inner: std::cell::RefCell::new(Some(value)),
        }))
    }
}
