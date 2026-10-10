use super::*;
#[napi]
pub struct DurationF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::DurationF64>>,
}
#[napi]
impl DurationF64 {
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
impl DurationF64 {
    #[napi(constructor)]
    pub fn binding_new() -> napi::Result<Self> {
        Ok(DurationF64 {
            inner: std::cell::RefCell::new(Some(crate::api::DurationF64::new())),
        })
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(DurationF64 {
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
        values: napi::bindgen_prelude::Float64Array,
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
        (self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .log_probability(frames, weight)
    }
    #[napi(js_name = "annealed_log_probability")]
    pub fn binding_annealed_log_probability(
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
        .annealed_log_probability(checked_usize(frames)?, weight, temperature)
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
        .probability(checked_usize(frames)?, weight, temperature)
    }
    #[napi(js_name = "prepare")]
    pub fn binding_prepare(
        &self,
        options: &PreparationOptionsF64,
    ) -> napi::Result<PreparedDurationF64> {
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
        .map(|value| PreparedDurationF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "prepare_owned")]
    pub fn binding_prepare_owned(
        &self,
        options: &PreparationOptionsF64,
    ) -> napi::Result<PreparedDurationF64> {
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
        .map(|value| PreparedDurationF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "update")]
    pub fn binding_update(
        &self,
        statistics: &DurationStatisticsF64,
        options: &UpdateOptionsF64,
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
impl DurationF64 {
    #[napi(js_name = "read")]
    pub fn binding_read(bytes: napi::bindgen_prelude::Uint8Array) -> napi::Result<Self> {
        let storage_bytes = bytes.to_vec();
        (crate::api::DurationF64::read(&storage_bytes)).map(|value| DurationF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "read_with_limits")]
    pub fn binding_read_with_limits(
        bytes: napi::bindgen_prelude::Uint8Array,
        max_array_entries: f64,
    ) -> napi::Result<Self> {
        let storage_bytes = bytes.to_vec();
        (crate::api::DurationF64::read_with_limits(
            &storage_bytes,
            checked_usize(max_array_entries)?,
        ))
        .map(|value| DurationF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "read_prefix")]
    pub fn binding_read_prefix(
        bytes: napi::bindgen_prelude::Uint8Array,
    ) -> napi::Result<DecodedDurationF64> {
        let storage_bytes = bytes.to_vec();
        (crate::api::DurationF64::read_prefix(&storage_bytes)).map(|value| DecodedDurationF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "read_prefix_with_limits")]
    pub fn binding_read_prefix_with_limits(
        bytes: napi::bindgen_prelude::Uint8Array,
        max_array_entries: f64,
    ) -> napi::Result<DecodedDurationF64> {
        let storage_bytes = bytes.to_vec();
        (crate::api::DurationF64::read_prefix_with_limits(
            &storage_bytes,
            checked_usize(max_array_entries)?,
        ))
        .map(|value| DecodedDurationF64 {
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
