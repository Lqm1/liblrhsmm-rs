use liblrhsmm_rs::numerics;

#[test]
fn public_coefficient_views_match_every_c_table_entry() {
    macro_rules! verify {
        ($fixture:expr, $width:expr, $table:expr, $length:expr, $index:expr) => {{
            let fixture = $fixture;
            let mut table = $table;
            let start = 12 + $index * 7000 * $width;
            let expected = &fixture[start..start + 7000 * $width];
            assert_eq!($length, 7000);
            assert_eq!(table.len(), 7000);
            assert_eq!(
                table.next().unwrap().to_le_bytes().as_slice(),
                &expected[..$width]
            );
            assert_eq!(table.len(), 6999);
            assert_eq!(
                table.next_back().unwrap().to_le_bytes().as_slice(),
                &expected[6999 * $width..]
            );
            assert_eq!(table.len(), 6998);
            for bytes in expected[$width..6999 * $width].chunks_exact($width) {
                assert_eq!(table.next().unwrap().to_le_bytes().as_slice(), bytes);
            }
            assert_eq!(table.len(), 0);
            assert!(table.next().is_none());
            assert!(table.next_back().is_none());
        }};
    }
    verify!(
        include_bytes!("../../../tests/fixtures/c-numerics-float.bin"),
        4,
        numerics::log_add_table(),
        numerics::LOG_ADD_TABLE_SIZE,
        0
    );
    verify!(
        include_bytes!("../../../tests/fixtures/c-numerics-float.bin"),
        4,
        numerics::exponential_table(),
        numerics::EXPONENTIAL_TABLE_SIZE,
        1
    );
    verify!(
        include_bytes!("../../../tests/fixtures/c-numerics-double.bin"),
        8,
        numerics::log_add_table_f64(),
        numerics::LOG_ADD_TABLE_SIZE,
        0
    );
    verify!(
        include_bytes!("../../../tests/fixtures/c-numerics-double.bin"),
        8,
        numerics::exponential_table_f64(),
        numerics::EXPONENTIAL_TABLE_SIZE,
        1
    );
    assert_eq!(numerics::LOG_ADD_TABLE_STEP, 0.001);
    assert_eq!(numerics::EXPONENTIAL_TABLE_STEP, 0.002);
    assert_eq!(numerics::OCCUPANCY_EPSILON, 1e-7);
}

fn integer(bytes: &[u8], offset: &mut usize) -> u32 {
    let value = u32::from_le_bytes(bytes[*offset..*offset + 4].try_into().unwrap());
    *offset += 4;
    value
}

fn scalar<const N: usize>(bytes: &[u8], offset: &mut usize) -> [u8; N] {
    let value = bytes[*offset..*offset + N].try_into().unwrap();
    *offset += N;
    value
}

fn header(bytes: &[u8], width: usize, tables: [&[u8]; 2]) -> (usize, u32) {
    assert_eq!(&bytes[..4], b"NUM1");
    let mut offset = 4;
    assert_eq!(integer(bytes, &mut offset) as usize, width);
    assert_eq!(integer(bytes, &mut offset), 7000);
    for table in tables {
        assert_eq!(&bytes[offset..offset + 7000 * width], table);
        offset += 7000 * width;
    }
    let count = integer(bytes, &mut offset);
    assert_eq!(count, 42008);
    (offset, count)
}

#[test]
fn binary32_matches_compiled_c_at_table_boundaries() {
    let bytes = include_bytes!("../../../tests/fixtures/c-numerics-float.bin");
    let (mut offset, count) = header(
        bytes,
        4,
        [
            include_bytes!("../src/lse-table.bin"),
            include_bytes!("../src/exp-table.bin"),
        ],
    );
    for record in 0..count {
        let kind = integer(bytes, &mut offset);
        let a = f32::from_le_bytes(scalar(bytes, &mut offset));
        let b = f32::from_le_bytes(scalar(bytes, &mut offset));
        let expected = f32::from_le_bytes(scalar(bytes, &mut offset));
        let actual = match kind {
            0 => numerics::log_add(a, b),
            1 => numerics::negative_exponential(a),
            2 => numerics::exponential(a),
            _ => panic!("unknown reference operation"),
        };
        assert_eq!(
            actual.to_bits(),
            expected.to_bits(),
            "record {record}: {kind}, {a}, {b}"
        );
    }
    assert_eq!(offset, bytes.len());
}

#[test]
fn binary64_matches_compiled_c_at_table_boundaries() {
    let bytes = include_bytes!("../../../tests/fixtures/c-numerics-double.bin");
    let (mut offset, count) = header(
        bytes,
        8,
        [
            include_bytes!("../src/lse-table-f64.bin"),
            include_bytes!("../src/exp-table-f64.bin"),
        ],
    );
    for record in 0..count {
        let kind = integer(bytes, &mut offset);
        let a = f64::from_le_bytes(scalar(bytes, &mut offset));
        let b = f64::from_le_bytes(scalar(bytes, &mut offset));
        let expected = f64::from_le_bytes(scalar(bytes, &mut offset));
        let actual = match kind {
            0 => numerics::log_add_f64(a, b),
            1 => numerics::negative_exponential_f64(a),
            2 => numerics::exponential_f64(a),
            _ => panic!("unknown reference operation"),
        };
        assert_eq!(
            actual.to_bits(),
            expected.to_bits(),
            "record {record}: {kind}, {a}, {b}"
        );
    }
    assert_eq!(offset, bytes.len());
}

#[test]
fn binary64_retains_precision_and_defines_nonfinite_limits() {
    assert_ne!(
        numerics::log_add_f64(0.0, 0.0),
        f64::from(numerics::log_add(0.0, 0.0))
    );
    for infinity in [f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(numerics::log_add_f64(infinity, infinity), infinity);
    }
    assert_eq!(numerics::log_add_f64(f64::MAX, -f64::MAX), f64::MAX);
    assert!(numerics::log_add_f64(f64::NAN, 0.0).is_nan());
    assert!(numerics::negative_exponential_f64(f64::NAN).is_nan());
    assert!(numerics::exponential_f64(f64::NAN).is_nan());
    assert_eq!(numerics::negative_exponential_f64(f64::NEG_INFINITY), 0.0);
    assert_eq!(numerics::negative_exponential_f64(f64::INFINITY), 1.0);
    assert_eq!(numerics::exponential_f64(f64::INFINITY), f64::INFINITY);
    assert_eq!(numerics::negative_exponential_f64(-0.0), 1.0);
    let input = 1.0 + f64::EPSILON;
    assert_eq!(
        numerics::exponential_f64(input),
        f64::from(numerics::exponential(input as f32))
    );
}
