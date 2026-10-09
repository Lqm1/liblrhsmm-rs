use super::*;
use crate::data::ObservationData;
use crate::generation::{choose_with_precision, gaussian_with_precision};
use wasm_bindgen::prelude::*;

fn error(value: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&value.to_string())
}

macro_rules! generation {
    ($scalar:ty, $model:ident, $sequence:ident, $observation:ident, $generated:ident, $gaussian:ident, $choose:ident) => {
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
        }

        #[wasm_bindgen]
        impl $generated {
            #[wasm_bindgen(getter)]
            pub fn draws_used(&self) -> usize {
                self.draws_used
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
    gaussian_sample_f32,
    choose_component_f32
);
generation!(
    f64,
    ModelF64,
    SegmentationF64,
    ObservationF64,
    GeneratedObservationF64,
    gaussian_sample_f64,
    choose_component_f64
);
