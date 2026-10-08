#![cfg(feature = "serialization")]
use liblrhsmm_rs::model::DurationParameters;
use liblrhsmm_rs::scalar::Scalar;
use liblrhsmm_rs::serial::DecodeLimits;
use std::io::Cursor;
fn check<T: Scalar>() {
    let bytes = include_bytes!("fixtures/c-standalone-duration.bin");
    let joined = [bytes.as_slice(), bytes.as_slice(), &[42]].concat();
    let expected = DurationParameters::<T>::read_from(bytes.as_slice()).unwrap();
    assert!(DurationParameters::<T>::read_from(joined.as_slice()).is_err());
    let mut reader = Cursor::new(joined);
    assert_eq!(
        DurationParameters::<T>::read_prefix(&mut reader).unwrap(),
        expected
    );
    assert_eq!(reader.position(), bytes.len() as u64);
    assert!(
        DurationParameters::<T>::read_prefix_with_limits(
            &mut reader,
            DecodeLimits {
                max_array_entries: 5
            }
        )
        .is_err()
    );
    reader.set_position(bytes.len() as u64);
    assert_eq!(
        DurationParameters::<T>::read_prefix_with_limits(
            &mut reader,
            DecodeLimits {
                max_array_entries: 6
            }
        )
        .unwrap(),
        expected
    );
    assert_eq!(reader.position(), bytes.len() as u64 * 2);
    assert_eq!(reader.get_ref()[reader.position() as usize], 42);
    for end in 0..bytes.len() {
        assert!(DurationParameters::<T>::read_prefix(&bytes[..end]).is_err());
    }
}
#[test]
fn binary32_prefix_preserves_original_c_object_boundary() {
    check::<f32>();
}
#[test]
fn binary64_prefix_preserves_original_c_object_boundary() {
    check::<f64>();
}
