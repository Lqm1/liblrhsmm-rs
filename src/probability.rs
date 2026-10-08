//! Prepared emission distributions compatible with upstream `inference.c`.
use crate::ModelError;
use crate::model::GaussianMixtureParameters;
use crate::scalar::Scalar;

/// Immutable parameters and their cached normalizing constants.
/// The borrow prevents parameter changes while the cache is in use.
#[derive(Debug)]
pub struct PreparedGaussianParameters<'a, T> {
    distribution: &'a GaussianMixtureParameters<T>,
    normalizers: Vec<T>,
}
/// Original binary32 prepared Gaussian API.
pub type PreparedGaussian<'a> = PreparedGaussianParameters<'a, f32>;

/// Owned immutable parameters with matching cached normalizers.
/// Moving or cloning the snapshot does not borrow the editable source.
#[derive(Debug, Clone)]
pub struct OwnedPreparedGaussianParameters<T> {
    distribution: GaussianMixtureParameters<T>,
    normalizers: Vec<T>,
}
/// Binary32 owned prepared Gaussian.
pub type OwnedPreparedGaussian = OwnedPreparedGaussianParameters<f32>;

// Temporary borrowed evaluation view: no self-reference or cache cloning.
struct GaussianEvaluation<'a, T> {
    distribution: &'a GaussianMixtureParameters<T>,
    normalizers: &'a [T],
}

fn gaussian_normalizers<T: Scalar>(
    distribution: &GaussianMixtureParameters<T>,
) -> Result<Vec<T>, ModelError> {
    distribution.validate()?;
    if distribution
        .means
        .iter()
        .any(|value| !(*value).into().is_finite())
        || distribution
            .variances
            .iter()
            .any(|value| !(*value).into().is_finite() || *value <= T::ZERO)
        || distribution
            .weights
            .iter()
            .any(|value| !(*value).into().is_finite() || *value < T::ZERO)
    {
        return Err(ModelError(
            "emissions require finite means, positive variances, and nonnegative finite weights",
        ));
    }
    let normalizers = distribution
        .variances
        .chunks_exact(distribution.dimensions)
        .map(|variances| {
            // C rounds each accumulated double log to the selected FP_TYPE.
            variances.iter().fold(T::ZERO, |sum, variance| {
                T::from_f64(sum.into() - ((*variance).into() * 2.0 * std::f64::consts::PI).ln())
            })
        })
        .collect();
    Ok(normalizers)
}

impl<T: Scalar> GaussianMixtureParameters<T> {
    /// Validate initialized parameters and prepare emission-density evaluation.
    pub fn prepare(&self) -> Result<PreparedGaussianParameters<'_, T>, ModelError> {
        Ok(PreparedGaussianParameters {
            distribution: self,
            normalizers: gaussian_normalizers(self)?,
        })
    }
    /// Consume initialized parameters into an independent immutable snapshot.
    /// Clone first to retain an editable copy of the source parameters.
    pub fn prepare_owned(self) -> Result<OwnedPreparedGaussianParameters<T>, ModelError> {
        let normalizers = gaussian_normalizers(&self)?;
        Ok(OwnedPreparedGaussianParameters {
            distribution: self,
            normalizers,
        })
    }
}

impl<T: Scalar> PreparedGaussianParameters<'_, T> {
    /// Borrow cached component sums before the density's factor of one half.
    pub fn cached_log_normalizers(&self) -> &[T] {
        &self.normalizers
    }
    fn evaluation(&self) -> GaussianEvaluation<'_, T> {
        GaussianEvaluation {
            distribution: self.distribution,
            normalizers: &self.normalizers,
        }
    }
    /// Unweighted component log density without annealing.
    pub fn component_log_probability(
        &self,
        observation: &[T],
        component: usize,
    ) -> Result<T, ModelError> {
        self.evaluation()
            .component_log_probability(observation, component)
    }
    /// Unweighted component density using the original approximation.
    pub fn component_probability(
        &self,
        observation: &[T],
        component: usize,
    ) -> Result<f64, ModelError> {
        self.evaluation()
            .component_probability(observation, component)
    }
    /// Native mixture log density with DAEM temperature.
    pub fn log_probability(&self, observation: &[T], temperature: T) -> Result<T, ModelError> {
        self.evaluation().log_probability(observation, temperature)
    }
    /// Native annealed mixture density.
    pub fn probability(&self, observation: &[T], temperature: T) -> Result<f64, ModelError> {
        self.evaluation().probability(observation, temperature)
    }
}

impl<T: Scalar> OwnedPreparedGaussianParameters<T> {
    /// Borrow cached component sums before the density's factor of one half.
    pub fn cached_log_normalizers(&self) -> &[T] {
        &self.normalizers
    }
    fn evaluation(&self) -> GaussianEvaluation<'_, T> {
        GaussianEvaluation {
            distribution: &self.distribution,
            normalizers: &self.normalizers,
        }
    }
    /// Immutable parameters matched to this cache.
    pub fn parameters(&self) -> &GaussianMixtureParameters<T> {
        &self.distribution
    }
    /// Discard derived values and recover editable parameter ownership.
    pub fn into_parameters(self) -> GaussianMixtureParameters<T> {
        self.distribution
    }
    /// Unweighted component log density without annealing.
    pub fn component_log_probability(
        &self,
        observation: &[T],
        component: usize,
    ) -> Result<T, ModelError> {
        self.evaluation()
            .component_log_probability(observation, component)
    }
    /// Unweighted component density using the original approximation.
    pub fn component_probability(
        &self,
        observation: &[T],
        component: usize,
    ) -> Result<f64, ModelError> {
        self.evaluation()
            .component_probability(observation, component)
    }
    /// Native mixture log density with DAEM temperature.
    pub fn log_probability(&self, observation: &[T], temperature: T) -> Result<T, ModelError> {
        self.evaluation().log_probability(observation, temperature)
    }
    /// Native annealed mixture density.
    pub fn probability(&self, observation: &[T], temperature: T) -> Result<f64, ModelError> {
        self.evaluation().probability(observation, temperature)
    }
}

impl<T: Scalar> GaussianEvaluation<'_, T> {
    fn validate_observation(&self, observation: &[T]) -> Result<(), ModelError> {
        if observation.len() != self.distribution.dimensions
            || observation.iter().any(|value| !(*value).into().is_finite())
        {
            return Err(ModelError(
                "observation must have matching dimensions and finite values",
            ));
        }
        Ok(())
    }

    fn component_log_density(&self, observation: &[T], component: usize) -> T {
        let start = component * self.distribution.dimensions;
        let end = start + self.distribution.dimensions;
        let quadratic = observation
            .iter()
            .zip(&self.distribution.means[start..end])
            .zip(&self.distribution.variances[start..end])
            .fold(T::ZERO, |sum, ((value, mean), variance)| {
                let difference = *value - *mean;
                sum + difference * difference / *variance
            });
        T::from_f64(self.normalizers[component].into() * 0.5 - quadratic.into() * 0.5)
    }

    /// Unweighted log density of one component, without annealing.
    pub fn component_log_probability(
        &self,
        observation: &[T],
        component: usize,
    ) -> Result<T, ModelError> {
        self.validate_observation(observation)?;
        if component >= self.distribution.weights.len() {
            return Err(ModelError("mixture component is out of range"));
        }
        Ok(self.component_log_density(observation, component))
    }

    /// Unweighted component density using the original exponential approximation.
    pub fn component_probability(
        &self,
        observation: &[T],
        component: usize,
    ) -> Result<f64, ModelError> {
        Ok(f64::from(compatibility::exp(
            self.component_log_probability(observation, component)?
                .into() as f32,
        )))
    }

    /// Mixture log density scaled by the original DAEM temperature multiplier.
    /// A single component retains the upstream convention of ignoring its weight.
    pub fn log_probability(&self, observation: &[T], temperature: T) -> Result<T, ModelError> {
        self.validate_observation(observation)?;
        if !temperature.into().is_finite() || temperature < T::ZERO {
            return Err(ModelError("temperature must be finite and nonnegative"));
        }
        if temperature == T::ZERO {
            return Ok(T::ZERO);
        }
        let density = if self.distribution.weights.len() == 1 {
            self.component_log_density(observation, 0)
        } else {
            let probability = self.distribution.weights.iter().enumerate().fold(
                0.0f64,
                |sum, (component, weight)| {
                    sum + f64::from(compatibility::exp(
                        self.component_log_density(observation, component).into() as f32,
                    )) * (*weight).into()
                },
            );
            T::from_f64(f64::from(compatibility::ln(probability as f32)).max(-1e15))
        };
        Ok(density * temperature)
    }

    /// Annealed mixture density using the original exponential approximation.
    pub fn probability(&self, observation: &[T], temperature: T) -> Result<f64, ModelError> {
        Ok(f64::from(compatibility::exp(
            self.log_probability(observation, temperature)?.into() as f32,
        )))
    }
}

// Preserve the upstream binary32 coefficients and operation order. These kernels
// adapt Paul Mineiro's fastapprox; the BSD notice is in LICENSES/fastapprox-BSD.txt.
#[allow(clippy::excessive_precision, clippy::approx_constant)]
pub(crate) mod compatibility {
    pub(crate) fn cos(value: f32) -> f32 {
        let angle = value
            + if value > 1.5707963267948966f32 {
                -4.7123889803846899f32
            } else {
                1.5707963267948966f32
            };
        let sign = angle.to_bits() & 0x8000_0000;
        let p = f32::from_bits(0.20363937680730309f32.to_bits() | sign);
        let r = f32::from_bits(0.015124940802184233f32.to_bits() | sign);
        let s = f32::from_bits((-0.0032225901625579573f32).to_bits() ^ sign);
        let approximation =
            1.2732395447351627f32 * angle - 0.40528473456935109f32 * angle * angle.abs();
        let square = approximation * approximation;
        0.78444488374548933f32 * approximation + square * (p + square * (r + square * s))
    }

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

    pub(crate) fn ln(value: f32) -> f32 {
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
