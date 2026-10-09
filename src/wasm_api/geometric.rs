use super::*;
use crate::geometric::{GeometricPath, GeometricSettings, StateScoreTable};
use wasm_bindgen::prelude::*;

fn error(value: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&value.to_string())
}

macro_rules! geometric {
    ($scalar:ty, $options:ident, $scores:ident, $path:ident, $model:ident, $segmentation:ident, $emissions:ident) => {
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $options {
            pub temperature: $scalar,
            pub pruning_slope: $scalar,
        }
        impl Default for $options {
            fn default() -> Self {
                Self::new()
            }
        }
        impl $options {
            pub(crate) fn native(&self) -> GeometricSettings<$scalar> {
                GeometricSettings {
                    temperature: self.temperature,
                    pruning_slope: self.pruning_slope,
                }
            }
        }
        #[wasm_bindgen]
        impl $options {
            #[wasm_bindgen(constructor)]
            pub fn new() -> Self {
                let value = GeometricSettings::<$scalar>::default();
                Self {
                    temperature: value.temperature,
                    pruning_slope: value.pruning_slope,
                }
            }
            pub fn cloned(&self) -> Self {
                self.clone()
            }
        }
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $scores {
            pub(crate) inner: StateScoreTable<$scalar>,
        }
        #[wasm_bindgen]
        impl $scores {
            #[wasm_bindgen(constructor)]
            pub fn new(frames: usize, states: usize, values: &[$scalar]) -> Result<Self, JsValue> {
                StateScoreTable::from_values(frames, states, values.to_vec())
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
            pub fn total_log_probability(&self) -> $scalar {
                self.inner.total_log_probability()
            }
            pub fn total_through_state(
                &self,
                backward: &$scores,
                state: usize,
            ) -> Result<$scalar, JsValue> {
                self.inner
                    .total_through_state(&backward.inner, state)
                    .map_err(error)
            }
            pub fn backward_log_probability(
                &self,
                emissions: &$emissions,
            ) -> Result<$scalar, JsValue> {
                self.inner
                    .backward_log_probability(&emissions.inner)
                    .map_err(error)
            }
        }
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $path {
            pub(crate) inner: GeometricPath<$scalar>,
        }
        #[wasm_bindgen]
        impl $path {
            #[wasm_bindgen(constructor)]
            pub fn new(
                states: &[u32],
                boundaries: &[u32],
                log_probability: &[$scalar],
                scores: &$scores,
            ) -> Result<Self, JsValue> {
                let [log_probability] = log_probability else {
                    return Err(error("path log probability requires one scalar"));
                };
                Ok(Self {
                    inner: GeometricPath {
                        states: states.iter().map(|&value| value as usize).collect(),
                        boundaries: boundaries.iter().map(|&value| value as usize).collect(),
                        log_probability: *log_probability,
                        scores: scores.inner.clone(),
                    },
                })
            }
            pub fn states(&self) -> Vec<u32> {
                self.inner
                    .states
                    .iter()
                    .map(|&value| value as u32)
                    .collect()
            }
            pub fn set_states(&mut self, values: &[u32]) {
                self.inner.states = values.iter().map(|&value| value as usize).collect();
            }
            pub fn boundaries(&self) -> Vec<u32> {
                self.inner
                    .boundaries
                    .iter()
                    .map(|&value| value as u32)
                    .collect()
            }
            pub fn set_boundaries(&mut self, values: &[u32]) {
                self.inner.boundaries = values.iter().map(|&value| value as usize).collect();
            }
            pub fn log_probability(&self) -> Vec<$scalar> {
                vec![self.inner.log_probability]
            }
            pub fn set_log_probability(&mut self, values: &[$scalar]) -> Result<(), JsValue> {
                let [value] = values else {
                    return Err(error("path log probability requires one scalar"));
                };
                self.inner.log_probability = *value;
                Ok(())
            }
            pub fn scores(&self) -> $scores {
                $scores {
                    inner: self.inner.scores.clone(),
                }
            }
            pub fn set_scores(&mut self, value: &$scores) {
                self.inner.scores = value.inner.clone();
            }
            pub fn cloned(&self) -> Self {
                self.clone()
            }
        }
        #[wasm_bindgen]
        impl $model {
            pub fn forward_geometric(
                &self,
                segmentation: &$segmentation,
                emissions: &$emissions,
                options: &$options,
            ) -> Result<$scores, JsValue> {
                self.inner
                    .forward_geometric(&segmentation.inner, &emissions.inner, options.native())
                    .map(|inner| $scores { inner })
                    .map_err(error)
            }
            pub fn backward_geometric(
                &self,
                segmentation: &$segmentation,
                emissions: &$emissions,
                options: &$options,
            ) -> Result<$scores, JsValue> {
                self.inner
                    .backward_geometric(&segmentation.inner, &emissions.inner, options.native())
                    .map(|inner| $scores { inner })
                    .map_err(error)
            }
            pub fn viterbi_geometric(
                &self,
                segmentation: &$segmentation,
                emissions: &$emissions,
                options: &$options,
            ) -> Result<$path, JsValue> {
                self.inner
                    .viterbi_geometric(&segmentation.inner, &emissions.inner, options.native())
                    .map(|inner| $path { inner })
                    .map_err(error)
            }
        }
    };
}
geometric!(
    f32,
    GeometricOptionsF32,
    StateScoresF32,
    GeometricPathF32,
    ModelF32,
    SegmentationF32,
    LogEmissionsF32
);
geometric!(
    f64,
    GeometricOptionsF64,
    StateScoresF64,
    GeometricPathF64,
    ModelF64,
    SegmentationF64,
    LogEmissionsF64
);
