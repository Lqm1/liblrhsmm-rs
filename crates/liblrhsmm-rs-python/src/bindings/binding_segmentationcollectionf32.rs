use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct SegmentationCollectionF32 {
    pub(crate) inner: Option<crate::api::SegmentationCollectionF32>,
}
#[pymethods]
impl SegmentationCollectionF32 {
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
impl SegmentationCollectionF32 {
    #[new]
    #[pyo3(signature = ())]
    pub fn binding_new() -> pyo3::PyResult<Self> {
        Ok(SegmentationCollectionF32 {
            inner: Some(crate::api::SegmentationCollectionF32::new()),
        })
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
    #[pyo3(name = "push")]
    #[pyo3(signature = (value))]
    pub fn binding_push(&mut self, value: pyo3::PyRef<'_, SegmentationF32>) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?)
            .push(value.inner.as_ref().ok_or_else(consumed)?);
        Ok(())
    }
    #[pyo3(name = "get")]
    #[pyo3(signature = (index))]
    pub fn binding_get(&self, index: usize) -> pyo3::PyResult<Option<SegmentationF32>> {
        Ok(((self.inner.as_ref().ok_or_else(consumed)?).get(index))
            .map(|value| SegmentationF32 { inner: Some(value) }))
    }
    #[pyo3(name = "replace")]
    #[pyo3(signature = (index, value))]
    pub fn binding_replace(
        &mut self,
        index: usize,
        value: pyo3::PyRef<'_, SegmentationF32>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?)
            .replace(index, value.inner.as_ref().ok_or_else(consumed)?)
    }
    #[pyo3(name = "clear")]
    #[pyo3(signature = ())]
    pub fn binding_clear(&mut self) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).clear();
        Ok(())
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(SegmentationCollectionF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
#[cfg(feature = "serialization")]
#[pymethods]
impl SegmentationCollectionF32 {
    #[staticmethod]
    #[pyo3(name = "read")]
    #[pyo3(signature = (bytes))]
    pub fn binding_read(bytes: Vec<u8>) -> pyo3::PyResult<Self> {
        (crate::api::SegmentationCollectionF32::read(&bytes))
            .map(|value| SegmentationCollectionF32 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "read_with_limits")]
    #[pyo3(signature = (bytes, max_array_entries))]
    pub fn binding_read_with_limits(
        bytes: Vec<u8>,
        max_array_entries: usize,
    ) -> pyo3::PyResult<Self> {
        (crate::api::SegmentationCollectionF32::read_with_limits(&bytes, max_array_entries))
            .map(|value| SegmentationCollectionF32 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "read_prefix")]
    #[pyo3(signature = (bytes))]
    pub fn binding_read_prefix(bytes: Vec<u8>) -> pyo3::PyResult<DecodedSegmentationSetF32> {
        (crate::api::SegmentationCollectionF32::read_prefix(&bytes))
            .map(|value| DecodedSegmentationSetF32 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "read_prefix_with_limits")]
    #[pyo3(signature = (bytes, max_array_entries))]
    pub fn binding_read_prefix_with_limits(
        bytes: Vec<u8>,
        max_array_entries: usize,
    ) -> pyo3::PyResult<DecodedSegmentationSetF32> {
        (crate::api::SegmentationCollectionF32::read_prefix_with_limits(&bytes, max_array_entries))
            .map(|value| DecodedSegmentationSetF32 { inner: Some(value) })
    }
    #[pyo3(name = "write")]
    #[pyo3(signature = ())]
    pub fn binding_write(&self) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyBytes>> {
        ((self.inner.as_ref().ok_or_else(consumed)?).write()).map(|value| {
            let bytes = value;
            pyo3::Python::attach(|py| pyo3::types::PyBytes::new(py, &bytes).unbind())
        })
    }
}
