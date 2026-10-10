use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct TransitionPosteriors {
    pub(crate) inner: Option<crate::api::TransitionPosteriors>,
}
#[pymethods]
impl TransitionPosteriors {
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
impl TransitionPosteriors {
    #[new]
    #[pyo3(signature = (frames, states, presence, values))]
    pub fn binding_new(
        frames: usize,
        states: usize,
        presence: Vec<u8>,
        values: Vec<f32>,
    ) -> pyo3::PyResult<Self> {
        (crate::api::TransitionPosteriors::new(frames, states, &presence, &values))
            .map(|value| TransitionPosteriors { inner: Some(value) })
    }
    #[getter(frames)]
    pub fn binding_frames(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).frames())
    }
    #[getter(states)]
    pub fn binding_states(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).states())
    }
    #[pyo3(name = "presence")]
    #[pyo3(signature = ())]
    pub fn binding_presence(&self) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyBytes>> {
        Ok({
            let bytes = (self.inner.as_ref().ok_or_else(consumed)?).presence();
            pyo3::Python::attach(|py| pyo3::types::PyBytes::new(py, &bytes).unbind())
        })
    }
    #[pyo3(name = "values")]
    #[pyo3(signature = ())]
    pub fn binding_values(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).values())
    }
    #[pyo3(name = "get")]
    #[pyo3(signature = (frame, state))]
    pub fn binding_get(
        &self,
        frame: usize,
        state: usize,
    ) -> pyo3::PyResult<Option<TransitionPosterior>> {
        Ok(
            ((self.inner.as_ref().ok_or_else(consumed)?).get(frame, state))
                .map(|value| TransitionPosterior { inner: Some(value) }),
        )
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(TransitionPosteriors {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
