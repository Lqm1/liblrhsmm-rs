use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct TransitionGraphF32 {
    pub(crate) inner: Option<crate::api::TransitionGraphF32>,
}
#[pymethods]
impl TransitionGraphF32 {
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
impl TransitionGraphF32 {
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(TransitionGraphF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
    #[pyo3(name = "segmentation")]
    #[pyo3(signature = ())]
    pub fn binding_segmentation(&self) -> pyo3::PyResult<SegmentationF32> {
        Ok(SegmentationF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).segmentation()),
        })
    }
    #[pyo3(name = "into_segmentation")]
    #[pyo3(signature = ())]
    pub fn binding_into_segmentation(&mut self) -> pyo3::PyResult<SegmentationF32> {
        Ok(SegmentationF32 {
            inner: Some((self.inner.take().ok_or_else(consumed)?).into_segmentation()),
        })
    }
    #[pyo3(name = "incoming")]
    #[pyo3(signature = (state))]
    pub fn binding_incoming(&self, state: usize) -> pyo3::PyResult<Option<JumpGroupF32>> {
        Ok(
            ((self.inner.as_ref().ok_or_else(consumed)?).incoming(state))
                .map(|value| JumpGroupF32 { inner: Some(value) }),
        )
    }
    #[pyo3(name = "outgoing")]
    #[pyo3(signature = (state))]
    pub fn binding_outgoing(&self, state: usize) -> pyo3::PyResult<Option<JumpGroupF32>> {
        Ok(
            ((self.inner.as_ref().ok_or_else(consumed)?).outgoing(state))
                .map(|value| JumpGroupF32 { inner: Some(value) }),
        )
    }
}
