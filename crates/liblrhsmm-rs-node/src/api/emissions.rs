use super::*;
use crate::emission::{EmissionDensityTable, EmissionPruningOptions, LogEmissionTable};
use napi::Error as BindingError;
fn error(value: impl std::fmt::Display) -> BindingError {
    napi::Error::from_reason(value.to_string())
}
/// Pruning mode: 0 evaluates all pairs, 1 uses a boundary radius,
/// and 2 uses a linear slope. The parameter retains selected precision.
#[derive(Clone)]
pub struct EmissionPruningF32 {
    pub mode: u32,
    pub parameter: f32,
}
impl EmissionPruningF32 {
    pub(crate) fn native(&self) -> Result<EmissionPruningOptions<f32>, BindingError> {
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
impl EmissionPruningF32 {
    pub fn new(mode: u32, parameter: f32) -> Self {
        Self { mode, parameter }
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
#[derive(Clone)]
pub struct LogEmissionsF32 {
    pub(crate) inner: LogEmissionTable<f32>,
}
impl LogEmissionsF32 {
    pub fn new(frames: usize, states: usize, values: &[f32]) -> Result<Self, BindingError> {
        LogEmissionTable::from_log_values(frames, states, values.to_vec())
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
    pub fn probabilities(&self) -> EmissionDensitiesF32 {
        EmissionDensitiesF32 {
            inner: self.inner.probabilities(),
        }
    }
}
#[derive(Clone)]
pub struct EmissionDensitiesF32 {
    pub(crate) inner: EmissionDensityTable<f32>,
}
impl EmissionDensitiesF32 {
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
}
impl ModelF32 {
    pub fn emission_log_probabilities(
        &self,
        observation: &ObservationF32,
        segmentation: &SegmentationF32,
        temperature: f32,
        pruning: &EmissionPruningF32,
    ) -> Result<LogEmissionsF32, BindingError> {
        self.inner
            .emission_log_probabilities(
                &observation.inner,
                &segmentation.inner,
                temperature,
                pruning.native()?,
            )
            .map(|inner| LogEmissionsF32 { inner })
            .map_err(error)
    }
    pub fn emission_probabilities(
        &self,
        observation: &ObservationF32,
        segmentation: &SegmentationF32,
        temperature: f32,
        pruning: &EmissionPruningF32,
    ) -> Result<EmissionDensitiesF32, BindingError> {
        self.inner
            .emission_probabilities(
                &observation.inner,
                &segmentation.inner,
                temperature,
                pruning.native()?,
            )
            .map(|inner| EmissionDensitiesF32 { inner })
            .map_err(error)
    }
}
impl PreparedModelF32 {
    pub fn emission_log_probabilities(
        &self,
        observation: &ObservationF32,
        segmentation: &SegmentationF32,
        temperature: f32,
        pruning: &EmissionPruningF32,
    ) -> Result<LogEmissionsF32, BindingError> {
        self.inner
            .emission_log_probabilities(
                &observation.inner,
                &segmentation.inner,
                temperature,
                pruning.native()?,
            )
            .map(|inner| LogEmissionsF32 { inner })
            .map_err(error)
    }
    pub fn emission_probabilities(
        &self,
        observation: &ObservationF32,
        segmentation: &SegmentationF32,
        temperature: f32,
        pruning: &EmissionPruningF32,
    ) -> Result<EmissionDensitiesF32, BindingError> {
        self.inner
            .emission_probabilities(
                &observation.inner,
                &segmentation.inner,
                temperature,
                pruning.native()?,
            )
            .map(|inner| EmissionDensitiesF32 { inner })
            .map_err(error)
    }
}
/// Pruning mode: 0 evaluates all pairs, 1 uses a boundary radius,
/// and 2 uses a linear slope. The parameter retains selected precision.
#[derive(Clone)]
pub struct EmissionPruningF64 {
    pub mode: u32,
    pub parameter: f64,
}
impl EmissionPruningF64 {
    pub(crate) fn native(&self) -> Result<EmissionPruningOptions<f64>, BindingError> {
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
impl EmissionPruningF64 {
    pub fn new(mode: u32, parameter: f64) -> Self {
        Self { mode, parameter }
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
#[derive(Clone)]
pub struct LogEmissionsF64 {
    pub(crate) inner: LogEmissionTable<f64>,
}
impl LogEmissionsF64 {
    pub fn new(frames: usize, states: usize, values: &[f64]) -> Result<Self, BindingError> {
        LogEmissionTable::from_log_values(frames, states, values.to_vec())
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
    pub fn probabilities(&self) -> EmissionDensitiesF64 {
        EmissionDensitiesF64 {
            inner: self.inner.probabilities(),
        }
    }
}
#[derive(Clone)]
pub struct EmissionDensitiesF64 {
    pub(crate) inner: EmissionDensityTable<f64>,
}
impl EmissionDensitiesF64 {
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
}
impl ModelF64 {
    pub fn emission_log_probabilities(
        &self,
        observation: &ObservationF64,
        segmentation: &SegmentationF64,
        temperature: f64,
        pruning: &EmissionPruningF64,
    ) -> Result<LogEmissionsF64, BindingError> {
        self.inner
            .emission_log_probabilities(
                &observation.inner,
                &segmentation.inner,
                temperature,
                pruning.native()?,
            )
            .map(|inner| LogEmissionsF64 { inner })
            .map_err(error)
    }
    pub fn emission_probabilities(
        &self,
        observation: &ObservationF64,
        segmentation: &SegmentationF64,
        temperature: f64,
        pruning: &EmissionPruningF64,
    ) -> Result<EmissionDensitiesF64, BindingError> {
        self.inner
            .emission_probabilities(
                &observation.inner,
                &segmentation.inner,
                temperature,
                pruning.native()?,
            )
            .map(|inner| EmissionDensitiesF64 { inner })
            .map_err(error)
    }
}
impl PreparedModelF64 {
    pub fn emission_log_probabilities(
        &self,
        observation: &ObservationF64,
        segmentation: &SegmentationF64,
        temperature: f64,
        pruning: &EmissionPruningF64,
    ) -> Result<LogEmissionsF64, BindingError> {
        self.inner
            .emission_log_probabilities(
                &observation.inner,
                &segmentation.inner,
                temperature,
                pruning.native()?,
            )
            .map(|inner| LogEmissionsF64 { inner })
            .map_err(error)
    }
    pub fn emission_probabilities(
        &self,
        observation: &ObservationF64,
        segmentation: &SegmentationF64,
        temperature: f64,
        pruning: &EmissionPruningF64,
    ) -> Result<EmissionDensitiesF64, BindingError> {
        self.inner
            .emission_probabilities(
                &observation.inner,
                &segmentation.inner,
                temperature,
                pruning.native()?,
            )
            .map(|inner| EmissionDensitiesF64 { inner })
            .map_err(error)
    }
}
