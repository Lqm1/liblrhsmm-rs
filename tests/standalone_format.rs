use liblrhsmm_rs::{
    Duration, GaussianMixture, Stream, StreamStatistics,
    serial::{DecodeLimits, ModelEncoding},
};
use std::io::{self, Write};

const DURATION: &[u8] = include_bytes!("fixtures/c-standalone-duration.bin");
const GMM: &[u8] = include_bytes!("fixtures/c-standalone-gmm.bin");
const STREAM: &[u8] = include_bytes!("fixtures/c-standalone-stream.bin");

#[test]
fn standalone_objects_preserve_c_bytes_and_fields() {
    let duration = Duration::read_from(DURATION).unwrap();
    assert_eq!(
        duration,
        Duration {
            mean: 4.25,
            variance: 1.5,
            minimum: 2,
            maximum: 9,
            fixed_mean: 4,
            variance_floor: 0.25
        }
    );
    let gmm = GaussianMixture::read_from(GMM).unwrap();
    assert_eq!(gmm.weights, [0.5, 0.5]);
    assert_eq!(gmm.means, [-1.0, -0.5, 0.0, 0.5]);
    assert_eq!(gmm.variances, [0.5, 0.75, 1.0, 1.25]);
    assert_eq!(gmm.variance_floors, [0.0, 0.125, 0.25, 0.375]);
    let stream = Stream::read_from(STREAM).unwrap();
    assert_eq!(stream.weight, 0.75);
    assert_eq!(stream.mixtures[0].means, [0.0, 0.5]);
    assert_eq!(stream.mixtures[1], gmm);
    let mut encoded = Vec::new();
    duration.write_to(&mut encoded).unwrap();
    assert_eq!(encoded, DURATION);
    encoded.clear();
    gmm.write_to(&mut encoded).unwrap();
    assert_eq!(encoded, GMM);
    encoded.clear();
    stream.write_to(&mut encoded).unwrap();
    assert_eq!(encoded, STREAM);
    let mut statistics = StreamStatistics::from_stream(&stream).unwrap();
    assert_eq!(statistics.mixtures[0].component_occupancies, [0.0]);
    assert_eq!(statistics.mixtures[1].weighted_sums, [0.0; 4]);
    statistics.mixtures[1].weighted_sums[3] = 10.0;
    statistics.clear();
    assert_eq!(statistics, StreamStatistics::from_stream(&stream).unwrap());
}

#[test]
fn standalone_decoders_reject_truncated_trailing_and_over_budget_data() {
    for length in 0..DURATION.len() {
        assert!(Duration::read_from(&DURATION[..length]).is_err());
    }
    for length in 0..GMM.len() {
        assert!(GaussianMixture::read_from(&GMM[..length]).is_err());
    }
    for length in 0..STREAM.len() {
        assert!(Stream::read_from(&STREAM[..length]).is_err());
    }
    let limits = DecodeLimits {
        max_array_entries: 1,
    };
    assert!(Duration::read_with_limits(DURATION, limits).is_err());
    assert!(GaussianMixture::read_with_limits(GMM, limits).is_err());
    assert!(Stream::read_with_limits(STREAM, limits).is_err());
    let trailing = [DURATION, &[0]].concat();
    assert!(Duration::read_from(trailing.as_slice()).is_err());
    let trailing = [GMM, &[0]].concat();
    assert!(GaussianMixture::read_from(trailing.as_slice()).is_err());
    let trailing = [STREAM, &[0]].concat();
    assert!(Stream::read_from(trailing.as_slice()).is_err());
}

#[test]
fn historical_standalone_encoding_preserves_parameters_or_reports_loss() {
    let mut stream = Stream::read_from(STREAM).unwrap();
    let mut encoded = Vec::new();
    assert!(
        stream
            .write_with_encoding(&mut encoded, ModelEncoding::WithoutVarianceFloors)
            .is_err()
    );
    assert!(encoded.is_empty());
    assert!(
        stream.mixtures[1]
            .write_with_encoding(&mut encoded, ModelEncoding::WithoutVarianceFloors)
            .is_err()
    );
    assert!(encoded.is_empty());
    for mixture in &mut stream.mixtures {
        mixture.variance_floors.fill(0.0);
    }
    stream
        .write_with_encoding(&mut encoded, ModelEncoding::WithoutVarianceFloors)
        .unwrap();
    assert_eq!(Stream::read_from(encoded.as_slice()).unwrap(), stream);
    encoded.clear();
    stream.mixtures[1]
        .write_with_encoding(&mut encoded, ModelEncoding::WithoutVarianceFloors)
        .unwrap();
    assert_eq!(
        GaussianMixture::read_from(encoded.as_slice()).unwrap(),
        stream.mixtures[1]
    );
}

struct FailedWriter;
impl Write for FailedWriter {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Err(io::Error::from(io::ErrorKind::BrokenPipe))
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn standalone_writers_preserve_io_errors_and_validate_before_output() {
    assert_eq!(
        Duration::default()
            .write_to(FailedWriter)
            .unwrap_err()
            .kind(),
        io::ErrorKind::BrokenPipe
    );
    let mut gmm = GaussianMixture::read_from(GMM).unwrap();
    assert_eq!(
        gmm.write_to(FailedWriter).unwrap_err().kind(),
        io::ErrorKind::BrokenPipe
    );
    let mut stream = Stream::read_from(STREAM).unwrap();
    assert_eq!(
        stream.write_to(FailedWriter).unwrap_err().kind(),
        io::ErrorKind::BrokenPipe
    );
    let mut encoded = Vec::new();
    gmm.means.pop();
    assert_eq!(
        gmm.write_to(&mut encoded).unwrap_err().kind(),
        io::ErrorKind::InvalidInput
    );
    assert!(encoded.is_empty());
    stream.mixtures[1] = gmm;
    assert!(stream.write_to(&mut encoded).is_err());
    assert!(encoded.is_empty());
    assert!(StreamStatistics::from_stream(&stream).is_err());
}
