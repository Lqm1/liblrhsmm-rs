#![cfg(feature = "serialization")]

use liblrhsmm_rs::data::{ObservationData, SegmentationData};
use liblrhsmm_rs::serial;
use liblrhsmm_rs_capi::*;
use std::{
    ffi::c_void,
    ptr::{null, null_mut},
};
unsafe fn compare_observation_f32(
    owner: *const LiblrhsmmRsObservationF32,
    expected: &ObservationData<f32>,
) {
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

unsafe fn compare_observation_f64(
    owner: *const LiblrhsmmRsObservationF64,
    expected: &ObservationData<f64>,
) {
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

unsafe fn compare_segmentation_f32(
    owner: *const LiblrhsmmRsSegmentationF32,
    expected: &SegmentationData<f32>,
) {
    // SAFETY: Live readable owner and independent initialized caller buffers.
    unsafe {
        let mut layout = LiblrhsmmRsSegmentationLayout {
            streams: 99,
            segments: 99,
        };
        assert_eq!(liblrhsmm_rs_segmentation_layout_f32(owner, &mut layout), 0);
        assert_eq!(layout.streams, expected.output_states.len());
        assert_eq!(layout.segments, expected.boundaries.len());
        let mut values = vec![99; layout.segments];
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_boundaries_f32(
                owner,
                0,
                values.as_mut_ptr(),
                values.len()
            ),
            0
        );
        assert_eq!(values, expected.boundaries);
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_duration_states_f32(
                owner,
                0,
                values.as_mut_ptr(),
                values.len()
            ),
            0
        );
        assert_eq!(values, expected.duration_states);
        for (stream, states) in expected.output_states.iter().enumerate() {
            assert_eq!(
                liblrhsmm_rs_segmentation_copy_output_states_f32(
                    owner,
                    stream,
                    0,
                    values.as_mut_ptr(),
                    values.len()
                ),
                0
            );
            assert_eq!(&values, states);
        }
        for (state, expected) in expected.outgoing.iter().enumerate() {
            let mut count = 99;
            assert_eq!(
                liblrhsmm_rs_segmentation_outgoing_count_f32(owner, state, &mut count),
                0
            );
            assert_eq!(count, expected.len());
            let mut jumps = vec![
                LiblrhsmmRsJumpF32 {
                    delta: 99,
                    probability: 99.0
                };
                count
            ];
            assert_eq!(
                liblrhsmm_rs_segmentation_copy_outgoing_f32(
                    owner,
                    state,
                    0,
                    jumps.as_mut_ptr(),
                    count
                ),
                0
            );
            for (actual, expected) in jumps.iter().zip(expected) {
                assert_eq!(actual.delta, expected.delta);
                assert_eq!(actual.probability.to_bits(), expected.probability.to_bits());
            }
        }
    }
}

unsafe fn compare_segmentation_f64(
    owner: *const LiblrhsmmRsSegmentationF64,
    expected: &SegmentationData<f64>,
) {
    // SAFETY: Live readable owner and independent initialized caller buffers.
    unsafe {
        let mut layout = LiblrhsmmRsSegmentationLayout {
            streams: 99,
            segments: 99,
        };
        assert_eq!(liblrhsmm_rs_segmentation_layout_f64(owner, &mut layout), 0);
        assert_eq!(layout.streams, expected.output_states.len());
        assert_eq!(layout.segments, expected.boundaries.len());
        let mut values = vec![99; layout.segments];
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_boundaries_f64(
                owner,
                0,
                values.as_mut_ptr(),
                values.len()
            ),
            0
        );
        assert_eq!(values, expected.boundaries);
        assert_eq!(
            liblrhsmm_rs_segmentation_copy_duration_states_f64(
                owner,
                0,
                values.as_mut_ptr(),
                values.len()
            ),
            0
        );
        assert_eq!(values, expected.duration_states);
        for (stream, states) in expected.output_states.iter().enumerate() {
            assert_eq!(
                liblrhsmm_rs_segmentation_copy_output_states_f64(
                    owner,
                    stream,
                    0,
                    values.as_mut_ptr(),
                    values.len()
                ),
                0
            );
            assert_eq!(&values, states);
        }
        for (state, expected) in expected.outgoing.iter().enumerate() {
            let mut count = 99;
            assert_eq!(
                liblrhsmm_rs_segmentation_outgoing_count_f64(owner, state, &mut count),
                0
            );
            assert_eq!(count, expected.len());
            let mut jumps = vec![
                LiblrhsmmRsJumpF64 {
                    delta: 99,
                    probability: 99.0
                };
                count
            ];
            assert_eq!(
                liblrhsmm_rs_segmentation_copy_outgoing_f64(
                    owner,
                    state,
                    0,
                    jumps.as_mut_ptr(),
                    count
                ),
                0
            );
            for (actual, expected) in jumps.iter().zip(expected) {
                assert_eq!(actual.delta, expected.delta);
                assert_eq!(actual.probability.to_bits(), expected.probability.to_bits());
            }
        }
    }
}

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

unsafe fn compare_set_observation_f32(
    owner: *const LiblrhsmmRsObservationSetF32,
    expected: &[ObservationData<f32>],
) {
    // SAFETY: Complete live set and independent child outputs, released uniquely.
    unsafe {
        let mut length = 99;
        assert_eq!(
            liblrhsmm_rs_observation_set_length_f32(owner, &mut length),
            0
        );
        assert_eq!(length, expected.len());
        for (index, sample) in expected.iter().enumerate() {
            let mut child = null_mut();
            assert_eq!(
                liblrhsmm_rs_observation_set_get_f32(owner, index, &mut child),
                0
            );
            compare_observation_f32(child, sample);
            assert_eq!(liblrhsmm_rs_observation_release_f32(&mut child), 0);
        }
    }
}
#[test]
fn observation_set_f32_original_wire_callbacks_limits_prefix_and_atomic_failures() {
    let mut fixtures =
        vec![include_bytes!("../../../tests/fixtures/c-observation-set.bin").to_vec()];
    let mut selected =
        serial::read_observation_set_with_precision::<f32>(fixtures[0].as_slice()).unwrap();
    selected[0].streams[0].values[0] = 0.25 + f32::EPSILON;
    selected[0].streams[0].values[1] = f32::from_bits(1);
    let mut wire = Vec::new();
    serial::write_observation_set_with_precision(&selected, &mut wire).unwrap();
    fixtures.push(wire);
    fixtures.push(vec![0x90]);
    for fixture in fixtures {
        let fixture = fixture.as_slice();
        let expected = serial::read_observation_set_with_precision::<f32>(fixture).unwrap();
        let required = expected.len()
            + expected
                .iter()
                .map(|sample| {
                    3 + 2 * sample.streams.len()
                        + sample
                            .streams
                            .iter()
                            .map(|stream| stream.values.len())
                            .sum::<usize>()
                })
                .sum::<usize>();
        // SAFETY: All borrowed storage is independent; callbacks and live owners remain valid.
        unsafe {
            let mut owner = null_mut();
            let mut bytes = null_mut();
            let mut decoded = null_mut();
            let limits = LiblrhsmmRsDecodeLimits {
                max_array_entries: required,
            };
            assert_eq!(
                liblrhsmm_rs_observation_set_read_bytes_f32(
                    fixture.as_ptr(),
                    fixture.len(),
                    &limits,
                    &mut owner
                ),
                0
            );
            compare_set_observation_f32(owner, &expected);
            assert_eq!(
                liblrhsmm_rs_observation_set_write_bytes_f32(owner, &mut bytes),
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
                liblrhsmm_rs_observation_set_read_stream_f32(&source, &limits, &mut decoded),
                0
            );
            compare_set_observation_f32(decoded, &expected);
            assert_eq!(incoming.position, fixture.len());
            assert!(incoming.calls > fixture.len());
            let mut outgoing = output();
            let mut sink = LiblrhsmmRsWriteStream {
                context: (&mut outgoing as *mut Output).cast(),
                write: Some(write),
                flush: Some(flush),
            };
            assert_eq!(
                liblrhsmm_rs_observation_set_write_stream_f32(decoded, &sink),
                0
            );
            assert_eq!(outgoing.bytes, fixture);
            assert!(outgoing.calls > fixture.len());
            assert_eq!(outgoing.flushes, 0);
            assert_eq!(liblrhsmm_rs_write_stream_flush(&sink), 0);
            assert_eq!(outgoing.flushes, 1);
            assert_eq!(liblrhsmm_rs_observation_set_release_f32(&mut decoded), 0);
            incoming = input(&[fixture, fixture, &[42]].concat());
            for position in [fixture.len(), fixture.len() * 2] {
                assert_eq!(
                    liblrhsmm_rs_observation_set_read_stream_prefix_f32(
                        &source,
                        &limits,
                        &mut decoded
                    ),
                    0
                );
                assert_eq!(incoming.position, position);
                compare_set_observation_f32(decoded, &expected);
                assert_eq!(liblrhsmm_rs_observation_set_release_f32(&mut decoded), 0);
            }
            assert_eq!(incoming.bytes[incoming.position], 42);
            // Guard the boundary: prefix reading must not issue even an EOF read.
            incoming = input(fixture);
            incoming.reject_eof = true;
            assert_eq!(
                liblrhsmm_rs_observation_set_read_stream_prefix_f32(&source, &limits, &mut decoded),
                0
            );
            assert_eq!(incoming.position, fixture.len());
            assert_eq!(liblrhsmm_rs_observation_set_release_f32(&mut decoded), 0);
            incoming.position = 0;
            assert_eq!(
                liblrhsmm_rs_observation_set_read_stream_f32(&source, &limits, &mut decoded),
                3
            );
            assert!(decoded.is_null());
            for length in 0..fixture.len() {
                let mut sentinel = owner;
                assert_eq!(
                    liblrhsmm_rs_observation_set_read_bytes_f32(
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
                bytes[0] = 0xc0;
                bytes
            }] {
                let mut sentinel = owner;
                assert_eq!(
                    liblrhsmm_rs_observation_set_read_bytes_f32(
                        malformed.as_ptr(),
                        malformed.len(),
                        &limits,
                        &mut sentinel
                    ),
                    3
                );
                assert_eq!(sentinel, owner);
            }
            for budget in 0..required {
                let mut sentinel = owner;
                let tight = LiblrhsmmRsDecodeLimits {
                    max_array_entries: budget,
                };
                assert_eq!(
                    liblrhsmm_rs_observation_set_read_bytes_f32(
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
                liblrhsmm_rs_observation_set_read_bytes_f32(
                    null(),
                    usize::MAX,
                    &limits,
                    &mut sentinel
                ),
                2
            );
            assert_eq!(
                liblrhsmm_rs_observation_set_read_bytes_f32(null(), 1, &limits, &mut sentinel),
                1
            );
            assert_eq!(
                liblrhsmm_rs_observation_set_read_bytes_f32(null(), 0, &limits, &mut sentinel),
                3
            );
            assert_eq!(
                liblrhsmm_rs_observation_set_read_bytes_f32(
                    fixture.as_ptr(),
                    fixture.len(),
                    null(),
                    &mut sentinel
                ),
                1
            );
            assert_eq!(
                liblrhsmm_rs_observation_set_read_bytes_f32(
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
                liblrhsmm_rs_observation_set_read_stream_f32(&source, &limits, &mut sentinel),
                3
            );
            assert_eq!(sentinel, owner);
            incoming.status = 0;
            incoming.oversized = true;
            assert_eq!(
                liblrhsmm_rs_observation_set_read_stream_f32(&source, &limits, &mut sentinel),
                3
            );
            assert_eq!(sentinel, owner);
            source.read = None;
            assert_eq!(
                liblrhsmm_rs_observation_set_read_stream_f32(&source, &limits, &mut sentinel),
                3
            );
            assert_eq!(
                liblrhsmm_rs_observation_set_read_stream_prefix_f32(
                    &source,
                    &limits,
                    &mut sentinel
                ),
                3
            );
            source.read = Some(read);
            let calls = incoming.calls;
            assert_eq!(
                liblrhsmm_rs_observation_set_read_stream_f32(&source, &limits, null_mut()),
                1
            );
            assert_eq!(
                liblrhsmm_rs_observation_set_read_stream_prefix_f32(&source, &limits, null_mut()),
                1
            );
            assert_eq!(incoming.calls, calls);
            assert_eq!(
                liblrhsmm_rs_observation_set_read_stream_f32(null(), &limits, &mut sentinel),
                1
            );
            assert_eq!(
                liblrhsmm_rs_observation_set_read_stream_prefix_f32(&source, null(), &mut sentinel),
                1
            );
            for mode in 0..3 {
                outgoing = output();
                outgoing.interrupt = false;
                outgoing.status = if mode == 0 { LIBLRHSMM_RS_IO_ERROR } else { 0 };
                outgoing.oversized = mode == 1;
                outgoing.zero = mode == 2;
                assert_eq!(
                    liblrhsmm_rs_observation_set_write_stream_f32(owner, &sink),
                    3
                );
                assert_eq!(outgoing.calls, 1);
                assert!(outgoing.bytes.is_empty());
            }
            if fixture.len() > 7 {
                outgoing = output();
                outgoing.fail_after = Some(7);
                assert_eq!(
                    liblrhsmm_rs_observation_set_write_stream_f32(owner, &sink),
                    3
                );
                assert_eq!(outgoing.bytes, fixture[..7]);
                assert_eq!(outgoing.flushes, 0);
            }
            sink.write = None;
            assert_eq!(
                liblrhsmm_rs_observation_set_write_stream_f32(owner, &sink),
                3
            );
            assert_eq!(
                liblrhsmm_rs_observation_set_write_stream_f32(owner, null()),
                1
            );
            assert_eq!(
                liblrhsmm_rs_observation_set_write_stream_f32(null(), &sink),
                1
            );
            assert_eq!(
                liblrhsmm_rs_observation_set_write_bytes_f32(owner, null_mut()),
                1
            );
            assert_eq!(
                liblrhsmm_rs_observation_set_write_bytes_f32(null(), &mut bytes),
                1
            );
            assert!(bytes.is_null());
            compare_set_observation_f32(owner, &expected);
            assert_eq!(liblrhsmm_rs_observation_set_release_f32(&mut owner), 0);
        }
    }
}

unsafe fn compare_set_observation_f64(
    owner: *const LiblrhsmmRsObservationSetF64,
    expected: &[ObservationData<f64>],
) {
    // SAFETY: Complete live set and independent child outputs, released uniquely.
    unsafe {
        let mut length = 99;
        assert_eq!(
            liblrhsmm_rs_observation_set_length_f64(owner, &mut length),
            0
        );
        assert_eq!(length, expected.len());
        for (index, sample) in expected.iter().enumerate() {
            let mut child = null_mut();
            assert_eq!(
                liblrhsmm_rs_observation_set_get_f64(owner, index, &mut child),
                0
            );
            compare_observation_f64(child, sample);
            assert_eq!(liblrhsmm_rs_observation_release_f64(&mut child), 0);
        }
    }
}
#[test]
fn observation_set_f64_original_wire_callbacks_limits_prefix_and_atomic_failures() {
    let mut fixtures =
        vec![include_bytes!("../../../tests/fixtures/c-observation-set.bin").to_vec()];
    let mut selected =
        serial::read_observation_set_with_precision::<f64>(fixtures[0].as_slice()).unwrap();
    selected[0].streams[0].values[0] = 0.25 + f64::EPSILON;
    selected[0].streams[0].values[1] = f64::from(f32::from_bits(1));
    let mut wire = Vec::new();
    serial::write_observation_set_with_precision(&selected, &mut wire).unwrap();
    fixtures.push(wire);
    fixtures.push(vec![0x90]);
    for fixture in fixtures {
        let fixture = fixture.as_slice();
        let expected = serial::read_observation_set_with_precision::<f64>(fixture).unwrap();
        let required = expected.len()
            + expected
                .iter()
                .map(|sample| {
                    3 + 2 * sample.streams.len()
                        + sample
                            .streams
                            .iter()
                            .map(|stream| stream.values.len())
                            .sum::<usize>()
                })
                .sum::<usize>();
        // SAFETY: All borrowed storage is independent; callbacks and live owners remain valid.
        unsafe {
            let mut owner = null_mut();
            let mut bytes = null_mut();
            let mut decoded = null_mut();
            let limits = LiblrhsmmRsDecodeLimits {
                max_array_entries: required,
            };
            assert_eq!(
                liblrhsmm_rs_observation_set_read_bytes_f64(
                    fixture.as_ptr(),
                    fixture.len(),
                    &limits,
                    &mut owner
                ),
                0
            );
            compare_set_observation_f64(owner, &expected);
            assert_eq!(
                liblrhsmm_rs_observation_set_write_bytes_f64(owner, &mut bytes),
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
                liblrhsmm_rs_observation_set_read_stream_f64(&source, &limits, &mut decoded),
                0
            );
            compare_set_observation_f64(decoded, &expected);
            assert_eq!(incoming.position, fixture.len());
            assert!(incoming.calls > fixture.len());
            let mut outgoing = output();
            let mut sink = LiblrhsmmRsWriteStream {
                context: (&mut outgoing as *mut Output).cast(),
                write: Some(write),
                flush: Some(flush),
            };
            assert_eq!(
                liblrhsmm_rs_observation_set_write_stream_f64(decoded, &sink),
                0
            );
            assert_eq!(outgoing.bytes, fixture);
            assert!(outgoing.calls > fixture.len());
            assert_eq!(outgoing.flushes, 0);
            assert_eq!(liblrhsmm_rs_write_stream_flush(&sink), 0);
            assert_eq!(outgoing.flushes, 1);
            assert_eq!(liblrhsmm_rs_observation_set_release_f64(&mut decoded), 0);
            incoming = input(&[fixture, fixture, &[42]].concat());
            for position in [fixture.len(), fixture.len() * 2] {
                assert_eq!(
                    liblrhsmm_rs_observation_set_read_stream_prefix_f64(
                        &source,
                        &limits,
                        &mut decoded
                    ),
                    0
                );
                assert_eq!(incoming.position, position);
                compare_set_observation_f64(decoded, &expected);
                assert_eq!(liblrhsmm_rs_observation_set_release_f64(&mut decoded), 0);
            }
            assert_eq!(incoming.bytes[incoming.position], 42);
            // Guard the boundary: prefix reading must not issue even an EOF read.
            incoming = input(fixture);
            incoming.reject_eof = true;
            assert_eq!(
                liblrhsmm_rs_observation_set_read_stream_prefix_f64(&source, &limits, &mut decoded),
                0
            );
            assert_eq!(incoming.position, fixture.len());
            assert_eq!(liblrhsmm_rs_observation_set_release_f64(&mut decoded), 0);
            incoming.position = 0;
            assert_eq!(
                liblrhsmm_rs_observation_set_read_stream_f64(&source, &limits, &mut decoded),
                3
            );
            assert!(decoded.is_null());
            for length in 0..fixture.len() {
                let mut sentinel = owner;
                assert_eq!(
                    liblrhsmm_rs_observation_set_read_bytes_f64(
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
                bytes[0] = 0xc0;
                bytes
            }] {
                let mut sentinel = owner;
                assert_eq!(
                    liblrhsmm_rs_observation_set_read_bytes_f64(
                        malformed.as_ptr(),
                        malformed.len(),
                        &limits,
                        &mut sentinel
                    ),
                    3
                );
                assert_eq!(sentinel, owner);
            }
            for budget in 0..required {
                let mut sentinel = owner;
                let tight = LiblrhsmmRsDecodeLimits {
                    max_array_entries: budget,
                };
                assert_eq!(
                    liblrhsmm_rs_observation_set_read_bytes_f64(
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
                liblrhsmm_rs_observation_set_read_bytes_f64(
                    null(),
                    usize::MAX,
                    &limits,
                    &mut sentinel
                ),
                2
            );
            assert_eq!(
                liblrhsmm_rs_observation_set_read_bytes_f64(null(), 1, &limits, &mut sentinel),
                1
            );
            assert_eq!(
                liblrhsmm_rs_observation_set_read_bytes_f64(null(), 0, &limits, &mut sentinel),
                3
            );
            assert_eq!(
                liblrhsmm_rs_observation_set_read_bytes_f64(
                    fixture.as_ptr(),
                    fixture.len(),
                    null(),
                    &mut sentinel
                ),
                1
            );
            assert_eq!(
                liblrhsmm_rs_observation_set_read_bytes_f64(
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
                liblrhsmm_rs_observation_set_read_stream_f64(&source, &limits, &mut sentinel),
                3
            );
            assert_eq!(sentinel, owner);
            incoming.status = 0;
            incoming.oversized = true;
            assert_eq!(
                liblrhsmm_rs_observation_set_read_stream_f64(&source, &limits, &mut sentinel),
                3
            );
            assert_eq!(sentinel, owner);
            source.read = None;
            assert_eq!(
                liblrhsmm_rs_observation_set_read_stream_f64(&source, &limits, &mut sentinel),
                3
            );
            assert_eq!(
                liblrhsmm_rs_observation_set_read_stream_prefix_f64(
                    &source,
                    &limits,
                    &mut sentinel
                ),
                3
            );
            source.read = Some(read);
            let calls = incoming.calls;
            assert_eq!(
                liblrhsmm_rs_observation_set_read_stream_f64(&source, &limits, null_mut()),
                1
            );
            assert_eq!(
                liblrhsmm_rs_observation_set_read_stream_prefix_f64(&source, &limits, null_mut()),
                1
            );
            assert_eq!(incoming.calls, calls);
            assert_eq!(
                liblrhsmm_rs_observation_set_read_stream_f64(null(), &limits, &mut sentinel),
                1
            );
            assert_eq!(
                liblrhsmm_rs_observation_set_read_stream_prefix_f64(&source, null(), &mut sentinel),
                1
            );
            for mode in 0..3 {
                outgoing = output();
                outgoing.interrupt = false;
                outgoing.status = if mode == 0 { LIBLRHSMM_RS_IO_ERROR } else { 0 };
                outgoing.oversized = mode == 1;
                outgoing.zero = mode == 2;
                assert_eq!(
                    liblrhsmm_rs_observation_set_write_stream_f64(owner, &sink),
                    3
                );
                assert_eq!(outgoing.calls, 1);
                assert!(outgoing.bytes.is_empty());
            }
            if fixture.len() > 7 {
                outgoing = output();
                outgoing.fail_after = Some(7);
                assert_eq!(
                    liblrhsmm_rs_observation_set_write_stream_f64(owner, &sink),
                    3
                );
                assert_eq!(outgoing.bytes, fixture[..7]);
                assert_eq!(outgoing.flushes, 0);
            }
            sink.write = None;
            assert_eq!(
                liblrhsmm_rs_observation_set_write_stream_f64(owner, &sink),
                3
            );
            assert_eq!(
                liblrhsmm_rs_observation_set_write_stream_f64(owner, null()),
                1
            );
            assert_eq!(
                liblrhsmm_rs_observation_set_write_stream_f64(null(), &sink),
                1
            );
            assert_eq!(
                liblrhsmm_rs_observation_set_write_bytes_f64(owner, null_mut()),
                1
            );
            assert_eq!(
                liblrhsmm_rs_observation_set_write_bytes_f64(null(), &mut bytes),
                1
            );
            assert!(bytes.is_null());
            compare_set_observation_f64(owner, &expected);
            assert_eq!(liblrhsmm_rs_observation_set_release_f64(&mut owner), 0);
        }
    }
}

unsafe fn compare_set_segmentation_f32(
    owner: *const LiblrhsmmRsSegmentationSetF32,
    expected: &[SegmentationData<f32>],
) {
    // SAFETY: Complete live set and independent child outputs, released uniquely.
    unsafe {
        let mut length = 99;
        assert_eq!(
            liblrhsmm_rs_segmentation_set_length_f32(owner, &mut length),
            0
        );
        assert_eq!(length, expected.len());
        for (index, sample) in expected.iter().enumerate() {
            let mut child = null_mut();
            assert_eq!(
                liblrhsmm_rs_segmentation_set_get_f32(owner, index, &mut child),
                0
            );
            compare_segmentation_f32(child, sample);
            assert_eq!(liblrhsmm_rs_segmentation_release_f32(&mut child), 0);
        }
    }
}
#[test]
fn segmentation_set_f32_original_wire_callbacks_limits_prefix_and_atomic_failures() {
    let mut fixtures = vec![
        include_bytes!("../../../tests/fixtures/c-segmentation-set.bin").to_vec(),
        include_bytes!("../../../tests/fixtures/c-copy-segmentation-set.bin").to_vec(),
    ];
    let mut selected =
        serial::read_segmentation_set_with_precision::<f32>(fixtures[0].as_slice()).unwrap();
    selected[0].outgoing[0][0].probability = 0.25 + f32::EPSILON;
    selected[0].outgoing[2][1].probability = f32::from_bits(1);
    let mut wire = Vec::new();
    serial::write_segmentation_set_with_precision(&selected, &mut wire).unwrap();
    fixtures.push(wire);
    fixtures.push(vec![0x90]);
    for fixture in fixtures {
        let fixture = fixture.as_slice();
        let expected = serial::read_segmentation_set_with_precision::<f32>(fixture).unwrap();
        let required = expected.len()
            + expected
                .iter()
                .map(|sample| {
                    3 + 3 * sample.boundaries.len()
                        + sample.output_states.len()
                        + sample.output_states.iter().map(Vec::len).sum::<usize>()
                        + 2 * sample.outgoing.iter().map(Vec::len).sum::<usize>()
                })
                .sum::<usize>();
        // SAFETY: All borrowed storage is independent; callbacks and live owners remain valid.
        unsafe {
            let mut owner = null_mut();
            let mut bytes = null_mut();
            let mut decoded = null_mut();
            let limits = LiblrhsmmRsDecodeLimits {
                max_array_entries: required,
            };
            assert_eq!(
                liblrhsmm_rs_segmentation_set_read_bytes_f32(
                    fixture.as_ptr(),
                    fixture.len(),
                    &limits,
                    &mut owner
                ),
                0
            );
            compare_set_segmentation_f32(owner, &expected);
            assert_eq!(
                liblrhsmm_rs_segmentation_set_write_bytes_f32(owner, &mut bytes),
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
                liblrhsmm_rs_segmentation_set_read_stream_f32(&source, &limits, &mut decoded),
                0
            );
            compare_set_segmentation_f32(decoded, &expected);
            assert_eq!(incoming.position, fixture.len());
            assert!(incoming.calls > fixture.len());
            let mut outgoing = output();
            let mut sink = LiblrhsmmRsWriteStream {
                context: (&mut outgoing as *mut Output).cast(),
                write: Some(write),
                flush: Some(flush),
            };
            assert_eq!(
                liblrhsmm_rs_segmentation_set_write_stream_f32(decoded, &sink),
                0
            );
            assert_eq!(outgoing.bytes, fixture);
            assert!(outgoing.calls > fixture.len());
            assert_eq!(outgoing.flushes, 0);
            assert_eq!(liblrhsmm_rs_write_stream_flush(&sink), 0);
            assert_eq!(outgoing.flushes, 1);
            assert_eq!(liblrhsmm_rs_segmentation_set_release_f32(&mut decoded), 0);
            incoming = input(&[fixture, fixture, &[42]].concat());
            for position in [fixture.len(), fixture.len() * 2] {
                assert_eq!(
                    liblrhsmm_rs_segmentation_set_read_stream_prefix_f32(
                        &source,
                        &limits,
                        &mut decoded
                    ),
                    0
                );
                assert_eq!(incoming.position, position);
                compare_set_segmentation_f32(decoded, &expected);
                assert_eq!(liblrhsmm_rs_segmentation_set_release_f32(&mut decoded), 0);
            }
            assert_eq!(incoming.bytes[incoming.position], 42);
            // Guard the boundary: prefix reading must not issue even an EOF read.
            incoming = input(fixture);
            incoming.reject_eof = true;
            assert_eq!(
                liblrhsmm_rs_segmentation_set_read_stream_prefix_f32(
                    &source,
                    &limits,
                    &mut decoded
                ),
                0
            );
            assert_eq!(incoming.position, fixture.len());
            assert_eq!(liblrhsmm_rs_segmentation_set_release_f32(&mut decoded), 0);
            incoming.position = 0;
            assert_eq!(
                liblrhsmm_rs_segmentation_set_read_stream_f32(&source, &limits, &mut decoded),
                3
            );
            assert!(decoded.is_null());
            for length in 0..fixture.len() {
                let mut sentinel = owner;
                assert_eq!(
                    liblrhsmm_rs_segmentation_set_read_bytes_f32(
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
                bytes[0] = 0xc0;
                bytes
            }] {
                let mut sentinel = owner;
                assert_eq!(
                    liblrhsmm_rs_segmentation_set_read_bytes_f32(
                        malformed.as_ptr(),
                        malformed.len(),
                        &limits,
                        &mut sentinel
                    ),
                    3
                );
                assert_eq!(sentinel, owner);
            }
            for budget in 0..required {
                let mut sentinel = owner;
                let tight = LiblrhsmmRsDecodeLimits {
                    max_array_entries: budget,
                };
                assert_eq!(
                    liblrhsmm_rs_segmentation_set_read_bytes_f32(
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
                liblrhsmm_rs_segmentation_set_read_bytes_f32(
                    null(),
                    usize::MAX,
                    &limits,
                    &mut sentinel
                ),
                2
            );
            assert_eq!(
                liblrhsmm_rs_segmentation_set_read_bytes_f32(null(), 1, &limits, &mut sentinel),
                1
            );
            assert_eq!(
                liblrhsmm_rs_segmentation_set_read_bytes_f32(null(), 0, &limits, &mut sentinel),
                3
            );
            assert_eq!(
                liblrhsmm_rs_segmentation_set_read_bytes_f32(
                    fixture.as_ptr(),
                    fixture.len(),
                    null(),
                    &mut sentinel
                ),
                1
            );
            assert_eq!(
                liblrhsmm_rs_segmentation_set_read_bytes_f32(
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
                liblrhsmm_rs_segmentation_set_read_stream_f32(&source, &limits, &mut sentinel),
                3
            );
            assert_eq!(sentinel, owner);
            incoming.status = 0;
            incoming.oversized = true;
            assert_eq!(
                liblrhsmm_rs_segmentation_set_read_stream_f32(&source, &limits, &mut sentinel),
                3
            );
            assert_eq!(sentinel, owner);
            source.read = None;
            assert_eq!(
                liblrhsmm_rs_segmentation_set_read_stream_f32(&source, &limits, &mut sentinel),
                3
            );
            assert_eq!(
                liblrhsmm_rs_segmentation_set_read_stream_prefix_f32(
                    &source,
                    &limits,
                    &mut sentinel
                ),
                3
            );
            source.read = Some(read);
            let calls = incoming.calls;
            assert_eq!(
                liblrhsmm_rs_segmentation_set_read_stream_f32(&source, &limits, null_mut()),
                1
            );
            assert_eq!(
                liblrhsmm_rs_segmentation_set_read_stream_prefix_f32(&source, &limits, null_mut()),
                1
            );
            assert_eq!(incoming.calls, calls);
            assert_eq!(
                liblrhsmm_rs_segmentation_set_read_stream_f32(null(), &limits, &mut sentinel),
                1
            );
            assert_eq!(
                liblrhsmm_rs_segmentation_set_read_stream_prefix_f32(
                    &source,
                    null(),
                    &mut sentinel
                ),
                1
            );
            for mode in 0..3 {
                outgoing = output();
                outgoing.interrupt = false;
                outgoing.status = if mode == 0 { LIBLRHSMM_RS_IO_ERROR } else { 0 };
                outgoing.oversized = mode == 1;
                outgoing.zero = mode == 2;
                assert_eq!(
                    liblrhsmm_rs_segmentation_set_write_stream_f32(owner, &sink),
                    3
                );
                assert_eq!(outgoing.calls, 1);
                assert!(outgoing.bytes.is_empty());
            }
            if fixture.len() > 7 {
                outgoing = output();
                outgoing.fail_after = Some(7);
                assert_eq!(
                    liblrhsmm_rs_segmentation_set_write_stream_f32(owner, &sink),
                    3
                );
                assert_eq!(outgoing.bytes, fixture[..7]);
                assert_eq!(outgoing.flushes, 0);
            }
            sink.write = None;
            assert_eq!(
                liblrhsmm_rs_segmentation_set_write_stream_f32(owner, &sink),
                3
            );
            assert_eq!(
                liblrhsmm_rs_segmentation_set_write_stream_f32(owner, null()),
                1
            );
            assert_eq!(
                liblrhsmm_rs_segmentation_set_write_stream_f32(null(), &sink),
                1
            );
            assert_eq!(
                liblrhsmm_rs_segmentation_set_write_bytes_f32(owner, null_mut()),
                1
            );
            assert_eq!(
                liblrhsmm_rs_segmentation_set_write_bytes_f32(null(), &mut bytes),
                1
            );
            assert!(bytes.is_null());
            compare_set_segmentation_f32(owner, &expected);
            assert_eq!(liblrhsmm_rs_segmentation_set_release_f32(&mut owner), 0);
        }
    }
}

unsafe fn compare_set_segmentation_f64(
    owner: *const LiblrhsmmRsSegmentationSetF64,
    expected: &[SegmentationData<f64>],
) {
    // SAFETY: Complete live set and independent child outputs, released uniquely.
    unsafe {
        let mut length = 99;
        assert_eq!(
            liblrhsmm_rs_segmentation_set_length_f64(owner, &mut length),
            0
        );
        assert_eq!(length, expected.len());
        for (index, sample) in expected.iter().enumerate() {
            let mut child = null_mut();
            assert_eq!(
                liblrhsmm_rs_segmentation_set_get_f64(owner, index, &mut child),
                0
            );
            compare_segmentation_f64(child, sample);
            assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut child), 0);
        }
    }
}
#[test]
fn segmentation_set_f64_original_wire_callbacks_limits_prefix_and_atomic_failures() {
    let mut fixtures = vec![
        include_bytes!("../../../tests/fixtures/c-segmentation-set.bin").to_vec(),
        include_bytes!("../../../tests/fixtures/c-copy-segmentation-set.bin").to_vec(),
    ];
    let mut selected =
        serial::read_segmentation_set_with_precision::<f64>(fixtures[0].as_slice()).unwrap();
    selected[0].outgoing[0][0].probability = 0.25 + f64::EPSILON;
    selected[0].outgoing[2][1].probability = f64::from(f32::from_bits(1));
    let mut wire = Vec::new();
    serial::write_segmentation_set_with_precision(&selected, &mut wire).unwrap();
    fixtures.push(wire);
    fixtures.push(vec![0x90]);
    for fixture in fixtures {
        let fixture = fixture.as_slice();
        let expected = serial::read_segmentation_set_with_precision::<f64>(fixture).unwrap();
        let required = expected.len()
            + expected
                .iter()
                .map(|sample| {
                    3 + 3 * sample.boundaries.len()
                        + sample.output_states.len()
                        + sample.output_states.iter().map(Vec::len).sum::<usize>()
                        + 2 * sample.outgoing.iter().map(Vec::len).sum::<usize>()
                })
                .sum::<usize>();
        // SAFETY: All borrowed storage is independent; callbacks and live owners remain valid.
        unsafe {
            let mut owner = null_mut();
            let mut bytes = null_mut();
            let mut decoded = null_mut();
            let limits = LiblrhsmmRsDecodeLimits {
                max_array_entries: required,
            };
            assert_eq!(
                liblrhsmm_rs_segmentation_set_read_bytes_f64(
                    fixture.as_ptr(),
                    fixture.len(),
                    &limits,
                    &mut owner
                ),
                0
            );
            compare_set_segmentation_f64(owner, &expected);
            assert_eq!(
                liblrhsmm_rs_segmentation_set_write_bytes_f64(owner, &mut bytes),
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
                liblrhsmm_rs_segmentation_set_read_stream_f64(&source, &limits, &mut decoded),
                0
            );
            compare_set_segmentation_f64(decoded, &expected);
            assert_eq!(incoming.position, fixture.len());
            assert!(incoming.calls > fixture.len());
            let mut outgoing = output();
            let mut sink = LiblrhsmmRsWriteStream {
                context: (&mut outgoing as *mut Output).cast(),
                write: Some(write),
                flush: Some(flush),
            };
            assert_eq!(
                liblrhsmm_rs_segmentation_set_write_stream_f64(decoded, &sink),
                0
            );
            assert_eq!(outgoing.bytes, fixture);
            assert!(outgoing.calls > fixture.len());
            assert_eq!(outgoing.flushes, 0);
            assert_eq!(liblrhsmm_rs_write_stream_flush(&sink), 0);
            assert_eq!(outgoing.flushes, 1);
            assert_eq!(liblrhsmm_rs_segmentation_set_release_f64(&mut decoded), 0);
            incoming = input(&[fixture, fixture, &[42]].concat());
            for position in [fixture.len(), fixture.len() * 2] {
                assert_eq!(
                    liblrhsmm_rs_segmentation_set_read_stream_prefix_f64(
                        &source,
                        &limits,
                        &mut decoded
                    ),
                    0
                );
                assert_eq!(incoming.position, position);
                compare_set_segmentation_f64(decoded, &expected);
                assert_eq!(liblrhsmm_rs_segmentation_set_release_f64(&mut decoded), 0);
            }
            assert_eq!(incoming.bytes[incoming.position], 42);
            // Guard the boundary: prefix reading must not issue even an EOF read.
            incoming = input(fixture);
            incoming.reject_eof = true;
            assert_eq!(
                liblrhsmm_rs_segmentation_set_read_stream_prefix_f64(
                    &source,
                    &limits,
                    &mut decoded
                ),
                0
            );
            assert_eq!(incoming.position, fixture.len());
            assert_eq!(liblrhsmm_rs_segmentation_set_release_f64(&mut decoded), 0);
            incoming.position = 0;
            assert_eq!(
                liblrhsmm_rs_segmentation_set_read_stream_f64(&source, &limits, &mut decoded),
                3
            );
            assert!(decoded.is_null());
            for length in 0..fixture.len() {
                let mut sentinel = owner;
                assert_eq!(
                    liblrhsmm_rs_segmentation_set_read_bytes_f64(
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
                bytes[0] = 0xc0;
                bytes
            }] {
                let mut sentinel = owner;
                assert_eq!(
                    liblrhsmm_rs_segmentation_set_read_bytes_f64(
                        malformed.as_ptr(),
                        malformed.len(),
                        &limits,
                        &mut sentinel
                    ),
                    3
                );
                assert_eq!(sentinel, owner);
            }
            for budget in 0..required {
                let mut sentinel = owner;
                let tight = LiblrhsmmRsDecodeLimits {
                    max_array_entries: budget,
                };
                assert_eq!(
                    liblrhsmm_rs_segmentation_set_read_bytes_f64(
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
                liblrhsmm_rs_segmentation_set_read_bytes_f64(
                    null(),
                    usize::MAX,
                    &limits,
                    &mut sentinel
                ),
                2
            );
            assert_eq!(
                liblrhsmm_rs_segmentation_set_read_bytes_f64(null(), 1, &limits, &mut sentinel),
                1
            );
            assert_eq!(
                liblrhsmm_rs_segmentation_set_read_bytes_f64(null(), 0, &limits, &mut sentinel),
                3
            );
            assert_eq!(
                liblrhsmm_rs_segmentation_set_read_bytes_f64(
                    fixture.as_ptr(),
                    fixture.len(),
                    null(),
                    &mut sentinel
                ),
                1
            );
            assert_eq!(
                liblrhsmm_rs_segmentation_set_read_bytes_f64(
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
                liblrhsmm_rs_segmentation_set_read_stream_f64(&source, &limits, &mut sentinel),
                3
            );
            assert_eq!(sentinel, owner);
            incoming.status = 0;
            incoming.oversized = true;
            assert_eq!(
                liblrhsmm_rs_segmentation_set_read_stream_f64(&source, &limits, &mut sentinel),
                3
            );
            assert_eq!(sentinel, owner);
            source.read = None;
            assert_eq!(
                liblrhsmm_rs_segmentation_set_read_stream_f64(&source, &limits, &mut sentinel),
                3
            );
            assert_eq!(
                liblrhsmm_rs_segmentation_set_read_stream_prefix_f64(
                    &source,
                    &limits,
                    &mut sentinel
                ),
                3
            );
            source.read = Some(read);
            let calls = incoming.calls;
            assert_eq!(
                liblrhsmm_rs_segmentation_set_read_stream_f64(&source, &limits, null_mut()),
                1
            );
            assert_eq!(
                liblrhsmm_rs_segmentation_set_read_stream_prefix_f64(&source, &limits, null_mut()),
                1
            );
            assert_eq!(incoming.calls, calls);
            assert_eq!(
                liblrhsmm_rs_segmentation_set_read_stream_f64(null(), &limits, &mut sentinel),
                1
            );
            assert_eq!(
                liblrhsmm_rs_segmentation_set_read_stream_prefix_f64(
                    &source,
                    null(),
                    &mut sentinel
                ),
                1
            );
            for mode in 0..3 {
                outgoing = output();
                outgoing.interrupt = false;
                outgoing.status = if mode == 0 { LIBLRHSMM_RS_IO_ERROR } else { 0 };
                outgoing.oversized = mode == 1;
                outgoing.zero = mode == 2;
                assert_eq!(
                    liblrhsmm_rs_segmentation_set_write_stream_f64(owner, &sink),
                    3
                );
                assert_eq!(outgoing.calls, 1);
                assert!(outgoing.bytes.is_empty());
            }
            if fixture.len() > 7 {
                outgoing = output();
                outgoing.fail_after = Some(7);
                assert_eq!(
                    liblrhsmm_rs_segmentation_set_write_stream_f64(owner, &sink),
                    3
                );
                assert_eq!(outgoing.bytes, fixture[..7]);
                assert_eq!(outgoing.flushes, 0);
            }
            sink.write = None;
            assert_eq!(
                liblrhsmm_rs_segmentation_set_write_stream_f64(owner, &sink),
                3
            );
            assert_eq!(
                liblrhsmm_rs_segmentation_set_write_stream_f64(owner, null()),
                1
            );
            assert_eq!(
                liblrhsmm_rs_segmentation_set_write_stream_f64(null(), &sink),
                1
            );
            assert_eq!(
                liblrhsmm_rs_segmentation_set_write_bytes_f64(owner, null_mut()),
                1
            );
            assert_eq!(
                liblrhsmm_rs_segmentation_set_write_bytes_f64(null(), &mut bytes),
                1
            );
            assert!(bytes.is_null());
            compare_set_segmentation_f64(owner, &expected);
            assert_eq!(liblrhsmm_rs_segmentation_set_release_f64(&mut owner), 0);
        }
    }
}

#[test]
fn observation_set_f64_writer_rounds_wire_without_changing_owned_precision() {
    let fixture = include_bytes!("../../../tests/fixtures/c-observation-set.bin").as_slice();
    let mut selected = serial::read_observation_set_with_precision::<f64>(fixture).unwrap();
    // SAFETY: Initialized independent arrays and unique live owners remain valid throughout.
    unsafe {
        let limits = LiblrhsmmRsDecodeLimits {
            max_array_entries: 1024,
        };
        let mut original = null_mut();
        let mut child = null_mut();
        let mut empty = null_mut();
        assert_eq!(
            liblrhsmm_rs_observation_set_read_bytes_f64(
                fixture.as_ptr(),
                fixture.len(),
                &limits,
                &mut original
            ),
            0
        );
        assert_eq!(
            liblrhsmm_rs_observation_set_get_f64(original, 0, &mut child),
            0
        );
        let values = [0.25 + f64::EPSILON, f64::from(f32::from_bits(1))];
        assert_eq!(
            liblrhsmm_rs_observation_set_values_f64(child, 0, 0, values.as_ptr(), values.len()),
            0
        );
        selected[0].streams[0].values[..2].copy_from_slice(&values);
        let dimensions = [1];
        assert_eq!(
            liblrhsmm_rs_observation_new_f64(0, dimensions.as_ptr(), 1, &mut empty),
            0
        );
        selected.insert(1, ObservationData::<f64>::new(0, &dimensions).unwrap());
        compare_observation_f64(child, &selected[0]);
        let mut original_second = null_mut();
        assert_eq!(
            liblrhsmm_rs_observation_set_get_f64(original, 1, &mut original_second),
            0
        );
        let inputs = [
            child.cast_const(),
            empty.cast_const(),
            original_second.cast_const(),
        ];
        let mut owner = null_mut();
        let mut bytes = null_mut();
        assert_eq!(
            liblrhsmm_rs_observation_set_create_f64(inputs.as_ptr(), inputs.len(), &mut owner),
            0
        );
        assert_eq!(liblrhsmm_rs_observation_set_release_f64(&mut original), 0);
        assert_eq!(liblrhsmm_rs_observation_release_f64(&mut child), 0);
        assert_eq!(liblrhsmm_rs_observation_release_f64(&mut empty), 0);
        assert_eq!(
            liblrhsmm_rs_observation_release_f64(&mut original_second),
            0
        );
        compare_set_observation_f64(owner, &selected);
        let mut expected_wire = Vec::new();
        serial::write_observation_set_with_precision(&selected, &mut expected_wire).unwrap();
        assert_eq!(
            liblrhsmm_rs_observation_set_write_bytes_f64(owner, &mut bytes),
            0
        );
        assert_eq!(copy(bytes), expected_wire);
        compare_set_observation_f64(owner, &selected);
        let decoded =
            serial::read_observation_set_with_precision::<f64>(expected_wire.as_slice()).unwrap();
        assert_eq!(
            decoded[0].streams[0].values[0].to_bits(),
            0.25_f64.to_bits()
        );
        assert_eq!(
            decoded[0].streams[0].values[1].to_bits(),
            f64::from(f32::from_bits(1)).to_bits()
        );
        assert_eq!(decoded[1], selected[1]);
        let mut restored = null_mut();
        assert_eq!(
            liblrhsmm_rs_observation_set_read_bytes_f64(
                expected_wire.as_ptr(),
                expected_wire.len(),
                &limits,
                &mut restored
            ),
            0
        );
        compare_set_observation_f64(restored, &decoded);
        assert_eq!(liblrhsmm_rs_observation_set_release_f64(&mut owner), 0);
        assert_eq!(liblrhsmm_rs_observation_set_release_f64(&mut restored), 0);
        assert_eq!(liblrhsmm_rs_bytes_release(&mut bytes), 0);
    }
}

#[test]
fn segmentation_set_f64_writer_rounds_wire_without_changing_owned_precision() {
    let fixture = include_bytes!("../../../tests/fixtures/c-segmentation-set.bin").as_slice();
    let mut selected = serial::read_segmentation_set_with_precision::<f64>(fixture).unwrap();
    // SAFETY: Initialized independent arrays and unique live owners remain valid throughout.
    unsafe {
        let limits = LiblrhsmmRsDecodeLimits {
            max_array_entries: 1024,
        };
        let mut original = null_mut();
        let mut child = null_mut();
        let mut empty = null_mut();
        assert_eq!(
            liblrhsmm_rs_segmentation_set_read_bytes_f64(
                fixture.as_ptr(),
                fixture.len(),
                &limits,
                &mut original
            ),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_get_f64(original, 0, &mut child),
            0
        );
        let first = [
            LiblrhsmmRsJumpF64 {
                delta: 2,
                probability: 0.25 + f64::EPSILON,
            },
            LiblrhsmmRsJumpF64 {
                delta: 1,
                probability: 0.75,
            },
        ];
        let third = [
            LiblrhsmmRsJumpF64 {
                delta: 0,
                probability: f64::from(0.2_f32),
            },
            LiblrhsmmRsJumpF64 {
                delta: -1,
                probability: f64::from(f32::from_bits(1)),
            },
            LiblrhsmmRsJumpF64 {
                delta: 1,
                probability: f64::from(0.7_f32),
            },
        ];
        assert_eq!(
            liblrhsmm_rs_segmentation_set_outgoing_f64(child, 0, first.as_ptr(), first.len()),
            0
        );
        assert_eq!(
            liblrhsmm_rs_segmentation_set_outgoing_f64(child, 2, third.as_ptr(), third.len()),
            0
        );
        selected[0].outgoing[0][0].probability = first[0].probability;
        selected[0].outgoing[2][1].probability = third[1].probability;
        assert_eq!(liblrhsmm_rs_segmentation_new_f64(1, 0, &mut empty), 0);
        selected.insert(1, SegmentationData::<f64>::new(1, 0));
        compare_segmentation_f64(child, &selected[0]);
        let mut original_second = null_mut();
        assert_eq!(
            liblrhsmm_rs_segmentation_set_get_f64(original, 1, &mut original_second),
            0
        );
        let inputs = [
            child.cast_const(),
            empty.cast_const(),
            original_second.cast_const(),
        ];
        let mut owner = null_mut();
        let mut bytes = null_mut();
        assert_eq!(
            liblrhsmm_rs_segmentation_set_create_f64(inputs.as_ptr(), inputs.len(), &mut owner),
            0
        );
        assert_eq!(liblrhsmm_rs_segmentation_set_release_f64(&mut original), 0);
        assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut child), 0);
        assert_eq!(liblrhsmm_rs_segmentation_release_f64(&mut empty), 0);
        assert_eq!(
            liblrhsmm_rs_segmentation_release_f64(&mut original_second),
            0
        );
        compare_set_segmentation_f64(owner, &selected);
        let mut expected_wire = Vec::new();
        serial::write_segmentation_set_with_precision(&selected, &mut expected_wire).unwrap();
        assert_eq!(
            liblrhsmm_rs_segmentation_set_write_bytes_f64(owner, &mut bytes),
            0
        );
        assert_eq!(copy(bytes), expected_wire);
        compare_set_segmentation_f64(owner, &selected);
        let decoded =
            serial::read_segmentation_set_with_precision::<f64>(expected_wire.as_slice()).unwrap();
        assert_eq!(
            decoded[0].outgoing[0][0].probability.to_bits(),
            0.25_f64.to_bits()
        );
        assert_eq!(
            decoded[0].outgoing[2][1].probability.to_bits(),
            f64::from(f32::from_bits(1)).to_bits()
        );
        assert_eq!(decoded[1], selected[1]);
        let mut restored = null_mut();
        assert_eq!(
            liblrhsmm_rs_segmentation_set_read_bytes_f64(
                expected_wire.as_ptr(),
                expected_wire.len(),
                &limits,
                &mut restored
            ),
            0
        );
        compare_set_segmentation_f64(restored, &decoded);
        assert_eq!(liblrhsmm_rs_segmentation_set_release_f64(&mut owner), 0);
        assert_eq!(liblrhsmm_rs_segmentation_set_release_f64(&mut restored), 0);
        assert_eq!(liblrhsmm_rs_bytes_release(&mut bytes), 0);
    }
}
