use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct PreparedModelF32 {
    pub(crate) inner: Option<crate::api::PreparedModelF32>,
}
#[pymethods]
impl PreparedModelF32 {
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
impl PreparedModelF32 {
    #[pyo3(name = "cloned")]
    #[pyo3(signature = ())]
    pub fn binding_cloned(&self) -> pyo3::PyResult<Self> {
        Ok(PreparedModelF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
    #[pyo3(name = "parameters")]
    #[pyo3(signature = ())]
    pub fn binding_parameters(&self) -> pyo3::PyResult<ModelF32> {
        Ok(ModelF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).parameters()),
        })
    }
    #[pyo3(name = "into_parameters")]
    #[pyo3(signature = ())]
    pub fn binding_into_parameters(&mut self) -> pyo3::PyResult<ModelF32> {
        Ok(ModelF32 {
            inner: Some((self.inner.take().ok_or_else(consumed)?).into_parameters()),
        })
    }
    #[pyo3(name = "options")]
    #[pyo3(signature = ())]
    pub fn binding_options(&self) -> pyo3::PyResult<PreparationOptionsF32> {
        Ok(PreparationOptionsF32 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).options()),
        })
    }
    #[pyo3(name = "duration")]
    #[pyo3(signature = (state))]
    pub fn binding_duration(&self, state: usize) -> pyo3::PyResult<Option<PreparedDurationF32>> {
        Ok(
            ((self.inner.as_ref().ok_or_else(consumed)?).duration(state))
                .map(|value| PreparedDurationF32 { inner: Some(value) }),
        )
    }
    #[pyo3(name = "emission")]
    #[pyo3(signature = (stream, state))]
    pub fn binding_emission(
        &self,
        stream: usize,
        state: usize,
    ) -> pyo3::PyResult<Option<PreparedGaussianF32>> {
        Ok(
            ((self.inner.as_ref().ok_or_else(consumed)?).emission(stream, state))
                .map(|value| PreparedGaussianF32 { inner: Some(value) }),
        )
    }
}
#[pymethods]
impl PreparedModelF32 {
    #[pyo3(name = "emission_log_probabilities")]
    #[pyo3(signature = (observation, segmentation, temperature, pruning))]
    pub fn binding_emission_log_probabilities(
        &self,
        observation: pyo3::PyRef<'_, ObservationF32>,
        segmentation: pyo3::PyRef<'_, SegmentationF32>,
        temperature: f32,
        pruning: pyo3::PyRef<'_, EmissionPruningF32>,
    ) -> pyo3::PyResult<LogEmissionsF32> {
        ((self.inner.as_ref().ok_or_else(consumed)?).emission_log_probabilities(
            observation.inner.as_ref().ok_or_else(consumed)?,
            segmentation.inner.as_ref().ok_or_else(consumed)?,
            temperature,
            pruning.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| LogEmissionsF32 { inner: Some(value) })
    }
    #[pyo3(name = "emission_probabilities")]
    #[pyo3(signature = (observation, segmentation, temperature, pruning))]
    pub fn binding_emission_probabilities(
        &self,
        observation: pyo3::PyRef<'_, ObservationF32>,
        segmentation: pyo3::PyRef<'_, SegmentationF32>,
        temperature: f32,
        pruning: pyo3::PyRef<'_, EmissionPruningF32>,
    ) -> pyo3::PyResult<EmissionDensitiesF32> {
        ((self.inner.as_ref().ok_or_else(consumed)?).emission_probabilities(
            observation.inner.as_ref().ok_or_else(consumed)?,
            segmentation.inner.as_ref().ok_or_else(consumed)?,
            temperature,
            pruning.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| EmissionDensitiesF32 { inner: Some(value) })
    }
}
#[pymethods]
impl PreparedModelF32 {
    #[pyo3(name = "forward_hsmm")]
    #[pyo3(signature = (segmentation, emissions, options))]
    pub fn binding_forward_hsmm(
        &self,
        segmentation: pyo3::PyRef<'_, SegmentationF32>,
        emissions: pyo3::PyRef<'_, LogEmissionsF32>,
        options: pyo3::PyRef<'_, HsmmOptionsF32>,
    ) -> pyo3::PyResult<HsmmScoresF32> {
        ((self.inner.as_ref().ok_or_else(consumed)?).forward_hsmm(
            segmentation.inner.as_ref().ok_or_else(consumed)?,
            emissions.inner.as_ref().ok_or_else(consumed)?,
            options.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| HsmmScoresF32 { inner: Some(value) })
    }
    #[pyo3(name = "backward_hsmm")]
    #[pyo3(signature = (segmentation, emissions, options))]
    pub fn binding_backward_hsmm(
        &self,
        segmentation: pyo3::PyRef<'_, SegmentationF32>,
        emissions: pyo3::PyRef<'_, LogEmissionsF32>,
        options: pyo3::PyRef<'_, HsmmOptionsF32>,
    ) -> pyo3::PyResult<HsmmScoresF32> {
        ((self.inner.as_ref().ok_or_else(consumed)?).backward_hsmm(
            segmentation.inner.as_ref().ok_or_else(consumed)?,
            emissions.inner.as_ref().ok_or_else(consumed)?,
            options.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| HsmmScoresF32 { inner: Some(value) })
    }
    #[pyo3(name = "viterbi_hsmm")]
    #[pyo3(signature = (segmentation, emissions, options))]
    pub fn binding_viterbi_hsmm(
        &self,
        segmentation: pyo3::PyRef<'_, SegmentationF32>,
        emissions: pyo3::PyRef<'_, LogEmissionsF32>,
        options: pyo3::PyRef<'_, HsmmOptionsF32>,
    ) -> pyo3::PyResult<HsmmPathF32> {
        ((self.inner.as_ref().ok_or_else(consumed)?).viterbi_hsmm(
            segmentation.inner.as_ref().ok_or_else(consumed)?,
            emissions.inner.as_ref().ok_or_else(consumed)?,
            options.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| HsmmPathF32 { inner: Some(value) })
    }
    #[pyo3(name = "forward_hsmm_with_graph")]
    #[pyo3(signature = (graph, emissions, options))]
    pub fn binding_forward_hsmm_with_graph(
        &self,
        graph: pyo3::PyRef<'_, TransitionGraphF32>,
        emissions: pyo3::PyRef<'_, LogEmissionsF32>,
        options: pyo3::PyRef<'_, HsmmOptionsF32>,
    ) -> pyo3::PyResult<HsmmScoresF32> {
        ((self.inner.as_ref().ok_or_else(consumed)?).forward_hsmm_with_graph(
            graph.inner.as_ref().ok_or_else(consumed)?,
            emissions.inner.as_ref().ok_or_else(consumed)?,
            options.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| HsmmScoresF32 { inner: Some(value) })
    }
    #[pyo3(name = "backward_hsmm_with_graph")]
    #[pyo3(signature = (graph, emissions, options))]
    pub fn binding_backward_hsmm_with_graph(
        &self,
        graph: pyo3::PyRef<'_, TransitionGraphF32>,
        emissions: pyo3::PyRef<'_, LogEmissionsF32>,
        options: pyo3::PyRef<'_, HsmmOptionsF32>,
    ) -> pyo3::PyResult<HsmmScoresF32> {
        ((self.inner.as_ref().ok_or_else(consumed)?).backward_hsmm_with_graph(
            graph.inner.as_ref().ok_or_else(consumed)?,
            emissions.inner.as_ref().ok_or_else(consumed)?,
            options.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| HsmmScoresF32 { inner: Some(value) })
    }
    #[pyo3(name = "viterbi_hsmm_with_graph")]
    #[pyo3(signature = (graph, emissions, options))]
    pub fn binding_viterbi_hsmm_with_graph(
        &self,
        graph: pyo3::PyRef<'_, TransitionGraphF32>,
        emissions: pyo3::PyRef<'_, LogEmissionsF32>,
        options: pyo3::PyRef<'_, HsmmOptionsF32>,
    ) -> pyo3::PyResult<HsmmPathF32> {
        ((self.inner.as_ref().ok_or_else(consumed)?).viterbi_hsmm_with_graph(
            graph.inner.as_ref().ok_or_else(consumed)?,
            emissions.inner.as_ref().ok_or_else(consumed)?,
            options.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| HsmmPathF32 { inner: Some(value) })
    }
}
