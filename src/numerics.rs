//! Original quantized probability helpers from upstream `math-funcs.h`.
//! Tables preserve published decimal coefficients at each build precision.
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

fn coefficients(table: &'static [u8]) -> impl ExactSizeIterator<Item = f32> + DoubleEndedIterator {
    table
        .chunks_exact(4)
        .map(|bytes| f32::from_le_bytes(bytes.try_into().expect("fixed binary32 coefficient")))
}

fn coefficients_f64(
    table: &'static [u8],
) -> impl ExactSizeIterator<Item = f64> + DoubleEndedIterator {
    table
        .chunks_exact(8)
        .map(|bytes| f64::from_le_bytes(bytes.try_into().expect("fixed binary64 coefficient")))
}

/// Read the original binary32 log-sum coefficients without allocating.
pub fn log_add_table() -> impl ExactSizeIterator<Item = f32> + DoubleEndedIterator {
    coefficients(LOG_ADD)
}

/// Read the original binary32 exponential coefficients without allocating.
pub fn exponential_table() -> impl ExactSizeIterator<Item = f32> + DoubleEndedIterator {
    coefficients(NEGATIVE_EXP)
}

/// Read the original binary64 log-sum coefficients without narrowing them.
pub fn log_add_table_f64() -> impl ExactSizeIterator<Item = f64> + DoubleEndedIterator {
    coefficients_f64(LOG_ADD_F64)
}

/// Read the original binary64 exponential coefficients without narrowing them.
pub fn exponential_table_f64() -> impl ExactSizeIterator<Item = f64> + DoubleEndedIterator {
    coefficients_f64(NEGATIVE_EXP_F64)
}

/// Finite sentinel used by the original inference algorithms.
pub const LOG_ZERO: f32 = -1e15;
/// Binary64 version of the original finite inference sentinel.
pub const LOG_ZERO_F64: f64 = -1e15;

fn lookup(table: &[u8; 28_000], index: usize) -> f32 {
    let start = index * 4;
    f32::from_le_bytes(
        table[start..start + 4]
            .try_into()
            .expect("fixed table entry"),
    )
}

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
    lookup(
        LOG_ADD,
        (f64::from(difference) / LOG_ADD_TABLE_STEP) as usize,
    ) + maximum
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
    lookup(
        NEGATIVE_EXP,
        (f64::from(-value) / EXPONENTIAL_TABLE_STEP) as usize,
    )
}

/// Original table exponential for negative input and fastapprox otherwise.
pub fn exponential(value: f32) -> f32 {
    if value >= 0.0 {
        crate::probability::compatibility::exp(value)
    } else {
        negative_exponential(value)
    }
}

fn lookup_f64(table: &[u8; 56_000], index: usize) -> f64 {
    let start = index * 8;
    f64::from_le_bytes(
        table[start..start + 8]
            .try_into()
            .expect("fixed table entry"),
    )
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
    lookup_f64(LOG_ADD_F64, (difference / LOG_ADD_TABLE_STEP) as usize) + maximum
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
    lookup_f64(NEGATIVE_EXP_F64, (-value / EXPONENTIAL_TABLE_STEP) as usize)
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
