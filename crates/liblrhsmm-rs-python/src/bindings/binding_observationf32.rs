use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct ObservationF32 {
    pub(crate) inner: Option<crate::api::ObservationF32>,
}
#[pymethods]
impl ObservationF32 {
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
impl ObservationF32 {
    #[new]
    #[pyo3(signature = (frames, dimensions))]
    pub fn binding_new(frames: usize, dimensions: Vec<u32>) -> pyo3::PyResult<Self> {
        (crate::api::ObservationF32::new(frames, &dimensions))
            .map(|value| ObservationF32 { inner: Some(value) })
    }
    #[getter(frames)]
    pub fn binding_frames(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).frames())
    }
    #[getter(streams)]
    pub fn binding_streams(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).streams())
    }
    #[pyo3(name = "stream")]
    #[pyo3(signature = (index))]
    pub fn binding_stream(&self, index: usize) -> pyo3::PyResult<Option<ObservationStreamF32>> {
        Ok(((self.inner.as_ref().ok_or_else(consumed)?).stream(index))
            .map(|value| ObservationStreamF32 { inner: Some(value) }))
    }
    #[pyo3(name = "set_stream")]
    #[pyo3(signature = (index, value))]
    pub fn binding_set_stream(
        &mut self,
        index: usize,
        value: pyo3::PyRef<'_, ObservationStreamF32>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?)
            .set_stream(index, value.inner.as_ref().ok_or_else(consumed)?)
    }
    #[pyo3(name = "replace")]
    #[pyo3(signature = (frames, streams))]
    pub fn binding_replace(
        &mut self,
        frames: usize,
        streams: pyo3::PyRef<'_, ObservationStreamCollectionF32>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?)
            .replace(frames, streams.inner.as_ref().ok_or_else(consumed)?)
    }
    #[pyo3(name = "frame")]
    #[pyo3(signature = (stream, time))]
    pub fn binding_frame(&self, stream: usize, time: usize) -> pyo3::PyResult<Option<Vec<f32>>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).frame(stream, time))
    }
    #[pyo3(name = "validate")]
    #[pyo3(signature = ())]
    pub fn binding_validate(&self) -> pyo3::PyResult<()> {
        (self.inner.as_ref().ok_or_else(consumed)?).validate()
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(ObservationF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
#[cfg(feature = "serialization")]
#[pymethods]
impl ObservationF32 {
    #[staticmethod]
    #[pyo3(name = "read")]
    #[pyo3(signature = (bytes))]
    pub fn binding_read(bytes: Vec<u8>) -> pyo3::PyResult<Self> {
        (crate::api::ObservationF32::read(&bytes))
            .map(|value| ObservationF32 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "read_with_limits")]
    #[pyo3(signature = (bytes, max_array_entries))]
    pub fn binding_read_with_limits(
        bytes: Vec<u8>,
        max_array_entries: usize,
    ) -> pyo3::PyResult<Self> {
        (crate::api::ObservationF32::read_with_limits(&bytes, max_array_entries))
            .map(|value| ObservationF32 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "read_prefix")]
    #[pyo3(signature = (bytes))]
    pub fn binding_read_prefix(bytes: Vec<u8>) -> pyo3::PyResult<DecodedObservationF32> {
        (crate::api::ObservationF32::read_prefix(&bytes))
            .map(|value| DecodedObservationF32 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "read_prefix_with_limits")]
    #[pyo3(signature = (bytes, max_array_entries))]
    pub fn binding_read_prefix_with_limits(
        bytes: Vec<u8>,
        max_array_entries: usize,
    ) -> pyo3::PyResult<DecodedObservationF32> {
        (crate::api::ObservationF32::read_prefix_with_limits(&bytes, max_array_entries))
            .map(|value| DecodedObservationF32 { inner: Some(value) })
    }
    #[pyo3(name = "write")]
    #[pyo3(signature = ())]
    pub fn binding_write(&self) -> pyo3::PyResult<pyo3::Py<pyo3::types::PyBytes>> {
        ((self.inner.as_ref().ok_or_else(consumed)?).write()).map(|value| {
            let bytes = value;
            pyo3::Python::attach(|py| pyo3::types::PyBytes::new(py, &bytes).unbind())
        })
    }
}
