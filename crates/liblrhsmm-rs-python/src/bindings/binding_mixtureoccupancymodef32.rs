use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct MixtureOccupancyModeF32 {
    pub(crate) inner: Option<crate::api::MixtureOccupancyModeF32>,
}
#[pymethods]
impl MixtureOccupancyModeF32 {
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
impl MixtureOccupancyModeF32 {
    #[getter(mode)]
    fn binding_get_mode(&self) -> PyResult<u32> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.mode)
    }
    #[setter(mode)]
    fn binding_set_mode(&mut self, value: u32) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.mode = value;
        Ok(())
    }
    #[getter(parameter)]
    fn binding_get_parameter(&self) -> PyResult<f32> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.parameter)
    }
    #[setter(parameter)]
    fn binding_set_parameter(&mut self, value: f32) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.parameter = value;
        Ok(())
    }
}
#[pymethods]
impl MixtureOccupancyModeF32 {
    #[new]
    #[pyo3(signature = (mode, parameter))]
    pub fn binding_new(mode: u32, parameter: f32) -> pyo3::PyResult<Self> {
        Ok(MixtureOccupancyModeF32 {
            inner: Some(crate::api::MixtureOccupancyModeF32::new(mode, parameter)),
        })
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(MixtureOccupancyModeF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
