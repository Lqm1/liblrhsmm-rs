use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct PreparedStreamF32 {
    pub(crate) inner: Option<crate::api::PreparedStreamF32>,
}
#[pymethods]
impl PreparedStreamF32 {
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
impl PreparedStreamF32 {
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(PreparedStreamF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
    #[pyo3(name = "parameters")]
    #[pyo3(signature = ())]
    pub fn binding_parameters(&self) -> pyo3::PyResult<StreamF32> {
        Ok(StreamF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).parameters()),
        })
    }
    #[pyo3(name = "into_parameters")]
    #[pyo3(signature = ())]
    pub fn binding_into_parameters(&mut self) -> pyo3::PyResult<StreamF32> {
        Ok(StreamF32 {
            inner: Some((self.inner.take().ok_or_else(consumed)?).into_parameters()),
        })
    }
    #[pyo3(name = "emission")]
    #[pyo3(signature = (state))]
    pub fn binding_emission(&self, state: usize) -> pyo3::PyResult<Option<PreparedGaussianF32>> {
        Ok(
            ((self.inner.as_ref().ok_or_else(consumed)?).emission(state))
                .map(|value| PreparedGaussianF32 { inner: Some(value) }),
        )
    }
}
