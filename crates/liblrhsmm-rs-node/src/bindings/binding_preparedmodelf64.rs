use super::*;
#[napi]
pub struct PreparedModelF64 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::PreparedModelF64>>,
}
#[napi]
impl PreparedModelF64 {
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
impl PreparedModelF64 {
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(PreparedModelF64 {
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
    #[napi(js_name = "parameters")]
    pub fn binding_parameters(&self) -> napi::Result<ModelF64> {
        Ok(ModelF64 {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .parameters(),
            )),
        })
    }
    #[napi(js_name = "into_parameters")]
    pub fn binding_into_parameters(&self) -> napi::Result<ModelF64> {
        Ok(ModelF64 {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow_mut()
                    .map_err(borrowed)?
                    .take()
                    .ok_or_else(consumed)?)
                .into_parameters(),
            )),
        })
    }
    #[napi(js_name = "options")]
    pub fn binding_options(&self) -> napi::Result<PreparationOptionsF64> {
        Ok(PreparationOptionsF64 {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .options(),
            )),
        })
    }
    #[napi(js_name = "duration")]
    pub fn binding_duration(&self, state: f64) -> napi::Result<Option<PreparedDurationF64>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .duration(checked_usize(state)?))
        .map(|value| PreparedDurationF64 {
            inner: std::cell::RefCell::new(Some(value)),
        }))
    }
    #[napi(js_name = "emission")]
    pub fn binding_emission(
        &self,
        stream: f64,
        state: f64,
    ) -> napi::Result<Option<PreparedGaussianF64>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .emission(checked_usize(stream)?, checked_usize(state)?))
        .map(|value| PreparedGaussianF64 {
            inner: std::cell::RefCell::new(Some(value)),
        }))
    }
}
#[napi]
impl PreparedModelF64 {
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
impl PreparedModelF64 {
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
