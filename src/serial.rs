//! Original model MessagePack schema. Reference: upstream `serial.c`.
//! Floating-point fields are binary32. Array schemas are explicit, not inferred.
use crate::data::{JumpParameters, ObservationData, ObservationStreamData, SegmentationData};
use crate::model::{
    DurationParameters, GaussianMixtureParameters, ModelParameters, StreamParameters,
};
use crate::scalar::Scalar;
use std::io::{self, Read, Write};

/// Resource bounds for model input.
#[derive(Debug, Clone, Copy)]
pub struct DecodeLimits {
    /// Maximum cumulative number of declared array entries.
    pub max_array_entries: usize,
}
impl Default for DecodeLimits {
    fn default() -> Self {
        Self {
            max_array_entries: 16 * 1024 * 1024,
        }
    }
}

/// Explicit output schema for interoperability with historical C readers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelEncoding {
    /// Schema introduced by upstream commit cd118de in September 2017.
    WithVarianceFloors,
    /// Earlier schema used by SHIRO's bundled pretrained model.
    WithoutVarianceFloors,
}

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}
fn codec(error: impl std::fmt::Display) -> io::Error {
    invalid(error.to_string())
}

struct Decoder<R> {
    reader: R,
    remaining_entries: usize,
}
impl<R: Read> Decoder<R> {
    fn end(&mut self) -> io::Result<()> {
        let mut trailing = [0];
        loop {
            match self.reader.read(&mut trailing) {
                Ok(0) => return Ok(()),
                Ok(_) => return Err(invalid("trailing data after object")),
                Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
                Err(error) => return Err(error),
            }
        }
    }

    fn nonnegative_integer(&mut self) -> io::Result<usize> {
        usize::try_from(self.integer()?).map_err(|_| invalid("negative array dimension"))
    }

    fn integers(&mut self, expected: usize) -> io::Result<Vec<i32>> {
        self.expect_array(expected)?;
        (0..expected).map(|_| self.integer()).collect()
    }

    fn observation<T: Scalar>(&mut self) -> io::Result<ObservationData<T>> {
        self.expect_array(3)?;
        let frames = self.nonnegative_integer()?;
        let count = self.array()?;
        let mut dimensions = Vec::new();
        for _ in 0..count {
            dimensions.push(self.dimension()?);
        }
        self.expect_array(count)?;
        let mut streams = Vec::new();
        for dimensions in dimensions {
            let length = frames
                .checked_mul(dimensions)
                .ok_or_else(|| invalid("observation dimensions overflow"))?;
            self.expect_array(length)?;
            let mut values = Vec::new();
            for _ in 0..length {
                values.push(self.float()?);
            }
            streams.push(ObservationStreamData { dimensions, values });
        }
        let observation = ObservationData { frames, streams };
        observation.validate().map_err(codec)?;
        Ok(observation)
    }

    fn segmentation<T: Scalar>(&mut self) -> io::Result<SegmentationData<T>> {
        // Upstream declares arity 3 but writes four children. Deliberately read
        // its schema rather than treating the fourth child as another object.
        self.expect_array(3)?;
        let segments = self.array()?;
        let mut boundaries = Vec::new();
        for _ in 0..segments {
            boundaries.push(self.integer()?);
        }
        let duration_states = self.integers(segments)?;
        let streams = self.array()?;
        let mut output_states = Vec::new();
        for _ in 0..streams {
            output_states.push(self.integers(segments)?);
        }
        self.expect_array(segments)?;
        let mut outgoing = Vec::new();
        for _ in 0..segments {
            // A second legacy mismatch: n jumps have 2*n flat scalar values.
            let count = self.array()?;
            self.remaining_entries = self
                .remaining_entries
                .checked_sub(count)
                .ok_or_else(|| invalid("jump data exceeds the decoding limit"))?;
            let mut jumps = Vec::new();
            for _ in 0..count {
                jumps.push(JumpParameters {
                    delta: self.integer()?,
                    probability: self.float()?,
                });
            }
            outgoing.push(jumps);
        }
        let segmentation = SegmentationData {
            boundaries,
            duration_states,
            output_states,
            outgoing,
        };
        segmentation.validate().map_err(codec)?;
        Ok(segmentation)
    }
    fn array(&mut self) -> io::Result<usize> {
        let length = rmp::decode::read_array_len(&mut self.reader).map_err(codec)? as usize;
        self.remaining_entries = self
            .remaining_entries
            .checked_sub(length)
            .ok_or_else(|| invalid("model exceeds the array-entry decoding limit"))?;
        Ok(length)
    }
    fn expect_array(&mut self, expected: usize) -> io::Result<()> {
        let actual = self.array()?;
        if actual != expected {
            return Err(invalid(format!(
                "expected array length {expected}, found {actual}"
            )));
        }
        Ok(())
    }
    fn integer(&mut self) -> io::Result<i32> {
        rmp::decode::read_int(&mut self.reader).map_err(codec)
    }
    fn dimension(&mut self) -> io::Result<usize> {
        let dimension = self.integer()?;
        if dimension <= 0 {
            return Err(invalid("mixture count and dimensions must be positive"));
        }
        Ok(dimension as usize)
    }
    fn float<T: Scalar>(&mut self) -> io::Result<T> {
        rmp::decode::read_f32(&mut self.reader)
            .map(T::from_f32)
            .map_err(codec)
    }
    fn duration<T: Scalar>(&mut self) -> io::Result<DurationParameters<T>> {
        self.expect_array(6)?;
        Ok(DurationParameters {
            mean: self.float()?,
            variance: self.float()?,
            minimum: self.integer()?,
            maximum: self.integer()?,
            fixed_mean: self.integer()?,
            variance_floor: self.float()?,
        })
    }
    fn mixture<T: Scalar>(&mut self) -> io::Result<GaussianMixtureParameters<T>> {
        self.expect_array(4)?;
        let mixtures = self.dimension()?;
        let dimensions = self.dimension()?;
        self.expect_array(mixtures)?;
        // Grow only after receiving values, never from an untrusted size alone.
        let mut weights = Vec::new();
        for _ in 0..mixtures {
            weights.push(self.float()?);
        }
        let parameters = mixtures
            .checked_mul(dimensions)
            .ok_or_else(|| invalid("dimensions overflow"))?;
        let length = self.array()?;
        let legacy_length = parameters
            .checked_mul(2)
            .ok_or_else(|| invalid("parameter length overflows"))?;
        let current_length = parameters
            .checked_mul(3)
            .ok_or_else(|| invalid("parameter length overflows"))?;
        let has_floors = if length == current_length {
            true
        } else if length == legacy_length {
            false
        } else {
            return Err(invalid(
                "parameter array matches neither historical nor current schema",
            ));
        };
        let mut means = Vec::new();
        let mut variances = Vec::new();
        let mut variance_floors = Vec::new();
        for _ in 0..parameters {
            means.push(self.float()?);
            variances.push(self.float()?);
            variance_floors.push(if has_floors { self.float()? } else { T::ZERO });
        }
        Ok(GaussianMixtureParameters {
            dimensions,
            weights,
            means,
            variances,
            variance_floors,
        })
    }
    fn stream<T: Scalar>(&mut self) -> io::Result<StreamParameters<T>> {
        self.expect_array(2)?;
        let weight = self.float()?;
        let count = self.array()?;
        let mut mixtures = Vec::new();
        for _ in 0..count {
            mixtures.push(self.mixture()?);
        }
        Ok(StreamParameters { weight, mixtures })
    }
    fn model<T: Scalar>(&mut self) -> io::Result<ModelParameters<T>> {
        self.expect_array(2)?;
        let count = self.array()?;
        let mut streams = Vec::new();
        for _ in 0..count {
            streams.push(self.stream()?);
        }
        let count = self.array()?;
        let mut durations = Vec::new();
        for _ in 0..count {
            durations.push(self.duration()?);
        }
        Ok(ModelParameters { streams, durations })
    }
}

fn array<W: Write>(writer: &mut W, length: usize) -> io::Result<()> {
    let length = u32::try_from(length).map_err(|_| invalid("array exceeds MessagePack limits"))?;
    rmp::encode::write_array_len(writer, length).map_err(write_error)?;
    Ok(())
}
fn integer<W: Write>(writer: &mut W, value: i32) -> io::Result<()> {
    rmp::encode::write_sint(writer, i64::from(value)).map_err(write_error)?;
    Ok(())
}
fn float<T: Scalar, W: Write>(writer: &mut W, value: T) -> io::Result<()> {
    rmp::encode::write_f32(writer, value.to_f32()).map_err(write_error)
}

fn write_error(error: rmp::encode::ValueWriteError<io::Error>) -> io::Error {
    match error {
        rmp::encode::ValueWriteError::InvalidMarkerWrite(error)
        | rmp::encode::ValueWriteError::InvalidDataWrite(error) => error,
    }
}

fn write_duration<T: Scalar>(
    writer: &mut impl Write,
    duration: &DurationParameters<T>,
) -> io::Result<()> {
    array(writer, 6)?;
    float(writer, duration.mean)?;
    float(writer, duration.variance)?;
    integer(writer, duration.minimum)?;
    integer(writer, duration.maximum)?;
    integer(writer, duration.fixed_mean)?;
    float(writer, duration.variance_floor)
}

fn validate_encoding<'a, T: Scalar + 'a>(
    mixtures: impl Iterator<Item = &'a GaussianMixtureParameters<T>>,
    encoding: ModelEncoding,
) -> io::Result<()> {
    if encoding == ModelEncoding::WithoutVarianceFloors
        && mixtures
            .flat_map(|g| &g.variance_floors)
            .any(|&floor| floor != T::ZERO)
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "historical schema cannot represent nonzero variance floors",
        ));
    }
    Ok(())
}

fn write_mixture<T: Scalar>(
    writer: &mut impl Write,
    mixture: &GaussianMixtureParameters<T>,
    encoding: ModelEncoding,
) -> io::Result<()> {
    let has_floors = encoding == ModelEncoding::WithVarianceFloors;
    array(writer, 4)?;
    integer(writer, mixture.weights.len() as i32)?;
    integer(writer, mixture.dimensions as i32)?;
    array(writer, mixture.weights.len())?;
    for &weight in &mixture.weights {
        float(writer, weight)?;
    }
    array(
        writer,
        mixture
            .means
            .len()
            .checked_mul(if has_floors { 3 } else { 2 })
            .ok_or_else(|| invalid("parameter length overflows"))?,
    )?;
    for index in 0..mixture.means.len() {
        float(writer, mixture.means[index])?;
        float(writer, mixture.variances[index])?;
        if has_floors {
            float(writer, mixture.variance_floors[index])?;
        }
    }
    Ok(())
}

fn write_stream<T: Scalar>(
    writer: &mut impl Write,
    stream: &StreamParameters<T>,
    encoding: ModelEncoding,
) -> io::Result<()> {
    array(writer, 2)?;
    float(writer, stream.weight)?;
    array(writer, stream.mixtures.len())?;
    for mixture in &stream.mixtures {
        write_mixture(writer, mixture, encoding)?;
    }
    Ok(())
}

impl<T: Scalar> DurationParameters<T> {
    /// Read one standalone legacy duration distribution, rejecting trailing data.
    pub fn read_from(reader: impl Read) -> io::Result<Self> {
        Self::read_with_limits(reader, DecodeLimits::default())
    }
    pub fn read_with_limits(reader: impl Read, limits: DecodeLimits) -> io::Result<Self> {
        let mut decoder = Decoder {
            reader,
            remaining_entries: limits.max_array_entries,
        };
        let duration = decoder.duration()?;
        decoder.end()?;
        Ok(duration)
    }
    /// Read exactly one distribution, preserving subsequent objects in the reader.
    /// Pass a mutable reader reference to retain its position across calls.
    pub fn read_prefix(reader: impl Read) -> io::Result<Self> {
        Self::read_prefix_with_limits(reader, DecodeLimits::default())
    }
    pub fn read_prefix_with_limits(reader: impl Read, limits: DecodeLimits) -> io::Result<Self> {
        let mut decoder = Decoder {
            reader,
            remaining_entries: limits.max_array_entries,
        };
        decoder.duration()
    }
    pub fn write_to(&self, mut writer: impl Write) -> io::Result<()> {
        write_duration(&mut writer, self)
    }
}

impl<T: Scalar> GaussianMixtureParameters<T> {
    /// Read a standalone GMM in the historical or current parameter layout.
    pub fn read_from(reader: impl Read) -> io::Result<Self> {
        Self::read_with_limits(reader, DecodeLimits::default())
    }
    pub fn read_with_limits(reader: impl Read, limits: DecodeLimits) -> io::Result<Self> {
        let mut decoder = Decoder {
            reader,
            remaining_entries: limits.max_array_entries,
        };
        let mixture = decoder.mixture()?;
        decoder.end()?;
        mixture.validate().map_err(codec)?;
        Ok(mixture)
    }
    pub fn write_to(&self, writer: impl Write) -> io::Result<()> {
        self.write_with_encoding(writer, ModelEncoding::WithVarianceFloors)
    }
    pub fn write_with_encoding(
        &self,
        mut writer: impl Write,
        encoding: ModelEncoding,
    ) -> io::Result<()> {
        self.validate()
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;
        validate_encoding(std::iter::once(self), encoding)?;
        write_mixture(&mut writer, self, encoding)
    }
}

impl<T: Scalar> StreamParameters<T> {
    /// Read a standalone weighted stream, rejecting trailing data.
    pub fn read_from(reader: impl Read) -> io::Result<Self> {
        Self::read_with_limits(reader, DecodeLimits::default())
    }
    pub fn read_with_limits(reader: impl Read, limits: DecodeLimits) -> io::Result<Self> {
        let mut decoder = Decoder {
            reader,
            remaining_entries: limits.max_array_entries,
        };
        let stream = decoder.stream()?;
        decoder.end()?;
        stream.validate().map_err(codec)?;
        Ok(stream)
    }
    pub fn write_to(&self, writer: impl Write) -> io::Result<()> {
        self.write_with_encoding(writer, ModelEncoding::WithVarianceFloors)
    }
    pub fn write_with_encoding(
        &self,
        mut writer: impl Write,
        encoding: ModelEncoding,
    ) -> io::Result<()> {
        self.validate()
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;
        validate_encoding(self.mixtures.iter(), encoding)?;
        write_stream(&mut writer, self, encoding)
    }
}

impl<T: Scalar> ModelParameters<T> {
    /// Read one complete model and reject trailing bytes.
    pub fn read_from(reader: impl Read) -> io::Result<Self> {
        Self::read_with_limits(reader, DecodeLimits::default())
    }
    /// Read with a caller-selected resource bound.
    pub fn read_with_limits(reader: impl Read, limits: DecodeLimits) -> io::Result<Self> {
        let mut decoder = Decoder {
            reader,
            remaining_entries: limits.max_array_entries,
        };
        let model = decoder.model()?;
        decoder.end()?;
        model.validate().map_err(codec)?;
        Ok(model)
    }
    /// Write the original C-readable model layout.
    pub fn write_to(&self, mut writer: impl Write) -> io::Result<()> {
        self.write_with_encoding(&mut writer, ModelEncoding::WithVarianceFloors)
    }

    /// Write a selected schema. Historical output rejects nonzero variance floors
    /// rather than silently discarding model parameters.
    pub fn write_with_encoding(
        &self,
        mut writer: impl Write,
        encoding: ModelEncoding,
    ) -> io::Result<()> {
        self.validate()
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;
        validate_encoding(self.streams.iter().flat_map(|s| &s.mixtures), encoding)?;
        array(&mut writer, 2)?;
        array(&mut writer, self.streams.len())?;
        for stream in &self.streams {
            write_stream(&mut writer, stream, encoding)?;
        }
        array(&mut writer, self.durations.len())?;
        for duration in &self.durations {
            write_duration(&mut writer, duration)?;
        }
        Ok(())
    }
}

fn integers(writer: &mut impl Write, values: &[i32]) -> io::Result<()> {
    array(writer, values.len())?;
    for &value in values {
        integer(writer, value)?;
    }
    Ok(())
}

impl<T: Scalar> ObservationData<T> {
    pub fn read_from(reader: impl Read) -> io::Result<Self> {
        let mut decoder = Decoder {
            reader,
            remaining_entries: DecodeLimits::default().max_array_entries,
        };
        let observation = decoder.observation()?;
        decoder.end()?;
        Ok(observation)
    }

    pub fn write_to(&self, mut writer: impl Write) -> io::Result<()> {
        self.validate()
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;
        array(&mut writer, 3)?;
        integer(&mut writer, self.frames as i32)?;
        array(&mut writer, self.streams.len())?;
        for stream in &self.streams {
            integer(&mut writer, stream.dimensions as i32)?;
        }
        array(&mut writer, self.streams.len())?;
        for stream in &self.streams {
            array(&mut writer, stream.values.len())?;
            for &value in &stream.values {
                float(&mut writer, value)?;
            }
        }
        Ok(())
    }
}

impl<T: Scalar> SegmentationData<T> {
    pub fn read_from(reader: impl Read) -> io::Result<Self> {
        let mut decoder = Decoder {
            reader,
            remaining_entries: DecodeLimits::default().max_array_entries,
        };
        let segmentation = decoder.segmentation()?;
        decoder.end()?;
        Ok(segmentation)
    }

    /// Preserve both legacy array-header mismatches for C interoperability.
    pub fn write_to(&self, mut writer: impl Write) -> io::Result<()> {
        self.validate()
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;
        array(&mut writer, 3)?;
        integers(&mut writer, &self.boundaries)?;
        integers(&mut writer, &self.duration_states)?;
        array(&mut writer, self.output_states.len())?;
        for states in &self.output_states {
            integers(&mut writer, states)?;
        }
        array(&mut writer, self.outgoing.len())?;
        for jumps in &self.outgoing {
            array(&mut writer, jumps.len())?;
            for jump in jumps {
                integer(&mut writer, jump.delta)?;
                float(&mut writer, jump.probability)?;
            }
        }
        Ok(())
    }
}

/// Read a complete legacy observation set.
pub fn read_observation_set_with_precision<T: Scalar>(
    reader: impl Read,
) -> io::Result<Vec<ObservationData<T>>> {
    let mut decoder = Decoder {
        reader,
        remaining_entries: DecodeLimits::default().max_array_entries,
    };
    let count = decoder.array()?;
    let mut samples = Vec::new();
    for _ in 0..count {
        samples.push(decoder.observation()?);
    }
    decoder.end()?;
    Ok(samples)
}

pub fn write_observation_set_with_precision<T: Scalar>(
    samples: &[ObservationData<T>],
    mut writer: impl Write,
) -> io::Result<()> {
    for sample in samples {
        sample
            .validate()
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;
    }
    array(&mut writer, samples.len())?;
    for sample in samples {
        sample.write_to(&mut writer)?;
    }
    Ok(())
}

/// Read nested segmentations despite their historical array-header mismatches.
pub fn read_segmentation_set_with_precision<T: Scalar>(
    reader: impl Read,
) -> io::Result<Vec<SegmentationData<T>>> {
    let mut decoder = Decoder {
        reader,
        remaining_entries: DecodeLimits::default().max_array_entries,
    };
    let count = decoder.array()?;
    let mut samples = Vec::new();
    for _ in 0..count {
        samples.push(decoder.segmentation()?);
    }
    decoder.end()?;
    Ok(samples)
}

pub fn write_segmentation_set_with_precision<T: Scalar>(
    samples: &[SegmentationData<T>],
    mut writer: impl Write,
) -> io::Result<()> {
    for sample in samples {
        sample
            .validate()
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidInput, error))?;
    }
    array(&mut writer, samples.len())?;
    for sample in samples {
        sample.write_to(&mut writer)?;
    }
    Ok(())
}

/// Read a binary32 observation set through the original API.
pub fn read_observation_set(reader: impl Read) -> io::Result<Vec<crate::Observation>> {
    read_observation_set_with_precision(reader)
}
/// Write a binary32 observation set through the original API.
pub fn write_observation_set(samples: &[crate::Observation], writer: impl Write) -> io::Result<()> {
    write_observation_set_with_precision(samples, writer)
}
/// Read a binary32 segmentation set through the original API.
pub fn read_segmentation_set(reader: impl Read) -> io::Result<Vec<crate::Segmentation>> {
    read_segmentation_set_with_precision(reader)
}
/// Write a binary32 segmentation set through the original API.
pub fn write_segmentation_set(
    samples: &[crate::Segmentation],
    writer: impl Write,
) -> io::Result<()> {
    write_segmentation_set_with_precision(samples, writer)
}
