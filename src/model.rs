//! Owned model parameters. Reference: upstream `model.h` and `model.c`.
use std::fmt;

/// Invalid dimensions or parameter storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelError(pub &'static str);

impl fmt::Display for ModelError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0)
    }
}
impl std::error::Error for ModelError {}

/// A normal duration distribution with optional frame constraints.
/// Negative constraints retain the upstream unset convention.
#[derive(Debug, Clone, PartialEq)]
pub struct Duration {
    pub mean: f32,
    pub variance: f32,
    pub minimum: i32,
    pub maximum: i32,
    pub fixed_mean: i32,
    pub variance_floor: f32,
}

impl Default for Duration {
    fn default() -> Self {
        Self {
            mean: 0.0,
            variance: 0.0,
            minimum: -1,
            maximum: -1,
            fixed_mean: -1,
            variance_floor: 0.0,
        }
    }
}

/// Diagonal Gaussian mixtures in mixture-major order.
/// Each parameter vector contains `weights.len() * dimensions` entries.
#[derive(Debug, Clone, PartialEq)]
pub struct GaussianMixture {
    pub dimensions: usize,
    pub weights: Vec<f32>,
    pub means: Vec<f32>,
    pub variances: Vec<f32>,
    pub variance_floors: Vec<f32>,
}

impl GaussianMixture {
    /// Create uninitialized parameters with uniform mixture weights.
    pub fn new(mixtures: usize, dimensions: usize) -> Result<Self, ModelError> {
        if mixtures == 0 || dimensions == 0 {
            return Err(ModelError("mixture count and dimensions must be positive"));
        }
        if mixtures > i32::MAX as usize || dimensions > i32::MAX as usize {
            return Err(ModelError("dimensions exceed the legacy format"));
        }
        let length = mixtures
            .checked_mul(dimensions)
            .ok_or(ModelError("dimensions overflow"))?;
        if length
            .checked_mul(size_of::<f32>())
            .is_none_or(|bytes| bytes > isize::MAX as usize)
        {
            return Err(ModelError(
                "parameter allocation exceeds addressable memory",
            ));
        }
        Ok(Self {
            dimensions,
            weights: vec![1.0 / mixtures as f32; mixtures],
            means: vec![0.0; length],
            variances: vec![0.0; length],
            variance_floors: vec![0.0; length],
        })
    }

    /// Check structural consistency without rejecting flat-start parameters.
    pub fn validate(&self) -> Result<(), ModelError> {
        let mixtures = self.weights.len();
        if mixtures == 0 || self.dimensions == 0 {
            return Err(ModelError("mixture count and dimensions must be positive"));
        }
        if mixtures > i32::MAX as usize || self.dimensions > i32::MAX as usize {
            return Err(ModelError("dimensions exceed the legacy format"));
        }
        let length = mixtures
            .checked_mul(self.dimensions)
            .ok_or(ModelError("dimensions overflow"))?;
        if self.means.len() != length
            || self.variances.len() != length
            || self.variance_floors.len() != length
        {
            return Err(ModelError("parameter lengths do not match dimensions"));
        }
        Ok(())
    }
}

/// A weighted observation stream and its emission distributions.
#[derive(Debug, Clone, PartialEq)]
pub struct Stream {
    pub weight: f32,
    pub mixtures: Vec<GaussianMixture>,
}

/// Independent observation streams and state-duration distributions.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Model {
    pub streams: Vec<Stream>,
    pub durations: Vec<Duration>,
}

impl Model {
    /// Validate structural consistency, allowing uninitialized variances.
    pub fn validate(&self) -> Result<(), ModelError> {
        for stream in &self.streams {
            for mixture in &stream.mixtures {
                mixture.validate()?;
            }
        }
        Ok(())
    }
}
