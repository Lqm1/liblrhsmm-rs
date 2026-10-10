fn new_err_owned(message: &str) -> pyo3::PyErr {
    pyo3::exceptions::PyValueError::new_err(message.to_owned())
}
use super::*;
use crate::model::{
    ModelParameters, ModelParametersBuilder, StreamLayout, StreamParameters,
    StreamParametersBuilder,
};
use crate::preparation::{OwnedPreparedModelParameters, OwnedPreparedStreamParameters};
use pyo3::PyErr as BindingError;
fn error(value: impl std::fmt::Display) -> BindingError {
    new_err_owned(&value.to_string())
}
#[derive(Clone, Default)]
pub struct StreamCollectionF32 {
    pub(crate) inner: Vec<StreamParameters<f32>>,
}
impl StreamCollectionF32 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn length(&self) -> usize {
        self.inner.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn push(&mut self, value: &StreamF32) {
        self.inner.push(value.inner.clone());
    }
    pub fn get(&self, index: usize) -> Option<StreamF32> {
        self.inner
            .get(index)
            .cloned()
            .map(|inner| StreamF32 { inner })
    }
    pub fn replace(&mut self, index: usize, value: &StreamF32) -> Result<(), BindingError> {
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
pub struct StreamF32 {
    pub(crate) inner: StreamParameters<f32>,
}
impl StreamF32 {
    pub fn new(
        emissions: usize,
        components: usize,
        dimensions: usize,
    ) -> Result<Self, BindingError> {
        StreamParameters::new(emissions, components, dimensions)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn empty() -> Self {
        Self {
            inner: StreamParameters::default(),
        }
    }
    pub fn emissions(&self) -> usize {
        self.inner.mixtures.len()
    }
    pub fn weight(&self) -> Vec<f32> {
        vec![self.inner.weight]
    }
    pub fn set_weight(&mut self, values: &[f32]) -> Result<(), BindingError> {
        let [weight] = values else {
            return Err(error("stream weight requires one scalar"));
        };
        self.inner.weight = *weight;
        Ok(())
    }
    pub fn emission(&self, index: usize) -> Option<GaussianF32> {
        self.inner
            .mixtures
            .get(index)
            .cloned()
            .map(|inner| GaussianF32 { inner })
    }
    pub fn set_emission(&mut self, index: usize, value: &GaussianF32) -> Result<(), BindingError> {
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
        weight: &[f32],
        emissions: &GaussianCollectionF32,
    ) -> Result<(), BindingError> {
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
    pub fn validate(&self) -> Result<(), BindingError> {
        self.inner.validate().map_err(error)
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
    pub fn prepare(&self) -> Result<PreparedStreamF32, BindingError> {
        self.cloned().prepare_owned()
    }
    pub fn prepare_owned(self) -> Result<PreparedStreamF32, BindingError> {
        self.inner
            .prepare_owned()
            .map(|inner| PreparedStreamF32 { inner })
            .map_err(error)
    }
}
#[derive(Clone)]
pub struct StreamBuilderF32 {
    inner: StreamParametersBuilder<f32>,
}
impl StreamBuilderF32 {
    pub fn new(emissions: usize) -> Result<Self, BindingError> {
        StreamParametersBuilder::new(emissions)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn emissions(&self) -> usize {
        self.inner.emissions.len()
    }
    pub fn weight(&self) -> Vec<f32> {
        vec![self.inner.weight]
    }
    pub fn set_weight(&mut self, values: &[f32]) -> Result<(), BindingError> {
        let [weight] = values else {
            return Err(error("stream weight requires one scalar"));
        };
        self.inner.weight = *weight;
        Ok(())
    }
    pub fn emission(&self, index: usize) -> Option<GaussianF32> {
        self.inner
            .emissions
            .get(index)
            .and_then(|value| value.clone())
            .map(|inner| GaussianF32 { inner })
    }
    pub fn has_emission(&self, index: usize) -> bool {
        self.inner.emissions.get(index).is_some_and(Option::is_some)
    }
    /// Retain existing slots; new slots are empty and removed slots drop.
    pub fn resize(&mut self, emissions: usize) -> Result<(), BindingError> {
        let mut inner = StreamParametersBuilder::new(emissions).map_err(error)?;
        inner.weight = self.inner.weight;
        for (target, source) in inner.emissions.iter_mut().zip(&self.inner.emissions) {
            target.clone_from(source);
        }
        self.inner = inner;
        Ok(())
    }
    pub fn set_emission(&mut self, index: usize, value: &GaussianF32) -> Result<(), BindingError> {
        let target = self
            .inner
            .emissions
            .get_mut(index)
            .ok_or_else(|| error("emission index out of range"))?;
        *target = Some(value.inner.clone());
        Ok(())
    }
    pub fn clear_emission(&mut self, index: usize) -> Result<(), BindingError> {
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
    pub fn build(self) -> Result<StreamF32, BindingError> {
        self.inner
            .build()
            .map(|inner| StreamF32 { inner })
            .map_err(error)
    }
}
#[derive(Clone)]
pub struct PreparedStreamF32 {
    pub(crate) inner: OwnedPreparedStreamParameters<f32>,
}
impl PreparedStreamF32 {
    pub fn cloned(&self) -> Self {
        self.clone()
    }
    pub fn parameters(&self) -> StreamF32 {
        StreamF32 {
            inner: self.inner.parameters().clone(),
        }
    }
    pub fn into_parameters(self) -> StreamF32 {
        StreamF32 {
            inner: self.inner.into_parameters(),
        }
    }
    pub fn emission(&self, state: usize) -> Option<PreparedGaussianF32> {
        self.inner.emission(state).map(|value| PreparedGaussianF32 {
            inner: value.to_owned(),
        })
    }
}
#[derive(Clone)]
pub struct ModelF32 {
    pub(crate) inner: ModelParameters<f32>,
}
impl ModelF32 {
    pub fn new(layouts: &[u32], durations: usize) -> Result<Self, BindingError> {
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
    pub fn streams(&self) -> usize {
        self.inner.streams.len()
    }
    pub fn durations(&self) -> usize {
        self.inner.durations.len()
    }
    pub fn stream(&self, index: usize) -> Option<StreamF32> {
        self.inner
            .streams
            .get(index)
            .cloned()
            .map(|inner| StreamF32 { inner })
    }
    pub fn duration(&self, index: usize) -> Option<DurationF32> {
        self.inner
            .durations
            .get(index)
            .cloned()
            .map(|inner| DurationF32 { inner })
    }
    pub fn set_stream(&mut self, index: usize, value: &StreamF32) -> Result<(), BindingError> {
        *self
            .inner
            .streams
            .get_mut(index)
            .ok_or_else(|| error("stream index out of range"))? = value.inner.clone();
        Ok(())
    }
    pub fn set_duration(&mut self, index: usize, value: &DurationF32) -> Result<(), BindingError> {
        *self
            .inner
            .durations
            .get_mut(index)
            .ok_or_else(|| error("duration index out of range"))? = value.inner.clone();
        Ok(())
    }
    pub fn replace(
        &mut self,
        streams: &StreamCollectionF32,
        durations: &DurationCollectionF32,
    ) -> Result<(), BindingError> {
        let inner = ModelParameters {
            streams: streams.inner.clone(),
            durations: durations.inner.clone(),
        };
        inner.validate().map_err(error)?;
        self.inner = inner;
        Ok(())
    }
    pub fn validate(&self) -> Result<(), BindingError> {
        self.inner.validate().map_err(error)
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
    pub fn prepare(
        &self,
        options: &PreparationOptionsF32,
    ) -> Result<PreparedModelF32, BindingError> {
        self.cloned().prepare_owned(options)
    }
    pub fn prepare_owned(
        self,
        options: &PreparationOptionsF32,
    ) -> Result<PreparedModelF32, BindingError> {
        self.inner
            .prepare_owned(options.native())
            .map(|inner| PreparedModelF32 { inner })
            .map_err(error)
    }
    pub fn prepare_selected_durations(
        &self,
        options: &PreparationOptionsF32,
        states: &[u32],
    ) -> Result<PreparedModelF32, BindingError> {
        self.cloned()
            .prepare_owned_selected_durations(options, states)
    }
    pub fn prepare_owned_selected_durations(
        self,
        options: &PreparationOptionsF32,
        states: &[u32],
    ) -> Result<PreparedModelF32, BindingError> {
        let states = states
            .iter()
            .map(|value| *value as usize)
            .collect::<Vec<_>>();
        self.inner
            .prepare_owned_selected_durations(options.native(), &states)
            .map(|inner| PreparedModelF32 { inner })
            .map_err(error)
    }
}
#[derive(Clone)]
pub struct ModelBuilderF32 {
    inner: ModelParametersBuilder<f32>,
}
impl ModelBuilderF32 {
    pub fn new(streams: usize, durations: usize) -> Result<Self, BindingError> {
        ModelParametersBuilder::new(streams, durations)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn streams(&self) -> usize {
        self.inner.streams.len()
    }
    pub fn durations(&self) -> usize {
        self.inner.durations.len()
    }
    pub fn stream(&self, index: usize) -> Option<StreamF32> {
        self.inner
            .streams
            .get(index)
            .and_then(|value| value.clone())
            .map(|inner| StreamF32 { inner })
    }
    pub fn duration(&self, index: usize) -> Option<DurationF32> {
        self.inner
            .durations
            .get(index)
            .and_then(|value| value.clone())
            .map(|inner| DurationF32 { inner })
    }
    pub fn has_stream(&self, index: usize) -> bool {
        self.inner.streams.get(index).is_some_and(Option::is_some)
    }
    pub fn has_duration(&self, index: usize) -> bool {
        self.inner.durations.get(index).is_some_and(Option::is_some)
    }
    /// Resize both public slot lists atomically, retaining their prefix.
    pub fn resize(&mut self, streams: usize, durations: usize) -> Result<(), BindingError> {
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
    pub fn set_stream(&mut self, index: usize, value: &StreamF32) -> Result<(), BindingError> {
        *self
            .inner
            .streams
            .get_mut(index)
            .ok_or_else(|| error("stream index out of range"))? = Some(value.inner.clone());
        Ok(())
    }
    pub fn set_duration(&mut self, index: usize, value: &DurationF32) -> Result<(), BindingError> {
        *self
            .inner
            .durations
            .get_mut(index)
            .ok_or_else(|| error("duration index out of range"))? = Some(value.inner.clone());
        Ok(())
    }
    pub fn clear_stream(&mut self, index: usize) -> Result<(), BindingError> {
        *self
            .inner
            .streams
            .get_mut(index)
            .ok_or_else(|| error("stream index out of range"))? = None;
        Ok(())
    }
    pub fn clear_duration(&mut self, index: usize) -> Result<(), BindingError> {
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
    pub fn build(self) -> Result<ModelF32, BindingError> {
        self.inner
            .build()
            .map(|inner| ModelF32 { inner })
            .map_err(error)
    }
}
#[derive(Clone)]
pub struct PreparedModelF32 {
    pub(crate) inner: OwnedPreparedModelParameters<f32>,
}
impl PreparedModelF32 {
    pub fn cloned(&self) -> Self {
        self.clone()
    }
    pub fn parameters(&self) -> ModelF32 {
        ModelF32 {
            inner: self.inner.model().clone(),
        }
    }
    pub fn into_parameters(self) -> ModelF32 {
        ModelF32 {
            inner: self.inner.into_model(),
        }
    }
    pub fn options(&self) -> PreparationOptionsF32 {
        let value = self.inner.options();
        PreparationOptionsF32 {
            duration_cache_length: value.duration_cache_length,
            duration_weight: value.duration_weight,
            temperature: value.temperature,
        }
    }
    pub fn duration(&self, state: usize) -> Option<PreparedDurationF32> {
        self.inner.duration(state).map(|value| PreparedDurationF32 {
            inner: value.to_owned(),
        })
    }
    pub fn emission(&self, stream: usize, state: usize) -> Option<PreparedGaussianF32> {
        self.inner
            .emission(stream, state)
            .map(|value| PreparedGaussianF32 {
                inner: value.to_owned(),
            })
    }
}
#[derive(Clone, Default)]
pub struct StreamCollectionF64 {
    pub(crate) inner: Vec<StreamParameters<f64>>,
}
impl StreamCollectionF64 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn length(&self) -> usize {
        self.inner.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn push(&mut self, value: &StreamF64) {
        self.inner.push(value.inner.clone());
    }
    pub fn get(&self, index: usize) -> Option<StreamF64> {
        self.inner
            .get(index)
            .cloned()
            .map(|inner| StreamF64 { inner })
    }
    pub fn replace(&mut self, index: usize, value: &StreamF64) -> Result<(), BindingError> {
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
pub struct StreamF64 {
    pub(crate) inner: StreamParameters<f64>,
}
impl StreamF64 {
    pub fn new(
        emissions: usize,
        components: usize,
        dimensions: usize,
    ) -> Result<Self, BindingError> {
        StreamParameters::new(emissions, components, dimensions)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn empty() -> Self {
        Self {
            inner: StreamParameters::default(),
        }
    }
    pub fn emissions(&self) -> usize {
        self.inner.mixtures.len()
    }
    pub fn weight(&self) -> Vec<f64> {
        vec![self.inner.weight]
    }
    pub fn set_weight(&mut self, values: &[f64]) -> Result<(), BindingError> {
        let [weight] = values else {
            return Err(error("stream weight requires one scalar"));
        };
        self.inner.weight = *weight;
        Ok(())
    }
    pub fn emission(&self, index: usize) -> Option<GaussianF64> {
        self.inner
            .mixtures
            .get(index)
            .cloned()
            .map(|inner| GaussianF64 { inner })
    }
    pub fn set_emission(&mut self, index: usize, value: &GaussianF64) -> Result<(), BindingError> {
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
        weight: &[f64],
        emissions: &GaussianCollectionF64,
    ) -> Result<(), BindingError> {
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
    pub fn validate(&self) -> Result<(), BindingError> {
        self.inner.validate().map_err(error)
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
    pub fn prepare(&self) -> Result<PreparedStreamF64, BindingError> {
        self.cloned().prepare_owned()
    }
    pub fn prepare_owned(self) -> Result<PreparedStreamF64, BindingError> {
        self.inner
            .prepare_owned()
            .map(|inner| PreparedStreamF64 { inner })
            .map_err(error)
    }
}
#[derive(Clone)]
pub struct StreamBuilderF64 {
    inner: StreamParametersBuilder<f64>,
}
impl StreamBuilderF64 {
    pub fn new(emissions: usize) -> Result<Self, BindingError> {
        StreamParametersBuilder::new(emissions)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn emissions(&self) -> usize {
        self.inner.emissions.len()
    }
    pub fn weight(&self) -> Vec<f64> {
        vec![self.inner.weight]
    }
    pub fn set_weight(&mut self, values: &[f64]) -> Result<(), BindingError> {
        let [weight] = values else {
            return Err(error("stream weight requires one scalar"));
        };
        self.inner.weight = *weight;
        Ok(())
    }
    pub fn emission(&self, index: usize) -> Option<GaussianF64> {
        self.inner
            .emissions
            .get(index)
            .and_then(|value| value.clone())
            .map(|inner| GaussianF64 { inner })
    }
    pub fn has_emission(&self, index: usize) -> bool {
        self.inner.emissions.get(index).is_some_and(Option::is_some)
    }
    /// Retain existing slots; new slots are empty and removed slots drop.
    pub fn resize(&mut self, emissions: usize) -> Result<(), BindingError> {
        let mut inner = StreamParametersBuilder::new(emissions).map_err(error)?;
        inner.weight = self.inner.weight;
        for (target, source) in inner.emissions.iter_mut().zip(&self.inner.emissions) {
            target.clone_from(source);
        }
        self.inner = inner;
        Ok(())
    }
    pub fn set_emission(&mut self, index: usize, value: &GaussianF64) -> Result<(), BindingError> {
        let target = self
            .inner
            .emissions
            .get_mut(index)
            .ok_or_else(|| error("emission index out of range"))?;
        *target = Some(value.inner.clone());
        Ok(())
    }
    pub fn clear_emission(&mut self, index: usize) -> Result<(), BindingError> {
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
    pub fn build(self) -> Result<StreamF64, BindingError> {
        self.inner
            .build()
            .map(|inner| StreamF64 { inner })
            .map_err(error)
    }
}
#[derive(Clone)]
pub struct PreparedStreamF64 {
    pub(crate) inner: OwnedPreparedStreamParameters<f64>,
}
impl PreparedStreamF64 {
    pub fn cloned(&self) -> Self {
        self.clone()
    }
    pub fn parameters(&self) -> StreamF64 {
        StreamF64 {
            inner: self.inner.parameters().clone(),
        }
    }
    pub fn into_parameters(self) -> StreamF64 {
        StreamF64 {
            inner: self.inner.into_parameters(),
        }
    }
    pub fn emission(&self, state: usize) -> Option<PreparedGaussianF64> {
        self.inner.emission(state).map(|value| PreparedGaussianF64 {
            inner: value.to_owned(),
        })
    }
}
#[derive(Clone)]
pub struct ModelF64 {
    pub(crate) inner: ModelParameters<f64>,
}
impl ModelF64 {
    pub fn new(layouts: &[u32], durations: usize) -> Result<Self, BindingError> {
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
    pub fn streams(&self) -> usize {
        self.inner.streams.len()
    }
    pub fn durations(&self) -> usize {
        self.inner.durations.len()
    }
    pub fn stream(&self, index: usize) -> Option<StreamF64> {
        self.inner
            .streams
            .get(index)
            .cloned()
            .map(|inner| StreamF64 { inner })
    }
    pub fn duration(&self, index: usize) -> Option<DurationF64> {
        self.inner
            .durations
            .get(index)
            .cloned()
            .map(|inner| DurationF64 { inner })
    }
    pub fn set_stream(&mut self, index: usize, value: &StreamF64) -> Result<(), BindingError> {
        *self
            .inner
            .streams
            .get_mut(index)
            .ok_or_else(|| error("stream index out of range"))? = value.inner.clone();
        Ok(())
    }
    pub fn set_duration(&mut self, index: usize, value: &DurationF64) -> Result<(), BindingError> {
        *self
            .inner
            .durations
            .get_mut(index)
            .ok_or_else(|| error("duration index out of range"))? = value.inner.clone();
        Ok(())
    }
    pub fn replace(
        &mut self,
        streams: &StreamCollectionF64,
        durations: &DurationCollectionF64,
    ) -> Result<(), BindingError> {
        let inner = ModelParameters {
            streams: streams.inner.clone(),
            durations: durations.inner.clone(),
        };
        inner.validate().map_err(error)?;
        self.inner = inner;
        Ok(())
    }
    pub fn validate(&self) -> Result<(), BindingError> {
        self.inner.validate().map_err(error)
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
    pub fn prepare(
        &self,
        options: &PreparationOptionsF64,
    ) -> Result<PreparedModelF64, BindingError> {
        self.cloned().prepare_owned(options)
    }
    pub fn prepare_owned(
        self,
        options: &PreparationOptionsF64,
    ) -> Result<PreparedModelF64, BindingError> {
        self.inner
            .prepare_owned(options.native())
            .map(|inner| PreparedModelF64 { inner })
            .map_err(error)
    }
    pub fn prepare_selected_durations(
        &self,
        options: &PreparationOptionsF64,
        states: &[u32],
    ) -> Result<PreparedModelF64, BindingError> {
        self.cloned()
            .prepare_owned_selected_durations(options, states)
    }
    pub fn prepare_owned_selected_durations(
        self,
        options: &PreparationOptionsF64,
        states: &[u32],
    ) -> Result<PreparedModelF64, BindingError> {
        let states = states
            .iter()
            .map(|value| *value as usize)
            .collect::<Vec<_>>();
        self.inner
            .prepare_owned_selected_durations(options.native(), &states)
            .map(|inner| PreparedModelF64 { inner })
            .map_err(error)
    }
}
#[derive(Clone)]
pub struct ModelBuilderF64 {
    inner: ModelParametersBuilder<f64>,
}
impl ModelBuilderF64 {
    pub fn new(streams: usize, durations: usize) -> Result<Self, BindingError> {
        ModelParametersBuilder::new(streams, durations)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn streams(&self) -> usize {
        self.inner.streams.len()
    }
    pub fn durations(&self) -> usize {
        self.inner.durations.len()
    }
    pub fn stream(&self, index: usize) -> Option<StreamF64> {
        self.inner
            .streams
            .get(index)
            .and_then(|value| value.clone())
            .map(|inner| StreamF64 { inner })
    }
    pub fn duration(&self, index: usize) -> Option<DurationF64> {
        self.inner
            .durations
            .get(index)
            .and_then(|value| value.clone())
            .map(|inner| DurationF64 { inner })
    }
    pub fn has_stream(&self, index: usize) -> bool {
        self.inner.streams.get(index).is_some_and(Option::is_some)
    }
    pub fn has_duration(&self, index: usize) -> bool {
        self.inner.durations.get(index).is_some_and(Option::is_some)
    }
    /// Resize both public slot lists atomically, retaining their prefix.
    pub fn resize(&mut self, streams: usize, durations: usize) -> Result<(), BindingError> {
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
    pub fn set_stream(&mut self, index: usize, value: &StreamF64) -> Result<(), BindingError> {
        *self
            .inner
            .streams
            .get_mut(index)
            .ok_or_else(|| error("stream index out of range"))? = Some(value.inner.clone());
        Ok(())
    }
    pub fn set_duration(&mut self, index: usize, value: &DurationF64) -> Result<(), BindingError> {
        *self
            .inner
            .durations
            .get_mut(index)
            .ok_or_else(|| error("duration index out of range"))? = Some(value.inner.clone());
        Ok(())
    }
    pub fn clear_stream(&mut self, index: usize) -> Result<(), BindingError> {
        *self
            .inner
            .streams
            .get_mut(index)
            .ok_or_else(|| error("stream index out of range"))? = None;
        Ok(())
    }
    pub fn clear_duration(&mut self, index: usize) -> Result<(), BindingError> {
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
    pub fn build(self) -> Result<ModelF64, BindingError> {
        self.inner
            .build()
            .map(|inner| ModelF64 { inner })
            .map_err(error)
    }
}
#[derive(Clone)]
pub struct PreparedModelF64 {
    pub(crate) inner: OwnedPreparedModelParameters<f64>,
}
impl PreparedModelF64 {
    pub fn cloned(&self) -> Self {
        self.clone()
    }
    pub fn parameters(&self) -> ModelF64 {
        ModelF64 {
            inner: self.inner.model().clone(),
        }
    }
    pub fn into_parameters(self) -> ModelF64 {
        ModelF64 {
            inner: self.inner.into_model(),
        }
    }
    pub fn options(&self) -> PreparationOptionsF64 {
        let value = self.inner.options();
        PreparationOptionsF64 {
            duration_cache_length: value.duration_cache_length,
            duration_weight: value.duration_weight,
            temperature: value.temperature,
        }
    }
    pub fn duration(&self, state: usize) -> Option<PreparedDurationF64> {
        self.inner.duration(state).map(|value| PreparedDurationF64 {
            inner: value.to_owned(),
        })
    }
    pub fn emission(&self, stream: usize, state: usize) -> Option<PreparedGaussianF64> {
        self.inner
            .emission(stream, state)
            .map(|value| PreparedGaussianF64 {
                inner: value.to_owned(),
            })
    }
}
