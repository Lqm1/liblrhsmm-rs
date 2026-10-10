use super::*;
#[napi]
pub struct MixturePosteriors {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::MixturePosteriors>>,
}
#[napi]
impl MixturePosteriors {
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
impl MixturePosteriors {
    #[napi(constructor)]
    pub fn binding_new(
        frames: f64,
        states: f64,
        presence: napi::bindgen_prelude::Uint8Array,
        offsets: napi::bindgen_prelude::Uint32Array,
        values: napi::bindgen_prelude::Float32Array,
    ) -> napi::Result<Self> {
        let storage_presence = presence.to_vec();
        let storage_offsets = offsets.to_vec();
        let storage_values = values.to_vec();
        (crate::api::MixturePosteriors::new(
            checked_usize(frames)?,
            checked_usize(states)?,
            &storage_presence,
            &storage_offsets,
            &storage_values,
        ))
        .map(|value| MixturePosteriors {
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
    #[napi(js_name = "presence")]
    pub fn binding_presence(&self) -> napi::Result<napi::bindgen_prelude::Uint8Array> {
        Ok(<napi::bindgen_prelude::Uint8Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .presence(),
        ))
    }
    #[napi(js_name = "offsets")]
    pub fn binding_offsets(&self) -> napi::Result<napi::bindgen_prelude::Uint32Array> {
        Ok(<napi::bindgen_prelude::Uint32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .offsets(),
        ))
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
    pub fn binding_get(&self, frame: f64, state: f64) -> napi::Result<Option<MixtureWeights>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .get(checked_usize(frame)?, checked_usize(state)?))
        .map(|value| MixtureWeights {
            inner: std::cell::RefCell::new(Some(value)),
        }))
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(MixturePosteriors {
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
