use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct GeometricPathF32 {
    pub(crate) inner: Option<crate::api::GeometricPathF32>,
}
#[pymethods]
impl GeometricPathF32 {
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
impl GeometricPathF32 {
    #[new]
    #[pyo3(signature = (states, boundaries, log_probability, scores))]
    pub fn binding_new(
        states: Vec<u32>,
        boundaries: Vec<u32>,
        log_probability: Vec<f32>,
        scores: pyo3::PyRef<'_, StateScoresF32>,
    ) -> pyo3::PyResult<Self> {
        (crate::api::GeometricPathF32::new(
            &states,
            &boundaries,
            &log_probability,
            scores.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| GeometricPathF32 { inner: Some(value) })
    }
    #[pyo3(name = "states")]
    #[pyo3(signature = ())]
    pub fn binding_states(&self) -> pyo3::PyResult<Vec<u32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).states())
    }
    #[pyo3(name = "set_states")]
    #[pyo3(signature = (values))]
    pub fn binding_set_states(&mut self, values: Vec<u32>) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_states(&values);
        Ok(())
    }
    #[pyo3(name = "boundaries")]
    #[pyo3(signature = ())]
    pub fn binding_boundaries(&self) -> pyo3::PyResult<Vec<u32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).boundaries())
    }
    #[pyo3(name = "set_boundaries")]
    #[pyo3(signature = (values))]
    pub fn binding_set_boundaries(&mut self, values: Vec<u32>) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).set_boundaries(&values);
        Ok(())
    }
    #[pyo3(name = "log_probability")]
    #[pyo3(signature = ())]
    pub fn binding_log_probability(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).log_probability())
    }
    #[pyo3(name = "set_log_probability")]
    #[pyo3(signature = (values))]
    pub fn binding_set_log_probability(&mut self, values: Vec<f32>) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).set_log_probability(&values)
    }
    #[pyo3(name = "scores")]
    #[pyo3(signature = ())]
    pub fn binding_scores(&self) -> pyo3::PyResult<StateScoresF32> {
        Ok(StateScoresF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).scores()),
        })
    }
    #[pyo3(name = "set_scores")]
    #[pyo3(signature = (value))]
    pub fn binding_set_scores(
        &mut self,
        value: pyo3::PyRef<'_, StateScoresF32>,
    ) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?)
            .set_scores(value.inner.as_ref().ok_or_else(consumed)?);
        Ok(())
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(GeometricPathF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
