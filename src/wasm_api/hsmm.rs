use super::*;
use crate::hsmm::{HsmmPath, HsmmScoreTables, HsmmSettings, StateOccurrence};
use wasm_bindgen::prelude::*;

fn error(value: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&value.to_string())
}

macro_rules! inference_methods {
    ($owner:ident, $options:ident, $scores:ident, $path:ident, $segmentation:ident, $graph:ident, $emissions:ident) => {
        #[wasm_bindgen]
        impl $owner {
            pub fn forward_hsmm(
                &self,
                segmentation: &$segmentation,
                emissions: &$emissions,
                options: &$options,
            ) -> Result<$scores, JsValue> {
                self.inner
                    .forward_hsmm(&segmentation.inner, &emissions.inner, options.native())
                    .map(|inner| $scores { inner })
                    .map_err(error)
            }
            pub fn backward_hsmm(
                &self,
                segmentation: &$segmentation,
                emissions: &$emissions,
                options: &$options,
            ) -> Result<$scores, JsValue> {
                self.inner
                    .backward_hsmm(&segmentation.inner, &emissions.inner, options.native())
                    .map(|inner| $scores { inner })
                    .map_err(error)
            }
            pub fn viterbi_hsmm(
                &self,
                segmentation: &$segmentation,
                emissions: &$emissions,
                options: &$options,
            ) -> Result<$path, JsValue> {
                self.inner
                    .viterbi_hsmm(&segmentation.inner, &emissions.inner, options.native())
                    .map(|inner| $path { inner })
                    .map_err(error)
            }
            pub fn forward_hsmm_with_graph(
                &self,
                graph: &$graph,
                emissions: &$emissions,
                options: &$options,
            ) -> Result<$scores, JsValue> {
                self.inner
                    .forward_hsmm_with_graph(
                        graph.inner.as_view(),
                        &emissions.inner,
                        options.native(),
                    )
                    .map(|inner| $scores { inner })
                    .map_err(error)
            }
            pub fn backward_hsmm_with_graph(
                &self,
                graph: &$graph,
                emissions: &$emissions,
                options: &$options,
            ) -> Result<$scores, JsValue> {
                self.inner
                    .backward_hsmm_with_graph(
                        graph.inner.as_view(),
                        &emissions.inner,
                        options.native(),
                    )
                    .map(|inner| $scores { inner })
                    .map_err(error)
            }
            pub fn viterbi_hsmm_with_graph(
                &self,
                graph: &$graph,
                emissions: &$emissions,
                options: &$options,
            ) -> Result<$path, JsValue> {
                self.inner
                    .viterbi_hsmm_with_graph(
                        graph.inner.as_view(),
                        &emissions.inner,
                        options.native(),
                    )
                    .map(|inner| $path { inner })
                    .map_err(error)
            }
        }
    };
}

macro_rules! hsmm {
    ($scalar:ty, $options:ident, $scores:ident, $state_scores:ident, $path:ident, $model:ident, $prepared:ident, $segmentation:ident, $graph:ident, $emissions:ident) => {
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $options {
            pub temperature: $scalar,
            pub duration_weight: $scalar,
            pub state_radius: $scalar,
            pub duration_extra: usize,
            pub duration_extra_factor: $scalar,
        }
        impl Default for $options {
            fn default() -> Self {
                Self::new()
            }
        }
        impl $options {
            pub(crate) fn native(&self) -> HsmmSettings<$scalar> {
                HsmmSettings {
                    temperature: self.temperature,
                    duration_weight: self.duration_weight,
                    state_radius: self.state_radius,
                    duration_extra: self.duration_extra,
                    duration_extra_factor: self.duration_extra_factor,
                }
            }
        }
        #[wasm_bindgen]
        impl $options {
            #[wasm_bindgen(constructor)]
            pub fn new() -> Self {
                let value = HsmmSettings::<$scalar>::default();
                Self {
                    temperature: value.temperature,
                    duration_weight: value.duration_weight,
                    state_radius: value.state_radius,
                    duration_extra: value.duration_extra,
                    duration_extra_factor: value.duration_extra_factor,
                }
            }
            pub fn cloned(&self) -> Self {
                self.clone()
            }
        }
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $scores {
            pub(crate) inner: HsmmScoreTables<$scalar>,
        }
        #[wasm_bindgen]
        impl $scores {
            #[wasm_bindgen(constructor)]
            pub fn new(begin: &$state_scores, end: &$state_scores) -> Self {
                Self {
                    inner: HsmmScoreTables {
                        begin: begin.inner.clone(),
                        end: end.inner.clone(),
                    },
                }
            }
            pub fn begin(&self) -> $state_scores {
                $state_scores {
                    inner: self.inner.begin.clone(),
                }
            }
            pub fn end(&self) -> $state_scores {
                $state_scores {
                    inner: self.inner.end.clone(),
                }
            }
            pub fn set_begin(&mut self, value: &$state_scores) {
                self.inner.begin = value.inner.clone();
            }
            pub fn set_end(&mut self, value: &$state_scores) {
                self.inner.end = value.inner.clone();
            }
            pub fn cloned(&self) -> Self {
                self.clone()
            }
        }
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $path {
            pub(crate) inner: HsmmPath<$scalar>,
        }
        #[wasm_bindgen]
        impl $path {
            #[wasm_bindgen(constructor)]
            pub fn new(
                occurrences: &[u32],
                log_probability: &[$scalar],
                scores: &$scores,
            ) -> Result<Self, JsValue> {
                let [log_probability] = log_probability else {
                    return Err(error("path log probability requires one scalar"));
                };
                if !occurrences.len().is_multiple_of(2) {
                    return Err(error("occurrences require state/end pairs"));
                }
                Ok(Self {
                    inner: HsmmPath {
                        occurrences: occurrences
                            .chunks_exact(2)
                            .map(|pair| StateOccurrence {
                                state: pair[0] as usize,
                                end: pair[1] as usize,
                            })
                            .collect(),
                        log_probability: *log_probability,
                        scores: scores.inner.clone(),
                    },
                })
            }
            pub fn occurrences(&self) -> Vec<u32> {
                self.inner
                    .occurrences
                    .iter()
                    .flat_map(|value| [value.state as u32, value.end as u32])
                    .collect()
            }
            pub fn set_occurrences(&mut self, values: &[u32]) -> Result<(), JsValue> {
                if !values.len().is_multiple_of(2) {
                    return Err(error("occurrences require state/end pairs"));
                }
                self.inner.occurrences = values
                    .chunks_exact(2)
                    .map(|pair| StateOccurrence {
                        state: pair[0] as usize,
                        end: pair[1] as usize,
                    })
                    .collect();
                Ok(())
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
        inference_methods!(
            $model,
            $options,
            $scores,
            $path,
            $segmentation,
            $graph,
            $emissions
        );
        inference_methods!(
            $prepared,
            $options,
            $scores,
            $path,
            $segmentation,
            $graph,
            $emissions
        );
    };
}
hsmm!(
    f32,
    HsmmOptionsF32,
    HsmmScoresF32,
    StateScoresF32,
    HsmmPathF32,
    ModelF32,
    PreparedModelF32,
    SegmentationF32,
    TransitionGraphF32,
    LogEmissionsF32
);
hsmm!(
    f64,
    HsmmOptionsF64,
    HsmmScoresF64,
    StateScoresF64,
    HsmmPathF64,
    ModelF64,
    PreparedModelF64,
    SegmentationF64,
    TransitionGraphF64,
    LogEmissionsF64
);
