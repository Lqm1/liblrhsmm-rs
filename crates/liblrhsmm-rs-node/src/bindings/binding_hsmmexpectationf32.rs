use super::*;
#[napi]
pub struct HsmmExpectationF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::HsmmExpectationF32>>,
}
#[napi]
impl HsmmExpectationF32 {
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
impl HsmmExpectationF32 {
    #[napi(getter, js_name = "state_radius")]
    pub fn binding_get_state_radius(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .state_radius as f64)
    }
    #[napi(setter, js_name = "state_radius")]
    pub fn binding_set_state_radius(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .state_radius = value as f32;
        Ok(())
    }
}
#[napi]
impl HsmmExpectationF32 {
    #[napi(constructor)]
    pub fn binding_new(
        durations: &DurationPosteriors,
        states: &StateOccupanciesF32,
        mixtures: &MixturePosteriorCollection,
        state_radius: f64,
    ) -> napi::Result<Self> {
        Ok(HsmmExpectationF32 {
            inner: std::cell::RefCell::new(Some(crate::api::HsmmExpectationF32::new(
                durations
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
                state_radius as f32,
            ))),
        })
    }
    #[napi(js_name = "durations")]
    pub fn binding_durations(&self) -> napi::Result<DurationPosteriors> {
        Ok(DurationPosteriors {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .durations(),
            )),
        })
    }
    #[napi(js_name = "set_durations")]
    pub fn binding_set_durations(&self, value: &DurationPosteriors) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_durations(
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
        Ok(HsmmExpectationF32 {
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
