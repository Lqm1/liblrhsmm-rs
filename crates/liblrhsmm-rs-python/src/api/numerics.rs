use crate::numerics;
use pyo3::PyErr as BindingError;
#[derive(Clone, Copy, PartialEq)]
#[pyo3::pyclass(eq, eq_int, from_py_object, module = "liblrhsmm_rs")]
pub enum NumericalTable {
    LogAdd = 0,
    Exponential = 1,
}
fn table(code: u32) -> Result<bool, BindingError> {
    match code {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(pyo3::exceptions::PyValueError::new_err(
            "invalid numerical coefficient table",
        )),
    }
}
pub fn numerical_constants() -> Vec<f64> {
    vec![
        numerics::LOG_ADD_TABLE_SIZE as f64,
        numerics::LOG_ADD_TABLE_STEP,
        numerics::EXPONENTIAL_TABLE_SIZE as f64,
        numerics::EXPONENTIAL_TABLE_STEP,
        numerics::OCCUPANCY_EPSILON,
        numerics::LOG_ZERO as f64,
        numerics::LOG_ZERO_F64,
    ]
}
pub struct NumericalTableF32 {
    exponential: bool,
}
impl NumericalTableF32 {
    pub fn new(code: u32) -> Result<Self, BindingError> {
        Ok(Self {
            exponential: table(code)?,
        })
    }
    pub fn length(&self) -> usize {
        if self.exponential {
            numerics::EXPONENTIAL_COEFFICIENTS.len()
        } else {
            numerics::LOG_ADD_COEFFICIENTS.len()
        }
    }
    pub fn is_empty(&self) -> bool {
        if self.exponential {
            numerics::EXPONENTIAL_COEFFICIENTS.is_empty()
        } else {
            numerics::LOG_ADD_COEFFICIENTS.is_empty()
        }
    }
    pub fn get(&self, index: usize) -> Option<f32> {
        if self.exponential {
            numerics::EXPONENTIAL_COEFFICIENTS.get(index)
        } else {
            numerics::LOG_ADD_COEFFICIENTS.get(index)
        }
    }
    pub fn replace(&self, index: usize, value: f32) -> Option<f32> {
        if self.exponential {
            numerics::EXPONENTIAL_COEFFICIENTS.replace(index, value)
        } else {
            numerics::LOG_ADD_COEFFICIENTS.replace(index, value)
        }
    }
    pub fn get_storage(&self, index: usize) -> Vec<f32> {
        self.get(index).into_iter().collect()
    }
    pub fn replace_storage(&self, index: usize, values: &[f32]) -> Result<Vec<f32>, BindingError> {
        let [value]: [f32; 1] = values.try_into().map_err(|_| {
            pyo3::exceptions::PyValueError::new_err("coefficient replacement requires one value")
        })?;
        Ok(self.replace(index, value).into_iter().collect())
    }
    pub fn iter(&self) -> NumericalIteratorF32 {
        NumericalIteratorF32 {
            exponential: self.exponential,
            front: 0,
            back: self.length(),
        }
    }
    pub fn values(&self) -> Vec<f32> {
        if self.exponential {
            numerics::EXPONENTIAL_COEFFICIENTS.iter().collect()
        } else {
            numerics::LOG_ADD_COEFFICIENTS.iter().collect()
        }
    }
    /// Cloning retains a handle to the same module-wide native table.
    pub fn cloned(&self) -> Self {
        Self {
            exponential: self.exponential,
        }
    }
}
pub struct NumericalIteratorF32 {
    exponential: bool,
    front: usize,
    back: usize,
}
impl Iterator for NumericalIteratorF32 {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            return None;
        }
        let index = self.front;
        self.front += 1;
        if self.exponential {
            numerics::EXPONENTIAL_COEFFICIENTS.get(index)
        } else {
            numerics::LOG_ADD_COEFFICIENTS.get(index)
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = self.back - self.front;
        (length, Some(length))
    }
}
impl DoubleEndedIterator for NumericalIteratorF32 {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            return None;
        }
        self.back -= 1;
        if self.exponential {
            numerics::EXPONENTIAL_COEFFICIENTS.get(self.back)
        } else {
            numerics::LOG_ADD_COEFFICIENTS.get(self.back)
        }
    }
}
impl ExactSizeIterator for NumericalIteratorF32 {}
impl std::iter::FusedIterator for NumericalIteratorF32 {}
impl NumericalIteratorF32 {
    pub fn length(&self) -> usize {
        self.back - self.front
    }
    pub fn next_value(&mut self) -> Option<f32> {
        Iterator::next(self)
    }
    pub fn next_back_value(&mut self) -> Option<f32> {
        DoubleEndedIterator::next_back(self)
    }
    pub fn next_storage(&mut self) -> Vec<f32> {
        self.next().into_iter().collect()
    }
    pub fn next_back_storage(&mut self) -> Vec<f32> {
        self.next_back().into_iter().collect()
    }
}
pub fn numerical_log_add_f32(a: f32, b: f32) -> f32 {
    numerics::log_add(a, b)
}
pub fn numerical_negative_exponential_f32(value: f32) -> f32 {
    numerics::negative_exponential(value)
}
pub fn numerical_exponential_f32(value: f32) -> f32 {
    numerics::exponential(value)
}
pub fn numerical_log_add_table_f32() -> NumericalIteratorF32 {
    NumericalIteratorF32 {
        exponential: false,
        front: 0,
        back: numerics::LOG_ADD_TABLE_SIZE,
    }
}
pub fn numerical_exponential_table_f32() -> NumericalIteratorF32 {
    NumericalIteratorF32 {
        exponential: true,
        front: 0,
        back: numerics::EXPONENTIAL_TABLE_SIZE,
    }
}
pub struct NumericalTableF64 {
    exponential: bool,
}
impl NumericalTableF64 {
    pub fn new(code: u32) -> Result<Self, BindingError> {
        Ok(Self {
            exponential: table(code)?,
        })
    }
    pub fn length(&self) -> usize {
        if self.exponential {
            numerics::EXPONENTIAL_COEFFICIENTS_F64.len()
        } else {
            numerics::LOG_ADD_COEFFICIENTS_F64.len()
        }
    }
    pub fn is_empty(&self) -> bool {
        if self.exponential {
            numerics::EXPONENTIAL_COEFFICIENTS_F64.is_empty()
        } else {
            numerics::LOG_ADD_COEFFICIENTS_F64.is_empty()
        }
    }
    pub fn get(&self, index: usize) -> Option<f64> {
        if self.exponential {
            numerics::EXPONENTIAL_COEFFICIENTS_F64.get(index)
        } else {
            numerics::LOG_ADD_COEFFICIENTS_F64.get(index)
        }
    }
    pub fn replace(&self, index: usize, value: f64) -> Option<f64> {
        if self.exponential {
            numerics::EXPONENTIAL_COEFFICIENTS_F64.replace(index, value)
        } else {
            numerics::LOG_ADD_COEFFICIENTS_F64.replace(index, value)
        }
    }
    pub fn get_storage(&self, index: usize) -> Vec<f64> {
        self.get(index).into_iter().collect()
    }
    pub fn replace_storage(&self, index: usize, values: &[f64]) -> Result<Vec<f64>, BindingError> {
        let [value]: [f64; 1] = values.try_into().map_err(|_| {
            pyo3::exceptions::PyValueError::new_err("coefficient replacement requires one value")
        })?;
        Ok(self.replace(index, value).into_iter().collect())
    }
    pub fn iter(&self) -> NumericalIteratorF64 {
        NumericalIteratorF64 {
            exponential: self.exponential,
            front: 0,
            back: self.length(),
        }
    }
    pub fn values(&self) -> Vec<f64> {
        if self.exponential {
            numerics::EXPONENTIAL_COEFFICIENTS_F64.iter().collect()
        } else {
            numerics::LOG_ADD_COEFFICIENTS_F64.iter().collect()
        }
    }
    /// Cloning retains a handle to the same module-wide native table.
    pub fn cloned(&self) -> Self {
        Self {
            exponential: self.exponential,
        }
    }
}
pub struct NumericalIteratorF64 {
    exponential: bool,
    front: usize,
    back: usize,
}
impl Iterator for NumericalIteratorF64 {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            return None;
        }
        let index = self.front;
        self.front += 1;
        if self.exponential {
            numerics::EXPONENTIAL_COEFFICIENTS_F64.get(index)
        } else {
            numerics::LOG_ADD_COEFFICIENTS_F64.get(index)
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = self.back - self.front;
        (length, Some(length))
    }
}
impl DoubleEndedIterator for NumericalIteratorF64 {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            return None;
        }
        self.back -= 1;
        if self.exponential {
            numerics::EXPONENTIAL_COEFFICIENTS_F64.get(self.back)
        } else {
            numerics::LOG_ADD_COEFFICIENTS_F64.get(self.back)
        }
    }
}
impl ExactSizeIterator for NumericalIteratorF64 {}
impl std::iter::FusedIterator for NumericalIteratorF64 {}
impl NumericalIteratorF64 {
    pub fn length(&self) -> usize {
        self.back - self.front
    }
    pub fn next_value(&mut self) -> Option<f64> {
        Iterator::next(self)
    }
    pub fn next_back_value(&mut self) -> Option<f64> {
        DoubleEndedIterator::next_back(self)
    }
    pub fn next_storage(&mut self) -> Vec<f64> {
        self.next().into_iter().collect()
    }
    pub fn next_back_storage(&mut self) -> Vec<f64> {
        self.next_back().into_iter().collect()
    }
}
pub fn numerical_log_add_f64(a: f64, b: f64) -> f64 {
    numerics::log_add_f64(a, b)
}
pub fn numerical_negative_exponential_f64(value: f64) -> f64 {
    numerics::negative_exponential_f64(value)
}
pub fn numerical_exponential_f64(value: f64) -> f64 {
    numerics::exponential_f64(value)
}
pub fn numerical_log_add_table_f64() -> NumericalIteratorF64 {
    NumericalIteratorF64 {
        exponential: false,
        front: 0,
        back: numerics::LOG_ADD_TABLE_SIZE,
    }
}
pub fn numerical_exponential_table_f64() -> NumericalIteratorF64 {
    NumericalIteratorF64 {
        exponential: true,
        front: 0,
        back: numerics::EXPONENTIAL_TABLE_SIZE,
    }
}
