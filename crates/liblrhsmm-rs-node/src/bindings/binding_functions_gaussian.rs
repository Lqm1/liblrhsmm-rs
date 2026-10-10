use super::*;
#[napi(js_name = "gaussian_sample_f32")]
pub fn binding_gaussian_sample_f32(
    mean: f64,
    variance: f64,
    radial: f64,
    angular: f64,
) -> napi::Result<f64> {
    (crate::api::generation::gaussian_sample_f32(
        mean as f32,
        variance as f32,
        radial as f32,
        angular as f32,
    ))
    .map(|value| value as f64)
}
#[napi(js_name = "gaussian_sample_f64")]
pub fn binding_gaussian_sample_f64(
    mean: f64,
    variance: f64,
    radial: f64,
    angular: f64,
) -> napi::Result<f64> {
    crate::api::generation::gaussian_sample_f64(mean, variance, radial, angular)
}
