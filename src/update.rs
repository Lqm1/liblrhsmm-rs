//! Maximization steps corresponding to upstream `lrh_*_update`.
use crate::ModelError;
use crate::model::{
    DurationParameters, GaussianMixtureParameters, ModelParameters, StreamParameters,
};
use crate::scalar::Scalar;
use crate::statistics::{
    DurationSufficientStatistics, GaussianSufficientStatistics, ModelSufficientStatistics,
    StreamSufficientStatistics,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurationMode {
    Normal,
    Geometric,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UpdateSettings<T> {
    pub duration_mode: DurationMode,
    pub duration_variance_floor: T,
    pub emission_variance_floor: T,
}
/// Original binary32 maximization settings.
pub type UpdateOptions = UpdateSettings<f32>;
impl<T: Scalar> Default for UpdateSettings<T> {
    fn default() -> Self {
        Self {
            duration_mode: DurationMode::Normal,
            duration_variance_floor: T::from_f64(0.05),
            emission_variance_floor: T::from_f64(0.01),
        }
    }
}
impl<T: Scalar> UpdateSettings<T> {
    fn validate(self) -> Result<(), ModelError> {
        if [self.duration_variance_floor, self.emission_variance_floor]
            .iter()
            .any(|value| !(*value).into().is_finite() || *value < T::ZERO)
        {
            return Err(ModelError("variance floors must be finite and nonnegative"));
        }
        Ok(())
    }
}

impl<T: Scalar> DurationParameters<T> {
    /// Return whether sufficient occupancy was available to update the model.
    pub fn update(
        &mut self,
        statistics: &DurationSufficientStatistics<T>,
        options: UpdateSettings<T>,
    ) -> Result<bool, ModelError> {
        statistics.validate()?;
        options.validate()?;
        if !self.variance_floor.into().is_finite() || self.variance_floor < T::ZERO {
            return Err(ModelError(
                "duration variance floor must be finite and nonnegative",
            ));
        }
        if statistics.occupancy.into() < 1e-7 {
            return Ok(false);
        }
        let sample_mean = statistics.value_sum / statistics.occupancy;
        let (mean, variance) = match options.duration_mode {
            DurationMode::Normal => {
                let mean = if self.fixed_mean >= 0 {
                    T::from_f64(f64::from(self.fixed_mean))
                } else {
                    sample_mean
                };
                let second_moment = statistics.squared_sum / statistics.occupancy;
                let variance = if self.fixed_mean >= 0 {
                    // E[(X - fixed)^2], rather than E[X^2] - fixed^2.
                    second_moment - T::from_f64(2.0) * mean * sample_mean + mean * mean
                } else {
                    second_moment - mean * mean
                };
                (mean, variance)
            }
            DurationMode::Geometric => {
                if sample_mean <= T::ZERO {
                    return Err(ModelError(
                        "geometric update requires positive transition occupancy",
                    ));
                }
                let mean = T::from_f64((1.0 / sample_mean.into()).max(1.0));
                (mean, mean)
            }
        };
        if !mean.into().is_finite() || !variance.into().is_finite() {
            return Err(ModelError("duration update overflows"));
        }
        self.mean = mean;
        self.variance = T::from_f64(
            variance
                .into()
                .max(options.duration_variance_floor.into())
                .max(self.variance_floor.into()),
        );
        Ok(true)
    }
}

impl<T: Scalar> GaussianMixtureParameters<T> {
    pub fn update(
        &mut self,
        statistics: &GaussianSufficientStatistics<T>,
        options: UpdateSettings<T>,
    ) -> Result<bool, ModelError> {
        statistics.validate_for(self)?;
        options.validate()?;
        if self
            .variance_floors
            .iter()
            .any(|value| !(*value).into().is_finite() || *value < T::ZERO)
        {
            return Err(ModelError(
                "emission variance floors must be finite and nonnegative",
            ));
        }
        if statistics.state_occupancy.into() < 1e-7 {
            return Ok(false);
        }
        // Build first so a failed update does not leave half-written parameters.
        let mut updated = self.clone();
        for (component, &occupancy) in statistics.component_occupancies.iter().enumerate() {
            if occupancy.into() < 1e-7 {
                continue;
            }
            let weight = occupancy / statistics.state_occupancy;
            if !weight.into().is_finite() {
                return Err(ModelError("mixture weight update overflows"));
            }
            updated.weights[component] = weight;
            for dimension in 0..self.dimensions {
                let index = component * self.dimensions + dimension;
                let mean = statistics.weighted_sums[index] / occupancy;
                let variance = statistics.weighted_squared_sums[index] / occupancy - mean * mean;
                if !mean.into().is_finite() || !variance.into().is_finite() {
                    return Err(ModelError("Gaussian update overflows"));
                }
                updated.means[index] = mean;
                updated.variances[index] = T::from_f64(
                    variance
                        .into()
                        .max(options.emission_variance_floor.into())
                        .max(self.variance_floors[index].into()),
                );
            }
        }
        *self = updated;
        Ok(true)
    }
}

impl<T: Scalar> StreamParameters<T> {
    pub fn update(
        &mut self,
        statistics: &StreamSufficientStatistics<T>,
        options: UpdateSettings<T>,
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

impl<T: Scalar> ModelParameters<T> {
    pub fn update(
        &mut self,
        statistics: &ModelSufficientStatistics<T>,
        options: UpdateSettings<T>,
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
