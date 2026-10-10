use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct StreamBuilderF64 {
    pub(crate) inner: Option<crate::api::StreamBuilderF64>,
}
#[pymethods]
impl StreamBuilderF64 {
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
impl StreamBuilderF64 {
    #[new]
    #[pyo3(signature = (emissions))]
    pub fn binding_new(emissions: usize) -> pyo3::PyResult<Self> {
        (crate::api::StreamBuilderF64::new(emissions))
            .map(|value| StreamBuilderF64 { inner: Some(value) })
    }
    #[getter(emissions)]
    pub fn binding_emissions(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).emissions())
    }
    #[pyo3(name = "weight")]
    #[pyo3(signature = ())]
    pub fn binding_weight(&self) -> pyo3::PyResult<Vec<f64>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).weight())
    }
    #[pyo3(name = "set_weight")]
    #[pyo3(signature = (values))]
    pub fn binding_set_weight(&mut self, values: Vec<f64>) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).set_weight(&values)
    }
    #[pyo3(name = "emission")]
    #[pyo3(signature = (index))]
    pub fn binding_emission(&self, index: usize) -> pyo3::PyResult<Option<GaussianF64>> {
        Ok(
            ((self.inner.as_ref().ok_or_else(consumed)?).emission(index))
                .map(|value| GaussianF64 { inner: Some(value) }),
        )
    }
    #[pyo3(name = "has_emission")]
    #[pyo3(signature = (index))]
    pub fn binding_has_emission(&self, index: usize) -> pyo3::PyResult<bool> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).has_emission(index))
    }
    #[pyo3(name = "resize")]
    #[pyo3(signature = (emissions))]
    pub fn binding_resize(&mut self, emissions: usize) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).resize(emissions)
    }
    #[pyo3(name = "set_emission")]
    #[pyo3(signature = (index, value))]
    pub fn binding_set_emission(
        &mut self,
        index: usize,
        value: pyo3::PyRef<'_, GaussianF64>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?)
            .set_emission(index, value.inner.as_ref().ok_or_else(consumed)?)
    }
    #[pyo3(name = "clear_emission")]
    #[pyo3(signature = (index))]
    pub fn binding_clear_emission(&mut self, index: usize) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).clear_emission(index)
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(StreamBuilderF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
    #[pyo3(name = "build")]
    #[pyo3(signature = ())]
    pub fn binding_build(&mut self) -> pyo3::PyResult<StreamF64> {
        ((self.inner.take().ok_or_else(consumed)?).build())
            .map(|value| StreamF64 { inner: Some(value) })
    }
}
