use super::*;
#[pyclass(unsendable, module = "liblrhsmm_rs")]
pub struct GeneratedObservationF64 {
    pub(crate) inner: Option<crate::api::GeneratedObservationF64>,
}
#[pymethods]
impl GeneratedObservationF64 {
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
impl GeneratedObservationF64 {
    #[getter(draws_used)]
    pub fn binding_draws_used(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).draws_used())
    }
    #[getter(random_words_used)]
    pub fn binding_random_words_used(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).random_words_used())
    }
    #[pyo3(name = "observation")]
    #[pyo3(signature = ())]
    pub fn binding_observation(&self) -> pyo3::PyResult<ObservationF64> {
        Ok(ObservationF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).observation()),
        })
    }
    #[pyo3(name = "into_observation")]
    #[pyo3(signature = ())]
    pub fn binding_into_observation(&mut self) -> pyo3::PyResult<ObservationF64> {
        Ok(ObservationF64 {
            inner: Some((self.inner.take().ok_or_else(consumed)?).into_observation()),
        })
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(GeneratedObservationF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
