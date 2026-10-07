//! One expectation step, without changing the source model's parameters.
use crate::ModelError;
use crate::data::{ObservationData, SegmentationData};
use crate::emission::EmissionPruningOptions;
use crate::expectation::{GeometricPosteriorExpectation, HsmmPosteriorExpectation};
use crate::geometric::GeometricSettings;
use crate::hsmm::HsmmSettings;
use crate::mixture_occupancy::MixtureOccupancySettings;
use crate::model::ModelParameters;
use crate::scalar::Scalar;
use crate::statistics::ModelSufficientStatistics;

impl<T: Scalar> ModelSufficientStatistics<T> {
    /// Run HSMM inference and accumulate expectations. Return the log likelihood.
    pub fn estimate_hsmm(
        &mut self,
        model: &ModelParameters<T>,
        observation: &ObservationData<T>,
        segmentation: &SegmentationData<T>,
        options: HsmmSettings<T>,
    ) -> Result<T, ModelError> {
        self.validate_for(model)?;
        let emissions = model.emission_log_probabilities(
            observation,
            segmentation,
            options.temperature,
            EmissionPruningOptions::AroundBoundaries {
                radius: options.state_radius,
            },
        )?;
        let forward = model.forward_hsmm(segmentation, &emissions, options)?;
        let backward = model.backward_hsmm(segmentation, &emissions, options)?;
        let total = forward.end.total_through_state(&backward.end, 0)?;
        let durations = model.duration_occupancies(
            segmentation,
            &emissions,
            &forward,
            &backward,
            total,
            options,
        )?;
        let states = model.hsmm_state_occupancies(segmentation, &emissions, &durations, options)?;
        let mixtures = (0..model.streams.len())
            .map(|stream| {
                model.mixture_occupancies(
                    observation,
                    segmentation,
                    &states,
                    stream,
                    options.temperature,
                    MixtureOccupancySettings::Hsmm {
                        state_radius: options.state_radius,
                    },
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        self.collect_hsmm(
            model,
            observation,
            segmentation,
            HsmmPosteriorExpectation {
                durations: &durations,
                states: &states,
                mixtures: &mixtures,
                state_radius: options.state_radius,
            },
        )?;
        Ok(total)
    }

    /// Run geometric HMM inference and accumulate expectations. Return its total.
    pub fn estimate_geometric(
        &mut self,
        model: &ModelParameters<T>,
        observation: &ObservationData<T>,
        segmentation: &SegmentationData<T>,
        options: GeometricSettings<T>,
    ) -> Result<T, ModelError> {
        self.validate_for(model)?;
        let emissions = model.emission_log_probabilities(
            observation,
            segmentation,
            options.temperature,
            EmissionPruningOptions::Linear {
                slope: options.pruning_slope,
            },
        )?;
        let forward = model.forward_geometric(segmentation, &emissions, options)?;
        let backward = model.backward_geometric(segmentation, &emissions, options)?;
        let total = backward.backward_log_probability(&emissions)?;
        let transitions = model.geometric_transition_occupancies(
            segmentation,
            &emissions,
            &forward,
            &backward,
            total,
            options,
        )?;
        let states = forward.geometric_occupancies(&backward, total, options.pruning_slope)?;
        let mixtures = (0..model.streams.len())
            .map(|stream| {
                model.mixture_occupancies(
                    observation,
                    segmentation,
                    &states,
                    stream,
                    options.temperature,
                    MixtureOccupancySettings::Geometric {
                        pruning_slope: options.pruning_slope,
                    },
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        self.collect_geometric(
            model,
            observation,
            segmentation,
            GeometricPosteriorExpectation {
                transitions: &transitions,
                states: &states,
                mixtures: &mixtures,
                pruning_slope: options.pruning_slope,
            },
        )?;
        Ok(total)
    }
}
