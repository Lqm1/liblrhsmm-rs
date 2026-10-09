#![cfg(feature = "serialization")]
use liblrhsmm_rs::{data::ObservationData, scalar::Scalar, serial::DecodeLimits};
use std::io::{self, Cursor, Read};

const ORIGINAL: &[u8] = include_bytes!("../../../tests/fixtures/c-observation.bin");
// Three top-level entries, two dimensions, two streams and nine feature values.
const BUDGET: usize = 16;

struct FailAtEnd<'a> {
    bytes: &'a [u8],
}
impl Read for FailAtEnd<'_> {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        if self.bytes.is_empty() {
            return Err(io::ErrorKind::PermissionDenied.into());
        }
        self.bytes.read(output)
    }
}
fn verify<T: Scalar>() {
    let expected = ObservationData::<T>::read_from(ORIGINAL).unwrap();
    for budget in 0..BUDGET {
        assert!(
            ObservationData::<T>::read_with_limits(
                ORIGINAL,
                DecodeLimits {
                    max_array_entries: budget
                }
            )
            .is_err()
        );
    }
    let decoded = ObservationData::<T>::read_with_limits(
        ORIGINAL,
        DecodeLimits {
            max_array_entries: BUDGET,
        },
    )
    .unwrap();
    assert_eq!(decoded, expected);
    let mut bytes = Vec::new();
    decoded.write_to(&mut bytes).unwrap();
    assert_eq!(bytes, ORIGINAL);
    for end in 0..ORIGINAL.len() {
        assert!(
            ObservationData::<T>::read_with_limits(&ORIGINAL[..end], DecodeLimits::default())
                .is_err()
        );
    }
    let joined = [ORIGINAL, ORIGINAL, &[42]].concat();
    assert!(
        ObservationData::<T>::read_with_limits(joined.as_slice(), DecodeLimits::default()).is_err()
    );
    let mut input = Cursor::new(joined);
    for position in [ORIGINAL.len(), ORIGINAL.len() * 2] {
        assert_eq!(
            ObservationData::<T>::read_prefix_with_limits(
                &mut input,
                DecodeLimits {
                    max_array_entries: BUDGET
                }
            )
            .unwrap(),
            expected
        );
        assert_eq!(input.position(), position as u64);
    }
    let error = ObservationData::<T>::read_with_limits(
        FailAtEnd { bytes: ORIGINAL },
        DecodeLimits::default(),
    )
    .unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
    assert_eq!(
        ObservationData::<T>::read_prefix_with_limits(
            FailAtEnd { bytes: ORIGINAL },
            DecodeLimits::default()
        )
        .unwrap(),
        expected
    );
}
#[test]
fn observation_f32_limits_exact_budget_wire_prefix_and_eof_error() {
    verify::<f32>();
}
#[test]
fn observation_f64_limits_exact_budget_wire_prefix_and_eof_error() {
    verify::<f64>();
}
