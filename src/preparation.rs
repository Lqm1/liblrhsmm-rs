//! Reusable whole-model preparation without mutable global cache state.
use crate::{
    Duration, EmissionProbabilities, EmissionPruning, EmissionTable, Model, ModelError,
    Observation, PreparedGaussian, Segmentation,
};

/// Inputs that affect the prepared duration lookup table.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PreparationOptions {
    pub duration_cache_length: usize,
    pub duration_weight: f32,
    pub temperature: f32,
}

impl Default for PreparationOptions {
    fn default() -> Self {
        Self {
            duration_cache_length: 300,
            duration_weight: 1.0,
            temperature: 1.0,
        }
    }
}

impl PreparationOptions {
    fn validate(self) -> Result<(), ModelError> {
        if self.duration_cache_length > i32::MAX as usize
            || !self.duration_weight.is_finite()
            || self.duration_weight < 0.0
            || !self.temperature.is_finite()
            || self.temperature < 0.0
        {
            return Err(ModelError("invalid duration preparation options"));
        }
        Ok(())
    }
}

/// Borrowed duration parameters and an annealed lookup table.
#[derive(Debug)]
pub struct PreparedDuration<'a> {
    distribution: &'a Duration,
    normalizer: f32,
    options: PreparationOptions,
    cached: Vec<f32>,
}

impl Duration {
    pub fn prepare(&self, options: PreparationOptions) -> Result<PreparedDuration<'_>, ModelError> {
        options.validate()?;
        // Validate the same initialized parameters as ordinary density evaluation.
        self.log_probability(0.0, options.duration_weight)?;
        let normalizer =
            (-0.5 * (f64::from(self.variance) * 2.0 * std::f64::consts::PI).ln()) as f32;
        let mut prepared = PreparedDuration {
            distribution: self,
            normalizer,
            options,
            cached: Vec::new(),
        };
        prepared
            .cached
            .try_reserve_exact(options.duration_cache_length)
            .map_err(|_| ModelError("duration cache allocation failed"))?;
        for frames in 0..options.duration_cache_length {
            let value = prepared.evaluate(frames);
            prepared.cached.push(value);
        }
        Ok(prepared)
    }
}

impl PreparedDuration<'_> {
    pub fn options(&self) -> PreparationOptions {
        self.options
    }

    pub fn cached_log_probabilities(&self) -> &[f32] {
        &self.cached
    }

    fn evaluate(&self, frames: usize) -> f32 {
        let below_minimum = self.distribution.minimum > 0
            && if self.options.temperature == 0.0 {
                frames < self.distribution.minimum as usize
            } else {
                (frames as f32) < self.distribution.minimum as f32
            };
        if below_minimum {
            return if self.options.temperature == 0.0 {
                crate::numerics::LOG_ZERO
            } else {
                crate::numerics::LOG_ZERO * self.options.temperature
            };
        }
        if self.options.temperature == 0.0 {
            return 0.0;
        }
        let difference = frames as f32 - self.distribution.mean;
        let quadratic = difference * difference / self.distribution.variance;
        let density = (f64::from(self.normalizer) - f64::from(quadratic) / 2.0) as f32;
        density * self.options.duration_weight * self.options.temperature
    }

    /// Use the table within its range, and the cached normalizer beyond it.
    pub fn log_probability(&self, frames: usize) -> Result<f32, ModelError> {
        if frames > i32::MAX as usize {
            return Err(ModelError("duration exceeds the legacy frame range"));
        }
        Ok(self
            .cached
            .get(frames)
            .copied()
            .unwrap_or_else(|| self.evaluate(frames)))
    }

    pub fn probability(&self, frames: usize) -> Result<f64, ModelError> {
        Ok(f64::from(crate::probability::compatibility::exp(
            self.log_probability(frames)?,
        )))
    }
}

/// A model and its prepared distributions. The immutable borrow prevents stale
/// parameter caches and permits independent preparations at different temperatures.
///
/// Parameters cannot be changed while a preparation is still used:
///
/// ```compile_fail
/// use liblrhsmm_rs::{Duration, Model, PreparationOptions};
/// let mut model = Model::default();
/// let prepared = model.prepare(PreparationOptions::default()).unwrap();
/// model.durations.push(Duration::default());
/// drop(prepared);
/// ```
#[derive(Debug)]
pub struct PreparedModel<'a> {
    pub(crate) model: &'a Model,
    pub(crate) streams: Vec<Vec<PreparedGaussian<'a>>>,
    durations: Vec<PreparedDuration<'a>>,
    options: PreparationOptions,
}

impl Model {
    /// Prepare every emission normalizer and duration cache, as in `lrh_model_precompute`.
    pub fn prepare(&self, options: PreparationOptions) -> Result<PreparedModel<'_>, ModelError> {
        options.validate()?;
        self.validate()?;
        let streams = self.prepare_emissions()?;
        let mut durations = Vec::new();
        durations
            .try_reserve_exact(self.durations.len())
            .map_err(|_| ModelError("prepared duration list allocation failed"))?;
        for duration in &self.durations {
            durations.push(duration.prepare(options)?);
        }
        Ok(PreparedModel {
            model: self,
            streams,
            durations,
            options,
        })
    }

    pub(crate) fn prepare_emissions(&self) -> Result<Vec<Vec<PreparedGaussian<'_>>>, ModelError> {
        let mut streams = Vec::new();
        streams
            .try_reserve_exact(self.streams.len())
            .map_err(|_| ModelError("prepared stream list allocation failed"))?;
        for stream in &self.streams {
            if !stream.weight.is_finite() || stream.weight < 0.0 {
                return Err(ModelError("stream weights must be finite and nonnegative"));
            }
            let mut distributions = Vec::new();
            distributions
                .try_reserve_exact(stream.mixtures.len())
                .map_err(|_| ModelError("prepared emission list allocation failed"))?;
            for distribution in &stream.mixtures {
                distributions.push(distribution.prepare()?);
            }
            streams.push(distributions);
        }
        Ok(streams)
    }
}

impl<'a> PreparedModel<'a> {
    pub fn model(&self) -> &'a Model {
        self.model
    }

    pub fn options(&self) -> PreparationOptions {
        self.options
    }

    pub fn duration(&self, state: usize) -> Option<&PreparedDuration<'a>> {
        self.durations.get(state)
    }

    pub fn emission(&self, stream: usize, state: usize) -> Option<&PreparedGaussian<'a>> {
        self.streams.get(stream)?.get(state)
    }

    /// Reuse all emission normalizers for another sample or pruning setting.
    pub fn emission_log_probabilities(
        &self,
        observation: &Observation,
        segmentation: &Segmentation,
        temperature: f32,
        pruning: EmissionPruning,
    ) -> Result<EmissionTable, ModelError> {
        self.model.sample_prepared_emissions(
            &self.streams,
            observation,
            segmentation,
            temperature,
            pruning,
        )
    }

    pub fn emission_probabilities(
        &self,
        observation: &Observation,
        segmentation: &Segmentation,
        temperature: f32,
        pruning: EmissionPruning,
    ) -> Result<EmissionProbabilities, ModelError> {
        Ok(self
            .emission_log_probabilities(observation, segmentation, temperature, pruning)?
            .probabilities())
    }
}
