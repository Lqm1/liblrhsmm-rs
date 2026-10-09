use crate::model::DurationParameters;
use crate::preparation::{OwnedPreparedDurationParameters, PreparationSettings};
use crate::statistics::DurationSufficientStatistics;
use crate::update::{DurationMode, UpdateSettings};
use wasm_bindgen::prelude::*;

fn error(value: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&value.to_string())
}

macro_rules! duration_api {
    ($scalar:ty, $duration:ident, $prepared:ident, $preparation:ident, $statistics:ident, $update:ident, $decoded:ident) => {
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $preparation {
            pub duration_cache_length: usize,
            pub duration_weight: $scalar,
            pub temperature: $scalar,
        }
        impl $preparation {
            fn native(&self) -> PreparationSettings<$scalar> {
                PreparationSettings {
                    duration_cache_length: self.duration_cache_length,
                    duration_weight: self.duration_weight,
                    temperature: self.temperature,
                }
            }
        }
        #[wasm_bindgen]
        impl $preparation {
            #[wasm_bindgen(constructor)]
            pub fn new() -> Self {
                let value = PreparationSettings::<$scalar>::default();
                Self {
                    duration_cache_length: value.duration_cache_length,
                    duration_weight: value.duration_weight,
                    temperature: value.temperature,
                }
            }
            pub fn cloned(&self) -> Self {
                self.clone()
            }
        }
        impl Default for $preparation {
            fn default() -> Self {
                Self::new()
            }
        }

        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $update {
            pub duration_mode: u32,
            pub duration_variance_floor: $scalar,
            pub emission_variance_floor: $scalar,
        }
        impl $update {
            fn native(&self) -> Result<UpdateSettings<$scalar>, JsValue> {
                let duration_mode = match self.duration_mode {
                    0 => DurationMode::Normal,
                    1 => DurationMode::Geometric,
                    _ => return Err(error("invalid duration update mode")),
                };
                Ok(UpdateSettings {
                    duration_mode,
                    duration_variance_floor: self.duration_variance_floor,
                    emission_variance_floor: self.emission_variance_floor,
                })
            }
        }
        #[wasm_bindgen]
        impl $update {
            #[wasm_bindgen(constructor)]
            pub fn new() -> Self {
                let value = UpdateSettings::<$scalar>::default();
                Self {
                    duration_mode: 0,
                    duration_variance_floor: value.duration_variance_floor,
                    emission_variance_floor: value.emission_variance_floor,
                }
            }
            pub fn cloned(&self) -> Self {
                self.clone()
            }
        }
        impl Default for $update {
            fn default() -> Self {
                Self::new()
            }
        }

        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $statistics {
            pub value_sum: $scalar,
            pub squared_sum: $scalar,
            pub occupancy: $scalar,
        }
        impl $statistics {
            fn native(&self) -> DurationSufficientStatistics<$scalar> {
                DurationSufficientStatistics {
                    value_sum: self.value_sum,
                    squared_sum: self.squared_sum,
                    occupancy: self.occupancy,
                }
            }
        }
        #[wasm_bindgen]
        impl $statistics {
            #[wasm_bindgen(constructor)]
            pub fn new() -> Self {
                Self {
                    value_sum: 0.0,
                    squared_sum: 0.0,
                    occupancy: 0.0,
                }
            }
            pub fn cloned(&self) -> Self {
                self.clone()
            }
            pub fn clear(&mut self) {
                *self = Self::new();
            }
            pub fn values(&self) -> Vec<$scalar> {
                vec![self.value_sum, self.squared_sum, self.occupancy]
            }
            pub fn set_values(&mut self, values: &[$scalar]) -> Result<(), JsValue> {
                let [value_sum, squared_sum, occupancy] = values else {
                    return Err(error("duration statistics require three values"));
                };
                self.value_sum = *value_sum;
                self.squared_sum = *squared_sum;
                self.occupancy = *occupancy;
                Ok(())
            }
        }
        impl Default for $statistics {
            fn default() -> Self {
                Self::new()
            }
        }

        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $duration {
            pub(crate) inner: DurationParameters<$scalar>,
        }
        #[wasm_bindgen]
        impl $duration {
            #[wasm_bindgen(constructor)]
            pub fn new() -> Self {
                Self {
                    inner: DurationParameters::default(),
                }
            }
            pub fn cloned(&self) -> Self {
                self.clone()
            }
            pub fn values(&self) -> Vec<$scalar> {
                vec![
                    self.inner.mean,
                    self.inner.variance,
                    self.inner.variance_floor,
                ]
            }
            pub fn constraints(&self) -> Vec<i32> {
                vec![
                    self.inner.minimum,
                    self.inner.maximum,
                    self.inner.fixed_mean,
                ]
            }
            pub fn set_values(&mut self, values: &[$scalar]) -> Result<(), JsValue> {
                let [mean, variance, variance_floor] = values else {
                    return Err(error("duration requires three scalar values"));
                };
                self.inner.mean = *mean;
                self.inner.variance = *variance;
                self.inner.variance_floor = *variance_floor;
                Ok(())
            }
            pub fn set_constraints(&mut self, values: &[i32]) -> Result<(), JsValue> {
                let [minimum, maximum, fixed_mean] = values else {
                    return Err(error("duration requires three constraints"));
                };
                self.inner.minimum = *minimum;
                self.inner.maximum = *maximum;
                self.inner.fixed_mean = *fixed_mean;
                Ok(())
            }
            pub fn log_probability(
                &self,
                frames: $scalar,
                weight: $scalar,
            ) -> Result<$scalar, JsValue> {
                self.inner.log_probability(frames, weight).map_err(error)
            }
            pub fn annealed_log_probability(
                &self,
                frames: usize,
                weight: $scalar,
                temperature: $scalar,
            ) -> Result<$scalar, JsValue> {
                self.inner
                    .annealed_log_probability(frames, weight, temperature)
                    .map_err(error)
            }
            pub fn probability(
                &self,
                frames: usize,
                weight: $scalar,
                temperature: $scalar,
            ) -> Result<f64, JsValue> {
                self.inner
                    .probability(frames, weight, temperature)
                    .map_err(error)
            }
            pub fn prepare(&self, options: &$preparation) -> Result<$prepared, JsValue> {
                self.cloned().prepare_owned(options)
            }
            pub fn prepare_owned(self, options: &$preparation) -> Result<$prepared, JsValue> {
                self.inner
                    .prepare_owned(options.native())
                    .map(|inner| $prepared { inner })
                    .map_err(error)
            }
            pub fn update(
                &mut self,
                statistics: &$statistics,
                options: &$update,
            ) -> Result<bool, JsValue> {
                self.inner
                    .update(&statistics.native(), options.native()?)
                    .map_err(error)
            }
            #[cfg(feature = "serialization")]
            pub fn read(bytes: &[u8]) -> Result<Self, JsValue> {
                DurationParameters::read_from(bytes)
                    .map(|inner| Self { inner })
                    .map_err(error)
            }
            #[cfg(feature = "serialization")]
            pub fn read_with_limits(
                bytes: &[u8],
                max_array_entries: usize,
            ) -> Result<Self, JsValue> {
                DurationParameters::read_with_limits(
                    bytes,
                    crate::serial::DecodeLimits { max_array_entries },
                )
                .map(|inner| Self { inner })
                .map_err(error)
            }
            #[cfg(feature = "serialization")]
            pub fn read_prefix(bytes: &[u8]) -> Result<$decoded, JsValue> {
                Self::read_prefix_with_limits(
                    bytes,
                    crate::serial::DecodeLimits::default().max_array_entries,
                )
            }
            #[cfg(feature = "serialization")]
            pub fn read_prefix_with_limits(
                bytes: &[u8],
                max_array_entries: usize,
            ) -> Result<$decoded, JsValue> {
                let mut reader = std::io::Cursor::new(bytes);
                let inner = DurationParameters::read_prefix_with_limits(
                    &mut reader,
                    crate::serial::DecodeLimits { max_array_entries },
                )
                .map_err(error)?;
                Ok($decoded {
                    inner,
                    position: reader.position(),
                })
            }
            #[cfg(feature = "serialization")]
            pub fn write(&self) -> Result<Vec<u8>, JsValue> {
                let mut bytes = Vec::new();
                self.inner.write_to(&mut bytes).map_err(error)?;
                Ok(bytes)
            }
        }
        impl Default for $duration {
            fn default() -> Self {
                Self::new()
            }
        }

        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $prepared {
            pub(crate) inner: OwnedPreparedDurationParameters<$scalar>,
        }
        #[wasm_bindgen]
        impl $prepared {
            pub fn cloned(&self) -> Self {
                self.clone()
            }
            pub fn parameters(&self) -> $duration {
                $duration {
                    inner: self.inner.parameters().clone(),
                }
            }
            pub fn into_parameters(self) -> $duration {
                $duration {
                    inner: self.inner.into_parameters(),
                }
            }
            pub fn options(&self) -> $preparation {
                let value = self.inner.options();
                $preparation {
                    duration_cache_length: value.duration_cache_length,
                    duration_weight: value.duration_weight,
                    temperature: value.temperature,
                }
            }
            pub fn log_normalizer(&self) -> $scalar {
                self.inner.log_normalizer()
            }
            pub fn cached_log_probabilities(&self) -> Vec<$scalar> {
                self.inner.cached_log_probabilities().to_vec()
            }
            pub fn log_probability(&self, frames: usize) -> Result<$scalar, JsValue> {
                self.inner.log_probability(frames).map_err(error)
            }
            pub fn probability(&self, frames: usize) -> Result<f64, JsValue> {
                self.inner.probability(frames).map_err(error)
            }
        }
        #[cfg(feature = "serialization")]
        #[wasm_bindgen]
        pub struct $decoded {
            inner: DurationParameters<$scalar>,
            position: u64,
        }
        #[cfg(feature = "serialization")]
        #[wasm_bindgen]
        impl $decoded {
            #[wasm_bindgen(getter)]
            pub fn position(&self) -> u64 {
                self.position
            }
            pub fn parameters(&self) -> $duration {
                $duration {
                    inner: self.inner.clone(),
                }
            }
            pub fn into_parameters(self) -> $duration {
                $duration { inner: self.inner }
            }
        }
    };
}
duration_api!(
    f32,
    DurationF32,
    PreparedDurationF32,
    PreparationOptionsF32,
    DurationStatisticsF32,
    UpdateOptionsF32,
    DecodedDurationF32
);
duration_api!(
    f64,
    DurationF64,
    PreparedDurationF64,
    PreparationOptionsF64,
    DurationStatisticsF64,
    UpdateOptionsF64,
    DecodedDurationF64
);
