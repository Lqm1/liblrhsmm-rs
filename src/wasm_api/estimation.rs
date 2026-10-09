use super::gaussian::collection;
use super::*;
use crate::duration_occupancy::DurationOccupancies;
use crate::expectation::{GeometricPosteriorExpectation, HsmmPosteriorExpectation};
use crate::mixture_occupancy::MixtureOccupancies;
use crate::occupancy::{StateOccupancyTable, TransitionOccupancies};
use wasm_bindgen::prelude::*;

fn error(value: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&value.to_string())
}

collection!(
    MixturePosteriorCollection,
    MixturePosteriors,
    MixtureOccupancies
);

macro_rules! estimation {
    ($scalar:ty, $hsmm:ident, $geometric:ident, $states:ident, $statistics:ident, $model:ident, $observation:ident, $segmentation:ident, $hsmm_options:ident, $geometric_options:ident) => {
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $hsmm {
            durations: DurationOccupancies,
            states: StateOccupancyTable<$scalar>,
            mixtures: Vec<MixtureOccupancies>,
            pub state_radius: $scalar,
        }
        impl $hsmm {
            fn native(&self) -> HsmmPosteriorExpectation<'_, $scalar> {
                HsmmPosteriorExpectation {
                    durations: &self.durations,
                    states: &self.states,
                    mixtures: &self.mixtures,
                    state_radius: self.state_radius,
                }
            }
        }
        #[wasm_bindgen]
        impl $hsmm {
            #[wasm_bindgen(constructor)]
            pub fn new(
                durations: &DurationPosteriors,
                states: &$states,
                mixtures: &MixturePosteriorCollection,
                state_radius: $scalar,
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
            pub fn states(&self) -> $states {
                $states {
                    inner: self.states.clone(),
                }
            }
            pub fn set_states(&mut self, value: &$states) {
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
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $geometric {
            transitions: TransitionOccupancies,
            states: StateOccupancyTable<$scalar>,
            mixtures: Vec<MixtureOccupancies>,
            pub pruning_slope: $scalar,
        }
        impl $geometric {
            fn native(&self) -> GeometricPosteriorExpectation<'_, $scalar> {
                GeometricPosteriorExpectation {
                    transitions: &self.transitions,
                    states: &self.states,
                    mixtures: &self.mixtures,
                    pruning_slope: self.pruning_slope,
                }
            }
        }
        #[wasm_bindgen]
        impl $geometric {
            #[wasm_bindgen(constructor)]
            pub fn new(
                transitions: &TransitionPosteriors,
                states: &$states,
                mixtures: &MixturePosteriorCollection,
                pruning_slope: $scalar,
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
            pub fn states(&self) -> $states {
                $states {
                    inner: self.states.clone(),
                }
            }
            pub fn set_states(&mut self, value: &$states) {
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
        #[wasm_bindgen]
        impl $statistics {
            pub fn collect_initialization(
                &mut self,
                model: &$model,
                observation: &$observation,
                segmentation: &$segmentation,
            ) -> Result<(), JsValue> {
                self.inner
                    .collect_initialization(&model.inner, &observation.inner, &segmentation.inner)
                    .map_err(error)
            }
            pub fn collect_hsmm(
                &mut self,
                model: &$model,
                observation: &$observation,
                segmentation: &$segmentation,
                expectation: &$hsmm,
            ) -> Result<(), JsValue> {
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
                model: &$model,
                observation: &$observation,
                segmentation: &$segmentation,
                expectation: &$geometric,
            ) -> Result<(), JsValue> {
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
                model: &$model,
                observation: &$observation,
                segmentation: &$segmentation,
                options: &$hsmm_options,
            ) -> Result<$scalar, JsValue> {
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
                model: &$model,
                observation: &$observation,
                segmentation: &$segmentation,
                options: &$geometric_options,
            ) -> Result<$scalar, JsValue> {
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
    };
}
estimation!(
    f32,
    HsmmExpectationF32,
    GeometricExpectationF32,
    StateOccupanciesF32,
    ModelStatisticsF32,
    ModelF32,
    ObservationF32,
    SegmentationF32,
    HsmmOptionsF32,
    GeometricOptionsF32
);
estimation!(
    f64,
    HsmmExpectationF64,
    GeometricExpectationF64,
    StateOccupanciesF64,
    ModelStatisticsF64,
    ModelF64,
    ObservationF64,
    SegmentationF64,
    HsmmOptionsF64,
    GeometricOptionsF64
);
