use super::*;
use crate::mixture_occupancy::MixtureOccupancySettings;
use crate::occupancy::StateOccupancyTable;
use napi::Error as BindingError;
fn error(value: impl std::fmt::Display) -> BindingError {
    napi::Error::from_reason(value.to_string())
}
#[derive(Clone)]
pub struct StateOccupanciesF32 {
    pub(crate) inner: StateOccupancyTable<f32>,
}
impl StateOccupanciesF32 {
    pub fn new(frames: usize, states: usize, values: &[f32]) -> Result<Self, BindingError> {
        StateOccupancyTable::from_values(frames, states, values.to_vec())
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn frames(&self) -> usize {
        self.inner.frames()
    }
    pub fn states(&self) -> usize {
        self.inner.states()
    }
    pub fn values(&self) -> Vec<f32> {
        self.inner.values().to_vec()
    }
    pub fn get(&self, frame: usize, state: usize) -> Option<f32> {
        self.inner.get(frame, state)
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
/// Mode 0 applies an HSMM state radius; mode 1 applies an HMM pruning slope.
#[derive(Clone)]
pub struct MixtureOccupancyModeF32 {
    pub mode: u32,
    pub parameter: f32,
}
impl MixtureOccupancyModeF32 {
    pub(super) fn native(&self) -> Result<MixtureOccupancySettings<f32>, BindingError> {
        match self.mode {
            0 => Ok(MixtureOccupancySettings::Hsmm {
                state_radius: self.parameter,
            }),
            1 => Ok(MixtureOccupancySettings::Geometric {
                pruning_slope: self.parameter,
            }),
            _ => Err(error("unknown mixture occupancy mode")),
        }
    }
}
impl MixtureOccupancyModeF32 {
    pub fn new(mode: u32, parameter: f32) -> Self {
        Self { mode, parameter }
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
impl StateScoresF32 {
    pub fn geometric_occupancies(
        &self,
        backward: &StateScoresF32,
        total: f32,
        pruning_slope: f32,
    ) -> Result<StateOccupanciesF32, BindingError> {
        self.inner
            .geometric_occupancies(&backward.inner, total, pruning_slope)
            .map(|inner| StateOccupanciesF32 { inner })
            .map_err(error)
    }
}
impl ModelF32 {
    pub fn geometric_transition_occupancies(
        &self,
        segmentation: &SegmentationF32,
        emissions: &LogEmissionsF32,
        forward: &StateScoresF32,
        backward: &StateScoresF32,
        total: f32,
        options: &GeometricOptionsF32,
    ) -> Result<TransitionPosteriors, BindingError> {
        self.inner
            .geometric_transition_occupancies(
                &segmentation.inner,
                &emissions.inner,
                &forward.inner,
                &backward.inner,
                total,
                options.native(),
            )
            .map(|inner| TransitionPosteriors { inner })
            .map_err(error)
    }
    pub fn duration_occupancies(
        &self,
        segmentation: &SegmentationF32,
        emissions: &LogEmissionsF32,
        forward: &HsmmScoresF32,
        backward: &HsmmScoresF32,
        total: f32,
        options: &HsmmOptionsF32,
    ) -> Result<DurationPosteriors, BindingError> {
        self.inner
            .duration_occupancies(
                &segmentation.inner,
                &emissions.inner,
                &forward.inner,
                &backward.inner,
                total,
                options.native(),
            )
            .map(|inner| DurationPosteriors { inner })
            .map_err(error)
    }
    pub fn duration_occupancies_with_graph(
        &self,
        graph: &TransitionGraphF32,
        emissions: &LogEmissionsF32,
        forward: &HsmmScoresF32,
        backward: &HsmmScoresF32,
        total: f32,
        options: &HsmmOptionsF32,
    ) -> Result<DurationPosteriors, BindingError> {
        self.inner
            .duration_occupancies_with_graph(
                graph.inner.as_view(),
                &emissions.inner,
                &forward.inner,
                &backward.inner,
                total,
                options.native(),
            )
            .map(|inner| DurationPosteriors { inner })
            .map_err(error)
    }
    pub fn hsmm_state_occupancies(
        &self,
        segmentation: &SegmentationF32,
        emissions: &LogEmissionsF32,
        duration_occupancies: &DurationPosteriors,
        options: &HsmmOptionsF32,
    ) -> Result<StateOccupanciesF32, BindingError> {
        self.inner
            .hsmm_state_occupancies(
                &segmentation.inner,
                &emissions.inner,
                &duration_occupancies.inner,
                options.native(),
            )
            .map(|inner| StateOccupanciesF32 { inner })
            .map_err(error)
    }
    pub fn mixture_occupancies(
        &self,
        observation: &ObservationF32,
        segmentation: &SegmentationF32,
        occupancy: &StateOccupanciesF32,
        stream_index: usize,
        temperature: f32,
        mode: &MixtureOccupancyModeF32,
    ) -> Result<MixturePosteriors, BindingError> {
        self.inner
            .mixture_occupancies(
                &observation.inner,
                &segmentation.inner,
                &occupancy.inner,
                stream_index,
                temperature,
                mode.native()?,
            )
            .map(|inner| MixturePosteriors { inner })
            .map_err(error)
    }
}
#[derive(Clone)]
pub struct StateOccupanciesF64 {
    pub(crate) inner: StateOccupancyTable<f64>,
}
impl StateOccupanciesF64 {
    pub fn new(frames: usize, states: usize, values: &[f64]) -> Result<Self, BindingError> {
        StateOccupancyTable::from_values(frames, states, values.to_vec())
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn frames(&self) -> usize {
        self.inner.frames()
    }
    pub fn states(&self) -> usize {
        self.inner.states()
    }
    pub fn values(&self) -> Vec<f64> {
        self.inner.values().to_vec()
    }
    pub fn get(&self, frame: usize, state: usize) -> Option<f64> {
        self.inner.get(frame, state)
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
/// Mode 0 applies an HSMM state radius; mode 1 applies an HMM pruning slope.
#[derive(Clone)]
pub struct MixtureOccupancyModeF64 {
    pub mode: u32,
    pub parameter: f64,
}
impl MixtureOccupancyModeF64 {
    pub(super) fn native(&self) -> Result<MixtureOccupancySettings<f64>, BindingError> {
        match self.mode {
            0 => Ok(MixtureOccupancySettings::Hsmm {
                state_radius: self.parameter,
            }),
            1 => Ok(MixtureOccupancySettings::Geometric {
                pruning_slope: self.parameter,
            }),
            _ => Err(error("unknown mixture occupancy mode")),
        }
    }
}
impl MixtureOccupancyModeF64 {
    pub fn new(mode: u32, parameter: f64) -> Self {
        Self { mode, parameter }
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
impl StateScoresF64 {
    pub fn geometric_occupancies(
        &self,
        backward: &StateScoresF64,
        total: f64,
        pruning_slope: f64,
    ) -> Result<StateOccupanciesF64, BindingError> {
        self.inner
            .geometric_occupancies(&backward.inner, total, pruning_slope)
            .map(|inner| StateOccupanciesF64 { inner })
            .map_err(error)
    }
}
impl ModelF64 {
    pub fn geometric_transition_occupancies(
        &self,
        segmentation: &SegmentationF64,
        emissions: &LogEmissionsF64,
        forward: &StateScoresF64,
        backward: &StateScoresF64,
        total: f64,
        options: &GeometricOptionsF64,
    ) -> Result<TransitionPosteriors, BindingError> {
        self.inner
            .geometric_transition_occupancies(
                &segmentation.inner,
                &emissions.inner,
                &forward.inner,
                &backward.inner,
                total,
                options.native(),
            )
            .map(|inner| TransitionPosteriors { inner })
            .map_err(error)
    }
    pub fn duration_occupancies(
        &self,
        segmentation: &SegmentationF64,
        emissions: &LogEmissionsF64,
        forward: &HsmmScoresF64,
        backward: &HsmmScoresF64,
        total: f64,
        options: &HsmmOptionsF64,
    ) -> Result<DurationPosteriors, BindingError> {
        self.inner
            .duration_occupancies(
                &segmentation.inner,
                &emissions.inner,
                &forward.inner,
                &backward.inner,
                total,
                options.native(),
            )
            .map(|inner| DurationPosteriors { inner })
            .map_err(error)
    }
    pub fn duration_occupancies_with_graph(
        &self,
        graph: &TransitionGraphF64,
        emissions: &LogEmissionsF64,
        forward: &HsmmScoresF64,
        backward: &HsmmScoresF64,
        total: f64,
        options: &HsmmOptionsF64,
    ) -> Result<DurationPosteriors, BindingError> {
        self.inner
            .duration_occupancies_with_graph(
                graph.inner.as_view(),
                &emissions.inner,
                &forward.inner,
                &backward.inner,
                total,
                options.native(),
            )
            .map(|inner| DurationPosteriors { inner })
            .map_err(error)
    }
    pub fn hsmm_state_occupancies(
        &self,
        segmentation: &SegmentationF64,
        emissions: &LogEmissionsF64,
        duration_occupancies: &DurationPosteriors,
        options: &HsmmOptionsF64,
    ) -> Result<StateOccupanciesF64, BindingError> {
        self.inner
            .hsmm_state_occupancies(
                &segmentation.inner,
                &emissions.inner,
                &duration_occupancies.inner,
                options.native(),
            )
            .map(|inner| StateOccupanciesF64 { inner })
            .map_err(error)
    }
    pub fn mixture_occupancies(
        &self,
        observation: &ObservationF64,
        segmentation: &SegmentationF64,
        occupancy: &StateOccupanciesF64,
        stream_index: usize,
        temperature: f64,
        mode: &MixtureOccupancyModeF64,
    ) -> Result<MixturePosteriors, BindingError> {
        self.inner
            .mixture_occupancies(
                &observation.inner,
                &segmentation.inner,
                &occupancy.inner,
                stream_index,
                temperature,
                mode.native()?,
            )
            .map(|inner| MixturePosteriors { inner })
            .map_err(error)
    }
}
