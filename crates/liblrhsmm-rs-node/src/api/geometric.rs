use super::*;
use crate::geometric::{GeometricPath, GeometricSettings, StateScoreTable};
use napi::Error as BindingError;
fn error(value: impl std::fmt::Display) -> BindingError {
    napi::Error::from_reason(value.to_string())
}
#[derive(Clone)]
pub struct GeometricOptionsF32 {
    pub temperature: f32,
    pub pruning_slope: f32,
}
impl Default for GeometricOptionsF32 {
    fn default() -> Self {
        Self::new()
    }
}
impl GeometricOptionsF32 {
    pub(crate) fn native(&self) -> GeometricSettings<f32> {
        GeometricSettings {
            temperature: self.temperature,
            pruning_slope: self.pruning_slope,
        }
    }
}
impl GeometricOptionsF32 {
    pub fn new() -> Self {
        let value = GeometricSettings::<f32>::default();
        Self {
            temperature: value.temperature,
            pruning_slope: value.pruning_slope,
        }
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
#[derive(Clone)]
pub struct StateScoresF32 {
    pub(crate) inner: StateScoreTable<f32>,
}
impl StateScoresF32 {
    pub fn new(frames: usize, states: usize, values: &[f32]) -> Result<Self, BindingError> {
        StateScoreTable::from_values(frames, states, values.to_vec())
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn frames(&self) -> usize {
        self.inner.frames()
    }
    pub fn states(&self) -> usize {
        self.inner.states()
    }
    pub fn values(&self) -> Vec<f32> {
        self.inner.values().to_vec()
    }
    pub fn get(&self, frame: usize, state: usize) -> Option<f32> {
        self.inner.get(frame, state)
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
    pub fn total_log_probability(&self) -> f32 {
        self.inner.total_log_probability()
    }
    pub fn total_through_state(
        &self,
        backward: &StateScoresF32,
        state: usize,
    ) -> Result<f32, BindingError> {
        self.inner
            .total_through_state(&backward.inner, state)
            .map_err(error)
    }
    pub fn backward_log_probability(
        &self,
        emissions: &LogEmissionsF32,
    ) -> Result<f32, BindingError> {
        self.inner
            .backward_log_probability(&emissions.inner)
            .map_err(error)
    }
}
#[derive(Clone)]
pub struct GeometricPathF32 {
    pub(crate) inner: GeometricPath<f32>,
}
impl GeometricPathF32 {
    pub fn new(
        states: &[u32],
        boundaries: &[u32],
        log_probability: &[f32],
        scores: &StateScoresF32,
    ) -> Result<Self, BindingError> {
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
    pub fn log_probability(&self) -> Vec<f32> {
        vec![self.inner.log_probability]
    }
    pub fn set_log_probability(&mut self, values: &[f32]) -> Result<(), BindingError> {
        let [value] = values else {
            return Err(error("path log probability requires one scalar"));
        };
        self.inner.log_probability = *value;
        Ok(())
    }
    pub fn scores(&self) -> StateScoresF32 {
        StateScoresF32 {
            inner: self.inner.scores.clone(),
        }
    }
    pub fn set_scores(&mut self, value: &StateScoresF32) {
        self.inner.scores = value.inner.clone();
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
impl ModelF32 {
    pub fn forward_geometric(
        &self,
        segmentation: &SegmentationF32,
        emissions: &LogEmissionsF32,
        options: &GeometricOptionsF32,
    ) -> Result<StateScoresF32, BindingError> {
        self.inner
            .forward_geometric(&segmentation.inner, &emissions.inner, options.native())
            .map(|inner| StateScoresF32 { inner })
            .map_err(error)
    }
    pub fn backward_geometric(
        &self,
        segmentation: &SegmentationF32,
        emissions: &LogEmissionsF32,
        options: &GeometricOptionsF32,
    ) -> Result<StateScoresF32, BindingError> {
        self.inner
            .backward_geometric(&segmentation.inner, &emissions.inner, options.native())
            .map(|inner| StateScoresF32 { inner })
            .map_err(error)
    }
    pub fn viterbi_geometric(
        &self,
        segmentation: &SegmentationF32,
        emissions: &LogEmissionsF32,
        options: &GeometricOptionsF32,
    ) -> Result<GeometricPathF32, BindingError> {
        self.inner
            .viterbi_geometric(&segmentation.inner, &emissions.inner, options.native())
            .map(|inner| GeometricPathF32 { inner })
            .map_err(error)
    }
}
#[derive(Clone)]
pub struct GeometricOptionsF64 {
    pub temperature: f64,
    pub pruning_slope: f64,
}
impl Default for GeometricOptionsF64 {
    fn default() -> Self {
        Self::new()
    }
}
impl GeometricOptionsF64 {
    pub(crate) fn native(&self) -> GeometricSettings<f64> {
        GeometricSettings {
            temperature: self.temperature,
            pruning_slope: self.pruning_slope,
        }
    }
}
impl GeometricOptionsF64 {
    pub fn new() -> Self {
        let value = GeometricSettings::<f64>::default();
        Self {
            temperature: value.temperature,
            pruning_slope: value.pruning_slope,
        }
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
#[derive(Clone)]
pub struct StateScoresF64 {
    pub(crate) inner: StateScoreTable<f64>,
}
impl StateScoresF64 {
    pub fn new(frames: usize, states: usize, values: &[f64]) -> Result<Self, BindingError> {
        StateScoreTable::from_values(frames, states, values.to_vec())
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn frames(&self) -> usize {
        self.inner.frames()
    }
    pub fn states(&self) -> usize {
        self.inner.states()
    }
    pub fn values(&self) -> Vec<f64> {
        self.inner.values().to_vec()
    }
    pub fn get(&self, frame: usize, state: usize) -> Option<f64> {
        self.inner.get(frame, state)
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
    pub fn total_log_probability(&self) -> f64 {
        self.inner.total_log_probability()
    }
    pub fn total_through_state(
        &self,
        backward: &StateScoresF64,
        state: usize,
    ) -> Result<f64, BindingError> {
        self.inner
            .total_through_state(&backward.inner, state)
            .map_err(error)
    }
    pub fn backward_log_probability(
        &self,
        emissions: &LogEmissionsF64,
    ) -> Result<f64, BindingError> {
        self.inner
            .backward_log_probability(&emissions.inner)
            .map_err(error)
    }
}
#[derive(Clone)]
pub struct GeometricPathF64 {
    pub(crate) inner: GeometricPath<f64>,
}
impl GeometricPathF64 {
    pub fn new(
        states: &[u32],
        boundaries: &[u32],
        log_probability: &[f64],
        scores: &StateScoresF64,
    ) -> Result<Self, BindingError> {
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
    pub fn log_probability(&self) -> Vec<f64> {
        vec![self.inner.log_probability]
    }
    pub fn set_log_probability(&mut self, values: &[f64]) -> Result<(), BindingError> {
        let [value] = values else {
            return Err(error("path log probability requires one scalar"));
        };
        self.inner.log_probability = *value;
        Ok(())
    }
    pub fn scores(&self) -> StateScoresF64 {
        StateScoresF64 {
            inner: self.inner.scores.clone(),
        }
    }
    pub fn set_scores(&mut self, value: &StateScoresF64) {
        self.inner.scores = value.inner.clone();
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
impl ModelF64 {
    pub fn forward_geometric(
        &self,
        segmentation: &SegmentationF64,
        emissions: &LogEmissionsF64,
        options: &GeometricOptionsF64,
    ) -> Result<StateScoresF64, BindingError> {
        self.inner
            .forward_geometric(&segmentation.inner, &emissions.inner, options.native())
            .map(|inner| StateScoresF64 { inner })
            .map_err(error)
    }
    pub fn backward_geometric(
        &self,
        segmentation: &SegmentationF64,
        emissions: &LogEmissionsF64,
        options: &GeometricOptionsF64,
    ) -> Result<StateScoresF64, BindingError> {
        self.inner
            .backward_geometric(&segmentation.inner, &emissions.inner, options.native())
            .map(|inner| StateScoresF64 { inner })
            .map_err(error)
    }
    pub fn viterbi_geometric(
        &self,
        segmentation: &SegmentationF64,
        emissions: &LogEmissionsF64,
        options: &GeometricOptionsF64,
    ) -> Result<GeometricPathF64, BindingError> {
        self.inner
            .viterbi_geometric(&segmentation.inner, &emissions.inner, options.native())
            .map(|inner| GeometricPathF64 { inner })
            .map_err(error)
    }
}
