use super::*;
#[pyclass(module = "liblrhsmm_rs")]
pub struct ModelF64 {
    pub(crate) inner: Option<crate::api::ModelF64>,
}
#[cfg(feature = "serialization")]
#[pymethods]
impl ModelF64 {
    #[staticmethod]
    #[pyo3(name = "read")]
    #[pyo3(signature = (bytes))]
    pub fn binding_read(bytes: Vec<u8>) -> pyo3::PyResult<Self> {
        (crate::api::ModelF64::read(&bytes)).map(|value| ModelF64 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "read_with_limits")]
    #[pyo3(signature = (bytes, max_array_entries))]
    pub fn binding_read_with_limits(
        bytes: Vec<u8>,
        max_array_entries: usize,
    ) -> pyo3::PyResult<Self> {
        (crate::api::ModelF64::read_with_limits(&bytes, max_array_entries))
            .map(|value| ModelF64 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "read_prefix")]
    #[pyo3(signature = (bytes))]
    pub fn binding_read_prefix(bytes: Vec<u8>) -> pyo3::PyResult<DecodedModelF64> {
        (crate::api::ModelF64::read_prefix(&bytes))
            .map(|value| DecodedModelF64 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "read_prefix_with_limits")]
    #[pyo3(signature = (bytes, max_array_entries))]
    pub fn binding_read_prefix_with_limits(
        bytes: Vec<u8>,
        max_array_entries: usize,
    ) -> pyo3::PyResult<DecodedModelF64> {
        (crate::api::ModelF64::read_prefix_with_limits(&bytes, max_array_entries))
            .map(|value| DecodedModelF64 { inner: Some(value) })
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
impl ModelF64 {
    #[pyo3(name = "update")]
    #[pyo3(signature = (statistics, options))]
    pub fn binding_update(
        &mut self,
        statistics: pyo3::PyRef<'_, ModelStatisticsF64>,
        options: pyo3::PyRef<'_, UpdateOptionsF64>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).update(
            statistics.inner.as_ref().ok_or_else(consumed)?,
            options.inner.as_ref().ok_or_else(consumed)?,
        )
    }
}
#[pymethods]
impl ModelF64 {
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
impl ModelF64 {
    #[new]
    #[pyo3(signature = (layouts, durations))]
    pub fn binding_new(layouts: Vec<u32>, durations: usize) -> pyo3::PyResult<Self> {
        (crate::api::ModelF64::new(&layouts, durations))
            .map(|value| ModelF64 { inner: Some(value) })
    }
    #[staticmethod]
    #[pyo3(name = "empty")]
    #[pyo3(signature = ())]
    pub fn binding_empty() -> pyo3::PyResult<Self> {
        Ok(ModelF64 {
            inner: Some(crate::api::ModelF64::empty()),
        })
    }
    #[getter(streams)]
    pub fn binding_streams(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).streams())
    }
    #[getter(durations)]
    pub fn binding_durations(&self) -> pyo3::PyResult<usize> {
        Ok((self.inner.as_ref().ok_or_else(consumed)?).durations())
    }
    #[pyo3(name = "stream")]
    #[pyo3(signature = (index))]
    pub fn binding_stream(&self, index: usize) -> pyo3::PyResult<Option<StreamF64>> {
        Ok(((self.inner.as_ref().ok_or_else(consumed)?).stream(index))
            .map(|value| StreamF64 { inner: Some(value) }))
    }
    #[pyo3(name = "duration")]
    #[pyo3(signature = (index))]
    pub fn binding_duration(&self, index: usize) -> pyo3::PyResult<Option<DurationF64>> {
        Ok(
            ((self.inner.as_ref().ok_or_else(consumed)?).duration(index))
                .map(|value| DurationF64 { inner: Some(value) }),
        )
    }
    #[pyo3(name = "set_stream")]
    #[pyo3(signature = (index, value))]
    pub fn binding_set_stream(
        &mut self,
        index: usize,
        value: pyo3::PyRef<'_, StreamF64>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?)
            .set_stream(index, value.inner.as_ref().ok_or_else(consumed)?)
    }
    #[pyo3(name = "set_duration")]
    #[pyo3(signature = (index, value))]
    pub fn binding_set_duration(
        &mut self,
        index: usize,
        value: pyo3::PyRef<'_, DurationF64>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?)
            .set_duration(index, value.inner.as_ref().ok_or_else(consumed)?)
    }
    #[pyo3(name = "replace")]
    #[pyo3(signature = (streams, durations))]
    pub fn binding_replace(
        &mut self,
        streams: pyo3::PyRef<'_, StreamCollectionF64>,
        durations: pyo3::PyRef<'_, DurationCollectionF64>,
    ) -> pyo3::PyResult<()> {
        (self.inner.as_mut().ok_or_else(consumed)?).replace(
            streams.inner.as_ref().ok_or_else(consumed)?,
            durations.inner.as_ref().ok_or_else(consumed)?,
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
        Ok(ModelF64 {
            inner: Some((self.inner.as_ref().ok_or_else(consumed)?).cloned()),
        })
    }
    #[pyo3(name = "prepare")]
    #[pyo3(signature = (options))]
    pub fn binding_prepare(
        &self,
        options: pyo3::PyRef<'_, PreparationOptionsF64>,
    ) -> pyo3::PyResult<PreparedModelF64> {
        ((self.inner.as_ref().ok_or_else(consumed)?)
            .prepare(options.inner.as_ref().ok_or_else(consumed)?))
        .map(|value| PreparedModelF64 { inner: Some(value) })
    }
    #[pyo3(name = "prepare_owned")]
    #[pyo3(signature = (options))]
    pub fn binding_prepare_owned(
        &mut self,
        options: pyo3::PyRef<'_, PreparationOptionsF64>,
    ) -> pyo3::PyResult<PreparedModelF64> {
        ((self.inner.take().ok_or_else(consumed)?)
            .prepare_owned(options.inner.as_ref().ok_or_else(consumed)?))
        .map(|value| PreparedModelF64 { inner: Some(value) })
    }
    #[pyo3(name = "prepare_selected_durations")]
    #[pyo3(signature = (options, states))]
    pub fn binding_prepare_selected_durations(
        &self,
        options: pyo3::PyRef<'_, PreparationOptionsF64>,
        states: Vec<u32>,
    ) -> pyo3::PyResult<PreparedModelF64> {
        ((self.inner.as_ref().ok_or_else(consumed)?)
            .prepare_selected_durations(options.inner.as_ref().ok_or_else(consumed)?, &states))
        .map(|value| PreparedModelF64 { inner: Some(value) })
    }
    #[pyo3(name = "prepare_owned_selected_durations")]
    #[pyo3(signature = (options, states))]
    pub fn binding_prepare_owned_selected_durations(
        &mut self,
        options: pyo3::PyRef<'_, PreparationOptionsF64>,
        states: Vec<u32>,
    ) -> pyo3::PyResult<PreparedModelF64> {
        ((self.inner.take().ok_or_else(consumed)?).prepare_owned_selected_durations(
            options.inner.as_ref().ok_or_else(consumed)?,
            &states,
        ))
        .map(|value| PreparedModelF64 { inner: Some(value) })
    }
}
#[pymethods]
impl ModelF64 {
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
impl ModelF64 {
    #[pyo3(name = "generate_with_uniform_values")]
    #[pyo3(signature = (sequence, values))]
    pub fn binding_generate_with_uniform_values(
        &self,
        mut sequence: pyo3::PyRefMut<'_, SegmentationF64>,
        values: Vec<f64>,
    ) -> pyo3::PyResult<GeneratedObservationF64> {
        ((self.inner.as_ref().ok_or_else(consumed)?)
            .generate_with_uniform_values(sequence.inner.as_mut().ok_or_else(consumed)?, &values))
        .map(|value| GeneratedObservationF64 { inner: Some(value) })
    }
}
#[pymethods]
impl ModelF64 {
    #[pyo3(name = "forward_geometric")]
    #[pyo3(signature = (segmentation, emissions, options))]
    pub fn binding_forward_geometric(
        &self,
        segmentation: pyo3::PyRef<'_, SegmentationF64>,
        emissions: pyo3::PyRef<'_, LogEmissionsF64>,
        options: pyo3::PyRef<'_, GeometricOptionsF64>,
    ) -> pyo3::PyResult<StateScoresF64> {
        ((self.inner.as_ref().ok_or_else(consumed)?).forward_geometric(
            segmentation.inner.as_ref().ok_or_else(consumed)?,
            emissions.inner.as_ref().ok_or_else(consumed)?,
            options.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| StateScoresF64 { inner: Some(value) })
    }
    #[pyo3(name = "backward_geometric")]
    #[pyo3(signature = (segmentation, emissions, options))]
    pub fn binding_backward_geometric(
        &self,
        segmentation: pyo3::PyRef<'_, SegmentationF64>,
        emissions: pyo3::PyRef<'_, LogEmissionsF64>,
        options: pyo3::PyRef<'_, GeometricOptionsF64>,
    ) -> pyo3::PyResult<StateScoresF64> {
        ((self.inner.as_ref().ok_or_else(consumed)?).backward_geometric(
            segmentation.inner.as_ref().ok_or_else(consumed)?,
            emissions.inner.as_ref().ok_or_else(consumed)?,
            options.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| StateScoresF64 { inner: Some(value) })
    }
    #[pyo3(name = "viterbi_geometric")]
    #[pyo3(signature = (segmentation, emissions, options))]
    pub fn binding_viterbi_geometric(
        &self,
        segmentation: pyo3::PyRef<'_, SegmentationF64>,
        emissions: pyo3::PyRef<'_, LogEmissionsF64>,
        options: pyo3::PyRef<'_, GeometricOptionsF64>,
    ) -> pyo3::PyResult<GeometricPathF64> {
        ((self.inner.as_ref().ok_or_else(consumed)?).viterbi_geometric(
            segmentation.inner.as_ref().ok_or_else(consumed)?,
            emissions.inner.as_ref().ok_or_else(consumed)?,
            options.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| GeometricPathF64 { inner: Some(value) })
    }
}
#[pymethods]
impl ModelF64 {
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
#[pymethods]
impl ModelF64 {
    #[pyo3(name = "geometric_transition_occupancies")]
    #[pyo3(signature = (segmentation, emissions, forward, backward, total, options))]
    pub fn binding_geometric_transition_occupancies(
        &self,
        segmentation: pyo3::PyRef<'_, SegmentationF64>,
        emissions: pyo3::PyRef<'_, LogEmissionsF64>,
        forward: pyo3::PyRef<'_, StateScoresF64>,
        backward: pyo3::PyRef<'_, StateScoresF64>,
        total: f64,
        options: pyo3::PyRef<'_, GeometricOptionsF64>,
    ) -> pyo3::PyResult<TransitionPosteriors> {
        ((self.inner.as_ref().ok_or_else(consumed)?).geometric_transition_occupancies(
            segmentation.inner.as_ref().ok_or_else(consumed)?,
            emissions.inner.as_ref().ok_or_else(consumed)?,
            forward.inner.as_ref().ok_or_else(consumed)?,
            backward.inner.as_ref().ok_or_else(consumed)?,
            total,
            options.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| TransitionPosteriors { inner: Some(value) })
    }
    #[pyo3(name = "duration_occupancies")]
    #[pyo3(signature = (segmentation, emissions, forward, backward, total, options))]
    pub fn binding_duration_occupancies(
        &self,
        segmentation: pyo3::PyRef<'_, SegmentationF64>,
        emissions: pyo3::PyRef<'_, LogEmissionsF64>,
        forward: pyo3::PyRef<'_, HsmmScoresF64>,
        backward: pyo3::PyRef<'_, HsmmScoresF64>,
        total: f64,
        options: pyo3::PyRef<'_, HsmmOptionsF64>,
    ) -> pyo3::PyResult<DurationPosteriors> {
        ((self.inner.as_ref().ok_or_else(consumed)?).duration_occupancies(
            segmentation.inner.as_ref().ok_or_else(consumed)?,
            emissions.inner.as_ref().ok_or_else(consumed)?,
            forward.inner.as_ref().ok_or_else(consumed)?,
            backward.inner.as_ref().ok_or_else(consumed)?,
            total,
            options.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| DurationPosteriors { inner: Some(value) })
    }
    #[pyo3(name = "duration_occupancies_with_graph")]
    #[pyo3(signature = (graph, emissions, forward, backward, total, options))]
    pub fn binding_duration_occupancies_with_graph(
        &self,
        graph: pyo3::PyRef<'_, TransitionGraphF64>,
        emissions: pyo3::PyRef<'_, LogEmissionsF64>,
        forward: pyo3::PyRef<'_, HsmmScoresF64>,
        backward: pyo3::PyRef<'_, HsmmScoresF64>,
        total: f64,
        options: pyo3::PyRef<'_, HsmmOptionsF64>,
    ) -> pyo3::PyResult<DurationPosteriors> {
        ((self.inner.as_ref().ok_or_else(consumed)?).duration_occupancies_with_graph(
            graph.inner.as_ref().ok_or_else(consumed)?,
            emissions.inner.as_ref().ok_or_else(consumed)?,
            forward.inner.as_ref().ok_or_else(consumed)?,
            backward.inner.as_ref().ok_or_else(consumed)?,
            total,
            options.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| DurationPosteriors { inner: Some(value) })
    }
    #[pyo3(name = "hsmm_state_occupancies")]
    #[pyo3(signature = (segmentation, emissions, duration_occupancies, options))]
    pub fn binding_hsmm_state_occupancies(
        &self,
        segmentation: pyo3::PyRef<'_, SegmentationF64>,
        emissions: pyo3::PyRef<'_, LogEmissionsF64>,
        duration_occupancies: pyo3::PyRef<'_, DurationPosteriors>,
        options: pyo3::PyRef<'_, HsmmOptionsF64>,
    ) -> pyo3::PyResult<StateOccupanciesF64> {
        ((self.inner.as_ref().ok_or_else(consumed)?).hsmm_state_occupancies(
            segmentation.inner.as_ref().ok_or_else(consumed)?,
            emissions.inner.as_ref().ok_or_else(consumed)?,
            duration_occupancies.inner.as_ref().ok_or_else(consumed)?,
            options.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| StateOccupanciesF64 { inner: Some(value) })
    }
    #[pyo3(name = "mixture_occupancies")]
    #[pyo3(
        signature = (
            observation,
            segmentation,
            occupancy,
            stream_index,
            temperature,
            mode
        )
    )]
    pub fn binding_mixture_occupancies(
        &self,
        observation: pyo3::PyRef<'_, ObservationF64>,
        segmentation: pyo3::PyRef<'_, SegmentationF64>,
        occupancy: pyo3::PyRef<'_, StateOccupanciesF64>,
        stream_index: usize,
        temperature: f64,
        mode: pyo3::PyRef<'_, MixtureOccupancyModeF64>,
    ) -> pyo3::PyResult<MixturePosteriors> {
        ((self.inner.as_ref().ok_or_else(consumed)?).mixture_occupancies(
            observation.inner.as_ref().ok_or_else(consumed)?,
            segmentation.inner.as_ref().ok_or_else(consumed)?,
            occupancy.inner.as_ref().ok_or_else(consumed)?,
            stream_index,
            temperature,
            mode.inner.as_ref().ok_or_else(consumed)?,
        ))
        .map(|value| MixturePosteriors { inner: Some(value) })
    }
}
