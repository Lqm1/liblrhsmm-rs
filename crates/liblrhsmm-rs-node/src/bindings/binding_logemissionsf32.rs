use super::*;
#[napi]
pub struct LogEmissionsF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::LogEmissionsF32>>,
}
#[napi]
impl LogEmissionsF32 {
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
impl LogEmissionsF32 {
    #[napi(constructor)]
    pub fn binding_new(
        frames: f64,
        states: f64,
        values: napi::bindgen_prelude::Float32Array,
    ) -> napi::Result<Self> {
        let storage_values = values.to_vec();
        (crate::api::LogEmissionsF32::new(
            checked_usize(frames)?,
            checked_usize(states)?,
            &storage_values,
        ))
        .map(|value| LogEmissionsF32 {
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
    #[napi(js_name = "get")]
    pub fn binding_get(&self, frame: f64, state: f64) -> napi::Result<Option<f64>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .get(checked_usize(frame)?, checked_usize(state)?))
        .map(|value| value as f64))
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(LogEmissionsF32 {
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
    #[napi(js_name = "probabilities")]
    pub fn binding_probabilities(&self) -> napi::Result<EmissionDensitiesF32> {
        Ok(EmissionDensitiesF32 {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .probabilities(),
            )),
        })
    }
}
