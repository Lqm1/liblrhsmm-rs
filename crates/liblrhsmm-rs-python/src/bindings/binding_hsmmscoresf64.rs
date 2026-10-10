use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct HsmmScoresF64 {
    pub(crate) inner: Option<crate::api::HsmmScoresF64>,
}
#[pymethods]
impl HsmmScoresF64 {
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
impl HsmmScoresF64 {
    #[new]
    #[pyo3(signature = (begin, end))]
    pub fn binding_new(
        begin: pyo3::PyRef<'_, StateScoresF64>,
        end: pyo3::PyRef<'_, StateScoresF64>,
    ) -> pyo3::PyResult<Self> {
        Ok(HsmmScoresF64 {
            inner: Some(crate::api::HsmmScoresF64::new(
                begin.inner.as_ref().ok_or_else(consumed)?,
                end.inner.as_ref().ok_or_else(consumed)?,
            )),
        })
    }
    #[pyo3(name = "begin")]
    #[pyo3(signature = ())]
    pub fn binding_begin(&self) -> pyo3::PyResult<StateScoresF64> {
        Ok(StateScoresF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).begin()),
        })
    }
    #[pyo3(name = "end")]
    #[pyo3(signature = ())]
    pub fn binding_end(&self) -> pyo3::PyResult<StateScoresF64> {
        Ok(StateScoresF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).end()),
        })
    }
    #[pyo3(name = "set_begin")]
    #[pyo3(signature = (value))]
    pub fn binding_set_begin(
        &mut self,
        value: pyo3::PyRef<'_, StateScoresF64>,
    ) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?)
            .set_begin(value.inner.as_ref().ok_or_else(consumed)?);
        Ok(())
    }
    #[pyo3(name = "set_end")]
    #[pyo3(signature = (value))]
    pub fn binding_set_end(
        &mut self,
        value: pyo3::PyRef<'_, StateScoresF64>,
    ) -> pyo3::PyResult<()> {
        let _: () = (self.inner.as_mut().ok_or_else(consumed)?)
            .set_end(value.inner.as_ref().ok_or_else(consumed)?);
        Ok(())
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(HsmmScoresF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
}
