use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct DatasetF64 {
    pub(crate) inner: Option<crate::api::DatasetF64>,
}
#[pymethods]
impl DatasetF64 {
    fn close(&mut self) {
        self.inner = None;
    }
    fn free(&mut self) {
        self.inner = None;
    }
    #[getter]
    fn is_closed(&self) -> bool {
        self.inner.is_none()
    }
}
#[pymethods]
impl DatasetF64 {
    #[new]
    #[pyo3(signature = (observations, segmentations))]
    pub fn binding_new(
        observations: pyo3::PyRef<'_, ObservationCollectionF64>,
        segmentations: pyo3::PyRef<'_, SegmentationCollectionF64>,
    ) -> pyo3::PyResult<Self> {
        Ok(DatasetF64 {
            inner: Some(crate::api::DatasetF64::new(
                observations.inner.as_ref().ok_or_else(consumed)?,
                segmentations.inner.as_ref().ok_or_else(consumed)?,
            )),
        })
    }
    #[getter(observation_count)]
    pub fn binding_observation_count(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).observation_count())
    }
    #[getter(segmentation_count)]
    pub fn binding_segmentation_count(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).segmentation_count())
    }
    #[pyo3(name = "observations")]
    #[pyo3(signature = ())]
    pub fn binding_observations(&self) -> pyo3::PyResult<ObservationCollectionF64> {
        Ok(ObservationCollectionF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).observations()),
        })
    }
    #[pyo3(name = "segmentations")]
    #[pyo3(signature = ())]
    pub fn binding_segmentations(&self) -> pyo3::PyResult<SegmentationCollectionF64> {
        Ok(SegmentationCollectionF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).segmentations()),
        })
    }
    #[pyo3(name = "observation")]
    #[pyo3(signature = (index))]
    pub fn binding_observation(&self, index: usize) -> pyo3::PyResult<Option<ObservationF64>> {
        Ok(
            ((self.inner.as_ref().ok_or_else(consumed)?).observation(index))
                .map(|value| ObservationF64 { inner: Some(value) }),
        )
    }
    #[pyo3(name = "segmentation")]
    #[pyo3(signature = (index))]
    pub fn binding_segmentation(&self, index: usize) -> pyo3::PyResult<Option<SegmentationF64>> {
        Ok(
            ((self.inner.as_ref().ok_or_else(consumed)?).segmentation(index))
                .map(|value| SegmentationF64 { inner: Some(value) }),
        )
    }
    #[pyo3(name = "set_observation")]
    #[pyo3(signature = (index, value))]
    pub fn binding_set_observation(
        &mut self,
        index: usize,
        value: pyo3::PyRef<'_, ObservationF64>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?)
            .set_observation(index, value.inner.as_ref().ok_or_else(consumed)?)
    }
    #[pyo3(name = "set_segmentation")]
    #[pyo3(signature = (index, value))]
    pub fn binding_set_segmentation(
        &mut self,
        index: usize,
        value: pyo3::PyRef<'_, SegmentationF64>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?)
            .set_segmentation(index, value.inner.as_ref().ok_or_else(consumed)?)
    }
    #[pyo3(name = "replace")]
    #[pyo3(signature = (observations, segmentations))]
    pub fn binding_replace(
        &mut self,
        observations: pyo3::PyRef<'_, ObservationCollectionF64>,
        segmentations: pyo3::PyRef<'_, SegmentationCollectionF64>,
    ) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).replace(
            observations.inner.as_ref().ok_or_else(consumed)?,
            segmentations.inner.as_ref().ok_or_else(consumed)?,
        );
        Ok(())
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(DatasetF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
    #[pyo3(name = "regroup")]
    #[pyo3(signature = (segments_per_unit))]
    pub fn binding_regroup(&self, segments_per_unit: usize) -> pyo3::PyResult<Self> {
        ((self.inner.as_ref().ok_or_else(consumed)?).regroup(segments_per_unit))
            .map(|value| DatasetF64 { inner: Some(value) })
    }
}
