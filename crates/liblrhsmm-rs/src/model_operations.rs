//! Parameter transformations corresponding to upstream `model.c`.
use crate::ModelError;
use crate::model::{DurationParameters, GaussianMixtureParameters};
use crate::scalar::Scalar;

impl<T: Scalar> DurationParameters<T> {
    /// Annealed normal log density corresponding to `lrh_duration_prob_lg`.
    /// Excluded durations remain excluded at temperature zero.
    pub fn annealed_log_probability(
        &self,
        frames: usize,
        weight: T,
        temperature: T,
    ) -> Result<T, ModelError> {
        if frames > i32::MAX as usize || !temperature.into().is_finite() || temperature < T::ZERO {
            return Err(ModelError(
                "duration must fit i32 and temperature must be finite and nonnegative",
            ));
        }
        let density = self.log_probability(T::from_f64(frames as f64), weight)?;
        if temperature == T::ZERO {
            if self.minimum > 0 && frames < self.minimum as usize {
                return Ok(T::from_f64(crate::numerics::LOG_ZERO_F64));
            }
            return Ok(T::ZERO);
        }
        Ok(density * temperature)
    }

    /// Annealed normal density using the upstream exponential approximation.
    pub fn probability(&self, frames: usize, weight: T, temperature: T) -> Result<f64, ModelError> {
        Ok(f64::from(crate::probability::compatibility::exp(
            self.annealed_log_probability(frames, weight, temperature)?
                .into() as f32,
        )))
    }

    /// Evaluate the weighted normal log density, honoring the minimum duration.
    /// The maximum is a search constraint, as in the upstream implementation.
    pub fn log_probability(&self, frames: T, weight: T) -> Result<T, ModelError> {
        if !self.mean.into().is_finite()
            || !self.variance.into().is_finite()
            || self.variance <= T::ZERO
        {
            return Err(ModelError(
                "duration requires a finite mean and positive variance",
            ));
        }
        if !frames.into().is_finite() || !weight.into().is_finite() {
            return Err(ModelError("duration and weight must be finite"));
        }
        if self.minimum > 0 && frames < T::from_f64(f64::from(self.minimum)) {
            return Ok(T::from_f64(crate::numerics::LOG_ZERO_F64));
        }
        let normalizer =
            T::from_f64(-0.5 * (self.variance.into() * 2.0 * std::f64::consts::PI).ln());
        let difference = frames - self.mean;
        let quadratic = difference * difference / self.variance;
        Ok(T::from_f64(normalizer.into() - quadratic.into() / 2.0) * weight)
    }
}

impl<T: Scalar> GaussianMixtureParameters<T> {
    /// Join single-component distributions along their feature dimensions.
    /// Multi-component inputs are rejected, matching the upstream restriction.
    pub fn combine(distributions: &[Self]) -> Result<Self, ModelError> {
        let mut dimensions = 0usize;
        for distribution in distributions {
            distribution.validate()?;
            if distribution.weights.len() != 1 {
                return Err(ModelError(
                    "combination requires single-component distributions",
                ));
            }
            dimensions = dimensions
                .checked_add(distribution.dimensions)
                .ok_or(ModelError("combined dimensions overflow"))?;
        }
        let mut combined = Self::new(1, dimensions)?;
        let mut offset = 0;
        for distribution in distributions {
            let end = offset + distribution.dimensions;
            combined.means[offset..end].copy_from_slice(&distribution.means);
            combined.variances[offset..end].copy_from_slice(&distribution.variances);
            combined.variance_floors[offset..end].copy_from_slice(&distribution.variance_floors);
            offset = end;
        }
        Ok(combined)
    }

    /// Represent independent duration distributions as one diagonal Gaussian.
    pub fn from_durations(durations: &[DurationParameters<T>]) -> Result<Self, ModelError> {
        let mut combined = Self::new(1, durations.len())?;
        for (index, duration) in durations.iter().enumerate() {
            combined.means[index] = duration.mean;
            combined.variances[index] = duration.variance;
        }
        Ok(combined)
    }

    /// Repeatedly split the highest-weight component until the requested count.
    /// Means move by `displacement` standard deviations in each dimension.
    pub fn increase_mixtures(&mut self, count: usize, displacement: T) -> Result<(), ModelError> {
        self.validate()?;
        if count <= self.weights.len() {
            return Ok(());
        }
        if count > i32::MAX as usize || !displacement.into().is_finite() {
            return Err(ModelError("invalid mixture count or displacement"));
        }
        if self
            .weights
            .iter()
            .any(|weight| !(*weight).into().is_finite() || *weight < T::ZERO)
            || self.means.iter().any(|mean| !(*mean).into().is_finite())
            || self
                .variances
                .iter()
                .any(|variance| !(*variance).into().is_finite() || *variance < T::ZERO)
        {
            return Err(ModelError(
                "splitting requires finite parameters and nonnegative weights and variances",
            ));
        }
        let length = count
            .checked_mul(self.dimensions)
            .ok_or(ModelError("dimensions overflow"))?;
        self.weights
            .try_reserve(count - self.weights.len())
            .map_err(|_| ModelError("mixture allocation failed"))?;
        for parameters in [
            &mut self.means,
            &mut self.variances,
            &mut self.variance_floors,
        ] {
            parameters
                .try_reserve(length - parameters.len())
                .map_err(|_| ModelError("parameter allocation failed"))?;
        }
        while self.weights.len() < count {
            // Strict comparison retains the earliest component on ties.
            let mut source = 0;
            for index in 1..self.weights.len() {
                if self.weights[index] > self.weights[source] {
                    source = index;
                }
            }
            let half_weight = self.weights[source] * T::from_f64(0.5);
            self.weights[source] = half_weight;
            self.weights.push(half_weight);
            for dimension in 0..self.dimensions {
                let index = source * self.dimensions + dimension;
                let mean = self.means[index].into();
                let shift = self.variances[index].into().sqrt() * displacement.into();
                self.means.push(T::from_f64(mean + shift));
                self.means[index] = T::from_f64(mean - shift);
                self.variances.push(self.variances[index]);
                self.variance_floors.push(self.variance_floors[index]);
            }
        }
        Ok(())
    }
}
