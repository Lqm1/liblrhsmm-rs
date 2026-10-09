#![cfg(feature = "serialization")]

use liblrhsmm_rs::data::{ObservationData, SegmentationData};
use liblrhsmm_rs::model::{GaussianMixtureParameters, ModelParameters, StreamParameters};
use liblrhsmm_rs::scalar::Scalar;
use liblrhsmm_rs::serial::{self, DecodeLimits};
use std::fmt::Debug;
use std::io::{self, Cursor, Read};

// A prefix reader must not ask the underlying stream for even one trailing byte.
struct BoundaryReader<'a> {
    input: &'a [u8],
    position: usize,
}
impl Read for BoundaryReader<'_> {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        assert!(
            self.position < self.input.len(),
            "read beyond object boundary"
        );
        let count = output.len().min(1);
        output[..count].copy_from_slice(&self.input[self.position..self.position + count]);
        self.position += count;
        Ok(count)
    }
}

fn verify<V: PartialEq + Debug>(
    bytes: &[u8],
    strict: impl Fn(&[u8]) -> io::Result<V>,
    prefix: impl Fn(&mut Cursor<Vec<u8>>, DecodeLimits) -> io::Result<V>,
    guarded: impl Fn(BoundaryReader<'_>) -> io::Result<V>,
) {
    let expected = strict(bytes).unwrap();
    assert_eq!(
        guarded(BoundaryReader {
            input: bytes,
            position: 0
        })
        .unwrap(),
        expected
    );
    let mut reader = Cursor::new([bytes, bytes, &[42]].concat());
    assert!(strict(reader.get_ref()).is_err());
    for index in 1..=2 {
        assert_eq!(
            prefix(&mut reader, DecodeLimits::default()).unwrap(),
            expected
        );
        assert_eq!(reader.position(), (index * bytes.len()) as u64);
    }
    let mut sentinel = [0];
    reader.read_exact(&mut sentinel).unwrap();
    assert_eq!(sentinel, [42]);
    reader.set_position(0);
    assert!(
        prefix(
            &mut reader,
            DecodeLimits {
                max_array_entries: 0
            }
        )
        .is_err()
    );
    let truncations: Vec<_> = if bytes.len() <= 4096 {
        (0..bytes.len()).collect()
    } else {
        // Large real models: exercise the header, interior and final payload
        // without making decoding quadratic in the entire fixture size.
        (0..64)
            .chain((1..64).map(|part| bytes.len() * part / 64))
            .chain(bytes.len() - 64..bytes.len())
            .collect()
    };
    for end in truncations {
        let mut reader = Cursor::new(bytes[..end].to_vec());
        assert!(
            prefix(&mut reader, DecodeLimits::default()).is_err(),
            "truncation at {end}"
        );
    }
}

fn check<T: Scalar>() {
    macro_rules! object {
        ($ty:ty, $file:literal) => {
            verify(
                include_bytes!($file),
                |bytes| <$ty>::read_from(bytes),
                |reader, limits| <$ty>::read_prefix_with_limits(reader, limits),
                |reader| <$ty>::read_prefix(reader),
            );
        };
    }
    object!(
        GaussianMixtureParameters<T>,
        "../../../tests/fixtures/c-standalone-gmm.bin"
    );
    object!(
        StreamParameters<T>,
        "../../../tests/fixtures/c-standalone-stream.bin"
    );
    object!(
        ModelParameters<T>,
        "../../../tests/fixtures/current-c-model.hsmm"
    );
    object!(
        ObservationData<T>,
        "../../../tests/fixtures/c-observation.bin"
    );
    object!(
        SegmentationData<T>,
        "../../../tests/fixtures/c-segmentation.bin"
    );
    verify(
        include_bytes!("../../../tests/fixtures/c-observation-set.bin"),
        |bytes| serial::read_observation_set_with_precision::<T>(bytes),
        |reader, limits| serial::read_observation_set_prefix_with_limits::<T>(reader, limits),
        |reader| serial::read_observation_set_prefix_with_precision::<T>(reader),
    );
    verify(
        include_bytes!("../../../tests/fixtures/c-segmentation-set.bin"),
        |bytes| serial::read_segmentation_set_with_precision::<T>(bytes),
        |reader, limits| serial::read_segmentation_set_prefix_with_limits::<T>(reader, limits),
        |reader| serial::read_segmentation_set_prefix_with_precision::<T>(reader),
    );
}

#[test]
fn binary32_readers_preserve_all_seven_original_c_object_boundaries() {
    check::<f32>();
    // Verify the default precision entry points on a sequential stream too.
    let mut reader = Cursor::new(
        [
            include_bytes!("../../../tests/fixtures/c-observation-set.bin").as_slice(),
            include_bytes!("../../../tests/fixtures/c-segmentation-set.bin").as_slice(),
        ]
        .concat(),
    );
    serial::read_observation_set_prefix(&mut reader).unwrap();
    serial::read_segmentation_set_prefix(&mut reader).unwrap();
    assert_eq!(reader.position(), reader.get_ref().len() as u64);
}
#[test]
fn binary64_readers_preserve_all_seven_original_c_object_boundaries() {
    check::<f64>();
}
