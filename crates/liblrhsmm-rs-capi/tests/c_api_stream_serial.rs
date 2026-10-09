#![cfg(feature = "serialization")]

use liblrhsmm_rs::model::{ModelParameters, StreamParameters};
use liblrhsmm_rs_capi::*;
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

unsafe fn compare_f32(handle: *const LiblrhsmmRsModelF32, expected: &ModelParameters<f32>) {
    // SAFETY: Live readable model and independent outputs; all returned owners are released.
    unsafe {
        let mut layout = LiblrhsmmRsModelLayout {
            streams: 99,
            durations: 99,
        };
        assert_eq!(liblrhsmm_rs_model_layout_f32(handle, &mut layout), 0);
        assert_eq!(
            layout,
            LiblrhsmmRsModelLayout {
                streams: expected.streams.len(),
                durations: expected.durations.len()
            }
        );
        for (index, native) in expected.streams.iter().enumerate() {
            let mut stream = null_mut();
            assert_eq!(
                liblrhsmm_rs_model_get_stream_f32(handle, index, &mut stream),
                0
            );
            let mut shape = LiblrhsmmRsStreamLayoutF32 {
                weight: 99.0,
                emissions: 99,
            };
            assert_eq!(liblrhsmm_rs_stream_layout_f32(stream, &mut shape), 0);
            assert_eq!(shape.weight.to_bits(), native.weight.to_bits());
            assert_eq!(shape.emissions, native.mixtures.len());
            for (emission, expected) in native.mixtures.iter().enumerate() {
                let mut gaussian = null_mut();
                assert_eq!(
                    liblrhsmm_rs_stream_get_emission_f32(stream, emission, &mut gaussian),
                    0
                );
                let mut shape = LiblrhsmmRsGaussianLayout {
                    mixtures: 99,
                    dimensions: 99,
                };
                assert_eq!(liblrhsmm_rs_gaussian_layout_f32(gaussian, &mut shape), 0);
                assert_eq!(shape.mixtures, expected.weights.len());
                assert_eq!(shape.dimensions, expected.dimensions);
                for (selector, values) in [
                    &expected.weights,
                    &expected.means,
                    &expected.variances,
                    &expected.variance_floors,
                ]
                .into_iter()
                .enumerate()
                {
                    let mut actual = vec![f32::MAX; values.len()];
                    assert_eq!(
                        liblrhsmm_rs_gaussian_copy_field_f32(
                            gaussian,
                            selector as u32,
                            0,
                            actual.as_mut_ptr(),
                            actual.len()
                        ),
                        0
                    );
                    assert_eq!(
                        actual.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
                        values.iter().map(|x| x.to_bits()).collect::<Vec<_>>()
                    );
                }
                assert_eq!(liblrhsmm_rs_gaussian_release_f32(&mut gaussian), 0);
            }
            assert_eq!(liblrhsmm_rs_stream_release_f32(&mut stream), 0);
        }
        for (index, native) in expected.durations.iter().enumerate() {
            let mut duration = null_mut();
            assert_eq!(
                liblrhsmm_rs_model_get_duration_f32(handle, index, &mut duration),
                0
            );
            let mut fields = LiblrhsmmRsDurationParametersF32 {
                mean: 99.0,
                variance: 99.0,
                minimum: 99,
                maximum: 99,
                fixed_mean: 99,
                variance_floor: 99.0,
            };
            assert_eq!(liblrhsmm_rs_duration_get_f32(duration, &mut fields), 0);
            assert_eq!(fields.mean.to_bits(), native.mean.to_bits());
            assert_eq!(fields.variance.to_bits(), native.variance.to_bits());
            assert_eq!(fields.minimum, native.minimum);
            assert_eq!(fields.maximum, native.maximum);
            assert_eq!(fields.fixed_mean, native.fixed_mean);
            assert_eq!(
                fields.variance_floor.to_bits(),
                native.variance_floor.to_bits()
            );
            assert_eq!(liblrhsmm_rs_duration_release_f32(&mut duration), 0);
        }
    }
}

unsafe fn compare_f64(handle: *const LiblrhsmmRsModelF64, expected: &ModelParameters<f64>) {
    // SAFETY: Live readable model and independent outputs; all returned owners are released.
    unsafe {
        let mut layout = LiblrhsmmRsModelLayout {
            streams: 99,
            durations: 99,
        };
        assert_eq!(liblrhsmm_rs_model_layout_f64(handle, &mut layout), 0);
        assert_eq!(
            layout,
            LiblrhsmmRsModelLayout {
                streams: expected.streams.len(),
                durations: expected.durations.len()
            }
        );
        for (index, native) in expected.streams.iter().enumerate() {
            let mut stream = null_mut();
            assert_eq!(
                liblrhsmm_rs_model_get_stream_f64(handle, index, &mut stream),
                0
            );
            let mut shape = LiblrhsmmRsStreamLayoutF64 {
                weight: 99.0,
                emissions: 99,
            };
            assert_eq!(liblrhsmm_rs_stream_layout_f64(stream, &mut shape), 0);
            assert_eq!(shape.weight.to_bits(), native.weight.to_bits());
            assert_eq!(shape.emissions, native.mixtures.len());
            for (emission, expected) in native.mixtures.iter().enumerate() {
                let mut gaussian = null_mut();
                assert_eq!(
                    liblrhsmm_rs_stream_get_emission_f64(stream, emission, &mut gaussian),
                    0
                );
                let mut shape = LiblrhsmmRsGaussianLayout {
                    mixtures: 99,
                    dimensions: 99,
                };
                assert_eq!(liblrhsmm_rs_gaussian_layout_f64(gaussian, &mut shape), 0);
                assert_eq!(shape.mixtures, expected.weights.len());
                assert_eq!(shape.dimensions, expected.dimensions);
                for (selector, values) in [
                    &expected.weights,
                    &expected.means,
                    &expected.variances,
                    &expected.variance_floors,
                ]
                .into_iter()
                .enumerate()
                {
                    let mut actual = vec![f64::MAX; values.len()];
                    assert_eq!(
                        liblrhsmm_rs_gaussian_copy_field_f64(
                            gaussian,
                            selector as u32,
                            0,
                            actual.as_mut_ptr(),
                            actual.len()
                        ),
                        0
                    );
                    assert_eq!(
                        actual.iter().map(|x| x.to_bits()).collect::<Vec<_>>(),
                        values.iter().map(|x| x.to_bits()).collect::<Vec<_>>()
                    );
                }
                assert_eq!(liblrhsmm_rs_gaussian_release_f64(&mut gaussian), 0);
            }
            assert_eq!(liblrhsmm_rs_stream_release_f64(&mut stream), 0);
        }
        for (index, native) in expected.durations.iter().enumerate() {
            let mut duration = null_mut();
            assert_eq!(
                liblrhsmm_rs_model_get_duration_f64(handle, index, &mut duration),
                0
            );
            let mut fields = LiblrhsmmRsDurationParametersF64 {
                mean: 99.0,
                variance: 99.0,
                minimum: 99,
                maximum: 99,
                fixed_mean: 99,
                variance_floor: 99.0,
            };
            assert_eq!(liblrhsmm_rs_duration_get_f64(duration, &mut fields), 0);
            assert_eq!(fields.mean.to_bits(), native.mean.to_bits());
            assert_eq!(fields.variance.to_bits(), native.variance.to_bits());
            assert_eq!(fields.minimum, native.minimum);
            assert_eq!(fields.maximum, native.maximum);
            assert_eq!(fields.fixed_mean, native.fixed_mean);
            assert_eq!(
                fields.variance_floor.to_bits(),
                native.variance_floor.to_bits()
            );
            assert_eq!(liblrhsmm_rs_duration_release_f64(&mut duration), 0);
        }
    }
}

#[test]
fn stream_f32_both_schemas_callbacks_exact_wire_limits_and_failures() {
    // SAFETY: Independent initialized buffers and contexts; each owner uniquely released.
    unsafe {
        for (fixture, encoding, budget) in [
            (
                include_bytes!("../../../tests/fixtures/c-standalone-stream.bin").as_slice(),
                0,
                33,
            ),
            (
                include_bytes!("../../../tests/fixtures/c-historical-standalone-stream.bin")
                    .as_slice(),
                1,
                6452,
            ),
        ] {
            let native = StreamParameters::<f32>::read_from(fixture).unwrap();
            let mut owner: *mut LiblrhsmmRsStreamF32 = null_mut();
            let mut decoded: *mut LiblrhsmmRsStreamF32 = null_mut();
            let mut bytes = null_mut();
            let limits = LiblrhsmmRsDecodeLimits {
                max_array_entries: budget,
            };
            assert_eq!(
                liblrhsmm_rs_stream_read_bytes_f32(
                    fixture.as_ptr(),
                    fixture.len(),
                    &limits,
                    &mut owner
                ),
                0
            );
            let children = [owner.cast_const()];
            let mut comparison = null_mut();
            assert_eq!(
                liblrhsmm_rs_model_create_f32(children.as_ptr(), 1, null(), 0, &mut comparison),
                0
            );
            compare_f32(
                comparison,
                &ModelParameters {
                    streams: vec![native.clone()],
                    durations: vec![],
                },
            );
            assert_eq!(liblrhsmm_rs_model_release_f32(&mut comparison), 0);
            assert_eq!(
                liblrhsmm_rs_stream_write_bytes_f32(owner, encoding, &mut bytes),
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
                liblrhsmm_rs_stream_read_stream_f32(&stream, &limits, &mut decoded),
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
                liblrhsmm_rs_stream_write_stream_f32(decoded, encoding, &writer),
                0
            );
            assert_eq!(outgoing.bytes, fixture);
            assert_eq!(outgoing.flushes, 0);
            assert_eq!(liblrhsmm_rs_write_stream_flush(&writer), 0);
            assert_eq!(outgoing.flushes, 1);
            assert_eq!(liblrhsmm_rs_stream_release_f32(&mut decoded), 0);
            incoming = input(&[fixture, fixture, &[42]].concat());
            for index in 1..=2 {
                assert_eq!(
                    liblrhsmm_rs_stream_read_stream_prefix_f32(&stream, &limits, &mut decoded),
                    0
                );
                assert_eq!(incoming.position, index * fixture.len());
                assert_eq!(liblrhsmm_rs_stream_release_f32(&mut decoded), 0);
            }
            assert_eq!(incoming.bytes[incoming.position], 42);
            let ends: Vec<usize> = if fixture.len() <= 512 {
                (0..fixture.len()).collect()
            } else {
                (0..64)
                    .chain((64..fixture.len() - 8).step_by(1024))
                    .chain(fixture.len() - 8..fixture.len())
                    .collect()
            };
            for end in ends {
                assert_eq!(
                    liblrhsmm_rs_stream_read_bytes_f32(
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
                liblrhsmm_rs_stream_read_bytes_f32(
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
                liblrhsmm_rs_stream_read_bytes_f32(
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
                liblrhsmm_rs_stream_read_stream_f32(&stream, &limits, &mut decoded),
                3
            );
            incoming.status = 0;
            incoming.oversized = true;
            assert_eq!(
                liblrhsmm_rs_stream_read_stream_f32(&stream, &limits, &mut decoded),
                3
            );
            incoming.oversized = false;
            incoming.calls = 0;
            assert_eq!(
                liblrhsmm_rs_stream_read_stream_f32(&stream, &limits, null_mut()),
                1
            );
            assert_eq!(incoming.calls, 0);
            let missing = LiblrhsmmRsReadStream {
                read: None,
                ..stream
            };
            assert_eq!(
                liblrhsmm_rs_stream_read_stream_f32(&missing, &limits, &mut decoded),
                3
            );
            outgoing = output();
            assert!(outgoing.bytes.is_empty());
            outgoing.status = LIBLRHSMM_RS_IO_ERROR;
            assert_eq!(
                liblrhsmm_rs_stream_write_stream_f32(owner, encoding, &writer),
                3
            );
            outgoing.status = 0;
            outgoing.oversized = true;
            assert_eq!(
                liblrhsmm_rs_stream_write_stream_f32(owner, encoding, &writer),
                3
            );
            outgoing.oversized = false;
            outgoing.zero = true;
            assert_eq!(
                liblrhsmm_rs_stream_write_stream_f32(owner, encoding, &writer),
                3
            );
            let missing = LiblrhsmmRsWriteStream {
                write: None,
                ..writer
            };
            assert_eq!(
                liblrhsmm_rs_stream_write_stream_f32(owner, encoding, &missing),
                3
            );
            outgoing = output();
            assert_eq!(liblrhsmm_rs_stream_write_stream_f32(owner, 99, &writer), 3);
            assert_eq!(outgoing.calls, 0);
            assert_eq!(
                liblrhsmm_rs_stream_write_bytes_f32(owner, 99, &mut bytes),
                3
            );
            assert!(bytes.is_null());
            if encoding == 0 {
                assert_eq!(liblrhsmm_rs_stream_write_stream_f32(owner, 1, &writer), 3);
                assert_eq!(outgoing.calls, 0);
                assert_eq!(liblrhsmm_rs_stream_write_bytes_f32(owner, 1, &mut bytes), 3);
                assert!(bytes.is_null());
            }
            assert_eq!(
                liblrhsmm_rs_stream_read_bytes_f32(null(), usize::MAX, &limits, &mut decoded),
                2
            );
            assert_eq!(
                liblrhsmm_rs_stream_read_bytes_f32(null(), 0, &limits, &mut decoded),
                3
            );
            assert_eq!(
                liblrhsmm_rs_stream_read_bytes_f32(
                    fixture.as_ptr(),
                    fixture.len(),
                    null(),
                    &mut decoded
                ),
                1
            );
            assert_eq!(
                liblrhsmm_rs_stream_write_bytes_f32(owner, encoding, null_mut()),
                1
            );
            assert_eq!(
                liblrhsmm_rs_stream_write_stream_f32(owner, encoding, null()),
                1
            );
            assert_eq!(liblrhsmm_rs_stream_release_f32(&mut owner), 0);
        }
    }
}

#[test]
fn stream_f64_both_schemas_callbacks_exact_wire_limits_and_failures() {
    // SAFETY: Independent initialized buffers and contexts; each owner uniquely released.
    unsafe {
        for (fixture, encoding, budget) in [
            (
                include_bytes!("../../../tests/fixtures/c-standalone-stream.bin").as_slice(),
                0,
                33,
            ),
            (
                include_bytes!("../../../tests/fixtures/c-historical-standalone-stream.bin")
                    .as_slice(),
                1,
                6452,
            ),
        ] {
            let native = StreamParameters::<f64>::read_from(fixture).unwrap();
            let mut owner: *mut LiblrhsmmRsStreamF64 = null_mut();
            let mut decoded: *mut LiblrhsmmRsStreamF64 = null_mut();
            let mut bytes = null_mut();
            let limits = LiblrhsmmRsDecodeLimits {
                max_array_entries: budget,
            };
            assert_eq!(
                liblrhsmm_rs_stream_read_bytes_f64(
                    fixture.as_ptr(),
                    fixture.len(),
                    &limits,
                    &mut owner
                ),
                0
            );
            let children = [owner.cast_const()];
            let mut comparison = null_mut();
            assert_eq!(
                liblrhsmm_rs_model_create_f64(children.as_ptr(), 1, null(), 0, &mut comparison),
                0
            );
            compare_f64(
                comparison,
                &ModelParameters {
                    streams: vec![native.clone()],
                    durations: vec![],
                },
            );
            assert_eq!(liblrhsmm_rs_model_release_f64(&mut comparison), 0);
            assert_eq!(
                liblrhsmm_rs_stream_write_bytes_f64(owner, encoding, &mut bytes),
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
                liblrhsmm_rs_stream_read_stream_f64(&stream, &limits, &mut decoded),
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
                liblrhsmm_rs_stream_write_stream_f64(decoded, encoding, &writer),
                0
            );
            assert_eq!(outgoing.bytes, fixture);
            assert_eq!(outgoing.flushes, 0);
            assert_eq!(liblrhsmm_rs_write_stream_flush(&writer), 0);
            assert_eq!(outgoing.flushes, 1);
            assert_eq!(liblrhsmm_rs_stream_release_f64(&mut decoded), 0);
            incoming = input(&[fixture, fixture, &[42]].concat());
            for index in 1..=2 {
                assert_eq!(
                    liblrhsmm_rs_stream_read_stream_prefix_f64(&stream, &limits, &mut decoded),
                    0
                );
                assert_eq!(incoming.position, index * fixture.len());
                assert_eq!(liblrhsmm_rs_stream_release_f64(&mut decoded), 0);
            }
            assert_eq!(incoming.bytes[incoming.position], 42);
            let ends: Vec<usize> = if fixture.len() <= 512 {
                (0..fixture.len()).collect()
            } else {
                (0..64)
                    .chain((64..fixture.len() - 8).step_by(1024))
                    .chain(fixture.len() - 8..fixture.len())
                    .collect()
            };
            for end in ends {
                assert_eq!(
                    liblrhsmm_rs_stream_read_bytes_f64(
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
                liblrhsmm_rs_stream_read_bytes_f64(
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
                liblrhsmm_rs_stream_read_bytes_f64(
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
                liblrhsmm_rs_stream_read_stream_f64(&stream, &limits, &mut decoded),
                3
            );
            incoming.status = 0;
            incoming.oversized = true;
            assert_eq!(
                liblrhsmm_rs_stream_read_stream_f64(&stream, &limits, &mut decoded),
                3
            );
            incoming.oversized = false;
            incoming.calls = 0;
            assert_eq!(
                liblrhsmm_rs_stream_read_stream_f64(&stream, &limits, null_mut()),
                1
            );
            assert_eq!(incoming.calls, 0);
            let missing = LiblrhsmmRsReadStream {
                read: None,
                ..stream
            };
            assert_eq!(
                liblrhsmm_rs_stream_read_stream_f64(&missing, &limits, &mut decoded),
                3
            );
            outgoing = output();
            assert!(outgoing.bytes.is_empty());
            outgoing.status = LIBLRHSMM_RS_IO_ERROR;
            assert_eq!(
                liblrhsmm_rs_stream_write_stream_f64(owner, encoding, &writer),
                3
            );
            outgoing.status = 0;
            outgoing.oversized = true;
            assert_eq!(
                liblrhsmm_rs_stream_write_stream_f64(owner, encoding, &writer),
                3
            );
            outgoing.oversized = false;
            outgoing.zero = true;
            assert_eq!(
                liblrhsmm_rs_stream_write_stream_f64(owner, encoding, &writer),
                3
            );
            let missing = LiblrhsmmRsWriteStream {
                write: None,
                ..writer
            };
            assert_eq!(
                liblrhsmm_rs_stream_write_stream_f64(owner, encoding, &missing),
                3
            );
            outgoing = output();
            assert_eq!(liblrhsmm_rs_stream_write_stream_f64(owner, 99, &writer), 3);
            assert_eq!(outgoing.calls, 0);
            assert_eq!(
                liblrhsmm_rs_stream_write_bytes_f64(owner, 99, &mut bytes),
                3
            );
            assert!(bytes.is_null());
            if encoding == 0 {
                assert_eq!(liblrhsmm_rs_stream_write_stream_f64(owner, 1, &writer), 3);
                assert_eq!(outgoing.calls, 0);
                assert_eq!(liblrhsmm_rs_stream_write_bytes_f64(owner, 1, &mut bytes), 3);
                assert!(bytes.is_null());
            }
            assert_eq!(
                liblrhsmm_rs_stream_read_bytes_f64(null(), usize::MAX, &limits, &mut decoded),
                2
            );
            assert_eq!(
                liblrhsmm_rs_stream_read_bytes_f64(null(), 0, &limits, &mut decoded),
                3
            );
            assert_eq!(
                liblrhsmm_rs_stream_read_bytes_f64(
                    fixture.as_ptr(),
                    fixture.len(),
                    null(),
                    &mut decoded
                ),
                1
            );
            assert_eq!(
                liblrhsmm_rs_stream_write_bytes_f64(owner, encoding, null_mut()),
                1
            );
            assert_eq!(
                liblrhsmm_rs_stream_write_stream_f64(owner, encoding, null()),
                1
            );
            assert_eq!(liblrhsmm_rs_stream_release_f64(&mut owner), 0);
        }
    }
}
