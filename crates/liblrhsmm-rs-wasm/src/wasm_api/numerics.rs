use crate::numerics;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum NumericalTable {
    LogAdd = 0,
    Exponential = 1,
}
fn table(code: u32) -> Result<bool, JsValue> {
    match code {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(JsValue::from_str("invalid numerical coefficient table")),
    }
}

#[wasm_bindgen]
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
macro_rules! numerical_api {
    ($scalar:ty, $owner:ident, $iterator:ident, $log_table:ident, $exp_table:ident,
     $log:ident, $negative:ident, $exp:ident, $native_log:ident, $native_negative:ident, $native_exp:ident,
     $log_iter:ident, $exp_iter:ident) => {
        #[wasm_bindgen]
        pub struct $owner {
            exponential: bool,
        }
        #[wasm_bindgen]
        impl $owner {
            #[wasm_bindgen(constructor)]
            pub fn new(code: u32) -> Result<Self, JsValue> {
                Ok(Self {
                    exponential: table(code)?,
                })
            }
            #[wasm_bindgen(getter)]
            pub fn length(&self) -> usize {
                if self.exponential {
                    numerics::$exp_table.len()
                } else {
                    numerics::$log_table.len()
                }
            }
            pub fn is_empty(&self) -> bool {
                if self.exponential {
                    numerics::$exp_table.is_empty()
                } else {
                    numerics::$log_table.is_empty()
                }
            }
            pub fn get(&self, index: usize) -> Option<$scalar> {
                if self.exponential {
                    numerics::$exp_table.get(index)
                } else {
                    numerics::$log_table.get(index)
                }
            }
            pub fn replace(&self, index: usize, value: $scalar) -> Option<$scalar> {
                if self.exponential {
                    numerics::$exp_table.replace(index, value)
                } else {
                    numerics::$log_table.replace(index, value)
                }
            }
            pub fn get_storage(&self, index: usize) -> Vec<$scalar> {
                self.get(index).into_iter().collect()
            }
            pub fn replace_storage(
                &self,
                index: usize,
                values: &[$scalar],
            ) -> Result<Vec<$scalar>, JsValue> {
                let [value]: [$scalar; 1] = values
                    .try_into()
                    .map_err(|_| JsValue::from_str("coefficient replacement requires one value"))?;
                Ok(self.replace(index, value).into_iter().collect())
            }
            pub fn iter(&self) -> $iterator {
                $iterator {
                    exponential: self.exponential,
                    front: 0,
                    back: self.length(),
                }
            }
            pub fn values(&self) -> Vec<$scalar> {
                if self.exponential {
                    numerics::$exp_table.iter().collect()
                } else {
                    numerics::$log_table.iter().collect()
                }
            }
            /// Cloning retains a handle to the same module-wide native table.
            pub fn cloned(&self) -> Self {
                Self {
                    exponential: self.exponential,
                }
            }
        }
        #[wasm_bindgen]
        pub struct $iterator {
            exponential: bool,
            front: usize,
            back: usize,
        }
        impl Iterator for $iterator {
            type Item = $scalar;
            fn next(&mut self) -> Option<Self::Item> {
                if self.front == self.back {
                    return None;
                }
                let index = self.front;
                self.front += 1;
                if self.exponential {
                    numerics::$exp_table.get(index)
                } else {
                    numerics::$log_table.get(index)
                }
            }
            fn size_hint(&self) -> (usize, Option<usize>) {
                let length = self.back - self.front;
                (length, Some(length))
            }
        }
        impl DoubleEndedIterator for $iterator {
            fn next_back(&mut self) -> Option<Self::Item> {
                if self.front == self.back {
                    return None;
                }
                self.back -= 1;
                if self.exponential {
                    numerics::$exp_table.get(self.back)
                } else {
                    numerics::$log_table.get(self.back)
                }
            }
        }
        impl ExactSizeIterator for $iterator {}
        impl std::iter::FusedIterator for $iterator {}
        #[wasm_bindgen]
        impl $iterator {
            #[wasm_bindgen(getter)]
            pub fn length(&self) -> usize {
                self.back - self.front
            }
            #[wasm_bindgen(js_name = next)]
            pub fn next_value(&mut self) -> Option<$scalar> {
                Iterator::next(self)
            }
            #[wasm_bindgen(js_name = next_back)]
            pub fn next_back_value(&mut self) -> Option<$scalar> {
                DoubleEndedIterator::next_back(self)
            }
            pub fn next_storage(&mut self) -> Vec<$scalar> {
                self.next().into_iter().collect()
            }
            pub fn next_back_storage(&mut self) -> Vec<$scalar> {
                self.next_back().into_iter().collect()
            }
        }
        #[wasm_bindgen]
        pub fn $log(a: $scalar, b: $scalar) -> $scalar {
            numerics::$native_log(a, b)
        }
        #[wasm_bindgen]
        pub fn $negative(value: $scalar) -> $scalar {
            numerics::$native_negative(value)
        }
        #[wasm_bindgen]
        pub fn $exp(value: $scalar) -> $scalar {
            numerics::$native_exp(value)
        }
        #[wasm_bindgen]
        pub fn $log_iter() -> $iterator {
            $iterator {
                exponential: false,
                front: 0,
                back: numerics::LOG_ADD_TABLE_SIZE,
            }
        }
        #[wasm_bindgen]
        pub fn $exp_iter() -> $iterator {
            $iterator {
                exponential: true,
                front: 0,
                back: numerics::EXPONENTIAL_TABLE_SIZE,
            }
        }
    };
}
numerical_api!(
    f32,
    NumericalTableF32,
    NumericalIteratorF32,
    LOG_ADD_COEFFICIENTS,
    EXPONENTIAL_COEFFICIENTS,
    numerical_log_add_f32,
    numerical_negative_exponential_f32,
    numerical_exponential_f32,
    log_add,
    negative_exponential,
    exponential,
    numerical_log_add_table_f32,
    numerical_exponential_table_f32
);
numerical_api!(
    f64,
    NumericalTableF64,
    NumericalIteratorF64,
    LOG_ADD_COEFFICIENTS_F64,
    EXPONENTIAL_COEFFICIENTS_F64,
    numerical_log_add_f64,
    numerical_negative_exponential_f64,
    numerical_exponential_f64,
    log_add_f64,
    negative_exponential_f64,
    exponential_f64,
    numerical_log_add_table_f64,
    numerical_exponential_table_f64
);
