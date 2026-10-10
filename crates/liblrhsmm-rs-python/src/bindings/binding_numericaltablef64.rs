use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct NumericalTableF64 {
    pub(crate) inner: Option<crate::api::NumericalTableF64>,
}
#[pymethods]
impl NumericalTableF64 {
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
impl NumericalTableF64 {
    #[new]
    #[pyo3(signature = (code))]
    pub fn binding_new(code: u32) -> pyo3::PyResult<Self> {
        (crate::api::NumericalTableF64::new(code))
            .map(|value| NumericalTableF64 { inner: Some(value) })
    }
    #[getter(length)]
    pub fn binding_length(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).length())
    }
    #[pyo3(name = "is_empty")]
    #[pyo3(signature = ())]
    pub fn binding_is_empty(&self) -> pyo3::PyResult<bool> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).is_empty())
    }
    #[pyo3(name = "get")]
    #[pyo3(signature = (index))]
    pub fn binding_get(&self, index: usize) -> pyo3::PyResult<Option<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).get(index))
    }
    #[pyo3(name = "replace")]
    #[pyo3(signature = (index, value))]
    pub fn binding_replace(&self, index: usize, value: f64) -> pyo3::PyResult<Option<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).replace(index, value))
    }
    #[pyo3(name = "get_storage")]
    #[pyo3(signature = (index))]
    pub fn binding_get_storage(&self, index: usize) -> pyo3::PyResult<Vec<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).get_storage(index))
    }
    #[pyo3(name = "replace_storage")]
    #[pyo3(signature = (index, values))]
    pub fn binding_replace_storage(
        &self,
        index: usize,
        values: Vec<f64>,
    ) -> pyo3::PyResult<Vec<f64>> {
        (self.inner.as_ref().ok_or_else(consumed)?).replace_storage(index, &values)
    }
    #[pyo3(name = "iter")]
    #[pyo3(signature = ())]
    pub fn binding_iter(&self) -> pyo3::PyResult<NumericalIteratorF64> {
        Ok(NumericalIteratorF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).iter()),
        })
    }
    #[pyo3(name = "values")]
    #[pyo3(signature = ())]
    pub fn binding_values(&self) -> pyo3::PyResult<Vec<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).values())
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(NumericalTableF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
