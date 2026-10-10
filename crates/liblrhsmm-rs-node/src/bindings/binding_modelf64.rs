use super::*;
#[napi]
pub struct ModelF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::ModelF64>>,
}
#[cfg(feature = "serialization")]
#[napi]
impl ModelF64 {
    #[napi(js_name = "read")]
    pub fn binding_read(bytes: napi::bindgen_prelude::Uint8Array) -> napi::Result<Self> {
        let storage_bytes = bytes.to_vec();
        (crate::api::ModelF64::read(&storage_bytes)).map(|value| ModelF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "read_with_limits")]
    pub fn binding_read_with_limits(
        bytes: napi::bindgen_prelude::Uint8Array,
        max_array_entries: f64,
    ) -> napi::Result<Self> {
        let storage_bytes = bytes.to_vec();
        (crate::api::ModelF64::read_with_limits(&storage_bytes, checked_usize(max_array_entries)?))
            .map(|value| ModelF64 {
                inner: std::cell::RefCell::new(Some(value)),
            })
    }
    #[napi(js_name = "read_prefix")]
    pub fn binding_read_prefix(
        bytes: napi::bindgen_prelude::Uint8Array,
    ) -> napi::Result<DecodedModelF64> {
        let storage_bytes = bytes.to_vec();
        (crate::api::ModelF64::read_prefix(&storage_bytes)).map(|value| DecodedModelF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "read_prefix_with_limits")]
    pub fn binding_read_prefix_with_limits(
        bytes: napi::bindgen_prelude::Uint8Array,
        max_array_entries: f64,
    ) -> napi::Result<DecodedModelF64> {
        let storage_bytes = bytes.to_vec();
        (crate::api::ModelF64::read_prefix_with_limits(
            &storage_bytes,
            checked_usize(max_array_entries)?,
        ))
        .map(|value| DecodedModelF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "write")]
    pub fn binding_write(&self) -> napi::Result<napi::bindgen_prelude::Uint8Array> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .write())
        .map(<napi::bindgen_prelude::Uint8Array>::from)
    }
    #[napi(js_name = "write_with_encoding")]
    pub fn binding_write_with_encoding(
        &self,
        code: u32,
    ) -> napi::Result<napi::bindgen_prelude::Uint8Array> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .write_with_encoding(code))
        .map(<napi::bindgen_prelude::Uint8Array>::from)
    }
}
#[napi]
impl ModelF64 {
    #[napi(js_name = "update")]
    pub fn binding_update(
        &self,
        statistics: &ModelStatisticsF64,
        options: &UpdateOptionsF64,
    ) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .update(
            statistics
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            options
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        )
    }
}
#[napi]
impl ModelF64 {
    #[napi]
    pub fn close(&self) -> napi::Result<()> {
        *self.inner.try_borrow_mut().map_err(borrowed)? = None;
        Ok(())
    }
    #[napi]
    pub fn free(&self) -> napi::Result<()> {
        self.close()
    }
    #[napi(getter, js_name = "is_closed")]
    pub fn is_closed(&self) -> napi::Result<bool> {
        Ok(self.inner.try_borrow().map_err(borrowed)?.is_none())
    }
}
#[napi]
impl ModelF64 {
    #[napi(constructor)]
    pub fn binding_new(
        layouts: napi::bindgen_prelude::Uint32Array,
        durations: f64,
    ) -> napi::Result<Self> {
        let storage_layouts = layouts.to_vec();
        (crate::api::ModelF64::new(&storage_layouts, checked_usize(durations)?)).map(|value| {
            ModelF64 {
                inner: std::cell::RefCell::new(Some(value)),
            }
        })
    }
    #[napi(js_name = "empty")]
    pub fn binding_empty() -> napi::Result<Self> {
        Ok(ModelF64 {
            inner: std::cell::RefCell::new(Some(crate::api::ModelF64::empty())),
        })
    }
    #[napi(getter, js_name = "streams")]
    pub fn binding_streams(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .streams() as f64)
    }
    #[napi(getter, js_name = "durations")]
    pub fn binding_durations(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .durations() as f64)
    }
    #[napi(js_name = "stream")]
    pub fn binding_stream(&self, index: f64) -> napi::Result<Option<StreamF64>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .stream(checked_usize(index)?))
        .map(|value| StreamF64 {
            inner: std::cell::RefCell::new(Some(value)),
        }))
    }
    #[napi(js_name = "duration")]
    pub fn binding_duration(&self, index: f64) -> napi::Result<Option<DurationF64>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .duration(checked_usize(index)?))
        .map(|value| DurationF64 {
            inner: std::cell::RefCell::new(Some(value)),
        }))
    }
    #[napi(js_name = "set_stream")]
    pub fn binding_set_stream(&self, index: f64, value: &StreamF64) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_stream(
            checked_usize(index)?,
            value
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        )
    }
    #[napi(js_name = "set_duration")]
    pub fn binding_set_duration(&self, index: f64, value: &DurationF64) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .set_duration(
            checked_usize(index)?,
            value
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        )
    }
    #[napi(js_name = "replace")]
    pub fn binding_replace(
        &self,
        streams: &StreamCollectionF64,
        durations: &DurationCollectionF64,
    ) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .replace(
            streams
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            durations
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        )
    }
    #[napi(js_name = "validate")]
    pub fn binding_validate(&self) -> napi::Result<()> {
        (self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .validate()
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(ModelF64 {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .cloned(),
            )),
        })
    }
    #[napi(js_name = "prepare")]
    pub fn binding_prepare(
        &self,
        options: &PreparationOptionsF64,
    ) -> napi::Result<PreparedModelF64> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .prepare(
            options
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| PreparedModelF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "prepare_owned")]
    pub fn binding_prepare_owned(
        &self,
        options: &PreparationOptionsF64,
    ) -> napi::Result<PreparedModelF64> {
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .take()
            .ok_or_else(consumed)?)
        .prepare_owned(
            options
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| PreparedModelF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "prepare_selected_durations")]
    pub fn binding_prepare_selected_durations(
        &self,
        options: &PreparationOptionsF64,
        states: napi::bindgen_prelude::Uint32Array,
    ) -> napi::Result<PreparedModelF64> {
        let storage_states = states.to_vec();
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .prepare_selected_durations(
            options
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            &storage_states,
        ))
        .map(|value| PreparedModelF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "prepare_owned_selected_durations")]
    pub fn binding_prepare_owned_selected_durations(
        &self,
        options: &PreparationOptionsF64,
        states: napi::bindgen_prelude::Uint32Array,
    ) -> napi::Result<PreparedModelF64> {
        let storage_states = states.to_vec();
        ((self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .take()
            .ok_or_else(consumed)?)
        .prepare_owned_selected_durations(
            options
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            &storage_states,
        ))
        .map(|value| PreparedModelF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
}
#[napi]
impl ModelF64 {
    #[napi(js_name = "emission_log_probabilities")]
    pub fn binding_emission_log_probabilities(
        &self,
        observation: &ObservationF64,
        segmentation: &SegmentationF64,
        temperature: f64,
        pruning: &EmissionPruningF64,
    ) -> napi::Result<LogEmissionsF64> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .emission_log_probabilities(
            observation
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            segmentation
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            temperature,
            pruning
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| LogEmissionsF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "emission_probabilities")]
    pub fn binding_emission_probabilities(
        &self,
        observation: &ObservationF64,
        segmentation: &SegmentationF64,
        temperature: f64,
        pruning: &EmissionPruningF64,
    ) -> napi::Result<EmissionDensitiesF64> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .emission_probabilities(
            observation
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            segmentation
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            temperature,
            pruning
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| EmissionDensitiesF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
}
#[napi]
impl ModelF64 {
    #[napi(js_name = "generate_with_uniform_values")]
    pub fn binding_generate_with_uniform_values(
        &self,
        sequence: &SegmentationF64,
        values: napi::bindgen_prelude::Float64Array,
    ) -> napi::Result<GeneratedObservationF64> {
        let storage_values = values.to_vec();
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .generate_with_uniform_values(
            sequence
                .inner
                .try_borrow_mut()
                .map_err(borrowed)?
                .as_mut()
                .ok_or_else(consumed)?,
            &storage_values,
        ))
        .map(|value| GeneratedObservationF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
}
#[napi]
impl ModelF64 {
    #[napi(js_name = "forward_geometric")]
    pub fn binding_forward_geometric(
        &self,
        segmentation: &SegmentationF64,
        emissions: &LogEmissionsF64,
        options: &GeometricOptionsF64,
    ) -> napi::Result<StateScoresF64> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .forward_geometric(
            segmentation
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            emissions
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            options
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| StateScoresF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "backward_geometric")]
    pub fn binding_backward_geometric(
        &self,
        segmentation: &SegmentationF64,
        emissions: &LogEmissionsF64,
        options: &GeometricOptionsF64,
    ) -> napi::Result<StateScoresF64> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .backward_geometric(
            segmentation
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            emissions
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            options
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| StateScoresF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "viterbi_geometric")]
    pub fn binding_viterbi_geometric(
        &self,
        segmentation: &SegmentationF64,
        emissions: &LogEmissionsF64,
        options: &GeometricOptionsF64,
    ) -> napi::Result<GeometricPathF64> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .viterbi_geometric(
            segmentation
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            emissions
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            options
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| GeometricPathF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
}
#[napi]
impl ModelF64 {
    #[napi(js_name = "forward_hsmm")]
    pub fn binding_forward_hsmm(
        &self,
        segmentation: &SegmentationF64,
        emissions: &LogEmissionsF64,
        options: &HsmmOptionsF64,
    ) -> napi::Result<HsmmScoresF64> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .forward_hsmm(
            segmentation
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            emissions
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            options
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| HsmmScoresF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "backward_hsmm")]
    pub fn binding_backward_hsmm(
        &self,
        segmentation: &SegmentationF64,
        emissions: &LogEmissionsF64,
        options: &HsmmOptionsF64,
    ) -> napi::Result<HsmmScoresF64> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .backward_hsmm(
            segmentation
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            emissions
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            options
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| HsmmScoresF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "viterbi_hsmm")]
    pub fn binding_viterbi_hsmm(
        &self,
        segmentation: &SegmentationF64,
        emissions: &LogEmissionsF64,
        options: &HsmmOptionsF64,
    ) -> napi::Result<HsmmPathF64> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .viterbi_hsmm(
            segmentation
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            emissions
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            options
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| HsmmPathF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "forward_hsmm_with_graph")]
    pub fn binding_forward_hsmm_with_graph(
        &self,
        graph: &TransitionGraphF64,
        emissions: &LogEmissionsF64,
        options: &HsmmOptionsF64,
    ) -> napi::Result<HsmmScoresF64> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .forward_hsmm_with_graph(
            graph
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            emissions
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            options
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| HsmmScoresF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "backward_hsmm_with_graph")]
    pub fn binding_backward_hsmm_with_graph(
        &self,
        graph: &TransitionGraphF64,
        emissions: &LogEmissionsF64,
        options: &HsmmOptionsF64,
    ) -> napi::Result<HsmmScoresF64> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .backward_hsmm_with_graph(
            graph
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            emissions
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            options
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| HsmmScoresF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "viterbi_hsmm_with_graph")]
    pub fn binding_viterbi_hsmm_with_graph(
        &self,
        graph: &TransitionGraphF64,
        emissions: &LogEmissionsF64,
        options: &HsmmOptionsF64,
    ) -> napi::Result<HsmmPathF64> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .viterbi_hsmm_with_graph(
            graph
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            emissions
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            options
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| HsmmPathF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
}
#[napi]
impl ModelF64 {
    #[napi(js_name = "geometric_transition_occupancies")]
    pub fn binding_geometric_transition_occupancies(
        &self,
        segmentation: &SegmentationF64,
        emissions: &LogEmissionsF64,
        forward: &StateScoresF64,
        backward: &StateScoresF64,
        total: f64,
        options: &GeometricOptionsF64,
    ) -> napi::Result<TransitionPosteriors> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .geometric_transition_occupancies(
            segmentation
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            emissions
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            forward
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            backward
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            total,
            options
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| TransitionPosteriors {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "duration_occupancies")]
    pub fn binding_duration_occupancies(
        &self,
        segmentation: &SegmentationF64,
        emissions: &LogEmissionsF64,
        forward: &HsmmScoresF64,
        backward: &HsmmScoresF64,
        total: f64,
        options: &HsmmOptionsF64,
    ) -> napi::Result<DurationPosteriors> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .duration_occupancies(
            segmentation
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            emissions
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            forward
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            backward
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            total,
            options
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| DurationPosteriors {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "duration_occupancies_with_graph")]
    pub fn binding_duration_occupancies_with_graph(
        &self,
        graph: &TransitionGraphF64,
        emissions: &LogEmissionsF64,
        forward: &HsmmScoresF64,
        backward: &HsmmScoresF64,
        total: f64,
        options: &HsmmOptionsF64,
    ) -> napi::Result<DurationPosteriors> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .duration_occupancies_with_graph(
            graph
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            emissions
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            forward
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            backward
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            total,
            options
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| DurationPosteriors {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "hsmm_state_occupancies")]
    pub fn binding_hsmm_state_occupancies(
        &self,
        segmentation: &SegmentationF64,
        emissions: &LogEmissionsF64,
        duration_occupancies: &DurationPosteriors,
        options: &HsmmOptionsF64,
    ) -> napi::Result<StateOccupanciesF64> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .hsmm_state_occupancies(
            segmentation
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            emissions
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            duration_occupancies
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            options
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| StateOccupanciesF64 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "mixture_occupancies")]
    pub fn binding_mixture_occupancies(
        &self,
        observation: &ObservationF64,
        segmentation: &SegmentationF64,
        occupancy: &StateOccupanciesF64,
        stream_index: f64,
        temperature: f64,
        mode: &MixtureOccupancyModeF64,
    ) -> napi::Result<MixturePosteriors> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .mixture_occupancies(
            observation
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            segmentation
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            occupancy
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
            checked_usize(stream_index)?,
            temperature,
            mode.inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| MixturePosteriors {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
}
