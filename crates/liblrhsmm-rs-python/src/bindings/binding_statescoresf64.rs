use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct StateScoresF64 {
    pub(crate) inner: Option<crate::api::StateScoresF64>,
}
#[pymethods]
impl StateScoresF64 {
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
impl StateScoresF64 {
    #[new]
    #[pyo3(signature = (frames, states, values))]
    pub fn binding_new(frames: usize, states: usize, values: Vec<f64>) -> pyo3::PyResult<Self> {
        (crate::api::StateScoresF64::new(frames, states, &values))
            .map(|value| StateScoresF64 { inner: Some(value) })
    }
    #[getter(frames)]
    pub fn binding_frames(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).frames())
    }
    #[getter(states)]
    pub fn binding_states(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).states())
    }
    #[pyo3(name = "values")]
    #[pyo3(signature = ())]
    pub fn binding_values(&self) -> pyo3::PyResult<Vec<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).values())
    }
    #[pyo3(name = "get")]
    #[pyo3(signature = (frame, state))]
    pub fn binding_get(&self, frame: usize, state: usize) -> pyo3::PyResult<Option<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).get(frame, state))
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(StateScoresF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
    #[pyo3(name = "total_log_probability")]
    #[pyo3(signature = ())]
    pub fn binding_total_log_probability(&self) -> pyo3::PyResult<f64> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).total_log_probability())
    }
    #[pyo3(name = "total_through_state")]
    #[pyo3(signature = (backward, state))]
    pub fn binding_total_through_state(
        &self,
        backward: pyo3::PyRef<'_, StateScoresF64>,
        state: usize,
    ) -> pyo3::PyResult<f64> {
        (self.inner.as_ref().ok_or_else(consumed)?)
            .total_through_state(backward.inner.as_ref().ok_or_else(consumed)?, state)
    }
    #[pyo3(name = "backward_log_probability")]
    #[pyo3(signature = (emissions))]
    pub fn binding_backward_log_probability(
        &self,
        emissions: pyo3::PyRef<'_, LogEmissionsF64>,
    ) -> pyo3::PyResult<f64> {
        (self.inner.as_ref().ok_or_else(consumed)?)
            .backward_log_probability(emissions.inner.as_ref().ok_or_else(consumed)?)
    }
}
#[pymethods]
impl StateScoresF64 {
    #[pyo3(name = "geometric_occupancies")]
    #[pyo3(signature = (backward, total, pruning_slope))]
    pub fn binding_geometric_occupancies(
        &self,
        backward: pyo3::PyRef<'_, StateScoresF64>,
        total: f64,
        pruning_slope: f64,
    ) -> pyo3::PyResult<StateOccupanciesF64> {
        ((self.inner.as_ref().ok_or_else(consumed)?).geometric_occupancies(
            backward.inner.as_ref().ok_or_else(consumed)?,
            total,
            pruning_slope,
        ))
        .map(|value| StateOccupanciesF64 { inner: Some(value) })
    }
}
