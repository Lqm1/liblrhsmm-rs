use super::*;
#[napi]
pub struct GaussianF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::GaussianF64>>,
}
#[napi]
impl GaussianF64 {
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
impl GaussianF64 {
    #[napi(constructor)]
    pub fn binding_new(mixtures: f64, dimensions: f64) -> napi::Result<Self> {
        (crate::api::GaussianF64::new(checked_usize(mixtures)?, checked_usize(dimensions)?)).map(
            |value| GaussianF64 {
                inner: std::cell::RefCell::new(Some(value)),
            },
        )
    }
    #[napi(getter, js_name = "dimensions")]
    pub fn binding_dimensions(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .dimensions() as f64)
    }
    #[napi(getter, js_name = "mixtures")]
    pub fn binding_mixtures(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .mixtures() as f64)
    }
    #[napi(js_name = "weights")]
    pub fn binding_weights(&self) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        Ok(<napi::bindgen_prelude::Float64Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .weights(),
        ))
    }
    #[napi(js_name = "means")]
    pub fn binding_means(&self) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        Ok(<napi::bindgen_prelude::Float64Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .means(),
        ))
    }
    #[napi(js_name = "variances")]
    pub fn binding_variances(&self) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        Ok(<napi::bindgen_prelude::Float64Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .variances(),
        ))
    }
    #[napi(js_name = "variance_floors")]
    pub fn binding_variance_floors(&self) -> napi::Result<napi::bindgen_prelude::Float64Array> {
        Ok(<napi::bindgen_prelude::Float64Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .variance_floors(),
        ))
    }
    #[napi(js_name = "replace")]
    pub fn binding_replace(
        &self,
        dimensions: f64,
        weights: napi::bindgen_prelude::Float64Array,
        means: napi::bindgen_prelude::Float64Array,
        variances: napi::bindgen_prelude::Float64Array,
        variance_floors: napi::bindgen_prelude::Float64Array,
    ) -> napi::Result<()> {
        let storage_weights = weights.to_vec();
        let storage_means = means.to_vec();
        let storage_variances = variances.to_vec();
        let storage_variance_floors = variance_floors.to_vec();
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .replace(
            checked_usize(dimensions)?,
            &storage_weights,
            &storage_means,
            &storage_variances,
            &storage_variance_floors,
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
        Ok(GaussianF64 {
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
    #[napi(js_name = "combine")]
    pub fn binding_combine(distributions: &GaussianCollectionF64) -> napi::Result<Self> {
        (crate::api::GaussianF64::combine(
            distributions
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| GaussianF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "from_durations")]
    pub fn binding_from_durations(distributions: &DurationCollectionF64) -> napi::Result<Self> {
        (crate::api::GaussianF64::from_durations(
            distributions
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| GaussianF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "increase_mixtures")]
    pub fn binding_increase_mixtures(&self, count: f64, displacement: f64) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .increase_mixtures(checked_usize(count)?, displacement)
    }
    #[napi(js_name = "prepare")]
    pub fn binding_prepare(&self) -> napi::Result<PreparedGaussianF64> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .prepare())
        .map(|value| PreparedGaussianF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "prepare_owned")]
    pub fn binding_prepare_owned(&self) -> napi::Result<PreparedGaussianF64> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .take()
            .ok_or_else(consumed)?)
        .prepare_owned())
        .map(|value| PreparedGaussianF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "update")]
    pub fn binding_update(
        &self,
        statistics: &GaussianStatisticsF64,
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
impl GaussianF64 {
    #[napi(js_name = "read")]
    pub fn binding_read(bytes: napi::bindgen_prelude::Uint8Array) -> napi::Result<Self> {
        let storage_bytes = bytes.to_vec();
        (crate::api::GaussianF64::read(&storage_bytes)).map(|value| GaussianF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "read_with_limits")]
    pub fn binding_read_with_limits(
        bytes: napi::bindgen_prelude::Uint8Array,
        max_array_entries: f64,
    ) -> napi::Result<Self> {
        let storage_bytes = bytes.to_vec();
        (crate::api::GaussianF64::read_with_limits(
            &storage_bytes,
            checked_usize(max_array_entries)?,
        ))
        .map(|value| GaussianF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "read_prefix")]
    pub fn binding_read_prefix(
        bytes: napi::bindgen_prelude::Uint8Array,
    ) -> napi::Result<DecodedGaussianF64> {
        let storage_bytes = bytes.to_vec();
        (crate::api::GaussianF64::read_prefix(&storage_bytes)).map(|value| DecodedGaussianF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "read_prefix_with_limits")]
    pub fn binding_read_prefix_with_limits(
        bytes: napi::bindgen_prelude::Uint8Array,
        max_array_entries: f64,
    ) -> napi::Result<DecodedGaussianF64> {
        let storage_bytes = bytes.to_vec();
        (crate::api::GaussianF64::read_prefix_with_limits(
            &storage_bytes,
            checked_usize(max_array_entries)?,
        ))
        .map(|value| DecodedGaussianF64 {
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
