//! State bounds matching the floating-point boundary-pruning loops in C.
use std::ops::RangeInclusive;

use crate::ModelError;

pub(crate) fn validate_boundary_radius(radius: f32) -> Result<(), ModelError> {
    if !radius.is_finite() || radius < 0.0 {
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
pub(crate) fn boundary_states(center: usize, radius: f32, upper: usize) -> RangeInclusive<usize> {
    let first = (center as f32 - radius).max(0.0) as usize;
    let last = (center as f32 + radius).min(upper as f32) as usize;
    first..=last.min(upper)
}
