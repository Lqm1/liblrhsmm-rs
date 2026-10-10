fn new_err_owned(message: &str) -> pyo3::PyErr {
    pyo3::exceptions::PyValueError::new_err(message.to_owned())
}
use super::{ObservationF32, ObservationF64, SegmentationF32, SegmentationF64};
use crate::data::{ObservationData, SegmentationData};
use crate::serial::DecodeLimits;
use pyo3::PyErr as BindingError;
fn error(value: impl std::fmt::Display) -> BindingError {
    new_err_owned(&value.to_string())
}
impl ObservationF32 {
    pub fn read(bytes: &[u8]) -> Result<Self, BindingError> {
        <ObservationData<f32>>::read_from(bytes)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn read_with_limits(bytes: &[u8], max_array_entries: usize) -> Result<Self, BindingError> {
        <ObservationData<f32>>::read_with_limits(bytes, DecodeLimits { max_array_entries })
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn read_prefix(bytes: &[u8]) -> Result<DecodedObservationF32, BindingError> {
        Self::read_prefix_with_limits(bytes, DecodeLimits::default().max_array_entries)
    }
    pub fn read_prefix_with_limits(
        bytes: &[u8],
        max_array_entries: usize,
    ) -> Result<DecodedObservationF32, BindingError> {
        let mut reader = std::io::Cursor::new(bytes);
        let inner = <ObservationData<f32>>::read_prefix_with_limits(
            &mut reader,
            DecodeLimits { max_array_entries },
        )
        .map_err(error)?;
        Ok(DecodedObservationF32 {
            inner,
            position: reader.position(),
        })
    }
    pub fn write(&self) -> Result<Vec<u8>, BindingError> {
        let mut bytes = Vec::new();
        self.inner.write_to(&mut bytes).map_err(error)?;
        Ok(bytes)
    }
}
pub struct DecodedObservationF32 {
    inner: ObservationData<f32>,
    position: u64,
}
impl DecodedObservationF32 {
    pub fn position(&self) -> u64 {
        self.position
    }
    pub fn parameters(&self) -> ObservationF32 {
        ObservationF32 {
            inner: self.inner.clone(),
        }
    }
    pub fn into_parameters(self) -> ObservationF32 {
        ObservationF32 { inner: self.inner }
    }
}
impl ObservationF64 {
    pub fn read(bytes: &[u8]) -> Result<Self, BindingError> {
        <ObservationData<f64>>::read_from(bytes)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn read_with_limits(bytes: &[u8], max_array_entries: usize) -> Result<Self, BindingError> {
        <ObservationData<f64>>::read_with_limits(bytes, DecodeLimits { max_array_entries })
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn read_prefix(bytes: &[u8]) -> Result<DecodedObservationF64, BindingError> {
        Self::read_prefix_with_limits(bytes, DecodeLimits::default().max_array_entries)
    }
    pub fn read_prefix_with_limits(
        bytes: &[u8],
        max_array_entries: usize,
    ) -> Result<DecodedObservationF64, BindingError> {
        let mut reader = std::io::Cursor::new(bytes);
        let inner = <ObservationData<f64>>::read_prefix_with_limits(
            &mut reader,
            DecodeLimits { max_array_entries },
        )
        .map_err(error)?;
        Ok(DecodedObservationF64 {
            inner,
            position: reader.position(),
        })
    }
    pub fn write(&self) -> Result<Vec<u8>, BindingError> {
        let mut bytes = Vec::new();
        self.inner.write_to(&mut bytes).map_err(error)?;
        Ok(bytes)
    }
}
pub struct DecodedObservationF64 {
    inner: ObservationData<f64>,
    position: u64,
}
impl DecodedObservationF64 {
    pub fn position(&self) -> u64 {
        self.position
    }
    pub fn parameters(&self) -> ObservationF64 {
        ObservationF64 {
            inner: self.inner.clone(),
        }
    }
    pub fn into_parameters(self) -> ObservationF64 {
        ObservationF64 { inner: self.inner }
    }
}
impl SegmentationF32 {
    pub fn read(bytes: &[u8]) -> Result<Self, BindingError> {
        <SegmentationData<f32>>::read_from(bytes)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn read_with_limits(bytes: &[u8], max_array_entries: usize) -> Result<Self, BindingError> {
        <SegmentationData<f32>>::read_with_limits(bytes, DecodeLimits { max_array_entries })
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn read_prefix(bytes: &[u8]) -> Result<DecodedSegmentationF32, BindingError> {
        Self::read_prefix_with_limits(bytes, DecodeLimits::default().max_array_entries)
    }
    pub fn read_prefix_with_limits(
        bytes: &[u8],
        max_array_entries: usize,
    ) -> Result<DecodedSegmentationF32, BindingError> {
        let mut reader = std::io::Cursor::new(bytes);
        let inner = <SegmentationData<f32>>::read_prefix_with_limits(
            &mut reader,
            DecodeLimits { max_array_entries },
        )
        .map_err(error)?;
        Ok(DecodedSegmentationF32 {
            inner,
            position: reader.position(),
        })
    }
    pub fn write(&self) -> Result<Vec<u8>, BindingError> {
        let mut bytes = Vec::new();
        self.inner.write_to(&mut bytes).map_err(error)?;
        Ok(bytes)
    }
}
pub struct DecodedSegmentationF32 {
    inner: SegmentationData<f32>,
    position: u64,
}
impl DecodedSegmentationF32 {
    pub fn position(&self) -> u64 {
        self.position
    }
    pub fn parameters(&self) -> SegmentationF32 {
        SegmentationF32 {
            inner: self.inner.clone(),
        }
    }
    pub fn into_parameters(self) -> SegmentationF32 {
        SegmentationF32 { inner: self.inner }
    }
}
impl SegmentationF64 {
    pub fn read(bytes: &[u8]) -> Result<Self, BindingError> {
        <SegmentationData<f64>>::read_from(bytes)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn read_with_limits(bytes: &[u8], max_array_entries: usize) -> Result<Self, BindingError> {
        <SegmentationData<f64>>::read_with_limits(bytes, DecodeLimits { max_array_entries })
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn read_prefix(bytes: &[u8]) -> Result<DecodedSegmentationF64, BindingError> {
        Self::read_prefix_with_limits(bytes, DecodeLimits::default().max_array_entries)
    }
    pub fn read_prefix_with_limits(
        bytes: &[u8],
        max_array_entries: usize,
    ) -> Result<DecodedSegmentationF64, BindingError> {
        let mut reader = std::io::Cursor::new(bytes);
        let inner = <SegmentationData<f64>>::read_prefix_with_limits(
            &mut reader,
            DecodeLimits { max_array_entries },
        )
        .map_err(error)?;
        Ok(DecodedSegmentationF64 {
            inner,
            position: reader.position(),
        })
    }
    pub fn write(&self) -> Result<Vec<u8>, BindingError> {
        let mut bytes = Vec::new();
        self.inner.write_to(&mut bytes).map_err(error)?;
        Ok(bytes)
    }
}
pub struct DecodedSegmentationF64 {
    inner: SegmentationData<f64>,
    position: u64,
}
impl DecodedSegmentationF64 {
    pub fn position(&self) -> u64 {
        self.position
    }
    pub fn parameters(&self) -> SegmentationF64 {
        SegmentationF64 {
            inner: self.inner.clone(),
        }
    }
    pub fn into_parameters(self) -> SegmentationF64 {
        SegmentationF64 { inner: self.inner }
    }
}
