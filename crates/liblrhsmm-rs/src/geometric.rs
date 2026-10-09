//! Left-to-right HMM inference with geometric state durations.
use crate::ModelError;
use crate::data::SegmentationData;
use crate::emission::LogEmissionTable;
use crate::model::ModelParameters;
use crate::scalar::Scalar;

/// Per-call configuration replaces the upstream process-global settings.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeometricSettings<T> {
    pub temperature: T,
    pub pruning_slope: T,
}
/// Original binary32 geometric inference settings.
pub type GeometricOptions = GeometricSettings<f32>;
impl<T: Scalar> Default for GeometricSettings<T> {
    fn default() -> Self {
        Self {
            temperature: T::ONE,
            pruning_slope: T::from_f64(0.3),
        }
    }
}

/// Time-major forward or backward log scores.
#[derive(Debug, Clone, PartialEq)]
pub struct StateScoreTable<T> {
    pub(crate) frames: usize,
    pub(crate) states: usize,
    pub(crate) values: Vec<T>,
}
/// Original binary32 state-score API.
pub type StateScores = StateScoreTable<f32>;
impl<T: Scalar> StateScoreTable<T> {
    /// Import a complete time-major score table, preserving every stored scalar bit.
    pub fn from_values(frames: usize, states: usize, values: Vec<T>) -> Result<Self, ModelError> {
        if frames == 0 || states == 0 || frames.checked_mul(states) != Some(values.len()) {
            return Err(ModelError(
                "state score table requires positive matching dimensions",
            ));
        }
        Ok(Self {
            frames,
            states,
            values,
        })
    }

    pub fn frames(&self) -> usize {
        self.frames
    }
    pub fn states(&self) -> usize {
        self.states
    }
    pub fn values(&self) -> &[T] {
        &self.values
    }
    pub fn get(&self, frame: usize, state: usize) -> Option<T> {
        if frame >= self.frames || state >= self.states {
            return None;
        }
        Some(self.values[frame * self.states + state])
    }
    /// Sum the final forward row across all possible ending states.
    pub fn total_log_probability(&self) -> T {
        self.values[(self.frames - 1) * self.states..]
            .iter()
            .copied()
            .fold(T::from_f64(-1e15), T::log_add)
    }

    /// Sum forward/backward products over time for a selected state.
    pub fn total_through_state(&self, backward: &Self, state: usize) -> Result<T, ModelError> {
        if self.frames != backward.frames || self.states != backward.states || state >= self.states
        {
            return Err(ModelError(
                "forward/backward dimensions or state index do not match",
            ));
        }
        Ok((0..self.frames).fold(T::from_f64(-1e15), |total, frame| {
            let index = frame * self.states + state;
            total.log_add(self.values[index] + backward.values[index])
        }))
    }

    /// Recover the original all-final-states likelihood from a backward table.
    pub fn backward_log_probability(
        &self,
        emissions: &LogEmissionTable<T>,
    ) -> Result<T, ModelError> {
        if self.frames != emissions.frames() || self.states != emissions.states() {
            return Err(ModelError("backward and emission dimensions do not match"));
        }
        Ok(self.values[0] + emissions.values()[0])
    }
}

/// Best frame-level state sequence and right segment boundaries.
#[derive(Debug, Clone, PartialEq)]
pub struct GeometricPath<T> {
    pub states: Vec<usize>,
    pub boundaries: Vec<usize>,
    pub log_probability: T,
    pub scores: StateScoreTable<T>,
}
/// Original binary32 geometric alignment API.
pub type GeometricAlignment = GeometricPath<f32>;

struct Search<'a, T> {
    emissions: &'a LogEmissionTable<T>,
    stay: Vec<f64>,
    advance: Vec<f64>,
    radius: usize,
}
impl<'a, T: Scalar> Search<'a, T> {
    fn new(
        model: &ModelParameters<T>,
        segmentation: &SegmentationData<T>,
        emissions: &'a LogEmissionTable<T>,
        options: GeometricSettings<T>,
    ) -> Result<Self, ModelError> {
        segmentation.validate()?;
        if emissions.states() != segmentation.duration_states.len() {
            return Err(ModelError(
                "emission state count differs from the segmentation",
            ));
        }
        if !options.temperature.into().is_finite()
            || options.temperature < T::ZERO
            || !options.pruning_slope.into().is_finite()
            || options.pruning_slope < T::ZERO
        {
            return Err(ModelError(
                "inference temperature and pruning slope must be finite and nonnegative",
            ));
        }
        let mut stay = Vec::with_capacity(emissions.states());
        let mut advance = Vec::with_capacity(emissions.states());
        for &state in &segmentation.duration_states {
            let duration = usize::try_from(state)
                .ok()
                .and_then(|state| model.durations.get(state))
                .ok_or(ModelError("duration state is out of range"))?;
            if !duration.mean.into().is_finite() {
                return Err(ModelError("duration mean must be finite"));
            }
            let denominator = duration.mean.into().max(1.01);
            let stay_probability = T::from_f64(1.0 - 1.0 / denominator);
            let advance_probability = T::from_f64(1.0 / denominator);
            stay.push(stay_probability.into().ln() * options.temperature.into());
            advance.push(advance_probability.into().ln() * options.temperature.into());
        }
        Ok(Self {
            emissions,
            stay,
            advance,
            radius: (options.pruning_slope * T::from_f64(emissions.states() as f64)).into()
                as usize,
        })
    }
    fn band(&self, frame: usize) -> std::ops::Range<usize> {
        let center = ((frame as u128 * self.emissions.states() as u128)
            / self.emissions.frames() as u128) as usize;
        center.saturating_sub(self.radius)
            ..center
                .saturating_add(self.radius)
                .min(self.emissions.states())
    }
    fn empty_scores(&self) -> Result<StateScoreTable<T>, ModelError> {
        let mut values = Vec::new();
        values
            .try_reserve_exact(self.emissions.values().len())
            .map_err(|_| ModelError("inference allocation failed"))?;
        values.resize(self.emissions.values().len(), T::from_f64(-1e15));
        Ok(StateScoreTable {
            frames: self.emissions.frames(),
            states: self.emissions.states(),
            values,
        })
    }
    fn forward(&self, viterbi: bool) -> Result<(StateScoreTable<T>, Vec<usize>), ModelError> {
        let mut scores = self.empty_scores()?;
        let mut predecessors = if viterbi {
            vec![usize::MAX; scores.values.len()]
        } else {
            Vec::new()
        };
        scores.values[0] = self.emissions.values()[0];
        for frame in 1..scores.frames {
            for state in self.band(frame) {
                let previous = (frame - 1) * scores.states + state;
                let same = T::from_f64(scores.values[previous].into() + self.stay[state]);
                let mut best = T::from_f64(-1e15);
                let mut from = usize::MAX;
                if viterbi {
                    if same > best {
                        best = same;
                        from = state;
                    }
                } else {
                    best = best.log_add(same);
                }
                if state > 0 {
                    let next =
                        T::from_f64(scores.values[previous - 1].into() + self.advance[state - 1]);
                    if viterbi {
                        if next > best {
                            best = next;
                            from = state - 1;
                        }
                    } else {
                        best = best.log_add(next);
                    }
                }
                let index = frame * scores.states + state;
                scores.values[index] = best + self.emissions.values()[index];
                if viterbi {
                    predecessors[index] = from;
                }
            }
        }
        Ok((scores, predecessors))
    }
}

impl<T: Scalar> ModelParameters<T> {
    pub fn forward_geometric(
        &self,
        segmentation: &SegmentationData<T>,
        emissions: &LogEmissionTable<T>,
        options: GeometricSettings<T>,
    ) -> Result<StateScoreTable<T>, ModelError> {
        Ok(Search::new(self, segmentation, emissions, options)?
            .forward(false)?
            .0)
    }

    /// The original backward initialization permits every final state.
    pub fn backward_geometric(
        &self,
        segmentation: &SegmentationData<T>,
        emissions: &LogEmissionTable<T>,
        options: GeometricSettings<T>,
    ) -> Result<StateScoreTable<T>, ModelError> {
        let search = Search::new(self, segmentation, emissions, options)?;
        let mut scores = search.empty_scores()?;
        let final_row = (scores.frames - 1) * scores.states;
        scores.values[final_row..].fill(T::ZERO);
        for frame in (0..scores.frames - 1).rev() {
            for state in search.band(frame) {
                let next = (frame + 1) * scores.states + state;
                let mut density = T::from_f64(
                    (scores.values[next] + emissions.values()[next]).into() + search.stay[state],
                );
                if state + 1 < scores.states {
                    let advance = T::from_f64(
                        (scores.values[next + 1] + emissions.values()[next + 1]).into()
                            + search.advance[state],
                    );
                    density = density.log_add(advance);
                }
                scores.values[frame * scores.states + state] = density;
            }
        }
        Ok(scores)
    }

    /// Require the best path to start in state zero and finish in the last state.
    pub fn viterbi_geometric(
        &self,
        segmentation: &SegmentationData<T>,
        emissions: &LogEmissionTable<T>,
        options: GeometricSettings<T>,
    ) -> Result<GeometricPath<T>, ModelError> {
        let (scores, predecessors) =
            Search::new(self, segmentation, emissions, options)?.forward(true)?;
        let mut states = vec![0; scores.frames];
        states[scores.frames - 1] = scores.states - 1;
        for frame in (1..scores.frames).rev() {
            let predecessor = predecessors[frame * scores.states + states[frame]];
            if predecessor == usize::MAX {
                return Err(ModelError(
                    "no complete path survives the inference constraints",
                ));
            }
            states[frame - 1] = predecessor;
        }
        if states[0] != 0 {
            return Err(ModelError("best path does not begin in the first state"));
        }
        let mut boundaries = vec![0; scores.states];
        for frame in 1..scores.frames {
            if states[frame] != states[frame - 1] {
                boundaries[states[frame] - 1] = frame;
            }
        }
        boundaries[scores.states - 1] = scores.frames;
        let log_probability = scores.values[scores.values.len() - 1];
        Ok(GeometricPath {
            states,
            boundaries,
            log_probability,
            scores,
        })
    }
}
