use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct StateOccupanciesF32 {
    pub(crate) inner: Option<crate::api::StateOccupanciesF32>,
}
#[pymethods]
impl StateOccupanciesF32 {
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
impl StateOccupanciesF32 {
    #[new]
    #[pyo3(signature = (frames, states, values))]
    pub fn binding_new(frames: usize, states: usize, values: Vec<f32>) -> pyo3::PyResult<Self> {
        (crate::api::StateOccupanciesF32::new(frames, states, &values))
            .map(|value| StateOccupanciesF32 { inner: Some(value) })
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
    pub fn binding_values(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).values())
    }
    #[pyo3(name = "get")]
    #[pyo3(signature = (frame, state))]
    pub fn binding_get(&self, frame: usize, state: usize) -> pyo3::PyResult<Option<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).get(frame, state))
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(StateOccupanciesF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
