use super::*;
#[pyfunction(name = "regroup_data_f32")]
#[pyo3(signature = (observations, segmentations, segments_per_unit))]
pub fn binding_regroup_data_f32(
    observations: pyo3::PyRef<'_, ObservationCollectionF32>,
    segmentations: pyo3::PyRef<'_, SegmentationCollectionF32>,
    segments_per_unit: usize,
) -> pyo3::PyResult<DatasetF32> {
    (crate::api::datasets::regroup_data_f32(
        observations.inner.as_ref().ok_or_else(consumed)?,
        segmentations.inner.as_ref().ok_or_else(consumed)?,
        segments_per_unit,
    ))
    .map(|value| DatasetF32 { inner: Some(value) })
}
#[pyfunction(name = "regroup_data_f64")]
#[pyo3(signature = (observations, segmentations, segments_per_unit))]
pub fn binding_regroup_data_f64(
    observations: pyo3::PyRef<'_, ObservationCollectionF64>,
    segmentations: pyo3::PyRef<'_, SegmentationCollectionF64>,
    segments_per_unit: usize,
) -> pyo3::PyResult<DatasetF64> {
    (crate::api::datasets::regroup_data_f64(
        observations.inner.as_ref().ok_or_else(consumed)?,
        segmentations.inner.as_ref().ok_or_else(consumed)?,
        segments_per_unit,
    ))
    .map(|value| DatasetF64 { inner: Some(value) })
}
