use super::*;
#[napi]
pub struct GaussianStatisticsF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::GaussianStatisticsF32>>,
}
#[napi]
impl GaussianStatisticsF32 {
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
impl GaussianStatisticsF32 {
    #[napi(constructor)]
    pub fn binding_new(mixtures: f64, dimensions: f64) -> napi::Result<Self> {
        (crate::api::GaussianStatisticsF32::new(
            checked_usize(mixtures)?,
            checked_usize(dimensions)?,
        ))
        .map(|value| GaussianStatisticsF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "from_gaussian")]
    pub fn binding_from_gaussian(value: &GaussianF32) -> napi::Result<Self> {
        (crate::api::GaussianStatisticsF32::from_gaussian(
            value
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| GaussianStatisticsF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
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
    #[napi(js_name = "weighted_sums")]
    pub fn binding_weighted_sums(&self) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        Ok(<napi::bindgen_prelude::Float32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .weighted_sums(),
        ))
    }
    #[napi(js_name = "weighted_squared_sums")]
    pub fn binding_weighted_squared_sums(
        &self,
    ) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        Ok(<napi::bindgen_prelude::Float32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .weighted_squared_sums(),
        ))
    }
    #[napi(js_name = "component_occupancies")]
    pub fn binding_component_occupancies(
        &self,
    ) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        Ok(<napi::bindgen_prelude::Float32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .component_occupancies(),
        ))
    }
    #[napi(js_name = "state_occupancy")]
    pub fn binding_state_occupancy(&self) -> napi::Result<napi::bindgen_prelude::Float32Array> {
        Ok(<napi::bindgen_prelude::Float32Array>::from(
            (self
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?)
            .state_occupancy(),
        ))
    }
    #[napi(js_name = "replace")]
    pub fn binding_replace(
        &self,
        dimensions: f64,
        weighted_sums: napi::bindgen_prelude::Float32Array,
        weighted_squared_sums: napi::bindgen_prelude::Float32Array,
        component_occupancies: napi::bindgen_prelude::Float32Array,
        state_occupancy: napi::bindgen_prelude::Float32Array,
    ) -> napi::Result<()> {
        let storage_weighted_sums = weighted_sums.to_vec();
        let storage_weighted_squared_sums = weighted_squared_sums.to_vec();
        let storage_component_occupancies = component_occupancies.to_vec();
        let storage_state_occupancy = state_occupancy.to_vec();
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .replace(
            checked_usize(dimensions)?,
            &storage_weighted_sums,
            &storage_weighted_squared_sums,
            &storage_component_occupancies,
            &storage_state_occupancy,
        )
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(GaussianStatisticsF32 {
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
    #[napi(js_name = "clear")]
    pub fn binding_clear(&self) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .clear();
        Ok(())
    }
}
