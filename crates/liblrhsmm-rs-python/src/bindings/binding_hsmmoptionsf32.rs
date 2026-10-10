use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct HsmmOptionsF32 {
    pub(crate) inner: Option<crate::api::HsmmOptionsF32>,
}
#[pymethods]
impl HsmmOptionsF32 {
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
impl HsmmOptionsF32 {
    #[getter(temperature)]
    fn binding_get_temperature(&self) -> PyResult<f32> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.temperature)
    }
    #[setter(temperature)]
    fn binding_set_temperature(&mut self, value: f32) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.temperature = value;
        Ok(())
    }
    #[getter(duration_weight)]
    fn binding_get_duration_weight(&self) -> PyResult<f32> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.duration_weight)
    }
    #[setter(duration_weight)]
    fn binding_set_duration_weight(&mut self, value: f32) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.duration_weight = value;
        Ok(())
    }
    #[getter(state_radius)]
    fn binding_get_state_radius(&self) -> PyResult<f32> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.state_radius)
    }
    #[setter(state_radius)]
    fn binding_set_state_radius(&mut self, value: f32) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.state_radius = value;
        Ok(())
    }
    #[getter(duration_extra)]
    fn binding_get_duration_extra(&self) -> PyResult<usize> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.duration_extra)
    }
    #[setter(duration_extra)]
    fn binding_set_duration_extra(&mut self, value: usize) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.duration_extra = value;
        Ok(())
    }
    #[getter(duration_extra_factor)]
    fn binding_get_duration_extra_factor(&self) -> PyResult<f32> {
        Ok(self
            .inner
            .as_ref()
            .ok_or_else(consumed)?
            .duration_extra_factor)
    }
    #[setter(duration_extra_factor)]
    fn binding_set_duration_extra_factor(&mut self, value: f32) -> PyResult<()> {
        self.inner
            .as_mut()
            .ok_or_else(consumed)?
            .duration_extra_factor = value;
        Ok(())
    }
}
#[pymethods]
impl HsmmOptionsF32 {
    #[new]
    #[pyo3(signature = ())]
    pub fn binding_new() -> pyo3::PyResult<Self> {
        Ok(HsmmOptionsF32 {
            inner: Some(crate::api::HsmmOptionsF32::new()),
        })
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(HsmmOptionsF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
