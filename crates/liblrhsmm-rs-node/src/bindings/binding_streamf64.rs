use super::*;
#[napi]
pub struct StreamF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::StreamF64>>,
}
#[cfg(feature = "serialization")]
#[napi]
impl StreamF64 {
    #[napi(js_name = "read")]
    pub fn binding_read(bytes: napi::bindgen_prelude::Uint8Array) -> napi::Result<Self> {
        let storage_bytes = bytes.to_vec();
        (crate::api::StreamF64::read(&storage_bytes)).map(|value| StreamF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "read_with_limits")]
    pub fn binding_read_with_limits(
        bytes: napi::bindgen_prelude::Uint8Array,
        max_array_entries: f64,
    ) -> napi::Result<Self> {
        let storage_bytes = bytes.to_vec();
        (crate::api::StreamF64::read_with_limits(&storage_bytes, checked_usize(max_array_entries)?))
            .map(|value| StreamF64 {
                inner: std::cell::RefCell::new(Some(value)),
            })
    }
    #[napi(js_name = "read_prefix")]
    pub fn binding_read_prefix(
        bytes: napi::bindgen_prelude::Uint8Array,
    ) -> napi::Result<DecodedStreamF64> {
        let storage_bytes = bytes.to_vec();
        (crate::api::StreamF64::read_prefix(&storage_bytes)).map(|value| DecodedStreamF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "read_prefix_with_limits")]
    pub fn binding_read_prefix_with_limits(
        bytes: napi::bindgen_prelude::Uint8Array,
        max_array_entries: f64,
    ) -> napi::Result<DecodedStreamF64> {
        let storage_bytes = bytes.to_vec();
        (crate::api::StreamF64::read_prefix_with_limits(
            &storage_bytes,
            checked_usize(max_array_entries)?,
        ))
        .map(|value| DecodedStreamF64 {
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
impl StreamF64 {
    #[napi(js_name = "update")]
    pub fn binding_update(
        &self,
        statistics: &StreamStatisticsF64,
        options: &UpdateOptionsF64,
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
impl StreamF64 {
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
impl StreamF64 {
    #[napi(constructor)]
    pub fn binding_new(emissions: f64, components: f64, dimensions: f64) -> napi::Result<Self> {
        (crate::api::StreamF64::new(
            checked_usize(emissions)?,
            checked_usize(components)?,
            checked_usize(dimensions)?,
        ))
        .map(|value| StreamF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "empty")]
    pub fn binding_empty() -> napi::Result<Self> {
        Ok(StreamF64 {
            inner: std::cell::RefCell::new(Some(crate::api::StreamF64::empty())),
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
    pub fn binding_weight(&self) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        Ok(<napi::bindgen_prelude::Float64Array>::from(
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
        values: napi::bindgen_prelude::Float64Array,
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
    pub fn binding_emission(&self, index: f64) -> napi::Result<Option<GaussianF64>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .emission(checked_usize(index)?))
        .map(|value| GaussianF64 {
            inner: std::cell::RefCell::new(Some(value)),
        }))
    }
    #[napi(js_name = "set_emission")]
    pub fn binding_set_emission(&self, index: f64, value: &GaussianF64) -> napi::Result<()> {
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
        weight: napi::bindgen_prelude::Float64Array,
        emissions: &GaussianCollectionF64,
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
        Ok(StreamF64 {
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
    pub fn binding_prepare(&self) -> napi::Result<PreparedStreamF64> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .prepare())
        .map(|value| PreparedStreamF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "prepare_owned")]
    pub fn binding_prepare_owned(&self) -> napi::Result<PreparedStreamF64> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .take()
            .ok_or_else(consumed)?)
        .prepare_owned())
        .map(|value| PreparedStreamF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
}
