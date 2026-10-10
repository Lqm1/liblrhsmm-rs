use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct NumericalIteratorF64 {
    pub(crate) inner: Option<crate::api::NumericalIteratorF64>,
}
#[pymethods]
impl NumericalIteratorF64 {
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
impl NumericalIteratorF64 {
    #[getter(length)]
    pub fn binding_length(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).length())
    }
    #[pyo3(name = "next_value")]
    #[pyo3(signature = ())]
    pub fn binding_next_value(&mut self) -> pyo3::PyResult<Option<f64>> {
        Ok((self.inner.as_mut().ok_or_else(consumed)?).next_value())
    }
    #[pyo3(name = "next_back_value")]
    #[pyo3(signature = ())]
    pub fn binding_next_back_value(&mut self) -> pyo3::PyResult<Option<f64>> {
        Ok((self.inner.as_mut().ok_or_else(consumed)?).next_back_value())
    }
    #[pyo3(name = "next_storage")]
    #[pyo3(signature = ())]
    pub fn binding_next_storage(&mut self) -> pyo3::PyResult<Vec<f64>> {
        Ok((self.inner.as_mut().ok_or_else(consumed)?).next_storage())
    }
    #[pyo3(name = "next_back_storage")]
    #[pyo3(signature = ())]
    pub fn binding_next_back_storage(&mut self) -> pyo3::PyResult<Vec<f64>> {
        Ok((self.inner.as_mut().ok_or_else(consumed)?).next_back_storage())
    }
}
