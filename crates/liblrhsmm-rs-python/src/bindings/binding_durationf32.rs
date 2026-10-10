use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct DurationF32 {
    pub(crate) inner: Option<crate::api::DurationF32>,
}
#[pymethods]
impl DurationF32 {
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
impl DurationF32 {
    #[new]
    #[pyo3(signature = ())]
    pub fn binding_new() -> pyo3::PyResult<Self> {
        Ok(DurationF32 {
            inner: Some(crate::api::DurationF32::new()),
        })
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(DurationF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
    #[pyo3(name = "values")]
    #[pyo3(signature = ())]
    pub fn binding_values(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).values())
    }
    #[pyo3(name = "constraints")]
    #[pyo3(signature = ())]
    pub fn binding_constraints(&self) -> pyo3::PyResult<Vec<i32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).constraints())
    }
    #[pyo3(name = "set_values")]
    #[pyo3(signature = (values))]
    pub fn binding_set_values(&mut self, values: Vec<f32>) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).set_values(&values)
    }
    #[pyo3(name = "set_constraints")]
    #[pyo3(signature = (values))]
    pub fn binding_set_constraints(&mut self, values: Vec<i32>) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).set_constraints(&values)
    }
    #[pyo3(name = "log_probability")]
    #[pyo3(signature = (frames, weight))]
    pub fn binding_log_probability(&self, frames: f32, weight: f32) -> pyo3::PyResult<f32> {
        (self.inner.as_ref().ok_or_else(consumed)?).log_probability(frames, weight)
    }
    #[pyo3(name = "annealed_log_probability")]
    #[pyo3(signature = (frames, weight, temperature))]
    pub fn binding_annealed_log_probability(
        &self,
        frames: usize,
        weight: f32,
        temperature: f32,
    ) -> pyo3::PyResult<f32> {
        (self.inner.as_ref().ok_or_else(consumed)?).annealed_log_probability(
            frames,
            weight,
            temperature,
        )
    }
    #[pyo3(name = "probability")]
    #[pyo3(signature = (frames, weight, temperature))]
    pub fn binding_probability(
        &self,
        frames: usize,
        weight: f32,
        temperature: f32,
    ) -> pyo3::PyResult<f64> {
        (self.inner.as_ref().ok_or_else(consumed)?).probability(frames, weight, temperature)
    }
    #[pyo3(name = "prepare")]
    #[pyo3(signature = (options))]
    pub fn binding_prepare(
        &self,
        options: pyo3::PyRef<'_, PreparationOptionsF32>,
    ) -> pyo3::PyResult<PreparedDurationF32> {
        ((self.inner.as_ref().ok_or_else(consumed)?)
            .prepare(options.inner.as_ref().ok_or_else(consumed)?))
        .map(|value| PreparedDurationF32 { inner: Some(value) })
    }
    #[pyo3(name = "prepare_owned")]
    #[pyo3(signature = (options))]
    pub fn binding_prepare_owned(
        &mut self,
        options: pyo3::PyRef<'_, PreparationOptionsF32>,
    ) -> pyo3::PyResult<PreparedDurationF32> {
        ((self.inner.take().ok_or_else(consumed)?)
            .prepare_owned(options.inner.as_ref().ok_or_else(consumed)?))
        .map(|value| PreparedDurationF32 { inner: Some(value) })
    }
    #[pyo3(name = "update")]
    #[pyo3(signature = (statistics, options))]
    pub fn binding_update(
        &mut self,
        statistics: pyo3::PyRef<'_, DurationStatisticsF32>,
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
        (crate::api::DurationF32::read(&bytes)).map(|value| DurationF32 { inner: Some(value) })
    }
    #[cfg(feature = "serialization")]
    #[staticmethod]
    #[pyo3(name = "read_with_limits")]
    #[pyo3(signature = (bytes, max_array_entries))]
    pub fn binding_read_with_limits(
        bytes: Vec<u8>,
        max_array_entries: usize,
    ) -> pyo3::PyResult<Self> {
        (crate::api::DurationF32::read_with_limits(&bytes, max_array_entries))
            .map(|value| DurationF32 { inner: Some(value) })
    }
    #[cfg(feature = "serialization")]
    #[staticmethod]
    #[pyo3(name = "read_prefix")]
    #[pyo3(signature = (bytes))]
    pub fn binding_read_prefix(bytes: Vec<u8>) -> pyo3::PyResult<DecodedDurationF32> {
        (crate::api::DurationF32::read_prefix(&bytes))
            .map(|value| DecodedDurationF32 { inner: Some(value) })
    }
    #[cfg(feature = "serialization")]
    #[staticmethod]
    #[pyo3(name = "read_prefix_with_limits")]
    #[pyo3(signature = (bytes, max_array_entries))]
    pub fn binding_read_prefix_with_limits(
        bytes: Vec<u8>,
        max_array_entries: usize,
    ) -> pyo3::PyResult<DecodedDurationF32> {
        (crate::api::DurationF32::read_prefix_with_limits(&bytes, max_array_entries))
            .map(|value| DecodedDurationF32 { inner: Some(value) })
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
}
