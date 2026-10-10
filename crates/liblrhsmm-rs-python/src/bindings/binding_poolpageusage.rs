use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct PoolPageUsage {
    pub(crate) inner: Option<crate::api::PoolPageUsage>,
}
#[pymethods]
impl PoolPageUsage {
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
impl PoolPageUsage {
    #[getter(used_bytes)]
    fn binding_get_used_bytes(&self) -> PyResult<usize> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.used_bytes)
    }
    #[setter(used_bytes)]
    fn binding_set_used_bytes(&mut self, value: usize) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.used_bytes = value;
        Ok(())
    }
    #[getter(capacity)]
    fn binding_get_capacity(&self) -> PyResult<usize> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.capacity)
    }
    #[setter(capacity)]
    fn binding_set_capacity(&mut self, value: usize) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.capacity = value;
        Ok(())
    }
}
