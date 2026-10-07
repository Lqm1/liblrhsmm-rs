//! Reusable whole-model preparation without mutable global cache state.
use crate::ModelError;
use crate::data::{ObservationData, SegmentationData};
use crate::emission::{EmissionDensityTable, EmissionPruningOptions, LogEmissionTable};
use crate::model::{DurationParameters, ModelParameters};
use crate::probability::PreparedGaussianParameters;
use crate::scalar::Scalar;

/// Inputs that affect the prepared duration lookup table.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PreparationSettings<T> {
    pub duration_cache_length: usize,
    pub duration_weight: T,
    pub temperature: T,
}
/// Original binary32 preparation options API.
pub type PreparationOptions = PreparationSettings<f32>;

impl<T: Scalar> Default for PreparationSettings<T> {
    fn default() -> Self {
        Self {
            duration_cache_length: 300,
            duration_weight: T::ONE,
            temperature: T::ONE,
        }
    }
}

impl<T: Scalar> PreparationSettings<T> {
    fn validate(self) -> Result<(), ModelError> {
        if self.duration_cache_length > i32::MAX as usize
            || !self.duration_weight.into().is_finite()
            || self.duration_weight < T::ZERO
            || !self.temperature.into().is_finite()
            || self.temperature < T::ZERO
        {
            return Err(ModelError("invalid duration preparation options"));
        }
        Ok(())
    }
}

/// Borrowed duration parameters and an annealed lookup table.
#[derive(Debug)]
pub struct PreparedDurationParameters<'a, T> {
    distribution: &'a DurationParameters<T>,
    normalizer: T,
    options: PreparationSettings<T>,
    cached: Vec<T>,
}
/// Original binary32 prepared duration API.
pub type PreparedDuration<'a> = PreparedDurationParameters<'a, f32>;

impl<T: Scalar> DurationParameters<T> {
    pub fn prepare(
        &self,
        options: PreparationSettings<T>,
    ) -> Result<PreparedDurationParameters<'_, T>, ModelError> {
        options.validate()?;
        // Validate the same initialized parameters as ordinary density evaluation.
        self.log_probability(T::ZERO, options.duration_weight)?;
        let normalizer =
            T::from_f64(-0.5 * (self.variance.into() * 2.0 * std::f64::consts::PI).ln());
        let mut prepared = PreparedDurationParameters {
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

impl<T: Scalar> PreparedDurationParameters<'_, T> {
    pub fn options(&self) -> PreparationSettings<T> {
        self.options
    }

    pub fn cached_log_probabilities(&self) -> &[T] {
        &self.cached
    }

    fn evaluate(&self, frames: usize) -> T {
        let below_minimum = self.distribution.minimum > 0
            && if self.options.temperature == T::ZERO {
                frames < self.distribution.minimum as usize
            } else {
                T::from_f64(frames as f64) < T::from_f64(f64::from(self.distribution.minimum))
            };
        if below_minimum {
            return if self.options.temperature == T::ZERO {
                T::from_f64(crate::numerics::LOG_ZERO_F64)
            } else {
                T::from_f64(crate::numerics::LOG_ZERO_F64) * self.options.temperature
            };
        }
        if self.options.temperature == T::ZERO {
            return T::ZERO;
        }
        let difference = T::from_f64(frames as f64) - self.distribution.mean;
        let quadratic = difference * difference / self.distribution.variance;
        let density = T::from_f64(self.normalizer.into() - quadratic.into() / 2.0);
        density * self.options.duration_weight * self.options.temperature
    }

    /// Use the table within its range, and the cached normalizer beyond it.
    pub fn log_probability(&self, frames: usize) -> Result<T, ModelError> {
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
            self.log_probability(frames)?.into() as f32,
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
///
/// The same ownership guarantee applies to binary64 parameters:
///
/// ```compile_fail
/// use liblrhsmm_rs::model::{DurationParameters, ModelParameters};
/// use liblrhsmm_rs::preparation::PreparationSettings;
/// let mut model = ModelParameters::<f64>::default();
/// let prepared = model.prepare(PreparationSettings::default()).unwrap();
/// model.durations.push(DurationParameters::default());
/// drop(prepared);
/// ```
#[derive(Debug)]
pub struct PreparedModelParameters<'a, T> {
    pub(crate) model: &'a ModelParameters<T>,
    pub(crate) streams: Vec<Vec<PreparedGaussianParameters<'a, T>>>,
    durations: Vec<Option<PreparedDurationParameters<'a, T>>>,
    options: PreparationSettings<T>,
}
/// Original binary32 prepared model API.
pub type PreparedModel<'a> = PreparedModelParameters<'a, f32>;

impl<T: Scalar> ModelParameters<T> {
    /// Prepare every emission normalizer and duration cache, as in `lrh_model_precompute`.
    pub fn prepare(
        &self,
        options: PreparationSettings<T>,
    ) -> Result<PreparedModelParameters<'_, T>, ModelError> {
        self.prepare_duration_states(options, 0..self.durations.len())
    }

    /// Prepare emissions and only the requested duration states. Unreferenced
    /// uninitialized durations remain unchanged and need no density evaluation.
    /// An empty selection supports geometric HMM inference. Duplicate indices
    /// are prepared once; invalid indices or selected distributions are errors.
    pub fn prepare_selected_durations(
        &self,
        options: PreparationSettings<T>,
        states: &[usize],
    ) -> Result<PreparedModelParameters<'_, T>, ModelError> {
        self.prepare_duration_states(options, states.iter().copied())
    }

    fn prepare_duration_states(
        &self,
        options: PreparationSettings<T>,
        states: impl IntoIterator<Item = usize>,
    ) -> Result<PreparedModelParameters<'_, T>, ModelError> {
        options.validate()?;
        self.validate()?;
        let streams = self.prepare_emissions()?;
        let mut durations = Vec::new();
        durations
            .try_reserve_exact(self.durations.len())
            .map_err(|_| ModelError("prepared duration list allocation failed"))?;
        durations.resize_with(self.durations.len(), || None);
        for state in states {
            let target = durations
                .get_mut(state)
                .ok_or(ModelError("selected duration state is out of range"))?;
            if target.is_none() {
                *target = Some(self.durations[state].prepare(options)?);
            }
        }
        Ok(PreparedModelParameters {
            model: self,
            streams,
            durations,
            options,
        })
    }

    pub(crate) fn prepare_emissions(
        &self,
    ) -> Result<Vec<Vec<PreparedGaussianParameters<'_, T>>>, ModelError> {
        let mut streams = Vec::new();
        streams
            .try_reserve_exact(self.streams.len())
            .map_err(|_| ModelError("prepared stream list allocation failed"))?;
        for stream in &self.streams {
            if !stream.weight.into().is_finite() || stream.weight < T::ZERO {
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

impl<'a, T: Scalar> PreparedModelParameters<'a, T> {
    pub fn model(&self) -> &'a ModelParameters<T> {
        self.model
    }

    pub fn options(&self) -> PreparationSettings<T> {
        self.options
    }

    pub fn duration(&self, state: usize) -> Option<&PreparedDurationParameters<'a, T>> {
        self.durations.get(state)?.as_ref()
    }

    pub fn emission(
        &self,
        stream: usize,
        state: usize,
    ) -> Option<&PreparedGaussianParameters<'a, T>> {
        self.streams.get(stream)?.get(state)
    }

    /// Reuse all emission normalizers for another sample or pruning setting.
    pub fn emission_log_probabilities(
        &self,
        observation: &ObservationData<T>,
        segmentation: &SegmentationData<T>,
        temperature: T,
        pruning: EmissionPruningOptions<T>,
    ) -> Result<LogEmissionTable<T>, ModelError> {
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
        observation: &ObservationData<T>,
        segmentation: &SegmentationData<T>,
        temperature: T,
        pruning: EmissionPruningOptions<T>,
    ) -> Result<EmissionDensityTable<T>, ModelError> {
        Ok(self
            .emission_log_probabilities(observation, segmentation, temperature, pruning)?
            .probabilities())
    }
}
