fn new_err_owned(message: &str) -> pyo3::PyErr {
    pyo3::exceptions::PyValueError::new_err(message.to_owned())
}
use crate::duration_occupancy::{
    DurationOccupancies, DurationPosterior as NativeDurationPosterior,
};
use crate::mixture_occupancy::MixtureOccupancies;
use crate::occupancy::{TransitionOccupancies, TransitionPosterior as NativeTransitionPosterior};
use pyo3::PyErr as BindingError;
fn error(value: impl std::fmt::Display) -> BindingError {
    new_err_owned(&value.to_string())
}
fn shape(frames: usize, states: usize, presence: &[u8]) -> Result<(), BindingError> {
    if frames == 0 || states == 0 || frames.checked_mul(states) != Some(presence.len()) {
        return Err(error(
            "posterior dimensions must be positive and match presence entries",
        ));
    }
    if presence.iter().any(|&value| value > 1) {
        return Err(error("posterior presence entries must be zero or one"));
    }
    Ok(())
}
fn slices<'a>(
    presence: &[u8],
    offsets: &[u32],
    values: &'a [f32],
) -> Result<Vec<Option<&'a [f32]>>, BindingError> {
    if presence.len().checked_add(1) != Some(offsets.len())
        || offsets.first() != Some(&0)
        || offsets.last().map(|&value| value as usize) != Some(values.len())
    {
        return Err(error(
            "posterior offsets must cover every entry and the complete values",
        ));
    }
    presence
        .iter()
        .zip(offsets.windows(2))
        .map(|(&present, range)| {
            let value = values
                .get(range[0] as usize..range[1] as usize)
                .ok_or_else(|| error("posterior offsets must be ordered within values"))?;
            if present == 0 {
                if !value.is_empty() {
                    return Err(error("absent posterior entries cannot contain values"));
                }
                Ok(None)
            } else {
                Ok(Some(value))
            }
        })
        .collect()
}
#[derive(Clone)]
pub struct TransitionPosterior {
    pub(crate) inner: NativeTransitionPosterior,
}
impl TransitionPosterior {
    pub fn new(values: &[f32]) -> Result<Self, BindingError> {
        let [stay, advance] = values else {
            return Err(error("transition posterior requires stay/advance values"));
        };
        Ok(Self {
            inner: NativeTransitionPosterior {
                stay: *stay,
                advance: *advance,
            },
        })
    }
    pub fn values(&self) -> Vec<f32> {
        vec![self.inner.stay, self.inner.advance]
    }
    pub fn set_values(&mut self, values: &[f32]) -> Result<(), BindingError> {
        self.inner = Self::new(values)?.inner;
        Ok(())
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
#[derive(Clone)]
pub struct TransitionPosteriors {
    pub(crate) inner: TransitionOccupancies,
}
impl TransitionPosteriors {
    pub fn new(
        frames: usize,
        states: usize,
        presence: &[u8],
        values: &[f32],
    ) -> Result<Self, BindingError> {
        shape(frames, states, presence)?;
        if presence.len().checked_mul(2) != Some(values.len()) {
            return Err(error("transition values require two scalars per entry"));
        }
        let values = presence
            .iter()
            .zip(values.chunks_exact(2))
            .map(|(&present, pair)| {
                if present == 0 {
                    None
                } else {
                    Some(NativeTransitionPosterior {
                        stay: pair[0],
                        advance: pair[1],
                    })
                }
            })
            .collect();
        TransitionOccupancies::from_values(frames, states, values)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn frames(&self) -> usize {
        self.inner.frames()
    }
    pub fn states(&self) -> usize {
        self.inner.states()
    }
    pub fn presence(&self) -> Vec<u8> {
        self.inner
            .values()
            .iter()
            .map(|value| u8::from(value.is_some()))
            .collect()
    }
    pub fn values(&self) -> Vec<f32> {
        self.inner
            .values()
            .iter()
            .flat_map(|value| value.map_or([0.0, 0.0], |value| [value.stay, value.advance]))
            .collect()
    }
    pub fn get(&self, frame: usize, state: usize) -> Option<TransitionPosterior> {
        self.inner
            .get(frame, state)
            .map(|inner| TransitionPosterior { inner })
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
#[derive(Clone)]
pub struct DurationPosterior {
    pub(crate) inner: NativeDurationPosterior,
}
impl DurationPosterior {
    pub fn new(minimum: usize, probabilities: &[f32]) -> Self {
        Self {
            inner: NativeDurationPosterior {
                minimum,
                probabilities: probabilities.to_vec(),
            },
        }
    }
    pub fn minimum(&self) -> usize {
        self.inner.minimum
    }
    pub fn set_minimum(&mut self, value: usize) {
        self.inner.minimum = value;
    }
    pub fn probabilities(&self) -> Vec<f32> {
        self.inner.probabilities.clone()
    }
    pub fn set_probabilities(&mut self, values: &[f32]) {
        self.inner.probabilities = values.to_vec();
    }
    pub fn probability(&self, duration: usize) -> f32 {
        self.inner.probability(duration)
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
#[derive(Clone)]
pub struct DurationPosteriors {
    pub(crate) inner: DurationOccupancies,
}
impl DurationPosteriors {
    pub fn new(
        frames: usize,
        states: usize,
        presence: &[u8],
        minimums: &[u32],
        offsets: &[u32],
        values: &[f32],
    ) -> Result<Self, BindingError> {
        shape(frames, states, presence)?;
        if minimums.len() != presence.len() {
            return Err(error("duration minimum count differs from entries"));
        }
        let values = slices(presence, offsets, values)?
            .into_iter()
            .zip(minimums)
            .map(|(value, &minimum)| {
                value.map(|probabilities| NativeDurationPosterior {
                    minimum: minimum as usize,
                    probabilities: probabilities.to_vec(),
                })
            })
            .collect();
        DurationOccupancies::from_values(frames, states, values)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn frames(&self) -> usize {
        self.inner.frames()
    }
    pub fn states(&self) -> usize {
        self.inner.states()
    }
    pub fn presence(&self) -> Vec<u8> {
        self.inner
            .values()
            .iter()
            .map(|value| u8::from(value.is_some()))
            .collect()
    }
    pub fn minimums(&self) -> Vec<u32> {
        self.inner
            .values()
            .iter()
            .map(|value| value.as_ref().map_or(0, |value| value.minimum as u32))
            .collect()
    }
    pub fn offsets(&self) -> Vec<u32> {
        std::iter::once(0)
            .chain(self.inner.values().iter().scan(0u32, |offset, value| {
                *offset += value
                    .as_ref()
                    .map_or(0, |value| value.probabilities.len() as u32);
                Some(*offset)
            }))
            .collect()
    }
    pub fn values(&self) -> Vec<f32> {
        self.inner
            .values()
            .iter()
            .flatten()
            .flat_map(|value| value.probabilities.iter().copied())
            .collect()
    }
    pub fn get(&self, frame: usize, state: usize) -> Option<DurationPosterior> {
        self.inner
            .get(frame, state)
            .cloned()
            .map(|inner| DurationPosterior { inner })
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
#[derive(Clone)]
pub struct MixtureWeights {
    inner: Vec<f32>,
}
impl MixtureWeights {
    pub fn new(values: &[f32]) -> Self {
        Self {
            inner: values.to_vec(),
        }
    }
    pub fn values(&self) -> Vec<f32> {
        self.inner.clone()
    }
    pub fn replace(&mut self, values: &[f32]) {
        self.inner = values.to_vec();
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
#[derive(Clone)]
pub struct MixturePosteriors {
    pub(crate) inner: MixtureOccupancies,
}
impl MixturePosteriors {
    pub fn new(
        frames: usize,
        states: usize,
        presence: &[u8],
        offsets: &[u32],
        values: &[f32],
    ) -> Result<Self, BindingError> {
        shape(frames, states, presence)?;
        let values = slices(presence, offsets, values)?
            .into_iter()
            .map(|value| value.map(<[f32]>::to_vec))
            .collect();
        MixtureOccupancies::from_values(frames, states, values)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn frames(&self) -> usize {
        self.inner.frames()
    }
    pub fn states(&self) -> usize {
        self.inner.states()
    }
    pub fn presence(&self) -> Vec<u8> {
        self.inner
            .values()
            .iter()
            .map(|value| u8::from(value.is_some()))
            .collect()
    }
    pub fn offsets(&self) -> Vec<u32> {
        std::iter::once(0)
            .chain(self.inner.values().iter().scan(0u32, |offset, value| {
                *offset += value.as_ref().map_or(0, |value| value.len() as u32);
                Some(*offset)
            }))
            .collect()
    }
    pub fn values(&self) -> Vec<f32> {
        self.inner
            .values()
            .iter()
            .flatten()
            .flatten()
            .copied()
            .collect()
    }
    pub fn get(&self, frame: usize, state: usize) -> Option<MixtureWeights> {
        self.inner.get(frame, state).map(|value| MixtureWeights {
            inner: value.to_vec(),
        })
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
