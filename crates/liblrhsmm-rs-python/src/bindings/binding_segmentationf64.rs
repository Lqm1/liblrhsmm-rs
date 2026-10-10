use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct SegmentationF64 {
    pub(crate) inner: Option<crate::api::SegmentationF64>,
}
#[pymethods]
impl SegmentationF64 {
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
impl SegmentationF64 {
    #[new]
    #[pyo3(signature = (streams, segments))]
    pub fn binding_new(streams: usize, segments: usize) -> pyo3::PyResult<Self> {
        (crate::api::SegmentationF64::new(streams, segments))
            .map(|value| SegmentationF64 { inner: Some(value) })
    }
    #[getter(streams)]
    pub fn binding_streams(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).streams())
    }
    #[getter(segments)]
    pub fn binding_segments(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).segments())
    }
    #[pyo3(name = "boundaries")]
    #[pyo3(signature = ())]
    pub fn binding_boundaries(&self) -> pyo3::PyResult<Vec<i32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).boundaries())
    }
    #[pyo3(name = "duration_states")]
    #[pyo3(signature = ())]
    pub fn binding_duration_states(&self) -> pyo3::PyResult<Vec<i32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).duration_states())
    }
    #[pyo3(name = "output_states")]
    #[pyo3(signature = (stream))]
    pub fn binding_output_states(&self, stream: usize) -> pyo3::PyResult<Vec<i32>> {
        (self.inner.as_ref().ok_or_else(consumed)?).output_states(stream)
    }
    #[pyo3(name = "set_boundaries")]
    #[pyo3(signature = (values))]
    pub fn binding_set_boundaries(&mut self, values: Vec<i32>) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).set_boundaries(&values)
    }
    #[pyo3(name = "set_duration_states")]
    #[pyo3(signature = (values))]
    pub fn binding_set_duration_states(&mut self, values: Vec<i32>) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).set_duration_states(&values)
    }
    #[pyo3(name = "set_output_states")]
    #[pyo3(signature = (stream, values))]
    pub fn binding_set_output_states(
        &mut self,
        stream: usize,
        values: Vec<i32>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).set_output_states(stream, &values)
    }
    #[pyo3(name = "outgoing")]
    #[pyo3(signature = (state))]
    pub fn binding_outgoing(&self, state: usize) -> pyo3::PyResult<Option<JumpGroupF64>> {
        Ok(
            ((self.inner.as_ref().ok_or_else(consumed)?).outgoing(state))
                .map(|value| JumpGroupF64 { inner: Some(value) }),
        )
    }
    #[pyo3(name = "set_outgoing")]
    #[pyo3(signature = (state, jumps))]
    pub fn binding_set_outgoing(
        &mut self,
        state: usize,
        jumps: pyo3::PyRef<'_, JumpGroupF64>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?)
            .set_outgoing(state, jumps.inner.as_ref().ok_or_else(consumed)?)
    }
    #[pyo3(name = "replace")]
    #[pyo3(signature = (value))]
    pub fn binding_replace(
        &mut self,
        value: pyo3::PyRef<'_, SegmentationF64>,
    ) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?)
            .replace(value.inner.as_ref().ok_or_else(consumed)?);
        Ok(())
    }
    #[pyo3(name = "validate")]
    #[pyo3(signature = ())]
    pub fn binding_validate(&self) -> pyo3::PyResult<()> {
        (self.inner.as_ref().ok_or_else(consumed)?).validate()
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(SegmentationF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
    #[pyo3(name = "add_jump")]
    #[pyo3(signature = (state, delta, probability))]
    pub fn binding_add_jump(
        &mut self,
        state: usize,
        delta: i32,
        probability: f64,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).add_jump(state, delta, probability)
    }
    #[pyo3(name = "build_transitions")]
    #[pyo3(signature = ())]
    pub fn binding_build_transitions(&self) -> pyo3::PyResult<TransitionGraphF64> {
        ((self.inner.as_ref().ok_or_else(consumed)?).build_transitions())
            .map(|value| TransitionGraphF64 { inner: Some(value) })
    }
    #[pyo3(name = "build_transitions_owned")]
    #[pyo3(signature = ())]
    pub fn binding_build_transitions_owned(&mut self) -> pyo3::PyResult<TransitionGraphF64> {
        ((self.inner.take().ok_or_else(consumed)?).build_transitions_owned())
            .map(|value| TransitionGraphF64 { inner: Some(value) })
    }
    #[pyo3(name = "resegment")]
    #[pyo3(signature = (occurrences))]
    pub fn binding_resegment(&self, occurrences: Vec<u32>) -> pyo3::PyResult<Self> {
        ((self.inner.as_ref().ok_or_else(consumed)?).resegment(&occurrences))
            .map(|value| SegmentationF64 { inner: Some(value) })
    }
}
#[cfg(feature = "serialization")]
#[pymethods]
impl SegmentationF64 {
    #[staticmethod]
    #[pyo3(name = "read")]
    #[pyo3(signature = (bytes))]
    pub fn binding_read(bytes: Vec<u8>) -> pyo3::PyResult<Self> {
        (crate::api::SegmentationF64::read(&bytes))
            .map(|value| SegmentationF64 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "read_with_limits")]
    #[pyo3(signature = (bytes, max_array_entries))]
    pub fn binding_read_with_limits(
        bytes: Vec<u8>,
        max_array_entries: usize,
    ) -> pyo3::PyResult<Self> {
        (crate::api::SegmentationF64::read_with_limits(&bytes, max_array_entries))
            .map(|value| SegmentationF64 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "read_prefix")]
    #[pyo3(signature = (bytes))]
    pub fn binding_read_prefix(bytes: Vec<u8>) -> pyo3::PyResult<DecodedSegmentationF64> {
        (crate::api::SegmentationF64::read_prefix(&bytes))
            .map(|value| DecodedSegmentationF64 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "read_prefix_with_limits")]
    #[pyo3(signature = (bytes, max_array_entries))]
    pub fn binding_read_prefix_with_limits(
        bytes: Vec<u8>,
        max_array_entries: usize,
    ) -> pyo3::PyResult<DecodedSegmentationF64> {
        (crate::api::SegmentationF64::read_prefix_with_limits(&bytes, max_array_entries))
            .map(|value| DecodedSegmentationF64 { inner: Some(value) })
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
