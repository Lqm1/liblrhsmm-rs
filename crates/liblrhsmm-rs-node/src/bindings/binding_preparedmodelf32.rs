use super::*;
#[napi]
pub struct PreparedModelF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::PreparedModelF32>>,
}
#[napi]
impl PreparedModelF32 {
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
impl PreparedModelF32 {
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(PreparedModelF32 {
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
    pub fn binding_parameters(&self) -> napi::Result<ModelF32> {
        Ok(ModelF32 {
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
    pub fn binding_into_parameters(&self) -> napi::Result<ModelF32> {
        Ok(ModelF32 {
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
    pub fn binding_options(&self) -> napi::Result<PreparationOptionsF32> {
        Ok(PreparationOptionsF32 {
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
    pub fn binding_duration(&self, state: f64) -> napi::Result<Option<PreparedDurationF32>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .duration(checked_usize(state)?))
        .map(|value| PreparedDurationF32 {
            inner: std::cell::RefCell::new(Some(value)),
        }))
    }
    #[napi(js_name = "emission")]
    pub fn binding_emission(
        &self,
        stream: f64,
        state: f64,
    ) -> napi::Result<Option<PreparedGaussianF32>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .emission(checked_usize(stream)?, checked_usize(state)?))
        .map(|value| PreparedGaussianF32 {
            inner: std::cell::RefCell::new(Some(value)),
        }))
    }
}
#[napi]
impl PreparedModelF32 {
    #[napi(js_name = "emission_log_probabilities")]
    pub fn binding_emission_log_probabilities(
        &self,
        observation: &ObservationF32,
        segmentation: &SegmentationF32,
        temperature: f64,
        pruning: &EmissionPruningF32,
    ) -> napi::Result<LogEmissionsF32> {
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
            temperature as f32,
            pruning
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| LogEmissionsF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "emission_probabilities")]
    pub fn binding_emission_probabilities(
        &self,
        observation: &ObservationF32,
        segmentation: &SegmentationF32,
        temperature: f64,
        pruning: &EmissionPruningF32,
    ) -> napi::Result<EmissionDensitiesF32> {
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
            temperature as f32,
            pruning
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        ))
        .map(|value| EmissionDensitiesF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
}
#[napi]
impl PreparedModelF32 {
    #[napi(js_name = "forward_hsmm")]
    pub fn binding_forward_hsmm(
        &self,
        segmentation: &SegmentationF32,
        emissions: &LogEmissionsF32,
        options: &HsmmOptionsF32,
    ) -> napi::Result<HsmmScoresF32> {
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
        .map(|value| HsmmScoresF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "backward_hsmm")]
    pub fn binding_backward_hsmm(
        &self,
        segmentation: &SegmentationF32,
        emissions: &LogEmissionsF32,
        options: &HsmmOptionsF32,
    ) -> napi::Result<HsmmScoresF32> {
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
        .map(|value| HsmmScoresF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "viterbi_hsmm")]
    pub fn binding_viterbi_hsmm(
        &self,
        segmentation: &SegmentationF32,
        emissions: &LogEmissionsF32,
        options: &HsmmOptionsF32,
    ) -> napi::Result<HsmmPathF32> {
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
        .map(|value| HsmmPathF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "forward_hsmm_with_graph")]
    pub fn binding_forward_hsmm_with_graph(
        &self,
        graph: &TransitionGraphF32,
        emissions: &LogEmissionsF32,
        options: &HsmmOptionsF32,
    ) -> napi::Result<HsmmScoresF32> {
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
        .map(|value| HsmmScoresF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "backward_hsmm_with_graph")]
    pub fn binding_backward_hsmm_with_graph(
        &self,
        graph: &TransitionGraphF32,
        emissions: &LogEmissionsF32,
        options: &HsmmOptionsF32,
    ) -> napi::Result<HsmmScoresF32> {
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
        .map(|value| HsmmScoresF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "viterbi_hsmm_with_graph")]
    pub fn binding_viterbi_hsmm_with_graph(
        &self,
        graph: &TransitionGraphF32,
        emissions: &LogEmissionsF32,
        options: &HsmmOptionsF32,
    ) -> napi::Result<HsmmPathF32> {
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
        .map(|value| HsmmPathF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
}
