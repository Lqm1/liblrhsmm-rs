use super::*;
#[napi]
pub struct ModelStatisticsF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::ModelStatisticsF64>>,
}
#[napi]
impl ModelStatisticsF64 {
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
impl ModelStatisticsF64 {
    #[napi(constructor)]
    pub fn binding_new() -> napi::Result<Self> {
        Ok(ModelStatisticsF64 {
            inner: std::cell::RefCell::new(Some(crate::api::ModelStatisticsF64::new())),
        })
    }
    #[napi(js_name = "from_model")]
    pub fn binding_from_model(value: &ModelF64) -> napi::Result<Self> {
        (crate::api::ModelStatisticsF64::from_model(
            value
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| ModelStatisticsF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(getter, js_name = "streams")]
    pub fn binding_streams(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .streams() as f64)
    }
    #[napi(getter, js_name = "durations")]
    pub fn binding_durations(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .durations() as f64)
    }
    #[napi(js_name = "stream")]
    pub fn binding_stream(&self, index: f64) -> napi::Result<Option<StreamStatisticsF64>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .stream(checked_usize(index)?))
        .map(|value| StreamStatisticsF64 {
            inner: std::cell::RefCell::new(Some(value)),
        }))
    }
    #[napi(js_name = "duration")]
    pub fn binding_duration(&self, index: f64) -> napi::Result<Option<DurationStatisticsF64>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .duration(checked_usize(index)?))
        .map(|value| DurationStatisticsF64 {
            inner: std::cell::RefCell::new(Some(value)),
        }))
    }
    #[napi(js_name = "set_stream")]
    pub fn binding_set_stream(&self, index: f64, value: &StreamStatisticsF64) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_stream(
            checked_usize(index)?,
            value
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        )
    }
    #[napi(js_name = "set_duration")]
    pub fn binding_set_duration(
        &self,
        index: f64,
        value: &DurationStatisticsF64,
    ) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_duration(
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
        streams: &StreamStatisticsCollectionF64,
        durations: &DurationStatisticsCollectionF64,
    ) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .replace(
            streams
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            durations
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        );
        Ok(())
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(ModelStatisticsF64 {
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
#[napi]
impl ModelStatisticsF64 {
    #[napi(js_name = "collect_initialization")]
    pub fn binding_collect_initialization(
        &self,
        model: &ModelF64,
        observation: &ObservationF64,
        segmentation: &SegmentationF64,
    ) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .collect_initialization(
            model
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            observation
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            segmentation
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        )
    }
    #[napi(js_name = "collect_hsmm")]
    pub fn binding_collect_hsmm(
        &self,
        model: &ModelF64,
        observation: &ObservationF64,
        segmentation: &SegmentationF64,
        expectation: &HsmmExpectationF64,
    ) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .collect_hsmm(
            model
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            observation
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            segmentation
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            expectation
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        )
    }
    #[napi(js_name = "collect_geometric")]
    pub fn binding_collect_geometric(
        &self,
        model: &ModelF64,
        observation: &ObservationF64,
        segmentation: &SegmentationF64,
        expectation: &GeometricExpectationF64,
    ) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .collect_geometric(
            model
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            observation
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            segmentation
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            expectation
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        )
    }
    #[napi(js_name = "estimate_hsmm")]
    pub fn binding_estimate_hsmm(
        &self,
        model: &ModelF64,
        observation: &ObservationF64,
        segmentation: &SegmentationF64,
        options: &HsmmOptionsF64,
    ) -> napi::Result<f64> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .estimate_hsmm(
            model
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            observation
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            segmentation
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
    #[napi(js_name = "estimate_geometric")]
    pub fn binding_estimate_geometric(
        &self,
        model: &ModelF64,
        observation: &ObservationF64,
        segmentation: &SegmentationF64,
        options: &GeometricOptionsF64,
    ) -> napi::Result<f64> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .estimate_geometric(
            model
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            observation
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            segmentation
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
