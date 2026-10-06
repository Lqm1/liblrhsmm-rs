//! Maximization steps corresponding to upstream `lrh_*_update`.
use crate::{
    Duration, DurationStatistics, GaussianMixture, GaussianStatistics, Model, ModelError,
    ModelStatistics, Stream, StreamStatistics,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurationMode {
    Normal,
    Geometric,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UpdateOptions {
    pub duration_mode: DurationMode,
    pub duration_variance_floor: f32,
    pub emission_variance_floor: f32,
}
impl Default for UpdateOptions {
    fn default() -> Self {
        Self {
            duration_mode: DurationMode::Normal,
            duration_variance_floor: 0.05,
            emission_variance_floor: 0.01,
        }
    }
}
impl UpdateOptions {
    fn validate(self) -> Result<(), ModelError> {
        if [self.duration_variance_floor, self.emission_variance_floor]
            .iter()
            .any(|value| !value.is_finite() || *value < 0.0)
        {
            return Err(ModelError("variance floors must be finite and nonnegative"));
        }
        Ok(())
    }
}

impl Duration {
    /// Return whether sufficient occupancy was available to update the model.
    pub fn update(
        &mut self,
        statistics: &DurationStatistics,
        options: UpdateOptions,
    ) -> Result<bool, ModelError> {
        statistics.validate()?;
        options.validate()?;
        if !self.variance_floor.is_finite() || self.variance_floor < 0.0 {
            return Err(ModelError(
                "duration variance floor must be finite and nonnegative",
            ));
        }
        if f64::from(statistics.occupancy) < 1e-7 {
            return Ok(false);
        }
        let sample_mean = statistics.value_sum / statistics.occupancy;
        let (mean, variance) = match options.duration_mode {
            DurationMode::Normal => {
                let mean = if self.fixed_mean >= 0 {
                    self.fixed_mean as f32
                } else {
                    sample_mean
                };
                let second_moment = statistics.squared_sum / statistics.occupancy;
                let variance = if self.fixed_mean >= 0 {
                    // E[(X - fixed)^2], rather than E[X^2] - fixed^2.
                    second_moment - 2.0 * mean * sample_mean + mean * mean
                } else {
                    second_moment - mean * mean
                };
                (mean, variance)
            }
            DurationMode::Geometric => {
                if sample_mean <= 0.0 {
                    return Err(ModelError(
                        "geometric update requires positive transition occupancy",
                    ));
                }
                let mean = (1.0 / f64::from(sample_mean)).max(1.0) as f32;
                (mean, mean)
            }
        };
        if !mean.is_finite() || !variance.is_finite() {
            return Err(ModelError("duration update overflows"));
        }
        self.mean = mean;
        self.variance = variance
            .max(options.duration_variance_floor)
            .max(self.variance_floor);
        Ok(true)
    }
}

impl GaussianMixture {
    pub fn update(
        &mut self,
        statistics: &GaussianStatistics,
        options: UpdateOptions,
    ) -> Result<bool, ModelError> {
        statistics.validate_for(self)?;
        options.validate()?;
        if self
            .variance_floors
            .iter()
            .any(|value| !value.is_finite() || *value < 0.0)
        {
            return Err(ModelError(
                "emission variance floors must be finite and nonnegative",
            ));
        }
        if f64::from(statistics.state_occupancy) < 1e-7 {
            return Ok(false);
        }
        // Build first so a failed update does not leave half-written parameters.
        let mut updated = self.clone();
        for (component, &occupancy) in statistics.component_occupancies.iter().enumerate() {
            if f64::from(occupancy) < 1e-7 {
                continue;
            }
            let weight = occupancy / statistics.state_occupancy;
            if !weight.is_finite() {
                return Err(ModelError("mixture weight update overflows"));
            }
            updated.weights[component] = weight;
            for dimension in 0..self.dimensions {
                let index = component * self.dimensions + dimension;
                let mean = statistics.weighted_sums[index] / occupancy;
                let variance = statistics.weighted_squared_sums[index] / occupancy - mean * mean;
                if !mean.is_finite() || !variance.is_finite() {
                    return Err(ModelError("Gaussian update overflows"));
                }
                updated.means[index] = mean;
                updated.variances[index] = variance
                    .max(options.emission_variance_floor)
                    .max(self.variance_floors[index]);
            }
        }
        *self = updated;
        Ok(true)
    }
}

impl Stream {
    pub fn update(
        &mut self,
        statistics: &StreamStatistics,
        options: UpdateOptions,
    ) -> Result<(), ModelError> {
        if self.mixtures.len() != statistics.mixtures.len() {
            return Err(ModelError("stream statistics emission count differs"));
        }
        let mut updated = self.clone();
        for (distribution, statistics) in updated.mixtures.iter_mut().zip(&statistics.mixtures) {
            distribution.update(statistics, options)?;
        }
        *self = updated;
        Ok(())
    }
}

impl Model {
    pub fn update(
        &mut self,
        statistics: &ModelStatistics,
        options: UpdateOptions,
    ) -> Result<(), ModelError> {
        statistics.validate_for(self)?;
        options.validate()?;
        let mut updated = self.clone();
        for (stream, statistics) in updated.streams.iter_mut().zip(&statistics.streams) {
            stream.update(statistics, options)?;
        }
        for (duration, statistics) in updated.durations.iter_mut().zip(&statistics.durations) {
            duration.update(statistics, options)?;
        }
        *self = updated;
        Ok(())
    }
}
