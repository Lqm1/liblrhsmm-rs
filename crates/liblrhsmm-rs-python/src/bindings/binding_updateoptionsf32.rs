use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct UpdateOptionsF32 {
    pub(crate) inner: Option<crate::api::UpdateOptionsF32>,
}
#[pymethods]
impl UpdateOptionsF32 {
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
impl UpdateOptionsF32 {
    #[getter(duration_mode)]
    fn binding_get_duration_mode(&self) -> PyResult<u32> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.duration_mode)
    }
    #[setter(duration_mode)]
    fn binding_set_duration_mode(&mut self, value: u32) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.duration_mode = value;
        Ok(())
    }
    #[getter(duration_variance_floor)]
    fn binding_get_duration_variance_floor(&self) -> PyResult<f32> {
        Ok(self
            .inner
            .as_ref()
            .ok_or_else(consumed)?
            .duration_variance_floor)
    }
    #[setter(duration_variance_floor)]
    fn binding_set_duration_variance_floor(&mut self, value: f32) -> PyResult<()> {
        self.inner
            .as_mut()
            .ok_or_else(consumed)?
            .duration_variance_floor = value;
        Ok(())
    }
    #[getter(emission_variance_floor)]
    fn binding_get_emission_variance_floor(&self) -> PyResult<f32> {
        Ok(self
            .inner
            .as_ref()
            .ok_or_else(consumed)?
            .emission_variance_floor)
    }
    #[setter(emission_variance_floor)]
    fn binding_set_emission_variance_floor(&mut self, value: f32) -> PyResult<()> {
        self.inner
            .as_mut()
            .ok_or_else(consumed)?
            .emission_variance_floor = value;
        Ok(())
    }
}
#[pymethods]
impl UpdateOptionsF32 {
    #[new]
    #[pyo3(signature = ())]
    pub fn binding_new() -> pyo3::PyResult<Self> {
        Ok(UpdateOptionsF32 {
            inner: Some(crate::api::UpdateOptionsF32::new()),
        })
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(UpdateOptionsF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
