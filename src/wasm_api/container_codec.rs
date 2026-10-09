use super::{ModelF32, ModelF64, StreamF32, StreamF64};
use crate::model::{ModelParameters, StreamParameters};
use crate::serial::{DecodeLimits, ModelEncoding};
use wasm_bindgen::prelude::*;
fn error(value: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&value.to_string())
}
macro_rules! codec {
    ($owner:ident, $decoded:ident, $native:ty) => {
        #[wasm_bindgen]
        impl $owner {
            pub fn read(bytes: &[u8]) -> Result<Self, JsValue> {
                <$native>::read_from(bytes)
                    .map(|inner| Self { inner })
                    .map_err(error)
            }
            pub fn read_with_limits(
                bytes: &[u8],
                max_array_entries: usize,
            ) -> Result<Self, JsValue> {
                <$native>::read_with_limits(bytes, DecodeLimits { max_array_entries })
                    .map(|inner| Self { inner })
                    .map_err(error)
            }
            pub fn read_prefix(bytes: &[u8]) -> Result<$decoded, JsValue> {
                Self::read_prefix_with_limits(bytes, DecodeLimits::default().max_array_entries)
            }
            pub fn read_prefix_with_limits(
                bytes: &[u8],
                max_array_entries: usize,
            ) -> Result<$decoded, JsValue> {
                let mut reader = std::io::Cursor::new(bytes);
                let inner = <$native>::read_prefix_with_limits(
                    &mut reader,
                    DecodeLimits { max_array_entries },
                )
                .map_err(error)?;
                Ok($decoded {
                    inner,
                    position: reader.position(),
                })
            }
            pub fn write(&self) -> Result<Vec<u8>, JsValue> {
                let mut bytes = Vec::new();
                self.inner.write_to(&mut bytes).map_err(error)?;
                Ok(bytes)
            }
            pub fn write_with_encoding(&self, code: u32) -> Result<Vec<u8>, JsValue> {
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
        #[wasm_bindgen]
        pub struct $decoded {
            inner: $native,
            position: u64,
        }
        #[wasm_bindgen]
        impl $decoded {
            #[wasm_bindgen(getter)]
            pub fn position(&self) -> u64 {
                self.position
            }
            pub fn parameters(&self) -> $owner {
                $owner {
                    inner: self.inner.clone(),
                }
            }
            pub fn into_parameters(self) -> $owner {
                $owner { inner: self.inner }
            }
        }
    };
}
codec!(StreamF32, DecodedStreamF32, StreamParameters<f32>);
codec!(StreamF64, DecodedStreamF64, StreamParameters<f64>);
codec!(ModelF32, DecodedModelF32, ModelParameters<f32>);
codec!(ModelF64, DecodedModelF64, ModelParameters<f64>);
