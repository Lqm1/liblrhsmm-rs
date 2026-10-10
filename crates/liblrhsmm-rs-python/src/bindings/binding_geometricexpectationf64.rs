use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct GeometricExpectationF64 {
    pub(crate) inner: Option<crate::api::GeometricExpectationF64>,
}
#[pymethods]
impl GeometricExpectationF64 {
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
impl GeometricExpectationF64 {
    #[getter(pruning_slope)]
    fn binding_get_pruning_slope(&self) -> PyResult<f64> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.pruning_slope)
    }
    #[setter(pruning_slope)]
    fn binding_set_pruning_slope(&mut self, value: f64) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.pruning_slope = value;
        Ok(())
    }
}
#[pymethods]
impl GeometricExpectationF64 {
    #[new]
    #[pyo3(signature = (transitions, states, mixtures, pruning_slope))]
    pub fn binding_new(
        transitions: pyo3::PyRef<'_, TransitionPosteriors>,
        states: pyo3::PyRef<'_, StateOccupanciesF64>,
        mixtures: pyo3::PyRef<'_, MixturePosteriorCollection>,
        pruning_slope: f64,
    ) -> pyo3::PyResult<Self> {
        Ok(GeometricExpectationF64 {
            inner: Some(crate::api::GeometricExpectationF64::new(
                transitions.inner.as_ref().ok_or_else(consumed)?,
                states.inner.as_ref().ok_or_else(consumed)?,
                mixtures.inner.as_ref().ok_or_else(consumed)?,
                pruning_slope,
            )),
        })
    }
    #[pyo3(name = "transitions")]
    #[pyo3(signature = ())]
    pub fn binding_transitions(&self) -> pyo3::PyResult<TransitionPosteriors> {
        Ok(TransitionPosteriors {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).transitions()),
        })
    }
    #[pyo3(name = "set_transitions")]
    #[pyo3(signature = (value))]
    pub fn binding_set_transitions(
        &mut self,
        value: pyo3::PyRef<'_, TransitionPosteriors>,
    ) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?)
            .set_transitions(value.inner.as_ref().ok_or_else(consumed)?);
        Ok(())
    }
    #[pyo3(name = "states")]
    #[pyo3(signature = ())]
    pub fn binding_states(&self) -> pyo3::PyResult<StateOccupanciesF64> {
        Ok(StateOccupanciesF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).states()),
        })
    }
    #[pyo3(name = "set_states")]
    #[pyo3(signature = (value))]
    pub fn binding_set_states(
        &mut self,
        value: pyo3::PyRef<'_, StateOccupanciesF64>,
    ) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?)
            .set_states(value.inner.as_ref().ok_or_else(consumed)?);
        Ok(())
    }
    #[pyo3(name = "mixtures")]
    #[pyo3(signature = ())]
    pub fn binding_mixtures(&self) -> pyo3::PyResult<MixturePosteriorCollection> {
        Ok(MixturePosteriorCollection {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).mixtures()),
        })
    }
    #[pyo3(name = "set_mixtures")]
    #[pyo3(signature = (value))]
    pub fn binding_set_mixtures(
        &mut self,
        value: pyo3::PyRef<'_, MixturePosteriorCollection>,
    ) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?)
            .set_mixtures(value.inner.as_ref().ok_or_else(consumed)?);
        Ok(())
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(GeometricExpectationF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
