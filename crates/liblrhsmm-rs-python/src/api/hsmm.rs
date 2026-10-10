fn new_err_owned(message: &str) -> pyo3::PyErr {
    pyo3::exceptions::PyValueError::new_err(message.to_owned())
}
use super::*;
use crate::hsmm::{HsmmPath, HsmmScoreTables, HsmmSettings, StateOccurrence};
use pyo3::PyErr as BindingError;
fn error(value: impl std::fmt::Display) -> BindingError {
    new_err_owned(&value.to_string())
}
#[derive(Clone)]
pub struct HsmmOptionsF32 {
    pub temperature: f32,
    pub duration_weight: f32,
    pub state_radius: f32,
    pub duration_extra: usize,
    pub duration_extra_factor: f32,
}
impl Default for HsmmOptionsF32 {
    fn default() -> Self {
        Self::new()
    }
}
impl HsmmOptionsF32 {
    pub(crate) fn native(&self) -> HsmmSettings<f32> {
        HsmmSettings {
            temperature: self.temperature,
            duration_weight: self.duration_weight,
            state_radius: self.state_radius,
            duration_extra: self.duration_extra,
            duration_extra_factor: self.duration_extra_factor,
        }
    }
}
impl HsmmOptionsF32 {
    pub fn new() -> Self {
        let value = HsmmSettings::<f32>::default();
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
#[derive(Clone)]
pub struct HsmmScoresF32 {
    pub(crate) inner: HsmmScoreTables<f32>,
}
impl HsmmScoresF32 {
    pub fn new(begin: &StateScoresF32, end: &StateScoresF32) -> Self {
        Self {
            inner: HsmmScoreTables {
                begin: begin.inner.clone(),
                end: end.inner.clone(),
            },
        }
    }
    pub fn begin(&self) -> StateScoresF32 {
        StateScoresF32 {
            inner: self.inner.begin.clone(),
        }
    }
    pub fn end(&self) -> StateScoresF32 {
        StateScoresF32 {
            inner: self.inner.end.clone(),
        }
    }
    pub fn set_begin(&mut self, value: &StateScoresF32) {
        self.inner.begin = value.inner.clone();
    }
    pub fn set_end(&mut self, value: &StateScoresF32) {
        self.inner.end = value.inner.clone();
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
#[derive(Clone)]
pub struct HsmmPathF32 {
    pub(crate) inner: HsmmPath<f32>,
}
impl HsmmPathF32 {
    pub fn new(
        occurrences: &[u32],
        log_probability: &[f32],
        scores: &HsmmScoresF32,
    ) -> Result<Self, BindingError> {
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
    pub fn set_occurrences(&mut self, values: &[u32]) -> Result<(), BindingError> {
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
    pub fn scores(&self) -> HsmmScoresF32 {
        HsmmScoresF32 {
            inner: self.inner.scores.clone(),
        }
    }
    pub fn set_scores(&mut self, value: &HsmmScoresF32) {
        self.inner.scores = value.inner.clone();
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
impl ModelF32 {
    pub fn forward_hsmm(
        &self,
        segmentation: &SegmentationF32,
        emissions: &LogEmissionsF32,
        options: &HsmmOptionsF32,
    ) -> Result<HsmmScoresF32, BindingError> {
        self.inner
            .forward_hsmm(&segmentation.inner, &emissions.inner, options.native())
            .map(|inner| HsmmScoresF32 { inner })
            .map_err(error)
    }
    pub fn backward_hsmm(
        &self,
        segmentation: &SegmentationF32,
        emissions: &LogEmissionsF32,
        options: &HsmmOptionsF32,
    ) -> Result<HsmmScoresF32, BindingError> {
        self.inner
            .backward_hsmm(&segmentation.inner, &emissions.inner, options.native())
            .map(|inner| HsmmScoresF32 { inner })
            .map_err(error)
    }
    pub fn viterbi_hsmm(
        &self,
        segmentation: &SegmentationF32,
        emissions: &LogEmissionsF32,
        options: &HsmmOptionsF32,
    ) -> Result<HsmmPathF32, BindingError> {
        self.inner
            .viterbi_hsmm(&segmentation.inner, &emissions.inner, options.native())
            .map(|inner| HsmmPathF32 { inner })
            .map_err(error)
    }
    pub fn forward_hsmm_with_graph(
        &self,
        graph: &TransitionGraphF32,
        emissions: &LogEmissionsF32,
        options: &HsmmOptionsF32,
    ) -> Result<HsmmScoresF32, BindingError> {
        self.inner
            .forward_hsmm_with_graph(graph.inner.as_view(), &emissions.inner, options.native())
            .map(|inner| HsmmScoresF32 { inner })
            .map_err(error)
    }
    pub fn backward_hsmm_with_graph(
        &self,
        graph: &TransitionGraphF32,
        emissions: &LogEmissionsF32,
        options: &HsmmOptionsF32,
    ) -> Result<HsmmScoresF32, BindingError> {
        self.inner
            .backward_hsmm_with_graph(graph.inner.as_view(), &emissions.inner, options.native())
            .map(|inner| HsmmScoresF32 { inner })
            .map_err(error)
    }
    pub fn viterbi_hsmm_with_graph(
        &self,
        graph: &TransitionGraphF32,
        emissions: &LogEmissionsF32,
        options: &HsmmOptionsF32,
    ) -> Result<HsmmPathF32, BindingError> {
        self.inner
            .viterbi_hsmm_with_graph(graph.inner.as_view(), &emissions.inner, options.native())
            .map(|inner| HsmmPathF32 { inner })
            .map_err(error)
    }
}
impl PreparedModelF32 {
    pub fn forward_hsmm(
        &self,
        segmentation: &SegmentationF32,
        emissions: &LogEmissionsF32,
        options: &HsmmOptionsF32,
    ) -> Result<HsmmScoresF32, BindingError> {
        self.inner
            .forward_hsmm(&segmentation.inner, &emissions.inner, options.native())
            .map(|inner| HsmmScoresF32 { inner })
            .map_err(error)
    }
    pub fn backward_hsmm(
        &self,
        segmentation: &SegmentationF32,
        emissions: &LogEmissionsF32,
        options: &HsmmOptionsF32,
    ) -> Result<HsmmScoresF32, BindingError> {
        self.inner
            .backward_hsmm(&segmentation.inner, &emissions.inner, options.native())
            .map(|inner| HsmmScoresF32 { inner })
            .map_err(error)
    }
    pub fn viterbi_hsmm(
        &self,
        segmentation: &SegmentationF32,
        emissions: &LogEmissionsF32,
        options: &HsmmOptionsF32,
    ) -> Result<HsmmPathF32, BindingError> {
        self.inner
            .viterbi_hsmm(&segmentation.inner, &emissions.inner, options.native())
            .map(|inner| HsmmPathF32 { inner })
            .map_err(error)
    }
    pub fn forward_hsmm_with_graph(
        &self,
        graph: &TransitionGraphF32,
        emissions: &LogEmissionsF32,
        options: &HsmmOptionsF32,
    ) -> Result<HsmmScoresF32, BindingError> {
        self.inner
            .forward_hsmm_with_graph(graph.inner.as_view(), &emissions.inner, options.native())
            .map(|inner| HsmmScoresF32 { inner })
            .map_err(error)
    }
    pub fn backward_hsmm_with_graph(
        &self,
        graph: &TransitionGraphF32,
        emissions: &LogEmissionsF32,
        options: &HsmmOptionsF32,
    ) -> Result<HsmmScoresF32, BindingError> {
        self.inner
            .backward_hsmm_with_graph(graph.inner.as_view(), &emissions.inner, options.native())
            .map(|inner| HsmmScoresF32 { inner })
            .map_err(error)
    }
    pub fn viterbi_hsmm_with_graph(
        &self,
        graph: &TransitionGraphF32,
        emissions: &LogEmissionsF32,
        options: &HsmmOptionsF32,
    ) -> Result<HsmmPathF32, BindingError> {
        self.inner
            .viterbi_hsmm_with_graph(graph.inner.as_view(), &emissions.inner, options.native())
            .map(|inner| HsmmPathF32 { inner })
            .map_err(error)
    }
}
#[derive(Clone)]
pub struct HsmmOptionsF64 {
    pub temperature: f64,
    pub duration_weight: f64,
    pub state_radius: f64,
    pub duration_extra: usize,
    pub duration_extra_factor: f64,
}
impl Default for HsmmOptionsF64 {
    fn default() -> Self {
        Self::new()
    }
}
impl HsmmOptionsF64 {
    pub(crate) fn native(&self) -> HsmmSettings<f64> {
        HsmmSettings {
            temperature: self.temperature,
            duration_weight: self.duration_weight,
            state_radius: self.state_radius,
            duration_extra: self.duration_extra,
            duration_extra_factor: self.duration_extra_factor,
        }
    }
}
impl HsmmOptionsF64 {
    pub fn new() -> Self {
        let value = HsmmSettings::<f64>::default();
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
#[derive(Clone)]
pub struct HsmmScoresF64 {
    pub(crate) inner: HsmmScoreTables<f64>,
}
impl HsmmScoresF64 {
    pub fn new(begin: &StateScoresF64, end: &StateScoresF64) -> Self {
        Self {
            inner: HsmmScoreTables {
                begin: begin.inner.clone(),
                end: end.inner.clone(),
            },
        }
    }
    pub fn begin(&self) -> StateScoresF64 {
        StateScoresF64 {
            inner: self.inner.begin.clone(),
        }
    }
    pub fn end(&self) -> StateScoresF64 {
        StateScoresF64 {
            inner: self.inner.end.clone(),
        }
    }
    pub fn set_begin(&mut self, value: &StateScoresF64) {
        self.inner.begin = value.inner.clone();
    }
    pub fn set_end(&mut self, value: &StateScoresF64) {
        self.inner.end = value.inner.clone();
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
#[derive(Clone)]
pub struct HsmmPathF64 {
    pub(crate) inner: HsmmPath<f64>,
}
impl HsmmPathF64 {
    pub fn new(
        occurrences: &[u32],
        log_probability: &[f64],
        scores: &HsmmScoresF64,
    ) -> Result<Self, BindingError> {
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
    pub fn set_occurrences(&mut self, values: &[u32]) -> Result<(), BindingError> {
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
    pub fn scores(&self) -> HsmmScoresF64 {
        HsmmScoresF64 {
            inner: self.inner.scores.clone(),
        }
    }
    pub fn set_scores(&mut self, value: &HsmmScoresF64) {
        self.inner.scores = value.inner.clone();
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
impl ModelF64 {
    pub fn forward_hsmm(
        &self,
        segmentation: &SegmentationF64,
        emissions: &LogEmissionsF64,
        options: &HsmmOptionsF64,
    ) -> Result<HsmmScoresF64, BindingError> {
        self.inner
            .forward_hsmm(&segmentation.inner, &emissions.inner, options.native())
            .map(|inner| HsmmScoresF64 { inner })
            .map_err(error)
    }
    pub fn backward_hsmm(
        &self,
        segmentation: &SegmentationF64,
        emissions: &LogEmissionsF64,
        options: &HsmmOptionsF64,
    ) -> Result<HsmmScoresF64, BindingError> {
        self.inner
            .backward_hsmm(&segmentation.inner, &emissions.inner, options.native())
            .map(|inner| HsmmScoresF64 { inner })
            .map_err(error)
    }
    pub fn viterbi_hsmm(
        &self,
        segmentation: &SegmentationF64,
        emissions: &LogEmissionsF64,
        options: &HsmmOptionsF64,
    ) -> Result<HsmmPathF64, BindingError> {
        self.inner
            .viterbi_hsmm(&segmentation.inner, &emissions.inner, options.native())
            .map(|inner| HsmmPathF64 { inner })
            .map_err(error)
    }
    pub fn forward_hsmm_with_graph(
        &self,
        graph: &TransitionGraphF64,
        emissions: &LogEmissionsF64,
        options: &HsmmOptionsF64,
    ) -> Result<HsmmScoresF64, BindingError> {
        self.inner
            .forward_hsmm_with_graph(graph.inner.as_view(), &emissions.inner, options.native())
            .map(|inner| HsmmScoresF64 { inner })
            .map_err(error)
    }
    pub fn backward_hsmm_with_graph(
        &self,
        graph: &TransitionGraphF64,
        emissions: &LogEmissionsF64,
        options: &HsmmOptionsF64,
    ) -> Result<HsmmScoresF64, BindingError> {
        self.inner
            .backward_hsmm_with_graph(graph.inner.as_view(), &emissions.inner, options.native())
            .map(|inner| HsmmScoresF64 { inner })
            .map_err(error)
    }
    pub fn viterbi_hsmm_with_graph(
        &self,
        graph: &TransitionGraphF64,
        emissions: &LogEmissionsF64,
        options: &HsmmOptionsF64,
    ) -> Result<HsmmPathF64, BindingError> {
        self.inner
            .viterbi_hsmm_with_graph(graph.inner.as_view(), &emissions.inner, options.native())
            .map(|inner| HsmmPathF64 { inner })
            .map_err(error)
    }
}
impl PreparedModelF64 {
    pub fn forward_hsmm(
        &self,
        segmentation: &SegmentationF64,
        emissions: &LogEmissionsF64,
        options: &HsmmOptionsF64,
    ) -> Result<HsmmScoresF64, BindingError> {
        self.inner
            .forward_hsmm(&segmentation.inner, &emissions.inner, options.native())
            .map(|inner| HsmmScoresF64 { inner })
            .map_err(error)
    }
    pub fn backward_hsmm(
        &self,
        segmentation: &SegmentationF64,
        emissions: &LogEmissionsF64,
        options: &HsmmOptionsF64,
    ) -> Result<HsmmScoresF64, BindingError> {
        self.inner
            .backward_hsmm(&segmentation.inner, &emissions.inner, options.native())
            .map(|inner| HsmmScoresF64 { inner })
            .map_err(error)
    }
    pub fn viterbi_hsmm(
        &self,
        segmentation: &SegmentationF64,
        emissions: &LogEmissionsF64,
        options: &HsmmOptionsF64,
    ) -> Result<HsmmPathF64, BindingError> {
        self.inner
            .viterbi_hsmm(&segmentation.inner, &emissions.inner, options.native())
            .map(|inner| HsmmPathF64 { inner })
            .map_err(error)
    }
    pub fn forward_hsmm_with_graph(
        &self,
        graph: &TransitionGraphF64,
        emissions: &LogEmissionsF64,
        options: &HsmmOptionsF64,
    ) -> Result<HsmmScoresF64, BindingError> {
        self.inner
            .forward_hsmm_with_graph(graph.inner.as_view(), &emissions.inner, options.native())
            .map(|inner| HsmmScoresF64 { inner })
            .map_err(error)
    }
    pub fn backward_hsmm_with_graph(
        &self,
        graph: &TransitionGraphF64,
        emissions: &LogEmissionsF64,
        options: &HsmmOptionsF64,
    ) -> Result<HsmmScoresF64, BindingError> {
        self.inner
            .backward_hsmm_with_graph(graph.inner.as_view(), &emissions.inner, options.native())
            .map(|inner| HsmmScoresF64 { inner })
            .map_err(error)
    }
    pub fn viterbi_hsmm_with_graph(
        &self,
        graph: &TransitionGraphF64,
        emissions: &LogEmissionsF64,
        options: &HsmmOptionsF64,
    ) -> Result<HsmmPathF64, BindingError> {
        self.inner
            .viterbi_hsmm_with_graph(graph.inner.as_view(), &emissions.inner, options.native())
            .map(|inner| HsmmPathF64 { inner })
            .map_err(error)
    }
}
