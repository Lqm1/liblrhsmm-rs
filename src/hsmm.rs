//! Explicit-duration forward and Viterbi inference with arbitrary state jumps.
use crate::{
    EmissionTable, Model, ModelError, Segmentation, StateScores,
    numerics::{LOG_ZERO, log_add},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HsmmOptions {
    pub temperature: f32,
    pub duration_weight: f32,
    pub state_radius: usize,
    pub duration_extra: usize,
    pub duration_extra_factor: f32,
}
impl Default for HsmmOptions {
    fn default() -> Self {
        Self {
            temperature: 1.0,
            duration_weight: 1.0,
            state_radius: 5,
            duration_extra: 30,
            duration_extra_factor: 1.0,
        }
    }
}

/// Scores at the first and last frame of each candidate state occurrence.
#[derive(Debug, Clone, PartialEq)]
pub struct HsmmScores {
    pub begin: StateScores,
    pub end: StateScores,
}

/// One occurrence in a path; states can repeat or be skipped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StateOccurrence {
    pub state: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HsmmAlignment {
    pub occurrences: Vec<StateOccurrence>,
    pub log_probability: f32,
    pub scores: HsmmScores,
}

fn filled<T: Clone>(length: usize, value: T) -> Result<Vec<T>, ModelError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(length)
        .map_err(|_| ModelError("HSMM allocation failed"))?;
    values.resize(length, value);
    Ok(values)
}

impl Model {
    pub(crate) fn prepare_hsmm_durations(
        &self,
        segmentation: &Segmentation,
        emissions: &EmissionTable,
        options: HsmmOptions,
        limit_to_frames: bool,
    ) -> Result<Vec<Vec<f32>>, ModelError> {
        self.prepare_hsmm_durations_with(segmentation, emissions, options, limit_to_frames, None)
    }

    fn prepare_hsmm_durations_with(
        &self,
        segmentation: &Segmentation,
        emissions: &EmissionTable,
        options: HsmmOptions,
        limit_to_frames: bool,
        prepared: Option<&crate::PreparedModel<'_>>,
    ) -> Result<Vec<Vec<f32>>, ModelError> {
        if let Some(prepared) = prepared {
            let cached_options = prepared.options();
            if cached_options.temperature != options.temperature
                || cached_options.duration_weight != options.duration_weight
            {
                return Err(ModelError(
                    "HSMM temperature and duration weight differ from preparation",
                ));
            }
        }
        segmentation.validate()?;
        let frames = emissions.frames();
        let states = emissions.states();
        if states != segmentation.boundaries.len() {
            return Err(ModelError("emission and segmentation state counts differ"));
        }
        if !options.temperature.is_finite()
            || options.temperature < 0.0
            || !options.duration_weight.is_finite()
            || options.duration_weight < 0.0
            || !options.duration_extra_factor.is_finite()
            || options.duration_extra_factor < 0.0
        {
            return Err(ModelError(
                "HSMM weights and duration factor must be finite and nonnegative",
            ));
        }
        let mut durations = Vec::with_capacity(states);
        let mut previous = 0;
        for (state, &boundary) in segmentation.boundaries.iter().enumerate() {
            if boundary < previous || boundary < 0 || boundary as usize > frames {
                return Err(ModelError(
                    "HSMM boundaries must be ordered within the observation",
                ));
            }
            let duration_index = usize::try_from(segmentation.duration_states[state])
                .map_err(|_| ModelError("negative duration state"))?;
            let distribution = self
                .durations
                .get(duration_index)
                .ok_or(ModelError("duration state is out of range"))?;
            if !distribution.mean.is_finite()
                || !distribution.variance.is_finite()
                || distribution.variance <= 0.0
            {
                return Err(ModelError(
                    "HSMM duration requires a finite mean and positive variance",
                ));
            }
            let standard_duration = boundary - previous;
            let initial = (f64::from(standard_duration)
                * (1.0 + f64::from(options.duration_extra_factor))
                + options.duration_extra as f64)
                .trunc();
            let mut maximum = initial
                .max(f64::from(distribution.mean) + f64::from(distribution.variance).sqrt() * 2.0)
                .trunc();
            if distribution.maximum > 0 {
                maximum = maximum.min(f64::from(distribution.maximum));
            }
            let maximum = maximum.max(0.0) as usize;
            // Forward candidates cannot begin before frame zero. The original
            // backward calculation also evaluates durations beyond the sample.
            let maximum = if limit_to_frames {
                maximum.min(frames.saturating_add(1))
            } else {
                maximum
            };
            let mut probabilities = filled(maximum, LOG_ZERO)?;
            for (duration, probability) in probabilities.iter_mut().enumerate().skip(1) {
                if let Some(prepared) = prepared {
                    *probability = prepared
                        .duration(duration_index)
                        .ok_or(ModelError("prepared duration state is out of range"))?
                        .log_probability(duration)?;
                    continue;
                }
                if options.temperature == 0.0
                    && distribution.minimum > 0
                    && duration < distribution.minimum as usize
                {
                    continue;
                }
                *probability = distribution
                    .log_probability(duration as f32, options.duration_weight)?
                    * options.temperature;
            }
            durations.push(probabilities);
            previous = boundary;
        }
        if previous as usize != frames {
            return Err(ModelError("final HSMM boundary must match the frame count"));
        }
        Ok(durations)
    }

    fn search_hsmm(
        &self,
        segmentation: &Segmentation,
        emissions: &EmissionTable,
        options: HsmmOptions,
        viterbi: bool,
        prepared: Option<&crate::PreparedModel<'_>>,
    ) -> Result<(HsmmScores, Vec<usize>, Vec<usize>), ModelError> {
        let graph = segmentation.build_transitions()?;
        let durations =
            self.prepare_hsmm_durations_with(segmentation, emissions, options, true, prepared)?;
        let frames = emissions.frames();
        let states = emissions.states();
        let length = emissions.values().len();
        let mut begin = filled(length, LOG_ZERO)?;
        let mut end = filled(length, LOG_ZERO)?;
        let mut from_state = if viterbi {
            filled(length, usize::MAX)?
        } else {
            Vec::new()
        };
        let mut from_frame = if viterbi {
            filled(length, usize::MAX)?
        } else {
            Vec::new()
        };
        let mut interval_start = 0;
        for (segment, &boundary) in segmentation.boundaries.iter().enumerate() {
            for frame in interval_start..=(boundary as usize).min(frames - 1) {
                for state in segment.saturating_sub(options.state_radius)
                    ..=segment.saturating_add(options.state_radius).min(states - 1)
                {
                    let index = frame * states + state;
                    let mut initiation = LOG_ZERO;
                    let mut predecessor = usize::MAX;
                    if frame > 0 {
                        for jump in &graph.incoming()[state] {
                            if jump.probability == 0.0 {
                                continue;
                            }
                            let source = state as i64 + i64::from(jump.delta);
                            if source < 0 {
                                continue;
                            }
                            let source = source as usize;
                            let transition = (f64::from(end[(frame - 1) * states + source])
                                + f64::from(jump.probability).ln() * f64::from(options.temperature))
                                as f32;
                            if viterbi {
                                if transition > initiation {
                                    initiation = transition;
                                    predecessor = source;
                                }
                            } else {
                                initiation = log_add(initiation, transition);
                            }
                        }
                    }
                    initiation += emissions.values()[index];
                    if (state == 0 && frame > 0) || (frame == 0 && state > 0) {
                        initiation = LOG_ZERO;
                    } else if frame == 0 && state == 0 {
                        initiation = emissions.values()[index];
                    }
                    begin[index] = initiation;
                    if viterbi {
                        from_state[index] = predecessor;
                    }
                    let mut termination = LOG_ZERO;
                    let mut observed = 0.0f32;
                    let mut start = usize::MAX;
                    for (duration, &duration_probability) in
                        durations[state].iter().enumerate().take(frame + 2).skip(1)
                    {
                        let origin = frame + 1 - duration;
                        let candidate =
                            observed + duration_probability + begin[origin * states + state];
                        if viterbi {
                            if candidate > termination {
                                termination = candidate;
                                start = origin;
                            }
                        } else {
                            termination = log_add(termination, candidate);
                        }
                        observed += emissions.values()[origin * states + state];
                    }
                    end[index] = termination;
                    if viterbi {
                        from_frame[index] = start;
                    }
                }
            }
            interval_start = boundary as usize;
        }
        Ok((
            HsmmScores {
                begin: StateScores {
                    frames,
                    states,
                    values: begin,
                },
                end: StateScores {
                    frames,
                    states,
                    values: end,
                },
            },
            from_state,
            from_frame,
        ))
    }

    pub fn forward_hsmm(
        &self,
        segmentation: &Segmentation,
        emissions: &EmissionTable,
        options: HsmmOptions,
    ) -> Result<HsmmScores, ModelError> {
        Ok(self
            .search_hsmm(segmentation, emissions, options, false, None)?
            .0)
    }

    /// Original backward recurrence, including the final-state tail convention.
    pub fn backward_hsmm(
        &self,
        segmentation: &Segmentation,
        emissions: &EmissionTable,
        options: HsmmOptions,
    ) -> Result<HsmmScores, ModelError> {
        self.backward_hsmm_with(segmentation, emissions, options, None)
    }

    fn backward_hsmm_with(
        &self,
        segmentation: &Segmentation,
        emissions: &EmissionTable,
        options: HsmmOptions,
        prepared: Option<&crate::PreparedModel<'_>>,
    ) -> Result<HsmmScores, ModelError> {
        let graph = segmentation.build_transitions()?;
        let durations =
            self.prepare_hsmm_durations_with(segmentation, emissions, options, false, prepared)?;
        let frames = emissions.frames();
        let states = emissions.states();
        let mut begin = filled(emissions.values().len(), LOG_ZERO)?;
        let mut end = filled(emissions.values().len(), LOG_ZERO)?;
        for segment in (0..states).rev() {
            let start = if segment == 0 {
                0
            } else {
                segmentation.boundaries[segment - 1] as usize
            };
            let boundary = segmentation.boundaries[segment] as usize;
            for frame in (start..boundary.min(frames)).rev() {
                for state in segment.saturating_sub(options.state_radius)
                    ..=segment.saturating_add(options.state_radius).min(states - 1)
                {
                    let index = frame * states + state;
                    let mut termination = LOG_ZERO;
                    if frame + 1 < frames && state + 1 < states {
                        for jump in &graph.outgoing()[state] {
                            if jump.probability == 0.0 {
                                continue;
                            }
                            let destination = state as i64 + i64::from(jump.delta);
                            if destination < 0 || destination >= states as i64 {
                                continue;
                            }
                            let next = (frame + 1) * states + destination as usize;
                            let candidate = (f64::from(emissions.values()[next] + begin[next])
                                + f64::from(jump.probability).ln() * f64::from(options.temperature))
                                as f32;
                            termination = log_add(termination, candidate);
                        }
                    } else if frame + 1 == frames {
                        termination = 0.0;
                    }
                    end[index] = termination;
                    let mut initiation = LOG_ZERO;
                    let mut observed = 0.0f32;
                    if state == states - 1 {
                        initiation = 0.0;
                        for tail in frame + 1..frames {
                            initiation += emissions.values()[tail * states + state];
                        }
                    } else {
                        for (duration, &probability) in durations[state].iter().enumerate().skip(1)
                        {
                            let tail = frame
                                .checked_add(duration - 1)
                                .ok_or(ModelError("duration end overflows"))?;
                            if tail < frames && tail > frame {
                                observed += emissions.values()[tail * states + state];
                            }
                            let origin = if tail < frames {
                                end[tail * states + state]
                            } else {
                                0.0
                            };
                            initiation = log_add(initiation, observed + probability + origin);
                        }
                    }
                    begin[index] = initiation;
                }
            }
        }
        Ok(HsmmScores {
            begin: StateScores {
                frames,
                states,
                values: begin,
            },
            end: StateScores {
                frames,
                states,
                values: end,
            },
        })
    }

    pub fn viterbi_hsmm(
        &self,
        segmentation: &Segmentation,
        emissions: &EmissionTable,
        options: HsmmOptions,
    ) -> Result<HsmmAlignment, ModelError> {
        self.viterbi_hsmm_with(segmentation, emissions, options, None)
    }

    fn viterbi_hsmm_with(
        &self,
        segmentation: &Segmentation,
        emissions: &EmissionTable,
        options: HsmmOptions,
        prepared: Option<&crate::PreparedModel<'_>>,
    ) -> Result<HsmmAlignment, ModelError> {
        let (scores, from_state, from_frame) =
            self.search_hsmm(segmentation, emissions, options, true, prepared)?;
        let states = emissions.states();
        let mut state = states - 1;
        let mut frame = emissions.frames() - 1;
        let log_probability = scores.end.values()[frame * states + state];
        let mut occurrences = Vec::new();
        loop {
            let origin = from_frame[frame * states + state];
            if origin == usize::MAX {
                return Err(ModelError(
                    "no complete HSMM path survives the inference constraints",
                ));
            }
            occurrences.push(StateOccurrence {
                state,
                end: frame + 1,
            });
            if origin == 0 {
                if state != 0 {
                    return Err(ModelError("HSMM path does not begin in the first state"));
                }
                break;
            }
            let predecessor = from_state[origin * states + state];
            if predecessor == usize::MAX {
                return Err(ModelError("HSMM path has no predecessor"));
            }
            state = predecessor;
            frame = origin - 1;
        }
        occurrences.reverse();
        Ok(HsmmAlignment {
            occurrences,
            log_probability,
            scores,
        })
    }
}

impl crate::PreparedModel<'_> {
    /// Reuse duration caches. Temperature and duration weight must match preparation.
    pub fn forward_hsmm(
        &self,
        segmentation: &Segmentation,
        emissions: &EmissionTable,
        options: HsmmOptions,
    ) -> Result<HsmmScores, ModelError> {
        Ok(self
            .model
            .search_hsmm(segmentation, emissions, options, false, Some(self))?
            .0)
    }

    pub fn backward_hsmm(
        &self,
        segmentation: &Segmentation,
        emissions: &EmissionTable,
        options: HsmmOptions,
    ) -> Result<HsmmScores, ModelError> {
        self.model
            .backward_hsmm_with(segmentation, emissions, options, Some(self))
    }

    pub fn viterbi_hsmm(
        &self,
        segmentation: &Segmentation,
        emissions: &EmissionTable,
        options: HsmmOptions,
    ) -> Result<HsmmAlignment, ModelError> {
        self.model
            .viterbi_hsmm_with(segmentation, emissions, options, Some(self))
    }
}
