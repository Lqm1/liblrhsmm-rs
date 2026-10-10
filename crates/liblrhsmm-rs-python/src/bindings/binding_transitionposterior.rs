use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct TransitionPosterior {
    pub(crate) inner: Option<crate::api::TransitionPosterior>,
}
#[pymethods]
impl TransitionPosterior {
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
impl TransitionPosterior {
    #[new]
    #[pyo3(signature = (values))]
    pub fn binding_new(values: Vec<f32>) -> pyo3::PyResult<Self> {
        (crate::api::TransitionPosterior::new(&values))
            .map(|value| TransitionPosterior { inner: Some(value) })
    }
    #[pyo3(name = "values")]
    #[pyo3(signature = ())]
    pub fn binding_values(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).values())
    }
    #[pyo3(name = "set_values")]
    #[pyo3(signature = (values))]
    pub fn binding_set_values(&mut self, values: Vec<f32>) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).set_values(&values)
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(TransitionPosterior {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
