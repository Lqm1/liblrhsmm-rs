fn new_err_owned(message: &str) -> pyo3::PyErr {
    pyo3::exceptions::PyValueError::new_err(message.to_owned())
}
use super::*;
use crate::statistics::{
    DurationSufficientStatistics, GaussianSufficientStatistics, ModelSufficientStatistics,
    StreamSufficientStatistics,
};
use pyo3::PyErr as BindingError;
fn error(value: impl std::fmt::Display) -> BindingError {
    new_err_owned(&value.to_string())
}
#[derive(Clone, Default)]
pub struct GaussianStatisticsCollectionF32 {
    pub(crate) inner: Vec<GaussianSufficientStatistics<f32>>,
}
impl GaussianStatisticsCollectionF32 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn length(&self) -> usize {
        self.inner.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn push(&mut self, value: &GaussianStatisticsF32) {
        self.inner.push(value.inner.clone());
    }
    pub fn get(&self, index: usize) -> Option<GaussianStatisticsF32> {
        self.inner
            .get(index)
            .cloned()
            .map(|inner| GaussianStatisticsF32 { inner })
    }
    pub fn replace(
        &mut self,
        index: usize,
        value: &GaussianStatisticsF32,
    ) -> Result<(), BindingError> {
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
#[derive(Clone, Default)]
pub struct StreamStatisticsCollectionF32 {
    pub(crate) inner: Vec<StreamSufficientStatistics<f32>>,
}
impl StreamStatisticsCollectionF32 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn length(&self) -> usize {
        self.inner.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn push(&mut self, value: &StreamStatisticsF32) {
        self.inner.push(value.inner.clone());
    }
    pub fn get(&self, index: usize) -> Option<StreamStatisticsF32> {
        self.inner
            .get(index)
            .cloned()
            .map(|inner| StreamStatisticsF32 { inner })
    }
    pub fn replace(
        &mut self,
        index: usize,
        value: &StreamStatisticsF32,
    ) -> Result<(), BindingError> {
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
#[derive(Clone, Default)]
pub struct DurationStatisticsCollectionF32 {
    inner: Vec<DurationSufficientStatistics<f32>>,
}
impl DurationStatisticsCollectionF32 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn length(&self) -> usize {
        self.inner.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn push(&mut self, value: &DurationStatisticsF32) {
        self.inner.push(value.native());
    }
    pub fn get(&self, index: usize) -> Option<DurationStatisticsF32> {
        self.inner
            .get(index)
            .copied()
            .map(DurationStatisticsF32::from_native)
    }
    pub fn replace(
        &mut self,
        index: usize,
        value: &DurationStatisticsF32,
    ) -> Result<(), BindingError> {
        *self
            .inner
            .get_mut(index)
            .ok_or_else(|| error("duration statistic index out of range"))? = value.native();
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
pub struct StreamStatisticsF32 {
    pub(crate) inner: StreamSufficientStatistics<f32>,
}
impl StreamStatisticsF32 {
    pub fn new() -> Self {
        Self {
            inner: StreamSufficientStatistics {
                mixtures: Vec::new(),
            },
        }
    }
    pub fn from_stream(value: &StreamF32) -> Result<Self, BindingError> {
        StreamSufficientStatistics::from_stream(&value.inner)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn emissions(&self) -> usize {
        self.inner.mixtures.len()
    }
    pub fn emission(&self, index: usize) -> Option<GaussianStatisticsF32> {
        self.inner
            .mixtures
            .get(index)
            .cloned()
            .map(|inner| GaussianStatisticsF32 { inner })
    }
    pub fn set_emission(
        &mut self,
        index: usize,
        value: &GaussianStatisticsF32,
    ) -> Result<(), BindingError> {
        *self
            .inner
            .mixtures
            .get_mut(index)
            .ok_or_else(|| error("emission statistic index out of range"))? = value.inner.clone();
        Ok(())
    }
    pub fn replace(&mut self, values: &GaussianStatisticsCollectionF32) {
        self.inner.mixtures = values.inner.clone();
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
    pub fn clear(&mut self) {
        self.inner.clear();
    }
}
impl Default for StreamStatisticsF32 {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone)]
pub struct ModelStatisticsF32 {
    pub(crate) inner: ModelSufficientStatistics<f32>,
}
impl ModelStatisticsF32 {
    pub fn new() -> Self {
        Self {
            inner: ModelSufficientStatistics {
                streams: Vec::new(),
                durations: Vec::new(),
            },
        }
    }
    pub fn from_model(value: &ModelF32) -> Result<Self, BindingError> {
        ModelSufficientStatistics::from_model(&value.inner)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn streams(&self) -> usize {
        self.inner.streams.len()
    }
    pub fn durations(&self) -> usize {
        self.inner.durations.len()
    }
    pub fn stream(&self, index: usize) -> Option<StreamStatisticsF32> {
        self.inner
            .streams
            .get(index)
            .cloned()
            .map(|inner| StreamStatisticsF32 { inner })
    }
    pub fn duration(&self, index: usize) -> Option<DurationStatisticsF32> {
        self.inner
            .durations
            .get(index)
            .copied()
            .map(DurationStatisticsF32::from_native)
    }
    pub fn set_stream(
        &mut self,
        index: usize,
        value: &StreamStatisticsF32,
    ) -> Result<(), BindingError> {
        *self
            .inner
            .streams
            .get_mut(index)
            .ok_or_else(|| error("stream statistic index out of range"))? = value.inner.clone();
        Ok(())
    }
    pub fn set_duration(
        &mut self,
        index: usize,
        value: &DurationStatisticsF32,
    ) -> Result<(), BindingError> {
        *self
            .inner
            .durations
            .get_mut(index)
            .ok_or_else(|| error("duration statistic index out of range"))? = value.native();
        Ok(())
    }
    pub fn replace(
        &mut self,
        streams: &StreamStatisticsCollectionF32,
        durations: &DurationStatisticsCollectionF32,
    ) {
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
impl Default for ModelStatisticsF32 {
    fn default() -> Self {
        Self::new()
    }
}
impl StreamF32 {
    pub fn update(
        &mut self,
        statistics: &StreamStatisticsF32,
        options: &UpdateOptionsF32,
    ) -> Result<(), BindingError> {
        self.inner
            .update(&statistics.inner, options.native()?)
            .map_err(error)
    }
}
impl ModelF32 {
    pub fn update(
        &mut self,
        statistics: &ModelStatisticsF32,
        options: &UpdateOptionsF32,
    ) -> Result<(), BindingError> {
        self.inner
            .update(&statistics.inner, options.native()?)
            .map_err(error)
    }
}
#[derive(Clone, Default)]
pub struct GaussianStatisticsCollectionF64 {
    pub(crate) inner: Vec<GaussianSufficientStatistics<f64>>,
}
impl GaussianStatisticsCollectionF64 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn length(&self) -> usize {
        self.inner.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn push(&mut self, value: &GaussianStatisticsF64) {
        self.inner.push(value.inner.clone());
    }
    pub fn get(&self, index: usize) -> Option<GaussianStatisticsF64> {
        self.inner
            .get(index)
            .cloned()
            .map(|inner| GaussianStatisticsF64 { inner })
    }
    pub fn replace(
        &mut self,
        index: usize,
        value: &GaussianStatisticsF64,
    ) -> Result<(), BindingError> {
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
#[derive(Clone, Default)]
pub struct StreamStatisticsCollectionF64 {
    pub(crate) inner: Vec<StreamSufficientStatistics<f64>>,
}
impl StreamStatisticsCollectionF64 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn length(&self) -> usize {
        self.inner.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn push(&mut self, value: &StreamStatisticsF64) {
        self.inner.push(value.inner.clone());
    }
    pub fn get(&self, index: usize) -> Option<StreamStatisticsF64> {
        self.inner
            .get(index)
            .cloned()
            .map(|inner| StreamStatisticsF64 { inner })
    }
    pub fn replace(
        &mut self,
        index: usize,
        value: &StreamStatisticsF64,
    ) -> Result<(), BindingError> {
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
#[derive(Clone, Default)]
pub struct DurationStatisticsCollectionF64 {
    inner: Vec<DurationSufficientStatistics<f64>>,
}
impl DurationStatisticsCollectionF64 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn length(&self) -> usize {
        self.inner.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn push(&mut self, value: &DurationStatisticsF64) {
        self.inner.push(value.native());
    }
    pub fn get(&self, index: usize) -> Option<DurationStatisticsF64> {
        self.inner
            .get(index)
            .copied()
            .map(DurationStatisticsF64::from_native)
    }
    pub fn replace(
        &mut self,
        index: usize,
        value: &DurationStatisticsF64,
    ) -> Result<(), BindingError> {
        *self
            .inner
            .get_mut(index)
            .ok_or_else(|| error("duration statistic index out of range"))? = value.native();
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
pub struct StreamStatisticsF64 {
    pub(crate) inner: StreamSufficientStatistics<f64>,
}
impl StreamStatisticsF64 {
    pub fn new() -> Self {
        Self {
            inner: StreamSufficientStatistics {
                mixtures: Vec::new(),
            },
        }
    }
    pub fn from_stream(value: &StreamF64) -> Result<Self, BindingError> {
        StreamSufficientStatistics::from_stream(&value.inner)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn emissions(&self) -> usize {
        self.inner.mixtures.len()
    }
    pub fn emission(&self, index: usize) -> Option<GaussianStatisticsF64> {
        self.inner
            .mixtures
            .get(index)
            .cloned()
            .map(|inner| GaussianStatisticsF64 { inner })
    }
    pub fn set_emission(
        &mut self,
        index: usize,
        value: &GaussianStatisticsF64,
    ) -> Result<(), BindingError> {
        *self
            .inner
            .mixtures
            .get_mut(index)
            .ok_or_else(|| error("emission statistic index out of range"))? = value.inner.clone();
        Ok(())
    }
    pub fn replace(&mut self, values: &GaussianStatisticsCollectionF64) {
        self.inner.mixtures = values.inner.clone();
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
    pub fn clear(&mut self) {
        self.inner.clear();
    }
}
impl Default for StreamStatisticsF64 {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone)]
pub struct ModelStatisticsF64 {
    pub(crate) inner: ModelSufficientStatistics<f64>,
}
impl ModelStatisticsF64 {
    pub fn new() -> Self {
        Self {
            inner: ModelSufficientStatistics {
                streams: Vec::new(),
                durations: Vec::new(),
            },
        }
    }
    pub fn from_model(value: &ModelF64) -> Result<Self, BindingError> {
        ModelSufficientStatistics::from_model(&value.inner)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn streams(&self) -> usize {
        self.inner.streams.len()
    }
    pub fn durations(&self) -> usize {
        self.inner.durations.len()
    }
    pub fn stream(&self, index: usize) -> Option<StreamStatisticsF64> {
        self.inner
            .streams
            .get(index)
            .cloned()
            .map(|inner| StreamStatisticsF64 { inner })
    }
    pub fn duration(&self, index: usize) -> Option<DurationStatisticsF64> {
        self.inner
            .durations
            .get(index)
            .copied()
            .map(DurationStatisticsF64::from_native)
    }
    pub fn set_stream(
        &mut self,
        index: usize,
        value: &StreamStatisticsF64,
    ) -> Result<(), BindingError> {
        *self
            .inner
            .streams
            .get_mut(index)
            .ok_or_else(|| error("stream statistic index out of range"))? = value.inner.clone();
        Ok(())
    }
    pub fn set_duration(
        &mut self,
        index: usize,
        value: &DurationStatisticsF64,
    ) -> Result<(), BindingError> {
        *self
            .inner
            .durations
            .get_mut(index)
            .ok_or_else(|| error("duration statistic index out of range"))? = value.native();
        Ok(())
    }
    pub fn replace(
        &mut self,
        streams: &StreamStatisticsCollectionF64,
        durations: &DurationStatisticsCollectionF64,
    ) {
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
impl Default for ModelStatisticsF64 {
    fn default() -> Self {
        Self::new()
    }
}
impl StreamF64 {
    pub fn update(
        &mut self,
        statistics: &StreamStatisticsF64,
        options: &UpdateOptionsF64,
    ) -> Result<(), BindingError> {
        self.inner
            .update(&statistics.inner, options.native()?)
            .map_err(error)
    }
}
impl ModelF64 {
    pub fn update(
        &mut self,
        statistics: &ModelStatisticsF64,
        options: &UpdateOptionsF64,
    ) -> Result<(), BindingError> {
        self.inner
            .update(&statistics.inner, options.native()?)
            .map_err(error)
    }
}
