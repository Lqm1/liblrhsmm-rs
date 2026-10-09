use super::gaussian::collection;
use super::*;
use crate::model::{
    ModelParameters, ModelParametersBuilder, StreamLayout, StreamParameters,
    StreamParametersBuilder,
};
use crate::preparation::{OwnedPreparedModelParameters, OwnedPreparedStreamParameters};
use wasm_bindgen::prelude::*;

fn error(value: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&value.to_string())
}

macro_rules! containers {
    ($scalar:ty, $stream:ident, $stream_builder:ident, $stream_prepared:ident, $streams:ident, $gaussian:ident, $gaussians:ident, $gaussian_prepared:ident, $model:ident, $builder:ident, $prepared:ident, $duration:ident, $durations:ident, $duration_prepared:ident, $options:ident) => {
        collection!($streams, $stream, StreamParameters<$scalar>);
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $stream {
            pub(crate) inner: StreamParameters<$scalar>,
        }
        #[wasm_bindgen]
        impl $stream {
            #[wasm_bindgen(constructor)]
            pub fn new(
                emissions: usize,
                components: usize,
                dimensions: usize,
            ) -> Result<Self, JsValue> {
                StreamParameters::new(emissions, components, dimensions)
                    .map(|inner| Self { inner })
                    .map_err(error)
            }
            pub fn empty() -> Self {
                Self {
                    inner: StreamParameters::default(),
                }
            }
            #[wasm_bindgen(getter)]
            pub fn emissions(&self) -> usize {
                self.inner.mixtures.len()
            }
            pub fn weight(&self) -> Vec<$scalar> {
                vec![self.inner.weight]
            }
            pub fn set_weight(&mut self, values: &[$scalar]) -> Result<(), JsValue> {
                let [weight] = values else {
                    return Err(error("stream weight requires one scalar"));
                };
                self.inner.weight = *weight;
                Ok(())
            }
            pub fn emission(&self, index: usize) -> Option<$gaussian> {
                self.inner
                    .mixtures
                    .get(index)
                    .cloned()
                    .map(|inner| $gaussian { inner })
            }
            pub fn set_emission(&mut self, index: usize, value: &$gaussian) -> Result<(), JsValue> {
                let target = self
                    .inner
                    .mixtures
                    .get_mut(index)
                    .ok_or_else(|| error("emission index out of range"))?;
                *target = value.inner.clone();
                Ok(())
            }
            pub fn replace(
                &mut self,
                weight: &[$scalar],
                emissions: &$gaussians,
            ) -> Result<(), JsValue> {
                let [weight] = weight else {
                    return Err(error("stream weight requires one scalar"));
                };
                let inner = StreamParameters {
                    weight: *weight,
                    mixtures: emissions.inner.clone(),
                };
                inner.validate().map_err(error)?;
                self.inner = inner;
                Ok(())
            }
            pub fn validate(&self) -> Result<(), JsValue> {
                self.inner.validate().map_err(error)
            }
            pub fn cloned(&self) -> Self {
                self.clone()
            }
            pub fn prepare(&self) -> Result<$stream_prepared, JsValue> {
                self.cloned().prepare_owned()
            }
            pub fn prepare_owned(self) -> Result<$stream_prepared, JsValue> {
                self.inner
                    .prepare_owned()
                    .map(|inner| $stream_prepared { inner })
                    .map_err(error)
            }
        }
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $stream_builder {
            inner: StreamParametersBuilder<$scalar>,
        }
        #[wasm_bindgen]
        impl $stream_builder {
            #[wasm_bindgen(constructor)]
            pub fn new(emissions: usize) -> Result<Self, JsValue> {
                StreamParametersBuilder::new(emissions)
                    .map(|inner| Self { inner })
                    .map_err(error)
            }
            #[wasm_bindgen(getter)]
            pub fn emissions(&self) -> usize {
                self.inner.emissions.len()
            }
            pub fn weight(&self) -> Vec<$scalar> {
                vec![self.inner.weight]
            }
            pub fn set_weight(&mut self, values: &[$scalar]) -> Result<(), JsValue> {
                let [weight] = values else {
                    return Err(error("stream weight requires one scalar"));
                };
                self.inner.weight = *weight;
                Ok(())
            }
            pub fn emission(&self, index: usize) -> Option<$gaussian> {
                self.inner
                    .emissions
                    .get(index)
                    .and_then(|value| value.clone())
                    .map(|inner| $gaussian { inner })
            }
            pub fn has_emission(&self, index: usize) -> bool {
                self.inner.emissions.get(index).is_some_and(Option::is_some)
            }
            /// Retain existing slots; new slots are empty and removed slots drop.
            pub fn resize(&mut self, emissions: usize) -> Result<(), JsValue> {
                let mut inner = StreamParametersBuilder::new(emissions).map_err(error)?;
                inner.weight = self.inner.weight;
                for (target, source) in inner.emissions.iter_mut().zip(&self.inner.emissions) {
                    target.clone_from(source);
                }
                self.inner = inner;
                Ok(())
            }
            pub fn set_emission(&mut self, index: usize, value: &$gaussian) -> Result<(), JsValue> {
                let target = self
                    .inner
                    .emissions
                    .get_mut(index)
                    .ok_or_else(|| error("emission index out of range"))?;
                *target = Some(value.inner.clone());
                Ok(())
            }
            pub fn clear_emission(&mut self, index: usize) -> Result<(), JsValue> {
                *self
                    .inner
                    .emissions
                    .get_mut(index)
                    .ok_or_else(|| error("emission index out of range"))? = None;
                Ok(())
            }
            pub fn cloned(&self) -> Self {
                self.clone()
            }
            pub fn build(self) -> Result<$stream, JsValue> {
                self.inner
                    .build()
                    .map(|inner| $stream { inner })
                    .map_err(error)
            }
        }
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $stream_prepared {
            pub(crate) inner: OwnedPreparedStreamParameters<$scalar>,
        }
        #[wasm_bindgen]
        impl $stream_prepared {
            pub fn cloned(&self) -> Self {
                self.clone()
            }
            pub fn parameters(&self) -> $stream {
                $stream {
                    inner: self.inner.parameters().clone(),
                }
            }
            pub fn into_parameters(self) -> $stream {
                $stream {
                    inner: self.inner.into_parameters(),
                }
            }
            pub fn emission(&self, state: usize) -> Option<$gaussian_prepared> {
                self.inner.emission(state).map(|value| $gaussian_prepared {
                    inner: value.to_owned(),
                })
            }
        }
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $model {
            pub(crate) inner: ModelParameters<$scalar>,
        }
        #[wasm_bindgen]
        impl $model {
            #[wasm_bindgen(constructor)]
            pub fn new(layouts: &[u32], durations: usize) -> Result<Self, JsValue> {
                if !layouts.len().is_multiple_of(3) {
                    return Err(error(
                        "stream layouts require emissions/components/dimensions triples",
                    ));
                }
                let layouts = layouts
                    .chunks_exact(3)
                    .map(|value| StreamLayout {
                        emissions: value[0] as usize,
                        components: value[1] as usize,
                        dimensions: value[2] as usize,
                    })
                    .collect::<Vec<_>>();
                ModelParameters::new(&layouts, durations)
                    .map(|inner| Self { inner })
                    .map_err(error)
            }
            pub fn empty() -> Self {
                Self {
                    inner: ModelParameters::default(),
                }
            }
            #[wasm_bindgen(getter)]
            pub fn streams(&self) -> usize {
                self.inner.streams.len()
            }
            #[wasm_bindgen(getter)]
            pub fn durations(&self) -> usize {
                self.inner.durations.len()
            }
            pub fn stream(&self, index: usize) -> Option<$stream> {
                self.inner
                    .streams
                    .get(index)
                    .cloned()
                    .map(|inner| $stream { inner })
            }
            pub fn duration(&self, index: usize) -> Option<$duration> {
                self.inner
                    .durations
                    .get(index)
                    .cloned()
                    .map(|inner| $duration { inner })
            }
            pub fn set_stream(&mut self, index: usize, value: &$stream) -> Result<(), JsValue> {
                *self
                    .inner
                    .streams
                    .get_mut(index)
                    .ok_or_else(|| error("stream index out of range"))? = value.inner.clone();
                Ok(())
            }
            pub fn set_duration(&mut self, index: usize, value: &$duration) -> Result<(), JsValue> {
                *self
                    .inner
                    .durations
                    .get_mut(index)
                    .ok_or_else(|| error("duration index out of range"))? = value.inner.clone();
                Ok(())
            }
            pub fn replace(
                &mut self,
                streams: &$streams,
                durations: &$durations,
            ) -> Result<(), JsValue> {
                let inner = ModelParameters {
                    streams: streams.inner.clone(),
                    durations: durations.inner.clone(),
                };
                inner.validate().map_err(error)?;
                self.inner = inner;
                Ok(())
            }
            pub fn validate(&self) -> Result<(), JsValue> {
                self.inner.validate().map_err(error)
            }
            pub fn cloned(&self) -> Self {
                self.clone()
            }
            pub fn prepare(&self, options: &$options) -> Result<$prepared, JsValue> {
                self.cloned().prepare_owned(options)
            }
            pub fn prepare_owned(self, options: &$options) -> Result<$prepared, JsValue> {
                self.inner
                    .prepare_owned(options.native())
                    .map(|inner| $prepared { inner })
                    .map_err(error)
            }
            pub fn prepare_selected_durations(
                &self,
                options: &$options,
                states: &[u32],
            ) -> Result<$prepared, JsValue> {
                self.cloned()
                    .prepare_owned_selected_durations(options, states)
            }
            pub fn prepare_owned_selected_durations(
                self,
                options: &$options,
                states: &[u32],
            ) -> Result<$prepared, JsValue> {
                let states = states
                    .iter()
                    .map(|value| *value as usize)
                    .collect::<Vec<_>>();
                self.inner
                    .prepare_owned_selected_durations(options.native(), &states)
                    .map(|inner| $prepared { inner })
                    .map_err(error)
            }
        }
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $builder {
            inner: ModelParametersBuilder<$scalar>,
        }
        #[wasm_bindgen]
        impl $builder {
            #[wasm_bindgen(constructor)]
            pub fn new(streams: usize, durations: usize) -> Result<Self, JsValue> {
                ModelParametersBuilder::new(streams, durations)
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
            pub fn stream(&self, index: usize) -> Option<$stream> {
                self.inner
                    .streams
                    .get(index)
                    .and_then(|value| value.clone())
                    .map(|inner| $stream { inner })
            }
            pub fn duration(&self, index: usize) -> Option<$duration> {
                self.inner
                    .durations
                    .get(index)
                    .and_then(|value| value.clone())
                    .map(|inner| $duration { inner })
            }
            pub fn has_stream(&self, index: usize) -> bool {
                self.inner.streams.get(index).is_some_and(Option::is_some)
            }
            pub fn has_duration(&self, index: usize) -> bool {
                self.inner.durations.get(index).is_some_and(Option::is_some)
            }
            /// Resize both public slot lists atomically, retaining their prefix.
            pub fn resize(&mut self, streams: usize, durations: usize) -> Result<(), JsValue> {
                let mut inner = ModelParametersBuilder::new(streams, durations).map_err(error)?;
                for (target, source) in inner.streams.iter_mut().zip(&self.inner.streams) {
                    target.clone_from(source);
                }
                for (target, source) in inner.durations.iter_mut().zip(&self.inner.durations) {
                    target.clone_from(source);
                }
                self.inner = inner;
                Ok(())
            }
            pub fn set_stream(&mut self, index: usize, value: &$stream) -> Result<(), JsValue> {
                *self
                    .inner
                    .streams
                    .get_mut(index)
                    .ok_or_else(|| error("stream index out of range"))? = Some(value.inner.clone());
                Ok(())
            }
            pub fn set_duration(&mut self, index: usize, value: &$duration) -> Result<(), JsValue> {
                *self
                    .inner
                    .durations
                    .get_mut(index)
                    .ok_or_else(|| error("duration index out of range"))? =
                    Some(value.inner.clone());
                Ok(())
            }
            pub fn clear_stream(&mut self, index: usize) -> Result<(), JsValue> {
                *self
                    .inner
                    .streams
                    .get_mut(index)
                    .ok_or_else(|| error("stream index out of range"))? = None;
                Ok(())
            }
            pub fn clear_duration(&mut self, index: usize) -> Result<(), JsValue> {
                *self
                    .inner
                    .durations
                    .get_mut(index)
                    .ok_or_else(|| error("duration index out of range"))? = None;
                Ok(())
            }
            pub fn cloned(&self) -> Self {
                self.clone()
            }
            pub fn build(self) -> Result<$model, JsValue> {
                self.inner
                    .build()
                    .map(|inner| $model { inner })
                    .map_err(error)
            }
        }
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $prepared {
            pub(crate) inner: OwnedPreparedModelParameters<$scalar>,
        }
        #[wasm_bindgen]
        impl $prepared {
            pub fn cloned(&self) -> Self {
                self.clone()
            }
            pub fn parameters(&self) -> $model {
                $model {
                    inner: self.inner.model().clone(),
                }
            }
            pub fn into_parameters(self) -> $model {
                $model {
                    inner: self.inner.into_model(),
                }
            }
            pub fn options(&self) -> $options {
                let value = self.inner.options();
                $options {
                    duration_cache_length: value.duration_cache_length,
                    duration_weight: value.duration_weight,
                    temperature: value.temperature,
                }
            }
            pub fn duration(&self, state: usize) -> Option<$duration_prepared> {
                self.inner.duration(state).map(|value| $duration_prepared {
                    inner: value.to_owned(),
                })
            }
            pub fn emission(&self, stream: usize, state: usize) -> Option<$gaussian_prepared> {
                self.inner
                    .emission(stream, state)
                    .map(|value| $gaussian_prepared {
                        inner: value.to_owned(),
                    })
            }
        }
    };
}
containers!(
    f32,
    StreamF32,
    StreamBuilderF32,
    PreparedStreamF32,
    StreamCollectionF32,
    GaussianF32,
    GaussianCollectionF32,
    PreparedGaussianF32,
    ModelF32,
    ModelBuilderF32,
    PreparedModelF32,
    DurationF32,
    DurationCollectionF32,
    PreparedDurationF32,
    PreparationOptionsF32
);
containers!(
    f64,
    StreamF64,
    StreamBuilderF64,
    PreparedStreamF64,
    StreamCollectionF64,
    GaussianF64,
    GaussianCollectionF64,
    PreparedGaussianF64,
    ModelF64,
    ModelBuilderF64,
    PreparedModelF64,
    DurationF64,
    DurationCollectionF64,
    PreparedDurationF64,
    PreparationOptionsF64
);
