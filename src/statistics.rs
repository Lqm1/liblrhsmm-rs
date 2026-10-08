//! Owned sufficient statistics for initialization and expectation maximization.
use crate::ModelError;
use crate::model::{GaussianMixtureParameters, ModelParameters, StreamParameters};
use crate::scalar::Scalar;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct DurationSufficientStatistics<T> {
    pub value_sum: T,
    pub squared_sum: T,
    pub occupancy: T,
}
impl<T: Scalar> DurationSufficientStatistics<T> {
    pub fn clear(&mut self) {
        *self = Self::default();
    }
    pub(crate) fn validate(&self) -> Result<(), ModelError> {
        if [self.value_sum, self.squared_sum, self.occupancy]
            .iter()
            .any(|value| !(*value).into().is_finite() || *value < T::ZERO)
        {
            return Err(ModelError(
                "duration statistics must be finite and nonnegative",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GaussianSufficientStatistics<T> {
    pub dimensions: usize,
    pub weighted_sums: Vec<T>,
    pub weighted_squared_sums: Vec<T>,
    pub component_occupancies: Vec<T>,
    pub state_occupancy: T,
}
impl<T: Scalar> GaussianSufficientStatistics<T> {
    /// Allocate zeroed sufficient statistics for a Gaussian shape.
    /// This validates dimensions without allocating temporary model parameters.
    pub fn new(mixtures: usize, dimensions: usize) -> Result<Self, ModelError> {
        let length = crate::model::parameter_length::<T>(mixtures, dimensions)?;
        Ok(Self {
            dimensions,
            weighted_sums: zeros(length)?,
            weighted_squared_sums: zeros(length)?,
            component_occupancies: zeros(mixtures)?,
            state_occupancy: T::ZERO,
        })
    }
    pub fn from_gaussian(distribution: &GaussianMixtureParameters<T>) -> Result<Self, ModelError> {
        distribution.validate()?;
        Self::new(distribution.weights.len(), distribution.dimensions)
    }
    pub fn clear(&mut self) {
        self.weighted_sums.fill(T::ZERO);
        self.weighted_squared_sums.fill(T::ZERO);
        self.component_occupancies.fill(T::ZERO);
        self.state_occupancy = T::ZERO;
    }
    pub(crate) fn validate_for(
        &self,
        distribution: &GaussianMixtureParameters<T>,
    ) -> Result<(), ModelError> {
        distribution.validate()?;
        if self.dimensions != distribution.dimensions
            || self.weighted_sums.len() != distribution.means.len()
            || self.weighted_squared_sums.len() != distribution.means.len()
            || self.component_occupancies.len() != distribution.weights.len()
        {
            return Err(ModelError(
                "Gaussian statistics dimensions differ from the model",
            ));
        }
        if !self.state_occupancy.into().is_finite()
            || self.state_occupancy < T::ZERO
            || self
                .weighted_sums
                .iter()
                .any(|value| !(*value).into().is_finite())
            || self
                .weighted_squared_sums
                .iter()
                .chain(&self.component_occupancies)
                .any(|value| !(*value).into().is_finite() || *value < T::ZERO)
        {
            return Err(ModelError("Gaussian statistics contain invalid values"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StreamSufficientStatistics<T> {
    pub mixtures: Vec<GaussianSufficientStatistics<T>>,
}
impl<T: Scalar> StreamSufficientStatistics<T> {
    pub fn from_stream(stream: &StreamParameters<T>) -> Result<Self, ModelError> {
        stream.validate()?;
        Ok(Self {
            mixtures: stream
                .mixtures
                .iter()
                .map(GaussianSufficientStatistics::from_gaussian)
                .collect::<Result<_, ModelError>>()?,
        })
    }
    pub fn clear(&mut self) {
        for distribution in &mut self.mixtures {
            distribution.clear();
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModelSufficientStatistics<T> {
    pub streams: Vec<StreamSufficientStatistics<T>>,
    pub durations: Vec<DurationSufficientStatistics<T>>,
}
impl<T: Scalar> ModelSufficientStatistics<T> {
    pub fn from_model(model: &ModelParameters<T>) -> Result<Self, ModelError> {
        model.validate()?;
        let streams = model
            .streams
            .iter()
            .map(StreamSufficientStatistics::from_stream)
            .collect::<Result<_, ModelError>>()?;
        Ok(Self {
            streams,
            durations: vec![DurationSufficientStatistics::default(); model.durations.len()],
        })
    }
    pub fn clear(&mut self) {
        for stream in &mut self.streams {
            stream.clear();
        }
        self.durations.fill(DurationSufficientStatistics::default());
    }
    pub(crate) fn validate_for(&self, model: &ModelParameters<T>) -> Result<(), ModelError> {
        if self.streams.len() != model.streams.len()
            || self.durations.len() != model.durations.len()
        {
            return Err(ModelError(
                "statistics stream or duration counts differ from the model",
            ));
        }
        for (statistics, stream) in self.streams.iter().zip(&model.streams) {
            if statistics.mixtures.len() != stream.mixtures.len() {
                return Err(ModelError(
                    "statistics emission counts differ from the model",
                ));
            }
            for (statistics, distribution) in statistics.mixtures.iter().zip(&stream.mixtures) {
                statistics.validate_for(distribution)?;
            }
        }
        for duration in &self.durations {
            duration.validate()?;
        }
        Ok(())
    }
}

fn zeros<T: Scalar>(length: usize) -> Result<Vec<T>, ModelError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(length)
        .map_err(|_| ModelError("statistics allocation failed"))?;
    values.resize(length, T::ZERO);
    Ok(values)
}

/// Original binary32 duration statistics.
pub type DurationStatistics = DurationSufficientStatistics<f32>;
/// Original binary32 Gaussian statistics.
pub type GaussianStatistics = GaussianSufficientStatistics<f32>;
/// Original binary32 stream statistics.
pub type StreamStatistics = StreamSufficientStatistics<f32>;
/// Original binary32 model statistics.
pub type ModelStatistics = ModelSufficientStatistics<f32>;
