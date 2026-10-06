//! Owned sufficient statistics for initialization and expectation maximization.
use crate::{GaussianMixture, Model, ModelError, Stream};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct DurationStatistics {
    pub value_sum: f32,
    pub squared_sum: f32,
    pub occupancy: f32,
}
impl DurationStatistics {
    pub fn clear(&mut self) {
        *self = Self::default();
    }
    pub(crate) fn validate(&self) -> Result<(), ModelError> {
        if [self.value_sum, self.squared_sum, self.occupancy]
            .iter()
            .any(|value| !value.is_finite() || *value < 0.0)
        {
            return Err(ModelError(
                "duration statistics must be finite and nonnegative",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GaussianStatistics {
    pub dimensions: usize,
    pub weighted_sums: Vec<f32>,
    pub weighted_squared_sums: Vec<f32>,
    pub component_occupancies: Vec<f32>,
    pub state_occupancy: f32,
}
impl GaussianStatistics {
    pub fn from_gaussian(distribution: &GaussianMixture) -> Result<Self, ModelError> {
        distribution.validate()?;
        Ok(Self {
            dimensions: distribution.dimensions,
            weighted_sums: zeros(distribution.means.len())?,
            weighted_squared_sums: zeros(distribution.means.len())?,
            component_occupancies: zeros(distribution.weights.len())?,
            state_occupancy: 0.0,
        })
    }
    pub fn clear(&mut self) {
        self.weighted_sums.fill(0.0);
        self.weighted_squared_sums.fill(0.0);
        self.component_occupancies.fill(0.0);
        self.state_occupancy = 0.0;
    }
    pub(crate) fn validate_for(&self, distribution: &GaussianMixture) -> Result<(), ModelError> {
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
        if !self.state_occupancy.is_finite()
            || self.state_occupancy < 0.0
            || self.weighted_sums.iter().any(|value| !value.is_finite())
            || self
                .weighted_squared_sums
                .iter()
                .chain(&self.component_occupancies)
                .any(|value| !value.is_finite() || *value < 0.0)
        {
            return Err(ModelError("Gaussian statistics contain invalid values"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StreamStatistics {
    pub mixtures: Vec<GaussianStatistics>,
}
impl StreamStatistics {
    pub fn from_stream(stream: &Stream) -> Result<Self, ModelError> {
        stream.validate()?;
        Ok(Self {
            mixtures: stream
                .mixtures
                .iter()
                .map(GaussianStatistics::from_gaussian)
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
pub struct ModelStatistics {
    pub streams: Vec<StreamStatistics>,
    pub durations: Vec<DurationStatistics>,
}
impl ModelStatistics {
    pub fn from_model(model: &Model) -> Result<Self, ModelError> {
        model.validate()?;
        let streams = model
            .streams
            .iter()
            .map(StreamStatistics::from_stream)
            .collect::<Result<_, ModelError>>()?;
        Ok(Self {
            streams,
            durations: vec![DurationStatistics::default(); model.durations.len()],
        })
    }
    pub fn clear(&mut self) {
        for stream in &mut self.streams {
            stream.clear();
        }
        self.durations.fill(DurationStatistics::default());
    }
    pub(crate) fn validate_for(&self, model: &Model) -> Result<(), ModelError> {
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

fn zeros(length: usize) -> Result<Vec<f32>, ModelError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(length)
        .map_err(|_| ModelError("statistics allocation failed"))?;
    values.resize(length, 0.0);
    Ok(values)
}
