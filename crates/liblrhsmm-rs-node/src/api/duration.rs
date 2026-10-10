use crate::model::DurationParameters;
use crate::preparation::{OwnedPreparedDurationParameters, PreparationSettings};
use crate::statistics::DurationSufficientStatistics;
use crate::update::{DurationMode, UpdateSettings};
use napi::Error as BindingError;
fn error(value: impl std::fmt::Display) -> BindingError {
    napi::Error::from_reason(value.to_string())
}
#[derive(Clone)]
pub struct PreparationOptionsF32 {
    pub duration_cache_length: usize,
    pub duration_weight: f32,
    pub temperature: f32,
}
impl PreparationOptionsF32 {
    pub(crate) fn native(&self) -> PreparationSettings<f32> {
        PreparationSettings {
            duration_cache_length: self.duration_cache_length,
            duration_weight: self.duration_weight,
            temperature: self.temperature,
        }
    }
}
impl PreparationOptionsF32 {
    pub fn new() -> Self {
        let value = PreparationSettings::<f32>::default();
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
impl Default for PreparationOptionsF32 {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone)]
pub struct UpdateOptionsF32 {
    pub duration_mode: u32,
    pub duration_variance_floor: f32,
    pub emission_variance_floor: f32,
}
impl UpdateOptionsF32 {
    pub(crate) fn native(&self) -> Result<UpdateSettings<f32>, BindingError> {
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
impl UpdateOptionsF32 {
    pub fn new() -> Self {
        let value = UpdateSettings::<f32>::default();
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
impl Default for UpdateOptionsF32 {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone)]
pub struct DurationStatisticsF32 {
    pub value_sum: f32,
    pub squared_sum: f32,
    pub occupancy: f32,
}
impl DurationStatisticsF32 {
    pub(crate) fn native(&self) -> DurationSufficientStatistics<f32> {
        DurationSufficientStatistics {
            value_sum: self.value_sum,
            squared_sum: self.squared_sum,
            occupancy: self.occupancy,
        }
    }
    pub(crate) fn from_native(value: DurationSufficientStatistics<f32>) -> Self {
        Self {
            value_sum: value.value_sum,
            squared_sum: value.squared_sum,
            occupancy: value.occupancy,
        }
    }
}
impl DurationStatisticsF32 {
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
    pub fn values(&self) -> Vec<f32> {
        vec![self.value_sum, self.squared_sum, self.occupancy]
    }
    pub fn set_values(&mut self, values: &[f32]) -> Result<(), BindingError> {
        let [value_sum, squared_sum, occupancy] = values else {
            return Err(error("duration statistics require three values"));
        };
        self.value_sum = *value_sum;
        self.squared_sum = *squared_sum;
        self.occupancy = *occupancy;
        Ok(())
    }
}
impl Default for DurationStatisticsF32 {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone)]
pub struct DurationF32 {
    pub(crate) inner: DurationParameters<f32>,
}
impl DurationF32 {
    pub fn new() -> Self {
        Self {
            inner: DurationParameters::default(),
        }
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
    pub fn values(&self) -> Vec<f32> {
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
    pub fn set_values(&mut self, values: &[f32]) -> Result<(), BindingError> {
        let [mean, variance, variance_floor] = values else {
            return Err(error("duration requires three scalar values"));
        };
        self.inner.mean = *mean;
        self.inner.variance = *variance;
        self.inner.variance_floor = *variance_floor;
        Ok(())
    }
    pub fn set_constraints(&mut self, values: &[i32]) -> Result<(), BindingError> {
        let [minimum, maximum, fixed_mean] = values else {
            return Err(error("duration requires three constraints"));
        };
        self.inner.minimum = *minimum;
        self.inner.maximum = *maximum;
        self.inner.fixed_mean = *fixed_mean;
        Ok(())
    }
    pub fn log_probability(&self, frames: f32, weight: f32) -> Result<f32, BindingError> {
        self.inner.log_probability(frames, weight).map_err(error)
    }
    pub fn annealed_log_probability(
        &self,
        frames: usize,
        weight: f32,
        temperature: f32,
    ) -> Result<f32, BindingError> {
        self.inner
            .annealed_log_probability(frames, weight, temperature)
            .map_err(error)
    }
    pub fn probability(
        &self,
        frames: usize,
        weight: f32,
        temperature: f32,
    ) -> Result<f64, BindingError> {
        self.inner
            .probability(frames, weight, temperature)
            .map_err(error)
    }
    pub fn prepare(
        &self,
        options: &PreparationOptionsF32,
    ) -> Result<PreparedDurationF32, BindingError> {
        self.cloned().prepare_owned(options)
    }
    pub fn prepare_owned(
        self,
        options: &PreparationOptionsF32,
    ) -> Result<PreparedDurationF32, BindingError> {
        self.inner
            .prepare_owned(options.native())
            .map(|inner| PreparedDurationF32 { inner })
            .map_err(error)
    }
    pub fn update(
        &mut self,
        statistics: &DurationStatisticsF32,
        options: &UpdateOptionsF32,
    ) -> Result<bool, BindingError> {
        self.inner
            .update(&statistics.native(), options.native()?)
            .map_err(error)
    }
    #[cfg(feature = "serialization")]
    pub fn read(bytes: &[u8]) -> Result<Self, BindingError> {
        DurationParameters::read_from(bytes)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    #[cfg(feature = "serialization")]
    pub fn read_with_limits(bytes: &[u8], max_array_entries: usize) -> Result<Self, BindingError> {
        DurationParameters::read_with_limits(
            bytes,
            crate::serial::DecodeLimits { max_array_entries },
        )
        .map(|inner| Self { inner })
        .map_err(error)
    }
    #[cfg(feature = "serialization")]
    pub fn read_prefix(bytes: &[u8]) -> Result<DecodedDurationF32, BindingError> {
        Self::read_prefix_with_limits(
            bytes,
            crate::serial::DecodeLimits::default().max_array_entries,
        )
    }
    #[cfg(feature = "serialization")]
    pub fn read_prefix_with_limits(
        bytes: &[u8],
        max_array_entries: usize,
    ) -> Result<DecodedDurationF32, BindingError> {
        let mut reader = std::io::Cursor::new(bytes);
        let inner = DurationParameters::read_prefix_with_limits(
            &mut reader,
            crate::serial::DecodeLimits { max_array_entries },
        )
        .map_err(error)?;
        Ok(DecodedDurationF32 {
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
}
impl Default for DurationF32 {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone)]
pub struct PreparedDurationF32 {
    pub(crate) inner: OwnedPreparedDurationParameters<f32>,
}
impl PreparedDurationF32 {
    pub fn cloned(&self) -> Self {
        self.clone()
    }
    pub fn parameters(&self) -> DurationF32 {
        DurationF32 {
            inner: self.inner.parameters().clone(),
        }
    }
    pub fn into_parameters(self) -> DurationF32 {
        DurationF32 {
            inner: self.inner.into_parameters(),
        }
    }
    pub fn options(&self) -> PreparationOptionsF32 {
        let value = self.inner.options();
        PreparationOptionsF32 {
            duration_cache_length: value.duration_cache_length,
            duration_weight: value.duration_weight,
            temperature: value.temperature,
        }
    }
    pub fn log_normalizer(&self) -> f32 {
        self.inner.log_normalizer()
    }
    pub fn cached_log_probabilities(&self) -> Vec<f32> {
        self.inner.cached_log_probabilities().to_vec()
    }
    pub fn log_probability(&self, frames: usize) -> Result<f32, BindingError> {
        self.inner.log_probability(frames).map_err(error)
    }
    pub fn probability(&self, frames: usize) -> Result<f64, BindingError> {
        self.inner.probability(frames).map_err(error)
    }
}
#[cfg(feature = "serialization")]
pub struct DecodedDurationF32 {
    inner: DurationParameters<f32>,
    position: u64,
}
#[cfg(feature = "serialization")]
impl DecodedDurationF32 {
    pub fn position(&self) -> u64 {
        self.position
    }
    pub fn parameters(&self) -> DurationF32 {
        DurationF32 {
            inner: self.inner.clone(),
        }
    }
    pub fn into_parameters(self) -> DurationF32 {
        DurationF32 { inner: self.inner }
    }
}
#[derive(Clone)]
pub struct PreparationOptionsF64 {
    pub duration_cache_length: usize,
    pub duration_weight: f64,
    pub temperature: f64,
}
impl PreparationOptionsF64 {
    pub(crate) fn native(&self) -> PreparationSettings<f64> {
        PreparationSettings {
            duration_cache_length: self.duration_cache_length,
            duration_weight: self.duration_weight,
            temperature: self.temperature,
        }
    }
}
impl PreparationOptionsF64 {
    pub fn new() -> Self {
        let value = PreparationSettings::<f64>::default();
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
impl Default for PreparationOptionsF64 {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone)]
pub struct UpdateOptionsF64 {
    pub duration_mode: u32,
    pub duration_variance_floor: f64,
    pub emission_variance_floor: f64,
}
impl UpdateOptionsF64 {
    pub(crate) fn native(&self) -> Result<UpdateSettings<f64>, BindingError> {
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
impl UpdateOptionsF64 {
    pub fn new() -> Self {
        let value = UpdateSettings::<f64>::default();
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
impl Default for UpdateOptionsF64 {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone)]
pub struct DurationStatisticsF64 {
    pub value_sum: f64,
    pub squared_sum: f64,
    pub occupancy: f64,
}
impl DurationStatisticsF64 {
    pub(crate) fn native(&self) -> DurationSufficientStatistics<f64> {
        DurationSufficientStatistics {
            value_sum: self.value_sum,
            squared_sum: self.squared_sum,
            occupancy: self.occupancy,
        }
    }
    pub(crate) fn from_native(value: DurationSufficientStatistics<f64>) -> Self {
        Self {
            value_sum: value.value_sum,
            squared_sum: value.squared_sum,
            occupancy: value.occupancy,
        }
    }
}
impl DurationStatisticsF64 {
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
    pub fn values(&self) -> Vec<f64> {
        vec![self.value_sum, self.squared_sum, self.occupancy]
    }
    pub fn set_values(&mut self, values: &[f64]) -> Result<(), BindingError> {
        let [value_sum, squared_sum, occupancy] = values else {
            return Err(error("duration statistics require three values"));
        };
        self.value_sum = *value_sum;
        self.squared_sum = *squared_sum;
        self.occupancy = *occupancy;
        Ok(())
    }
}
impl Default for DurationStatisticsF64 {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone)]
pub struct DurationF64 {
    pub(crate) inner: DurationParameters<f64>,
}
impl DurationF64 {
    pub fn new() -> Self {
        Self {
            inner: DurationParameters::default(),
        }
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
    pub fn values(&self) -> Vec<f64> {
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
    pub fn set_values(&mut self, values: &[f64]) -> Result<(), BindingError> {
        let [mean, variance, variance_floor] = values else {
            return Err(error("duration requires three scalar values"));
        };
        self.inner.mean = *mean;
        self.inner.variance = *variance;
        self.inner.variance_floor = *variance_floor;
        Ok(())
    }
    pub fn set_constraints(&mut self, values: &[i32]) -> Result<(), BindingError> {
        let [minimum, maximum, fixed_mean] = values else {
            return Err(error("duration requires three constraints"));
        };
        self.inner.minimum = *minimum;
        self.inner.maximum = *maximum;
        self.inner.fixed_mean = *fixed_mean;
        Ok(())
    }
    pub fn log_probability(&self, frames: f64, weight: f64) -> Result<f64, BindingError> {
        self.inner.log_probability(frames, weight).map_err(error)
    }
    pub fn annealed_log_probability(
        &self,
        frames: usize,
        weight: f64,
        temperature: f64,
    ) -> Result<f64, BindingError> {
        self.inner
            .annealed_log_probability(frames, weight, temperature)
            .map_err(error)
    }
    pub fn probability(
        &self,
        frames: usize,
        weight: f64,
        temperature: f64,
    ) -> Result<f64, BindingError> {
        self.inner
            .probability(frames, weight, temperature)
            .map_err(error)
    }
    pub fn prepare(
        &self,
        options: &PreparationOptionsF64,
    ) -> Result<PreparedDurationF64, BindingError> {
        self.cloned().prepare_owned(options)
    }
    pub fn prepare_owned(
        self,
        options: &PreparationOptionsF64,
    ) -> Result<PreparedDurationF64, BindingError> {
        self.inner
            .prepare_owned(options.native())
            .map(|inner| PreparedDurationF64 { inner })
            .map_err(error)
    }
    pub fn update(
        &mut self,
        statistics: &DurationStatisticsF64,
        options: &UpdateOptionsF64,
    ) -> Result<bool, BindingError> {
        self.inner
            .update(&statistics.native(), options.native()?)
            .map_err(error)
    }
    #[cfg(feature = "serialization")]
    pub fn read(bytes: &[u8]) -> Result<Self, BindingError> {
        DurationParameters::read_from(bytes)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    #[cfg(feature = "serialization")]
    pub fn read_with_limits(bytes: &[u8], max_array_entries: usize) -> Result<Self, BindingError> {
        DurationParameters::read_with_limits(
            bytes,
            crate::serial::DecodeLimits { max_array_entries },
        )
        .map(|inner| Self { inner })
        .map_err(error)
    }
    #[cfg(feature = "serialization")]
    pub fn read_prefix(bytes: &[u8]) -> Result<DecodedDurationF64, BindingError> {
        Self::read_prefix_with_limits(
            bytes,
            crate::serial::DecodeLimits::default().max_array_entries,
        )
    }
    #[cfg(feature = "serialization")]
    pub fn read_prefix_with_limits(
        bytes: &[u8],
        max_array_entries: usize,
    ) -> Result<DecodedDurationF64, BindingError> {
        let mut reader = std::io::Cursor::new(bytes);
        let inner = DurationParameters::read_prefix_with_limits(
            &mut reader,
            crate::serial::DecodeLimits { max_array_entries },
        )
        .map_err(error)?;
        Ok(DecodedDurationF64 {
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
}
impl Default for DurationF64 {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone)]
pub struct PreparedDurationF64 {
    pub(crate) inner: OwnedPreparedDurationParameters<f64>,
}
impl PreparedDurationF64 {
    pub fn cloned(&self) -> Self {
        self.clone()
    }
    pub fn parameters(&self) -> DurationF64 {
        DurationF64 {
            inner: self.inner.parameters().clone(),
        }
    }
    pub fn into_parameters(self) -> DurationF64 {
        DurationF64 {
            inner: self.inner.into_parameters(),
        }
    }
    pub fn options(&self) -> PreparationOptionsF64 {
        let value = self.inner.options();
        PreparationOptionsF64 {
            duration_cache_length: value.duration_cache_length,
            duration_weight: value.duration_weight,
            temperature: value.temperature,
        }
    }
    pub fn log_normalizer(&self) -> f64 {
        self.inner.log_normalizer()
    }
    pub fn cached_log_probabilities(&self) -> Vec<f64> {
        self.inner.cached_log_probabilities().to_vec()
    }
    pub fn log_probability(&self, frames: usize) -> Result<f64, BindingError> {
        self.inner.log_probability(frames).map_err(error)
    }
    pub fn probability(&self, frames: usize) -> Result<f64, BindingError> {
        self.inner.probability(frames).map_err(error)
    }
}
#[cfg(feature = "serialization")]
pub struct DecodedDurationF64 {
    inner: DurationParameters<f64>,
    position: u64,
}
#[cfg(feature = "serialization")]
impl DecodedDurationF64 {
    pub fn position(&self) -> u64 {
        self.position
    }
    pub fn parameters(&self) -> DurationF64 {
        DurationF64 {
            inner: self.inner.clone(),
        }
    }
    pub fn into_parameters(self) -> DurationF64 {
        DurationF64 { inner: self.inner }
    }
}
