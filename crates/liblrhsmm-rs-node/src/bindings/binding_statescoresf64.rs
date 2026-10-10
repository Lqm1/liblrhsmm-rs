use super::*;
#[napi]
pub struct StateScoresF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::StateScoresF64>>,
}
#[napi]
impl StateScoresF64 {
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
impl StateScoresF64 {
    #[napi(constructor)]
    pub fn binding_new(
        frames: f64,
        states: f64,
        values: napi::bindgen_prelude::Float64Array,
    ) -> napi::Result<Self> {
        let storage_values = values.to_vec();
        (crate::api::StateScoresF64::new(
            checked_usize(frames)?,
            checked_usize(states)?,
            &storage_values,
        ))
        .map(|value| StateScoresF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(getter, js_name = "frames")]
    pub fn binding_frames(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .frames() as f64)
    }
    #[napi(getter, js_name = "states")]
    pub fn binding_states(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .states() as f64)
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
    #[napi(js_name = "get")]
    pub fn binding_get(&self, frame: f64, state: f64) -> napi::Result<Option<f64>> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .get(checked_usize(frame)?, checked_usize(state)?))
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(StateScoresF64 {
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
    #[napi(js_name = "total_log_probability")]
    pub fn binding_total_log_probability(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .total_log_probability())
    }
    #[napi(js_name = "total_through_state")]
    pub fn binding_total_through_state(
        &self,
        backward: &StateScoresF64,
        state: f64,
    ) -> napi::Result<f64> {
        (self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .total_through_state(
            backward
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            checked_usize(state)?,
        )
    }
    #[napi(js_name = "backward_log_probability")]
    pub fn binding_backward_log_probability(
        &self,
        emissions: &LogEmissionsF64,
    ) -> napi::Result<f64> {
        (self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .backward_log_probability(
            emissions
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        )
    }
}
#[napi]
impl StateScoresF64 {
    #[napi(js_name = "geometric_occupancies")]
    pub fn binding_geometric_occupancies(
        &self,
        backward: &StateScoresF64,
        total: f64,
        pruning_slope: f64,
    ) -> napi::Result<StateOccupanciesF64> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .geometric_occupancies(
            backward
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            total,
            pruning_slope,
        ))
        .map(|value| StateOccupanciesF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
}
