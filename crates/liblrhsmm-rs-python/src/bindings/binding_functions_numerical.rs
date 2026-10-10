use super::*;
#[pyfunction(name = "numerical_constants")]
#[pyo3(signature = ())]
pub fn binding_numerical_constants() -> pyo3::PyResult<Vec<f64>> {
    Ok(crate::api::numerics::numerical_constants())
}
#[pyfunction(name = "numerical_log_add_f32")]
#[pyo3(signature = (a, b))]
pub fn binding_numerical_log_add_f32(a: f32, b: f32) -> pyo3::PyResult<f32> {
    Ok(crate::api::numerics::numerical_log_add_f32(a, b))
}
#[pyfunction(name = "numerical_negative_exponential_f32")]
#[pyo3(signature = (value))]
pub fn binding_numerical_negative_exponential_f32(value: f32) -> pyo3::PyResult<f32> {
    Ok(crate::api::numerics::numerical_negative_exponential_f32(
        value,
    ))
}
#[pyfunction(name = "numerical_exponential_f32")]
#[pyo3(signature = (value))]
pub fn binding_numerical_exponential_f32(value: f32) -> pyo3::PyResult<f32> {
    Ok(crate::api::numerics::numerical_exponential_f32(value))
}
#[pyfunction(name = "numerical_log_add_table_f32")]
#[pyo3(signature = ())]
pub fn binding_numerical_log_add_table_f32() -> pyo3::PyResult<NumericalIteratorF32> {
    Ok(NumericalIteratorF32 {
        inner: Some(crate::api::numerics::numerical_log_add_table_f32()),
    })
}
#[pyfunction(name = "numerical_exponential_table_f32")]
#[pyo3(signature = ())]
pub fn binding_numerical_exponential_table_f32() -> pyo3::PyResult<NumericalIteratorF32> {
    Ok(NumericalIteratorF32 {
        inner: Some(crate::api::numerics::numerical_exponential_table_f32()),
    })
}
#[pyfunction(name = "numerical_log_add_f64")]
#[pyo3(signature = (a, b))]
pub fn binding_numerical_log_add_f64(a: f64, b: f64) -> pyo3::PyResult<f64> {
    Ok(crate::api::numerics::numerical_log_add_f64(a, b))
}
#[pyfunction(name = "numerical_negative_exponential_f64")]
#[pyo3(signature = (value))]
pub fn binding_numerical_negative_exponential_f64(value: f64) -> pyo3::PyResult<f64> {
    Ok(crate::api::numerics::numerical_negative_exponential_f64(
        value,
    ))
}
#[pyfunction(name = "numerical_exponential_f64")]
#[pyo3(signature = (value))]
pub fn binding_numerical_exponential_f64(value: f64) -> pyo3::PyResult<f64> {
    Ok(crate::api::numerics::numerical_exponential_f64(value))
}
#[pyfunction(name = "numerical_log_add_table_f64")]
#[pyo3(signature = ())]
pub fn binding_numerical_log_add_table_f64() -> pyo3::PyResult<NumericalIteratorF64> {
    Ok(NumericalIteratorF64 {
        inner: Some(crate::api::numerics::numerical_log_add_table_f64()),
    })
}
#[pyfunction(name = "numerical_exponential_table_f64")]
#[pyo3(signature = ())]
pub fn binding_numerical_exponential_table_f64() -> pyo3::PyResult<NumericalIteratorF64> {
    Ok(NumericalIteratorF64 {
        inner: Some(crate::api::numerics::numerical_exponential_table_f64()),
    })
}
