use super::*;
#[napi]
pub struct SegmentationCollectionF32 {
    pub(crate) inner: std::cell::RefCell<Option<crate::api::SegmentationCollectionF32>>,
}
#[napi]
impl SegmentationCollectionF32 {
    #[napi]
    pub fn close(&self) -> napi::Result<()> {
        *self.inner.try_borrow_mut().map_err(borrowed)? = None;
        Ok(())
    }
    #[napi]
    pub fn free(&self) -> napi::Result<()> {
        self.close()
    }
    #[napi(getter, js_name = "is_closed")]
    pub fn is_closed(&self) -> napi::Result<bool> {
        Ok(self.inner.try_borrow().map_err(borrowed)?.is_none())
    }
}
#[napi]
impl SegmentationCollectionF32 {
    #[napi(constructor)]
    pub fn binding_new() -> napi::Result<Self> {
        Ok(SegmentationCollectionF32 {
            inner: std::cell::RefCell::new(Some(crate::api::SegmentationCollectionF32::new())),
        })
    }
    #[napi(getter, js_name = "length")]
    pub fn binding_length(&self) -> napi::Result<f64> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .length() as f64)
    }
    #[napi(js_name = "is_empty")]
    pub fn binding_is_empty(&self) -> napi::Result<bool> {
        Ok((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .is_empty())
    }
    #[napi(js_name = "push")]
    pub fn binding_push(&self, value: &SegmentationF32) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .push(
            value
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        );
        Ok(())
    }
    #[napi(js_name = "get")]
    pub fn binding_get(&self, index: f64) -> napi::Result<Option<SegmentationF32>> {
        Ok(((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .get(checked_usize(index)?))
        .map(|value| SegmentationF32 {
            inner: std::cell::RefCell::new(Some(value)),
        }))
    }
    #[napi(js_name = "replace")]
    pub fn binding_replace(&self, index: f64, value: &SegmentationF32) -> napi::Result<()> {
        (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .replace(
            checked_usize(index)?,
            value
                .inner
                .try_borrow()
                .map_err(borrowed)?
                .as_ref()
                .ok_or_else(consumed)?,
        )
    }
    #[napi(js_name = "clear")]
    pub fn binding_clear(&self) -> napi::Result<()> {
        let _: () = (self
            .inner
            .try_borrow_mut()
            .map_err(borrowed)?
            .as_mut()
            .ok_or_else(consumed)?)
        .clear();
        Ok(())
    }
    #[napi(js_name = "cloned")]
    pub fn binding_cloned(&self) -> napi::Result<Self> {
        Ok(SegmentationCollectionF32 {
            inner: std::cell::RefCell::new(Some(
                (self
                    .inner
                    .try_borrow()
                    .map_err(borrowed)?
                    .as_ref()
                    .ok_or_else(consumed)?)
                .cloned(),
            )),
        })
    }
}
#[cfg(feature = "serialization")]
#[napi]
impl SegmentationCollectionF32 {
    #[napi(js_name = "read")]
    pub fn binding_read(bytes: napi::bindgen_prelude::Uint8Array) -> napi::Result<Self> {
        let storage_bytes = bytes.to_vec();
        (crate::api::SegmentationCollectionF32::read(&storage_bytes)).map(|value| {
            SegmentationCollectionF32 {
                inner: std::cell::RefCell::new(Some(value)),
            }
        })
    }
    #[napi(js_name = "read_with_limits")]
    pub fn binding_read_with_limits(
        bytes: napi::bindgen_prelude::Uint8Array,
        max_array_entries: f64,
    ) -> napi::Result<Self> {
        let storage_bytes = bytes.to_vec();
        (crate::api::SegmentationCollectionF32::read_with_limits(
            &storage_bytes,
            checked_usize(max_array_entries)?,
        ))
        .map(|value| SegmentationCollectionF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "read_prefix")]
    pub fn binding_read_prefix(
        bytes: napi::bindgen_prelude::Uint8Array,
    ) -> napi::Result<DecodedSegmentationSetF32> {
        let storage_bytes = bytes.to_vec();
        (crate::api::SegmentationCollectionF32::read_prefix(&storage_bytes)).map(|value| {
            DecodedSegmentationSetF32 {
                inner: std::cell::RefCell::new(Some(value)),
            }
        })
    }
    #[napi(js_name = "read_prefix_with_limits")]
    pub fn binding_read_prefix_with_limits(
        bytes: napi::bindgen_prelude::Uint8Array,
        max_array_entries: f64,
    ) -> napi::Result<DecodedSegmentationSetF32> {
        let storage_bytes = bytes.to_vec();
        (crate::api::SegmentationCollectionF32::read_prefix_with_limits(
            &storage_bytes,
            checked_usize(max_array_entries)?,
        ))
        .map(|value| DecodedSegmentationSetF32 {
            inner: std::cell::RefCell::new(Some(value)),
        })
    }
    #[napi(js_name = "write")]
    pub fn binding_write(&self) -> napi::Result<napi::bindgen_prelude::Uint8Array> {
        ((self
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?)
        .write())
        .map(<napi::bindgen_prelude::Uint8Array>::from)
    }
}
