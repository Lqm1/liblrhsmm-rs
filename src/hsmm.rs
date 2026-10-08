//! Explicit-duration forward and Viterbi inference with arbitrary state jumps.
use crate::ModelError;
use crate::data::SegmentationData;
use crate::emission::LogEmissionTable;
use crate::geometric::StateScoreTable;
use crate::model::{DurationParameters, ModelParameters};
use crate::preparation::{
    OwnedPreparedModelParameters, PreparationSettings, PreparedModelParameters,
};
use crate::scalar::Scalar;
use crate::transitions::{TransitionGraphInput, TransitionGraphView};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HsmmSettings<T> {
    pub temperature: T,
    pub duration_weight: T,
    pub state_radius: T,
    pub duration_extra: usize,
    pub duration_extra_factor: T,
}
impl<T: Scalar> Default for HsmmSettings<T> {
    fn default() -> Self {
        Self {
            temperature: T::from_f64(1.0),
            duration_weight: T::from_f64(1.0),
            state_radius: T::from_f64(5.0),
            duration_extra: 30,
            duration_extra_factor: T::from_f64(1.0),
        }
    }
}

/// Scores at the first and last frame of each candidate state occurrence.
#[derive(Debug, Clone, PartialEq)]
pub struct HsmmScoreTables<T> {
    pub begin: StateScoreTable<T>,
    pub end: StateScoreTable<T>,
}

/// One occurrence in a path; states can repeat or be skipped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StateOccurrence {
    pub state: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HsmmPath<T> {
    pub occurrences: Vec<StateOccurrence>,
    pub log_probability: T,
    pub scores: HsmmScoreTables<T>,
}

struct SearchResult<T> {
    scores: HsmmScoreTables<T>,
    from_state: Vec<usize>,
    from_frame: Vec<usize>,
}

/// Reference-only duration cache selection for both prepared model representations.
enum PreparedHsmmDurations<'a, 'm, T> {
    Borrowed(&'a PreparedModelParameters<'m, T>),
    Owned(&'a OwnedPreparedModelParameters<T>),
}

impl<T> Copy for PreparedHsmmDurations<'_, '_, T> {}
impl<T> Clone for PreparedHsmmDurations<'_, '_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Scalar> PreparedHsmmDurations<'_, '_, T> {
    fn options(&self) -> PreparationSettings<T> {
        match self {
            Self::Borrowed(model) => model.options(),
            Self::Owned(model) => model.options(),
        }
    }

    fn log_probability(&self, state: usize, duration: usize) -> Result<T, ModelError> {
        match self {
            Self::Borrowed(model) => model
                .duration(state)
                .ok_or(ModelError("prepared duration state is out of range"))?
                .log_probability(duration),
            Self::Owned(model) => model
                .duration(state)
                .ok_or(ModelError("prepared duration state is out of range"))?
                .log_probability(duration),
        }
    }
}

fn filled<T: Clone>(length: usize, value: T) -> Result<Vec<T>, ModelError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(length)
        .map_err(|_| ModelError("HSMM allocation failed"))?;
    values.resize(length, value);
    Ok(values)
}

// Exclusive upper bound, preserving both integer assignments in the C macro.
fn duration_limit<T: Scalar>(
    standard_duration: i32,
    distribution: &DurationParameters<T>,
    options: HsmmSettings<T>,
) -> usize {
    let initial = (f64::from(standard_duration) * (1.0 + options.duration_extra_factor.into())
        + options.duration_extra as f64)
        .trunc();
    let mut maximum = initial
        .max(distribution.mean.into() + distribution.variance.into().sqrt() * 2.0)
        .trunc();
    if distribution.maximum > 0 {
        maximum = maximum.min(f64::from(distribution.maximum));
    }
    maximum.max(0.0) as usize
}

impl<T: Scalar> ModelParameters<T> {
    pub(crate) fn prepare_hsmm_durations(
        &self,
        segmentation: &SegmentationData<T>,
        emissions: &LogEmissionTable<T>,
        options: HsmmSettings<T>,
        limit_to_frames: bool,
    ) -> Result<Vec<Vec<T>>, ModelError> {
        self.prepare_hsmm_durations_with(segmentation, emissions, options, limit_to_frames, None)
    }

    fn prepare_hsmm_durations_with(
        &self,
        segmentation: &SegmentationData<T>,
        emissions: &LogEmissionTable<T>,
        options: HsmmSettings<T>,
        limit_to_frames: bool,
        prepared: Option<PreparedHsmmDurations<'_, '_, T>>,
    ) -> Result<Vec<Vec<T>>, ModelError> {
        crate::pruning::validate_boundary_radius(options.state_radius)?;
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
        if !options.temperature.into().is_finite()
            || options.temperature < T::ZERO
            || !options.duration_weight.into().is_finite()
            || options.duration_weight < T::ZERO
            || !options.duration_extra_factor.into().is_finite()
            || options.duration_extra_factor < T::ZERO
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
            if !distribution.mean.into().is_finite()
                || !distribution.variance.into().is_finite()
                || distribution.variance <= T::ZERO
            {
                return Err(ModelError(
                    "HSMM duration requires a finite mean and positive variance",
                ));
            }
            let standard_duration = boundary - previous;
            let maximum = duration_limit(standard_duration, distribution, options);
            // Forward candidates cannot begin before frame zero. The original
            // backward calculation also evaluates durations beyond the sample.
            let maximum = if limit_to_frames {
                maximum.min(frames.saturating_add(1))
            } else {
                maximum
            };
            let mut probabilities = filled(maximum, T::from_f64(-1e15))?;
            for (duration, probability) in probabilities.iter_mut().enumerate().skip(1) {
                if let Some(prepared) = prepared {
                    *probability = prepared.log_probability(duration_index, duration)?;
                    continue;
                }
                if options.temperature == T::ZERO
                    && distribution.minimum > 0
                    && duration < distribution.minimum as usize
                {
                    continue;
                }
                *probability = distribution
                    .log_probability(T::from_f64(duration as f64), options.duration_weight)?
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

    /// Reuse immutable incoming jumps without rebuilding or copying the graph cache.
    pub fn forward_hsmm_with_graph(
        &self,
        graph: TransitionGraphView<'_, T>,
        emissions: &LogEmissionTable<T>,
        options: HsmmSettings<T>,
    ) -> Result<HsmmScoreTables<T>, ModelError> {
        Ok(self
            .search_hsmm(
                TransitionGraphInput::Cached(graph),
                emissions,
                options,
                false,
                None,
            )?
            .scores)
    }

    /// Reuse complete immutable outgoing topology with the original backward recurrence.
    pub fn backward_hsmm_with_graph(
        &self,
        graph: TransitionGraphView<'_, T>,
        emissions: &LogEmissionTable<T>,
        options: HsmmSettings<T>,
    ) -> Result<HsmmScoreTables<T>, ModelError> {
        self.backward_hsmm_with(
            TransitionGraphInput::Cached(graph),
            emissions,
            options,
            None,
        )
    }

    /// Reuse immutable incoming jumps with the original Viterbi path and score rules.
    pub fn viterbi_hsmm_with_graph(
        &self,
        graph: TransitionGraphView<'_, T>,
        emissions: &LogEmissionTable<T>,
        options: HsmmSettings<T>,
    ) -> Result<HsmmPath<T>, ModelError> {
        self.viterbi_hsmm_with(
            TransitionGraphInput::Cached(graph),
            emissions,
            options,
            None,
        )
    }

    fn search_hsmm(
        &self,
        input: TransitionGraphInput<'_, T>,
        emissions: &LogEmissionTable<T>,
        options: HsmmSettings<T>,
        viterbi: bool,
        prepared: Option<PreparedHsmmDurations<'_, '_, T>>,
    ) -> Result<SearchResult<T>, ModelError> {
        let segmentation = input.segmentation();
        let built;
        let graph = if let Some(graph) = input.cached() {
            graph
        } else {
            built = segmentation.build_transitions()?;
            built.as_view()
        };
        let durations =
            self.prepare_hsmm_durations_with(segmentation, emissions, options, true, prepared)?;
        let frames = emissions.frames();
        let states = emissions.states();
        let length = emissions.values().len();
        let mut begin = filled(length, T::from_f64(-1e15))?;
        let mut end = filled(length, T::from_f64(-1e15))?;
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
                for state in
                    crate::pruning::boundary_states(segment, options.state_radius, states - 1)
                {
                    let index = frame * states + state;
                    let mut initiation = T::from_f64(-1e15);
                    let mut predecessor = usize::MAX;
                    if frame > 0 {
                        for jump in &graph.incoming()[state] {
                            if jump.probability == T::ZERO {
                                continue;
                            }
                            let source = state as i64 + i64::from(jump.delta);
                            if source < 0 {
                                continue;
                            }
                            let source = source as usize;
                            let transition = T::from_f64(
                                end[(frame - 1) * states + source].into()
                                    + jump.probability.into().ln() * options.temperature.into(),
                            );
                            if viterbi {
                                if transition > initiation {
                                    initiation = transition;
                                    predecessor = source;
                                }
                            } else {
                                initiation = T::log_add(initiation, transition);
                            }
                        }
                    }
                    initiation += emissions.values()[index];
                    if (state == 0 && frame > 0) || (frame == 0 && state > 0) {
                        initiation = T::from_f64(-1e15);
                    } else if frame == 0 && state == 0 {
                        initiation = emissions.values()[index];
                    }
                    begin[index] = initiation;
                    if viterbi {
                        from_state[index] = predecessor;
                    }
                    let mut termination = T::from_f64(-1e15);
                    let mut observed = T::ZERO;
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
                            termination = T::log_add(termination, candidate);
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
        Ok(SearchResult {
            scores: HsmmScoreTables {
                begin: StateScoreTable {
                    frames,
                    states,
                    values: begin,
                },
                end: StateScoreTable {
                    frames,
                    states,
                    values: end,
                },
            },
            from_state,
            from_frame,
        })
    }

    pub fn forward_hsmm(
        &self,
        segmentation: &SegmentationData<T>,
        emissions: &LogEmissionTable<T>,
        options: HsmmSettings<T>,
    ) -> Result<HsmmScoreTables<T>, ModelError> {
        Ok(self
            .search_hsmm(
                TransitionGraphInput::Segmentation(segmentation),
                emissions,
                options,
                false,
                None,
            )?
            .scores)
    }

    /// Original backward recurrence, including the final-state tail convention.
    pub fn backward_hsmm(
        &self,
        segmentation: &SegmentationData<T>,
        emissions: &LogEmissionTable<T>,
        options: HsmmSettings<T>,
    ) -> Result<HsmmScoreTables<T>, ModelError> {
        self.backward_hsmm_with(
            TransitionGraphInput::Segmentation(segmentation),
            emissions,
            options,
            None,
        )
    }

    fn backward_hsmm_with(
        &self,
        input: TransitionGraphInput<'_, T>,
        emissions: &LogEmissionTable<T>,
        options: HsmmSettings<T>,
        prepared: Option<PreparedHsmmDurations<'_, '_, T>>,
    ) -> Result<HsmmScoreTables<T>, ModelError> {
        let segmentation = input.segmentation();
        let built;
        let graph = if let Some(graph) = input.cached() {
            graph
        } else {
            built = segmentation.build_transitions()?;
            built.as_view()
        };
        let durations =
            self.prepare_hsmm_durations_with(segmentation, emissions, options, false, prepared)?;
        let frames = emissions.frames();
        let states = emissions.states();
        let mut begin = filled(emissions.values().len(), T::from_f64(-1e15))?;
        let mut end = filled(emissions.values().len(), T::from_f64(-1e15))?;
        for segment in (0..states).rev() {
            let start = if segment == 0 {
                0
            } else {
                segmentation.boundaries[segment - 1] as usize
            };
            let boundary = segmentation.boundaries[segment] as usize;
            for frame in (start..boundary.min(frames)).rev() {
                for state in
                    crate::pruning::boundary_states(segment, options.state_radius, states - 1)
                {
                    let index = frame * states + state;
                    let mut termination = T::from_f64(-1e15);
                    if frame + 1 < frames && state + 1 < states {
                        for jump in &graph.outgoing()[state] {
                            if jump.probability == T::ZERO {
                                continue;
                            }
                            let destination = state as i64 + i64::from(jump.delta);
                            if destination < 0 || destination >= states as i64 {
                                continue;
                            }
                            let next = (frame + 1) * states + destination as usize;
                            let candidate = T::from_f64(
                                (emissions.values()[next] + begin[next]).into()
                                    + jump.probability.into().ln() * options.temperature.into(),
                            );
                            termination = T::log_add(termination, candidate);
                        }
                    } else if frame + 1 == frames {
                        termination = T::ZERO;
                    }
                    end[index] = termination;
                    let mut initiation = T::from_f64(-1e15);
                    let mut observed = T::ZERO;
                    if state == states - 1 {
                        initiation = T::ZERO;
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
                                T::ZERO
                            };
                            initiation = T::log_add(initiation, observed + probability + origin);
                        }
                    }
                    begin[index] = initiation;
                }
            }
        }
        Ok(HsmmScoreTables {
            begin: StateScoreTable {
                frames,
                states,
                values: begin,
            },
            end: StateScoreTable {
                frames,
                states,
                values: end,
            },
        })
    }

    pub fn viterbi_hsmm(
        &self,
        segmentation: &SegmentationData<T>,
        emissions: &LogEmissionTable<T>,
        options: HsmmSettings<T>,
    ) -> Result<HsmmPath<T>, ModelError> {
        self.viterbi_hsmm_with(
            TransitionGraphInput::Segmentation(segmentation),
            emissions,
            options,
            None,
        )
    }

    fn viterbi_hsmm_with(
        &self,
        input: TransitionGraphInput<'_, T>,
        emissions: &LogEmissionTable<T>,
        options: HsmmSettings<T>,
        prepared: Option<PreparedHsmmDurations<'_, '_, T>>,
    ) -> Result<HsmmPath<T>, ModelError> {
        let SearchResult {
            scores,
            from_state,
            from_frame,
        } = self.search_hsmm(input, emissions, options, true, prepared)?;
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
        Ok(HsmmPath {
            occurrences,
            log_probability,
            scores,
        })
    }
}

#[cfg(test)]
mod tests;

impl<T: Scalar> PreparedModelParameters<'_, T> {
    /// Reuse prepared durations and immutable incoming jumps without copying caches.
    pub fn forward_hsmm_with_graph(
        &self,
        graph: TransitionGraphView<'_, T>,
        emissions: &LogEmissionTable<T>,
        options: HsmmSettings<T>,
    ) -> Result<HsmmScoreTables<T>, ModelError> {
        Ok(self
            .model
            .search_hsmm(
                TransitionGraphInput::Cached(graph),
                emissions,
                options,
                false,
                Some(PreparedHsmmDurations::Borrowed(self)),
            )?
            .scores)
    }

    /// Reuse complete immutable outgoing topology with the original backward recurrence.
    pub fn backward_hsmm_with_graph(
        &self,
        graph: TransitionGraphView<'_, T>,
        emissions: &LogEmissionTable<T>,
        options: HsmmSettings<T>,
    ) -> Result<HsmmScoreTables<T>, ModelError> {
        self.model.backward_hsmm_with(
            TransitionGraphInput::Cached(graph),
            emissions,
            options,
            Some(PreparedHsmmDurations::Borrowed(self)),
        )
    }

    /// Reuse immutable incoming jumps with the original Viterbi path and score rules.
    pub fn viterbi_hsmm_with_graph(
        &self,
        graph: TransitionGraphView<'_, T>,
        emissions: &LogEmissionTable<T>,
        options: HsmmSettings<T>,
    ) -> Result<HsmmPath<T>, ModelError> {
        self.model.viterbi_hsmm_with(
            TransitionGraphInput::Cached(graph),
            emissions,
            options,
            Some(PreparedHsmmDurations::Borrowed(self)),
        )
    }

    /// Reuse duration caches. Temperature and duration weight must match preparation.
    pub fn forward_hsmm(
        &self,
        segmentation: &SegmentationData<T>,
        emissions: &LogEmissionTable<T>,
        options: HsmmSettings<T>,
    ) -> Result<HsmmScoreTables<T>, ModelError> {
        Ok(self
            .model
            .search_hsmm(
                TransitionGraphInput::Segmentation(segmentation),
                emissions,
                options,
                false,
                Some(PreparedHsmmDurations::Borrowed(self)),
            )?
            .scores)
    }

    pub fn backward_hsmm(
        &self,
        segmentation: &SegmentationData<T>,
        emissions: &LogEmissionTable<T>,
        options: HsmmSettings<T>,
    ) -> Result<HsmmScoreTables<T>, ModelError> {
        self.model.backward_hsmm_with(
            TransitionGraphInput::Segmentation(segmentation),
            emissions,
            options,
            Some(PreparedHsmmDurations::Borrowed(self)),
        )
    }

    pub fn viterbi_hsmm(
        &self,
        segmentation: &SegmentationData<T>,
        emissions: &LogEmissionTable<T>,
        options: HsmmSettings<T>,
    ) -> Result<HsmmPath<T>, ModelError> {
        self.model.viterbi_hsmm_with(
            TransitionGraphInput::Segmentation(segmentation),
            emissions,
            options,
            Some(PreparedHsmmDurations::Borrowed(self)),
        )
    }
}

impl<T: Scalar> OwnedPreparedModelParameters<T> {
    /// Reuse the owned duration cache and construct incoming jumps for this sample.
    pub fn forward_hsmm(
        &self,
        segmentation: &SegmentationData<T>,
        emissions: &LogEmissionTable<T>,
        options: HsmmSettings<T>,
    ) -> Result<HsmmScoreTables<T>, ModelError> {
        Ok(self
            .model()
            .search_hsmm(
                TransitionGraphInput::Segmentation(segmentation),
                emissions,
                options,
                false,
                Some(PreparedHsmmDurations::Owned(self)),
            )?
            .scores)
    }

    /// Reuse the owned duration cache and original backward recurrence.
    pub fn backward_hsmm(
        &self,
        segmentation: &SegmentationData<T>,
        emissions: &LogEmissionTable<T>,
        options: HsmmSettings<T>,
    ) -> Result<HsmmScoreTables<T>, ModelError> {
        self.model().backward_hsmm_with(
            TransitionGraphInput::Segmentation(segmentation),
            emissions,
            options,
            Some(PreparedHsmmDurations::Owned(self)),
        )
    }

    /// Reuse the owned duration cache with the original Viterbi path rules.
    pub fn viterbi_hsmm(
        &self,
        segmentation: &SegmentationData<T>,
        emissions: &LogEmissionTable<T>,
        options: HsmmSettings<T>,
    ) -> Result<HsmmPath<T>, ModelError> {
        self.model().viterbi_hsmm_with(
            TransitionGraphInput::Segmentation(segmentation),
            emissions,
            options,
            Some(PreparedHsmmDurations::Owned(self)),
        )
    }

    /// Reuse owned duration and incoming jump caches without copying or rebuilding them.
    pub fn forward_hsmm_with_graph(
        &self,
        graph: TransitionGraphView<'_, T>,
        emissions: &LogEmissionTable<T>,
        options: HsmmSettings<T>,
    ) -> Result<HsmmScoreTables<T>, ModelError> {
        Ok(self
            .model()
            .search_hsmm(
                TransitionGraphInput::Cached(graph),
                emissions,
                options,
                false,
                Some(PreparedHsmmDurations::Owned(self)),
            )?
            .scores)
    }

    /// Reuse owned duration caches and immutable outgoing jumps.
    pub fn backward_hsmm_with_graph(
        &self,
        graph: TransitionGraphView<'_, T>,
        emissions: &LogEmissionTable<T>,
        options: HsmmSettings<T>,
    ) -> Result<HsmmScoreTables<T>, ModelError> {
        self.model().backward_hsmm_with(
            TransitionGraphInput::Cached(graph),
            emissions,
            options,
            Some(PreparedHsmmDurations::Owned(self)),
        )
    }

    /// Reuse owned duration and incoming jump caches with original Viterbi path rules.
    pub fn viterbi_hsmm_with_graph(
        &self,
        graph: TransitionGraphView<'_, T>,
        emissions: &LogEmissionTable<T>,
        options: HsmmSettings<T>,
    ) -> Result<HsmmPath<T>, ModelError> {
        self.model().viterbi_hsmm_with(
            TransitionGraphInput::Cached(graph),
            emissions,
            options,
            Some(PreparedHsmmDurations::Owned(self)),
        )
    }
}

/// Original binary32 explicit-duration inference settings.
pub type HsmmOptions = HsmmSettings<f32>;
/// Original binary32 HSMM score tables.
pub type HsmmScores = HsmmScoreTables<f32>;
/// Original binary32 HSMM alignment.
pub type HsmmAlignment = HsmmPath<f32>;
