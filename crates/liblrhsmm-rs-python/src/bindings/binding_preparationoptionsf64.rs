use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct PreparationOptionsF64 {
    pub(crate) inner: Option<crate::api::PreparationOptionsF64>,
}
#[pymethods]
impl PreparationOptionsF64 {
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
impl PreparationOptionsF64 {
    #[getter(duration_cache_length)]
    fn binding_get_duration_cache_length(&self) -> PyResult<usize> {
        Ok(self
            .inner
            .as_ref()
            .ok_or_else(consumed)?
            .duration_cache_length)
    }
    #[setter(duration_cache_length)]
    fn binding_set_duration_cache_length(&mut self, value: usize) -> PyResult<()> {
        self.inner
            .as_mut()
            .ok_or_else(consumed)?
            .duration_cache_length = value;
        Ok(())
    }
    #[getter(duration_weight)]
    fn binding_get_duration_weight(&self) -> PyResult<f64> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.duration_weight)
    }
    #[setter(duration_weight)]
    fn binding_set_duration_weight(&mut self, value: f64) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.duration_weight = value;
        Ok(())
    }
    #[getter(temperature)]
    fn binding_get_temperature(&self) -> PyResult<f64> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.temperature)
    }
    #[setter(temperature)]
    fn binding_set_temperature(&mut self, value: f64) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.temperature = value;
        Ok(())
    }
}
#[pymethods]
impl PreparationOptionsF64 {
    #[new]
    #[pyo3(signature = ())]
    pub fn binding_new() -> pyo3::PyResult<Self> {
        Ok(PreparationOptionsF64 {
            inner: Some(crate::api::PreparationOptionsF64::new()),
        })
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(PreparationOptionsF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
