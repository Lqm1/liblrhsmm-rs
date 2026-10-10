use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct GaussianStatisticsF32 {
    pub(crate) inner: Option<crate::api::GaussianStatisticsF32>,
}
#[pymethods]
impl GaussianStatisticsF32 {
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
impl GaussianStatisticsF32 {
    #[new]
    #[pyo3(signature = (mixtures, dimensions))]
    pub fn binding_new(mixtures: usize, dimensions: usize) -> pyo3::PyResult<Self> {
        (crate::api::GaussianStatisticsF32::new(mixtures, dimensions))
            .map(|value| GaussianStatisticsF32 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "from_gaussian")]
    #[pyo3(signature = (value))]
    pub fn binding_from_gaussian(value: pyo3::PyRef<'_, GaussianF32>) -> pyo3::PyResult<Self> {
        (crate::api::GaussianStatisticsF32::from_gaussian(
            value.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| GaussianStatisticsF32 { inner: Some(value) })
    }
    #[getter(dimensions)]
    pub fn binding_dimensions(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).dimensions())
    }
    #[getter(mixtures)]
    pub fn binding_mixtures(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).mixtures())
    }
    #[pyo3(name = "weighted_sums")]
    #[pyo3(signature = ())]
    pub fn binding_weighted_sums(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).weighted_sums())
    }
    #[pyo3(name = "weighted_squared_sums")]
    #[pyo3(signature = ())]
    pub fn binding_weighted_squared_sums(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).weighted_squared_sums())
    }
    #[pyo3(name = "component_occupancies")]
    #[pyo3(signature = ())]
    pub fn binding_component_occupancies(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).component_occupancies())
    }
    #[pyo3(name = "state_occupancy")]
    #[pyo3(signature = ())]
    pub fn binding_state_occupancy(&self) -> pyo3::PyResult<Vec<f32>> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).state_occupancy())
    }
    #[pyo3(name = "replace")]
    #[pyo3(
        signature = (
            dimensions,
            weighted_sums,
            weighted_squared_sums,
            component_occupancies,
            state_occupancy
        )
    )]
    pub fn binding_replace(
        &mut self,
        dimensions: usize,
        weighted_sums: Vec<f32>,
        weighted_squared_sums: Vec<f32>,
        component_occupancies: Vec<f32>,
        state_occupancy: Vec<f32>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).replace(
            dimensions,
            &weighted_sums,
            &weighted_squared_sums,
            &component_occupancies,
            &state_occupancy,
        )
    }
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(GaussianStatisticsF32 {
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
