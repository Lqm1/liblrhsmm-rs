use super::{
    ObservationCollectionF32, ObservationCollectionF64, SegmentationCollectionF32,
    SegmentationCollectionF64,
};
use crate::data::{ObservationData, SegmentationData};
use crate::serial;
use wasm_bindgen::prelude::*;
fn error(value: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&value.to_string())
}
macro_rules! codec {
    ($scalar:ty, $collection:ident, $decoded:ident, $sample:ty, $read:ident, $limited:ident, $prefix:ident, $prefix_limited:ident, $write:ident) => {
        #[wasm_bindgen]
        impl $collection {
            pub fn read(bytes: &[u8]) -> Result<Self, JsValue> {
                serial::$read::<$scalar>(bytes)
                    .map(|inner| Self { inner })
                    .map_err(error)
            }
            pub fn read_with_limits(
                bytes: &[u8],
                max_array_entries: usize,
            ) -> Result<Self, JsValue> {
                serial::$limited::<$scalar>(bytes, serial::DecodeLimits { max_array_entries })
                    .map(|inner| Self { inner })
                    .map_err(error)
            }
            pub fn read_prefix(bytes: &[u8]) -> Result<$decoded, JsValue> {
                let mut reader = std::io::Cursor::new(bytes);
                let inner = serial::$prefix::<$scalar>(&mut reader).map_err(error)?;
                Ok($decoded {
                    inner,
                    position: reader.position(),
                })
            }
            pub fn read_prefix_with_limits(
                bytes: &[u8],
                max_array_entries: usize,
            ) -> Result<$decoded, JsValue> {
                let mut reader = std::io::Cursor::new(bytes);
                let inner = serial::$prefix_limited::<$scalar>(
                    &mut reader,
                    serial::DecodeLimits { max_array_entries },
                )
                .map_err(error)?;
                Ok($decoded {
                    inner,
                    position: reader.position(),
                })
            }
            pub fn write(&self) -> Result<Vec<u8>, JsValue> {
                let mut bytes = Vec::new();
                serial::$write::<$scalar>(&self.inner, &mut bytes).map_err(error)?;
                Ok(bytes)
            }
        }
        #[wasm_bindgen]
        pub struct $decoded {
            inner: Vec<$sample>,
            position: u64,
        }
        #[wasm_bindgen]
        impl $decoded {
            #[wasm_bindgen(getter)]
            pub fn position(&self) -> u64 {
                self.position
            }
            pub fn samples(&self) -> $collection {
                $collection {
                    inner: self.inner.clone(),
                }
            }
            pub fn into_samples(self) -> $collection {
                $collection { inner: self.inner }
            }
        }
    };
}
codec!(
    f32,
    ObservationCollectionF32,
    DecodedObservationSetF32,
    ObservationData<f32>,
    read_observation_set_with_precision,
    read_observation_set_with_limits,
    read_observation_set_prefix_with_precision,
    read_observation_set_prefix_with_limits,
    write_observation_set_with_precision
);
codec!(
    f64,
    ObservationCollectionF64,
    DecodedObservationSetF64,
    ObservationData<f64>,
    read_observation_set_with_precision,
    read_observation_set_with_limits,
    read_observation_set_prefix_with_precision,
    read_observation_set_prefix_with_limits,
    write_observation_set_with_precision
);
codec!(
    f32,
    SegmentationCollectionF32,
    DecodedSegmentationSetF32,
    SegmentationData<f32>,
    read_segmentation_set_with_precision,
    read_segmentation_set_with_limits,
    read_segmentation_set_prefix_with_precision,
    read_segmentation_set_prefix_with_limits,
    write_segmentation_set_with_precision
);
codec!(
    f64,
    SegmentationCollectionF64,
    DecodedSegmentationSetF64,
    SegmentationData<f64>,
    read_segmentation_set_with_precision,
    read_segmentation_set_with_limits,
    read_segmentation_set_prefix_with_precision,
    read_segmentation_set_prefix_with_limits,
    write_segmentation_set_with_precision
);
