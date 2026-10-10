use super::*;
#[pyfunction(name = "uniform_sample_f32")]
#[pyo3(signature = (callback))]
pub fn binding_uniform_sample_f32(callback: Py<PyAny>) -> pyo3::PyResult<f32> {
    crate::api::generation::uniform_sample_f32(&crate::callbacks::Function::new(callback)?)
}
#[pyfunction(name = "uniform_sample_f64")]
#[pyo3(signature = (callback))]
pub fn binding_uniform_sample_f64(callback: Py<PyAny>) -> pyo3::PyResult<f64> {
    crate::api::generation::uniform_sample_f64(&crate::callbacks::Function::new(callback)?)
}
