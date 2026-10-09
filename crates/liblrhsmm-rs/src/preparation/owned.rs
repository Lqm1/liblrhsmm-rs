//! Independent stream/model snapshots with borrowed cache views for evaluation.
use super::{DurationPreparation, OwnedPreparedDurationParameters, PreparationSettings};
use crate::ModelError;
use crate::data::{ObservationData, SegmentationData};
use crate::emission::{EmissionDensityTable, EmissionPruningOptions, LogEmissionTable};
use crate::model::{DurationParameters, ModelParameters, StreamParameters};
use crate::probability::{PreparedGaussianView, gaussian_normalizers};
use crate::scalar::Scalar;

#[derive(Debug, Clone)]
struct StreamPreparation<T> {
    normalizers: Vec<Vec<T>>,
}

impl<T: Scalar> StreamPreparation<T> {
    fn new(stream: &StreamParameters<T>) -> Result<Self, ModelError> {
        stream.validate()?;
        if !stream.weight.into().is_finite() || stream.weight < T::ZERO {
            return Err(ModelError("stream weights must be finite and nonnegative"));
        }
        let mut normalizers = Vec::new();
        normalizers
            .try_reserve_exact(stream.mixtures.len())
            .map_err(|_| ModelError("prepared emission list allocation failed"))?;
        for distribution in &stream.mixtures {
            normalizers.push(gaussian_normalizers(distribution)?);
        }
        Ok(Self { normalizers })
    }

    fn emission<'a>(
        &'a self,
        stream: &'a StreamParameters<T>,
        state: usize,
    ) -> Option<PreparedGaussianView<'a, T>> {
        Some(PreparedGaussianView {
            distribution: stream.mixtures.get(state)?,
            normalizers: self.normalizers.get(state)?,
        })
    }
}

/// Immutable stream parameters and their matching Gaussian caches.
/// Moving or cloning the snapshot does not retain a borrow of its source.
#[derive(Debug, Clone)]
pub struct OwnedPreparedStreamParameters<T> {
    parameters: StreamParameters<T>,
    state: StreamPreparation<T>,
}

/// Binary32 owned prepared stream.
pub type OwnedPreparedStream = OwnedPreparedStreamParameters<f32>;

impl<T: Scalar> StreamParameters<T> {
    /// Consume a stream into an independent immutable preparation.
    /// Clone first to keep an editable source stream.
    pub fn prepare_owned(self) -> Result<OwnedPreparedStreamParameters<T>, ModelError> {
        let state = StreamPreparation::new(&self)?;
        Ok(OwnedPreparedStreamParameters {
            parameters: self,
            state,
        })
    }
}

impl<T> OwnedPreparedStreamParameters<T> {
    pub fn parameters(&self) -> &StreamParameters<T> {
        &self.parameters
    }

    /// Discard caches and recover editable parameters without copying their vectors.
    pub fn into_parameters(self) -> StreamParameters<T> {
        self.parameters
    }
}

impl<T: Scalar> OwnedPreparedStreamParameters<T> {
    /// Borrow a prepared emission without copying parameters or cached values.
    pub fn emission(&self, state: usize) -> Option<PreparedGaussianView<'_, T>> {
        self.state.emission(&self.parameters, state)
    }
}

/// Immutable duration parameters and derived state borrowed from a snapshot.
#[derive(Debug, Clone, Copy)]
pub struct PreparedDurationView<'a, T> {
    distribution: &'a DurationParameters<T>,
    state: &'a DurationPreparation<T>,
}

impl<T: Scalar> PreparedDurationView<'_, T> {
    pub fn parameters(&self) -> &DurationParameters<T> {
        self.distribution
    }

    pub fn options(&self) -> PreparationSettings<T> {
        self.state.options
    }

    pub fn log_normalizer(&self) -> T {
        self.state.normalizer
    }

    pub fn cached_log_probabilities(&self) -> &[T] {
        &self.state.cached
    }

    pub fn log_probability(&self, frames: usize) -> Result<T, ModelError> {
        self.state.log_probability(self.distribution, frames)
    }

    pub fn probability(&self, frames: usize) -> Result<f64, ModelError> {
        self.state.probability(self.distribution, frames)
    }

    /// Copy both parameters and the existing cache without recomputing it.
    pub fn to_owned(&self) -> OwnedPreparedDurationParameters<T> {
        OwnedPreparedDurationParameters {
            distribution: self.distribution.clone(),
            state: self.state.clone(),
        }
    }
}

/// Immutable complete model parameters and all requested derived caches.
/// Parameters remain stored once; evaluation borrows matching cache slices.
#[derive(Debug, Clone)]
pub struct OwnedPreparedModelParameters<T> {
    model: ModelParameters<T>,
    streams: Vec<StreamPreparation<T>>,
    durations: Vec<Option<DurationPreparation<T>>>,
    options: PreparationSettings<T>,
}

/// Binary32 owned prepared model.
pub type OwnedPreparedModel = OwnedPreparedModelParameters<f32>;

impl<T: Scalar> ModelParameters<T> {
    /// Consume the model and prepare all emission and duration distributions.
    pub fn prepare_owned(
        self,
        options: PreparationSettings<T>,
    ) -> Result<OwnedPreparedModelParameters<T>, ModelError> {
        let count = self.durations.len();
        self.prepare_owned_duration_states(options, 0..count)
    }

    /// Consume the model and prepare only selected duration states.
    /// Duplicate indices are prepared once. An empty selection supports models
    /// whose unused duration distributions have not been initialized.
    pub fn prepare_owned_selected_durations(
        self,
        options: PreparationSettings<T>,
        states: &[usize],
    ) -> Result<OwnedPreparedModelParameters<T>, ModelError> {
        self.prepare_owned_duration_states(options, states.iter().copied())
    }

    fn prepare_owned_duration_states(
        self,
        options: PreparationSettings<T>,
        states: impl IntoIterator<Item = usize>,
    ) -> Result<OwnedPreparedModelParameters<T>, ModelError> {
        options.validate()?;
        self.validate()?;
        let mut streams = Vec::new();
        streams
            .try_reserve_exact(self.streams.len())
            .map_err(|_| ModelError("prepared stream list allocation failed"))?;
        for stream in &self.streams {
            streams.push(StreamPreparation::new(stream)?);
        }
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
                *target = Some(DurationPreparation::new(&self.durations[state], options)?);
            }
        }
        Ok(OwnedPreparedModelParameters {
            model: self,
            streams,
            durations,
            options,
        })
    }
}

impl<T> OwnedPreparedModelParameters<T> {
    pub fn model(&self) -> &ModelParameters<T> {
        &self.model
    }

    /// Discard derived state and recover the complete editable model.
    pub fn into_model(self) -> ModelParameters<T> {
        self.model
    }
}

impl<T: Scalar> OwnedPreparedModelParameters<T> {
    pub fn options(&self) -> PreparationSettings<T> {
        self.options
    }

    pub fn duration(&self, state: usize) -> Option<PreparedDurationView<'_, T>> {
        Some(PreparedDurationView {
            distribution: self.model.durations.get(state)?,
            state: self.durations.get(state)?.as_ref()?,
        })
    }

    pub fn emission(&self, stream: usize, state: usize) -> Option<PreparedGaussianView<'_, T>> {
        self.streams
            .get(stream)?
            .emission(self.model.streams.get(stream)?, state)
    }

    pub fn emission_log_probabilities(
        &self,
        observation: &ObservationData<T>,
        segmentation: &SegmentationData<T>,
        temperature: T,
        pruning: EmissionPruningOptions<T>,
    ) -> Result<LogEmissionTable<T>, ModelError> {
        self.model.sample_emission_densities(
            observation,
            segmentation,
            temperature,
            pruning,
            |stream, state, features, temperature| {
                self.emission(stream, state)
                    .ok_or(ModelError("emission state is out of range"))?
                    .log_probability(features, temperature)
            },
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
