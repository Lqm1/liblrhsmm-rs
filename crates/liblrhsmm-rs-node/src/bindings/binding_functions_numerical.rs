use super::*;
#[napi(js_name = "numerical_constants")]
pub fn binding_numerical_constants() -> napi::Result<napi::bindgen_prelude::Float64Array> {
    Ok(<napi::bindgen_prelude::Float64Array>::from(
        crate::api::numerics::numerical_constants(),
    ))
}
#[napi(js_name = "numerical_log_add_f32")]
pub fn binding_numerical_log_add_f32(a: f64, b: f64) -> napi::Result<f64> {
    Ok(crate::api::numerics::numerical_log_add_f32(a as f32, b as f32) as f64)
}
#[napi(js_name = "numerical_negative_exponential_f32")]
pub fn binding_numerical_negative_exponential_f32(value: f64) -> napi::Result<f64> {
    Ok(crate::api::numerics::numerical_negative_exponential_f32(value as f32) as f64)
}
#[napi(js_name = "numerical_exponential_f32")]
pub fn binding_numerical_exponential_f32(value: f64) -> napi::Result<f64> {
    Ok(crate::api::numerics::numerical_exponential_f32(value as f32) as f64)
}
#[napi(js_name = "numerical_log_add_table_f32")]
pub fn binding_numerical_log_add_table_f32() -> napi::Result<NumericalIteratorF32> {
    Ok(NumericalIteratorF32 {
        inner: std::cell::RefCell::new(Some(crate::api::numerics::numerical_log_add_table_f32())),
    })
}
#[napi(js_name = "numerical_exponential_table_f32")]
pub fn binding_numerical_exponential_table_f32() -> napi::Result<NumericalIteratorF32> {
    Ok(NumericalIteratorF32 {
        inner: std::cell::RefCell::new(Some(
            crate::api::numerics::numerical_exponential_table_f32(),
        )),
    })
}
#[napi(js_name = "numerical_log_add_f64")]
pub fn binding_numerical_log_add_f64(a: f64, b: f64) -> napi::Result<f64> {
    Ok(crate::api::numerics::numerical_log_add_f64(a, b))
}
#[napi(js_name = "numerical_negative_exponential_f64")]
pub fn binding_numerical_negative_exponential_f64(value: f64) -> napi::Result<f64> {
    Ok(crate::api::numerics::numerical_negative_exponential_f64(
        value,
    ))
}
#[napi(js_name = "numerical_exponential_f64")]
pub fn binding_numerical_exponential_f64(value: f64) -> napi::Result<f64> {
    Ok(crate::api::numerics::numerical_exponential_f64(value))
}
#[napi(js_name = "numerical_log_add_table_f64")]
pub fn binding_numerical_log_add_table_f64() -> napi::Result<NumericalIteratorF64> {
    Ok(NumericalIteratorF64 {
        inner: std::cell::RefCell::new(Some(crate::api::numerics::numerical_log_add_table_f64())),
    })
}
#[napi(js_name = "numerical_exponential_table_f64")]
pub fn binding_numerical_exponential_table_f64() -> napi::Result<NumericalIteratorF64> {
    Ok(NumericalIteratorF64 {
        inner: std::cell::RefCell::new(Some(
            crate::api::numerics::numerical_exponential_table_f64(),
        )),
    })
}
