use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct DurationStatisticsF32 {
    pub(crate) inner: Option<crate::api::DurationStatisticsF32>,
}
#[pymethods]
impl DurationStatisticsF32 {
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
impl DurationStatisticsF32 {
    #[getter(value_sum)]
    fn binding_get_value_sum(&self) -> PyResult<f32> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.value_sum)
    }
    #[setter(value_sum)]
    fn binding_set_value_sum(&mut self, value: f32) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.value_sum = value;
        Ok(())
    }
    #[getter(squared_sum)]
    fn binding_get_squared_sum(&self) -> PyResult<f32> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.squared_sum)
    }
    #[setter(squared_sum)]
    fn binding_set_squared_sum(&mut self, value: f32) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.squared_sum = value;
        Ok(())
    }
    #[getter(occupancy)]
    fn binding_get_occupancy(&self) -> PyResult<f32> {
        Ok(self.inner.as_ref().ok_or_else(consumed)?.occupancy)
    }
    #[setter(occupancy)]
    fn binding_set_occupancy(&mut self, value: f32) -> PyResult<()> {
        self.inner.as_mut().ok_or_else(consumed)?.occupancy = value;
        Ok(())
    }
}
#[pymethods]
impl DurationStatisticsF32 {
    #[new]
    #[pyo3(signature = ())]
    pub fn binding_new() -> pyo3::PyResult<Self> {
        Ok(DurationStatisticsF32 {
            inner: Some(crate::api::DurationStatisticsF32::new()),
        })
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(DurationStatisticsF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
    #[pyo3(name = "clear")]
    #[pyo3(signature = ())]
    pub fn binding_clear(&mut self) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).clear();
        Ok(())
    }
    #[pyo3(name = "values")]
    #[pyo3(signature = ())]
    pub fn binding_values(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).values())
    }
    #[pyo3(name = "set_values")]
    #[pyo3(signature = (values))]
    pub fn binding_set_values(&mut self, values: Vec<f32>) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).set_values(&values)
    }
}
