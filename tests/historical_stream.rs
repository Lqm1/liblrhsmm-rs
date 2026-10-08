#![cfg(feature = "serialization")]
use liblrhsmm_rs::model::{ModelParameters, StreamParameters};
use liblrhsmm_rs::scalar::Scalar;
use liblrhsmm_rs::serial::ModelEncoding;
fn check<T: Scalar>() {
    let original = include_bytes!("fixtures/cmu-arctic-all-speakers.hsmm");
    let fixture = include_bytes!("fixtures/c-historical-standalone-stream.bin");
    assert_eq!(fixture.as_slice(), &original[2..28391]);
    let value = StreamParameters::<T>::read_from(fixture.as_slice()).unwrap();
    let model = ModelParameters::<T>::read_from(original.as_slice()).unwrap();
    assert_eq!(value, model.streams[0]);
    let mut output = Vec::new();
    value
        .write_with_encoding(&mut output, ModelEncoding::WithoutVarianceFloors)
        .unwrap();
    assert_eq!(output, fixture);
}
#[test]
fn extracted_original_pretrained_stream_preserves_historical_bytes_and_both_precisions() {
    check::<f32>();
    check::<f64>();
}
