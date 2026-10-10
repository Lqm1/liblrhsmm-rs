use super::*;
#[napi]
pub struct GeometricExpectationF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::GeometricExpectationF32>>,
}
#[napi]
impl GeometricExpectationF32 {
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
impl GeometricExpectationF32 {
    #[napi(getter, js_name = "pruning_slope")]
    pub fn binding_get_pruning_slope(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .pruning_slope as f64)
    }
    #[napi(setter, js_name = "pruning_slope")]
    pub fn binding_set_pruning_slope(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .pruning_slope = value as f32;
        Ok(())
    }
}
#[napi]
impl GeometricExpectationF32 {
    #[napi(constructor)]
    pub fn binding_new(
        transitions: &TransitionPosteriors,
        states: &StateOccupanciesF32,
        mixtures: &MixturePosteriorCollection,
        pruning_slope: f64,
    ) -> napi::Result<Self> {
        Ok(GeometricExpectationF32 {
            inner: std::cell::RefCell::new(Some(crate::api::GeometricExpectationF32::new(
                transitions
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?,
                states
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?,
                mixtures
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?,
                pruning_slope as f32,
            ))),
        })
    }
    #[napi(js_name = "transitions")]
    pub fn binding_transitions(&self) -> napi::Result<TransitionPosteriors> {
        Ok(TransitionPosteriors {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .transitions(),
            )),
        })
    }
    #[napi(js_name = "set_transitions")]
    pub fn binding_set_transitions(&self, value: &TransitionPosteriors) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_transitions(
            value
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        );
        Ok(())
    }
    #[napi(js_name = "states")]
    pub fn binding_states(&self) -> napi::Result<StateOccupanciesF32> {
        Ok(StateOccupanciesF32 {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .states(),
            )),
        })
    }
    #[napi(js_name = "set_states")]
    pub fn binding_set_states(&self, value: &StateOccupanciesF32) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_states(
            value
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        );
        Ok(())
    }
    #[napi(js_name = "mixtures")]
    pub fn binding_mixtures(&self) -> napi::Result<MixturePosteriorCollection> {
        Ok(MixturePosteriorCollection {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .mixtures(),
            )),
        })
    }
    #[napi(js_name = "set_mixtures")]
    pub fn binding_set_mixtures(&self, value: &MixturePosteriorCollection) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_mixtures(
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
        Ok(GeometricExpectationF32 {
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
