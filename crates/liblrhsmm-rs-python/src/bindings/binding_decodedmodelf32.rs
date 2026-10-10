use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct DecodedModelF32 {
    pub(crate) inner: Option<crate::api::DecodedModelF32>,
}
#[pymethods]
impl DecodedModelF32 {
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
#[cfg(feature = "serialization")]
#[pymethods]
impl DecodedModelF32 {
    #[getter(position)]
    pub fn binding_position(&self) -> pyo3::PyResult<u64> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).position())
    }
    #[pyo3(name = "parameters")]
    #[pyo3(signature = ())]
    pub fn binding_parameters(&self) -> pyo3::PyResult<ModelF32> {
        Ok(ModelF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).parameters()),
        })
    }
    #[pyo3(name = "into_parameters")]
    #[pyo3(signature = ())]
    pub fn binding_into_parameters(&mut self) -> pyo3::PyResult<ModelF32> {
        Ok(ModelF32 {
            inner: Some((self.inner.take().ok_or_else(consumed)?).into_parameters()),
        })
    }
}
