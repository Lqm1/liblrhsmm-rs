use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct PreparedModelF64 {
    pub(crate) inner: Option<crate::api::PreparedModelF64>,
}
#[pymethods]
impl PreparedModelF64 {
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
impl PreparedModelF64 {
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(PreparedModelF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
    #[pyo3(name = "parameters")]
    #[pyo3(signature = ())]
    pub fn binding_parameters(&self) -> pyo3::PyResult<ModelF64> {
        Ok(ModelF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).parameters()),
        })
    }
    #[pyo3(name = "into_parameters")]
    #[pyo3(signature = ())]
    pub fn binding_into_parameters(&mut self) -> pyo3::PyResult<ModelF64> {
        Ok(ModelF64 {
            inner: Some((self.inner.take().ok_or_else(consumed)?).into_parameters()),
        })
    }
    #[pyo3(name = "options")]
    #[pyo3(signature = ())]
    pub fn binding_options(&self) -> pyo3::PyResult<PreparationOptionsF64> {
        Ok(PreparationOptionsF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).options()),
        })
    }
    #[pyo3(name = "duration")]
    #[pyo3(signature = (state))]
    pub fn binding_duration(&self, state: usize) -> pyo3::PyResult<Option<PreparedDurationF64>> {
        Ok(
            ((self.inner.as_ref().ok_or_else(consumed)?).duration(state))
                .map(|value| PreparedDurationF64 { inner: Some(value) }),
        )
    }
    #[pyo3(name = "emission")]
    #[pyo3(signature = (stream, state))]
    pub fn binding_emission(
        &self,
        stream: usize,
        state: usize,
    ) -> pyo3::PyResult<Option<PreparedGaussianF64>> {
        Ok(
            ((self.inner.as_ref().ok_or_else(consumed)?).emission(stream, state))
                .map(|value| PreparedGaussianF64 { inner: Some(value) }),
        )
    }
}
#[pymethods]
impl PreparedModelF64 {
    #[pyo3(name = "emission_log_probabilities")]
    #[pyo3(signature = (observation, segmentation, temperature, pruning))]
    pub fn binding_emission_log_probabilities(
        &self,
        observation: pyo3::PyRef<'_, ObservationF64>,
        segmentation: pyo3::PyRef<'_, SegmentationF64>,
        temperature: f64,
        pruning: pyo3::PyRef<'_, EmissionPruningF64>,
    ) -> pyo3::PyResult<LogEmissionsF64> {
        ((self.inner.as_ref().ok_or_else(consumed)?).emission_log_probabilities(
            observation.inner.as_ref().ok_or_else(consumed)?,
            segmentation.inner.as_ref().ok_or_else(consumed)?,
            temperature,
            pruning.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| LogEmissionsF64 { inner: Some(value) })
    }
    #[pyo3(name = "emission_probabilities")]
    #[pyo3(signature = (observation, segmentation, temperature, pruning))]
    pub fn binding_emission_probabilities(
        &self,
        observation: pyo3::PyRef<'_, ObservationF64>,
        segmentation: pyo3::PyRef<'_, SegmentationF64>,
        temperature: f64,
        pruning: pyo3::PyRef<'_, EmissionPruningF64>,
    ) -> pyo3::PyResult<EmissionDensitiesF64> {
        ((self.inner.as_ref().ok_or_else(consumed)?).emission_probabilities(
            observation.inner.as_ref().ok_or_else(consumed)?,
            segmentation.inner.as_ref().ok_or_else(consumed)?,
            temperature,
            pruning.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| EmissionDensitiesF64 { inner: Some(value) })
    }
}
#[pymethods]
impl PreparedModelF64 {
    #[pyo3(name = "forward_hsmm")]
    #[pyo3(signature = (segmentation, emissions, options))]
    pub fn binding_forward_hsmm(
        &self,
        segmentation: pyo3::PyRef<'_, SegmentationF64>,
        emissions: pyo3::PyRef<'_, LogEmissionsF64>,
        options: pyo3::PyRef<'_, HsmmOptionsF64>,
    ) -> pyo3::PyResult<HsmmScoresF64> {
        ((self.inner.as_ref().ok_or_else(consumed)?).forward_hsmm(
            segmentation.inner.as_ref().ok_or_else(consumed)?,
            emissions.inner.as_ref().ok_or_else(consumed)?,
            options.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| HsmmScoresF64 { inner: Some(value) })
    }
    #[pyo3(name = "backward_hsmm")]
    #[pyo3(signature = (segmentation, emissions, options))]
    pub fn binding_backward_hsmm(
        &self,
        segmentation: pyo3::PyRef<'_, SegmentationF64>,
        emissions: pyo3::PyRef<'_, LogEmissionsF64>,
        options: pyo3::PyRef<'_, HsmmOptionsF64>,
    ) -> pyo3::PyResult<HsmmScoresF64> {
        ((self.inner.as_ref().ok_or_else(consumed)?).backward_hsmm(
            segmentation.inner.as_ref().ok_or_else(consumed)?,
            emissions.inner.as_ref().ok_or_else(consumed)?,
            options.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| HsmmScoresF64 { inner: Some(value) })
    }
    #[pyo3(name = "viterbi_hsmm")]
    #[pyo3(signature = (segmentation, emissions, options))]
    pub fn binding_viterbi_hsmm(
        &self,
        segmentation: pyo3::PyRef<'_, SegmentationF64>,
        emissions: pyo3::PyRef<'_, LogEmissionsF64>,
        options: pyo3::PyRef<'_, HsmmOptionsF64>,
    ) -> pyo3::PyResult<HsmmPathF64> {
        ((self.inner.as_ref().ok_or_else(consumed)?).viterbi_hsmm(
            segmentation.inner.as_ref().ok_or_else(consumed)?,
            emissions.inner.as_ref().ok_or_else(consumed)?,
            options.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| HsmmPathF64 { inner: Some(value) })
    }
    #[pyo3(name = "forward_hsmm_with_graph")]
    #[pyo3(signature = (graph, emissions, options))]
    pub fn binding_forward_hsmm_with_graph(
        &self,
        graph: pyo3::PyRef<'_, TransitionGraphF64>,
        emissions: pyo3::PyRef<'_, LogEmissionsF64>,
        options: pyo3::PyRef<'_, HsmmOptionsF64>,
    ) -> pyo3::PyResult<HsmmScoresF64> {
        ((self.inner.as_ref().ok_or_else(consumed)?).forward_hsmm_with_graph(
            graph.inner.as_ref().ok_or_else(consumed)?,
            emissions.inner.as_ref().ok_or_else(consumed)?,
            options.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| HsmmScoresF64 { inner: Some(value) })
    }
    #[pyo3(name = "backward_hsmm_with_graph")]
    #[pyo3(signature = (graph, emissions, options))]
    pub fn binding_backward_hsmm_with_graph(
        &self,
        graph: pyo3::PyRef<'_, TransitionGraphF64>,
        emissions: pyo3::PyRef<'_, LogEmissionsF64>,
        options: pyo3::PyRef<'_, HsmmOptionsF64>,
    ) -> pyo3::PyResult<HsmmScoresF64> {
        ((self.inner.as_ref().ok_or_else(consumed)?).backward_hsmm_with_graph(
            graph.inner.as_ref().ok_or_else(consumed)?,
            emissions.inner.as_ref().ok_or_else(consumed)?,
            options.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| HsmmScoresF64 { inner: Some(value) })
    }
    #[pyo3(name = "viterbi_hsmm_with_graph")]
    #[pyo3(signature = (graph, emissions, options))]
    pub fn binding_viterbi_hsmm_with_graph(
        &self,
        graph: pyo3::PyRef<'_, TransitionGraphF64>,
        emissions: pyo3::PyRef<'_, LogEmissionsF64>,
        options: pyo3::PyRef<'_, HsmmOptionsF64>,
    ) -> pyo3::PyResult<HsmmPathF64> {
        ((self.inner.as_ref().ok_or_else(consumed)?).viterbi_hsmm_with_graph(
            graph.inner.as_ref().ok_or_else(consumed)?,
            emissions.inner.as_ref().ok_or_else(consumed)?,
            options.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| HsmmPathF64 { inner: Some(value) })
    }
}
