use super::gaussian::collection;
use crate::StateOccurrence;
use crate::data::{JumpParameters, ObservationData, ObservationStreamData, SegmentationData};
use crate::transitions::OwnedTransitionGraphParameters;
use wasm_bindgen::prelude::*;
fn error(value: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&value.to_string())
}

macro_rules! data {
    ($scalar:ty, $stream:ident, $streams:ident, $observation:ident, $observations:ident, $jumps:ident, $segmentation:ident, $segmentations:ident, $graph:ident) => {
        collection!($streams, $stream, ObservationStreamData<$scalar>);
        collection!($observations, $observation, ObservationData<$scalar>);
        collection!($segmentations, $segmentation, SegmentationData<$scalar>);
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $stream {
            pub(crate) inner: ObservationStreamData<$scalar>,
        }
        #[wasm_bindgen]
        impl $stream {
            #[wasm_bindgen(constructor)]
            pub fn new(dimensions: usize, values: &[$scalar]) -> Self {
                Self {
                    inner: ObservationStreamData {
                        dimensions,
                        values: values.to_vec(),
                    },
                }
            }
            #[wasm_bindgen(getter)]
            pub fn dimensions(&self) -> usize {
                self.inner.dimensions
            }
            pub fn values(&self) -> Vec<$scalar> {
                self.inner.values.clone()
            }
            pub fn replace(&mut self, dimensions: usize, values: &[$scalar]) {
                self.inner = ObservationStreamData {
                    dimensions,
                    values: values.to_vec(),
                };
            }
            pub fn cloned(&self) -> Self {
                self.clone()
            }
        }
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $observation {
            pub(crate) inner: ObservationData<$scalar>,
        }
        #[wasm_bindgen]
        impl $observation {
            #[wasm_bindgen(constructor)]
            pub fn new(frames: usize, dimensions: &[u32]) -> Result<Self, JsValue> {
                let dimensions = dimensions
                    .iter()
                    .map(|value| *value as usize)
                    .collect::<Vec<_>>();
                ObservationData::new(frames, &dimensions)
                    .map(|inner| Self { inner })
                    .map_err(error)
            }
            #[wasm_bindgen(getter)]
            pub fn frames(&self) -> usize {
                self.inner.frames
            }
            #[wasm_bindgen(getter)]
            pub fn streams(&self) -> usize {
                self.inner.streams.len()
            }
            pub fn stream(&self, index: usize) -> Option<$stream> {
                self.inner
                    .streams
                    .get(index)
                    .cloned()
                    .map(|inner| $stream { inner })
            }
            pub fn set_stream(&mut self, index: usize, value: &$stream) -> Result<(), JsValue> {
                let mut inner = self.inner.clone();
                *inner
                    .streams
                    .get_mut(index)
                    .ok_or_else(|| error("observation stream index out of range"))? =
                    value.inner.clone();
                inner.validate().map_err(error)?;
                self.inner = inner;
                Ok(())
            }
            pub fn replace(&mut self, frames: usize, streams: &$streams) -> Result<(), JsValue> {
                let inner = ObservationData {
                    frames,
                    streams: streams.inner.clone(),
                };
                inner.validate().map_err(error)?;
                self.inner = inner;
                Ok(())
            }
            pub fn frame(&self, stream: usize, time: usize) -> Option<Vec<$scalar>> {
                self.inner.frame(stream, time).map(<[$scalar]>::to_vec)
            }
            pub fn validate(&self) -> Result<(), JsValue> {
                self.inner.validate().map_err(error)
            }
            pub fn cloned(&self) -> Self {
                self.clone()
            }
        }
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $jumps {
            inner: Vec<JumpParameters<$scalar>>,
        }
        #[wasm_bindgen]
        impl $jumps {
            #[wasm_bindgen(constructor)]
            pub fn new(deltas: &[i32], probabilities: &[$scalar]) -> Result<Self, JsValue> {
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
            #[wasm_bindgen(getter)]
            pub fn length(&self) -> usize {
                self.inner.len()
            }
            pub fn deltas(&self) -> Vec<i32> {
                self.inner.iter().map(|value| value.delta).collect()
            }
            pub fn probabilities(&self) -> Vec<$scalar> {
                self.inner.iter().map(|value| value.probability).collect()
            }
            pub fn cloned(&self) -> Self {
                self.clone()
            }
        }
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $segmentation {
            pub(crate) inner: SegmentationData<$scalar>,
        }
        #[wasm_bindgen]
        impl $segmentation {
            #[wasm_bindgen(constructor)]
            pub fn new(streams: usize, segments: usize) -> Result<Self, JsValue> {
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
            #[wasm_bindgen(getter)]
            pub fn streams(&self) -> usize {
                self.inner.output_states.len()
            }
            #[wasm_bindgen(getter)]
            pub fn segments(&self) -> usize {
                self.inner.boundaries.len()
            }
            pub fn boundaries(&self) -> Vec<i32> {
                self.inner.boundaries.clone()
            }
            pub fn duration_states(&self) -> Vec<i32> {
                self.inner.duration_states.clone()
            }
            pub fn output_states(&self, stream: usize) -> Result<Vec<i32>, JsValue> {
                self.inner
                    .output_states
                    .get(stream)
                    .cloned()
                    .ok_or_else(|| error("output stream index out of range"))
            }
            pub fn set_boundaries(&mut self, values: &[i32]) -> Result<(), JsValue> {
                if values.len() != self.segments() {
                    return Err(error("boundary count differs"));
                }
                self.inner.boundaries.copy_from_slice(values);
                Ok(())
            }
            pub fn set_duration_states(&mut self, values: &[i32]) -> Result<(), JsValue> {
                if values.len() != self.segments() {
                    return Err(error("duration state count differs"));
                }
                self.inner.duration_states.copy_from_slice(values);
                Ok(())
            }
            pub fn set_output_states(
                &mut self,
                stream: usize,
                values: &[i32],
            ) -> Result<(), JsValue> {
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
            pub fn outgoing(&self, state: usize) -> Option<$jumps> {
                self.inner
                    .outgoing
                    .get(state)
                    .cloned()
                    .map(|inner| $jumps { inner })
            }
            pub fn set_outgoing(&mut self, state: usize, jumps: &$jumps) -> Result<(), JsValue> {
                let mut inner = self.inner.clone();
                *inner
                    .outgoing
                    .get_mut(state)
                    .ok_or_else(|| error("jump state index out of range"))? = jumps.inner.clone();
                inner.validate().map_err(error)?;
                self.inner = inner;
                Ok(())
            }
            pub fn replace(&mut self, value: &$segmentation) {
                self.inner = value.inner.clone();
            }
            pub fn validate(&self) -> Result<(), JsValue> {
                self.inner.validate().map_err(error)
            }
            pub fn cloned(&self) -> Self {
                self.clone()
            }
            pub fn add_jump(
                &mut self,
                state: usize,
                delta: i32,
                probability: $scalar,
            ) -> Result<(), JsValue> {
                self.inner
                    .add_jump(state, delta, probability)
                    .map_err(error)
            }
            pub fn build_transitions(&self) -> Result<$graph, JsValue> {
                self.cloned().build_transitions_owned()
            }
            pub fn build_transitions_owned(self) -> Result<$graph, JsValue> {
                self.inner
                    .build_transitions_owned()
                    .map(|inner| $graph { inner })
                    .map_err(error)
            }
            pub fn resegment(&self, occurrences: &[u32]) -> Result<Self, JsValue> {
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
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $graph {
            pub(crate) inner: OwnedTransitionGraphParameters<$scalar>,
        }
        #[wasm_bindgen]
        impl $graph {
            pub fn cloned(&self) -> Self {
                self.clone()
            }
            pub fn segmentation(&self) -> $segmentation {
                $segmentation {
                    inner: self.inner.segmentation().clone(),
                }
            }
            pub fn into_segmentation(self) -> $segmentation {
                $segmentation {
                    inner: self.inner.into_segmentation(),
                }
            }
            pub fn incoming(&self, state: usize) -> Option<$jumps> {
                self.inner
                    .incoming()
                    .get(state)
                    .cloned()
                    .map(|inner| $jumps { inner })
            }
            pub fn outgoing(&self, state: usize) -> Option<$jumps> {
                self.inner
                    .outgoing()
                    .get(state)
                    .cloned()
                    .map(|inner| $jumps { inner })
            }
        }
    };
}
data!(
    f32,
    ObservationStreamF32,
    ObservationStreamCollectionF32,
    ObservationF32,
    ObservationCollectionF32,
    JumpGroupF32,
    SegmentationF32,
    SegmentationCollectionF32,
    TransitionGraphF32
);
data!(
    f64,
    ObservationStreamF64,
    ObservationStreamCollectionF64,
    ObservationF64,
    ObservationCollectionF64,
    JumpGroupF64,
    SegmentationF64,
    SegmentationCollectionF64,
    TransitionGraphF64
);
