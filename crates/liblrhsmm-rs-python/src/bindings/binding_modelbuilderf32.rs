use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct ModelBuilderF32 {
    pub(crate) inner: Option<crate::api::ModelBuilderF32>,
}
#[pymethods]
impl ModelBuilderF32 {
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
impl ModelBuilderF32 {
    #[new]
    #[pyo3(signature = (streams, durations))]
    pub fn binding_new(streams: usize, durations: usize) -> pyo3::PyResult<Self> {
        (crate::api::ModelBuilderF32::new(streams, durations))
            .map(|value| ModelBuilderF32 { inner: Some(value) })
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
    pub fn binding_stream(&self, index: usize) -> pyo3::PyResult<Option<StreamF32>> {
        Ok(((self.inner.as_ref().ok_or_else(consumed)?).stream(index))
            .map(|value| StreamF32 { inner: Some(value) }))
    }
    #[pyo3(name = "duration")]
    #[pyo3(signature = (index))]
    pub fn binding_duration(&self, index: usize) -> pyo3::PyResult<Option<DurationF32>> {
        Ok(
            ((self.inner.as_ref().ok_or_else(consumed)?).duration(index))
                .map(|value| DurationF32 { inner: Some(value) }),
        )
    }
    #[pyo3(name = "has_stream")]
    #[pyo3(signature = (index))]
    pub fn binding_has_stream(&self, index: usize) -> pyo3::PyResult<bool> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).has_stream(index))
    }
    #[pyo3(name = "has_duration")]
    #[pyo3(signature = (index))]
    pub fn binding_has_duration(&self, index: usize) -> pyo3::PyResult<bool> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).has_duration(index))
    }
    #[pyo3(name = "resize")]
    #[pyo3(signature = (streams, durations))]
    pub fn binding_resize(&mut self, streams: usize, durations: usize) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).resize(streams, durations)
    }
    #[pyo3(name = "set_stream")]
    #[pyo3(signature = (index, value))]
    pub fn binding_set_stream(
        &mut self,
        index: usize,
        value: pyo3::PyRef<'_, StreamF32>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?)
            .set_stream(index, value.inner.as_ref().ok_or_else(consumed)?)
    }
    #[pyo3(name = "set_duration")]
    #[pyo3(signature = (index, value))]
    pub fn binding_set_duration(
        &mut self,
        index: usize,
        value: pyo3::PyRef<'_, DurationF32>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?)
            .set_duration(index, value.inner.as_ref().ok_or_else(consumed)?)
    }
    #[pyo3(name = "clear_stream")]
    #[pyo3(signature = (index))]
    pub fn binding_clear_stream(&mut self, index: usize) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).clear_stream(index)
    }
    #[pyo3(name = "clear_duration")]
    #[pyo3(signature = (index))]
    pub fn binding_clear_duration(&mut self, index: usize) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).clear_duration(index)
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(ModelBuilderF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
    #[pyo3(name = "build")]
    #[pyo3(signature = ())]
    pub fn binding_build(&mut self) -> pyo3::PyResult<ModelF32> {
        ((self.inner.take().ok_or_else(consumed)?).build())
            .map(|value| ModelF32 { inner: Some(value) })
    }
}
