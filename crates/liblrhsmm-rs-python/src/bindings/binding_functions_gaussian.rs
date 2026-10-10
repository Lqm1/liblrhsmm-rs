use super::*;
#[pyfunction(name = "gaussian_sample_f32")]
#[pyo3(signature = (mean, variance, radial, angular))]
pub fn binding_gaussian_sample_f32(
    mean: f32,
    variance: f32,
    radial: f32,
    angular: f32,
) -> pyo3::PyResult<f32> {
    crate::api::generation::gaussian_sample_f32(mean, variance, radial, angular)
}
#[pyfunction(name = "gaussian_sample_f64")]
#[pyo3(signature = (mean, variance, radial, angular))]
pub fn binding_gaussian_sample_f64(
    mean: f64,
    variance: f64,
    radial: f64,
    angular: f64,
) -> pyo3::PyResult<f64> {
    crate::api::generation::gaussian_sample_f64(mean, variance, radial, angular)
}
