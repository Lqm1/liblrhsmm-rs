//! Left-to-right HMM inference with geometric state durations.
use crate::{
    EmissionTable, Model, ModelError, Segmentation,
    numerics::{LOG_ZERO, log_add},
};

/// Per-call configuration replaces the upstream process-global settings.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeometricOptions {
    pub temperature: f32,
    pub pruning_slope: f32,
}
impl Default for GeometricOptions {
    fn default() -> Self {
        Self {
            temperature: 1.0,
            pruning_slope: 0.3,
        }
    }
}

/// Time-major forward or backward log scores.
#[derive(Debug, Clone, PartialEq)]
pub struct StateScores {
    frames: usize,
    states: usize,
    values: Vec<f32>,
}
impl StateScores {
    pub fn frames(&self) -> usize {
        self.frames
    }
    pub fn states(&self) -> usize {
        self.states
    }
    pub fn values(&self) -> &[f32] {
        &self.values
    }
    pub fn get(&self, frame: usize, state: usize) -> Option<f32> {
        if frame >= self.frames || state >= self.states {
            return None;
        }
        Some(self.values[frame * self.states + state])
    }
    /// Sum the final forward row across all possible ending states.
    pub fn total_log_probability(&self) -> f32 {
        self.values[(self.frames - 1) * self.states..]
            .iter()
            .copied()
            .fold(LOG_ZERO, log_add)
    }

    /// Sum forward/backward products over time for a selected state.
    pub fn total_through_state(&self, backward: &Self, state: usize) -> Result<f32, ModelError> {
        if self.frames != backward.frames || self.states != backward.states || state >= self.states
        {
            return Err(ModelError(
                "forward/backward dimensions or state index do not match",
            ));
        }
        Ok((0..self.frames).fold(LOG_ZERO, |total, frame| {
            let index = frame * self.states + state;
            log_add(total, self.values[index] + backward.values[index])
        }))
    }

    /// Recover the original all-final-states likelihood from a backward table.
    pub fn backward_log_probability(&self, emissions: &EmissionTable) -> Result<f32, ModelError> {
        if self.frames != emissions.frames() || self.states != emissions.states() {
            return Err(ModelError("backward and emission dimensions do not match"));
        }
        Ok(self.values[0] + emissions.values()[0])
    }
}

/// Best frame-level state sequence and right segment boundaries.
#[derive(Debug, Clone, PartialEq)]
pub struct GeometricAlignment {
    pub states: Vec<usize>,
    pub boundaries: Vec<usize>,
    pub log_probability: f32,
    pub scores: StateScores,
}

struct Search<'a> {
    emissions: &'a EmissionTable,
    stay: Vec<f64>,
    advance: Vec<f64>,
    radius: usize,
}
impl<'a> Search<'a> {
    fn new(
        model: &Model,
        segmentation: &Segmentation,
        emissions: &'a EmissionTable,
        options: GeometricOptions,
    ) -> Result<Self, ModelError> {
        segmentation.validate()?;
        if emissions.states() != segmentation.duration_states.len() {
            return Err(ModelError(
                "emission state count differs from the segmentation",
            ));
        }
        if !options.temperature.is_finite()
            || options.temperature < 0.0
            || !options.pruning_slope.is_finite()
            || options.pruning_slope < 0.0
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
            if !duration.mean.is_finite() {
                return Err(ModelError("duration mean must be finite"));
            }
            let denominator = f64::from(duration.mean).max(1.01);
            let stay_probability = (1.0 - 1.0 / denominator) as f32;
            let advance_probability = (1.0 / denominator) as f32;
            stay.push(f64::from(stay_probability).ln() * f64::from(options.temperature));
            advance.push(f64::from(advance_probability).ln() * f64::from(options.temperature));
        }
        Ok(Self {
            emissions,
            stay,
            advance,
            radius: (options.pruning_slope * emissions.states() as f32) as usize,
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
    fn empty_scores(&self) -> Result<StateScores, ModelError> {
        let mut values = Vec::new();
        values
            .try_reserve_exact(self.emissions.values().len())
            .map_err(|_| ModelError("inference allocation failed"))?;
        values.resize(self.emissions.values().len(), LOG_ZERO);
        Ok(StateScores {
            frames: self.emissions.frames(),
            states: self.emissions.states(),
            values,
        })
    }
    fn forward(&self, viterbi: bool) -> Result<(StateScores, Vec<usize>), ModelError> {
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
                let same = (f64::from(scores.values[previous]) + self.stay[state]) as f32;
                let mut best = LOG_ZERO;
                let mut from = usize::MAX;
                if viterbi {
                    if same > best {
                        best = same;
                        from = state;
                    }
                } else {
                    best = log_add(best, same);
                }
                if state > 0 {
                    let next =
                        (f64::from(scores.values[previous - 1]) + self.advance[state - 1]) as f32;
                    if viterbi {
                        if next > best {
                            best = next;
                            from = state - 1;
                        }
                    } else {
                        best = log_add(best, next);
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

impl Model {
    pub fn forward_geometric(
        &self,
        segmentation: &Segmentation,
        emissions: &EmissionTable,
        options: GeometricOptions,
    ) -> Result<StateScores, ModelError> {
        Ok(Search::new(self, segmentation, emissions, options)?
            .forward(false)?
            .0)
    }

    /// The original backward initialization permits every final state.
    pub fn backward_geometric(
        &self,
        segmentation: &Segmentation,
        emissions: &EmissionTable,
        options: GeometricOptions,
    ) -> Result<StateScores, ModelError> {
        let search = Search::new(self, segmentation, emissions, options)?;
        let mut scores = search.empty_scores()?;
        let final_row = (scores.frames - 1) * scores.states;
        scores.values[final_row..].fill(0.0);
        for frame in (0..scores.frames - 1).rev() {
            for state in search.band(frame) {
                let next = (frame + 1) * scores.states + state;
                let mut density = (f64::from(scores.values[next] + emissions.values()[next])
                    + search.stay[state]) as f32;
                if state + 1 < scores.states {
                    let advance =
                        (f64::from(scores.values[next + 1] + emissions.values()[next + 1])
                            + search.advance[state]) as f32;
                    density = log_add(density, advance);
                }
                scores.values[frame * scores.states + state] = density;
            }
        }
        Ok(scores)
    }

    /// Require the best path to start in state zero and finish in the last state.
    pub fn viterbi_geometric(
        &self,
        segmentation: &Segmentation,
        emissions: &EmissionTable,
        options: GeometricOptions,
    ) -> Result<GeometricAlignment, ModelError> {
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
        Ok(GeometricAlignment {
            states,
            boundaries,
            log_probability,
            scores,
        })
    }
}
