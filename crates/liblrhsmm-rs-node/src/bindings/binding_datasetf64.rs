use super::*;
#[napi]
pub struct DatasetF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::DatasetF64>>,
}
#[napi]
impl DatasetF64 {
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
impl DatasetF64 {
    #[napi(constructor)]
    pub fn binding_new(
        observations: &ObservationCollectionF64,
        segmentations: &SegmentationCollectionF64,
    ) -> napi::Result<Self> {
        Ok(DatasetF64 {
            inner: std::cell::RefCell::new(Some(crate::api::DatasetF64::new(
                observations
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?,
                segmentations
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?,
            ))),
        })
    }
    #[napi(getter, js_name = "observation_count")]
    pub fn binding_observation_count(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .observation_count() as f64)
    }
    #[napi(getter, js_name = "segmentation_count")]
    pub fn binding_segmentation_count(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .segmentation_count() as f64)
    }
    #[napi(js_name = "observations")]
    pub fn binding_observations(&self) -> napi::Result<ObservationCollectionF64> {
        Ok(ObservationCollectionF64 {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .observations(),
            )),
        })
    }
    #[napi(js_name = "segmentations")]
    pub fn binding_segmentations(&self) -> napi::Result<SegmentationCollectionF64> {
        Ok(SegmentationCollectionF64 {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .segmentations(),
            )),
        })
    }
    #[napi(js_name = "observation")]
    pub fn binding_observation(&self, index: f64) -> napi::Result<Option<ObservationF64>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .observation(checked_usize(index)?))
        .map(|value| ObservationF64 {
            inner: std::cell::RefCell::new(Some(value)),
        }))
    }
    #[napi(js_name = "segmentation")]
    pub fn binding_segmentation(&self, index: f64) -> napi::Result<Option<SegmentationF64>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .segmentation(checked_usize(index)?))
        .map(|value| SegmentationF64 {
            inner: std::cell::RefCell::new(Some(value)),
        }))
    }
    #[napi(js_name = "set_observation")]
    pub fn binding_set_observation(&self, index: f64, value: &ObservationF64) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_observation(
            checked_usize(index)?,
            value
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        )
    }
    #[napi(js_name = "set_segmentation")]
    pub fn binding_set_segmentation(
        &self,
        index: f64,
        value: &SegmentationF64,
    ) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_segmentation(
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
        observations: &ObservationCollectionF64,
        segmentations: &SegmentationCollectionF64,
    ) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .replace(
            observations
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            segmentations
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
        Ok(DatasetF64 {
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
    #[napi(js_name = "regroup")]
    pub fn binding_regroup(&self, segments_per_unit: f64) -> napi::Result<Self> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .regroup(checked_usize(segments_per_unit)?))
        .map(|value| DatasetF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
}
