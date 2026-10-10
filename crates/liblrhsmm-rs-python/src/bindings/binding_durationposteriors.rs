use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct DurationPosteriors {
    pub(crate) inner: Option<crate::api::DurationPosteriors>,
}
#[pymethods]
impl DurationPosteriors {
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
impl DurationPosteriors {
    #[new]
    #[pyo3(signature = (frames, states, presence, minimums, offsets, values))]
    pub fn binding_new(
        frames: usize,
        states: usize,
        presence: Vec<u8>,
        minimums: Vec<u32>,
        offsets: Vec<u32>,
        values: Vec<f32>,
    ) -> pyo3::PyResult<Self> {
        (crate::api::DurationPosteriors::new(
            frames, states, &presence, &minimums, &offsets, &values,
        ))
        .map(|value| DurationPosteriors { inner: Some(value) })
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
    #[pyo3(name = "minimums")]
    #[pyo3(signature = ())]
    pub fn binding_minimums(&self) -> pyo3::PyResult<Vec<u32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).minimums())
    }
    #[pyo3(name = "offsets")]
    #[pyo3(signature = ())]
    pub fn binding_offsets(&self) -> pyo3::PyResult<Vec<u32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).offsets())
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
    ) -> pyo3::PyResult<Option<DurationPosterior>> {
        Ok(
            ((self.inner.as_ref().ok_or_else(consumed)?).get(frame, state))
                .map(|value| DurationPosterior { inner: Some(value) }),
        )
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(DurationPosteriors {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
