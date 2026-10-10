use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct MemoryPool {
    pub(crate) inner: Option<crate::api::MemoryPool>,
}
#[pymethods]
impl MemoryPool {
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
impl MemoryPool {
    #[new]
    #[pyo3(signature = (page_size))]
    pub fn binding_new(page_size: usize) -> pyo3::PyResult<Self> {
        (crate::api::MemoryPool::new(page_size)).map(|value| MemoryPool { inner: Some(value) })
    }
    #[getter(page_size)]
    pub fn binding_page_size(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).page_size())
    }
    #[getter(page_count)]
    pub fn binding_page_count(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).page_count())
    }
    #[getter(total_capacity)]
    pub fn binding_total_capacity(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).total_capacity())
    }
    #[getter(allocated_bytes)]
    pub fn binding_allocated_bytes(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).allocated_bytes())
    }
    #[pyo3(name = "pages")]
    #[pyo3(signature = ())]
    pub fn binding_pages(&self) -> pyo3::PyResult<PoolPages> {
        Ok(PoolPages {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).pages()),
        })
    }
    #[pyo3(name = "page")]
    #[pyo3(signature = (index))]
    pub fn binding_page(&self, index: usize) -> pyo3::PyResult<Option<PoolPageUsage>> {
        Ok(((self.inner.as_ref().ok_or_else(consumed)?).page(index))
            .map(|value| PoolPageUsage { inner: Some(value) }))
    }
    #[pyo3(name = "allocate")]
    #[pyo3(signature = (size))]
    pub fn binding_allocate(&mut self, size: usize) -> pyo3::PyResult<PoolAllocation> {
        ((self.inner.as_mut().ok_or_else(consumed)?).allocate(size))
            .map(|value| PoolAllocation { inner: Some(value) })
    }
    #[pyo3(name = "allocate_zeroed")]
    #[pyo3(signature = (number, size))]
    pub fn binding_allocate_zeroed(
        &mut self,
        number: usize,
        size: usize,
    ) -> pyo3::PyResult<PoolAllocation> {
        ((self.inner.as_mut().ok_or_else(consumed)?).allocate_zeroed(number, size))
            .map(|value| PoolAllocation { inner: Some(value) })
    }
    #[pyo3(name = "get")]
    #[pyo3(signature = (allocation))]
    pub fn binding_get(
        &self,
        allocation: pyo3::PyRef<'_, PoolAllocation>,
    ) -> pyo3::PyResult<Option<pyo3::Py<pyo3::types::PyBytes>>> {
        Ok(((self.inner.as_ref().ok_or_else(consumed)?)
            .get(allocation.inner.as_ref().ok_or_else(consumed)?))
        .map(|value| {
            let bytes = value;
            pyo3::Python::attach(|py| pyo3::types::PyBytes::new(py, &bytes).unbind())
        }))
    }
    #[pyo3(name = "get_range")]
    #[pyo3(signature = (allocation, offset, length))]
    pub fn binding_get_range(
        &self,
        allocation: pyo3::PyRef<'_, PoolAllocation>,
        offset: usize,
        length: usize,
    ) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyBytes>> {
        ((self.inner.as_ref().ok_or_else(consumed)?).get_range(
            allocation.inner.as_ref().ok_or_else(consumed)?,
            offset,
            length,
        ))
        .map(|value| {
            let bytes = value;
            pyo3::Python::attach(|py| pyo3::types::PyBytes::new(py, &bytes).unbind())
        })
    }
    #[pyo3(name = "set_bytes")]
    #[pyo3(signature = (allocation, offset, values))]
    pub fn binding_set_bytes(
        &mut self,
        allocation: pyo3::PyRef<'_, PoolAllocation>,
        offset: usize,
        values: Vec<u8>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).set_bytes(
            allocation.inner.as_ref().ok_or_else(consumed)?,
            offset,
            &values,
        )
    }
}
