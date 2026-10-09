use super::{ObservationF32, ObservationF64, SegmentationF32, SegmentationF64};
use crate::data::{ObservationData, SegmentationData};
use crate::serial::DecodeLimits;
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
codec!(ObservationF32, DecodedObservationF32, ObservationData<f32>);
codec!(ObservationF64, DecodedObservationF64, ObservationData<f64>);
codec!(
    SegmentationF32,
    DecodedSegmentationF32,
    SegmentationData<f32>
);
codec!(
    SegmentationF64,
    DecodedSegmentationF64,
    SegmentationData<f64>
);
