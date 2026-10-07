//! Original quantized probability helpers from upstream `math-funcs.h`.
//! Tables preserve published decimal coefficients at each build precision.
//!
//! Coefficient changes affect subsequent helper, inference and training calls
//! process-wide. Each coefficient is atomic; a table iteration or an inference
//! call is not a snapshot of concurrent changes. Configure tables before starting
//! a calculation when reproducible results are required.
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
/// Number of coefficients in the original log-sum lookup table.
pub const LOG_ADD_TABLE_SIZE: usize = 7000;
/// Binary64 quantization step used before indexing the log-sum table.
pub const LOG_ADD_TABLE_STEP: f64 = 0.001;
/// Number of coefficients in the original negative-exponential table.
pub const EXPONENTIAL_TABLE_SIZE: usize = 7000;
/// Binary64 quantization step used before indexing the exponential table.
pub const EXPONENTIAL_TABLE_STEP: f64 = 0.002;
/// Original unsuffixed C epsilon for occupancy normalization and updates.
pub const OCCUPANCY_EPSILON: f64 = 1e-7;

const LOG_ADD: &[u8; LOG_ADD_TABLE_SIZE * 4] = include_bytes!("lse-table.bin");
const NEGATIVE_EXP: &[u8; EXPONENTIAL_TABLE_SIZE * 4] = include_bytes!("exp-table.bin");
const LOG_ADD_F64: &[u8; LOG_ADD_TABLE_SIZE * 8] = include_bytes!("lse-table-f64.bin");
const NEGATIVE_EXP_F64: &[u8; EXPONENTIAL_TABLE_SIZE * 8] = include_bytes!("exp-table-f64.bin");

macro_rules! coefficient_table {
    ($name:ident, $atomic:ident, $word:ty, $scalar:ty, $width:expr) => {
        /// Mutable numerical coefficients with atomic, bit-preserving access.
        /// Changes are independent per entry, without a whole-table snapshot.
        #[derive(Debug)]
        pub struct $name<const N: usize> {
            values: [$atomic; N],
        }

        impl<const N: usize> $name<N> {
            const fn from_bytes(bytes: &[u8]) -> Self {
                assert!(bytes.len() == N * $width);
                let mut values = [const { $atomic::new(0) }; N];
                let mut index = 0;
                while index < N {
                    let mut entry = [0; $width];
                    let mut byte = 0;
                    while byte < $width {
                        entry[byte] = bytes[index * $width + byte];
                        byte += 1;
                    }
                    values[index] = $atomic::new(<$word>::from_le_bytes(entry));
                    index += 1;
                }
                Self { values }
            }

            /// Number of addressable coefficients.
            pub const fn len(&self) -> usize {
                N
            }

            pub const fn is_empty(&self) -> bool {
                N == 0
            }

            /// Read one coefficient, or return `None` for an invalid index.
            pub fn get(&self, index: usize) -> Option<$scalar> {
                self.values
                    .get(index)
                    .map(|value| <$scalar>::from_bits(value.load(Ordering::Relaxed)))
            }

            /// Replace one coefficient and return its previous value.
            /// Any scalar bits are accepted, as with the original C array.
            /// Invalid indices leave the table unchanged and return `None`.
            pub fn replace(&self, index: usize, value: $scalar) -> Option<$scalar> {
                self.values.get(index).map(|entry| {
                    <$scalar>::from_bits(entry.swap(value.to_bits(), Ordering::Relaxed))
                })
            }

            /// Iterate current values without allocating or locking a table.
            pub fn iter(
                &self,
            ) -> impl ExactSizeIterator<Item = $scalar> + DoubleEndedIterator + '_ {
                self.values
                    .iter()
                    .map(|entry| <$scalar>::from_bits(entry.load(Ordering::Relaxed)))
            }

            fn coefficient(&self, index: usize) -> $scalar {
                <$scalar>::from_bits(self.values[index].load(Ordering::Relaxed))
            }
        }
    };
}
coefficient_table!(CoefficientTable32, AtomicU32, u32, f32, 4);
coefficient_table!(CoefficientTable64, AtomicU64, u64, f64, 8);

/// Process-wide binary32 log-sum table, initialized from original coefficients.
pub static LOG_ADD_COEFFICIENTS: CoefficientTable32<LOG_ADD_TABLE_SIZE> =
    CoefficientTable32::from_bytes(LOG_ADD);
/// Process-wide binary32 exponential table, initialized from original coefficients.
pub static EXPONENTIAL_COEFFICIENTS: CoefficientTable32<EXPONENTIAL_TABLE_SIZE> =
    CoefficientTable32::from_bytes(NEGATIVE_EXP);
/// Process-wide binary64 log-sum table, independent of the binary32 table.
pub static LOG_ADD_COEFFICIENTS_F64: CoefficientTable64<LOG_ADD_TABLE_SIZE> =
    CoefficientTable64::from_bytes(LOG_ADD_F64);
/// Process-wide binary64 exponential table, independent of the binary32 table.
pub static EXPONENTIAL_COEFFICIENTS_F64: CoefficientTable64<EXPONENTIAL_TABLE_SIZE> =
    CoefficientTable64::from_bytes(NEGATIVE_EXP_F64);

/// Read the current binary32 log-sum coefficients without allocating.
pub fn log_add_table() -> impl ExactSizeIterator<Item = f32> + DoubleEndedIterator {
    LOG_ADD_COEFFICIENTS.iter()
}

/// Read the current binary32 exponential coefficients without allocating.
pub fn exponential_table() -> impl ExactSizeIterator<Item = f32> + DoubleEndedIterator {
    EXPONENTIAL_COEFFICIENTS.iter()
}

/// Read the current binary64 log-sum coefficients without narrowing them.
pub fn log_add_table_f64() -> impl ExactSizeIterator<Item = f64> + DoubleEndedIterator {
    LOG_ADD_COEFFICIENTS_F64.iter()
}

/// Read the current binary64 exponential coefficients without narrowing them.
pub fn exponential_table_f64() -> impl ExactSizeIterator<Item = f64> + DoubleEndedIterator {
    EXPONENTIAL_COEFFICIENTS_F64.iter()
}

/// Finite sentinel used by the original inference algorithms.
pub const LOG_ZERO: f32 = -1e15;
/// Binary64 version of the original finite inference sentinel.
pub const LOG_ZERO_F64: f64 = -1e15;

/// Approximate log(exp(a) + exp(b)), with the original quantization and cutoff.
pub fn log_add(a: f32, b: f32) -> f32 {
    if a.is_nan() || b.is_nan() {
        return f32::NAN;
    }
    let maximum = a.max(b);
    let difference = maximum - a.min(b);
    if !difference.is_finite() || f64::from(difference) > 6.91 {
        return maximum;
    }
    LOG_ADD_COEFFICIENTS.coefficient((f64::from(difference) / LOG_ADD_TABLE_STEP) as usize)
        + maximum
}

/// Quantized exp for log probabilities, saturated at one for nonnegative input.
pub fn negative_exponential(value: f32) -> f32 {
    if value.is_nan() {
        return f32::NAN;
    }
    if f64::from(value) < -13.9 {
        return 0.0;
    }
    if value >= 0.0 {
        return 1.0;
    }
    EXPONENTIAL_COEFFICIENTS.coefficient((f64::from(-value) / EXPONENTIAL_TABLE_STEP) as usize)
}

/// Original table exponential for negative input and fastapprox otherwise.
pub fn exponential(value: f32) -> f32 {
    if value >= 0.0 {
        crate::probability::compatibility::exp(value)
    } else {
        negative_exponential(value)
    }
}

/// Binary64-build log sum, using the original decimal table rounded to binary64.
pub fn log_add_f64(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        return f64::NAN;
    }
    let maximum = a.max(b);
    let difference = maximum - a.min(b);
    if !difference.is_finite() || difference > 6.91 {
        return maximum;
    }
    LOG_ADD_COEFFICIENTS_F64.coefficient((difference / LOG_ADD_TABLE_STEP) as usize) + maximum
}

/// Binary64-build quantized negative exponential, with the source cutoffs.
pub fn negative_exponential_f64(value: f64) -> f64 {
    if value.is_nan() {
        return f64::NAN;
    }
    if value < -13.9 {
        return 0.0;
    }
    if value >= 0.0 {
        return 1.0;
    }
    EXPONENTIAL_COEFFICIENTS_F64.coefficient((-value / EXPONENTIAL_TABLE_STEP) as usize)
}

/// Binary64-build exponential. The source positive fastapprox branch still
/// narrows its argument and result to binary32, even when FP_TYPE is double.
pub fn exponential_f64(value: f64) -> f64 {
    if value >= 0.0 {
        f64::from(crate::probability::compatibility::exp(value as f32))
    } else {
        negative_exponential_f64(value)
    }
}
