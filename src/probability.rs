//! Prepared emission distributions compatible with upstream `inference.c`.
use crate::{GaussianMixture, ModelError};

/// Immutable parameters and their cached normalizing constants.
/// The borrow prevents parameter changes while the cache is in use.
#[derive(Debug)]
pub struct PreparedGaussian<'a> {
    distribution: &'a GaussianMixture,
    normalizers: Vec<f32>,
}

impl GaussianMixture {
    /// Validate initialized parameters and prepare emission-density evaluation.
    pub fn prepare(&self) -> Result<PreparedGaussian<'_>, ModelError> {
        self.validate()?;
        if self.means.iter().any(|value| !value.is_finite())
            || self
                .variances
                .iter()
                .any(|value| !value.is_finite() || *value <= 0.0)
            || self
                .weights
                .iter()
                .any(|value| !value.is_finite() || *value < 0.0)
        {
            return Err(ModelError(
                "emissions require finite means, positive variances, and nonnegative finite weights",
            ));
        }
        let normalizers = self
            .variances
            .chunks_exact(self.dimensions)
            .map(|variances| {
                // C rounds the accumulated double-precision log back to float each time.
                variances.iter().fold(0.0f32, |sum, variance| {
                    (f64::from(sum) - (f64::from(*variance) * 2.0 * std::f64::consts::PI).ln())
                        as f32
                })
            })
            .collect();
        Ok(PreparedGaussian {
            distribution: self,
            normalizers,
        })
    }
}

impl PreparedGaussian<'_> {
    fn validate_observation(&self, observation: &[f32]) -> Result<(), ModelError> {
        if observation.len() != self.distribution.dimensions
            || observation.iter().any(|value| !value.is_finite())
        {
            return Err(ModelError(
                "observation must have matching dimensions and finite values",
            ));
        }
        Ok(())
    }

    fn component_log_density(&self, observation: &[f32], component: usize) -> f32 {
        let start = component * self.distribution.dimensions;
        let end = start + self.distribution.dimensions;
        let quadratic = observation
            .iter()
            .zip(&self.distribution.means[start..end])
            .zip(&self.distribution.variances[start..end])
            .fold(0.0f32, |sum, ((value, mean), variance)| {
                let difference = value - mean;
                sum + difference * difference / variance
            });
        (f64::from(self.normalizers[component]) * 0.5 - f64::from(quadratic) * 0.5) as f32
    }

    /// Unweighted log density of one component, without annealing.
    pub fn component_log_probability(
        &self,
        observation: &[f32],
        component: usize,
    ) -> Result<f32, ModelError> {
        self.validate_observation(observation)?;
        if component >= self.distribution.weights.len() {
            return Err(ModelError("mixture component is out of range"));
        }
        Ok(self.component_log_density(observation, component))
    }

    /// Unweighted component density using the original exponential approximation.
    pub fn component_probability(
        &self,
        observation: &[f32],
        component: usize,
    ) -> Result<f64, ModelError> {
        Ok(f64::from(compatibility::exp(
            self.component_log_probability(observation, component)?,
        )))
    }

    /// Mixture log density scaled by the original DAEM temperature multiplier.
    /// A single component retains the upstream convention of ignoring its weight.
    pub fn log_probability(
        &self,
        observation: &[f32],
        temperature: f32,
    ) -> Result<f32, ModelError> {
        self.validate_observation(observation)?;
        if !temperature.is_finite() || temperature < 0.0 {
            return Err(ModelError("temperature must be finite and nonnegative"));
        }
        if temperature == 0.0 {
            return Ok(0.0);
        }
        let density = if self.distribution.weights.len() == 1 {
            self.component_log_density(observation, 0)
        } else {
            let probability = self.distribution.weights.iter().enumerate().fold(
                0.0f64,
                |sum, (component, weight)| {
                    sum + f64::from(compatibility::exp(
                        self.component_log_density(observation, component),
                    )) * f64::from(*weight)
                },
            );
            compatibility::ln(probability as f32).max(-1e15)
        };
        Ok(density * temperature)
    }

    /// Annealed mixture density using the original exponential approximation.
    pub fn probability(&self, observation: &[f32], temperature: f32) -> Result<f64, ModelError> {
        Ok(f64::from(compatibility::exp(
            self.log_probability(observation, temperature)?,
        )))
    }
}

// Preserve the upstream binary32 coefficients and operation order. These kernels
// adapt Paul Mineiro's fastapprox; the BSD notice is in LICENSES/fastapprox-BSD.txt.
#[allow(clippy::excessive_precision, clippy::approx_constant)]
pub(crate) mod compatibility {
    pub(crate) fn exp(value: f32) -> f32 {
        if value.is_nan() {
            return f32::NAN;
        }
        let exponent = 1.442695040f32 * value;
        // Upstream conversion to u32 is undefined above the representable range.
        if exponent >= 128.0 {
            return f32::INFINITY;
        }
        let offset = if exponent < 0.0 { 1.0 } else { 0.0 };
        let clipped = exponent.max(-126.0);
        let fraction = clipped - (clipped as i32) as f32 + offset;
        let bits = 8_388_608.0
            * (clipped + 121.2740575f32 + 27.7280233f32 / (4.84252568f32 - fraction)
                - 1.49012907f32 * fraction);
        f32::from_bits(bits as u32)
    }

    pub(super) fn ln(value: f32) -> f32 {
        let bits = value.to_bits();
        let mantissa = f32::from_bits((bits & 0x007f_ffff) | 0x3f00_0000);
        let exponent = bits as f32 * 1.1920928955078125e-7f32;
        0.69314718f32
            * (exponent
                - 124.22551499f32
                - 1.498030302f32 * mantissa
                - 1.72587999f32 / (0.3520887068f32 + mantissa))
    }
}
