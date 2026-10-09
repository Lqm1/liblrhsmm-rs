use super::gaussian::collection;
use super::*;
use crate::statistics::{
    DurationSufficientStatistics, GaussianSufficientStatistics, ModelSufficientStatistics,
    StreamSufficientStatistics,
};
use wasm_bindgen::prelude::*;
fn error(value: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&value.to_string())
}
macro_rules! statistics {
    ($scalar:ty, $stream:ident, $model:ident, $gaussian:ident, $duration:ident, $stream_stats:ident, $model_stats:ident, $gaussians:ident, $streams:ident, $durations:ident, $update:ident) => {
        collection!($gaussians, $gaussian, GaussianSufficientStatistics<$scalar>);
        collection!($streams, $stream_stats, StreamSufficientStatistics<$scalar>);
        #[wasm_bindgen]
        #[derive(Clone, Default)]
        pub struct $durations {
            inner: Vec<DurationSufficientStatistics<$scalar>>,
        }
        #[wasm_bindgen]
        impl $durations {
            #[wasm_bindgen(constructor)]
            pub fn new() -> Self {
                Self::default()
            }
            #[wasm_bindgen(getter)]
            pub fn length(&self) -> usize {
                self.inner.len()
            }
            pub fn is_empty(&self) -> bool {
                self.inner.is_empty()
            }
            pub fn push(&mut self, value: &$duration) {
                self.inner.push(value.native());
            }
            pub fn get(&self, index: usize) -> Option<$duration> {
                self.inner.get(index).copied().map($duration::from_native)
            }
            pub fn replace(&mut self, index: usize, value: &$duration) -> Result<(), JsValue> {
                *self
                    .inner
                    .get_mut(index)
                    .ok_or_else(|| error("duration statistic index out of range"))? =
                    value.native();
                Ok(())
            }
            pub fn clear(&mut self) {
                self.inner.clear();
            }
            pub fn cloned(&self) -> Self {
                self.clone()
            }
        }
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $stream_stats {
            pub(crate) inner: StreamSufficientStatistics<$scalar>,
        }
        #[wasm_bindgen]
        impl $stream_stats {
            #[wasm_bindgen(constructor)]
            pub fn new() -> Self {
                Self {
                    inner: StreamSufficientStatistics {
                        mixtures: Vec::new(),
                    },
                }
            }
            pub fn from_stream(value: &$stream) -> Result<Self, JsValue> {
                StreamSufficientStatistics::from_stream(&value.inner)
                    .map(|inner| Self { inner })
                    .map_err(error)
            }
            #[wasm_bindgen(getter)]
            pub fn emissions(&self) -> usize {
                self.inner.mixtures.len()
            }
            pub fn emission(&self, index: usize) -> Option<$gaussian> {
                self.inner
                    .mixtures
                    .get(index)
                    .cloned()
                    .map(|inner| $gaussian { inner })
            }
            pub fn set_emission(&mut self, index: usize, value: &$gaussian) -> Result<(), JsValue> {
                *self
                    .inner
                    .mixtures
                    .get_mut(index)
                    .ok_or_else(|| error("emission statistic index out of range"))? =
                    value.inner.clone();
                Ok(())
            }
            pub fn replace(&mut self, values: &$gaussians) {
                self.inner.mixtures = values.inner.clone();
            }
            pub fn cloned(&self) -> Self {
                self.clone()
            }
            pub fn clear(&mut self) {
                self.inner.clear();
            }
        }
        impl Default for $stream_stats {
            fn default() -> Self {
                Self::new()
            }
        }
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $model_stats {
            pub(crate) inner: ModelSufficientStatistics<$scalar>,
        }
        #[wasm_bindgen]
        impl $model_stats {
            #[wasm_bindgen(constructor)]
            pub fn new() -> Self {
                Self {
                    inner: ModelSufficientStatistics {
                        streams: Vec::new(),
                        durations: Vec::new(),
                    },
                }
            }
            pub fn from_model(value: &$model) -> Result<Self, JsValue> {
                ModelSufficientStatistics::from_model(&value.inner)
                    .map(|inner| Self { inner })
                    .map_err(error)
            }
            #[wasm_bindgen(getter)]
            pub fn streams(&self) -> usize {
                self.inner.streams.len()
            }
            #[wasm_bindgen(getter)]
            pub fn durations(&self) -> usize {
                self.inner.durations.len()
            }
            pub fn stream(&self, index: usize) -> Option<$stream_stats> {
                self.inner
                    .streams
                    .get(index)
                    .cloned()
                    .map(|inner| $stream_stats { inner })
            }
            pub fn duration(&self, index: usize) -> Option<$duration> {
                self.inner
                    .durations
                    .get(index)
                    .copied()
                    .map($duration::from_native)
            }
            pub fn set_stream(
                &mut self,
                index: usize,
                value: &$stream_stats,
            ) -> Result<(), JsValue> {
                *self
                    .inner
                    .streams
                    .get_mut(index)
                    .ok_or_else(|| error("stream statistic index out of range"))? =
                    value.inner.clone();
                Ok(())
            }
            pub fn set_duration(&mut self, index: usize, value: &$duration) -> Result<(), JsValue> {
                *self
                    .inner
                    .durations
                    .get_mut(index)
                    .ok_or_else(|| error("duration statistic index out of range"))? =
                    value.native();
                Ok(())
            }
            pub fn replace(&mut self, streams: &$streams, durations: &$durations) {
                self.inner = ModelSufficientStatistics {
                    streams: streams.inner.clone(),
                    durations: durations.inner.clone(),
                };
            }
            pub fn cloned(&self) -> Self {
                self.clone()
            }
            pub fn clear(&mut self) {
                self.inner.clear();
            }
        }
        impl Default for $model_stats {
            fn default() -> Self {
                Self::new()
            }
        }
        #[wasm_bindgen]
        impl $stream {
            pub fn update(
                &mut self,
                statistics: &$stream_stats,
                options: &$update,
            ) -> Result<(), JsValue> {
                self.inner
                    .update(&statistics.inner, options.native()?)
                    .map_err(error)
            }
        }
        #[wasm_bindgen]
        impl $model {
            pub fn update(
                &mut self,
                statistics: &$model_stats,
                options: &$update,
            ) -> Result<(), JsValue> {
                self.inner
                    .update(&statistics.inner, options.native()?)
                    .map_err(error)
            }
        }
    };
}
statistics!(
    f32,
    StreamF32,
    ModelF32,
    GaussianStatisticsF32,
    DurationStatisticsF32,
    StreamStatisticsF32,
    ModelStatisticsF32,
    GaussianStatisticsCollectionF32,
    StreamStatisticsCollectionF32,
    DurationStatisticsCollectionF32,
    UpdateOptionsF32
);
statistics!(
    f64,
    StreamF64,
    ModelF64,
    GaussianStatisticsF64,
    DurationStatisticsF64,
    StreamStatisticsF64,
    ModelStatisticsF64,
    GaussianStatisticsCollectionF64,
    StreamStatisticsCollectionF64,
    DurationStatisticsCollectionF64,
    UpdateOptionsF64
);
