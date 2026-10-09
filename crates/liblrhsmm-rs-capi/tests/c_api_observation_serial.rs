#![cfg(feature = "serialization")]

use liblrhsmm_rs::data::ObservationData;
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
    reject_eof: bool,
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
        if input.reject_eof && input.position == input.bytes.len() {
            return LIBLRHSMM_RS_IO_ERROR;
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
    fail_after: Option<usize>,
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
        if output
            .fail_after
            .is_some_and(|limit| output.bytes.len() >= limit)
        {
            return LIBLRHSMM_RS_IO_ERROR;
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
        reject_eof: false,
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
        fail_after: None,
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

unsafe fn compare_f32(owner: *const LiblrhsmmRsObservationF32, expected: &ObservationData<f32>) {
    // SAFETY: Live readable owner and independent initialized caller buffers.
    unsafe {
        let mut layout = LiblrhsmmRsObservationLayout {
            frames: 99,
            streams: 99,
        };
        assert_eq!(liblrhsmm_rs_observation_layout_f32(owner, &mut layout), 0);
        assert_eq!(layout.frames, expected.frames);
        assert_eq!(layout.streams, expected.streams.len());
        for (index, stream) in expected.streams.iter().enumerate() {
            let mut shape = LiblrhsmmRsObservationStreamLayout {
                frames: 99,
                dimensions: 99,
                values: 99,
            };
            assert_eq!(
                liblrhsmm_rs_observation_get_stream_layout_f32(owner, index, &mut shape),
                0
            );
            assert_eq!(
                shape,
                LiblrhsmmRsObservationStreamLayout {
                    frames: expected.frames,
                    dimensions: stream.dimensions,
                    values: stream.values.len()
                }
            );
            let mut values = vec![99.0; stream.values.len()];
            assert_eq!(
                liblrhsmm_rs_observation_copy_values_f32(
                    owner,
                    index,
                    0,
                    values.as_mut_ptr(),
                    values.len()
                ),
                0
            );
            assert_eq!(
                values
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>(),
                stream
                    .values
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>()
            );
        }
    }
}

#[test]
fn observation_f32_original_wire_callbacks_limits_prefix_and_atomic_failures() {
    let fixture = include_bytes!("../../../tests/fixtures/c-observation.bin").as_slice();
    let expected = ObservationData::<f32>::read_from(fixture).unwrap();
    // SAFETY: All borrowed storage is independent; callbacks and live owners remain valid.
    unsafe {
        let mut owner = null_mut();
        let mut bytes = null_mut();
        let mut decoded = null_mut();
        let limits = LiblrhsmmRsDecodeLimits {
            max_array_entries: 16,
        };
        assert_eq!(
            liblrhsmm_rs_observation_read_bytes_f32(
                fixture.as_ptr(),
                fixture.len(),
                &limits,
                &mut owner
            ),
            0
        );
        compare_f32(owner, &expected);
        assert_eq!(
            liblrhsmm_rs_observation_write_bytes_f32(owner, &mut bytes),
            0
        );
        assert_eq!(copy(bytes), fixture);
        assert_eq!(liblrhsmm_rs_bytes_release(&mut bytes), 0);
        let mut incoming = input(fixture);
        let mut source = LiblrhsmmRsReadStream {
            context: (&mut incoming as *mut Input).cast(),
            read: Some(read),
        };
        assert_eq!(
            liblrhsmm_rs_observation_read_stream_f32(&source, &limits, &mut decoded),
            0
        );
        compare_f32(decoded, &expected);
        assert_eq!(incoming.position, fixture.len());
        assert!(incoming.calls > fixture.len());
        let mut outgoing = output();
        let mut sink = LiblrhsmmRsWriteStream {
            context: (&mut outgoing as *mut Output).cast(),
            write: Some(write),
            flush: Some(flush),
        };
        assert_eq!(liblrhsmm_rs_observation_write_stream_f32(decoded, &sink), 0);
        assert_eq!(outgoing.bytes, fixture);
        assert!(outgoing.calls > fixture.len());
        assert_eq!(outgoing.flushes, 0);
        assert_eq!(liblrhsmm_rs_write_stream_flush(&sink), 0);
        assert_eq!(outgoing.flushes, 1);
        assert_eq!(liblrhsmm_rs_observation_release_f32(&mut decoded), 0);
        incoming = input(&[fixture, fixture, &[42]].concat());
        for position in [fixture.len(), fixture.len() * 2] {
            assert_eq!(
                liblrhsmm_rs_observation_read_stream_prefix_f32(&source, &limits, &mut decoded),
                0
            );
            assert_eq!(incoming.position, position);
            compare_f32(decoded, &expected);
            assert_eq!(liblrhsmm_rs_observation_release_f32(&mut decoded), 0);
        }
        assert_eq!(incoming.bytes[incoming.position], 42);
        // Guard the boundary: prefix reading must not issue even an EOF read.
        incoming = input(fixture);
        incoming.reject_eof = true;
        assert_eq!(
            liblrhsmm_rs_observation_read_stream_prefix_f32(&source, &limits, &mut decoded),
            0
        );
        assert_eq!(incoming.position, fixture.len());
        assert_eq!(liblrhsmm_rs_observation_release_f32(&mut decoded), 0);
        incoming.position = 0;
        assert_eq!(
            liblrhsmm_rs_observation_read_stream_f32(&source, &limits, &mut decoded),
            3
        );
        assert!(decoded.is_null());
        for length in 0..fixture.len() {
            let mut sentinel = owner;
            assert_eq!(
                liblrhsmm_rs_observation_read_bytes_f32(
                    fixture.as_ptr(),
                    length,
                    &limits,
                    &mut sentinel
                ),
                3
            );
            assert_eq!(sentinel, owner);
        }
        for malformed in [[fixture, &[0]].concat(), {
            let mut bytes = fixture.to_vec();
            bytes[0] = 0x92;
            bytes
        }] {
            let mut sentinel = owner;
            assert_eq!(
                liblrhsmm_rs_observation_read_bytes_f32(
                    malformed.as_ptr(),
                    malformed.len(),
                    &limits,
                    &mut sentinel
                ),
                3
            );
            assert_eq!(sentinel, owner);
        }
        for budget in 0..16 {
            let mut sentinel = owner;
            let tight = LiblrhsmmRsDecodeLimits {
                max_array_entries: budget,
            };
            assert_eq!(
                liblrhsmm_rs_observation_read_bytes_f32(
                    fixture.as_ptr(),
                    fixture.len(),
                    &tight,
                    &mut sentinel
                ),
                3
            );
            assert_eq!(sentinel, owner);
        }
        let mut sentinel = owner;
        assert_eq!(
            liblrhsmm_rs_observation_read_bytes_f32(null(), usize::MAX, &limits, &mut sentinel),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_read_bytes_f32(null(), 1, &limits, &mut sentinel),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_read_bytes_f32(null(), 0, &limits, &mut sentinel),
            3
        );
        assert_eq!(
            liblrhsmm_rs_observation_read_bytes_f32(
                fixture.as_ptr(),
                fixture.len(),
                null(),
                &mut sentinel
            ),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_read_bytes_f32(
                fixture.as_ptr(),
                fixture.len(),
                &limits,
                null_mut()
            ),
            1
        );
        assert_eq!(sentinel, owner);
        incoming = input(fixture);
        incoming.status = LIBLRHSMM_RS_IO_ERROR;
        assert_eq!(
            liblrhsmm_rs_observation_read_stream_f32(&source, &limits, &mut sentinel),
            3
        );
        assert_eq!(sentinel, owner);
        incoming.status = 0;
        incoming.oversized = true;
        assert_eq!(
            liblrhsmm_rs_observation_read_stream_f32(&source, &limits, &mut sentinel),
            3
        );
        assert_eq!(sentinel, owner);
        source.read = None;
        assert_eq!(
            liblrhsmm_rs_observation_read_stream_f32(&source, &limits, &mut sentinel),
            3
        );
        assert_eq!(
            liblrhsmm_rs_observation_read_stream_prefix_f32(&source, &limits, &mut sentinel),
            3
        );
        source.read = Some(read);
        let calls = incoming.calls;
        assert_eq!(
            liblrhsmm_rs_observation_read_stream_f32(&source, &limits, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_read_stream_prefix_f32(&source, &limits, null_mut()),
            1
        );
        assert_eq!(incoming.calls, calls);
        assert_eq!(
            liblrhsmm_rs_observation_read_stream_f32(null(), &limits, &mut sentinel),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_read_stream_prefix_f32(&source, null(), &mut sentinel),
            1
        );
        for mode in 0..3 {
            outgoing = output();
            outgoing.interrupt = false;
            outgoing.status = if mode == 0 { LIBLRHSMM_RS_IO_ERROR } else { 0 };
            outgoing.oversized = mode == 1;
            outgoing.zero = mode == 2;
            assert_eq!(liblrhsmm_rs_observation_write_stream_f32(owner, &sink), 3);
            assert_eq!(outgoing.calls, 1);
            assert!(outgoing.bytes.is_empty());
        }
        outgoing = output();
        outgoing.fail_after = Some(7);
        assert_eq!(liblrhsmm_rs_observation_write_stream_f32(owner, &sink), 3);
        assert_eq!(outgoing.bytes, fixture[..7]);
        assert_eq!(outgoing.flushes, 0);
        sink.write = None;
        assert_eq!(liblrhsmm_rs_observation_write_stream_f32(owner, &sink), 3);
        assert_eq!(liblrhsmm_rs_observation_write_stream_f32(owner, null()), 1);
        assert_eq!(liblrhsmm_rs_observation_write_stream_f32(null(), &sink), 1);
        assert_eq!(
            liblrhsmm_rs_observation_write_bytes_f32(owner, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_write_bytes_f32(null(), &mut bytes),
            1
        );
        assert!(bytes.is_null());
        compare_f32(owner, &expected);
        assert_eq!(liblrhsmm_rs_observation_release_f32(&mut owner), 0);
    }
}

unsafe fn compare_f64(owner: *const LiblrhsmmRsObservationF64, expected: &ObservationData<f64>) {
    // SAFETY: Live readable owner and independent initialized caller buffers.
    unsafe {
        let mut layout = LiblrhsmmRsObservationLayout {
            frames: 99,
            streams: 99,
        };
        assert_eq!(liblrhsmm_rs_observation_layout_f64(owner, &mut layout), 0);
        assert_eq!(layout.frames, expected.frames);
        assert_eq!(layout.streams, expected.streams.len());
        for (index, stream) in expected.streams.iter().enumerate() {
            let mut shape = LiblrhsmmRsObservationStreamLayout {
                frames: 99,
                dimensions: 99,
                values: 99,
            };
            assert_eq!(
                liblrhsmm_rs_observation_get_stream_layout_f64(owner, index, &mut shape),
                0
            );
            assert_eq!(
                shape,
                LiblrhsmmRsObservationStreamLayout {
                    frames: expected.frames,
                    dimensions: stream.dimensions,
                    values: stream.values.len()
                }
            );
            let mut values = vec![99.0; stream.values.len()];
            assert_eq!(
                liblrhsmm_rs_observation_copy_values_f64(
                    owner,
                    index,
                    0,
                    values.as_mut_ptr(),
                    values.len()
                ),
                0
            );
            assert_eq!(
                values
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>(),
                stream
                    .values
                    .iter()
                    .map(|value| value.to_bits())
                    .collect::<Vec<_>>()
            );
        }
    }
}

#[test]
fn observation_f64_original_wire_callbacks_limits_prefix_and_atomic_failures() {
    let fixture = include_bytes!("../../../tests/fixtures/c-observation.bin").as_slice();
    let expected = ObservationData::<f64>::read_from(fixture).unwrap();
    // SAFETY: All borrowed storage is independent; callbacks and live owners remain valid.
    unsafe {
        let mut owner = null_mut();
        let mut bytes = null_mut();
        let mut decoded = null_mut();
        let limits = LiblrhsmmRsDecodeLimits {
            max_array_entries: 16,
        };
        assert_eq!(
            liblrhsmm_rs_observation_read_bytes_f64(
                fixture.as_ptr(),
                fixture.len(),
                &limits,
                &mut owner
            ),
            0
        );
        compare_f64(owner, &expected);
        assert_eq!(
            liblrhsmm_rs_observation_write_bytes_f64(owner, &mut bytes),
            0
        );
        assert_eq!(copy(bytes), fixture);
        assert_eq!(liblrhsmm_rs_bytes_release(&mut bytes), 0);
        let mut incoming = input(fixture);
        let mut source = LiblrhsmmRsReadStream {
            context: (&mut incoming as *mut Input).cast(),
            read: Some(read),
        };
        assert_eq!(
            liblrhsmm_rs_observation_read_stream_f64(&source, &limits, &mut decoded),
            0
        );
        compare_f64(decoded, &expected);
        assert_eq!(incoming.position, fixture.len());
        assert!(incoming.calls > fixture.len());
        let mut outgoing = output();
        let mut sink = LiblrhsmmRsWriteStream {
            context: (&mut outgoing as *mut Output).cast(),
            write: Some(write),
            flush: Some(flush),
        };
        assert_eq!(liblrhsmm_rs_observation_write_stream_f64(decoded, &sink), 0);
        assert_eq!(outgoing.bytes, fixture);
        assert!(outgoing.calls > fixture.len());
        assert_eq!(outgoing.flushes, 0);
        assert_eq!(liblrhsmm_rs_write_stream_flush(&sink), 0);
        assert_eq!(outgoing.flushes, 1);
        assert_eq!(liblrhsmm_rs_observation_release_f64(&mut decoded), 0);
        incoming = input(&[fixture, fixture, &[42]].concat());
        for position in [fixture.len(), fixture.len() * 2] {
            assert_eq!(
                liblrhsmm_rs_observation_read_stream_prefix_f64(&source, &limits, &mut decoded),
                0
            );
            assert_eq!(incoming.position, position);
            compare_f64(decoded, &expected);
            assert_eq!(liblrhsmm_rs_observation_release_f64(&mut decoded), 0);
        }
        assert_eq!(incoming.bytes[incoming.position], 42);
        // Guard the boundary: prefix reading must not issue even an EOF read.
        incoming = input(fixture);
        incoming.reject_eof = true;
        assert_eq!(
            liblrhsmm_rs_observation_read_stream_prefix_f64(&source, &limits, &mut decoded),
            0
        );
        assert_eq!(incoming.position, fixture.len());
        assert_eq!(liblrhsmm_rs_observation_release_f64(&mut decoded), 0);
        incoming.position = 0;
        assert_eq!(
            liblrhsmm_rs_observation_read_stream_f64(&source, &limits, &mut decoded),
            3
        );
        assert!(decoded.is_null());
        for length in 0..fixture.len() {
            let mut sentinel = owner;
            assert_eq!(
                liblrhsmm_rs_observation_read_bytes_f64(
                    fixture.as_ptr(),
                    length,
                    &limits,
                    &mut sentinel
                ),
                3
            );
            assert_eq!(sentinel, owner);
        }
        for malformed in [[fixture, &[0]].concat(), {
            let mut bytes = fixture.to_vec();
            bytes[0] = 0x92;
            bytes
        }] {
            let mut sentinel = owner;
            assert_eq!(
                liblrhsmm_rs_observation_read_bytes_f64(
                    malformed.as_ptr(),
                    malformed.len(),
                    &limits,
                    &mut sentinel
                ),
                3
            );
            assert_eq!(sentinel, owner);
        }
        for budget in 0..16 {
            let mut sentinel = owner;
            let tight = LiblrhsmmRsDecodeLimits {
                max_array_entries: budget,
            };
            assert_eq!(
                liblrhsmm_rs_observation_read_bytes_f64(
                    fixture.as_ptr(),
                    fixture.len(),
                    &tight,
                    &mut sentinel
                ),
                3
            );
            assert_eq!(sentinel, owner);
        }
        let mut sentinel = owner;
        assert_eq!(
            liblrhsmm_rs_observation_read_bytes_f64(null(), usize::MAX, &limits, &mut sentinel),
            2
        );
        assert_eq!(
            liblrhsmm_rs_observation_read_bytes_f64(null(), 1, &limits, &mut sentinel),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_read_bytes_f64(null(), 0, &limits, &mut sentinel),
            3
        );
        assert_eq!(
            liblrhsmm_rs_observation_read_bytes_f64(
                fixture.as_ptr(),
                fixture.len(),
                null(),
                &mut sentinel
            ),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_read_bytes_f64(
                fixture.as_ptr(),
                fixture.len(),
                &limits,
                null_mut()
            ),
            1
        );
        assert_eq!(sentinel, owner);
        incoming = input(fixture);
        incoming.status = LIBLRHSMM_RS_IO_ERROR;
        assert_eq!(
            liblrhsmm_rs_observation_read_stream_f64(&source, &limits, &mut sentinel),
            3
        );
        assert_eq!(sentinel, owner);
        incoming.status = 0;
        incoming.oversized = true;
        assert_eq!(
            liblrhsmm_rs_observation_read_stream_f64(&source, &limits, &mut sentinel),
            3
        );
        assert_eq!(sentinel, owner);
        source.read = None;
        assert_eq!(
            liblrhsmm_rs_observation_read_stream_f64(&source, &limits, &mut sentinel),
            3
        );
        assert_eq!(
            liblrhsmm_rs_observation_read_stream_prefix_f64(&source, &limits, &mut sentinel),
            3
        );
        source.read = Some(read);
        let calls = incoming.calls;
        assert_eq!(
            liblrhsmm_rs_observation_read_stream_f64(&source, &limits, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_read_stream_prefix_f64(&source, &limits, null_mut()),
            1
        );
        assert_eq!(incoming.calls, calls);
        assert_eq!(
            liblrhsmm_rs_observation_read_stream_f64(null(), &limits, &mut sentinel),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_read_stream_prefix_f64(&source, null(), &mut sentinel),
            1
        );
        for mode in 0..3 {
            outgoing = output();
            outgoing.interrupt = false;
            outgoing.status = if mode == 0 { LIBLRHSMM_RS_IO_ERROR } else { 0 };
            outgoing.oversized = mode == 1;
            outgoing.zero = mode == 2;
            assert_eq!(liblrhsmm_rs_observation_write_stream_f64(owner, &sink), 3);
            assert_eq!(outgoing.calls, 1);
            assert!(outgoing.bytes.is_empty());
        }
        outgoing = output();
        outgoing.fail_after = Some(7);
        assert_eq!(liblrhsmm_rs_observation_write_stream_f64(owner, &sink), 3);
        assert_eq!(outgoing.bytes, fixture[..7]);
        assert_eq!(outgoing.flushes, 0);
        sink.write = None;
        assert_eq!(liblrhsmm_rs_observation_write_stream_f64(owner, &sink), 3);
        assert_eq!(liblrhsmm_rs_observation_write_stream_f64(owner, null()), 1);
        assert_eq!(liblrhsmm_rs_observation_write_stream_f64(null(), &sink), 1);
        assert_eq!(
            liblrhsmm_rs_observation_write_bytes_f64(owner, null_mut()),
            1
        );
        assert_eq!(
            liblrhsmm_rs_observation_write_bytes_f64(null(), &mut bytes),
            1
        );
        assert!(bytes.is_null());
        compare_f64(owner, &expected);
        assert_eq!(liblrhsmm_rs_observation_release_f64(&mut owner), 0);
    }
}
