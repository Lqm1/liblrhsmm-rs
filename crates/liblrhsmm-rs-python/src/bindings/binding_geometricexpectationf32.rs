use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct GeometricExpectationF32 {
    pub(crate) inner: Option<crate::api::GeometricExpectationF32>,
}
#[pymethods]
impl GeometricExpectationF32 {
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
impl GeometricExpectationF32 {
    #[getter(pruning_slope)]
    fn binding_get_pruning_slope(&self) -> PyResult<f32> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.pruning_slope)
    }
    #[setter(pruning_slope)]
    fn binding_set_pruning_slope(&mut self, value: f32) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.pruning_slope = value;
        Ok(())
    }
}
#[pymethods]
impl GeometricExpectationF32 {
    #[new]
    #[pyo3(signature = (transitions, states, mixtures, pruning_slope))]
    pub fn binding_new(
        transitions: pyo3::PyRef<'_, TransitionPosteriors>,
        states: pyo3::PyRef<'_, StateOccupanciesF32>,
        mixtures: pyo3::PyRef<'_, MixturePosteriorCollection>,
        pruning_slope: f32,
    ) -> pyo3::PyResult<Self> {
        Ok(GeometricExpectationF32 {
            inner: Some(crate::api::GeometricExpectationF32::new(
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
    pub fn binding_states(&self) -> pyo3::PyResult<StateOccupanciesF32> {
        Ok(StateOccupanciesF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).states()),
        })
    }
    #[pyo3(name = "set_states")]
    #[pyo3(signature = (value))]
    pub fn binding_set_states(
        &mut self,
        value: pyo3::PyRef<'_, StateOccupanciesF32>,
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
        Ok(GeometricExpectationF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
