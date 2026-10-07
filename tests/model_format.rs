#![cfg(feature = "serialization")]

use liblrhsmm_rs::{
    Model,
    serial::{DecodeLimits, ModelEncoding},
};
const LEGACY_MODEL: &[u8] = include_bytes!("fixtures/cmu-arctic-all-speakers.hsmm");

#[test]
fn preserves_every_field_of_the_upstream_model() {
    let model = Model::read_from(LEGACY_MODEL).unwrap();
    assert!(!model.streams.is_empty());
    assert!(!model.durations.is_empty());
    let mut encoded = Vec::new();
    model.write_to(&mut encoded).unwrap();
    assert_eq!(Model::read_from(encoded.as_slice()).unwrap(), model);
}

#[test]
fn matches_the_current_c_serializer_byte_for_byte() {
    let historical = Model::read_from(LEGACY_MODEL).unwrap();
    let current_bytes = include_bytes!("fixtures/current-c-model.hsmm");
    assert_eq!(
        Model::read_from(current_bytes.as_slice()).unwrap(),
        historical
    );
    let mut output = Vec::new();
    historical.write_to(&mut output).unwrap();
    assert_eq!(output, current_bytes);
    output.clear();
    historical
        .write_with_encoding(&mut output, ModelEncoding::WithoutVarianceFloors)
        .unwrap();
    assert_eq!(output, LEGACY_MODEL);
}
#[test]
fn rejects_truncated_models() {
    for length in [0, 1, 2, 3, 5, 10, 100, LEGACY_MODEL.len() - 1] {
        assert!(
            Model::read_from(&LEGACY_MODEL[..length]).is_err(),
            "length {length}"
        );
    }
}
#[test]
fn rejects_trailing_objects() {
    let mut input = LEGACY_MODEL.to_vec();
    input.push(0xc0);
    assert!(Model::read_from(input.as_slice()).is_err());
}
#[test]
fn bounds_declared_array_sizes_before_allocation() {
    let input = [0x92, 0xdd, 0xff, 0xff, 0xff, 0xff];
    assert!(
        Model::read_from(input.as_slice())
            .unwrap_err()
            .to_string()
            .contains("decoding limit")
    );
    assert!(
        Model::read_with_limits(
            LEGACY_MODEL,
            DecodeLimits {
                max_array_entries: 10
            }
        )
        .is_err()
    );
}
#[test]
fn rejects_inconsistent_storage_before_writing() {
    let mut model = Model::read_from(LEGACY_MODEL).unwrap();
    model.streams[0].mixtures[0].means.pop();
    let mut output = Vec::new();
    assert!(model.write_to(&mut output).is_err());
    assert!(output.is_empty());
}

#[test]
fn preserves_the_historical_schema_without_discarding_parameters() {
    let mut model = Model::read_from(LEGACY_MODEL).unwrap();
    let mut output = Vec::new();
    model
        .write_with_encoding(&mut output, ModelEncoding::WithoutVarianceFloors)
        .unwrap();
    assert_eq!(Model::read_from(output.as_slice()).unwrap(), model);
    model.streams[0].mixtures[0].variance_floors[0] = 0.25;
    output.clear();
    assert!(
        model
            .write_with_encoding(&mut output, ModelEncoding::WithoutVarianceFloors)
            .is_err()
    );
    assert!(output.is_empty());
}

#[test]
fn preserves_output_io_error_kind() {
    struct BrokenWriter;
    impl std::io::Write for BrokenWriter {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::from(std::io::ErrorKind::BrokenPipe))
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let model = Model::read_from(LEGACY_MODEL).unwrap();
    assert_eq!(
        model.write_to(BrokenWriter).unwrap_err().kind(),
        std::io::ErrorKind::BrokenPipe
    );
}

#[test]
fn rejects_unaddressable_constructor_dimensions() {
    assert!(liblrhsmm_rs::GaussianMixture::new(i32::MAX as usize, i32::MAX as usize).is_err());
}
