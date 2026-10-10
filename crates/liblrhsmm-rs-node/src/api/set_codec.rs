use super::{
    ObservationCollectionF32, ObservationCollectionF64, SegmentationCollectionF32,
    SegmentationCollectionF64,
};
use crate::data::{ObservationData, SegmentationData};
use crate::serial;
use napi::Error as BindingError;
fn error(value: impl std::fmt::Display) -> BindingError {
    napi::Error::from_reason(value.to_string())
}
impl ObservationCollectionF32 {
    pub fn read(bytes: &[u8]) -> Result<Self, BindingError> {
        serial::read_observation_set_with_precision::<f32>(bytes)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn read_with_limits(bytes: &[u8], max_array_entries: usize) -> Result<Self, BindingError> {
        serial::read_observation_set_with_limits::<f32>(
            bytes,
            serial::DecodeLimits { max_array_entries },
        )
        .map(|inner| Self { inner })
        .map_err(error)
    }
    pub fn read_prefix(bytes: &[u8]) -> Result<DecodedObservationSetF32, BindingError> {
        let mut reader = std::io::Cursor::new(bytes);
        let inner = serial::read_observation_set_prefix_with_precision::<f32>(&mut reader)
            .map_err(error)?;
        Ok(DecodedObservationSetF32 {
            inner,
            position: reader.position(),
        })
    }
    pub fn read_prefix_with_limits(
        bytes: &[u8],
        max_array_entries: usize,
    ) -> Result<DecodedObservationSetF32, BindingError> {
        let mut reader = std::io::Cursor::new(bytes);
        let inner = serial::read_observation_set_prefix_with_limits::<f32>(
            &mut reader,
            serial::DecodeLimits { max_array_entries },
        )
        .map_err(error)?;
        Ok(DecodedObservationSetF32 {
            inner,
            position: reader.position(),
        })
    }
    pub fn write(&self) -> Result<Vec<u8>, BindingError> {
        let mut bytes = Vec::new();
        serial::write_observation_set_with_precision::<f32>(&self.inner, &mut bytes)
            .map_err(error)?;
        Ok(bytes)
    }
}
pub struct DecodedObservationSetF32 {
    inner: Vec<ObservationData<f32>>,
    position: u64,
}
impl DecodedObservationSetF32 {
    pub fn position(&self) -> u64 {
        self.position
    }
    pub fn samples(&self) -> ObservationCollectionF32 {
        ObservationCollectionF32 {
            inner: self.inner.clone(),
        }
    }
    pub fn into_samples(self) -> ObservationCollectionF32 {
        ObservationCollectionF32 { inner: self.inner }
    }
}
impl ObservationCollectionF64 {
    pub fn read(bytes: &[u8]) -> Result<Self, BindingError> {
        serial::read_observation_set_with_precision::<f64>(bytes)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn read_with_limits(bytes: &[u8], max_array_entries: usize) -> Result<Self, BindingError> {
        serial::read_observation_set_with_limits::<f64>(
            bytes,
            serial::DecodeLimits { max_array_entries },
        )
        .map(|inner| Self { inner })
        .map_err(error)
    }
    pub fn read_prefix(bytes: &[u8]) -> Result<DecodedObservationSetF64, BindingError> {
        let mut reader = std::io::Cursor::new(bytes);
        let inner = serial::read_observation_set_prefix_with_precision::<f64>(&mut reader)
            .map_err(error)?;
        Ok(DecodedObservationSetF64 {
            inner,
            position: reader.position(),
        })
    }
    pub fn read_prefix_with_limits(
        bytes: &[u8],
        max_array_entries: usize,
    ) -> Result<DecodedObservationSetF64, BindingError> {
        let mut reader = std::io::Cursor::new(bytes);
        let inner = serial::read_observation_set_prefix_with_limits::<f64>(
            &mut reader,
            serial::DecodeLimits { max_array_entries },
        )
        .map_err(error)?;
        Ok(DecodedObservationSetF64 {
            inner,
            position: reader.position(),
        })
    }
    pub fn write(&self) -> Result<Vec<u8>, BindingError> {
        let mut bytes = Vec::new();
        serial::write_observation_set_with_precision::<f64>(&self.inner, &mut bytes)
            .map_err(error)?;
        Ok(bytes)
    }
}
pub struct DecodedObservationSetF64 {
    inner: Vec<ObservationData<f64>>,
    position: u64,
}
impl DecodedObservationSetF64 {
    pub fn position(&self) -> u64 {
        self.position
    }
    pub fn samples(&self) -> ObservationCollectionF64 {
        ObservationCollectionF64 {
            inner: self.inner.clone(),
        }
    }
    pub fn into_samples(self) -> ObservationCollectionF64 {
        ObservationCollectionF64 { inner: self.inner }
    }
}
impl SegmentationCollectionF32 {
    pub fn read(bytes: &[u8]) -> Result<Self, BindingError> {
        serial::read_segmentation_set_with_precision::<f32>(bytes)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn read_with_limits(bytes: &[u8], max_array_entries: usize) -> Result<Self, BindingError> {
        serial::read_segmentation_set_with_limits::<f32>(
            bytes,
            serial::DecodeLimits { max_array_entries },
        )
        .map(|inner| Self { inner })
        .map_err(error)
    }
    pub fn read_prefix(bytes: &[u8]) -> Result<DecodedSegmentationSetF32, BindingError> {
        let mut reader = std::io::Cursor::new(bytes);
        let inner = serial::read_segmentation_set_prefix_with_precision::<f32>(&mut reader)
            .map_err(error)?;
        Ok(DecodedSegmentationSetF32 {
            inner,
            position: reader.position(),
        })
    }
    pub fn read_prefix_with_limits(
        bytes: &[u8],
        max_array_entries: usize,
    ) -> Result<DecodedSegmentationSetF32, BindingError> {
        let mut reader = std::io::Cursor::new(bytes);
        let inner = serial::read_segmentation_set_prefix_with_limits::<f32>(
            &mut reader,
            serial::DecodeLimits { max_array_entries },
        )
        .map_err(error)?;
        Ok(DecodedSegmentationSetF32 {
            inner,
            position: reader.position(),
        })
    }
    pub fn write(&self) -> Result<Vec<u8>, BindingError> {
        let mut bytes = Vec::new();
        serial::write_segmentation_set_with_precision::<f32>(&self.inner, &mut bytes)
            .map_err(error)?;
        Ok(bytes)
    }
}
pub struct DecodedSegmentationSetF32 {
    inner: Vec<SegmentationData<f32>>,
    position: u64,
}
impl DecodedSegmentationSetF32 {
    pub fn position(&self) -> u64 {
        self.position
    }
    pub fn samples(&self) -> SegmentationCollectionF32 {
        SegmentationCollectionF32 {
            inner: self.inner.clone(),
        }
    }
    pub fn into_samples(self) -> SegmentationCollectionF32 {
        SegmentationCollectionF32 { inner: self.inner }
    }
}
impl SegmentationCollectionF64 {
    pub fn read(bytes: &[u8]) -> Result<Self, BindingError> {
        serial::read_segmentation_set_with_precision::<f64>(bytes)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn read_with_limits(bytes: &[u8], max_array_entries: usize) -> Result<Self, BindingError> {
        serial::read_segmentation_set_with_limits::<f64>(
            bytes,
            serial::DecodeLimits { max_array_entries },
        )
        .map(|inner| Self { inner })
        .map_err(error)
    }
    pub fn read_prefix(bytes: &[u8]) -> Result<DecodedSegmentationSetF64, BindingError> {
        let mut reader = std::io::Cursor::new(bytes);
        let inner = serial::read_segmentation_set_prefix_with_precision::<f64>(&mut reader)
            .map_err(error)?;
        Ok(DecodedSegmentationSetF64 {
            inner,
            position: reader.position(),
        })
    }
    pub fn read_prefix_with_limits(
        bytes: &[u8],
        max_array_entries: usize,
    ) -> Result<DecodedSegmentationSetF64, BindingError> {
        let mut reader = std::io::Cursor::new(bytes);
        let inner = serial::read_segmentation_set_prefix_with_limits::<f64>(
            &mut reader,
            serial::DecodeLimits { max_array_entries },
        )
        .map_err(error)?;
        Ok(DecodedSegmentationSetF64 {
            inner,
            position: reader.position(),
        })
    }
    pub fn write(&self) -> Result<Vec<u8>, BindingError> {
        let mut bytes = Vec::new();
        serial::write_segmentation_set_with_precision::<f64>(&self.inner, &mut bytes)
            .map_err(error)?;
        Ok(bytes)
    }
}
pub struct DecodedSegmentationSetF64 {
    inner: Vec<SegmentationData<f64>>,
    position: u64,
}
impl DecodedSegmentationSetF64 {
    pub fn position(&self) -> u64 {
        self.position
    }
    pub fn samples(&self) -> SegmentationCollectionF64 {
        SegmentationCollectionF64 {
            inner: self.inner.clone(),
        }
    }
    pub fn into_samples(self) -> SegmentationCollectionF64 {
        SegmentationCollectionF64 { inner: self.inner }
    }
}
