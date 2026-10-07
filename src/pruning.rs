//! State bounds matching the floating-point boundary-pruning loops in C.
use std::ops::RangeInclusive;

use crate::ModelError;
use crate::scalar::Scalar;

pub(crate) fn validate_boundary_radius<T: Scalar>(radius: T) -> Result<(), ModelError> {
    if !radius.into().is_finite() || radius < T::ZERO {
        return Err(ModelError(
            "state pruning radius must be finite and nonnegative",
        ));
    }
    Ok(())
}

/// C truncates the nonnegative lower bound when initializing its integer loop
/// index, and includes integer indices up to the floating upper bound. Fractional
/// radii therefore include an extra preceding state, rather than rounding both
/// sides to the same integer radius. Callers validate the radius first.
pub(crate) fn boundary_states<T: Scalar>(
    center: usize,
    radius: T,
    upper: usize,
) -> RangeInclusive<usize> {
    let first = (T::from_f64(center as f64) - radius).into().max(0.0) as usize;
    let last = (T::from_f64(center as f64) + radius)
        .into()
        .min(T::from_f64(upper as f64).into()) as usize;
    first..=last.min(upper)
}
