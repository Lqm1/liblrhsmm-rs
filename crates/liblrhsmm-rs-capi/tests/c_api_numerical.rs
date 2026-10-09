use liblrhsmm_rs_capi::*;
use std::{ptr, sync::Mutex};
static TABLES: Mutex<()> = Mutex::new(());

fn integer(bytes: &[u8], offset: &mut usize) -> u32 {
    let value = u32::from_le_bytes(bytes[*offset..*offset + 4].try_into().unwrap());
    *offset += 4;
    value
}
macro_rules! corpus {
    ($name:ident,$t:ty,$width:expr,$fixture:expr,$copy:ident,$add:ident,$negative:ident,$exp:ident) => {
        #[test]
        fn $name() {
            let _guard = TABLES.lock().unwrap();
            let bytes = $fixture;
            assert_eq!(&bytes[..4], b"NUM1");
            let mut offset = 4;
            assert_eq!(integer(bytes, &mut offset), $width);
            assert_eq!(integer(bytes, &mut offset), 7000);
            // SAFETY: Independent initialized output buffers retained through every checked C call.
            unsafe {
                for table in 0..2 {
                    let mut output = vec![0.0 as $t; 7000];
                    assert_eq!($copy(table, 0, output.as_mut_ptr(), 7000), 0);
                    for value in output {
                        assert_eq!(
                            value.to_le_bytes().as_slice(),
                            &bytes[offset..offset + $width as usize]
                        );
                        offset += $width as usize;
                    }
                }
                let count = integer(bytes, &mut offset);
                assert_eq!(count, 42008);
                for record in 0..count {
                    let kind = integer(bytes, &mut offset);
                    let mut read = || {
                        let v = <$t>::from_le_bytes(
                            bytes[offset..offset + $width as usize].try_into().unwrap(),
                        );
                        offset += $width as usize;
                        v
                    };
                    let a = read();
                    let b = read();
                    let expected = read();
                    let mut output = 0.0 as $t;
                    let status = match kind {
                        0 => $add(a, b, &mut output),
                        1 => $negative(a, &mut output),
                        2 => $exp(a, &mut output),
                        _ => panic!("bad fixture kind"),
                    };
                    assert_eq!(status, 0);
                    assert_eq!(output.to_bits(), expected.to_bits(), "record {record}");
                }
            }
            assert_eq!(offset, bytes.len());
        }
    };
}
corpus!(
    original_binary32_tables_and_all_42008_records,
    f32,
    4,
    include_bytes!("../../../tests/fixtures/c-numerics-float.bin"),
    liblrhsmm_rs_table_copy_f32,
    liblrhsmm_rs_log_add_f32,
    liblrhsmm_rs_negative_exponential_f32,
    liblrhsmm_rs_exponential_f32
);
corpus!(
    original_binary64_tables_and_all_42008_records,
    f64,
    8,
    include_bytes!("../../../tests/fixtures/c-numerics-double.bin"),
    liblrhsmm_rs_table_copy_f64,
    liblrhsmm_rs_log_add_f64,
    liblrhsmm_rs_negative_exponential_f64,
    liblrhsmm_rs_exponential_f64
);

macro_rules! mutation {
    ($name:ident,$t:ty,$bits:expr,$get:ident,$replace:ident,$copy:ident,$add:ident,$negative:ident) => {
        #[test]
        fn $name() {
            let _guard = TABLES.lock().unwrap();
            struct Restore([$t; 2]);
            impl Drop for Restore {
                fn drop(&mut self) {
                    // SAFETY: Restoration uses independent temporary output storage and valid table indices.
                    unsafe {
                        for table in 0..2 {
                            let mut ignored = 0.0;
                            assert_eq!($replace(table, 0, self.0[table as usize], &mut ignored), 0);
                        }
                    }
                }
            }
            let mut originals = [0.0; 2];
            // SAFETY: All tables use valid indices and outputs are independent initialized storage.
            unsafe {
                for table in 0..2 {
                    assert_eq!($get(table, 0, &mut originals[table as usize]), 0);
                }
                let restore = Restore(originals);
                for table in 0..2 {
                    let mut previous = 0.0;
                    let changed = if table == 0 { 0.125 } else { 0.375 };
                    assert_eq!($replace(table, 0, changed, &mut previous), 0);
                    assert_eq!(previous.to_bits(), restore.0[table as usize].to_bits());
                    let mut result = 0.0;
                    assert_eq!(
                        if table == 0 {
                            $add(0.0, 0.0, &mut result)
                        } else {
                            $negative(-0.001, &mut result)
                        },
                        0
                    );
                    assert_eq!(result, changed);
                    let nan = <$t>::from_bits($bits);
                    assert_eq!($replace(table, 0, nan, &mut previous), 0);
                    assert_eq!(previous, changed);
                    assert_eq!($get(table, 0, &mut result), 0);
                    assert_eq!(result.to_bits(), nan.to_bits());
                    let mut output = [0.0; 1];
                    assert_eq!($copy(table, 0, output.as_mut_ptr(), 1), 0);
                    assert_eq!(output[0].to_bits(), nan.to_bits());
                    assert_eq!($replace(table, 0, 1.0, ptr::null_mut()), 1);
                    assert_eq!($get(table, 0, &mut result), 0);
                    assert_eq!(result.to_bits(), nan.to_bits());
                }
            }
        }
    };
}
mutation!(
    binary32_mutation_preserves_bits_and_changes_native_kernels,
    f32,
    0x7fc1_2345,
    liblrhsmm_rs_table_get_f32,
    liblrhsmm_rs_table_replace_f32,
    liblrhsmm_rs_table_copy_f32,
    liblrhsmm_rs_log_add_f32,
    liblrhsmm_rs_negative_exponential_f32
);
mutation!(
    binary64_mutation_preserves_bits_and_changes_native_kernels,
    f64,
    0x7ff8_0000_0001_2345,
    liblrhsmm_rs_table_get_f64,
    liblrhsmm_rs_table_replace_f64,
    liblrhsmm_rs_table_copy_f64,
    liblrhsmm_rs_log_add_f64,
    liblrhsmm_rs_negative_exponential_f64
);

#[test]
fn invalid_ranges_and_outputs_preserve_values() {
    let _guard = TABLES.lock().unwrap();
    let mut output = [71.0_f64; 2];
    // SAFETY: Invalid null/ranges are checked before use; other buffers are independent live storage.
    unsafe {
        assert_eq!(
            liblrhsmm_rs_table_copy_f64(0, 6999, output.as_mut_ptr(), 2),
            2
        );
        assert_eq!(output, [71.0; 2]);
        assert_eq!(liblrhsmm_rs_table_copy_f64(0, 7000, ptr::null_mut(), 0), 0);
        assert_eq!(liblrhsmm_rs_table_copy_f64(0, 7001, ptr::null_mut(), 0), 2);
        assert_eq!(liblrhsmm_rs_table_copy_f64(3, 0, output.as_mut_ptr(), 1), 3);
        assert_eq!(liblrhsmm_rs_table_get_f64(0, 7000, &mut output[0]), 2);
        assert_eq!(
            liblrhsmm_rs_table_replace_f64(0, 7000, 1.0, &mut output[0]),
            2
        );
        assert_eq!(liblrhsmm_rs_table_get_f64(0, 0, ptr::null_mut()), 1);
        assert_eq!(liblrhsmm_rs_log_add_f64(0.0, 0.0, ptr::null_mut()), 1);
        assert_eq!(liblrhsmm_rs_exponential_f32(0.0, ptr::null_mut()), 1);
        assert_eq!(
            liblrhsmm_rs_negative_exponential_f32(0.0, ptr::null_mut()),
            1
        );
        #[repr(align(8))]
        struct Aligned([u8; 16]);
        let mut bytes = Aligned([0; 16]);
        assert_eq!(
            liblrhsmm_rs_table_get_f64(0, 0, bytes.0.as_mut_ptr().add(1).cast()),
            1
        );
        assert_eq!(output, [71.0; 2]);
    }
}

#[test]
fn numerical_constants_and_ieee_limits() {
    let _guard = TABLES.lock().unwrap();
    let mut constants = LiblrhsmmRsNumericalConstants {
        log_add_count: 0,
        exponential_count: 0,
        log_add_step: 0.0,
        exponential_step: 0.0,
        occupancy_epsilon: 0.0,
        log_zero_f32: 0.0,
        log_zero_f64: 0.0,
        pi: 0.0,
    };
    // SAFETY: Every output is independent exclusively writable initialized storage.
    unsafe {
        assert_eq!(liblrhsmm_rs_numerical_constants(&mut constants), 0);
        assert_eq!(
            (constants.log_add_count, constants.exponential_count),
            (7000, 7000)
        );
        assert_eq!(constants.log_add_step.to_bits(), 0.001_f64.to_bits());
        assert_eq!(constants.exponential_step.to_bits(), 0.002_f64.to_bits());
        assert_eq!(constants.occupancy_epsilon.to_bits(), 1e-7_f64.to_bits());
        assert_eq!(constants.log_zero_f32.to_bits(), (-1e15_f32).to_bits());
        assert_eq!(constants.log_zero_f64.to_bits(), (-1e15_f64).to_bits());
        assert_eq!(constants.pi.to_bits(), std::f64::consts::PI.to_bits());
        assert_eq!(liblrhsmm_rs_numerical_constants(ptr::null_mut()), 1);
        assert_eq!(liblrhsmm_rs_abi_version(), 1);
        let mut output = 0.0;
        assert_eq!(
            liblrhsmm_rs_log_add_f64(f64::INFINITY, f64::INFINITY, &mut output),
            0
        );
        assert_eq!(output, f64::INFINITY);
        assert_eq!(
            liblrhsmm_rs_log_add_f64(f64::NEG_INFINITY, f64::NEG_INFINITY, &mut output),
            0
        );
        assert_eq!(output, f64::NEG_INFINITY);
        assert_eq!(liblrhsmm_rs_log_add_f64(f64::NAN, 0.0, &mut output), 0);
        assert!(output.is_nan());
        assert_eq!(
            liblrhsmm_rs_negative_exponential_f64(f64::NEG_INFINITY, &mut output),
            0
        );
        assert_eq!(output, 0.0);
        assert_eq!(liblrhsmm_rs_negative_exponential_f64(-0.0, &mut output), 0);
        assert_eq!(output, 1.0);
        assert_eq!(liblrhsmm_rs_exponential_f64(f64::INFINITY, &mut output), 0);
        assert_eq!(output, f64::INFINITY);
        assert_eq!(liblrhsmm_rs_exponential_f64(f64::NAN, &mut output), 0);
        assert!(output.is_nan());
    }
}
