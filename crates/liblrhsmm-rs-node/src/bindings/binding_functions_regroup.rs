use super::*;
#[napi(js_name = "regroup_data_f32")]
pub fn binding_regroup_data_f32(
    observations: &ObservationCollectionF32,
    segmentations: &SegmentationCollectionF32,
    segments_per_unit: f64,
) -> napi::Result<DatasetF32> {
    (crate::api::datasets::regroup_data_f32(
        observations
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
        segmentations
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
        checked_usize(segments_per_unit)?,
    ))
    .map(|value| DatasetF32 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
#[napi(js_name = "regroup_data_f64")]
pub fn binding_regroup_data_f64(
    observations: &ObservationCollectionF64,
    segmentations: &SegmentationCollectionF64,
    segments_per_unit: f64,
) -> napi::Result<DatasetF64> {
    (crate::api::datasets::regroup_data_f64(
        observations
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
        segmentations
            .inner
            .try_borrow()
            .map_err(borrowed)?
            .as_ref()
            .ok_or_else(consumed)?,
        checked_usize(segments_per_unit)?,
    ))
    .map(|value| DatasetF64 {
        inner: std::cell::RefCell::new(Some(value)),
    })
}
