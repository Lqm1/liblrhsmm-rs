use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct GaussianF32 {
    pub(crate) inner: Option<crate::api::GaussianF32>,
}
#[pymethods]
impl GaussianF32 {
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
impl GaussianF32 {
    #[new]
    #[pyo3(signature = (mixtures, dimensions))]
    pub fn binding_new(mixtures: usize, dimensions: usize) -> pyo3::PyResult<Self> {
        (crate::api::GaussianF32::new(mixtures, dimensions))
            .map(|value| GaussianF32 { inner: Some(value) })
    }
    #[getter(dimensions)]
    pub fn binding_dimensions(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).dimensions())
    }
    #[getter(mixtures)]
    pub fn binding_mixtures(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).mixtures())
    }
    #[pyo3(name = "weights")]
    #[pyo3(signature = ())]
    pub fn binding_weights(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).weights())
    }
    #[pyo3(name = "means")]
    #[pyo3(signature = ())]
    pub fn binding_means(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).means())
    }
    #[pyo3(name = "variances")]
    #[pyo3(signature = ())]
    pub fn binding_variances(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).variances())
    }
    #[pyo3(name = "variance_floors")]
    #[pyo3(signature = ())]
    pub fn binding_variance_floors(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).variance_floors())
    }
    #[pyo3(name = "replace")]
    #[pyo3(signature = (dimensions, weights, means, variances, variance_floors))]
    pub fn binding_replace(
        &mut self,
        dimensions: usize,
        weights: Vec<f32>,
        means: Vec<f32>,
        variances: Vec<f32>,
        variance_floors: Vec<f32>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).replace(
            dimensions,
            &weights,
            &means,
            &variances,
            &variance_floors,
        )
    }
    #[pyo3(name = "validate")]
    #[pyo3(signature = ())]
    pub fn binding_validate(&self) -> pyo3::PyResult<()> {
        (self.inner.as_ref().ok_or_else(consumed)?).validate()
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(GaussianF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
    #[staticmethod]
    #[pyo3(name = "combine")]
    #[pyo3(signature = (distributions))]
    pub fn binding_combine(
        distributions: pyo3::PyRef<'_, GaussianCollectionF32>,
    ) -> pyo3::PyResult<Self> {
        (crate::api::GaussianF32::combine(distributions.inner.as_ref().ok_or_else(consumed)?))
            .map(|value| GaussianF32 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "from_durations")]
    #[pyo3(signature = (distributions))]
    pub fn binding_from_durations(
        distributions: pyo3::PyRef<'_, DurationCollectionF32>,
    ) -> pyo3::PyResult<Self> {
        (crate::api::GaussianF32::from_durations(
            distributions.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| GaussianF32 { inner: Some(value) })
    }
    #[pyo3(name = "increase_mixtures")]
    #[pyo3(signature = (count, displacement))]
    pub fn binding_increase_mixtures(
        &mut self,
        count: usize,
        displacement: f32,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).increase_mixtures(count, displacement)
    }
    #[pyo3(name = "prepare")]
    #[pyo3(signature = ())]
    pub fn binding_prepare(&self) -> pyo3::PyResult<PreparedGaussianF32> {
        ((self.inner.as_ref().ok_or_else(consumed)?).prepare())
            .map(|value| PreparedGaussianF32 { inner: Some(value) })
    }
    #[pyo3(name = "prepare_owned")]
    #[pyo3(signature = ())]
    pub fn binding_prepare_owned(&mut self) -> pyo3::PyResult<PreparedGaussianF32> {
        ((self.inner.take().ok_or_else(consumed)?).prepare_owned())
            .map(|value| PreparedGaussianF32 { inner: Some(value) })
    }
    #[pyo3(name = "update")]
    #[pyo3(signature = (statistics, options))]
    pub fn binding_update(
        &mut self,
        statistics: pyo3::PyRef<'_, GaussianStatisticsF32>,
        options: pyo3::PyRef<'_, UpdateOptionsF32>,
    ) -> pyo3::PyResult<bool> {
        (self.inner.as_mut().ok_or_else(consumed)?).update(
            statistics.inner.as_ref().ok_or_else(consumed)?,
            options.inner.as_ref().ok_or_else(consumed)?,
        )
    }
    #[cfg(feature = "serialization")]
    #[staticmethod]
    #[pyo3(name = "read")]
    #[pyo3(signature = (bytes))]
    pub fn binding_read(bytes: Vec<u8>) -> pyo3::PyResult<Self> {
        (crate::api::GaussianF32::read(&bytes)).map(|value| GaussianF32 { inner: Some(value) })
    }
    #[cfg(feature = "serialization")]
    #[staticmethod]
    #[pyo3(name = "read_with_limits")]
    #[pyo3(signature = (bytes, max_array_entries))]
    pub fn binding_read_with_limits(
        bytes: Vec<u8>,
        max_array_entries: usize,
    ) -> pyo3::PyResult<Self> {
        (crate::api::GaussianF32::read_with_limits(&bytes, max_array_entries))
            .map(|value| GaussianF32 { inner: Some(value) })
    }
    #[cfg(feature = "serialization")]
    #[staticmethod]
    #[pyo3(name = "read_prefix")]
    #[pyo3(signature = (bytes))]
    pub fn binding_read_prefix(bytes: Vec<u8>) -> pyo3::PyResult<DecodedGaussianF32> {
        (crate::api::GaussianF32::read_prefix(&bytes))
            .map(|value| DecodedGaussianF32 { inner: Some(value) })
    }
    #[cfg(feature = "serialization")]
    #[staticmethod]
    #[pyo3(name = "read_prefix_with_limits")]
    #[pyo3(signature = (bytes, max_array_entries))]
    pub fn binding_read_prefix_with_limits(
        bytes: Vec<u8>,
        max_array_entries: usize,
    ) -> pyo3::PyResult<DecodedGaussianF32> {
        (crate::api::GaussianF32::read_prefix_with_limits(&bytes, max_array_entries))
            .map(|value| DecodedGaussianF32 { inner: Some(value) })
    }
    #[cfg(feature = "serialization")]
    #[pyo3(name = "write")]
    #[pyo3(signature = ())]
    pub fn binding_write(&self) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyBytes>> {
        ((self.inner.as_ref().ok_or_else(consumed)?).write()).map(|value| {
            let bytes = value;
            pyo3::Python::attach(|py| pyo3::types::PyBytes::new(py, &bytes).unbind())
        })
    }
    #[cfg(feature = "serialization")]
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
