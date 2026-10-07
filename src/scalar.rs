//! Storage precision supported by the original `FP_TYPE` build setting.
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, Mul, Sub};

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
    + AddAssign
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
    const ZERO: Self;
    const ONE: Self;

    fn from_f64(value: f64) -> Self;

    /// Decode binary32 storage without an intermediate conversion in float mode.
    fn from_f32(value: f32) -> Self;

    /// Encode legacy binary32 storage, narrowing only in double mode.
    fn to_f32(self) -> f32;

    /// Quantized upstream log sum at the selected table precision.
    fn log_add(self, other: Self) -> Self;

    /// Quantized upstream negative exponential at the selected precision.
    fn negative_exponential(self) -> Self;

    /// Upstream exponential, including its float-only positive approximation.
    fn exponential(self) -> Self;
}

impl Scalar for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;

    fn from_f64(value: f64) -> Self {
        value as Self
    }

    fn from_f32(value: f32) -> Self {
        value
    }
    fn to_f32(self) -> f32 {
        self
    }

    fn log_add(self, other: Self) -> Self {
        crate::numerics::log_add(self, other)
    }

    fn negative_exponential(self) -> Self {
        crate::numerics::negative_exponential(self)
    }

    fn exponential(self) -> Self {
        crate::numerics::exponential(self)
    }
}

impl Scalar for f64 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;

    fn from_f64(value: f64) -> Self {
        value
    }

    fn from_f32(value: f32) -> Self {
        Self::from(value)
    }
    fn to_f32(self) -> f32 {
        self as f32
    }

    fn log_add(self, other: Self) -> Self {
        crate::numerics::log_add_f64(self, other)
    }

    fn negative_exponential(self) -> Self {
        crate::numerics::negative_exponential_f64(self)
    }

    fn exponential(self) -> Self {
        crate::numerics::exponential_f64(self)
    }
}
