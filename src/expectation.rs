//! Accumulate posterior expectations for normal and geometric durations.
use crate::{
    DurationOccupancies, MixtureOccupancies, Model, ModelError, ModelStatistics, Observation,
    OccupancyTable, Segmentation, TransitionOccupancies,
};

/// Posterior tables and the boundary pruning radius used to compute them.
pub struct HsmmExpectation<'a> {
    pub durations: &'a DurationOccupancies,
    pub states: &'a OccupancyTable,
    pub mixtures: &'a [MixtureOccupancies],
    pub state_radius: f32,
}

/// Posterior tables and the diagonal pruning slope used to compute them.
pub struct GeometricExpectation<'a> {
    pub transitions: &'a TransitionOccupancies,
    pub states: &'a OccupancyTable,
    pub mixtures: &'a [MixtureOccupancies],
    pub pruning_slope: f32,
}

impl ModelStatistics {
    fn validate_expectation(
        &self,
        model: &Model,
        observation: &Observation,
        segmentation: &Segmentation,
        states: &OccupancyTable,
        mixtures: &[MixtureOccupancies],
    ) -> Result<(), ModelError> {
        self.validate_for(model)?;
        observation.validate()?;
        segmentation.validate()?;
        let count = segmentation.boundaries.len();
        if count == 0
            || observation.frames == 0
            || states.frames() != observation.frames
            || states.states() != count
            || mixtures.len() != self.streams.len()
            || observation.streams.len() != self.streams.len()
            || segmentation.output_states.len() != self.streams.len()
        {
            return Err(ModelError("expectation dimensions or stream counts differ"));
        }
        if observation
            .streams
            .iter()
            .flat_map(|stream| &stream.values)
            .any(|x| !x.is_finite())
        {
            return Err(ModelError("expectation observations must be finite"));
        }
        for (state, &duration) in segmentation.duration_states.iter().enumerate() {
            if duration < 0 || duration as usize >= self.durations.len() {
                return Err(ModelError("expectation duration state is out of range"));
            }
            for (stream, posterior) in mixtures.iter().enumerate() {
                if posterior.frames() != observation.frames || posterior.states() != count {
                    return Err(ModelError("mixture posterior dimensions differ"));
                }
                let index = segmentation.output_states[stream][state];
                let distribution = usize::try_from(index)
                    .ok()
                    .and_then(|index| self.streams[stream].mixtures.get(index))
                    .ok_or(ModelError("expectation emission state is out of range"))?;
                if distribution.dimensions != observation.streams[stream].dimensions {
                    return Err(ModelError("expectation feature dimensions differ"));
                }
                for time in 0..observation.frames {
                    if let Some(values) = posterior.get(time, state)
                        && (values.len() != distribution.component_occupancies.len()
                            || values.iter().any(|x| !x.is_finite() || *x < 0.0))
                    {
                        return Err(ModelError("invalid mixture posterior components"));
                    }
                }
            }
        }
        Ok(())
    }

    fn collect_emissions(
        &mut self,
        observation: &Observation,
        segmentation: &Segmentation,
        states: &OccupancyTable,
        mixtures: &[MixtureOccupancies],
        time: usize,
        state: usize,
    ) {
        for (stream, posterior) in mixtures.iter().enumerate() {
            let distribution = &mut self.streams[stream].mixtures
                [segmentation.output_states[stream][state] as usize];
            distribution.state_occupancy +=
                states.get(time, state).expect("validated state occupancy");
            if let Some(components) = posterior.get(time, state) {
                let features = observation
                    .frame(stream, time)
                    .expect("validated observation");
                for (component, &probability) in components.iter().enumerate() {
                    distribution.component_occupancies[component] += probability;
                    for (dimension, &value) in features.iter().enumerate() {
                        let index = component * distribution.dimensions + dimension;
                        distribution.weighted_sums[index] += probability * value;
                        distribution.weighted_squared_sums[index] += probability * value * value;
                    }
                }
            }
        }
    }

    /// Corresponds to `lrh_collect_ocp`, preserving inclusive boundary traversal.
    /// Overlapping boundary visits are accumulated repeatedly, as in upstream.
    pub fn collect_hsmm(
        &mut self,
        model: &Model,
        observation: &Observation,
        segmentation: &Segmentation,
        expectation: HsmmExpectation<'_>,
    ) -> Result<(), ModelError> {
        crate::pruning::validate_boundary_radius(expectation.state_radius)?;
        self.validate_expectation(
            model,
            observation,
            segmentation,
            expectation.states,
            expectation.mixtures,
        )?;
        let frames = observation.frames;
        let states = segmentation.boundaries.len();
        if expectation.durations.frames() != frames || expectation.durations.states() != states {
            return Err(ModelError("duration posterior dimensions differ"));
        }
        let mut previous = 0;
        for &boundary in &segmentation.boundaries {
            if boundary < previous || boundary as usize > frames {
                return Err(ModelError(
                    "expectation boundaries are unordered or out of range",
                ));
            }
            previous = boundary;
        }
        if previous as usize != frames {
            return Err(ModelError("final boundary must match frame count"));
        }
        let mut updated = self.clone();
        let mut begin = 0;
        for (segment, &boundary) in segmentation.boundaries.iter().enumerate() {
            for time in begin..=(boundary as usize).min(frames - 1) {
                for state in
                    crate::pruning::boundary_states(segment, expectation.state_radius, states - 1)
                {
                    let duration =
                        &mut updated.durations[segmentation.duration_states[state] as usize];
                    if let Some(posterior) = expectation.durations.get(time, state) {
                        for (index, &probability) in posterior.probabilities.iter().enumerate() {
                            if !probability.is_finite() || probability < 0.0 {
                                return Err(ModelError("invalid duration posterior probability"));
                            }
                            let length = posterior
                                .minimum
                                .checked_add(index)
                                .ok_or(ModelError("posterior duration overflows"))?
                                as f32;
                            duration.value_sum += probability * length;
                            duration.squared_sum += probability * length * length;
                            duration.occupancy += probability;
                        }
                    }
                    updated.collect_emissions(
                        observation,
                        segmentation,
                        expectation.states,
                        expectation.mixtures,
                        time,
                        state,
                    );
                }
            }
            begin = boundary as usize;
        }
        updated.validate_for(model)?;
        *self = updated;
        Ok(())
    }

    /// Corresponds to `lrh_collect_ocp_geometric`; endpoint frames are excluded.
    pub fn collect_geometric(
        &mut self,
        model: &Model,
        observation: &Observation,
        segmentation: &Segmentation,
        expectation: GeometricExpectation<'_>,
    ) -> Result<(), ModelError> {
        self.validate_expectation(
            model,
            observation,
            segmentation,
            expectation.states,
            expectation.mixtures,
        )?;
        let frames = observation.frames;
        let states = segmentation.boundaries.len();
        if expectation.transitions.frames() != frames
            || expectation.transitions.states() != states
            || !expectation.pruning_slope.is_finite()
            || expectation.pruning_slope < 0.0
        {
            return Err(ModelError("invalid transition dimensions or pruning slope"));
        }
        let radius = (expectation.pruning_slope * states as f32) as usize;
        let mut updated = self.clone();
        for time in 1..frames.saturating_sub(1) {
            let center = (time as u64 * states as u64 / frames as u64) as usize;
            for state in center.saturating_sub(radius)..center.saturating_add(radius).min(states) {
                let duration = &mut updated.durations[segmentation.duration_states[state] as usize];
                if let Some(posterior) = expectation.transitions.get(time, state) {
                    if !posterior.advance.is_finite() || posterior.advance < 0.0 {
                        return Err(ModelError("invalid transition posterior probability"));
                    }
                    duration.value_sum += posterior.advance;
                }
                duration.occupancy += expectation
                    .states
                    .get(time, state)
                    .expect("validated state occupancy");
                updated.collect_emissions(
                    observation,
                    segmentation,
                    expectation.states,
                    expectation.mixtures,
                    time,
                    state,
                );
            }
        }
        updated.validate_for(model)?;
        *self = updated;
        Ok(())
    }
}
