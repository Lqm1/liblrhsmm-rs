use super::{DurationF32, DurationF64, UpdateOptionsF32, UpdateOptionsF64};
use crate::model::{DurationParameters, GaussianMixtureParameters};
use crate::probability::OwnedPreparedGaussianParameters;
use crate::statistics::GaussianSufficientStatistics;
use napi::Error as BindingError;
fn error(value: impl std::fmt::Display) -> BindingError {
    napi::Error::from_reason(value.to_string())
}
#[derive(Clone, Default)]
pub struct DurationCollectionF32 {
    pub(crate) inner: Vec<DurationParameters<f32>>,
}
impl DurationCollectionF32 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn length(&self) -> usize {
        self.inner.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn push(&mut self, value: &DurationF32) {
        self.inner.push(value.inner.clone());
    }
    pub fn get(&self, index: usize) -> Option<DurationF32> {
        self.inner
            .get(index)
            .cloned()
            .map(|inner| DurationF32 { inner })
    }
    pub fn replace(&mut self, index: usize, value: &DurationF32) -> Result<(), BindingError> {
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
pub struct DurationCollectionF64 {
    pub(crate) inner: Vec<DurationParameters<f64>>,
}
impl DurationCollectionF64 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn length(&self) -> usize {
        self.inner.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn push(&mut self, value: &DurationF64) {
        self.inner.push(value.inner.clone());
    }
    pub fn get(&self, index: usize) -> Option<DurationF64> {
        self.inner
            .get(index)
            .cloned()
            .map(|inner| DurationF64 { inner })
    }
    pub fn replace(&mut self, index: usize, value: &DurationF64) -> Result<(), BindingError> {
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
pub struct GaussianCollectionF32 {
    pub(crate) inner: Vec<GaussianMixtureParameters<f32>>,
}
impl GaussianCollectionF32 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn length(&self) -> usize {
        self.inner.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn push(&mut self, value: &GaussianF32) {
        self.inner.push(value.inner.clone());
    }
    pub fn get(&self, index: usize) -> Option<GaussianF32> {
        self.inner
            .get(index)
            .cloned()
            .map(|inner| GaussianF32 { inner })
    }
    pub fn replace(&mut self, index: usize, value: &GaussianF32) -> Result<(), BindingError> {
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
pub struct GaussianF32 {
    pub(crate) inner: GaussianMixtureParameters<f32>,
}
impl GaussianF32 {
    pub fn new(mixtures: usize, dimensions: usize) -> Result<Self, BindingError> {
        GaussianMixtureParameters::new(mixtures, dimensions)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn dimensions(&self) -> usize {
        self.inner.dimensions
    }
    pub fn mixtures(&self) -> usize {
        self.inner.weights.len()
    }
    pub fn weights(&self) -> Vec<f32> {
        self.inner.weights.clone()
    }
    pub fn means(&self) -> Vec<f32> {
        self.inner.means.clone()
    }
    pub fn variances(&self) -> Vec<f32> {
        self.inner.variances.clone()
    }
    pub fn variance_floors(&self) -> Vec<f32> {
        self.inner.variance_floors.clone()
    }
    pub fn replace(
        &mut self,
        dimensions: usize,
        weights: &[f32],
        means: &[f32],
        variances: &[f32],
        variance_floors: &[f32],
    ) -> Result<(), BindingError> {
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
    pub fn validate(&self) -> Result<(), BindingError> {
        self.inner.validate().map_err(error)
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
    pub fn combine(distributions: &GaussianCollectionF32) -> Result<Self, BindingError> {
        GaussianMixtureParameters::combine(&distributions.inner)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn from_durations(distributions: &DurationCollectionF32) -> Result<Self, BindingError> {
        GaussianMixtureParameters::from_durations(&distributions.inner)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn increase_mixtures(
        &mut self,
        count: usize,
        displacement: f32,
    ) -> Result<(), BindingError> {
        self.inner
            .increase_mixtures(count, displacement)
            .map_err(error)
    }
    pub fn prepare(&self) -> Result<PreparedGaussianF32, BindingError> {
        self.cloned().prepare_owned()
    }
    pub fn prepare_owned(self) -> Result<PreparedGaussianF32, BindingError> {
        self.inner
            .prepare_owned()
            .map(|inner| PreparedGaussianF32 { inner })
            .map_err(error)
    }
    pub fn update(
        &mut self,
        statistics: &GaussianStatisticsF32,
        options: &UpdateOptionsF32,
    ) -> Result<bool, BindingError> {
        self.inner
            .update(&statistics.inner, options.native()?)
            .map_err(error)
    }
    #[cfg(feature = "serialization")]
    pub fn read(bytes: &[u8]) -> Result<Self, BindingError> {
        GaussianMixtureParameters::read_from(bytes)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    #[cfg(feature = "serialization")]
    pub fn read_with_limits(bytes: &[u8], max_array_entries: usize) -> Result<Self, BindingError> {
        GaussianMixtureParameters::read_with_limits(
            bytes,
            crate::serial::DecodeLimits { max_array_entries },
        )
        .map(|inner| Self { inner })
        .map_err(error)
    }
    #[cfg(feature = "serialization")]
    pub fn read_prefix(bytes: &[u8]) -> Result<DecodedGaussianF32, BindingError> {
        Self::read_prefix_with_limits(
            bytes,
            crate::serial::DecodeLimits::default().max_array_entries,
        )
    }
    #[cfg(feature = "serialization")]
    pub fn read_prefix_with_limits(
        bytes: &[u8],
        max_array_entries: usize,
    ) -> Result<DecodedGaussianF32, BindingError> {
        let mut reader = std::io::Cursor::new(bytes);
        let inner = GaussianMixtureParameters::read_prefix_with_limits(
            &mut reader,
            crate::serial::DecodeLimits { max_array_entries },
        )
        .map_err(error)?;
        Ok(DecodedGaussianF32 {
            inner,
            position: reader.position(),
        })
    }
    #[cfg(feature = "serialization")]
    pub fn write(&self) -> Result<Vec<u8>, BindingError> {
        let mut bytes = Vec::new();
        self.inner.write_to(&mut bytes).map_err(error)?;
        Ok(bytes)
    }
    #[cfg(feature = "serialization")]
    pub fn write_with_encoding(&self, code: u32) -> Result<Vec<u8>, BindingError> {
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
#[derive(Clone)]
pub struct PreparedGaussianF32 {
    pub(crate) inner: OwnedPreparedGaussianParameters<f32>,
}
impl PreparedGaussianF32 {
    pub fn cloned(&self) -> Self {
        self.clone()
    }
    pub fn parameters(&self) -> GaussianF32 {
        GaussianF32 {
            inner: self.inner.parameters().clone(),
        }
    }
    pub fn into_parameters(self) -> GaussianF32 {
        GaussianF32 {
            inner: self.inner.into_parameters(),
        }
    }
    pub fn cached_log_normalizers(&self) -> Vec<f32> {
        self.inner.cached_log_normalizers().to_vec()
    }
    pub fn component_log_probability(
        &self,
        observation: &[f32],
        component: usize,
    ) -> Result<f32, BindingError> {
        self.inner
            .component_log_probability(observation, component)
            .map_err(error)
    }
    pub fn component_probability(
        &self,
        observation: &[f32],
        component: usize,
    ) -> Result<f64, BindingError> {
        self.inner
            .component_probability(observation, component)
            .map_err(error)
    }
    pub fn log_probability(
        &self,
        observation: &[f32],
        temperature: f32,
    ) -> Result<f32, BindingError> {
        self.inner
            .log_probability(observation, temperature)
            .map_err(error)
    }
    pub fn probability(&self, observation: &[f32], temperature: f32) -> Result<f64, BindingError> {
        self.inner
            .probability(observation, temperature)
            .map_err(error)
    }
}
#[derive(Clone)]
pub struct GaussianStatisticsF32 {
    pub(crate) inner: GaussianSufficientStatistics<f32>,
}
impl GaussianStatisticsF32 {
    pub fn new(mixtures: usize, dimensions: usize) -> Result<Self, BindingError> {
        GaussianSufficientStatistics::new(mixtures, dimensions)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn from_gaussian(value: &GaussianF32) -> Result<Self, BindingError> {
        GaussianSufficientStatistics::from_gaussian(&value.inner)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn dimensions(&self) -> usize {
        self.inner.dimensions
    }
    pub fn mixtures(&self) -> usize {
        self.inner.component_occupancies.len()
    }
    pub fn weighted_sums(&self) -> Vec<f32> {
        self.inner.weighted_sums.clone()
    }
    pub fn weighted_squared_sums(&self) -> Vec<f32> {
        self.inner.weighted_squared_sums.clone()
    }
    pub fn component_occupancies(&self) -> Vec<f32> {
        self.inner.component_occupancies.clone()
    }
    pub fn state_occupancy(&self) -> Vec<f32> {
        vec![self.inner.state_occupancy]
    }
    pub fn replace(
        &mut self,
        dimensions: usize,
        weighted_sums: &[f32],
        weighted_squared_sums: &[f32],
        component_occupancies: &[f32],
        state_occupancy: &[f32],
    ) -> Result<(), BindingError> {
        let [state_occupancy] = state_occupancy else {
            return Err(error("state occupancy requires one scalar"));
        };
        let expected =
            GaussianSufficientStatistics::<f32>::new(component_occupancies.len(), dimensions)
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
pub struct DecodedGaussianF32 {
    inner: GaussianMixtureParameters<f32>,
    position: u64,
}
#[cfg(feature = "serialization")]
impl DecodedGaussianF32 {
    pub fn position(&self) -> u64 {
        self.position
    }
    pub fn parameters(&self) -> GaussianF32 {
        GaussianF32 {
            inner: self.inner.clone(),
        }
    }
    pub fn into_parameters(self) -> GaussianF32 {
        GaussianF32 { inner: self.inner }
    }
}
#[derive(Clone, Default)]
pub struct GaussianCollectionF64 {
    pub(crate) inner: Vec<GaussianMixtureParameters<f64>>,
}
impl GaussianCollectionF64 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn length(&self) -> usize {
        self.inner.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn push(&mut self, value: &GaussianF64) {
        self.inner.push(value.inner.clone());
    }
    pub fn get(&self, index: usize) -> Option<GaussianF64> {
        self.inner
            .get(index)
            .cloned()
            .map(|inner| GaussianF64 { inner })
    }
    pub fn replace(&mut self, index: usize, value: &GaussianF64) -> Result<(), BindingError> {
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
pub struct GaussianF64 {
    pub(crate) inner: GaussianMixtureParameters<f64>,
}
impl GaussianF64 {
    pub fn new(mixtures: usize, dimensions: usize) -> Result<Self, BindingError> {
        GaussianMixtureParameters::new(mixtures, dimensions)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn dimensions(&self) -> usize {
        self.inner.dimensions
    }
    pub fn mixtures(&self) -> usize {
        self.inner.weights.len()
    }
    pub fn weights(&self) -> Vec<f64> {
        self.inner.weights.clone()
    }
    pub fn means(&self) -> Vec<f64> {
        self.inner.means.clone()
    }
    pub fn variances(&self) -> Vec<f64> {
        self.inner.variances.clone()
    }
    pub fn variance_floors(&self) -> Vec<f64> {
        self.inner.variance_floors.clone()
    }
    pub fn replace(
        &mut self,
        dimensions: usize,
        weights: &[f64],
        means: &[f64],
        variances: &[f64],
        variance_floors: &[f64],
    ) -> Result<(), BindingError> {
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
    pub fn validate(&self) -> Result<(), BindingError> {
        self.inner.validate().map_err(error)
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
    pub fn combine(distributions: &GaussianCollectionF64) -> Result<Self, BindingError> {
        GaussianMixtureParameters::combine(&distributions.inner)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn from_durations(distributions: &DurationCollectionF64) -> Result<Self, BindingError> {
        GaussianMixtureParameters::from_durations(&distributions.inner)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn increase_mixtures(
        &mut self,
        count: usize,
        displacement: f64,
    ) -> Result<(), BindingError> {
        self.inner
            .increase_mixtures(count, displacement)
            .map_err(error)
    }
    pub fn prepare(&self) -> Result<PreparedGaussianF64, BindingError> {
        self.cloned().prepare_owned()
    }
    pub fn prepare_owned(self) -> Result<PreparedGaussianF64, BindingError> {
        self.inner
            .prepare_owned()
            .map(|inner| PreparedGaussianF64 { inner })
            .map_err(error)
    }
    pub fn update(
        &mut self,
        statistics: &GaussianStatisticsF64,
        options: &UpdateOptionsF64,
    ) -> Result<bool, BindingError> {
        self.inner
            .update(&statistics.inner, options.native()?)
            .map_err(error)
    }
    #[cfg(feature = "serialization")]
    pub fn read(bytes: &[u8]) -> Result<Self, BindingError> {
        GaussianMixtureParameters::read_from(bytes)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    #[cfg(feature = "serialization")]
    pub fn read_with_limits(bytes: &[u8], max_array_entries: usize) -> Result<Self, BindingError> {
        GaussianMixtureParameters::read_with_limits(
            bytes,
            crate::serial::DecodeLimits { max_array_entries },
        )
        .map(|inner| Self { inner })
        .map_err(error)
    }
    #[cfg(feature = "serialization")]
    pub fn read_prefix(bytes: &[u8]) -> Result<DecodedGaussianF64, BindingError> {
        Self::read_prefix_with_limits(
            bytes,
            crate::serial::DecodeLimits::default().max_array_entries,
        )
    }
    #[cfg(feature = "serialization")]
    pub fn read_prefix_with_limits(
        bytes: &[u8],
        max_array_entries: usize,
    ) -> Result<DecodedGaussianF64, BindingError> {
        let mut reader = std::io::Cursor::new(bytes);
        let inner = GaussianMixtureParameters::read_prefix_with_limits(
            &mut reader,
            crate::serial::DecodeLimits { max_array_entries },
        )
        .map_err(error)?;
        Ok(DecodedGaussianF64 {
            inner,
            position: reader.position(),
        })
    }
    #[cfg(feature = "serialization")]
    pub fn write(&self) -> Result<Vec<u8>, BindingError> {
        let mut bytes = Vec::new();
        self.inner.write_to(&mut bytes).map_err(error)?;
        Ok(bytes)
    }
    #[cfg(feature = "serialization")]
    pub fn write_with_encoding(&self, code: u32) -> Result<Vec<u8>, BindingError> {
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
#[derive(Clone)]
pub struct PreparedGaussianF64 {
    pub(crate) inner: OwnedPreparedGaussianParameters<f64>,
}
impl PreparedGaussianF64 {
    pub fn cloned(&self) -> Self {
        self.clone()
    }
    pub fn parameters(&self) -> GaussianF64 {
        GaussianF64 {
            inner: self.inner.parameters().clone(),
        }
    }
    pub fn into_parameters(self) -> GaussianF64 {
        GaussianF64 {
            inner: self.inner.into_parameters(),
        }
    }
    pub fn cached_log_normalizers(&self) -> Vec<f64> {
        self.inner.cached_log_normalizers().to_vec()
    }
    pub fn component_log_probability(
        &self,
        observation: &[f64],
        component: usize,
    ) -> Result<f64, BindingError> {
        self.inner
            .component_log_probability(observation, component)
            .map_err(error)
    }
    pub fn component_probability(
        &self,
        observation: &[f64],
        component: usize,
    ) -> Result<f64, BindingError> {
        self.inner
            .component_probability(observation, component)
            .map_err(error)
    }
    pub fn log_probability(
        &self,
        observation: &[f64],
        temperature: f64,
    ) -> Result<f64, BindingError> {
        self.inner
            .log_probability(observation, temperature)
            .map_err(error)
    }
    pub fn probability(&self, observation: &[f64], temperature: f64) -> Result<f64, BindingError> {
        self.inner
            .probability(observation, temperature)
            .map_err(error)
    }
}
#[derive(Clone)]
pub struct GaussianStatisticsF64 {
    pub(crate) inner: GaussianSufficientStatistics<f64>,
}
impl GaussianStatisticsF64 {
    pub fn new(mixtures: usize, dimensions: usize) -> Result<Self, BindingError> {
        GaussianSufficientStatistics::new(mixtures, dimensions)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn from_gaussian(value: &GaussianF64) -> Result<Self, BindingError> {
        GaussianSufficientStatistics::from_gaussian(&value.inner)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn dimensions(&self) -> usize {
        self.inner.dimensions
    }
    pub fn mixtures(&self) -> usize {
        self.inner.component_occupancies.len()
    }
    pub fn weighted_sums(&self) -> Vec<f64> {
        self.inner.weighted_sums.clone()
    }
    pub fn weighted_squared_sums(&self) -> Vec<f64> {
        self.inner.weighted_squared_sums.clone()
    }
    pub fn component_occupancies(&self) -> Vec<f64> {
        self.inner.component_occupancies.clone()
    }
    pub fn state_occupancy(&self) -> Vec<f64> {
        vec![self.inner.state_occupancy]
    }
    pub fn replace(
        &mut self,
        dimensions: usize,
        weighted_sums: &[f64],
        weighted_squared_sums: &[f64],
        component_occupancies: &[f64],
        state_occupancy: &[f64],
    ) -> Result<(), BindingError> {
        let [state_occupancy] = state_occupancy else {
            return Err(error("state occupancy requires one scalar"));
        };
        let expected =
            GaussianSufficientStatistics::<f64>::new(component_occupancies.len(), dimensions)
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
pub struct DecodedGaussianF64 {
    inner: GaussianMixtureParameters<f64>,
    position: u64,
}
#[cfg(feature = "serialization")]
impl DecodedGaussianF64 {
    pub fn position(&self) -> u64 {
        self.position
    }
    pub fn parameters(&self) -> GaussianF64 {
        GaussianF64 {
            inner: self.inner.clone(),
        }
    }
    pub fn into_parameters(self) -> GaussianF64 {
        GaussianF64 { inner: self.inner }
    }
}
