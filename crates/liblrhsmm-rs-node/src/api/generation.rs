use super::*;
use crate::callbacks::Function;
use crate::data::{ObservationData, SegmentationData};
use crate::generation::{choose_with_precision, gaussian_with_precision, uniform_with_precision};
use crate::model::ModelParameters;
use napi::Error as BindingError;
use rand::TryRng;
use std::{cell::RefCell, convert::Infallible};
fn error(value: impl std::fmt::Display) -> BindingError {
    napi::Error::from_reason(value.to_string())
}
struct WordRng<'a> {
    callback: &'a Function,
    failure: Option<BindingError>,
    words_used: usize,
}
impl<'a> WordRng<'a> {
    fn new(callback: &'a Function) -> Self {
        Self {
            callback,
            failure: None,
            words_used: 0,
        }
    }
    fn word(&mut self) -> u32 {
        if self.failure.is_some() {
            return u32::MAX;
        }
        self.words_used += 1;
        let result = self.callback.call0(&()).and_then(|value| {
            value
                .as_f64()
                .filter(|value| {
                    value.is_finite()
                        && value.fract() == 0.0
                        && (0.0..=u32::MAX as f64).contains(value)
                })
                .map(|value| value as u32)
                .ok_or_else(|| error("random word callback must return a u32 integer"))
        });
        match result {
            Ok(value) => value,
            Err(value) => {
                self.failure = Some(value);
                u32::MAX
            }
        }
    }
}
impl TryRng for WordRng<'_> {
    type Error = Infallible;
    fn try_next_u32(&mut self) -> Result<u32, Infallible> {
        Ok(self.word())
    }
    fn try_next_u64(&mut self) -> Result<u64, Infallible> {
        Ok(u64::from(self.word()) | (u64::from(self.word()) << 32))
    }
    fn try_fill_bytes(&mut self, bytes: &mut [u8]) -> Result<(), Infallible> {
        for chunk in bytes.chunks_mut(4) {
            chunk.copy_from_slice(&self.word().to_le_bytes()[..chunk.len()]);
        }
        Ok(())
    }
}
#[derive(Clone)]
struct GenerationState<T> {
    model: ModelParameters<T>,
    sequence: SegmentationData<T>,
}
/// Draw in (0, 1] from caller-owned u32 words, low word first for f64.
pub fn uniform_sample_f32(callback: &Function) -> Result<f32, BindingError> {
    let mut rng = WordRng::new(callback);
    let value = uniform_with_precision(&mut rng);
    match rng.failure {
        Some(failure) => Err(failure),
        None => Ok(value),
    }
}
pub fn gaussian_sample_f32(
    mean: f32,
    variance: f32,
    radial: f32,
    angular: f32,
) -> Result<f32, BindingError> {
    gaussian_with_precision(mean, variance, radial, angular).map_err(error)
}
pub fn choose_component_f32(weights: &[f32], uniform: f32) -> Result<usize, BindingError> {
    choose_with_precision(weights, uniform).map_err(error)
}
/// An independent observation snapshot and the number of consumed draws.
#[derive(Clone)]
pub struct GeneratedObservationF32 {
    observation: ObservationData<f32>,
    draws_used: usize,
    random_words_used: usize,
}
impl GeneratedObservationF32 {
    pub fn draws_used(&self) -> usize {
        self.draws_used
    }
    pub fn random_words_used(&self) -> usize {
        self.random_words_used
    }
    pub fn observation(&self) -> ObservationF32 {
        ObservationF32 {
            inner: self.observation.clone(),
        }
    }
    pub fn into_observation(self) -> ObservationF32 {
        ObservationF32 {
            inner: self.observation,
        }
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
/// Owned model and segmentation for calls into JavaScript.
pub struct GenerationF32 {
    inner: RefCell<GenerationState<f32>>,
}
impl GenerationF32 {
    pub fn new(model: &ModelF32, sequence: &SegmentationF32) -> Self {
        Self {
            inner: RefCell::new(GenerationState {
                model: model.inner.clone(),
                sequence: sequence.inner.clone(),
            }),
        }
    }
    pub fn model(&self) -> Result<ModelF32, BindingError> {
        let inner = self
            .inner
            .try_borrow()
            .map_err(|_| error("generation is already in use"))?;
        Ok(ModelF32 {
            inner: inner.model.clone(),
        })
    }
    pub fn sequence(&self) -> Result<SegmentationF32, BindingError> {
        let inner = self
            .inner
            .try_borrow()
            .map_err(|_| error("generation is already in use"))?;
        Ok(SegmentationF32 {
            inner: inner.sequence.clone(),
        })
    }
    pub fn set_model(&self, model: &ModelF32) -> Result<(), BindingError> {
        let mut inner = self
            .inner
            .try_borrow_mut()
            .map_err(|_| error("generation is already in use"))?;
        inner.model = model.inner.clone();
        Ok(())
    }
    pub fn set_sequence(&self, sequence: &SegmentationF32) -> Result<(), BindingError> {
        let mut inner = self
            .inner
            .try_borrow_mut()
            .map_err(|_| error("generation is already in use"))?;
        inner.sequence = sequence.inner.clone();
        Ok(())
    }
    pub fn cloned(&self) -> Result<Self, BindingError> {
        let inner = self
            .inner
            .try_borrow()
            .map_err(|_| error("generation is already in use"))?;
        Ok(Self {
            inner: RefCell::new(inner.clone()),
        })
    }
    /// A throwing or invalid callback leaves the owned sequence unchanged.
    pub fn generate_with_uniform(
        &self,
        callback: &Function,
    ) -> Result<GeneratedObservationF32, BindingError> {
        let mut inner = self
            .inner
            .try_borrow_mut()
            .map_err(|_| error("generation is already in use"))?;
        let GenerationState { model, sequence } = &mut *inner;
        let mut failure = None;
        let mut draws_used = 0;
        let result = model.generate_with_uniform(sequence, || {
            if failure.is_some() {
                return <f32>::NAN;
            }
            draws_used += 1;
            match callback.call0(&()).and_then(|value| {
                value
                    .as_f64()
                    .ok_or_else(|| error("uniform callback must return a number"))
            }) {
                Ok(value) => value as f32,
                Err(value) => {
                    failure = Some(value);
                    <f32>::NAN
                }
            }
        });
        if let Some(failure) = failure {
            return Err(failure);
        }
        result
            .map(|observation| GeneratedObservationF32 {
                observation,
                draws_used,
                random_words_used: 0,
            })
            .map_err(error)
    }
    /// Draw native OpenClosed01 values from arbitrary caller-owned words.
    pub fn generate_with_random_words(
        &self,
        callback: &Function,
    ) -> Result<GeneratedObservationF32, BindingError> {
        let mut inner = self
            .inner
            .try_borrow_mut()
            .map_err(|_| error("generation is already in use"))?;
        let GenerationState { model, sequence } = &mut *inner;
        let mut rng = WordRng::new(callback);
        let mut draws_used = 0;
        let result = model.generate_with_uniform(sequence, || {
            if rng.failure.is_some() {
                return <f32>::NAN;
            }
            draws_used += 1;
            let value = uniform_with_precision(&mut rng);
            if rng.failure.is_some() {
                <f32>::NAN
            } else {
                value
            }
        });
        if let Some(failure) = rng.failure {
            return Err(failure);
        }
        result
            .map(|observation| GeneratedObservationF32 {
                observation,
                draws_used,
                random_words_used: rng.words_used,
            })
            .map_err(error)
    }
}
impl ModelF32 {
    /// Use explicit uniform draws in native consumption order.
    /// Unused trailing values are ignored. An exhausted or invalid draw
    /// fails without changing the segmentation.
    pub fn generate_with_uniform_values(
        &self,
        sequence: &mut SegmentationF32,
        values: &[f32],
    ) -> Result<GeneratedObservationF32, BindingError> {
        let mut draws_used = 0;
        let mut exhausted = false;
        let result = self.inner.generate_with_uniform(&mut sequence.inner, || {
            let Some(&value) = values.get(draws_used) else {
                exhausted = true;
                return <f32>::NAN;
            };
            draws_used += 1;
            value
        });
        if exhausted {
            return Err(error("uniform draw sequence exhausted"));
        }
        result
            .map(|observation| GeneratedObservationF32 {
                observation,
                draws_used,
                random_words_used: 0,
            })
            .map_err(error)
    }
}
/// Draw in (0, 1] from caller-owned u32 words, low word first for f64.
pub fn uniform_sample_f64(callback: &Function) -> Result<f64, BindingError> {
    let mut rng = WordRng::new(callback);
    let value = uniform_with_precision(&mut rng);
    match rng.failure {
        Some(failure) => Err(failure),
        None => Ok(value),
    }
}
pub fn gaussian_sample_f64(
    mean: f64,
    variance: f64,
    radial: f64,
    angular: f64,
) -> Result<f64, BindingError> {
    gaussian_with_precision(mean, variance, radial, angular).map_err(error)
}
pub fn choose_component_f64(weights: &[f64], uniform: f64) -> Result<usize, BindingError> {
    choose_with_precision(weights, uniform).map_err(error)
}
/// An independent observation snapshot and the number of consumed draws.
#[derive(Clone)]
pub struct GeneratedObservationF64 {
    observation: ObservationData<f64>,
    draws_used: usize,
    random_words_used: usize,
}
impl GeneratedObservationF64 {
    pub fn draws_used(&self) -> usize {
        self.draws_used
    }
    pub fn random_words_used(&self) -> usize {
        self.random_words_used
    }
    pub fn observation(&self) -> ObservationF64 {
        ObservationF64 {
            inner: self.observation.clone(),
        }
    }
    pub fn into_observation(self) -> ObservationF64 {
        ObservationF64 {
            inner: self.observation,
        }
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
/// Owned model and segmentation for calls into JavaScript.
pub struct GenerationF64 {
    inner: RefCell<GenerationState<f64>>,
}
impl GenerationF64 {
    pub fn new(model: &ModelF64, sequence: &SegmentationF64) -> Self {
        Self {
            inner: RefCell::new(GenerationState {
                model: model.inner.clone(),
                sequence: sequence.inner.clone(),
            }),
        }
    }
    pub fn model(&self) -> Result<ModelF64, BindingError> {
        let inner = self
            .inner
            .try_borrow()
            .map_err(|_| error("generation is already in use"))?;
        Ok(ModelF64 {
            inner: inner.model.clone(),
        })
    }
    pub fn sequence(&self) -> Result<SegmentationF64, BindingError> {
        let inner = self
            .inner
            .try_borrow()
            .map_err(|_| error("generation is already in use"))?;
        Ok(SegmentationF64 {
            inner: inner.sequence.clone(),
        })
    }
    pub fn set_model(&self, model: &ModelF64) -> Result<(), BindingError> {
        let mut inner = self
            .inner
            .try_borrow_mut()
            .map_err(|_| error("generation is already in use"))?;
        inner.model = model.inner.clone();
        Ok(())
    }
    pub fn set_sequence(&self, sequence: &SegmentationF64) -> Result<(), BindingError> {
        let mut inner = self
            .inner
            .try_borrow_mut()
            .map_err(|_| error("generation is already in use"))?;
        inner.sequence = sequence.inner.clone();
        Ok(())
    }
    pub fn cloned(&self) -> Result<Self, BindingError> {
        let inner = self
            .inner
            .try_borrow()
            .map_err(|_| error("generation is already in use"))?;
        Ok(Self {
            inner: RefCell::new(inner.clone()),
        })
    }
    /// A throwing or invalid callback leaves the owned sequence unchanged.
    pub fn generate_with_uniform(
        &self,
        callback: &Function,
    ) -> Result<GeneratedObservationF64, BindingError> {
        let mut inner = self
            .inner
            .try_borrow_mut()
            .map_err(|_| error("generation is already in use"))?;
        let GenerationState { model, sequence } = &mut *inner;
        let mut failure = None;
        let mut draws_used = 0;
        let result = model.generate_with_uniform(sequence, || {
            if failure.is_some() {
                return <f64>::NAN;
            }
            draws_used += 1;
            match callback.call0(&()).and_then(|value| {
                value
                    .as_f64()
                    .ok_or_else(|| error("uniform callback must return a number"))
            }) {
                Ok(value) => value,
                Err(value) => {
                    failure = Some(value);
                    <f64>::NAN
                }
            }
        });
        if let Some(failure) = failure {
            return Err(failure);
        }
        result
            .map(|observation| GeneratedObservationF64 {
                observation,
                draws_used,
                random_words_used: 0,
            })
            .map_err(error)
    }
    /// Draw native OpenClosed01 values from arbitrary caller-owned words.
    pub fn generate_with_random_words(
        &self,
        callback: &Function,
    ) -> Result<GeneratedObservationF64, BindingError> {
        let mut inner = self
            .inner
            .try_borrow_mut()
            .map_err(|_| error("generation is already in use"))?;
        let GenerationState { model, sequence } = &mut *inner;
        let mut rng = WordRng::new(callback);
        let mut draws_used = 0;
        let result = model.generate_with_uniform(sequence, || {
            if rng.failure.is_some() {
                return <f64>::NAN;
            }
            draws_used += 1;
            let value = uniform_with_precision(&mut rng);
            if rng.failure.is_some() {
                <f64>::NAN
            } else {
                value
            }
        });
        if let Some(failure) = rng.failure {
            return Err(failure);
        }
        result
            .map(|observation| GeneratedObservationF64 {
                observation,
                draws_used,
                random_words_used: rng.words_used,
            })
            .map_err(error)
    }
}
impl ModelF64 {
    /// Use explicit uniform draws in native consumption order.
    /// Unused trailing values are ignored. An exhausted or invalid draw
    /// fails without changing the segmentation.
    pub fn generate_with_uniform_values(
        &self,
        sequence: &mut SegmentationF64,
        values: &[f64],
    ) -> Result<GeneratedObservationF64, BindingError> {
        let mut draws_used = 0;
        let mut exhausted = false;
        let result = self.inner.generate_with_uniform(&mut sequence.inner, || {
            let Some(&value) = values.get(draws_used) else {
                exhausted = true;
                return <f64>::NAN;
            };
            draws_used += 1;
            value
        });
        if exhausted {
            return Err(error("uniform draw sequence exhausted"));
        }
        result
            .map(|observation| GeneratedObservationF64 {
                observation,
                draws_used,
                random_words_used: 0,
            })
            .map_err(error)
    }
}
