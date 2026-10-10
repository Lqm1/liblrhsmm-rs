use super::*;
#[napi]
pub struct StreamF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::StreamF32>>,
}
#[cfg(feature = "serialization")]
#[napi]
impl StreamF32 {
    #[napi(js_name = "read")]
    pub fn binding_read(bytes: napi::bindgen_prelude::Uint8Array) -> napi::Result<Self> {
        let storage_bytes = bytes.to_vec();
        (crate::api::StreamF32::read(&storage_bytes)).map(|value| StreamF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "read_with_limits")]
    pub fn binding_read_with_limits(
        bytes: napi::bindgen_prelude::Uint8Array,
        max_array_entries: f64,
    ) -> napi::Result<Self> {
        let storage_bytes = bytes.to_vec();
        (crate::api::StreamF32::read_with_limits(&storage_bytes, checked_usize(max_array_entries)?))
            .map(|value| StreamF32 {
                inner: std::cell::RefCell::new(Some(value)),
            })
    }
    #[napi(js_name = "read_prefix")]
    pub fn binding_read_prefix(
        bytes: napi::bindgen_prelude::Uint8Array,
    ) -> napi::Result<DecodedStreamF32> {
        let storage_bytes = bytes.to_vec();
        (crate::api::StreamF32::read_prefix(&storage_bytes)).map(|value| DecodedStreamF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "read_prefix_with_limits")]
    pub fn binding_read_prefix_with_limits(
        bytes: napi::bindgen_prelude::Uint8Array,
        max_array_entries: f64,
    ) -> napi::Result<DecodedStreamF32> {
        let storage_bytes = bytes.to_vec();
        (crate::api::StreamF32::read_prefix_with_limits(
            &storage_bytes,
            checked_usize(max_array_entries)?,
        ))
        .map(|value| DecodedStreamF32 {
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
    #[napi(js_name = "write_with_encoding")]
    pub fn binding_write_with_encoding(
        &self,
        code: u32,
    ) -> napi::Result<napi::bindgen_prelude::Uint8Array> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .write_with_encoding(code))
        .map(<napi::bindgen_prelude::Uint8Array>::from)
    }
}
#[napi]
impl StreamF32 {
    #[napi(js_name = "update")]
    pub fn binding_update(
        &self,
        statistics: &StreamStatisticsF32,
        options: &UpdateOptionsF32,
    ) -> napi::Result<()> {
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
#[napi]
impl StreamF32 {
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
impl StreamF32 {
    #[napi(constructor)]
    pub fn binding_new(emissions: f64, components: f64, dimensions: f64) -> napi::Result<Self> {
        (crate::api::StreamF32::new(
            checked_usize(emissions)?,
            checked_usize(components)?,
            checked_usize(dimensions)?,
        ))
        .map(|value| StreamF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "empty")]
    pub fn binding_empty() -> napi::Result<Self> {
        Ok(StreamF32 {
            inner: std::cell::RefCell::new(Some(crate::api::StreamF32::empty())),
        })
    }
    #[napi(getter, js_name = "emissions")]
    pub fn binding_emissions(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .emissions() as f64)
    }
    #[napi(js_name = "weight")]
    pub fn binding_weight(&self) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        Ok(<napi::bindgen_prelude::Float32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .weight(),
        ))
    }
    #[napi(js_name = "set_weight")]
    pub fn binding_set_weight(
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
        .set_weight(&storage_values)
    }
    #[napi(js_name = "emission")]
    pub fn binding_emission(&self, index: f64) -> napi::Result<Option<GaussianF32>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .emission(checked_usize(index)?))
        .map(|value| GaussianF32 {
            inner: std::cell::RefCell::new(Some(value)),
        }))
    }
    #[napi(js_name = "set_emission")]
    pub fn binding_set_emission(&self, index: f64, value: &GaussianF32) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_emission(
            checked_usize(index)?,
            value
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        )
    }
    #[napi(js_name = "replace")]
    pub fn binding_replace(
        &self,
        weight: napi::bindgen_prelude::Float32Array,
        emissions: &GaussianCollectionF32,
    ) -> napi::Result<()> {
        let storage_weight = weight.to_vec();
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .replace(
            &storage_weight,
            emissions
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        )
    }
    #[napi(js_name = "validate")]
    pub fn binding_validate(&self) -> napi::Result<()> {
        (self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .validate()
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(StreamF32 {
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
    #[napi(js_name = "prepare")]
    pub fn binding_prepare(&self) -> napi::Result<PreparedStreamF32> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .prepare())
        .map(|value| PreparedStreamF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "prepare_owned")]
    pub fn binding_prepare_owned(&self) -> napi::Result<PreparedStreamF32> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .take()
            .ok_or_else(consumed)?)
        .prepare_owned())
        .map(|value| PreparedStreamF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
}
