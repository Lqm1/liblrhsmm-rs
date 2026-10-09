use super::{DurationF32, DurationF64, UpdateOptionsF32, UpdateOptionsF64};
use crate::model::{DurationParameters, GaussianMixtureParameters};
use crate::probability::OwnedPreparedGaussianParameters;
use crate::statistics::GaussianSufficientStatistics;
use wasm_bindgen::prelude::*;

fn error(value: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&value.to_string())
}

macro_rules! collection {
    ($name:ident, $owner:ident, $native:ty) => {
        #[wasm_bindgen]
        #[derive(Clone, Default)]
        pub struct $name {
            pub(crate) inner: Vec<$native>,
        }
        #[wasm_bindgen]
        impl $name {
            #[wasm_bindgen(constructor)]
            pub fn new() -> Self {
                Self::default()
            }
            #[wasm_bindgen(getter)]
            pub fn length(&self) -> usize {
                self.inner.len()
            }
            pub fn is_empty(&self) -> bool {
                self.inner.is_empty()
            }
            pub fn push(&mut self, value: &$owner) {
                self.inner.push(value.inner.clone());
            }
            pub fn get(&self, index: usize) -> Option<$owner> {
                self.inner.get(index).cloned().map(|inner| $owner { inner })
            }
            pub fn replace(&mut self, index: usize, value: &$owner) -> Result<(), JsValue> {
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
    };
}
collection!(DurationCollectionF32, DurationF32, DurationParameters<f32>);
collection!(DurationCollectionF64, DurationF64, DurationParameters<f64>);

macro_rules! gaussian_api {
    ($scalar:ty, $gaussian:ident, $prepared:ident, $statistics:ident, $update:ident, $collection:ident, $durations:ident, $decoded:ident) => {
        collection!($collection, $gaussian, GaussianMixtureParameters<$scalar>);
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $gaussian {
            pub(crate) inner: GaussianMixtureParameters<$scalar>,
        }
        #[wasm_bindgen]
        impl $gaussian {
            #[wasm_bindgen(constructor)]
            pub fn new(mixtures: usize, dimensions: usize) -> Result<Self, JsValue> {
                GaussianMixtureParameters::new(mixtures, dimensions)
                    .map(|inner| Self { inner })
                    .map_err(error)
            }
            #[wasm_bindgen(getter)]
            pub fn dimensions(&self) -> usize {
                self.inner.dimensions
            }
            #[wasm_bindgen(getter)]
            pub fn mixtures(&self) -> usize {
                self.inner.weights.len()
            }
            pub fn weights(&self) -> Vec<$scalar> {
                self.inner.weights.clone()
            }
            pub fn means(&self) -> Vec<$scalar> {
                self.inner.means.clone()
            }
            pub fn variances(&self) -> Vec<$scalar> {
                self.inner.variances.clone()
            }
            pub fn variance_floors(&self) -> Vec<$scalar> {
                self.inner.variance_floors.clone()
            }
            pub fn replace(
                &mut self,
                dimensions: usize,
                weights: &[$scalar],
                means: &[$scalar],
                variances: &[$scalar],
                variance_floors: &[$scalar],
            ) -> Result<(), JsValue> {
                let inner = GaussianMixtureParameters {
                    dimensions,
                    weights: weights.to_vec(),
                    means: means.to_vec(),
                    variances: variances.to_vec(),
                    variance_floors: variance_floors.to_vec(),
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
            pub fn combine(distributions: &$collection) -> Result<Self, JsValue> {
                GaussianMixtureParameters::combine(&distributions.inner)
                    .map(|inner| Self { inner })
                    .map_err(error)
            }
            pub fn from_durations(distributions: &$durations) -> Result<Self, JsValue> {
                GaussianMixtureParameters::from_durations(&distributions.inner)
                    .map(|inner| Self { inner })
                    .map_err(error)
            }
            pub fn increase_mixtures(
                &mut self,
                count: usize,
                displacement: $scalar,
            ) -> Result<(), JsValue> {
                self.inner
                    .increase_mixtures(count, displacement)
                    .map_err(error)
            }
            pub fn prepare(&self) -> Result<$prepared, JsValue> {
                self.cloned().prepare_owned()
            }
            pub fn prepare_owned(self) -> Result<$prepared, JsValue> {
                self.inner
                    .prepare_owned()
                    .map(|inner| $prepared { inner })
                    .map_err(error)
            }
            pub fn update(
                &mut self,
                statistics: &$statistics,
                options: &$update,
            ) -> Result<bool, JsValue> {
                self.inner
                    .update(&statistics.inner, options.native()?)
                    .map_err(error)
            }
            #[cfg(feature = "serialization")]
            pub fn read(bytes: &[u8]) -> Result<Self, JsValue> {
                GaussianMixtureParameters::read_from(bytes)
                    .map(|inner| Self { inner })
                    .map_err(error)
            }
            #[cfg(feature = "serialization")]
            pub fn read_with_limits(
                bytes: &[u8],
                max_array_entries: usize,
            ) -> Result<Self, JsValue> {
                GaussianMixtureParameters::read_with_limits(
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
                let inner = GaussianMixtureParameters::read_prefix_with_limits(
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
            #[cfg(feature = "serialization")]
            pub fn write_with_encoding(&self, code: u32) -> Result<Vec<u8>, JsValue> {
                let encoding = match code {
                    0 => crate::serial::ModelEncoding::WithVarianceFloors,
                    1 => crate::serial::ModelEncoding::WithoutVarianceFloors,
                    _ => return Err(error("invalid model encoding")),
                };
                let mut bytes = Vec::new();
                self.inner
                    .write_with_encoding(&mut bytes, encoding)
                    .map_err(error)?;
                Ok(bytes)
            }
        }
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $prepared {
            pub(crate) inner: OwnedPreparedGaussianParameters<$scalar>,
        }
        #[wasm_bindgen]
        impl $prepared {
            pub fn cloned(&self) -> Self {
                self.clone()
            }
            pub fn parameters(&self) -> $gaussian {
                $gaussian {
                    inner: self.inner.parameters().clone(),
                }
            }
            pub fn into_parameters(self) -> $gaussian {
                $gaussian {
                    inner: self.inner.into_parameters(),
                }
            }
            pub fn cached_log_normalizers(&self) -> Vec<$scalar> {
                self.inner.cached_log_normalizers().to_vec()
            }
            pub fn component_log_probability(
                &self,
                observation: &[$scalar],
                component: usize,
            ) -> Result<$scalar, JsValue> {
                self.inner
                    .component_log_probability(observation, component)
                    .map_err(error)
            }
            pub fn component_probability(
                &self,
                observation: &[$scalar],
                component: usize,
            ) -> Result<f64, JsValue> {
                self.inner
                    .component_probability(observation, component)
                    .map_err(error)
            }
            pub fn log_probability(
                &self,
                observation: &[$scalar],
                temperature: $scalar,
            ) -> Result<$scalar, JsValue> {
                self.inner
                    .log_probability(observation, temperature)
                    .map_err(error)
            }
            pub fn probability(
                &self,
                observation: &[$scalar],
                temperature: $scalar,
            ) -> Result<f64, JsValue> {
                self.inner
                    .probability(observation, temperature)
                    .map_err(error)
            }
        }
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $statistics {
            pub(crate) inner: GaussianSufficientStatistics<$scalar>,
        }
        #[wasm_bindgen]
        impl $statistics {
            #[wasm_bindgen(constructor)]
            pub fn new(mixtures: usize, dimensions: usize) -> Result<Self, JsValue> {
                GaussianSufficientStatistics::new(mixtures, dimensions)
                    .map(|inner| Self { inner })
                    .map_err(error)
            }
            pub fn from_gaussian(value: &$gaussian) -> Result<Self, JsValue> {
                GaussianSufficientStatistics::from_gaussian(&value.inner)
                    .map(|inner| Self { inner })
                    .map_err(error)
            }
            #[wasm_bindgen(getter)]
            pub fn dimensions(&self) -> usize {
                self.inner.dimensions
            }
            #[wasm_bindgen(getter)]
            pub fn mixtures(&self) -> usize {
                self.inner.component_occupancies.len()
            }
            pub fn weighted_sums(&self) -> Vec<$scalar> {
                self.inner.weighted_sums.clone()
            }
            pub fn weighted_squared_sums(&self) -> Vec<$scalar> {
                self.inner.weighted_squared_sums.clone()
            }
            pub fn component_occupancies(&self) -> Vec<$scalar> {
                self.inner.component_occupancies.clone()
            }
            pub fn state_occupancy(&self) -> Vec<$scalar> {
                vec![self.inner.state_occupancy]
            }
            pub fn replace(
                &mut self,
                dimensions: usize,
                weighted_sums: &[$scalar],
                weighted_squared_sums: &[$scalar],
                component_occupancies: &[$scalar],
                state_occupancy: &[$scalar],
            ) -> Result<(), JsValue> {
                let [state_occupancy] = state_occupancy else {
                    return Err(error("state occupancy requires one scalar"));
                };
                let expected = GaussianSufficientStatistics::<$scalar>::new(
                    component_occupancies.len(),
                    dimensions,
                )
                .map_err(error)?;
                if weighted_sums.len() != expected.weighted_sums.len()
                    || weighted_squared_sums.len() != expected.weighted_squared_sums.len()
                {
                    return Err(error("Gaussian statistic shapes differ"));
                }
                self.inner = GaussianSufficientStatistics {
                    dimensions,
                    weighted_sums: weighted_sums.to_vec(),
                    weighted_squared_sums: weighted_squared_sums.to_vec(),
                    component_occupancies: component_occupancies.to_vec(),
                    state_occupancy: *state_occupancy,
                };
                Ok(())
            }
            pub fn cloned(&self) -> Self {
                self.clone()
            }
            pub fn clear(&mut self) {
                self.inner.clear();
            }
        }
        #[cfg(feature = "serialization")]
        #[wasm_bindgen]
        pub struct $decoded {
            inner: GaussianMixtureParameters<$scalar>,
            position: u64,
        }
        #[cfg(feature = "serialization")]
        #[wasm_bindgen]
        impl $decoded {
            #[wasm_bindgen(getter)]
            pub fn position(&self) -> u64 {
                self.position
            }
            pub fn parameters(&self) -> $gaussian {
                $gaussian {
                    inner: self.inner.clone(),
                }
            }
            pub fn into_parameters(self) -> $gaussian {
                $gaussian { inner: self.inner }
            }
        }
    };
}
gaussian_api!(
    f32,
    GaussianF32,
    PreparedGaussianF32,
    GaussianStatisticsF32,
    UpdateOptionsF32,
    GaussianCollectionF32,
    DurationCollectionF32,
    DecodedGaussianF32
);
gaussian_api!(
    f64,
    GaussianF64,
    PreparedGaussianF64,
    GaussianStatisticsF64,
    UpdateOptionsF64,
    GaussianCollectionF64,
    DurationCollectionF64,
    DecodedGaussianF64
);
