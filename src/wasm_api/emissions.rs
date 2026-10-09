use super::*;
use crate::emission::{EmissionDensityTable, EmissionPruningOptions, LogEmissionTable};
use wasm_bindgen::prelude::*;

fn error(value: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&value.to_string())
}

macro_rules! emissions {
    ($scalar:ty, $pruning:ident, $log:ident, $density:ident, $model:ident, $prepared:ident, $observation:ident, $segmentation:ident) => {
        /// Pruning mode: 0 evaluates all pairs, 1 uses a boundary radius,
        /// and 2 uses a linear slope. The parameter retains selected precision.
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $pruning {
            pub mode: u32,
            pub parameter: $scalar,
        }
        impl $pruning {
            pub(crate) fn native(&self) -> Result<EmissionPruningOptions<$scalar>, JsValue> {
                match self.mode {
                    0 => Ok(EmissionPruningOptions::None),
                    1 => Ok(EmissionPruningOptions::AroundBoundaries {
                        radius: self.parameter,
                    }),
                    2 => Ok(EmissionPruningOptions::Linear {
                        slope: self.parameter,
                    }),
                    _ => Err(error("unknown emission pruning mode")),
                }
            }
        }
        #[wasm_bindgen]
        impl $pruning {
            #[wasm_bindgen(constructor)]
            pub fn new(mode: u32, parameter: $scalar) -> Self {
                Self { mode, parameter }
            }
            pub fn cloned(&self) -> Self {
                self.clone()
            }
        }
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $log {
            pub(crate) inner: LogEmissionTable<$scalar>,
        }
        #[wasm_bindgen]
        impl $log {
            #[wasm_bindgen(constructor)]
            pub fn new(frames: usize, states: usize, values: &[$scalar]) -> Result<Self, JsValue> {
                LogEmissionTable::from_log_values(frames, states, values.to_vec())
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
            pub fn probabilities(&self) -> $density {
                $density {
                    inner: self.inner.probabilities(),
                }
            }
        }
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $density {
            pub(crate) inner: EmissionDensityTable<$scalar>,
        }
        #[wasm_bindgen]
        impl $density {
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
        #[wasm_bindgen]
        impl $model {
            pub fn emission_log_probabilities(
                &self,
                observation: &$observation,
                segmentation: &$segmentation,
                temperature: $scalar,
                pruning: &$pruning,
            ) -> Result<$log, JsValue> {
                self.inner
                    .emission_log_probabilities(
                        &observation.inner,
                        &segmentation.inner,
                        temperature,
                        pruning.native()?,
                    )
                    .map(|inner| $log { inner })
                    .map_err(error)
            }
            pub fn emission_probabilities(
                &self,
                observation: &$observation,
                segmentation: &$segmentation,
                temperature: $scalar,
                pruning: &$pruning,
            ) -> Result<$density, JsValue> {
                self.inner
                    .emission_probabilities(
                        &observation.inner,
                        &segmentation.inner,
                        temperature,
                        pruning.native()?,
                    )
                    .map(|inner| $density { inner })
                    .map_err(error)
            }
        }
        #[wasm_bindgen]
        impl $prepared {
            pub fn emission_log_probabilities(
                &self,
                observation: &$observation,
                segmentation: &$segmentation,
                temperature: $scalar,
                pruning: &$pruning,
            ) -> Result<$log, JsValue> {
                self.inner
                    .emission_log_probabilities(
                        &observation.inner,
                        &segmentation.inner,
                        temperature,
                        pruning.native()?,
                    )
                    .map(|inner| $log { inner })
                    .map_err(error)
            }
            pub fn emission_probabilities(
                &self,
                observation: &$observation,
                segmentation: &$segmentation,
                temperature: $scalar,
                pruning: &$pruning,
            ) -> Result<$density, JsValue> {
                self.inner
                    .emission_probabilities(
                        &observation.inner,
                        &segmentation.inner,
                        temperature,
                        pruning.native()?,
                    )
                    .map(|inner| $density { inner })
                    .map_err(error)
            }
        }
    };
}
emissions!(
    f32,
    EmissionPruningF32,
    LogEmissionsF32,
    EmissionDensitiesF32,
    ModelF32,
    PreparedModelF32,
    ObservationF32,
    SegmentationF32
);
emissions!(
    f64,
    EmissionPruningF64,
    LogEmissionsF64,
    EmissionDensitiesF64,
    ModelF64,
    PreparedModelF64,
    ObservationF64,
    SegmentationF64
);
