use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct HsmmPathF32 {
    pub(crate) inner: Option<crate::api::HsmmPathF32>,
}
#[pymethods]
impl HsmmPathF32 {
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
impl HsmmPathF32 {
    #[new]
    #[pyo3(signature = (occurrences, log_probability, scores))]
    pub fn binding_new(
        occurrences: Vec<u32>,
        log_probability: Vec<f32>,
        scores: pyo3::PyRef<'_, HsmmScoresF32>,
    ) -> pyo3::PyResult<Self> {
        (crate::api::HsmmPathF32::new(
            &occurrences,
            &log_probability,
            scores.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| HsmmPathF32 { inner: Some(value) })
    }
    #[pyo3(name = "occurrences")]
    #[pyo3(signature = ())]
    pub fn binding_occurrences(&self) -> pyo3::PyResult<Vec<u32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).occurrences())
    }
    #[pyo3(name = "set_occurrences")]
    #[pyo3(signature = (values))]
    pub fn binding_set_occurrences(&mut self, values: Vec<u32>) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).set_occurrences(&values)
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
    pub fn binding_scores(&self) -> pyo3::PyResult<HsmmScoresF32> {
        Ok(HsmmScoresF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).scores()),
        })
    }
    #[pyo3(name = "set_scores")]
    #[pyo3(signature = (value))]
    pub fn binding_set_scores(
        &mut self,
        value: pyo3::PyRef<'_, HsmmScoresF32>,
    ) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?)
            .set_scores(value.inner.as_ref().ok_or_else(consumed)?);
        Ok(())
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(HsmmPathF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
