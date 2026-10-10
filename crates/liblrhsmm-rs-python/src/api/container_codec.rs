fn new_err_owned(message: &str) -> pyo3::PyErr {
    pyo3::exceptions::PyValueError::new_err(message.to_owned())
}
use super::{ModelF32, ModelF64, StreamF32, StreamF64};
use crate::model::{ModelParameters, StreamParameters};
use crate::serial::{DecodeLimits, ModelEncoding};
use pyo3::PyErr as BindingError;
fn error(value: impl std::fmt::Display) -> BindingError {
    new_err_owned(&value.to_string())
}
impl StreamF32 {
    pub fn read(bytes: &[u8]) -> Result<Self, BindingError> {
        <StreamParameters<f32>>::read_from(bytes)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn read_with_limits(bytes: &[u8], max_array_entries: usize) -> Result<Self, BindingError> {
        <StreamParameters<f32>>::read_with_limits(bytes, DecodeLimits { max_array_entries })
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn read_prefix(bytes: &[u8]) -> Result<DecodedStreamF32, BindingError> {
        Self::read_prefix_with_limits(bytes, DecodeLimits::default().max_array_entries)
    }
    pub fn read_prefix_with_limits(
        bytes: &[u8],
        max_array_entries: usize,
    ) -> Result<DecodedStreamF32, BindingError> {
        let mut reader = std::io::Cursor::new(bytes);
        let inner = <StreamParameters<f32>>::read_prefix_with_limits(
            &mut reader,
            DecodeLimits { max_array_entries },
        )
        .map_err(error)?;
        Ok(DecodedStreamF32 {
            inner,
            position: reader.position(),
        })
    }
    pub fn write(&self) -> Result<Vec<u8>, BindingError> {
        let mut bytes = Vec::new();
        self.inner.write_to(&mut bytes).map_err(error)?;
        Ok(bytes)
    }
    pub fn write_with_encoding(&self, code: u32) -> Result<Vec<u8>, BindingError> {
        let encoding = match code {
            0 => ModelEncoding::WithVarianceFloors,
            1 => ModelEncoding::WithoutVarianceFloors,
            _ => return Err(error("invalid model encoding")),
        };
        let mut bytes = Vec::new();
        self.inner
            .write_with_encoding(&mut bytes, encoding)
            .map_err(error)?;
        Ok(bytes)
    }
}
pub struct DecodedStreamF32 {
    inner: StreamParameters<f32>,
    position: u64,
}
impl DecodedStreamF32 {
    pub fn position(&self) -> u64 {
        self.position
    }
    pub fn parameters(&self) -> StreamF32 {
        StreamF32 {
            inner: self.inner.clone(),
        }
    }
    pub fn into_parameters(self) -> StreamF32 {
        StreamF32 { inner: self.inner }
    }
}
impl StreamF64 {
    pub fn read(bytes: &[u8]) -> Result<Self, BindingError> {
        <StreamParameters<f64>>::read_from(bytes)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn read_with_limits(bytes: &[u8], max_array_entries: usize) -> Result<Self, BindingError> {
        <StreamParameters<f64>>::read_with_limits(bytes, DecodeLimits { max_array_entries })
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn read_prefix(bytes: &[u8]) -> Result<DecodedStreamF64, BindingError> {
        Self::read_prefix_with_limits(bytes, DecodeLimits::default().max_array_entries)
    }
    pub fn read_prefix_with_limits(
        bytes: &[u8],
        max_array_entries: usize,
    ) -> Result<DecodedStreamF64, BindingError> {
        let mut reader = std::io::Cursor::new(bytes);
        let inner = <StreamParameters<f64>>::read_prefix_with_limits(
            &mut reader,
            DecodeLimits { max_array_entries },
        )
        .map_err(error)?;
        Ok(DecodedStreamF64 {
            inner,
            position: reader.position(),
        })
    }
    pub fn write(&self) -> Result<Vec<u8>, BindingError> {
        let mut bytes = Vec::new();
        self.inner.write_to(&mut bytes).map_err(error)?;
        Ok(bytes)
    }
    pub fn write_with_encoding(&self, code: u32) -> Result<Vec<u8>, BindingError> {
        let encoding = match code {
            0 => ModelEncoding::WithVarianceFloors,
            1 => ModelEncoding::WithoutVarianceFloors,
            _ => return Err(error("invalid model encoding")),
        };
        let mut bytes = Vec::new();
        self.inner
            .write_with_encoding(&mut bytes, encoding)
            .map_err(error)?;
        Ok(bytes)
    }
}
pub struct DecodedStreamF64 {
    inner: StreamParameters<f64>,
    position: u64,
}
impl DecodedStreamF64 {
    pub fn position(&self) -> u64 {
        self.position
    }
    pub fn parameters(&self) -> StreamF64 {
        StreamF64 {
            inner: self.inner.clone(),
        }
    }
    pub fn into_parameters(self) -> StreamF64 {
        StreamF64 { inner: self.inner }
    }
}
impl ModelF32 {
    pub fn read(bytes: &[u8]) -> Result<Self, BindingError> {
        <ModelParameters<f32>>::read_from(bytes)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn read_with_limits(bytes: &[u8], max_array_entries: usize) -> Result<Self, BindingError> {
        <ModelParameters<f32>>::read_with_limits(bytes, DecodeLimits { max_array_entries })
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn read_prefix(bytes: &[u8]) -> Result<DecodedModelF32, BindingError> {
        Self::read_prefix_with_limits(bytes, DecodeLimits::default().max_array_entries)
    }
    pub fn read_prefix_with_limits(
        bytes: &[u8],
        max_array_entries: usize,
    ) -> Result<DecodedModelF32, BindingError> {
        let mut reader = std::io::Cursor::new(bytes);
        let inner = <ModelParameters<f32>>::read_prefix_with_limits(
            &mut reader,
            DecodeLimits { max_array_entries },
        )
        .map_err(error)?;
        Ok(DecodedModelF32 {
            inner,
            position: reader.position(),
        })
    }
    pub fn write(&self) -> Result<Vec<u8>, BindingError> {
        let mut bytes = Vec::new();
        self.inner.write_to(&mut bytes).map_err(error)?;
        Ok(bytes)
    }
    pub fn write_with_encoding(&self, code: u32) -> Result<Vec<u8>, BindingError> {
        let encoding = match code {
            0 => ModelEncoding::WithVarianceFloors,
            1 => ModelEncoding::WithoutVarianceFloors,
            _ => return Err(error("invalid model encoding")),
        };
        let mut bytes = Vec::new();
        self.inner
            .write_with_encoding(&mut bytes, encoding)
            .map_err(error)?;
        Ok(bytes)
    }
}
pub struct DecodedModelF32 {
    inner: ModelParameters<f32>,
    position: u64,
}
impl DecodedModelF32 {
    pub fn position(&self) -> u64 {
        self.position
    }
    pub fn parameters(&self) -> ModelF32 {
        ModelF32 {
            inner: self.inner.clone(),
        }
    }
    pub fn into_parameters(self) -> ModelF32 {
        ModelF32 { inner: self.inner }
    }
}
impl ModelF64 {
    pub fn read(bytes: &[u8]) -> Result<Self, BindingError> {
        <ModelParameters<f64>>::read_from(bytes)
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn read_with_limits(bytes: &[u8], max_array_entries: usize) -> Result<Self, BindingError> {
        <ModelParameters<f64>>::read_with_limits(bytes, DecodeLimits { max_array_entries })
            .map(|inner| Self { inner })
            .map_err(error)
    }
    pub fn read_prefix(bytes: &[u8]) -> Result<DecodedModelF64, BindingError> {
        Self::read_prefix_with_limits(bytes, DecodeLimits::default().max_array_entries)
    }
    pub fn read_prefix_with_limits(
        bytes: &[u8],
        max_array_entries: usize,
    ) -> Result<DecodedModelF64, BindingError> {
        let mut reader = std::io::Cursor::new(bytes);
        let inner = <ModelParameters<f64>>::read_prefix_with_limits(
            &mut reader,
            DecodeLimits { max_array_entries },
        )
        .map_err(error)?;
        Ok(DecodedModelF64 {
            inner,
            position: reader.position(),
        })
    }
    pub fn write(&self) -> Result<Vec<u8>, BindingError> {
        let mut bytes = Vec::new();
        self.inner.write_to(&mut bytes).map_err(error)?;
        Ok(bytes)
    }
    pub fn write_with_encoding(&self, code: u32) -> Result<Vec<u8>, BindingError> {
        let encoding = match code {
            0 => ModelEncoding::WithVarianceFloors,
            1 => ModelEncoding::WithoutVarianceFloors,
            _ => return Err(error("invalid model encoding")),
        };
        let mut bytes = Vec::new();
        self.inner
            .write_with_encoding(&mut bytes, encoding)
            .map_err(error)?;
        Ok(bytes)
    }
}
pub struct DecodedModelF64 {
    inner: ModelParameters<f64>,
    position: u64,
}
impl DecodedModelF64 {
    pub fn position(&self) -> u64 {
        self.position
    }
    pub fn parameters(&self) -> ModelF64 {
        ModelF64 {
            inner: self.inner.clone(),
        }
    }
    pub fn into_parameters(self) -> ModelF64 {
        ModelF64 { inner: self.inner }
    }
}
