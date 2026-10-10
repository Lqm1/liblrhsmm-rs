use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct PreparedDurationF32 {
    pub(crate) inner: Option<crate::api::PreparedDurationF32>,
}
#[pymethods]
impl PreparedDurationF32 {
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
impl PreparedDurationF32 {
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(PreparedDurationF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
    #[pyo3(name = "parameters")]
    #[pyo3(signature = ())]
    pub fn binding_parameters(&self) -> pyo3::PyResult<DurationF32> {
        Ok(DurationF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).parameters()),
        })
    }
    #[pyo3(name = "into_parameters")]
    #[pyo3(signature = ())]
    pub fn binding_into_parameters(&mut self) -> pyo3::PyResult<DurationF32> {
        Ok(DurationF32 {
            inner: Some((self.inner.take().ok_or_else(consumed)?).into_parameters()),
        })
    }
    #[pyo3(name = "options")]
    #[pyo3(signature = ())]
    pub fn binding_options(&self) -> pyo3::PyResult<PreparationOptionsF32> {
        Ok(PreparationOptionsF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).options()),
        })
    }
    #[pyo3(name = "log_normalizer")]
    #[pyo3(signature = ())]
    pub fn binding_log_normalizer(&self) -> pyo3::PyResult<f32> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).log_normalizer())
    }
    #[pyo3(name = "cached_log_probabilities")]
    #[pyo3(signature = ())]
    pub fn binding_cached_log_probabilities(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).cached_log_probabilities())
    }
    #[pyo3(name = "log_probability")]
    #[pyo3(signature = (frames))]
    pub fn binding_log_probability(&self, frames: usize) -> pyo3::PyResult<f32> {
        (self.inner.as_ref().ok_or_else(consumed)?).log_probability(frames)
    }
    #[pyo3(name = "probability")]
    #[pyo3(signature = (frames))]
    pub fn binding_probability(&self, frames: usize) -> pyo3::PyResult<f64> {
        (self.inner.as_ref().ok_or_else(consumed)?).probability(frames)
    }
}
