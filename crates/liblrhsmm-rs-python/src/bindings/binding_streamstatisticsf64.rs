use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct StreamStatisticsF64 {
    pub(crate) inner: Option<crate::api::StreamStatisticsF64>,
}
#[pymethods]
impl StreamStatisticsF64 {
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
impl StreamStatisticsF64 {
    #[new]
    #[pyo3(signature = ())]
    pub fn binding_new() -> pyo3::PyResult<Self> {
        Ok(StreamStatisticsF64 {
            inner: Some(crate::api::StreamStatisticsF64::new()),
        })
    }
    #[staticmethod]
    #[pyo3(name = "from_stream")]
    #[pyo3(signature = (value))]
    pub fn binding_from_stream(value: pyo3::PyRef<'_, StreamF64>) -> pyo3::PyResult<Self> {
        (crate::api::StreamStatisticsF64::from_stream(value.inner.as_ref().ok_or_else(consumed)?))
            .map(|value| StreamStatisticsF64 { inner: Some(value) })
    }
    #[getter(emissions)]
    pub fn binding_emissions(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).emissions())
    }
    #[pyo3(name = "emission")]
    #[pyo3(signature = (index))]
    pub fn binding_emission(&self, index: usize) -> pyo3::PyResult<Option<GaussianStatisticsF64>> {
        Ok(
            ((self.inner.as_ref().ok_or_else(consumed)?).emission(index))
                .map(|value| GaussianStatisticsF64 { inner: Some(value) }),
        )
    }
    #[pyo3(name = "set_emission")]
    #[pyo3(signature = (index, value))]
    pub fn binding_set_emission(
        &mut self,
        index: usize,
        value: pyo3::PyRef<'_, GaussianStatisticsF64>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?)
            .set_emission(index, value.inner.as_ref().ok_or_else(consumed)?)
    }
    #[pyo3(name = "replace")]
    #[pyo3(signature = (values))]
    pub fn binding_replace(
        &mut self,
        values: pyo3::PyRef<'_, GaussianStatisticsCollectionF64>,
    ) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?)
            .replace(values.inner.as_ref().ok_or_else(consumed)?);
        Ok(())
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(StreamStatisticsF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
    #[pyo3(name = "clear")]
    #[pyo3(signature = ())]
    pub fn binding_clear(&mut self) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?).clear();
        Ok(())
    }
}
