//! Observation generation with explicit caller-owned random sources.
use crate::{Model, ModelError, Observation, Segmentation, probability::compatibility};
use rand::{Rng, RngExt, distr::OpenClosed01};

/// Draw a uniform binary32 value in (0, 1], avoiding a zero logarithm.
pub fn uniform_sample<R: Rng + ?Sized>(rng: &mut R) -> f32 {
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
    if !mean.is_finite() || !variance.is_finite() || variance < 0.0 {
        return Err(ModelError(
            "sampling parameters must be finite with nonnegative variance",
        ));
    }
    if !radial.is_finite()
        || radial <= 0.0
        || radial > 1.0
        || !angular.is_finite()
        || !(0.0..=1.0).contains(&angular)
    {
        return Err(ModelError(
            "normal sampling requires radial in (0,1] and angular in [0,1]",
        ));
    }
    let angle = (2.0 * std::f64::consts::PI * f64::from(angular)) as f32;
    let radius = (-2.0 * f64::from(radial).ln() * f64::from(variance)).sqrt();
    let value = (radius * f64::from(compatibility::cos(angle)) + f64::from(mean)) as f32;
    if !value.is_finite() {
        return Err(ModelError("normal sample overflows"));
    }
    Ok(value)
}

/// Select a mixture using the original cumulative-weight and last-index fallback.
/// Weights are not normalized, retaining the upstream convention.
pub fn choose_component(weights: &[f32], uniform: f32) -> Result<usize, ModelError> {
    if weights.is_empty()
        || weights.iter().any(|x| !x.is_finite() || *x < 0.0)
        || !uniform.is_finite()
        || !(0.0..=1.0).contains(&uniform)
    {
        return Err(ModelError(
            "mixture sampling requires weights and a uniform draw in [0,1]",
        ));
    }
    let mut cumulative = 0.0;
    for (index, &weight) in weights.iter().enumerate() {
        cumulative += weight;
        if cumulative > uniform {
            return Ok(index);
        }
    }
    Ok(weights.len() - 1)
}

impl Model {
    /// Generate with a caller-supplied RNG, without requiring OS entropy.
    /// Open-closed draws avoid the upstream Box-Muller `log(0)` case.
    pub fn generate<R: Rng + ?Sized>(
        &self,
        sequence: &mut Segmentation,
        rng: &mut R,
    ) -> Result<Observation, ModelError> {
        self.generate_with_uniform(sequence, || uniform_sample(rng))
    }

    /// Generate with explicit uniform draws for reproducible compatibility tests.
    /// Successful generation writes the sampled right boundaries into `sequence`.
    /// Errors preserve the sequence, but consumed random draws cannot be restored.
    /// Duration constraints and jumps are ignored here, as in upstream generation.
    pub fn generate_with_uniform(
        &self,
        sequence: &mut Segmentation,
        mut uniform: impl FnMut() -> f32,
    ) -> Result<Observation, ModelError> {
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
            if !duration.mean.is_finite()
                || !duration.variance.is_finite()
                || duration.variance < 0.0
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
                        .any(|x| !x.is_finite() || *x < 0.0)
                    || distribution.means.iter().any(|x| !x.is_finite())
                    || distribution
                        .variances
                        .iter()
                        .any(|x| !x.is_finite() || *x < 0.0)
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
            let length = f64::from(gaussian_sample(
                duration.mean,
                duration.variance,
                radial,
                angular,
            )?)
            .max(1.0)
            .round();
            if length > i32::MAX as f64 {
                return Err(ModelError("generated duration exceeds legacy frame limit"));
            }
            frames = frames
                .checked_add(length as usize)
                .filter(|&n| n <= i32::MAX as usize)
                .ok_or(ModelError("generated sequence exceeds legacy frame limit"))?;
            boundaries.push(frames as i32);
        }
        let mut observation = Observation::new(frames, &dimensions)?;
        let mut begin = 0;
        for (state, &end) in boundaries.iter().enumerate() {
            for time in begin..end as usize {
                for (stream, destination) in observation.streams.iter_mut().enumerate() {
                    let distribution = &self.streams[stream].mixtures
                        [sequence.output_states[stream][state] as usize];
                    let component = choose_component(&distribution.weights, uniform())?;
                    for dimension in 0..destination.dimensions {
                        let index = component * destination.dimensions + dimension;
                        let radial = uniform();
                        let angular = uniform();
                        destination.values[time * destination.dimensions + dimension] =
                            gaussian_sample(
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
