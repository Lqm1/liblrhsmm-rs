use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct ObservationStreamF32 {
    pub(crate) inner: Option<crate::api::ObservationStreamF32>,
}
#[pymethods]
impl ObservationStreamF32 {
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
impl ObservationStreamF32 {
    #[new]
    #[pyo3(signature = (dimensions, values))]
    pub fn binding_new(dimensions: usize, values: Vec<f32>) -> pyo3::PyResult<Self> {
        Ok(ObservationStreamF32 {
            inner: Some(crate::api::ObservationStreamF32::new(dimensions, &values)),
        })
    }
    #[getter(dimensions)]
    pub fn binding_dimensions(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).dimensions())
    }
    #[pyo3(name = "values")]
    #[pyo3(signature = ())]
    pub fn binding_values(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).values())
    }
    #[pyo3(name = "replace")]
    #[pyo3(signature = (dimensions, values))]
    pub fn binding_replace(&mut self, dimensions: usize, values: Vec<f32>) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).replace(dimensions, &values);
        Ok(())
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(ObservationStreamF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
