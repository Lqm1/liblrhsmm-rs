use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct HsmmExpectationF64 {
    pub(crate) inner: Option<crate::api::HsmmExpectationF64>,
}
#[pymethods]
impl HsmmExpectationF64 {
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
impl HsmmExpectationF64 {
    #[getter(state_radius)]
    fn binding_get_state_radius(&self) -> PyResult<f64> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.state_radius)
    }
    #[setter(state_radius)]
    fn binding_set_state_radius(&mut self, value: f64) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.state_radius = value;
        Ok(())
    }
}
#[pymethods]
impl HsmmExpectationF64 {
    #[new]
    #[pyo3(signature = (durations, states, mixtures, state_radius))]
    pub fn binding_new(
        durations: pyo3::PyRef<'_, DurationPosteriors>,
        states: pyo3::PyRef<'_, StateOccupanciesF64>,
        mixtures: pyo3::PyRef<'_, MixturePosteriorCollection>,
        state_radius: f64,
    ) -> pyo3::PyResult<Self> {
        Ok(HsmmExpectationF64 {
            inner: Some(crate::api::HsmmExpectationF64::new(
                durations.inner.as_ref().ok_or_else(consumed)?,
                states.inner.as_ref().ok_or_else(consumed)?,
                mixtures.inner.as_ref().ok_or_else(consumed)?,
                state_radius,
            )),
        })
    }
    #[pyo3(name = "durations")]
    #[pyo3(signature = ())]
    pub fn binding_durations(&self) -> pyo3::PyResult<DurationPosteriors> {
        Ok(DurationPosteriors {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).durations()),
        })
    }
    #[pyo3(name = "set_durations")]
    #[pyo3(signature = (value))]
    pub fn binding_set_durations(
        &mut self,
        value: pyo3::PyRef<'_, DurationPosteriors>,
    ) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?)
            .set_durations(value.inner.as_ref().ok_or_else(consumed)?);
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
        Ok(HsmmExpectationF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
