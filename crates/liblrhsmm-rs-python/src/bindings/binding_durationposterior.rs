use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct DurationPosterior {
    pub(crate) inner: Option<crate::api::DurationPosterior>,
}
#[pymethods]
impl DurationPosterior {
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
impl DurationPosterior {
    #[new]
    #[pyo3(signature = (minimum, probabilities))]
    pub fn binding_new(minimum: usize, probabilities: Vec<f32>) -> pyo3::PyResult<Self> {
        Ok(DurationPosterior {
            inner: Some(crate::api::DurationPosterior::new(minimum, &probabilities)),
        })
    }
    #[getter(minimum)]
    pub fn binding_minimum(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).minimum())
    }
    #[setter(minimum)]
    pub fn binding_set_minimum(&mut self, value: usize) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_minimum(value);
        Ok(())
    }
    #[pyo3(name = "probabilities")]
    #[pyo3(signature = ())]
    pub fn binding_probabilities(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).probabilities())
    }
    #[pyo3(name = "set_probabilities")]
    #[pyo3(signature = (values))]
    pub fn binding_set_probabilities(&mut self, values: Vec<f32>) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_probabilities(&values);
        Ok(())
    }
    #[pyo3(name = "probability")]
    #[pyo3(signature = (duration))]
    pub fn binding_probability(&self, duration: usize) -> pyo3::PyResult<f32> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).probability(duration))
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(DurationPosterior {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
