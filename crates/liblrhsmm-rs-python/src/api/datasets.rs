fn new_err_owned(message: &str) -> pyo3::PyErr {
    pyo3::exceptions::PyValueError::new_err(message.to_owned())
}
use super::*;
use crate::data::{ObservationData, SegmentationData};
use crate::data_operations::{AlignedDataset, SampleSetBuilder, regroup_with_precision};
use pyo3::PyErr as BindingError;
fn error(value: impl std::fmt::Display) -> BindingError {
    new_err_owned(&value.to_string())
}
#[derive(Clone)]
pub struct ObservationSetBuilderF32 {
    inner: SampleSetBuilder<ObservationData<f32>>,
}
impl ObservationSetBuilderF32 {
    pub fn new(samples: usize) -> Result<Self, BindingError> {
        SampleSetBuilder::new(samples)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn length(&self) -> usize {
        self.inner.samples.len()
    }
    pub fn has(&self, index: usize) -> bool {
        self.inner.samples.get(index).is_some_and(Option::is_some)
    }
    pub fn get(&self, index: usize) -> Option<ObservationF32> {
        self.inner
            .samples
            .get(index)
            .and_then(|value| value.clone())
            .map(|inner| ObservationF32 { inner })
    }
    pub fn set(&mut self, index: usize, value: &ObservationF32) -> Result<(), BindingError> {
        *self
            .inner
            .samples
            .get_mut(index)
            .ok_or_else(|| error("sample slot index out of range"))? = Some(value.inner.clone());
        Ok(())
    }
    pub fn clear(&mut self, index: usize) -> Result<(), BindingError> {
        *self
            .inner
            .samples
            .get_mut(index)
            .ok_or_else(|| error("sample slot index out of range"))? = None;
        Ok(())
    }
    pub fn resize(&mut self, count: usize) -> Result<(), BindingError> {
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
    pub fn build(self) -> Result<ObservationCollectionF32, BindingError> {
        self.inner
            .build()
            .map(|inner| ObservationCollectionF32 { inner })
            .map_err(error)
    }
}
#[derive(Clone)]
pub struct ObservationSetBuilderF64 {
    inner: SampleSetBuilder<ObservationData<f64>>,
}
impl ObservationSetBuilderF64 {
    pub fn new(samples: usize) -> Result<Self, BindingError> {
        SampleSetBuilder::new(samples)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn length(&self) -> usize {
        self.inner.samples.len()
    }
    pub fn has(&self, index: usize) -> bool {
        self.inner.samples.get(index).is_some_and(Option::is_some)
    }
    pub fn get(&self, index: usize) -> Option<ObservationF64> {
        self.inner
            .samples
            .get(index)
            .and_then(|value| value.clone())
            .map(|inner| ObservationF64 { inner })
    }
    pub fn set(&mut self, index: usize, value: &ObservationF64) -> Result<(), BindingError> {
        *self
            .inner
            .samples
            .get_mut(index)
            .ok_or_else(|| error("sample slot index out of range"))? = Some(value.inner.clone());
        Ok(())
    }
    pub fn clear(&mut self, index: usize) -> Result<(), BindingError> {
        *self
            .inner
            .samples
            .get_mut(index)
            .ok_or_else(|| error("sample slot index out of range"))? = None;
        Ok(())
    }
    pub fn resize(&mut self, count: usize) -> Result<(), BindingError> {
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
    pub fn build(self) -> Result<ObservationCollectionF64, BindingError> {
        self.inner
            .build()
            .map(|inner| ObservationCollectionF64 { inner })
            .map_err(error)
    }
}
#[derive(Clone)]
pub struct SegmentationSetBuilderF32 {
    inner: SampleSetBuilder<SegmentationData<f32>>,
}
impl SegmentationSetBuilderF32 {
    pub fn new(samples: usize) -> Result<Self, BindingError> {
        SampleSetBuilder::new(samples)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn length(&self) -> usize {
        self.inner.samples.len()
    }
    pub fn has(&self, index: usize) -> bool {
        self.inner.samples.get(index).is_some_and(Option::is_some)
    }
    pub fn get(&self, index: usize) -> Option<SegmentationF32> {
        self.inner
            .samples
            .get(index)
            .and_then(|value| value.clone())
            .map(|inner| SegmentationF32 { inner })
    }
    pub fn set(&mut self, index: usize, value: &SegmentationF32) -> Result<(), BindingError> {
        *self
            .inner
            .samples
            .get_mut(index)
            .ok_or_else(|| error("sample slot index out of range"))? = Some(value.inner.clone());
        Ok(())
    }
    pub fn clear(&mut self, index: usize) -> Result<(), BindingError> {
        *self
            .inner
            .samples
            .get_mut(index)
            .ok_or_else(|| error("sample slot index out of range"))? = None;
        Ok(())
    }
    pub fn resize(&mut self, count: usize) -> Result<(), BindingError> {
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
    pub fn build(self) -> Result<SegmentationCollectionF32, BindingError> {
        self.inner
            .build()
            .map(|inner| SegmentationCollectionF32 { inner })
            .map_err(error)
    }
}
#[derive(Clone)]
pub struct SegmentationSetBuilderF64 {
    inner: SampleSetBuilder<SegmentationData<f64>>,
}
impl SegmentationSetBuilderF64 {
    pub fn new(samples: usize) -> Result<Self, BindingError> {
        SampleSetBuilder::new(samples)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn length(&self) -> usize {
        self.inner.samples.len()
    }
    pub fn has(&self, index: usize) -> bool {
        self.inner.samples.get(index).is_some_and(Option::is_some)
    }
    pub fn get(&self, index: usize) -> Option<SegmentationF64> {
        self.inner
            .samples
            .get(index)
            .and_then(|value| value.clone())
            .map(|inner| SegmentationF64 { inner })
    }
    pub fn set(&mut self, index: usize, value: &SegmentationF64) -> Result<(), BindingError> {
        *self
            .inner
            .samples
            .get_mut(index)
            .ok_or_else(|| error("sample slot index out of range"))? = Some(value.inner.clone());
        Ok(())
    }
    pub fn clear(&mut self, index: usize) -> Result<(), BindingError> {
        *self
            .inner
            .samples
            .get_mut(index)
            .ok_or_else(|| error("sample slot index out of range"))? = None;
        Ok(())
    }
    pub fn resize(&mut self, count: usize) -> Result<(), BindingError> {
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
    pub fn build(self) -> Result<SegmentationCollectionF64, BindingError> {
        self.inner
            .build()
            .map(|inner| SegmentationCollectionF64 { inner })
            .map_err(error)
    }
}
#[derive(Clone)]
pub struct DatasetF32 {
    pub(crate) inner: AlignedDataset<f32>,
}
impl DatasetF32 {
    pub fn new(
        observations: &ObservationCollectionF32,
        segmentations: &SegmentationCollectionF32,
    ) -> Self {
        Self {
            inner: AlignedDataset {
                observations: observations.inner.clone(),
                segmentations: segmentations.inner.clone(),
            },
        }
    }
    pub fn observation_count(&self) -> usize {
        self.inner.observations.len()
    }
    pub fn segmentation_count(&self) -> usize {
        self.inner.segmentations.len()
    }
    pub fn observations(&self) -> ObservationCollectionF32 {
        ObservationCollectionF32 {
            inner: self.inner.observations.clone(),
        }
    }
    pub fn segmentations(&self) -> SegmentationCollectionF32 {
        SegmentationCollectionF32 {
            inner: self.inner.segmentations.clone(),
        }
    }
    pub fn observation(&self, index: usize) -> Option<ObservationF32> {
        self.inner
            .observations
            .get(index)
            .cloned()
            .map(|inner| ObservationF32 { inner })
    }
    pub fn segmentation(&self, index: usize) -> Option<SegmentationF32> {
        self.inner
            .segmentations
            .get(index)
            .cloned()
            .map(|inner| SegmentationF32 { inner })
    }
    pub fn set_observation(
        &mut self,
        index: usize,
        value: &ObservationF32,
    ) -> Result<(), BindingError> {
        *self
            .inner
            .observations
            .get_mut(index)
            .ok_or_else(|| error("observation sample index out of range"))? = value.inner.clone();
        Ok(())
    }
    pub fn set_segmentation(
        &mut self,
        index: usize,
        value: &SegmentationF32,
    ) -> Result<(), BindingError> {
        *self
            .inner
            .segmentations
            .get_mut(index)
            .ok_or_else(|| error("segmentation sample index out of range"))? = value.inner.clone();
        Ok(())
    }
    pub fn replace(
        &mut self,
        observations: &ObservationCollectionF32,
        segmentations: &SegmentationCollectionF32,
    ) {
        self.inner = AlignedDataset {
            observations: observations.inner.clone(),
            segmentations: segmentations.inner.clone(),
        };
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
    pub fn regroup(&self, segments_per_unit: usize) -> Result<Self, BindingError> {
        regroup_with_precision(
            &self.inner.observations,
            &self.inner.segmentations,
            segments_per_unit,
        )
        .map(|inner| Self { inner })
        .map_err(error)
    }
}
pub fn regroup_data_f32(
    observations: &ObservationCollectionF32,
    segmentations: &SegmentationCollectionF32,
    segments_per_unit: usize,
) -> Result<DatasetF32, BindingError> {
    regroup_with_precision(&observations.inner, &segmentations.inner, segments_per_unit)
        .map(|inner| DatasetF32 { inner })
        .map_err(error)
}
#[derive(Clone)]
pub struct DatasetF64 {
    pub(crate) inner: AlignedDataset<f64>,
}
impl DatasetF64 {
    pub fn new(
        observations: &ObservationCollectionF64,
        segmentations: &SegmentationCollectionF64,
    ) -> Self {
        Self {
            inner: AlignedDataset {
                observations: observations.inner.clone(),
                segmentations: segmentations.inner.clone(),
            },
        }
    }
    pub fn observation_count(&self) -> usize {
        self.inner.observations.len()
    }
    pub fn segmentation_count(&self) -> usize {
        self.inner.segmentations.len()
    }
    pub fn observations(&self) -> ObservationCollectionF64 {
        ObservationCollectionF64 {
            inner: self.inner.observations.clone(),
        }
    }
    pub fn segmentations(&self) -> SegmentationCollectionF64 {
        SegmentationCollectionF64 {
            inner: self.inner.segmentations.clone(),
        }
    }
    pub fn observation(&self, index: usize) -> Option<ObservationF64> {
        self.inner
            .observations
            .get(index)
            .cloned()
            .map(|inner| ObservationF64 { inner })
    }
    pub fn segmentation(&self, index: usize) -> Option<SegmentationF64> {
        self.inner
            .segmentations
            .get(index)
            .cloned()
            .map(|inner| SegmentationF64 { inner })
    }
    pub fn set_observation(
        &mut self,
        index: usize,
        value: &ObservationF64,
    ) -> Result<(), BindingError> {
        *self
            .inner
            .observations
            .get_mut(index)
            .ok_or_else(|| error("observation sample index out of range"))? = value.inner.clone();
        Ok(())
    }
    pub fn set_segmentation(
        &mut self,
        index: usize,
        value: &SegmentationF64,
    ) -> Result<(), BindingError> {
        *self
            .inner
            .segmentations
            .get_mut(index)
            .ok_or_else(|| error("segmentation sample index out of range"))? = value.inner.clone();
        Ok(())
    }
    pub fn replace(
        &mut self,
        observations: &ObservationCollectionF64,
        segmentations: &SegmentationCollectionF64,
    ) {
        self.inner = AlignedDataset {
            observations: observations.inner.clone(),
            segmentations: segmentations.inner.clone(),
        };
    }
    pub fn cloned(&self) -> Self {
        self.clone()
    }
    pub fn regroup(&self, segments_per_unit: usize) -> Result<Self, BindingError> {
        regroup_with_precision(
            &self.inner.observations,
            &self.inner.segmentations,
            segments_per_unit,
        )
        .map(|inner| Self { inner })
        .map_err(error)
    }
}
pub fn regroup_data_f64(
    observations: &ObservationCollectionF64,
    segmentations: &SegmentationCollectionF64,
    segments_per_unit: usize,
) -> Result<DatasetF64, BindingError> {
    regroup_with_precision(&observations.inner, &segmentations.inner, segments_per_unit)
        .map(|inner| DatasetF64 { inner })
        .map_err(error)
}
