use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct JumpGroupF64 {
    pub(crate) inner: Option<crate::api::JumpGroupF64>,
}
#[pymethods]
impl JumpGroupF64 {
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
impl JumpGroupF64 {
    #[new]
    #[pyo3(signature = (deltas, probabilities))]
    pub fn binding_new(deltas: Vec<i32>, probabilities: Vec<f64>) -> pyo3::PyResult<Self> {
        (crate::api::JumpGroupF64::new(&deltas, &probabilities))
            .map(|value| JumpGroupF64 { inner: Some(value) })
    }
    #[getter(length)]
    pub fn binding_length(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).length())
    }
    #[pyo3(name = "deltas")]
    #[pyo3(signature = ())]
    pub fn binding_deltas(&self) -> pyo3::PyResult<Vec<i32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).deltas())
    }
    #[pyo3(name = "probabilities")]
    #[pyo3(signature = ())]
    pub fn binding_probabilities(&self) -> pyo3::PyResult<Vec<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).probabilities())
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(JumpGroupF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
