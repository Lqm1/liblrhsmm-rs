//! One expectation step, without changing the source model's parameters.
use crate::{
    EmissionPruning, GeometricExpectation, GeometricOptions, HsmmExpectation, HsmmOptions,
    MixtureOccupancyMode, Model, ModelError, ModelStatistics, Observation, Segmentation,
};

impl ModelStatistics {
    /// Run HSMM inference and accumulate expectations. Return the log likelihood.
    pub fn estimate_hsmm(
        &mut self,
        model: &Model,
        observation: &Observation,
        segmentation: &Segmentation,
        options: HsmmOptions,
    ) -> Result<f32, ModelError> {
        self.validate_for(model)?;
        let emissions = model.emission_log_probabilities(
            observation,
            segmentation,
            options.temperature,
            EmissionPruning::AroundBoundaries {
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
                    MixtureOccupancyMode::Hsmm {
                        state_radius: options.state_radius,
                    },
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        self.collect_hsmm(
            model,
            observation,
            segmentation,
            HsmmExpectation {
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
        model: &Model,
        observation: &Observation,
        segmentation: &Segmentation,
        options: GeometricOptions,
    ) -> Result<f32, ModelError> {
        self.validate_for(model)?;
        let emissions = model.emission_log_probabilities(
            observation,
            segmentation,
            options.temperature,
            EmissionPruning::Linear {
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
                    MixtureOccupancyMode::Geometric {
                        pruning_slope: options.pruning_slope,
                    },
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        self.collect_geometric(
            model,
            observation,
            segmentation,
            GeometricExpectation {
                transitions: &transitions,
                states: &states,
                mixtures: &mixtures,
                pruning_slope: options.pruning_slope,
            },
        )?;
        Ok(total)
    }
}
