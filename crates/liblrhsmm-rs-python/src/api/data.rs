fn new_err_owned(message: &str) -> pyo3::PyErr {
    pyo3::exceptions::PyValueError::new_err(message.to_owned())
}
use crate::StateOccurrence;
use crate::data::{JumpParameters, ObservationData, ObservationStreamData, SegmentationData};
use crate::transitions::OwnedTransitionGraphParameters;
use pyo3::PyErr as BindingError;
fn error(value: impl std::fmt::Display) -> BindingError {
    new_err_owned(&value.to_string())
}
#[derive(Clone, Default)]
pub struct ObservationStreamCollectionF32 {
    pub(crate) inner: Vec<ObservationStreamData<f32>>,
}
impl ObservationStreamCollectionF32 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn length(&self) -> usize {
        self.inner.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn push(&mut self, value: &ObservationStreamF32) {
        self.inner.push(value.inner.clone());
    }
    pub fn get(&self, index: usize) -> Option<ObservationStreamF32> {
        self.inner
            .get(index)
            .cloned()
            .map(|inner| ObservationStreamF32 { inner })
    }
    pub fn replace(
        &mut self,
        index: usize,
        value: &ObservationStreamF32,
    ) -> Result<(), BindingError> {
        let target = self
            .inner
            .get_mut(index)
            .ok_or_else(|| error("collection index out of range"))?;
        *target = value.inner.clone();
        Ok(())
    }
    pub fn clear(&mut self) {
        self.inner.clear();
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
#[derive(Clone, Default)]
pub struct ObservationCollectionF32 {
    pub(crate) inner: Vec<ObservationData<f32>>,
}
impl ObservationCollectionF32 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn length(&self) -> usize {
        self.inner.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn push(&mut self, value: &ObservationF32) {
        self.inner.push(value.inner.clone());
    }
    pub fn get(&self, index: usize) -> Option<ObservationF32> {
        self.inner
            .get(index)
            .cloned()
            .map(|inner| ObservationF32 { inner })
    }
    pub fn replace(&mut self, index: usize, value: &ObservationF32) -> Result<(), BindingError> {
        let target = self
            .inner
            .get_mut(index)
            .ok_or_else(|| error("collection index out of range"))?;
        *target = value.inner.clone();
        Ok(())
    }
    pub fn clear(&mut self) {
        self.inner.clear();
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
#[derive(Clone, Default)]
pub struct SegmentationCollectionF32 {
    pub(crate) inner: Vec<SegmentationData<f32>>,
}
impl SegmentationCollectionF32 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn length(&self) -> usize {
        self.inner.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn push(&mut self, value: &SegmentationF32) {
        self.inner.push(value.inner.clone());
    }
    pub fn get(&self, index: usize) -> Option<SegmentationF32> {
        self.inner
            .get(index)
            .cloned()
            .map(|inner| SegmentationF32 { inner })
    }
    pub fn replace(&mut self, index: usize, value: &SegmentationF32) -> Result<(), BindingError> {
        let target = self
            .inner
            .get_mut(index)
            .ok_or_else(|| error("collection index out of range"))?;
        *target = value.inner.clone();
        Ok(())
    }
    pub fn clear(&mut self) {
        self.inner.clear();
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
#[derive(Clone)]
pub struct ObservationStreamF32 {
    pub(crate) inner: ObservationStreamData<f32>,
}
impl ObservationStreamF32 {
    pub fn new(dimensions: usize, values: &[f32]) -> Self {
        Self {
            inner: ObservationStreamData {
                dimensions,
                values: values.to_vec(),
            },
        }
    }
    pub fn dimensions(&self) -> usize {
        self.inner.dimensions
    }
    pub fn values(&self) -> Vec<f32> {
        self.inner.values.clone()
    }
    pub fn replace(&mut self, dimensions: usize, values: &[f32]) {
        self.inner = ObservationStreamData {
            dimensions,
            values: values.to_vec(),
        };
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
#[derive(Clone)]
pub struct ObservationF32 {
    pub(crate) inner: ObservationData<f32>,
}
impl ObservationF32 {
    pub fn new(frames: usize, dimensions: &[u32]) -> Result<Self, BindingError> {
        let dimensions = dimensions
            .iter()
            .map(|value| *value as usize)
            .collect::<Vec<_>>();
        ObservationData::new(frames, &dimensions)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn frames(&self) -> usize {
        self.inner.frames
    }
    pub fn streams(&self) -> usize {
        self.inner.streams.len()
    }
    pub fn stream(&self, index: usize) -> Option<ObservationStreamF32> {
        self.inner
            .streams
            .get(index)
            .cloned()
            .map(|inner| ObservationStreamF32 { inner })
    }
    pub fn set_stream(
        &mut self,
        index: usize,
        value: &ObservationStreamF32,
    ) -> Result<(), BindingError> {
        let mut inner = self.inner.clone();
        *inner
            .streams
            .get_mut(index)
            .ok_or_else(|| error("observation stream index out of range"))? = value.inner.clone();
        inner.validate().map_err(error)?;
        self.inner = inner;
        Ok(())
    }
    pub fn replace(
        &mut self,
        frames: usize,
        streams: &ObservationStreamCollectionF32,
    ) -> Result<(), BindingError> {
        let inner = ObservationData {
            frames,
            streams: streams.inner.clone(),
        };
        inner.validate().map_err(error)?;
        self.inner = inner;
        Ok(())
    }
    pub fn frame(&self, stream: usize, time: usize) -> Option<Vec<f32>> {
        self.inner.frame(stream, time).map(<[f32]>::to_vec)
    }
    pub fn validate(&self) -> Result<(), BindingError> {
        self.inner.validate().map_err(error)
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
#[derive(Clone)]
pub struct JumpGroupF32 {
    inner: Vec<JumpParameters<f32>>,
}
impl JumpGroupF32 {
    pub fn new(deltas: &[i32], probabilities: &[f32]) -> Result<Self, BindingError> {
        if deltas.len() != probabilities.len() {
            return Err(error("jump deltas and probabilities differ"));
        }
        Ok(Self {
            inner: deltas
                .iter()
                .zip(probabilities)
                .map(|(&delta, &probability)| JumpParameters { delta, probability })
                .collect(),
        })
    }
    pub fn length(&self) -> usize {
        self.inner.len()
    }
    pub fn deltas(&self) -> Vec<i32> {
        self.inner.iter().map(|value| value.delta).collect()
    }
    pub fn probabilities(&self) -> Vec<f32> {
        self.inner.iter().map(|value| value.probability).collect()
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
#[derive(Clone)]
pub struct SegmentationF32 {
    pub(crate) inner: SegmentationData<f32>,
}
impl SegmentationF32 {
    pub fn new(streams: usize, segments: usize) -> Result<Self, BindingError> {
        if streams > i32::MAX as usize
            || segments > i32::MAX as usize
            || streams > isize::MAX as usize / size_of::<Vec<i32>>()
            || segments > isize::MAX as usize / size_of::<Vec<i32>>()
        {
            return Err(error("segmentation shape exceeds the legacy format"));
        }
        Ok(Self {
            inner: SegmentationData::new(streams, segments),
        })
    }
    pub fn streams(&self) -> usize {
        self.inner.output_states.len()
    }
    pub fn segments(&self) -> usize {
        self.inner.boundaries.len()
    }
    pub fn boundaries(&self) -> Vec<i32> {
        self.inner.boundaries.clone()
    }
    pub fn duration_states(&self) -> Vec<i32> {
        self.inner.duration_states.clone()
    }
    pub fn output_states(&self, stream: usize) -> Result<Vec<i32>, BindingError> {
        self.inner
            .output_states
            .get(stream)
            .cloned()
            .ok_or_else(|| error("output stream index out of range"))
    }
    pub fn set_boundaries(&mut self, values: &[i32]) -> Result<(), BindingError> {
        if values.len() != self.segments() {
            return Err(error("boundary count differs"));
        }
        self.inner.boundaries.copy_from_slice(values);
        Ok(())
    }
    pub fn set_duration_states(&mut self, values: &[i32]) -> Result<(), BindingError> {
        if values.len() != self.segments() {
            return Err(error("duration state count differs"));
        }
        self.inner.duration_states.copy_from_slice(values);
        Ok(())
    }
    pub fn set_output_states(&mut self, stream: usize, values: &[i32]) -> Result<(), BindingError> {
        let target = self
            .inner
            .output_states
            .get_mut(stream)
            .ok_or_else(|| error("output stream index out of range"))?;
        if target.len() != values.len() {
            return Err(error("output state count differs"));
        }
        target.copy_from_slice(values);
        Ok(())
    }
    pub fn outgoing(&self, state: usize) -> Option<JumpGroupF32> {
        self.inner
            .outgoing
            .get(state)
            .cloned()
            .map(|inner| JumpGroupF32 { inner })
    }
    pub fn set_outgoing(&mut self, state: usize, jumps: &JumpGroupF32) -> Result<(), BindingError> {
        let mut inner = self.inner.clone();
        *inner
            .outgoing
            .get_mut(state)
            .ok_or_else(|| error("jump state index out of range"))? = jumps.inner.clone();
        inner.validate().map_err(error)?;
        self.inner = inner;
        Ok(())
    }
    pub fn replace(&mut self, value: &SegmentationF32) {
        self.inner = value.inner.clone();
    }
    pub fn validate(&self) -> Result<(), BindingError> {
        self.inner.validate().map_err(error)
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
    pub fn add_jump(
        &mut self,
        state: usize,
        delta: i32,
        probability: f32,
    ) -> Result<(), BindingError> {
        self.inner
            .add_jump(state, delta, probability)
            .map_err(error)
    }
    pub fn build_transitions(&self) -> Result<TransitionGraphF32, BindingError> {
        self.cloned().build_transitions_owned()
    }
    pub fn build_transitions_owned(self) -> Result<TransitionGraphF32, BindingError> {
        self.inner
            .build_transitions_owned()
            .map(|inner| TransitionGraphF32 { inner })
            .map_err(error)
    }
    pub fn resegment(&self, occurrences: &[u32]) -> Result<Self, BindingError> {
        if !occurrences.len().is_multiple_of(2) {
            return Err(error("state occurrences require state/end pairs"));
        }
        let occurrences = occurrences
            .chunks_exact(2)
            .map(|value| StateOccurrence {
                state: value[0] as usize,
                end: value[1] as usize,
            })
            .collect::<Vec<_>>();
        self.inner
            .resegment(&occurrences)
            .map(|inner| Self { inner })
            .map_err(error)
    }
}
#[derive(Clone)]
pub struct TransitionGraphF32 {
    pub(crate) inner: OwnedTransitionGraphParameters<f32>,
}
impl TransitionGraphF32 {
    pub fn cloned(&self) -> Self {
        self.clone()
    }
    pub fn segmentation(&self) -> SegmentationF32 {
        SegmentationF32 {
            inner: self.inner.segmentation().clone(),
        }
    }
    pub fn into_segmentation(self) -> SegmentationF32 {
        SegmentationF32 {
            inner: self.inner.into_segmentation(),
        }
    }
    pub fn incoming(&self, state: usize) -> Option<JumpGroupF32> {
        self.inner
            .incoming()
            .get(state)
            .cloned()
            .map(|inner| JumpGroupF32 { inner })
    }
    pub fn outgoing(&self, state: usize) -> Option<JumpGroupF32> {
        self.inner
            .outgoing()
            .get(state)
            .cloned()
            .map(|inner| JumpGroupF32 { inner })
    }
}
#[derive(Clone, Default)]
pub struct ObservationStreamCollectionF64 {
    pub(crate) inner: Vec<ObservationStreamData<f64>>,
}
impl ObservationStreamCollectionF64 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn length(&self) -> usize {
        self.inner.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn push(&mut self, value: &ObservationStreamF64) {
        self.inner.push(value.inner.clone());
    }
    pub fn get(&self, index: usize) -> Option<ObservationStreamF64> {
        self.inner
            .get(index)
            .cloned()
            .map(|inner| ObservationStreamF64 { inner })
    }
    pub fn replace(
        &mut self,
        index: usize,
        value: &ObservationStreamF64,
    ) -> Result<(), BindingError> {
        let target = self
            .inner
            .get_mut(index)
            .ok_or_else(|| error("collection index out of range"))?;
        *target = value.inner.clone();
        Ok(())
    }
    pub fn clear(&mut self) {
        self.inner.clear();
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
#[derive(Clone, Default)]
pub struct ObservationCollectionF64 {
    pub(crate) inner: Vec<ObservationData<f64>>,
}
impl ObservationCollectionF64 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn length(&self) -> usize {
        self.inner.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn push(&mut self, value: &ObservationF64) {
        self.inner.push(value.inner.clone());
    }
    pub fn get(&self, index: usize) -> Option<ObservationF64> {
        self.inner
            .get(index)
            .cloned()
            .map(|inner| ObservationF64 { inner })
    }
    pub fn replace(&mut self, index: usize, value: &ObservationF64) -> Result<(), BindingError> {
        let target = self
            .inner
            .get_mut(index)
            .ok_or_else(|| error("collection index out of range"))?;
        *target = value.inner.clone();
        Ok(())
    }
    pub fn clear(&mut self) {
        self.inner.clear();
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
#[derive(Clone, Default)]
pub struct SegmentationCollectionF64 {
    pub(crate) inner: Vec<SegmentationData<f64>>,
}
impl SegmentationCollectionF64 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn length(&self) -> usize {
        self.inner.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn push(&mut self, value: &SegmentationF64) {
        self.inner.push(value.inner.clone());
    }
    pub fn get(&self, index: usize) -> Option<SegmentationF64> {
        self.inner
            .get(index)
            .cloned()
            .map(|inner| SegmentationF64 { inner })
    }
    pub fn replace(&mut self, index: usize, value: &SegmentationF64) -> Result<(), BindingError> {
        let target = self
            .inner
            .get_mut(index)
            .ok_or_else(|| error("collection index out of range"))?;
        *target = value.inner.clone();
        Ok(())
    }
    pub fn clear(&mut self) {
        self.inner.clear();
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
#[derive(Clone)]
pub struct ObservationStreamF64 {
    pub(crate) inner: ObservationStreamData<f64>,
}
impl ObservationStreamF64 {
    pub fn new(dimensions: usize, values: &[f64]) -> Self {
        Self {
            inner: ObservationStreamData {
                dimensions,
                values: values.to_vec(),
            },
        }
    }
    pub fn dimensions(&self) -> usize {
        self.inner.dimensions
    }
    pub fn values(&self) -> Vec<f64> {
        self.inner.values.clone()
    }
    pub fn replace(&mut self, dimensions: usize, values: &[f64]) {
        self.inner = ObservationStreamData {
            dimensions,
            values: values.to_vec(),
        };
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
#[derive(Clone)]
pub struct ObservationF64 {
    pub(crate) inner: ObservationData<f64>,
}
impl ObservationF64 {
    pub fn new(frames: usize, dimensions: &[u32]) -> Result<Self, BindingError> {
        let dimensions = dimensions
            .iter()
            .map(|value| *value as usize)
            .collect::<Vec<_>>();
        ObservationData::new(frames, &dimensions)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn frames(&self) -> usize {
        self.inner.frames
    }
    pub fn streams(&self) -> usize {
        self.inner.streams.len()
    }
    pub fn stream(&self, index: usize) -> Option<ObservationStreamF64> {
        self.inner
            .streams
            .get(index)
            .cloned()
            .map(|inner| ObservationStreamF64 { inner })
    }
    pub fn set_stream(
        &mut self,
        index: usize,
        value: &ObservationStreamF64,
    ) -> Result<(), BindingError> {
        let mut inner = self.inner.clone();
        *inner
            .streams
            .get_mut(index)
            .ok_or_else(|| error("observation stream index out of range"))? = value.inner.clone();
        inner.validate().map_err(error)?;
        self.inner = inner;
        Ok(())
    }
    pub fn replace(
        &mut self,
        frames: usize,
        streams: &ObservationStreamCollectionF64,
    ) -> Result<(), BindingError> {
        let inner = ObservationData {
            frames,
            streams: streams.inner.clone(),
        };
        inner.validate().map_err(error)?;
        self.inner = inner;
        Ok(())
    }
    pub fn frame(&self, stream: usize, time: usize) -> Option<Vec<f64>> {
        self.inner.frame(stream, time).map(<[f64]>::to_vec)
    }
    pub fn validate(&self) -> Result<(), BindingError> {
        self.inner.validate().map_err(error)
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
#[derive(Clone)]
pub struct JumpGroupF64 {
    inner: Vec<JumpParameters<f64>>,
}
impl JumpGroupF64 {
    pub fn new(deltas: &[i32], probabilities: &[f64]) -> Result<Self, BindingError> {
        if deltas.len() != probabilities.len() {
            return Err(error("jump deltas and probabilities differ"));
        }
        Ok(Self {
            inner: deltas
                .iter()
                .zip(probabilities)
                .map(|(&delta, &probability)| JumpParameters { delta, probability })
                .collect(),
        })
    }
    pub fn length(&self) -> usize {
        self.inner.len()
    }
    pub fn deltas(&self) -> Vec<i32> {
        self.inner.iter().map(|value| value.delta).collect()
    }
    pub fn probabilities(&self) -> Vec<f64> {
        self.inner.iter().map(|value| value.probability).collect()
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
}
#[derive(Clone)]
pub struct SegmentationF64 {
    pub(crate) inner: SegmentationData<f64>,
}
impl SegmentationF64 {
    pub fn new(streams: usize, segments: usize) -> Result<Self, BindingError> {
        if streams > i32::MAX as usize
            || segments > i32::MAX as usize
            || streams > isize::MAX as usize / size_of::<Vec<i32>>()
            || segments > isize::MAX as usize / size_of::<Vec<i32>>()
        {
            return Err(error("segmentation shape exceeds the legacy format"));
        }
        Ok(Self {
            inner: SegmentationData::new(streams, segments),
        })
    }
    pub fn streams(&self) -> usize {
        self.inner.output_states.len()
    }
    pub fn segments(&self) -> usize {
        self.inner.boundaries.len()
    }
    pub fn boundaries(&self) -> Vec<i32> {
        self.inner.boundaries.clone()
    }
    pub fn duration_states(&self) -> Vec<i32> {
        self.inner.duration_states.clone()
    }
    pub fn output_states(&self, stream: usize) -> Result<Vec<i32>, BindingError> {
        self.inner
            .output_states
            .get(stream)
            .cloned()
            .ok_or_else(|| error("output stream index out of range"))
    }
    pub fn set_boundaries(&mut self, values: &[i32]) -> Result<(), BindingError> {
        if values.len() != self.segments() {
            return Err(error("boundary count differs"));
        }
        self.inner.boundaries.copy_from_slice(values);
        Ok(())
    }
    pub fn set_duration_states(&mut self, values: &[i32]) -> Result<(), BindingError> {
        if values.len() != self.segments() {
            return Err(error("duration state count differs"));
        }
        self.inner.duration_states.copy_from_slice(values);
        Ok(())
    }
    pub fn set_output_states(&mut self, stream: usize, values: &[i32]) -> Result<(), BindingError> {
        let target = self
            .inner
            .output_states
            .get_mut(stream)
            .ok_or_else(|| error("output stream index out of range"))?;
        if target.len() != values.len() {
            return Err(error("output state count differs"));
        }
        target.copy_from_slice(values);
        Ok(())
    }
    pub fn outgoing(&self, state: usize) -> Option<JumpGroupF64> {
        self.inner
            .outgoing
            .get(state)
            .cloned()
            .map(|inner| JumpGroupF64 { inner })
    }
    pub fn set_outgoing(&mut self, state: usize, jumps: &JumpGroupF64) -> Result<(), BindingError> {
        let mut inner = self.inner.clone();
        *inner
            .outgoing
            .get_mut(state)
            .ok_or_else(|| error("jump state index out of range"))? = jumps.inner.clone();
        inner.validate().map_err(error)?;
        self.inner = inner;
        Ok(())
    }
    pub fn replace(&mut self, value: &SegmentationF64) {
        self.inner = value.inner.clone();
    }
    pub fn validate(&self) -> Result<(), BindingError> {
        self.inner.validate().map_err(error)
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
    pub fn add_jump(
        &mut self,
        state: usize,
        delta: i32,
        probability: f64,
    ) -> Result<(), BindingError> {
        self.inner
            .add_jump(state, delta, probability)
            .map_err(error)
    }
    pub fn build_transitions(&self) -> Result<TransitionGraphF64, BindingError> {
        self.cloned().build_transitions_owned()
    }
    pub fn build_transitions_owned(self) -> Result<TransitionGraphF64, BindingError> {
        self.inner
            .build_transitions_owned()
            .map(|inner| TransitionGraphF64 { inner })
            .map_err(error)
    }
    pub fn resegment(&self, occurrences: &[u32]) -> Result<Self, BindingError> {
        if !occurrences.len().is_multiple_of(2) {
            return Err(error("state occurrences require state/end pairs"));
        }
        let occurrences = occurrences
            .chunks_exact(2)
            .map(|value| StateOccurrence {
                state: value[0] as usize,
                end: value[1] as usize,
            })
            .collect::<Vec<_>>();
        self.inner
            .resegment(&occurrences)
            .map(|inner| Self { inner })
            .map_err(error)
    }
}
#[derive(Clone)]
pub struct TransitionGraphF64 {
    pub(crate) inner: OwnedTransitionGraphParameters<f64>,
}
impl TransitionGraphF64 {
    pub fn cloned(&self) -> Self {
        self.clone()
    }
    pub fn segmentation(&self) -> SegmentationF64 {
        SegmentationF64 {
            inner: self.inner.segmentation().clone(),
        }
    }
    pub fn into_segmentation(self) -> SegmentationF64 {
        SegmentationF64 {
            inner: self.inner.into_segmentation(),
        }
    }
    pub fn incoming(&self, state: usize) -> Option<JumpGroupF64> {
        self.inner
            .incoming()
            .get(state)
            .cloned()
            .map(|inner| JumpGroupF64 { inner })
    }
    pub fn outgoing(&self, state: usize) -> Option<JumpGroupF64> {
        self.inner
            .outgoing()
            .get(state)
            .cloned()
            .map(|inner| JumpGroupF64 { inner })
    }
}
