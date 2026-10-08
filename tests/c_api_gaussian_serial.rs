#![cfg(all(feature = "c-api", feature = "serialization"))]
use liblrhsmm_rs::c_api::*;
use liblrhsmm_rs::model::GaussianMixtureParameters;
use std::{
    ffi::c_void,
    ptr::{null, null_mut},
};

struct Input {
    bytes: Vec<u8>,
    position: usize,
    calls: usize,
    interrupt: bool,
    status: u32,
    oversized: bool,
}
unsafe extern "C" fn read(
    context: *mut c_void,
    output: *mut u8,
    capacity: usize,
    count: *mut usize,
) -> u32 {
    // SAFETY: Test context is an exclusive live Input; borrowed buffer/count are valid for this call.
    unsafe {
        let input = &mut *context.cast::<Input>();
        input.calls += 1;
        if input.interrupt {
            input.interrupt = false;
            return LIBLRHSMM_RS_IO_INTERRUPTED;
        }
        if input.status != 0 {
            return input.status;
        }
        if input.oversized {
            count.write(capacity + 1);
            return 0;
        }
        let length = capacity.min(1).min(input.bytes.len() - input.position);
        if length != 0 {
            std::ptr::copy_nonoverlapping(input.bytes.as_ptr().add(input.position), output, length);
        }
        input.position += length;
        count.write(length);
        0
    }
}
struct Output {
    bytes: Vec<u8>,
    calls: usize,
    flushes: usize,
    interrupt: bool,
    status: u32,
    oversized: bool,
    zero: bool,
}
unsafe extern "C" fn write(
    context: *mut c_void,
    input: *const u8,
    capacity: usize,
    count: *mut usize,
) -> u32 {
    // SAFETY: Test context is exclusive live Output; input/count remain valid for this call.
    unsafe {
        let output = &mut *context.cast::<Output>();
        output.calls += 1;
        if output.interrupt {
            output.interrupt = false;
            return LIBLRHSMM_RS_IO_INTERRUPTED;
        }
        if output.status != 0 {
            return output.status;
        }
        if output.oversized {
            count.write(capacity + 1);
            return 0;
        }
        let length = if output.zero { 0 } else { capacity.min(1) };
        if length != 0 {
            output
                .bytes
                .extend_from_slice(std::slice::from_raw_parts(input, length));
        }
        count.write(length);
        0
    }
}
unsafe extern "C" fn flush(context: *mut c_void) -> u32 {
    // SAFETY: Test passes a live exclusive output context.
    unsafe {
        let output = &mut *context.cast::<Output>();
        output.flushes += 1;
        output.status
    }
}
fn input(bytes: &[u8]) -> Input {
    Input {
        bytes: bytes.to_vec(),
        position: 0,
        calls: 0,
        interrupt: true,
        status: 0,
        oversized: false,
    }
}
fn output() -> Output {
    Output {
        bytes: vec![],
        calls: 0,
        flushes: 0,
        interrupt: true,
        status: 0,
        oversized: false,
        zero: false,
    }
}
unsafe fn copy(owner: *const LiblrhsmmRsBytes) -> Vec<u8> {
    // SAFETY: All output storage is independent; supplied owner is live.
    unsafe {
        let mut length = 0;
        assert_eq!(liblrhsmm_rs_bytes_length(owner, &mut length), 0);
        let mut bytes = vec![0; length];
        assert_eq!(
            liblrhsmm_rs_bytes_copy(owner, 0, bytes.as_mut_ptr(), length),
            0
        );
        bytes
    }
}

#[test]
fn gaussian_f32_both_schemas_callbacks_exact_wire_limits_and_failures() {
    // SAFETY: Independent initialized buffers and contexts; each owner uniquely released.
    unsafe {
        for (fixture, encoding, budget) in [
            (
                include_bytes!("fixtures/c-standalone-gmm.bin").as_slice(),
                0,
                18,
            ),
            (
                include_bytes!("fixtures/c-historical-standalone-gmm.bin").as_slice(),
                1,
                29,
            ),
        ] {
            let native = GaussianMixtureParameters::<f32>::read_from(fixture).unwrap();
            let mut owner: *mut LiblrhsmmRsGaussianF32 = null_mut();
            let mut decoded: *mut LiblrhsmmRsGaussianF32 = null_mut();
            let mut bytes = null_mut();
            let limits = LiblrhsmmRsDecodeLimits {
                max_array_entries: budget,
            };
            assert_eq!(
                liblrhsmm_rs_gaussian_read_bytes_f32(
                    fixture.as_ptr(),
                    fixture.len(),
                    &limits,
                    &mut owner
                ),
                0
            );
            for (selector, expected) in [
                &native.weights,
                &native.means,
                &native.variances,
                &native.variance_floors,
            ]
            .into_iter()
            .enumerate()
            {
                let mut values = vec![0.0; expected.len()];
                assert_eq!(
                    liblrhsmm_rs_gaussian_copy_field_f32(
                        owner,
                        selector as u32,
                        0,
                        values.as_mut_ptr(),
                        values.len()
                    ),
                    0
                );
                assert_eq!(
                    values.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                    expected.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
                );
            }
            assert_eq!(
                liblrhsmm_rs_gaussian_write_bytes_f32(owner, encoding, &mut bytes),
                0
            );
            assert_eq!(copy(bytes), fixture);
            assert_eq!(liblrhsmm_rs_bytes_release(&mut bytes), 0);
            let mut incoming = input(fixture);
            let stream = LiblrhsmmRsReadStream {
                context: (&mut incoming as *mut Input).cast(),
                read: Some(read),
            };
            assert_eq!(
                liblrhsmm_rs_gaussian_read_stream_f32(&stream, &limits, &mut decoded),
                0
            );
            assert_eq!(incoming.position, fixture.len());
            assert!(incoming.calls > fixture.len());
            let mut outgoing = output();
            let writer = LiblrhsmmRsWriteStream {
                context: (&mut outgoing as *mut Output).cast(),
                write: Some(write),
                flush: Some(flush),
            };
            assert_eq!(
                liblrhsmm_rs_gaussian_write_stream_f32(decoded, encoding, &writer),
                0
            );
            assert_eq!(outgoing.bytes, fixture);
            assert_eq!(outgoing.flushes, 0);
            assert_eq!(liblrhsmm_rs_write_stream_flush(&writer), 0);
            assert_eq!(outgoing.flushes, 1);
            assert_eq!(liblrhsmm_rs_gaussian_release_f32(&mut decoded), 0);
            incoming = input(&[fixture, fixture, &[42]].concat());
            for index in 1..=2 {
                assert_eq!(
                    liblrhsmm_rs_gaussian_read_stream_prefix_f32(&stream, &limits, &mut decoded),
                    0
                );
                assert_eq!(incoming.position, index * fixture.len());
                assert_eq!(liblrhsmm_rs_gaussian_release_f32(&mut decoded), 0);
            }
            assert_eq!(incoming.bytes[incoming.position], 42);
            for end in 0..fixture.len() {
                assert_eq!(
                    liblrhsmm_rs_gaussian_read_bytes_f32(
                        fixture.as_ptr(),
                        end,
                        &limits,
                        &mut decoded
                    ),
                    3
                );
                assert!(decoded.is_null());
            }
            let trailing = [fixture, &[42]].concat();
            assert_eq!(
                liblrhsmm_rs_gaussian_read_bytes_f32(
                    trailing.as_ptr(),
                    trailing.len(),
                    &limits,
                    &mut decoded
                ),
                3
            );
            let tight = LiblrhsmmRsDecodeLimits {
                max_array_entries: budget - 1,
            };
            assert_eq!(
                liblrhsmm_rs_gaussian_read_bytes_f32(
                    fixture.as_ptr(),
                    fixture.len(),
                    &tight,
                    &mut decoded
                ),
                3
            );
            assert!(decoded.is_null());
            incoming = input(fixture);
            incoming.status = LIBLRHSMM_RS_IO_ERROR;
            assert_eq!(
                liblrhsmm_rs_gaussian_read_stream_f32(&stream, &limits, &mut decoded),
                3
            );
            incoming.status = 0;
            incoming.oversized = true;
            assert_eq!(
                liblrhsmm_rs_gaussian_read_stream_f32(&stream, &limits, &mut decoded),
                3
            );
            incoming.oversized = false;
            incoming.calls = 0;
            assert_eq!(
                liblrhsmm_rs_gaussian_read_stream_f32(&stream, &limits, null_mut()),
                1
            );
            assert_eq!(incoming.calls, 0);
            let missing = LiblrhsmmRsReadStream {
                read: None,
                ..stream
            };
            assert_eq!(
                liblrhsmm_rs_gaussian_read_stream_f32(&missing, &limits, &mut decoded),
                3
            );
            outgoing = output();
            assert!(outgoing.bytes.is_empty());
            outgoing.status = LIBLRHSMM_RS_IO_ERROR;
            assert_eq!(
                liblrhsmm_rs_gaussian_write_stream_f32(owner, encoding, &writer),
                3
            );
            outgoing.status = 0;
            outgoing.oversized = true;
            assert_eq!(
                liblrhsmm_rs_gaussian_write_stream_f32(owner, encoding, &writer),
                3
            );
            outgoing.oversized = false;
            outgoing.zero = true;
            assert_eq!(
                liblrhsmm_rs_gaussian_write_stream_f32(owner, encoding, &writer),
                3
            );
            let missing = LiblrhsmmRsWriteStream {
                write: None,
                ..writer
            };
            assert_eq!(
                liblrhsmm_rs_gaussian_write_stream_f32(owner, encoding, &missing),
                3
            );
            outgoing = output();
            assert_eq!(
                liblrhsmm_rs_gaussian_write_stream_f32(owner, 99, &writer),
                3
            );
            assert_eq!(outgoing.calls, 0);
            assert_eq!(
                liblrhsmm_rs_gaussian_write_bytes_f32(owner, 99, &mut bytes),
                3
            );
            assert!(bytes.is_null());
            if encoding == 0 {
                assert_eq!(liblrhsmm_rs_gaussian_write_stream_f32(owner, 1, &writer), 3);
                assert_eq!(outgoing.calls, 0);
                assert_eq!(
                    liblrhsmm_rs_gaussian_write_bytes_f32(owner, 1, &mut bytes),
                    3
                );
                assert!(bytes.is_null());
            }
            assert_eq!(
                liblrhsmm_rs_gaussian_read_bytes_f32(null(), usize::MAX, &limits, &mut decoded),
                2
            );
            assert_eq!(
                liblrhsmm_rs_gaussian_read_bytes_f32(null(), 0, &limits, &mut decoded),
                3
            );
            assert_eq!(
                liblrhsmm_rs_gaussian_read_bytes_f32(
                    fixture.as_ptr(),
                    fixture.len(),
                    null(),
                    &mut decoded
                ),
                1
            );
            assert_eq!(
                liblrhsmm_rs_gaussian_write_bytes_f32(owner, encoding, null_mut()),
                1
            );
            assert_eq!(
                liblrhsmm_rs_gaussian_write_stream_f32(owner, encoding, null()),
                1
            );
            assert_eq!(liblrhsmm_rs_gaussian_release_f32(&mut owner), 0);
        }
    }
}

#[test]
fn gaussian_f64_both_schemas_callbacks_exact_wire_limits_and_failures() {
    // SAFETY: Independent initialized buffers and contexts; each owner uniquely released.
    unsafe {
        for (fixture, encoding, budget) in [
            (
                include_bytes!("fixtures/c-standalone-gmm.bin").as_slice(),
                0,
                18,
            ),
            (
                include_bytes!("fixtures/c-historical-standalone-gmm.bin").as_slice(),
                1,
                29,
            ),
        ] {
            let native = GaussianMixtureParameters::<f64>::read_from(fixture).unwrap();
            let mut owner: *mut LiblrhsmmRsGaussianF64 = null_mut();
            let mut decoded: *mut LiblrhsmmRsGaussianF64 = null_mut();
            let mut bytes = null_mut();
            let limits = LiblrhsmmRsDecodeLimits {
                max_array_entries: budget,
            };
            assert_eq!(
                liblrhsmm_rs_gaussian_read_bytes_f64(
                    fixture.as_ptr(),
                    fixture.len(),
                    &limits,
                    &mut owner
                ),
                0
            );
            for (selector, expected) in [
                &native.weights,
                &native.means,
                &native.variances,
                &native.variance_floors,
            ]
            .into_iter()
            .enumerate()
            {
                let mut values = vec![0.0; expected.len()];
                assert_eq!(
                    liblrhsmm_rs_gaussian_copy_field_f64(
                        owner,
                        selector as u32,
                        0,
                        values.as_mut_ptr(),
                        values.len()
                    ),
                    0
                );
                assert_eq!(
                    values.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                    expected.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
                );
            }
            assert_eq!(
                liblrhsmm_rs_gaussian_write_bytes_f64(owner, encoding, &mut bytes),
                0
            );
            assert_eq!(copy(bytes), fixture);
            assert_eq!(liblrhsmm_rs_bytes_release(&mut bytes), 0);
            let mut incoming = input(fixture);
            let stream = LiblrhsmmRsReadStream {
                context: (&mut incoming as *mut Input).cast(),
                read: Some(read),
            };
            assert_eq!(
                liblrhsmm_rs_gaussian_read_stream_f64(&stream, &limits, &mut decoded),
                0
            );
            assert_eq!(incoming.position, fixture.len());
            assert!(incoming.calls > fixture.len());
            let mut outgoing = output();
            let writer = LiblrhsmmRsWriteStream {
                context: (&mut outgoing as *mut Output).cast(),
                write: Some(write),
                flush: Some(flush),
            };
            assert_eq!(
                liblrhsmm_rs_gaussian_write_stream_f64(decoded, encoding, &writer),
                0
            );
            assert_eq!(outgoing.bytes, fixture);
            assert_eq!(outgoing.flushes, 0);
            assert_eq!(liblrhsmm_rs_write_stream_flush(&writer), 0);
            assert_eq!(outgoing.flushes, 1);
            assert_eq!(liblrhsmm_rs_gaussian_release_f64(&mut decoded), 0);
            incoming = input(&[fixture, fixture, &[42]].concat());
            for index in 1..=2 {
                assert_eq!(
                    liblrhsmm_rs_gaussian_read_stream_prefix_f64(&stream, &limits, &mut decoded),
                    0
                );
                assert_eq!(incoming.position, index * fixture.len());
                assert_eq!(liblrhsmm_rs_gaussian_release_f64(&mut decoded), 0);
            }
            assert_eq!(incoming.bytes[incoming.position], 42);
            for end in 0..fixture.len() {
                assert_eq!(
                    liblrhsmm_rs_gaussian_read_bytes_f64(
                        fixture.as_ptr(),
                        end,
                        &limits,
                        &mut decoded
                    ),
                    3
                );
                assert!(decoded.is_null());
            }
            let trailing = [fixture, &[42]].concat();
            assert_eq!(
                liblrhsmm_rs_gaussian_read_bytes_f64(
                    trailing.as_ptr(),
                    trailing.len(),
                    &limits,
                    &mut decoded
                ),
                3
            );
            let tight = LiblrhsmmRsDecodeLimits {
                max_array_entries: budget - 1,
            };
            assert_eq!(
                liblrhsmm_rs_gaussian_read_bytes_f64(
                    fixture.as_ptr(),
                    fixture.len(),
                    &tight,
                    &mut decoded
                ),
                3
            );
            assert!(decoded.is_null());
            incoming = input(fixture);
            incoming.status = LIBLRHSMM_RS_IO_ERROR;
            assert_eq!(
                liblrhsmm_rs_gaussian_read_stream_f64(&stream, &limits, &mut decoded),
                3
            );
            incoming.status = 0;
            incoming.oversized = true;
            assert_eq!(
                liblrhsmm_rs_gaussian_read_stream_f64(&stream, &limits, &mut decoded),
                3
            );
            incoming.oversized = false;
            incoming.calls = 0;
            assert_eq!(
                liblrhsmm_rs_gaussian_read_stream_f64(&stream, &limits, null_mut()),
                1
            );
            assert_eq!(incoming.calls, 0);
            let missing = LiblrhsmmRsReadStream {
                read: None,
                ..stream
            };
            assert_eq!(
                liblrhsmm_rs_gaussian_read_stream_f64(&missing, &limits, &mut decoded),
                3
            );
            outgoing = output();
            assert!(outgoing.bytes.is_empty());
            outgoing.status = LIBLRHSMM_RS_IO_ERROR;
            assert_eq!(
                liblrhsmm_rs_gaussian_write_stream_f64(owner, encoding, &writer),
                3
            );
            outgoing.status = 0;
            outgoing.oversized = true;
            assert_eq!(
                liblrhsmm_rs_gaussian_write_stream_f64(owner, encoding, &writer),
                3
            );
            outgoing.oversized = false;
            outgoing.zero = true;
            assert_eq!(
                liblrhsmm_rs_gaussian_write_stream_f64(owner, encoding, &writer),
                3
            );
            let missing = LiblrhsmmRsWriteStream {
                write: None,
                ..writer
            };
            assert_eq!(
                liblrhsmm_rs_gaussian_write_stream_f64(owner, encoding, &missing),
                3
            );
            outgoing = output();
            assert_eq!(
                liblrhsmm_rs_gaussian_write_stream_f64(owner, 99, &writer),
                3
            );
            assert_eq!(outgoing.calls, 0);
            assert_eq!(
                liblrhsmm_rs_gaussian_write_bytes_f64(owner, 99, &mut bytes),
                3
            );
            assert!(bytes.is_null());
            if encoding == 0 {
                assert_eq!(liblrhsmm_rs_gaussian_write_stream_f64(owner, 1, &writer), 3);
                assert_eq!(outgoing.calls, 0);
                assert_eq!(
                    liblrhsmm_rs_gaussian_write_bytes_f64(owner, 1, &mut bytes),
                    3
                );
                assert!(bytes.is_null());
            }
            assert_eq!(
                liblrhsmm_rs_gaussian_read_bytes_f64(null(), usize::MAX, &limits, &mut decoded),
                2
            );
            assert_eq!(
                liblrhsmm_rs_gaussian_read_bytes_f64(null(), 0, &limits, &mut decoded),
                3
            );
            assert_eq!(
                liblrhsmm_rs_gaussian_read_bytes_f64(
                    fixture.as_ptr(),
                    fixture.len(),
                    null(),
                    &mut decoded
                ),
                1
            );
            assert_eq!(
                liblrhsmm_rs_gaussian_write_bytes_f64(owner, encoding, null_mut()),
                1
            );
            assert_eq!(
                liblrhsmm_rs_gaussian_write_stream_f64(owner, encoding, null()),
                1
            );
            assert_eq!(liblrhsmm_rs_gaussian_release_f64(&mut owner), 0);
        }
    }
}

#[test]
fn gaussian_binary64_wire_rounding_and_signed_zero_use_binary32_in_both_encodings() {
    // SAFETY: Independent initialized arrays/output slots; owners released once.
    unsafe {
        let weights = [1.0 + 2.0_f64.powi(-25)];
        let means = [1.0 + 2.0_f64.powi(-25)];
        let variances = [1.0 + 3.0 * 2.0_f64.powi(-25)];
        let floors = [-0.0];
        let input = LiblrhsmmRsGaussianParametersF64 {
            mixtures: 1,
            dimensions: 1,
            weights: weights.as_ptr(),
            means: means.as_ptr(),
            variances: variances.as_ptr(),
            variance_floors: floors.as_ptr(),
        };
        let mut owner = null_mut();
        let mut bytes = null_mut();
        assert_eq!(liblrhsmm_rs_gaussian_create_f64(&input, &mut owner), 0);
        for encoding in [0, 1] {
            assert_eq!(
                liblrhsmm_rs_gaussian_write_bytes_f64(owner, encoding, &mut bytes),
                0
            );
            let mut expected = vec![
                0x94,
                1,
                1,
                0x91,
                0xca,
                0x3f,
                0x80,
                0,
                0,
                if encoding == 0 { 0x93 } else { 0x92 },
                0xca,
                0x3f,
                0x80,
                0,
                0,
                0xca,
                0x3f,
                0x80,
                0,
                1,
            ];
            if encoding == 0 {
                expected.extend([0xca, 0x80, 0, 0, 0]);
            }
            assert_eq!(copy(bytes), expected);
            let decoded = GaussianMixtureParameters::<f64>::read_from(expected.as_slice()).unwrap();
            assert_eq!(decoded.weights, [1.0]);
            assert_eq!(decoded.means, [1.0]);
            assert_eq!(decoded.variances, [f64::from(f32::from_bits(0x3f800001))]);
            assert_eq!(
                decoded.variance_floors[0].to_bits(),
                if encoding == 0 {
                    (-0.0_f64).to_bits()
                } else {
                    0.0_f64.to_bits()
                }
            );
            assert_eq!(liblrhsmm_rs_bytes_release(&mut bytes), 0);
        }
        let mut mean = 0.0;
        assert_eq!(
            liblrhsmm_rs_gaussian_copy_field_f64(owner, 1, 0, &mut mean, 1),
            0
        );
        assert_eq!(mean.to_bits(), means[0].to_bits());
        assert_eq!(liblrhsmm_rs_gaussian_release_f64(&mut owner), 0);
    }
}
