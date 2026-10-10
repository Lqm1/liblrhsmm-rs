use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct EmissionDensitiesF64 {
    pub(crate) inner: Option<crate::api::EmissionDensitiesF64>,
}
#[pymethods]
impl EmissionDensitiesF64 {
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
impl EmissionDensitiesF64 {
    #[getter(frames)]
    pub fn binding_frames(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).frames())
    }
    #[getter(states)]
    pub fn binding_states(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).states())
    }
    #[pyo3(name = "values")]
    #[pyo3(signature = ())]
    pub fn binding_values(&self) -> pyo3::PyResult<Vec<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).values())
    }
    #[pyo3(name = "get")]
    #[pyo3(signature = (frame, state))]
    pub fn binding_get(&self, frame: usize, state: usize) -> pyo3::PyResult<Option<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).get(frame, state))
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(EmissionDensitiesF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
