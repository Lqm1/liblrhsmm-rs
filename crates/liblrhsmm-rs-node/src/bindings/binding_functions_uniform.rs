use super::*;
#[napi(js_name = "uniform_sample_f32")]
pub fn binding_uniform_sample_f32(
    #[napi(ts_arg_type = "() => number")] callback: napi::bindgen_prelude::Unknown<'_>,
    env: napi::Env,
) -> napi::Result<f64> {
    (crate::api::generation::uniform_sample_f32(&crate::callbacks::Function::new(env, callback)?))
        .map(|value| value as f64)
}
#[napi(js_name = "uniform_sample_f64")]
pub fn binding_uniform_sample_f64(
    #[napi(ts_arg_type = "() => number")] callback: napi::bindgen_prelude::Unknown<'_>,
    env: napi::Env,
) -> napi::Result<f64> {
    crate::api::generation::uniform_sample_f64(&crate::callbacks::Function::new(env, callback)?)
}
