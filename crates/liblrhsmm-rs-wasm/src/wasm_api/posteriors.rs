use super::*;
use crate::mixture_occupancy::MixtureOccupancySettings;
use crate::occupancy::StateOccupancyTable;
use wasm_bindgen::prelude::*;

fn error(value: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&value.to_string())
}

macro_rules! posteriors {
    ($scalar:ty, $occupancy:ident, $mode:ident, $model:ident, $state_scores:ident, $hsmm_scores:ident, $geometric_options:ident, $hsmm_options:ident, $observation:ident, $segmentation:ident, $graph:ident, $emissions:ident) => {
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $occupancy {
            pub(crate) inner: StateOccupancyTable<$scalar>,
        }
        #[wasm_bindgen]
        impl $occupancy {
            #[wasm_bindgen(constructor)]
            pub fn new(frames: usize, states: usize, values: &[$scalar]) -> Result<Self, JsValue> {
                StateOccupancyTable::from_values(frames, states, values.to_vec())
                    .map(|inner| Self { inner })
                    .map_err(error)
            }
            #[wasm_bindgen(getter)]
            pub fn frames(&self) -> usize {
                self.inner.frames()
            }
            #[wasm_bindgen(getter)]
            pub fn states(&self) -> usize {
                self.inner.states()
            }
            pub fn values(&self) -> Vec<$scalar> {
                self.inner.values().to_vec()
            }
            pub fn get(&self, frame: usize, state: usize) -> Option<$scalar> {
                self.inner.get(frame, state)
            }
            pub fn cloned(&self) -> Self {
                self.clone()
            }
        }
        /// Mode 0 applies an HSMM state radius; mode 1 applies an HMM pruning slope.
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $mode {
            pub mode: u32,
            pub parameter: $scalar,
        }
        impl $mode {
            fn native(&self) -> Result<MixtureOccupancySettings<$scalar>, JsValue> {
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
        #[wasm_bindgen]
        impl $mode {
            #[wasm_bindgen(constructor)]
            pub fn new(mode: u32, parameter: $scalar) -> Self {
                Self { mode, parameter }
            }
            pub fn cloned(&self) -> Self {
                self.clone()
            }
        }
        #[wasm_bindgen]
        impl $state_scores {
            pub fn geometric_occupancies(
                &self,
                backward: &$state_scores,
                total: $scalar,
                pruning_slope: $scalar,
            ) -> Result<$occupancy, JsValue> {
                self.inner
                    .geometric_occupancies(&backward.inner, total, pruning_slope)
                    .map(|inner| $occupancy { inner })
                    .map_err(error)
            }
        }
        #[wasm_bindgen]
        impl $model {
            pub fn geometric_transition_occupancies(
                &self,
                segmentation: &$segmentation,
                emissions: &$emissions,
                forward: &$state_scores,
                backward: &$state_scores,
                total: $scalar,
                options: &$geometric_options,
            ) -> Result<TransitionPosteriors, JsValue> {
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
                segmentation: &$segmentation,
                emissions: &$emissions,
                forward: &$hsmm_scores,
                backward: &$hsmm_scores,
                total: $scalar,
                options: &$hsmm_options,
            ) -> Result<DurationPosteriors, JsValue> {
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
                graph: &$graph,
                emissions: &$emissions,
                forward: &$hsmm_scores,
                backward: &$hsmm_scores,
                total: $scalar,
                options: &$hsmm_options,
            ) -> Result<DurationPosteriors, JsValue> {
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
                segmentation: &$segmentation,
                emissions: &$emissions,
                duration_occupancies: &DurationPosteriors,
                options: &$hsmm_options,
            ) -> Result<$occupancy, JsValue> {
                self.inner
                    .hsmm_state_occupancies(
                        &segmentation.inner,
                        &emissions.inner,
                        &duration_occupancies.inner,
                        options.native(),
                    )
                    .map(|inner| $occupancy { inner })
                    .map_err(error)
            }
            pub fn mixture_occupancies(
                &self,
                observation: &$observation,
                segmentation: &$segmentation,
                occupancy: &$occupancy,
                stream_index: usize,
                temperature: $scalar,
                mode: &$mode,
            ) -> Result<MixturePosteriors, JsValue> {
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
    };
}
posteriors!(
    f32,
    StateOccupanciesF32,
    MixtureOccupancyModeF32,
    ModelF32,
    StateScoresF32,
    HsmmScoresF32,
    GeometricOptionsF32,
    HsmmOptionsF32,
    ObservationF32,
    SegmentationF32,
    TransitionGraphF32,
    LogEmissionsF32
);
posteriors!(
    f64,
    StateOccupanciesF64,
    MixtureOccupancyModeF64,
    ModelF64,
    StateScoresF64,
    HsmmScoresF64,
    GeometricOptionsF64,
    HsmmOptionsF64,
    ObservationF64,
    SegmentationF64,
    TransitionGraphF64,
    LogEmissionsF64
);
