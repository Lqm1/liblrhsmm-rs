use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct StreamF64 {
    pub(crate) inner: Option<crate::api::StreamF64>,
}
#[cfg(feature = "serialization")]
#[pymethods]
impl StreamF64 {
    #[staticmethod]
    #[pyo3(name = "read")]
    #[pyo3(signature = (bytes))]
    pub fn binding_read(bytes: Vec<u8>) -> pyo3::PyResult<Self> {
        (crate::api::StreamF64::read(&bytes)).map(|value| StreamF64 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "read_with_limits")]
    #[pyo3(signature = (bytes, max_array_entries))]
    pub fn binding_read_with_limits(
        bytes: Vec<u8>,
        max_array_entries: usize,
    ) -> pyo3::PyResult<Self> {
        (crate::api::StreamF64::read_with_limits(&bytes, max_array_entries))
            .map(|value| StreamF64 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "read_prefix")]
    #[pyo3(signature = (bytes))]
    pub fn binding_read_prefix(bytes: Vec<u8>) -> pyo3::PyResult<DecodedStreamF64> {
        (crate::api::StreamF64::read_prefix(&bytes))
            .map(|value| DecodedStreamF64 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "read_prefix_with_limits")]
    #[pyo3(signature = (bytes, max_array_entries))]
    pub fn binding_read_prefix_with_limits(
        bytes: Vec<u8>,
        max_array_entries: usize,
    ) -> pyo3::PyResult<DecodedStreamF64> {
        (crate::api::StreamF64::read_prefix_with_limits(&bytes, max_array_entries))
            .map(|value| DecodedStreamF64 { inner: Some(value) })
    }
    #[pyo3(name = "write")]
    #[pyo3(signature = ())]
    pub fn binding_write(&self) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyBytes>> {
        ((self.inner.as_ref().ok_or_else(consumed)?).write()).map(|value| {
            let bytes = value;
            pyo3::Python::attach(|py| pyo3::types::PyBytes::new(py, &bytes).unbind())
        })
    }
    #[pyo3(name = "write_with_encoding")]
    #[pyo3(signature = (code))]
    pub fn binding_write_with_encoding(
        &self,
        code: u32,
    ) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyBytes>> {
        ((self.inner.as_ref().ok_or_else(consumed)?).write_with_encoding(code)).map(|value| {
            let bytes = value;
            pyo3::Python::attach(|py| pyo3::types::PyBytes::new(py, &bytes).unbind())
        })
    }
}
#[pymethods]
impl StreamF64 {
    #[pyo3(name = "update")]
    #[pyo3(signature = (statistics, options))]
    pub fn binding_update(
        &mut self,
        statistics: pyo3::PyRef<'_, StreamStatisticsF64>,
        options: pyo3::PyRef<'_, UpdateOptionsF64>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).update(
            statistics.inner.as_ref().ok_or_else(consumed)?,
            options.inner.as_ref().ok_or_else(consumed)?,
        )
    }
}
#[pymethods]
impl StreamF64 {
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
impl StreamF64 {
    #[new]
    #[pyo3(signature = (emissions, components, dimensions))]
    pub fn binding_new(
        emissions: usize,
        components: usize,
        dimensions: usize,
    ) -> pyo3::PyResult<Self> {
        (crate::api::StreamF64::new(emissions, components, dimensions))
            .map(|value| StreamF64 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "empty")]
    #[pyo3(signature = ())]
    pub fn binding_empty() -> pyo3::PyResult<Self> {
        Ok(StreamF64 {
            inner: Some(crate::api::StreamF64::empty()),
        })
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
    #[pyo3(name = "replace")]
    #[pyo3(signature = (weight, emissions))]
    pub fn binding_replace(
        &mut self,
        weight: Vec<f64>,
        emissions: pyo3::PyRef<'_, GaussianCollectionF64>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?)
            .replace(&weight, emissions.inner.as_ref().ok_or_else(consumed)?)
    }
    #[pyo3(name = "validate")]
    #[pyo3(signature = ())]
    pub fn binding_validate(&self) -> pyo3::PyResult<()> {
        (self.inner.as_ref().ok_or_else(consumed)?).validate()
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(StreamF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
    #[pyo3(name = "prepare")]
    #[pyo3(signature = ())]
    pub fn binding_prepare(&self) -> pyo3::PyResult<PreparedStreamF64> {
        ((self.inner.as_ref().ok_or_else(consumed)?).prepare())
            .map(|value| PreparedStreamF64 { inner: Some(value) })
    }
    #[pyo3(name = "prepare_owned")]
    #[pyo3(signature = ())]
    pub fn binding_prepare_owned(&mut self) -> pyo3::PyResult<PreparedStreamF64> {
        ((self.inner.take().ok_or_else(consumed)?).prepare_owned())
            .map(|value| PreparedStreamF64 { inner: Some(value) })
    }
}
