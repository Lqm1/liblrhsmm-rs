use super::*;
#[napi]
pub struct DurationStatisticsF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::DurationStatisticsF32>>,
}
#[napi]
impl DurationStatisticsF32 {
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
impl DurationStatisticsF32 {
    #[napi(getter, js_name = "value_sum")]
    pub fn binding_get_value_sum(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .value_sum as f64)
    }
    #[napi(setter, js_name = "value_sum")]
    pub fn binding_set_value_sum(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .value_sum = value as f32;
        Ok(())
    }
    #[napi(getter, js_name = "squared_sum")]
    pub fn binding_get_squared_sum(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .squared_sum as f64)
    }
    #[napi(setter, js_name = "squared_sum")]
    pub fn binding_set_squared_sum(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .squared_sum = value as f32;
        Ok(())
    }
    #[napi(getter, js_name = "occupancy")]
    pub fn binding_get_occupancy(&self) -> napi::Result<f64> {
        Ok(self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?
            .occupancy as f64)
    }
    #[napi(setter, js_name = "occupancy")]
    pub fn binding_set_occupancy(&self, value: f64) -> napi::Result<()> {
        self.inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?
            .occupancy = value as f32;
        Ok(())
    }
}
#[napi]
impl DurationStatisticsF32 {
    #[napi(constructor)]
    pub fn binding_new() -> napi::Result<Self> {
        Ok(DurationStatisticsF32 {
            inner: std::cell::RefCell::new(Some(crate::api::DurationStatisticsF32::new())),
        })
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(DurationStatisticsF32 {
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
}
