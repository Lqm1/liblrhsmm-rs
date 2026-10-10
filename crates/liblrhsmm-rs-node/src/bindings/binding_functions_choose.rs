use super::*;
#[napi(js_name = "choose_component_f32")]
pub fn binding_choose_component_f32(
    weights: napi::bindgen_prelude::Float32Array,
    uniform: f64,
) -> napi::Result<f64> {
    let storage_weights = weights.to_vec();
    (crate::api::generation::choose_component_f32(&storage_weights, uniform as f32))
        .map(|value| value as f64)
}
#[napi(js_name = "choose_component_f64")]
pub fn binding_choose_component_f64(
    weights: napi::bindgen_prelude::Float64Array,
    uniform: f64,
) -> napi::Result<f64> {
    let storage_weights = weights.to_vec();
    (crate::api::generation::choose_component_f64(&storage_weights, uniform))
        .map(|value| value as f64)
}
