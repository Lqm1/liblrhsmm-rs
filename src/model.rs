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
            weights: vec![(1.0 / mixtures as f64) as f32; mixtures],
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

impl Default for Stream {
    fn default() -> Self {
        Self {
            weight: 1.0,
            mixtures: Vec::new(),
        }
    }
}

impl Stream {
    /// Construct equally sized flat-start emissions with uniform mixture weights.
    pub fn new(emissions: usize, components: usize, dimensions: usize) -> Result<Self, ModelError> {
        if emissions > i32::MAX as usize {
            return Err(ModelError("emission count exceeds the legacy format"));
        }
        let mut mixtures = Vec::new();
        mixtures
            .try_reserve_exact(emissions)
            .map_err(|_| ModelError("stream allocation failed"))?;
        for _ in 0..emissions {
            mixtures.push(GaussianMixture::new(components, dimensions)?);
        }
        Ok(Self {
            weight: 1.0,
            mixtures,
        })
    }
    /// Check structural consistency, allowing empty and flat-start streams.
    pub fn validate(&self) -> Result<(), ModelError> {
        for mixture in &self.mixtures {
            mixture.validate()?;
        }
        Ok(())
    }
}

/// Independent observation streams and state-duration distributions.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Model {
    pub streams: Vec<Stream>,
    pub durations: Vec<Duration>,
}

impl Model {
    /// Create a flat-start model with one layout per observation stream.
    pub fn new(layouts: &[StreamLayout], duration_count: usize) -> Result<Self, ModelError> {
        let mut builder = ModelBuilder::new(layouts.len(), duration_count)?;
        for (slot, layout) in builder.streams.iter_mut().zip(layouts) {
            *slot = Some(Stream::new(
                layout.emissions,
                layout.components,
                layout.dimensions,
            )?);
        }
        builder.durations.fill(Some(Duration::default()));
        builder.build()
    }
    /// Validate structural consistency, allowing uninitialized variances.
    pub fn validate(&self) -> Result<(), ModelError> {
        for stream in &self.streams {
            stream.validate()?;
        }
        Ok(())
    }
}

/// Dimensions of equally sized GMMs in one flat-start stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StreamLayout {
    pub emissions: usize,
    pub components: usize,
    pub dimensions: usize,
}

/// Partially populated stream, corresponding to `lrh_create_empty_stream`.
/// Fill every emission slot before calling `build`.
#[derive(Debug, Clone, PartialEq)]
pub struct StreamBuilder {
    pub weight: f32,
    pub emissions: Vec<Option<GaussianMixture>>,
}
impl StreamBuilder {
    pub fn new(emissions: usize) -> Result<Self, ModelError> {
        Ok(Self {
            weight: 1.0,
            emissions: empty_slots(emissions)?,
        })
    }
    pub fn build(self) -> Result<Stream, ModelError> {
        let mixtures = self
            .emissions
            .into_iter()
            .map(|slot| {
                slot.ok_or(ModelError(
                    "stream builder contains an unfilled emission slot",
                ))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let stream = Stream {
            weight: self.weight,
            mixtures,
        };
        stream.validate()?;
        Ok(stream)
    }
}

/// Partially populated model, corresponding to `lrh_create_empty_model`.
/// Optional slots permit safe incremental construction without null pointers.
#[derive(Debug, Clone, PartialEq)]
pub struct ModelBuilder {
    pub streams: Vec<Option<Stream>>,
    pub durations: Vec<Option<Duration>>,
}
impl ModelBuilder {
    pub fn new(streams: usize, durations: usize) -> Result<Self, ModelError> {
        Ok(Self {
            streams: empty_slots(streams)?,
            durations: empty_slots(durations)?,
        })
    }
    pub fn build(self) -> Result<Model, ModelError> {
        let streams = self
            .streams
            .into_iter()
            .map(|slot| slot.ok_or(ModelError("model builder contains an unfilled stream slot")))
            .collect::<Result<Vec<_>, _>>()?;
        let durations = self
            .durations
            .into_iter()
            .map(|slot| {
                slot.ok_or(ModelError(
                    "model builder contains an unfilled duration slot",
                ))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let model = Model { streams, durations };
        model.validate()?;
        Ok(model)
    }
}

fn empty_slots<T>(count: usize) -> Result<Vec<Option<T>>, ModelError> {
    if count > i32::MAX as usize {
        return Err(ModelError("slot count exceeds the legacy format"));
    }
    let mut slots = Vec::new();
    slots
        .try_reserve_exact(count)
        .map_err(|_| ModelError("builder allocation failed"))?;
    slots.resize_with(count, || None);
    Ok(slots)
}
