//! Storage precision supported by the original `FP_TYPE` build setting.
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};

mod sealed {
    pub trait Sealed {}
    impl Sealed for f32 {}
    impl Sealed for f64 {}
}

/// A supported model parameter precision: binary32 or binary64.
///
/// Conversion from binary64 rounds to the selected storage precision, matching
/// assignments to upstream `FP_TYPE`. It does not validate model parameters.
pub trait Scalar:
    sealed::Sealed
    + Copy
    + Default
    + Debug
    + PartialOrd
    + Into<f64>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
    const ZERO: Self;
    const ONE: Self;

    fn from_f64(value: f64) -> Self;
}

impl Scalar for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;

    fn from_f64(value: f64) -> Self {
        value as Self
    }
}

impl Scalar for f64 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;

    fn from_f64(value: f64) -> Self {
        value
    }
}
