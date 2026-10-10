use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct GeometricOptionsF32 {
    pub(crate) inner: Option<crate::api::GeometricOptionsF32>,
}
#[pymethods]
impl GeometricOptionsF32 {
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
impl GeometricOptionsF32 {
    #[getter(temperature)]
    fn binding_get_temperature(&self) -> PyResult<f32> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.temperature)
    }
    #[setter(temperature)]
    fn binding_set_temperature(&mut self, value: f32) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.temperature = value;
        Ok(())
    }
    #[getter(pruning_slope)]
    fn binding_get_pruning_slope(&self) -> PyResult<f32> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.pruning_slope)
    }
    #[setter(pruning_slope)]
    fn binding_set_pruning_slope(&mut self, value: f32) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.pruning_slope = value;
        Ok(())
    }
}
#[pymethods]
impl GeometricOptionsF32 {
    #[new]
    #[pyo3(signature = ())]
    pub fn binding_new() -> pyo3::PyResult<Self> {
        Ok(GeometricOptionsF32 {
            inner: Some(crate::api::GeometricOptionsF32::new()),
        })
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(GeometricOptionsF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
