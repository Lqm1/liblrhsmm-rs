use super::*;
#[pyfunction(name = "choose_component_f32")]
#[pyo3(signature = (weights, uniform))]
pub fn binding_choose_component_f32(weights: Vec<f32>, uniform: f32) -> pyo3::PyResult<usize> {
    crate::api::generation::choose_component_f32(&weights, uniform)
}
#[pyfunction(name = "choose_component_f64")]
#[pyo3(signature = (weights, uniform))]
pub fn binding_choose_component_f64(weights: Vec<f64>, uniform: f64) -> pyo3::PyResult<usize> {
    crate::api::generation::choose_component_f64(&weights, uniform)
}
