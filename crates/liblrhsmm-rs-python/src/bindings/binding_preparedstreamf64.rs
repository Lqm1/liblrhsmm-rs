use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct PreparedStreamF64 {
    pub(crate) inner: Option<crate::api::PreparedStreamF64>,
}
#[pymethods]
impl PreparedStreamF64 {
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
impl PreparedStreamF64 {
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(PreparedStreamF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
    #[pyo3(name = "parameters")]
    #[pyo3(signature = ())]
    pub fn binding_parameters(&self) -> pyo3::PyResult<StreamF64> {
        Ok(StreamF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).parameters()),
        })
    }
    #[pyo3(name = "into_parameters")]
    #[pyo3(signature = ())]
    pub fn binding_into_parameters(&mut self) -> pyo3::PyResult<StreamF64> {
        Ok(StreamF64 {
            inner: Some((self.inner.take().ok_or_else(consumed)?).into_parameters()),
        })
    }
    #[pyo3(name = "emission")]
    #[pyo3(signature = (state))]
    pub fn binding_emission(&self, state: usize) -> pyo3::PyResult<Option<PreparedGaussianF64>> {
        Ok(
            ((self.inner.as_ref().ok_or_else(consumed)?).emission(state))
                .map(|value| PreparedGaussianF64 { inner: Some(value) }),
        )
    }
}
