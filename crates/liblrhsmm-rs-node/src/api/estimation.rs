use super::*;
use crate::duration_occupancy::DurationOccupancies;
use crate::expectation::{GeometricPosteriorExpectation, HsmmPosteriorExpectation};
use crate::mixture_occupancy::MixtureOccupancies;
use crate::occupancy::{StateOccupancyTable, TransitionOccupancies};
use napi::Error as BindingError;
fn error(value: impl std::fmt::Display) -> BindingError {
    napi::Error::from_reason(value.to_string())
}
#[derive(Clone, Default)]
pub struct MixturePosteriorCollection {
    pub(crate) inner: Vec<MixtureOccupancies>,
}
impl MixturePosteriorCollection {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn length(&self) -> usize {
        self.inner.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn push(&mut self, value: &MixturePosteriors) {
        self.inner.push(value.inner.clone());
    }
    pub fn get(&self, index: usize) -> Option<MixturePosteriors> {
        self.inner
            .get(index)
            .cloned()
            .map(|inner| MixturePosteriors { inner })
    }
    pub fn replace(&mut self, index: usize, value: &MixturePosteriors) -> Result<(), BindingError> {
        let target = self
            .inner
            .get_mut(index)
            .ok_or_else(|| error("collection index out of range"))?;
        *target = value.inner.clone();
        Ok(())
    }
    pub fn clear(&mut self) {
        self.inner.clear();
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
#[derive(Clone)]
pub struct HsmmExpectationF32 {
    durations: DurationOccupancies,
    states: StateOccupancyTable<f32>,
    mixtures: Vec<MixtureOccupancies>,
    pub state_radius: f32,
}
impl HsmmExpectationF32 {
    pub(super) fn native(&self) -> HsmmPosteriorExpectation<'_, f32> {
        HsmmPosteriorExpectation {
            durations: &self.durations,
            states: &self.states,
            mixtures: &self.mixtures,
            state_radius: self.state_radius,
        }
    }
}
impl HsmmExpectationF32 {
    pub fn new(
        durations: &DurationPosteriors,
        states: &StateOccupanciesF32,
        mixtures: &MixturePosteriorCollection,
        state_radius: f32,
    ) -> Self {
        Self {
            durations: durations.inner.clone(),
            states: states.inner.clone(),
            mixtures: mixtures.inner.clone(),
            state_radius,
        }
    }
    pub fn durations(&self) -> DurationPosteriors {
        DurationPosteriors {
            inner: self.durations.clone(),
        }
    }
    pub fn set_durations(&mut self, value: &DurationPosteriors) {
        self.durations = value.inner.clone();
    }
    pub fn states(&self) -> StateOccupanciesF32 {
        StateOccupanciesF32 {
            inner: self.states.clone(),
        }
    }
    pub fn set_states(&mut self, value: &StateOccupanciesF32) {
        self.states = value.inner.clone();
    }
    pub fn mixtures(&self) -> MixturePosteriorCollection {
        MixturePosteriorCollection {
            inner: self.mixtures.clone(),
        }
    }
    pub fn set_mixtures(&mut self, value: &MixturePosteriorCollection) {
        self.mixtures = value.inner.clone();
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
#[derive(Clone)]
pub struct GeometricExpectationF32 {
    transitions: TransitionOccupancies,
    states: StateOccupancyTable<f32>,
    mixtures: Vec<MixtureOccupancies>,
    pub pruning_slope: f32,
}
impl GeometricExpectationF32 {
    pub(super) fn native(&self) -> GeometricPosteriorExpectation<'_, f32> {
        GeometricPosteriorExpectation {
            transitions: &self.transitions,
            states: &self.states,
            mixtures: &self.mixtures,
            pruning_slope: self.pruning_slope,
        }
    }
}
impl GeometricExpectationF32 {
    pub fn new(
        transitions: &TransitionPosteriors,
        states: &StateOccupanciesF32,
        mixtures: &MixturePosteriorCollection,
        pruning_slope: f32,
    ) -> Self {
        Self {
            transitions: transitions.inner.clone(),
            states: states.inner.clone(),
            mixtures: mixtures.inner.clone(),
            pruning_slope,
        }
    }
    pub fn transitions(&self) -> TransitionPosteriors {
        TransitionPosteriors {
            inner: self.transitions.clone(),
        }
    }
    pub fn set_transitions(&mut self, value: &TransitionPosteriors) {
        self.transitions = value.inner.clone();
    }
    pub fn states(&self) -> StateOccupanciesF32 {
        StateOccupanciesF32 {
            inner: self.states.clone(),
        }
    }
    pub fn set_states(&mut self, value: &StateOccupanciesF32) {
        self.states = value.inner.clone();
    }
    pub fn mixtures(&self) -> MixturePosteriorCollection {
        MixturePosteriorCollection {
            inner: self.mixtures.clone(),
        }
    }
    pub fn set_mixtures(&mut self, value: &MixturePosteriorCollection) {
        self.mixtures = value.inner.clone();
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
impl ModelStatisticsF32 {
    pub fn collect_initialization(
        &mut self,
        model: &ModelF32,
        observation: &ObservationF32,
        segmentation: &SegmentationF32,
    ) -> Result<(), BindingError> {
        self.inner
            .collect_initialization(&model.inner, &observation.inner, &segmentation.inner)
            .map_err(error)
    }
    pub fn collect_hsmm(
        &mut self,
        model: &ModelF32,
        observation: &ObservationF32,
        segmentation: &SegmentationF32,
        expectation: &HsmmExpectationF32,
    ) -> Result<(), BindingError> {
        self.inner
            .collect_hsmm(
                &model.inner,
                &observation.inner,
                &segmentation.inner,
                expectation.native(),
            )
            .map_err(error)
    }
    pub fn collect_geometric(
        &mut self,
        model: &ModelF32,
        observation: &ObservationF32,
        segmentation: &SegmentationF32,
        expectation: &GeometricExpectationF32,
    ) -> Result<(), BindingError> {
        self.inner
            .collect_geometric(
                &model.inner,
                &observation.inner,
                &segmentation.inner,
                expectation.native(),
            )
            .map_err(error)
    }
    pub fn estimate_hsmm(
        &mut self,
        model: &ModelF32,
        observation: &ObservationF32,
        segmentation: &SegmentationF32,
        options: &HsmmOptionsF32,
    ) -> Result<f32, BindingError> {
        self.inner
            .estimate_hsmm(
                &model.inner,
                &observation.inner,
                &segmentation.inner,
                options.native(),
            )
            .map_err(error)
    }
    pub fn estimate_geometric(
        &mut self,
        model: &ModelF32,
        observation: &ObservationF32,
        segmentation: &SegmentationF32,
        options: &GeometricOptionsF32,
    ) -> Result<f32, BindingError> {
        self.inner
            .estimate_geometric(
                &model.inner,
                &observation.inner,
                &segmentation.inner,
                options.native(),
            )
            .map_err(error)
    }
}
#[derive(Clone)]
pub struct HsmmExpectationF64 {
    durations: DurationOccupancies,
    states: StateOccupancyTable<f64>,
    mixtures: Vec<MixtureOccupancies>,
    pub state_radius: f64,
}
impl HsmmExpectationF64 {
    pub(super) fn native(&self) -> HsmmPosteriorExpectation<'_, f64> {
        HsmmPosteriorExpectation {
            durations: &self.durations,
            states: &self.states,
            mixtures: &self.mixtures,
            state_radius: self.state_radius,
        }
    }
}
impl HsmmExpectationF64 {
    pub fn new(
        durations: &DurationPosteriors,
        states: &StateOccupanciesF64,
        mixtures: &MixturePosteriorCollection,
        state_radius: f64,
    ) -> Self {
        Self {
            durations: durations.inner.clone(),
            states: states.inner.clone(),
            mixtures: mixtures.inner.clone(),
            state_radius,
        }
    }
    pub fn durations(&self) -> DurationPosteriors {
        DurationPosteriors {
            inner: self.durations.clone(),
        }
    }
    pub fn set_durations(&mut self, value: &DurationPosteriors) {
        self.durations = value.inner.clone();
    }
    pub fn states(&self) -> StateOccupanciesF64 {
        StateOccupanciesF64 {
            inner: self.states.clone(),
        }
    }
    pub fn set_states(&mut self, value: &StateOccupanciesF64) {
        self.states = value.inner.clone();
    }
    pub fn mixtures(&self) -> MixturePosteriorCollection {
        MixturePosteriorCollection {
            inner: self.mixtures.clone(),
        }
    }
    pub fn set_mixtures(&mut self, value: &MixturePosteriorCollection) {
        self.mixtures = value.inner.clone();
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
#[derive(Clone)]
pub struct GeometricExpectationF64 {
    transitions: TransitionOccupancies,
    states: StateOccupancyTable<f64>,
    mixtures: Vec<MixtureOccupancies>,
    pub pruning_slope: f64,
}
impl GeometricExpectationF64 {
    pub(super) fn native(&self) -> GeometricPosteriorExpectation<'_, f64> {
        GeometricPosteriorExpectation {
            transitions: &self.transitions,
            states: &self.states,
            mixtures: &self.mixtures,
            pruning_slope: self.pruning_slope,
        }
    }
}
impl GeometricExpectationF64 {
    pub fn new(
        transitions: &TransitionPosteriors,
        states: &StateOccupanciesF64,
        mixtures: &MixturePosteriorCollection,
        pruning_slope: f64,
    ) -> Self {
        Self {
            transitions: transitions.inner.clone(),
            states: states.inner.clone(),
            mixtures: mixtures.inner.clone(),
            pruning_slope,
        }
    }
    pub fn transitions(&self) -> TransitionPosteriors {
        TransitionPosteriors {
            inner: self.transitions.clone(),
        }
    }
    pub fn set_transitions(&mut self, value: &TransitionPosteriors) {
        self.transitions = value.inner.clone();
    }
    pub fn states(&self) -> StateOccupanciesF64 {
        StateOccupanciesF64 {
            inner: self.states.clone(),
        }
    }
    pub fn set_states(&mut self, value: &StateOccupanciesF64) {
        self.states = value.inner.clone();
    }
    pub fn mixtures(&self) -> MixturePosteriorCollection {
        MixturePosteriorCollection {
            inner: self.mixtures.clone(),
        }
    }
    pub fn set_mixtures(&mut self, value: &MixturePosteriorCollection) {
        self.mixtures = value.inner.clone();
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
impl ModelStatisticsF64 {
    pub fn collect_initialization(
        &mut self,
        model: &ModelF64,
        observation: &ObservationF64,
        segmentation: &SegmentationF64,
    ) -> Result<(), BindingError> {
        self.inner
            .collect_initialization(&model.inner, &observation.inner, &segmentation.inner)
            .map_err(error)
    }
    pub fn collect_hsmm(
        &mut self,
        model: &ModelF64,
        observation: &ObservationF64,
        segmentation: &SegmentationF64,
        expectation: &HsmmExpectationF64,
    ) -> Result<(), BindingError> {
        self.inner
            .collect_hsmm(
                &model.inner,
                &observation.inner,
                &segmentation.inner,
                expectation.native(),
            )
            .map_err(error)
    }
    pub fn collect_geometric(
        &mut self,
        model: &ModelF64,
        observation: &ObservationF64,
        segmentation: &SegmentationF64,
        expectation: &GeometricExpectationF64,
    ) -> Result<(), BindingError> {
        self.inner
            .collect_geometric(
                &model.inner,
                &observation.inner,
                &segmentation.inner,
                expectation.native(),
            )
            .map_err(error)
    }
    pub fn estimate_hsmm(
        &mut self,
        model: &ModelF64,
        observation: &ObservationF64,
        segmentation: &SegmentationF64,
        options: &HsmmOptionsF64,
    ) -> Result<f64, BindingError> {
        self.inner
            .estimate_hsmm(
                &model.inner,
                &observation.inner,
                &segmentation.inner,
                options.native(),
            )
            .map_err(error)
    }
    pub fn estimate_geometric(
        &mut self,
        model: &ModelF64,
        observation: &ObservationF64,
        segmentation: &SegmentationF64,
        options: &GeometricOptionsF64,
    ) -> Result<f64, BindingError> {
        self.inner
            .estimate_geometric(
                &model.inner,
                &observation.inner,
                &segmentation.inner,
                options.native(),
            )
            .map_err(error)
    }
}
