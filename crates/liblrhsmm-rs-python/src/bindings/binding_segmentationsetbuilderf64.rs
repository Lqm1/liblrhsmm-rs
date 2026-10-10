use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct SegmentationSetBuilderF64 {
    pub(crate) inner: Option<crate::api::SegmentationSetBuilderF64>,
}
#[pymethods]
impl SegmentationSetBuilderF64 {
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
impl SegmentationSetBuilderF64 {
    #[new]
    #[pyo3(signature = (samples))]
    pub fn binding_new(samples: usize) -> pyo3::PyResult<Self> {
        (crate::api::SegmentationSetBuilderF64::new(samples))
            .map(|value| SegmentationSetBuilderF64 { inner: Some(value) })
    }
    #[getter(length)]
    pub fn binding_length(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).length())
    }
    #[pyo3(name = "has")]
    #[pyo3(signature = (index))]
    pub fn binding_has(&self, index: usize) -> pyo3::PyResult<bool> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).has(index))
    }
    #[pyo3(name = "get")]
    #[pyo3(signature = (index))]
    pub fn binding_get(&self, index: usize) -> pyo3::PyResult<Option<SegmentationF64>> {
        Ok(((self.inner.as_ref().ok_or_else(consumed)?).get(index))
            .map(|value| SegmentationF64 { inner: Some(value) }))
    }
    #[pyo3(name = "set")]
    #[pyo3(signature = (index, value))]
    pub fn binding_set(
        &mut self,
        index: usize,
        value: pyo3::PyRef<'_, SegmentationF64>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?)
            .set(index, value.inner.as_ref().ok_or_else(consumed)?)
    }
    #[pyo3(name = "clear")]
    #[pyo3(signature = (index))]
    pub fn binding_clear(&mut self, index: usize) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).clear(index)
    }
    #[pyo3(name = "resize")]
    #[pyo3(signature = (count))]
    pub fn binding_resize(&mut self, count: usize) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).resize(count)
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(SegmentationSetBuilderF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
    #[pyo3(name = "build")]
    #[pyo3(signature = ())]
    pub fn binding_build(&mut self) -> pyo3::PyResult<SegmentationCollectionF64> {
        ((self.inner.take().ok_or_else(consumed)?).build())
            .map(|value| SegmentationCollectionF64 { inner: Some(value) })
    }
}
