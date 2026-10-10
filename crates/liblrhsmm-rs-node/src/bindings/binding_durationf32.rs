use super::*;
#[napi]
pub struct DurationF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::DurationF32>>,
}
#[napi]
impl DurationF32 {
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
impl DurationF32 {
    #[napi(constructor)]
    pub fn binding_new() -> napi::Result<Self> {
        Ok(DurationF32 {
            inner: std::cell::RefCell::new(Some(crate::api::DurationF32::new())),
        })
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(DurationF32 {
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
    #[napi(js_name = "constraints")]
    pub fn binding_constraints(&self) -> napi::Result<napi::bindgen_prelude::Int32Array> {
        Ok(<napi::bindgen_prelude::Int32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .constraints(),
        ))
    }
    #[napi(js_name = "set_values")]
    pub fn binding_set_values(
        &self,
        values: napi::bindgen_prelude::Float32Array,
    ) -> napi::Result<()> {
        let storage_values = values.to_vec();
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_values(&storage_values)
    }
    #[napi(js_name = "set_constraints")]
    pub fn binding_set_constraints(
        &self,
        values: napi::bindgen_prelude::Int32Array,
    ) -> napi::Result<()> {
        let storage_values = values.to_vec();
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_constraints(&storage_values)
    }
    #[napi(js_name = "log_probability")]
    pub fn binding_log_probability(&self, frames: f64, weight: f64) -> napi::Result<f64> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .log_probability(frames as f32, weight as f32))
        .map(|value| value as f64)
    }
    #[napi(js_name = "annealed_log_probability")]
    pub fn binding_annealed_log_probability(
        &self,
        frames: f64,
        weight: f64,
        temperature: f64,
    ) -> napi::Result<f64> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .annealed_log_probability(checked_usize(frames)?, weight as f32, temperature as f32))
        .map(|value| value as f64)
    }
    #[napi(js_name = "probability")]
    pub fn binding_probability(
        &self,
        frames: f64,
        weight: f64,
        temperature: f64,
    ) -> napi::Result<f64> {
        (self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .probability(checked_usize(frames)?, weight as f32, temperature as f32)
    }
    #[napi(js_name = "prepare")]
    pub fn binding_prepare(
        &self,
        options: &PreparationOptionsF32,
    ) -> napi::Result<PreparedDurationF32> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .prepare(
            options
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| PreparedDurationF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "prepare_owned")]
    pub fn binding_prepare_owned(
        &self,
        options: &PreparationOptionsF32,
    ) -> napi::Result<PreparedDurationF32> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .take()
            .ok_or_else(consumed)?)
        .prepare_owned(
            options
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| PreparedDurationF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "update")]
    pub fn binding_update(
        &self,
        statistics: &DurationStatisticsF32,
        options: &UpdateOptionsF32,
    ) -> napi::Result<bool> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .update(
            statistics
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            options
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        )
    }
}
#[cfg(feature = "serialization")]
#[napi]
impl DurationF32 {
    #[napi(js_name = "read")]
    pub fn binding_read(bytes: napi::bindgen_prelude::Uint8Array) -> napi::Result<Self> {
        let storage_bytes = bytes.to_vec();
        (crate::api::DurationF32::read(&storage_bytes)).map(|value| DurationF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "read_with_limits")]
    pub fn binding_read_with_limits(
        bytes: napi::bindgen_prelude::Uint8Array,
        max_array_entries: f64,
    ) -> napi::Result<Self> {
        let storage_bytes = bytes.to_vec();
        (crate::api::DurationF32::read_with_limits(
            &storage_bytes,
            checked_usize(max_array_entries)?,
        ))
        .map(|value| DurationF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "read_prefix")]
    pub fn binding_read_prefix(
        bytes: napi::bindgen_prelude::Uint8Array,
    ) -> napi::Result<DecodedDurationF32> {
        let storage_bytes = bytes.to_vec();
        (crate::api::DurationF32::read_prefix(&storage_bytes)).map(|value| DecodedDurationF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "read_prefix_with_limits")]
    pub fn binding_read_prefix_with_limits(
        bytes: napi::bindgen_prelude::Uint8Array,
        max_array_entries: f64,
    ) -> napi::Result<DecodedDurationF32> {
        let storage_bytes = bytes.to_vec();
        (crate::api::DurationF32::read_prefix_with_limits(
            &storage_bytes,
            checked_usize(max_array_entries)?,
        ))
        .map(|value| DecodedDurationF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "write")]
    pub fn binding_write(&self) -> napi::Result<napi::bindgen_prelude::Uint8Array> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .write())
        .map(<napi::bindgen_prelude::Uint8Array>::from)
    }
}
