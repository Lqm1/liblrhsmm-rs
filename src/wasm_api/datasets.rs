use super::*;
use crate::data::{ObservationData, SegmentationData};
use crate::data_operations::{AlignedDataset, SampleSetBuilder, regroup_with_precision};
use wasm_bindgen::prelude::*;
fn error(value: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&value.to_string())
}

macro_rules! builder {
    ($builder:ident, $sample:ident, $collection:ident, $native:ty) => {
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $builder {
            inner: SampleSetBuilder<$native>,
        }
        #[wasm_bindgen]
        impl $builder {
            #[wasm_bindgen(constructor)]
            pub fn new(samples: usize) -> Result<Self, JsValue> {
                SampleSetBuilder::new(samples)
                    .map(|inner| Self { inner })
                    .map_err(error)
            }
            #[wasm_bindgen(getter)]
            pub fn length(&self) -> usize {
                self.inner.samples.len()
            }
            pub fn has(&self, index: usize) -> bool {
                self.inner.samples.get(index).is_some_and(Option::is_some)
            }
            pub fn get(&self, index: usize) -> Option<$sample> {
                self.inner
                    .samples
                    .get(index)
                    .and_then(|value| value.clone())
                    .map(|inner| $sample { inner })
            }
            pub fn set(&mut self, index: usize, value: &$sample) -> Result<(), JsValue> {
                *self
                    .inner
                    .samples
                    .get_mut(index)
                    .ok_or_else(|| error("sample slot index out of range"))? =
                    Some(value.inner.clone());
                Ok(())
            }
            pub fn clear(&mut self, index: usize) -> Result<(), JsValue> {
                *self
                    .inner
                    .samples
                    .get_mut(index)
                    .ok_or_else(|| error("sample slot index out of range"))? = None;
                Ok(())
            }
            pub fn resize(&mut self, count: usize) -> Result<(), JsValue> {
                let mut inner = SampleSetBuilder::new(count).map_err(error)?;
                for (target, source) in inner.samples.iter_mut().zip(&self.inner.samples) {
                    *target = source.clone();
                }
                self.inner = inner;
                Ok(())
            }
            pub fn cloned(&self) -> Self {
                self.clone()
            }
            pub fn build(self) -> Result<$collection, JsValue> {
                self.inner
                    .build()
                    .map(|inner| $collection { inner })
                    .map_err(error)
            }
        }
    };
}
builder!(
    ObservationSetBuilderF32,
    ObservationF32,
    ObservationCollectionF32,
    ObservationData<f32>
);
builder!(
    ObservationSetBuilderF64,
    ObservationF64,
    ObservationCollectionF64,
    ObservationData<f64>
);
builder!(
    SegmentationSetBuilderF32,
    SegmentationF32,
    SegmentationCollectionF32,
    SegmentationData<f32>
);
builder!(
    SegmentationSetBuilderF64,
    SegmentationF64,
    SegmentationCollectionF64,
    SegmentationData<f64>
);

macro_rules! dataset {
    ($scalar:ty, $dataset:ident, $observation:ident, $observations:ident, $segmentation:ident, $segmentations:ident, $regroup:ident) => {
        #[wasm_bindgen]
        #[derive(Clone)]
        pub struct $dataset {
            pub(crate) inner: AlignedDataset<$scalar>,
        }
        #[wasm_bindgen]
        impl $dataset {
            #[wasm_bindgen(constructor)]
            pub fn new(observations: &$observations, segmentations: &$segmentations) -> Self {
                Self {
                    inner: AlignedDataset {
                        observations: observations.inner.clone(),
                        segmentations: segmentations.inner.clone(),
                    },
                }
            }
            #[wasm_bindgen(getter)]
            pub fn observation_count(&self) -> usize {
                self.inner.observations.len()
            }
            #[wasm_bindgen(getter)]
            pub fn segmentation_count(&self) -> usize {
                self.inner.segmentations.len()
            }
            pub fn observations(&self) -> $observations {
                $observations {
                    inner: self.inner.observations.clone(),
                }
            }
            pub fn segmentations(&self) -> $segmentations {
                $segmentations {
                    inner: self.inner.segmentations.clone(),
                }
            }
            pub fn observation(&self, index: usize) -> Option<$observation> {
                self.inner
                    .observations
                    .get(index)
                    .cloned()
                    .map(|inner| $observation { inner })
            }
            pub fn segmentation(&self, index: usize) -> Option<$segmentation> {
                self.inner
                    .segmentations
                    .get(index)
                    .cloned()
                    .map(|inner| $segmentation { inner })
            }
            pub fn set_observation(
                &mut self,
                index: usize,
                value: &$observation,
            ) -> Result<(), JsValue> {
                *self
                    .inner
                    .observations
                    .get_mut(index)
                    .ok_or_else(|| error("observation sample index out of range"))? =
                    value.inner.clone();
                Ok(())
            }
            pub fn set_segmentation(
                &mut self,
                index: usize,
                value: &$segmentation,
            ) -> Result<(), JsValue> {
                *self
                    .inner
                    .segmentations
                    .get_mut(index)
                    .ok_or_else(|| error("segmentation sample index out of range"))? =
                    value.inner.clone();
                Ok(())
            }
            pub fn replace(
                &mut self,
                observations: &$observations,
                segmentations: &$segmentations,
            ) {
                self.inner = AlignedDataset {
                    observations: observations.inner.clone(),
                    segmentations: segmentations.inner.clone(),
                };
            }
            pub fn cloned(&self) -> Self {
                self.clone()
            }
            pub fn regroup(&self, segments_per_unit: usize) -> Result<Self, JsValue> {
                regroup_with_precision(
                    &self.inner.observations,
                    &self.inner.segmentations,
                    segments_per_unit,
                )
                .map(|inner| Self { inner })
                .map_err(error)
            }
        }
        #[wasm_bindgen]
        pub fn $regroup(
            observations: &$observations,
            segmentations: &$segmentations,
            segments_per_unit: usize,
        ) -> Result<$dataset, JsValue> {
            regroup_with_precision(&observations.inner, &segmentations.inner, segments_per_unit)
                .map(|inner| $dataset { inner })
                .map_err(error)
        }
    };
}
dataset!(
    f32,
    DatasetF32,
    ObservationF32,
    ObservationCollectionF32,
    SegmentationF32,
    SegmentationCollectionF32,
    regroup_data_f32
);
dataset!(
    f64,
    DatasetF64,
    ObservationF64,
    ObservationCollectionF64,
    SegmentationF64,
    SegmentationCollectionF64,
    regroup_data_f64
);
