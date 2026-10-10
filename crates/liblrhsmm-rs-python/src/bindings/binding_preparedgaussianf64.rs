use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct PreparedGaussianF64 {
    pub(crate) inner: Option<crate::api::PreparedGaussianF64>,
}
#[pymethods]
impl PreparedGaussianF64 {
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
impl PreparedGaussianF64 {
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(PreparedGaussianF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
    #[pyo3(name = "parameters")]
    #[pyo3(signature = ())]
    pub fn binding_parameters(&self) -> pyo3::PyResult<GaussianF64> {
        Ok(GaussianF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).parameters()),
        })
    }
    #[pyo3(name = "into_parameters")]
    #[pyo3(signature = ())]
    pub fn binding_into_parameters(&mut self) -> pyo3::PyResult<GaussianF64> {
        Ok(GaussianF64 {
            inner: Some((self.inner.take().ok_or_else(consumed)?).into_parameters()),
        })
    }
    #[pyo3(name = "cached_log_normalizers")]
    #[pyo3(signature = ())]
    pub fn binding_cached_log_normalizers(&self) -> pyo3::PyResult<Vec<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).cached_log_normalizers())
    }
    #[pyo3(name = "component_log_probability")]
    #[pyo3(signature = (observation, component))]
    pub fn binding_component_log_probability(
        &self,
        observation: Vec<f64>,
        component: usize,
    ) -> pyo3::PyResult<f64> {
        (self.inner.as_ref().ok_or_else(consumed)?)
            .component_log_probability(&observation, component)
    }
    #[pyo3(name = "component_probability")]
    #[pyo3(signature = (observation, component))]
    pub fn binding_component_probability(
        &self,
        observation: Vec<f64>,
        component: usize,
    ) -> pyo3::PyResult<f64> {
        (self.inner.as_ref().ok_or_else(consumed)?).component_probability(&observation, component)
    }
    #[pyo3(name = "log_probability")]
    #[pyo3(signature = (observation, temperature))]
    pub fn binding_log_probability(
        &self,
        observation: Vec<f64>,
        temperature: f64,
    ) -> pyo3::PyResult<f64> {
        (self.inner.as_ref().ok_or_else(consumed)?).log_probability(&observation, temperature)
    }
    #[pyo3(name = "probability")]
    #[pyo3(signature = (observation, temperature))]
    pub fn binding_probability(
        &self,
        observation: Vec<f64>,
        temperature: f64,
    ) -> pyo3::PyResult<f64> {
        (self.inner.as_ref().ok_or_else(consumed)?).probability(&observation, temperature)
    }
}
