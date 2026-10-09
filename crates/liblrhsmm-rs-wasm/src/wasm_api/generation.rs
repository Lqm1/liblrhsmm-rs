use super::*;
use crate::data::{ObservationData, SegmentationData};
use crate::generation::{choose_with_precision, gaussian_with_precision, uniform_with_precision};
use crate::model::ModelParameters;
use js_sys::Function;
use rand::TryRng;
use std::{cell::RefCell, convert::Infallible};
use wasm_bindgen::prelude::*;

fn error(value: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&value.to_string())
}

struct WordRng<'a> {
    callback: &'a Function,
    failure: Option<JsValue>,
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
        let result = self.callback.call0(&JsValue::UNDEFINED).and_then(|value| {
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

macro_rules! generation {
    ($scalar:ty, $model:ident, $sequence:ident, $observation:ident, $generated:ident, $request:ident, $gaussian:ident, $choose:ident, $uniform:ident) => {
        /// Draw in (0, 1] from caller-owned u32 words, low word first for f64.
        #[wasm_bindgen]
        pub fn $uniform(callback: &Function) -> Result<$scalar, JsValue> {
            let mut rng = WordRng::new(callback);
            let value = uniform_with_precision(&mut rng);
            match rng.failure {
                Some(failure) => Err(failure),
                None => Ok(value),
            }
        }
        #[wasm_bindgen]
        pub fn $gaussian(
            mean: $scalar,
            variance: $scalar,
            radial: $scalar,
            angular: $scalar,
        ) -> Result<$scalar, JsValue> {
            gaussian_with_precision(mean, variance, radial, angular).map_err(error)
        }

        #[wasm_bindgen]
        pub fn $choose(weights: &[$scalar], uniform: $scalar) -> Result<usize, JsValue> {
            choose_with_precision(weights, uniform).map_err(error)
        }

        /// An independent observation snapshot and the number of consumed draws.
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $generated {
            observation: ObservationData<$scalar>,
            draws_used: usize,
            random_words_used: usize,
        }

        #[wasm_bindgen]
        impl $generated {
            #[wasm_bindgen(getter)]
            pub fn draws_used(&self) -> usize {
                self.draws_used
            }

            #[wasm_bindgen(getter)]
            pub fn random_words_used(&self) -> usize {
                self.random_words_used
            }

            pub fn observation(&self) -> $observation {
                $observation {
                    inner: self.observation.clone(),
                }
            }

            pub fn into_observation(self) -> $observation {
                $observation {
                    inner: self.observation,
                }
            }

            pub fn cloned(&self) -> Self {
                self.clone()
            }
        }

        /// Owned model and segmentation for calls into JavaScript.
        #[wasm_bindgen]
        pub struct $request {
            inner: RefCell<GenerationState<$scalar>>,
        }
        #[wasm_bindgen]
        impl $request {
            #[wasm_bindgen(constructor)]
            pub fn new(model: &$model, sequence: &$sequence) -> Self {
                Self {
                    inner: RefCell::new(GenerationState {
                        model: model.inner.clone(),
                        sequence: sequence.inner.clone(),
                    }),
                }
            }

            pub fn model(&self) -> Result<$model, JsValue> {
                let inner = self
                    .inner
                    .try_borrow()
                    .map_err(|_| error("generation is already in use"))?;
                Ok($model {
                    inner: inner.model.clone(),
                })
            }

            pub fn sequence(&self) -> Result<$sequence, JsValue> {
                let inner = self
                    .inner
                    .try_borrow()
                    .map_err(|_| error("generation is already in use"))?;
                Ok($sequence {
                    inner: inner.sequence.clone(),
                })
            }

            pub fn set_model(&self, model: &$model) -> Result<(), JsValue> {
                let mut inner = self
                    .inner
                    .try_borrow_mut()
                    .map_err(|_| error("generation is already in use"))?;
                inner.model = model.inner.clone();
                Ok(())
            }

            pub fn set_sequence(&self, sequence: &$sequence) -> Result<(), JsValue> {
                let mut inner = self
                    .inner
                    .try_borrow_mut()
                    .map_err(|_| error("generation is already in use"))?;
                inner.sequence = sequence.inner.clone();
                Ok(())
            }

            pub fn cloned(&self) -> Result<Self, JsValue> {
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
            ) -> Result<$generated, JsValue> {
                let mut inner = self
                    .inner
                    .try_borrow_mut()
                    .map_err(|_| error("generation is already in use"))?;
                let GenerationState { model, sequence } = &mut *inner;
                let mut failure = None;
                let mut draws_used = 0;
                let result = model.generate_with_uniform(sequence, || {
                    if failure.is_some() {
                        return <$scalar>::NAN;
                    }
                    draws_used += 1;
                    match callback.call0(&JsValue::UNDEFINED).and_then(|value| {
                        value
                            .as_f64()
                            .ok_or_else(|| error("uniform callback must return a number"))
                    }) {
                        Ok(value) => value as $scalar,
                        Err(value) => {
                            failure = Some(value);
                            <$scalar>::NAN
                        }
                    }
                });
                if let Some(failure) = failure {
                    return Err(failure);
                }
                result
                    .map(|observation| $generated {
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
            ) -> Result<$generated, JsValue> {
                let mut inner = self
                    .inner
                    .try_borrow_mut()
                    .map_err(|_| error("generation is already in use"))?;
                let GenerationState { model, sequence } = &mut *inner;
                let mut rng = WordRng::new(callback);
                let mut draws_used = 0;
                let result = model.generate_with_uniform(sequence, || {
                    if rng.failure.is_some() {
                        return <$scalar>::NAN;
                    }
                    draws_used += 1;
                    let value = uniform_with_precision(&mut rng);
                    if rng.failure.is_some() {
                        <$scalar>::NAN
                    } else {
                        value
                    }
                });
                if let Some(failure) = rng.failure {
                    return Err(failure);
                }
                result
                    .map(|observation| $generated {
                        observation,
                        draws_used,
                        random_words_used: rng.words_used,
                    })
                    .map_err(error)
            }
        }

        #[wasm_bindgen]
        impl $model {
            /// Use explicit uniform draws in native consumption order.
            /// Unused trailing values are ignored. An exhausted or invalid draw
            /// fails without changing the segmentation.
            pub fn generate_with_uniform_values(
                &self,
                sequence: &mut $sequence,
                values: &[$scalar],
            ) -> Result<$generated, JsValue> {
                let mut draws_used = 0;
                let mut exhausted = false;
                let result = self.inner.generate_with_uniform(&mut sequence.inner, || {
                    let Some(&value) = values.get(draws_used) else {
                        exhausted = true;
                        return <$scalar>::NAN;
                    };
                    draws_used += 1;
                    value
                });
                if exhausted {
                    return Err(error("uniform draw sequence exhausted"));
                }
                result
                    .map(|observation| $generated {
                        observation,
                        draws_used,
                        random_words_used: 0,
                    })
                    .map_err(error)
            }
        }
    };
}

generation!(
    f32,
    ModelF32,
    SegmentationF32,
    ObservationF32,
    GeneratedObservationF32,
    GenerationF32,
    gaussian_sample_f32,
    choose_component_f32,
    uniform_sample_f32
);
generation!(
    f64,
    ModelF64,
    SegmentationF64,
    ObservationF64,
    GeneratedObservationF64,
    GenerationF64,
    gaussian_sample_f64,
    choose_component_f64,
    uniform_sample_f64
);
