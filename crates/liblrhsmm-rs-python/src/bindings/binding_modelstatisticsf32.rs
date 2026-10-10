use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct ModelStatisticsF32 {
    pub(crate) inner: Option<crate::api::ModelStatisticsF32>,
}
#[pymethods]
impl ModelStatisticsF32 {
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
impl ModelStatisticsF32 {
    #[new]
    #[pyo3(signature = ())]
    pub fn binding_new() -> pyo3::PyResult<Self> {
        Ok(ModelStatisticsF32 {
            inner: Some(crate::api::ModelStatisticsF32::new()),
        })
    }
    #[staticmethod]
    #[pyo3(name = "from_model")]
    #[pyo3(signature = (value))]
    pub fn binding_from_model(value: pyo3::PyRef<'_, ModelF32>) -> pyo3::PyResult<Self> {
        (crate::api::ModelStatisticsF32::from_model(value.inner.as_ref().ok_or_else(consumed)?))
            .map(|value| ModelStatisticsF32 { inner: Some(value) })
    }
    #[getter(streams)]
    pub fn binding_streams(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).streams())
    }
    #[getter(durations)]
    pub fn binding_durations(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).durations())
    }
    #[pyo3(name = "stream")]
    #[pyo3(signature = (index))]
    pub fn binding_stream(&self, index: usize) -> pyo3::PyResult<Option<StreamStatisticsF32>> {
        Ok(((self.inner.as_ref().ok_or_else(consumed)?).stream(index))
            .map(|value| StreamStatisticsF32 { inner: Some(value) }))
    }
    #[pyo3(name = "duration")]
    #[pyo3(signature = (index))]
    pub fn binding_duration(&self, index: usize) -> pyo3::PyResult<Option<DurationStatisticsF32>> {
        Ok(
            ((self.inner.as_ref().ok_or_else(consumed)?).duration(index))
                .map(|value| DurationStatisticsF32 { inner: Some(value) }),
        )
    }
    #[pyo3(name = "set_stream")]
    #[pyo3(signature = (index, value))]
    pub fn binding_set_stream(
        &mut self,
        index: usize,
        value: pyo3::PyRef<'_, StreamStatisticsF32>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?)
            .set_stream(index, value.inner.as_ref().ok_or_else(consumed)?)
    }
    #[pyo3(name = "set_duration")]
    #[pyo3(signature = (index, value))]
    pub fn binding_set_duration(
        &mut self,
        index: usize,
        value: pyo3::PyRef<'_, DurationStatisticsF32>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?)
            .set_duration(index, value.inner.as_ref().ok_or_else(consumed)?)
    }
    #[pyo3(name = "replace")]
    #[pyo3(signature = (streams, durations))]
    pub fn binding_replace(
        &mut self,
        streams: pyo3::PyRef<'_, StreamStatisticsCollectionF32>,
        durations: pyo3::PyRef<'_, DurationStatisticsCollectionF32>,
    ) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).replace(
            streams.inner.as_ref().ok_or_else(consumed)?,
            durations.inner.as_ref().ok_or_else(consumed)?,
        );
        Ok(())
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(ModelStatisticsF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
    #[pyo3(name = "clear")]
    #[pyo3(signature = ())]
    pub fn binding_clear(&mut self) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).clear();
        Ok(())
    }
}
#[pymethods]
impl ModelStatisticsF32 {
    #[pyo3(name = "collect_initialization")]
    #[pyo3(signature = (model, observation, segmentation))]
    pub fn binding_collect_initialization(
        &mut self,
        model: pyo3::PyRef<'_, ModelF32>,
        observation: pyo3::PyRef<'_, ObservationF32>,
        segmentation: pyo3::PyRef<'_, SegmentationF32>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).collect_initialization(
            model.inner.as_ref().ok_or_else(consumed)?,
            observation.inner.as_ref().ok_or_else(consumed)?,
            segmentation.inner.as_ref().ok_or_else(consumed)?,
        )
    }
    #[pyo3(name = "collect_hsmm")]
    #[pyo3(signature = (model, observation, segmentation, expectation))]
    pub fn binding_collect_hsmm(
        &mut self,
        model: pyo3::PyRef<'_, ModelF32>,
        observation: pyo3::PyRef<'_, ObservationF32>,
        segmentation: pyo3::PyRef<'_, SegmentationF32>,
        expectation: pyo3::PyRef<'_, HsmmExpectationF32>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).collect_hsmm(
            model.inner.as_ref().ok_or_else(consumed)?,
            observation.inner.as_ref().ok_or_else(consumed)?,
            segmentation.inner.as_ref().ok_or_else(consumed)?,
            expectation.inner.as_ref().ok_or_else(consumed)?,
        )
    }
    #[pyo3(name = "collect_geometric")]
    #[pyo3(signature = (model, observation, segmentation, expectation))]
    pub fn binding_collect_geometric(
        &mut self,
        model: pyo3::PyRef<'_, ModelF32>,
        observation: pyo3::PyRef<'_, ObservationF32>,
        segmentation: pyo3::PyRef<'_, SegmentationF32>,
        expectation: pyo3::PyRef<'_, GeometricExpectationF32>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).collect_geometric(
            model.inner.as_ref().ok_or_else(consumed)?,
            observation.inner.as_ref().ok_or_else(consumed)?,
            segmentation.inner.as_ref().ok_or_else(consumed)?,
            expectation.inner.as_ref().ok_or_else(consumed)?,
        )
    }
    #[pyo3(name = "estimate_hsmm")]
    #[pyo3(signature = (model, observation, segmentation, options))]
    pub fn binding_estimate_hsmm(
        &mut self,
        model: pyo3::PyRef<'_, ModelF32>,
        observation: pyo3::PyRef<'_, ObservationF32>,
        segmentation: pyo3::PyRef<'_, SegmentationF32>,
        options: pyo3::PyRef<'_, HsmmOptionsF32>,
    ) -> pyo3::PyResult<f32> {
        (self.inner.as_mut().ok_or_else(consumed)?).estimate_hsmm(
            model.inner.as_ref().ok_or_else(consumed)?,
            observation.inner.as_ref().ok_or_else(consumed)?,
            segmentation.inner.as_ref().ok_or_else(consumed)?,
            options.inner.as_ref().ok_or_else(consumed)?,
        )
    }
    #[pyo3(name = "estimate_geometric")]
    #[pyo3(signature = (model, observation, segmentation, options))]
    pub fn binding_estimate_geometric(
        &mut self,
        model: pyo3::PyRef<'_, ModelF32>,
        observation: pyo3::PyRef<'_, ObservationF32>,
        segmentation: pyo3::PyRef<'_, SegmentationF32>,
        options: pyo3::PyRef<'_, GeometricOptionsF32>,
    ) -> pyo3::PyResult<f32> {
        (self.inner.as_mut().ok_or_else(consumed)?).estimate_geometric(
            model.inner.as_ref().ok_or_else(consumed)?,
            observation.inner.as_ref().ok_or_else(consumed)?,
            segmentation.inner.as_ref().ok_or_else(consumed)?,
            options.inner.as_ref().ok_or_else(consumed)?,
        )
    }
}
