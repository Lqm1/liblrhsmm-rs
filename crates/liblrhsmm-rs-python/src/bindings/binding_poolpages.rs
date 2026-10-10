use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct PoolPages {
    pub(crate) inner: Option<crate::api::PoolPages>,
}
#[pymethods]
impl PoolPages {
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
impl PoolPages {
    #[getter(length)]
    pub fn binding_length(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).length())
    }
    #[pyo3(name = "is_empty")]
    #[pyo3(signature = ())]
    pub fn binding_is_empty(&self) -> pyo3::PyResult<bool> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).is_empty())
    }
    #[pyo3(name = "next_value")]
    #[pyo3(signature = ())]
    pub fn binding_next_value(&mut self) -> pyo3::PyResult<Option<PoolPageUsage>> {
        Ok(((self.inner.as_mut().ok_or_else(consumed)?).next_value())
            .map(|value| PoolPageUsage { inner: Some(value) }))
    }
    #[pyo3(name = "next_back_value")]
    #[pyo3(signature = ())]
    pub fn binding_next_back_value(&mut self) -> pyo3::PyResult<Option<PoolPageUsage>> {
        Ok(
            ((self.inner.as_mut().ok_or_else(consumed)?).next_back_value())
                .map(|value| PoolPageUsage { inner: Some(value) }),
        )
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(PoolPages {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
