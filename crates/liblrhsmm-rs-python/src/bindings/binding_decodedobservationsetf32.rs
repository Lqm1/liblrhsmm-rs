use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct DecodedObservationSetF32 {
    pub(crate) inner: Option<crate::api::DecodedObservationSetF32>,
}
#[pymethods]
impl DecodedObservationSetF32 {
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
impl DecodedObservationSetF32 {
    #[getter(position)]
    pub fn binding_position(&self) -> pyo3::PyResult<u64> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).position())
    }
    #[pyo3(name = "samples")]
    #[pyo3(signature = ())]
    pub fn binding_samples(&self) -> pyo3::PyResult<ObservationCollectionF32> {
        Ok(ObservationCollectionF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).samples()),
        })
    }
    #[pyo3(name = "into_samples")]
    #[pyo3(signature = ())]
    pub fn binding_into_samples(&mut self) -> pyo3::PyResult<ObservationCollectionF32> {
        Ok(ObservationCollectionF32 {
            inner: Some((self.inner.take().ok_or_else(consumed)?).into_samples()),
        })
    }
}
