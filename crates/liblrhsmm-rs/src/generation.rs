//! Observation generation with explicit caller-owned random sources.
use crate::data::{ObservationData, SegmentationData};
use crate::model::ModelParameters;
use crate::scalar::Scalar;
use crate::{ModelError, probability::compatibility};
use rand::{
    Rng, RngExt,
    distr::{Distribution, OpenClosed01},
};

/// Draw a uniform binary32 value in (0, 1], avoiding a zero logarithm.
pub fn uniform_sample<R: Rng + ?Sized>(rng: &mut R) -> f32 {
    uniform_with_precision(rng)
}

/// Draw directly at the selected floating-point precision in (0, 1].
pub fn uniform_with_precision<T, R: Rng + ?Sized>(rng: &mut R) -> T
where
    OpenClosed01: Distribution<T>,
{
    rng.sample(OpenClosed01)
}

/// Sample a normal value from two uniform draws, using upstream's approximation.
/// `radial` must be in (0, 1]; `angular` may include either endpoint of [0, 1].
pub fn gaussian_sample(
    mean: f32,
    variance: f32,
    radial: f32,
    angular: f32,
) -> Result<f32, ModelError> {
    gaussian_with_precision(mean, variance, radial, angular)
}

/// Source-compatible normal sampling at binary32 or binary64 precision.
/// The cosine kernel remains binary32, as in either C precision build.
pub fn gaussian_with_precision<T: Scalar>(
    mean: T,
    variance: T,
    radial: T,
    angular: T,
) -> Result<T, ModelError> {
    if !mean.into().is_finite() || !variance.into().is_finite() || variance < T::ZERO {
        return Err(ModelError(
            "sampling parameters must be finite with nonnegative variance",
        ));
    }
    if !radial.into().is_finite()
        || radial <= T::ZERO
        || radial > T::ONE
        || !angular.into().is_finite()
        || angular < T::ZERO
        || angular > T::ONE
    {
        return Err(ModelError(
            "normal sampling requires radial in (0,1] and angular in [0,1]",
        ));
    }
    let angle = (2.0 * std::f64::consts::PI * angular.into()) as f32;
    let radius = (-2.0 * radial.into().ln() * variance.into()).sqrt();
    let value = T::from_f64(radius * f64::from(compatibility::cos(angle)) + mean.into());
    if !value.into().is_finite() {
        return Err(ModelError("normal sample overflows"));
    }
    Ok(value)
}

/// Select a mixture using the original cumulative-weight and last-index fallback.
/// Weights are not normalized, retaining the upstream convention.
pub fn choose_component(weights: &[f32], uniform: f32) -> Result<usize, ModelError> {
    choose_with_precision(weights, uniform)
}

/// Select a component without narrowing its cumulative weights or random draw.
pub fn choose_with_precision<T: Scalar>(weights: &[T], uniform: T) -> Result<usize, ModelError> {
    if weights.is_empty()
        || weights
            .iter()
            .any(|x| !(*x).into().is_finite() || *x < T::ZERO)
        || !uniform.into().is_finite()
        || uniform < T::ZERO
        || uniform > T::ONE
    {
        return Err(ModelError(
            "mixture sampling requires weights and a uniform draw in [0,1]",
        ));
    }
    let mut cumulative = T::ZERO;
    for (index, &weight) in weights.iter().enumerate() {
        cumulative += weight;
        if cumulative > uniform {
            return Ok(index);
        }
    }
    Ok(weights.len() - 1)
}

impl<T: Scalar> ModelParameters<T> {
    /// Generate with a caller-supplied RNG, without requiring OS entropy.
    /// Open-closed draws avoid the upstream Box-Muller `log(0)` case.
    pub fn generate<R: Rng + ?Sized>(
        &self,
        sequence: &mut SegmentationData<T>,
        rng: &mut R,
    ) -> Result<ObservationData<T>, ModelError>
    where
        OpenClosed01: Distribution<T>,
    {
        self.generate_with_uniform(sequence, || uniform_with_precision(rng))
    }

    /// Generate with explicit uniform draws for reproducible compatibility tests.
    /// Successful generation writes the sampled right boundaries into `sequence`.
    /// Errors preserve the sequence, but consumed random draws cannot be restored.
    /// Duration constraints and jumps are ignored here, as in upstream generation.
    pub fn generate_with_uniform(
        &self,
        sequence: &mut SegmentationData<T>,
        mut uniform: impl FnMut() -> T,
    ) -> Result<ObservationData<T>, ModelError> {
        self.validate()?;
        sequence.validate()?;
        if sequence.boundaries.is_empty()
            || self.streams.is_empty()
            || self.streams.len() != sequence.output_states.len()
        {
            return Err(ModelError(
                "generation requires nonempty matching stream and state sequences",
            ));
        }
        let mut dimensions = Vec::with_capacity(self.streams.len());
        for stream in &self.streams {
            let first = stream
                .mixtures
                .first()
                .ok_or(ModelError("generation stream has no emissions"))?;
            dimensions.push(first.dimensions);
        }
        for state in 0..sequence.boundaries.len() {
            let duration = usize::try_from(sequence.duration_states[state])
                .ok()
                .and_then(|index| self.durations.get(index))
                .ok_or(ModelError("generation duration state is out of range"))?;
            if !duration.mean.into().is_finite()
                || !duration.variance.into().is_finite()
                || duration.variance < T::ZERO
            {
                return Err(ModelError("generation duration parameters are invalid"));
            }
            for (stream, &dimension) in dimensions.iter().enumerate() {
                let distribution = usize::try_from(sequence.output_states[stream][state])
                    .ok()
                    .and_then(|index| self.streams[stream].mixtures.get(index))
                    .ok_or(ModelError("generation emission state is out of range"))?;
                if distribution.dimensions != dimension
                    || distribution
                        .weights
                        .iter()
                        .any(|x| !(*x).into().is_finite() || *x < T::ZERO)
                    || distribution.means.iter().any(|x| !(*x).into().is_finite())
                    || distribution
                        .variances
                        .iter()
                        .any(|x| !(*x).into().is_finite() || *x < T::ZERO)
                {
                    return Err(ModelError(
                        "generation emission parameters or dimensions are invalid",
                    ));
                }
            }
        }
        let mut boundaries = Vec::with_capacity(sequence.boundaries.len());
        let mut frames = 0usize;
        for &index in &sequence.duration_states {
            let duration = &self.durations[index as usize];
            let radial = uniform();
            let angular = uniform();
            let length: f64 =
                gaussian_with_precision(duration.mean, duration.variance, radial, angular)?.into();
            let length = length.max(1.0).round();
            if length > i32::MAX as f64 {
                return Err(ModelError("generated duration exceeds legacy frame limit"));
            }
            frames = frames
                .checked_add(length as usize)
                .filter(|&n| n <= i32::MAX as usize)
                .ok_or(ModelError("generated sequence exceeds legacy frame limit"))?;
            boundaries.push(frames as i32);
        }
        let mut observation = ObservationData::new(frames, &dimensions)?;
        let mut begin = 0;
        for (state, &end) in boundaries.iter().enumerate() {
            for time in begin..end as usize {
                for (stream, destination) in observation.streams.iter_mut().enumerate() {
                    let distribution = &self.streams[stream].mixtures
                        [sequence.output_states[stream][state] as usize];
                    let component = choose_with_precision(&distribution.weights, uniform())?;
                    for dimension in 0..destination.dimensions {
                        let index = component * destination.dimensions + dimension;
                        let radial = uniform();
                        let angular = uniform();
                        destination.values[time * destination.dimensions + dimension] =
                            gaussian_with_precision(
                                distribution.means[index],
                                distribution.variances[index],
                                radial,
                                angular,
                            )?;
                    }
                }
            }
            begin = end as usize;
        }
        sequence.boundaries = boundaries;
        Ok(observation)
    }
}
