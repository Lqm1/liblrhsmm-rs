use super::*;
#[pyclass(unsendable, module = "liblrhsmm_rs")]
pub struct GenerationF64 {
    pub(crate) inner: Option<crate::api::GenerationF64>,
}
#[pymethods]
impl GenerationF64 {
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
impl GenerationF64 {
    #[new]
    #[pyo3(signature = (model, sequence))]
    pub fn binding_new(
        model: pyo3::PyRef<'_, ModelF64>,
        sequence: pyo3::PyRef<'_, SegmentationF64>,
    ) -> pyo3::PyResult<Self> {
        Ok(GenerationF64 {
            inner: Some(crate::api::GenerationF64::new(
                model.inner.as_ref().ok_or_else(consumed)?,
                sequence.inner.as_ref().ok_or_else(consumed)?,
            )),
        })
    }
    #[pyo3(name = "model")]
    #[pyo3(signature = ())]
    pub fn binding_model(&mut self) -> pyo3::PyResult<ModelF64> {
        ((self.inner.as_ref().ok_or_else(consumed)?).model())
            .map(|value| ModelF64 { inner: Some(value) })
    }
    #[pyo3(name = "sequence")]
    #[pyo3(signature = ())]
    pub fn binding_sequence(&mut self) -> pyo3::PyResult<SegmentationF64> {
        ((self.inner.as_ref().ok_or_else(consumed)?).sequence())
            .map(|value| SegmentationF64 { inner: Some(value) })
    }
    #[pyo3(name = "set_model")]
    #[pyo3(signature = (model))]
    pub fn binding_set_model(&mut self, model: pyo3::PyRef<'_, ModelF64>) -> pyo3::PyResult<()> {
        (self.inner.as_ref().ok_or_else(consumed)?)
            .set_model(model.inner.as_ref().ok_or_else(consumed)?)
    }
    #[pyo3(name = "set_sequence")]
    #[pyo3(signature = (sequence))]
    pub fn binding_set_sequence(
        &mut self,
        sequence: pyo3::PyRef<'_, SegmentationF64>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_ref().ok_or_else(consumed)?)
            .set_sequence(sequence.inner.as_ref().ok_or_else(consumed)?)
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&mut self) -> pyo3::PyResult<Self> {
        ((self.inner.as_ref().ok_or_else(consumed)?).cloned())
            .map(|value| GenerationF64 { inner: Some(value) })
    }
    #[pyo3(name = "generate_with_uniform")]
    #[pyo3(signature = (callback))]
    pub fn binding_generate_with_uniform(
        &mut self,
        callback: Py<PyAny>,
    ) -> pyo3::PyResult<GeneratedObservationF64> {
        ((self.inner.as_ref().ok_or_else(consumed)?)
            .generate_with_uniform(&crate::callbacks::Function::new(callback)?))
        .map(|value| GeneratedObservationF64 { inner: Some(value) })
    }
    #[pyo3(name = "generate_with_random_words")]
    #[pyo3(signature = (callback))]
    pub fn binding_generate_with_random_words(
        &mut self,
        callback: Py<PyAny>,
    ) -> pyo3::PyResult<GeneratedObservationF64> {
        ((self.inner.as_ref().ok_or_else(consumed)?)
            .generate_with_random_words(&crate::callbacks::Function::new(callback)?))
        .map(|value| GeneratedObservationF64 { inner: Some(value) })
    }
}
