#![cfg(feature = "serialization")]

use liblrhsmm_rs::model::DurationParameters;
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
#[test]
fn byte_owners_keep_independent_storage_and_checked_intervals() {
    // SAFETY: Independent valid input/output storage and uniquely released owners.
    unsafe {
        let mut owner = null_mut();
        let mut original = [1, 2, 3, 4];
        assert_eq!(
            liblrhsmm_rs_bytes_create(original.as_ptr(), original.len(), &mut owner),
            0
        );
        original.fill(99);
        assert_eq!(copy(owner), [1, 2, 3, 4]);
        let mut cloned = null_mut();
        assert_eq!(liblrhsmm_rs_bytes_clone(owner, &mut cloned), 0);
        assert_eq!(liblrhsmm_rs_bytes_release(&mut owner), 0);
        assert!(owner.is_null());
        assert_eq!(liblrhsmm_rs_bytes_release(&mut owner), 0);
        assert_eq!(copy(cloned), [1, 2, 3, 4]);
        assert_eq!(liblrhsmm_rs_bytes_copy(cloned, 4, null_mut(), 0), 0);
        assert_eq!(liblrhsmm_rs_bytes_copy(cloned, 5, null_mut(), 0), 2);
        let mut result = [71; 2];
        assert_eq!(
            liblrhsmm_rs_bytes_copy(cloned, 3, result.as_mut_ptr(), 2),
            2
        );
        assert_eq!(result, [71; 2]);
        assert_eq!(
            liblrhsmm_rs_bytes_copy(cloned, usize::MAX, result.as_mut_ptr(), 1),
            2
        );
        assert_eq!(liblrhsmm_rs_bytes_copy(cloned, 0, null_mut(), 1), 1);
        assert_eq!(liblrhsmm_rs_bytes_create(null(), usize::MAX, &mut owner), 2);
        assert!(owner.is_null());
        assert_eq!(liblrhsmm_rs_bytes_create(null(), 1, &mut owner), 1);
        assert_eq!(liblrhsmm_rs_bytes_create(null(), 0, &mut owner), 0);
        assert!(copy(owner).is_empty());
        assert_eq!(liblrhsmm_rs_bytes_copy(owner, 0, null_mut(), 0), 0);
        assert_eq!(liblrhsmm_rs_bytes_length(owner, null_mut()), 1);
        assert_eq!(liblrhsmm_rs_bytes_clone(cloned, null_mut()), 1);
        assert_eq!(liblrhsmm_rs_bytes_release(null_mut()), 1);
        assert_eq!(liblrhsmm_rs_bytes_release(&mut owner), 0);
        assert_eq!(liblrhsmm_rs_bytes_release(&mut cloned), 0);
    }
}

#[test]
fn duration_serial_f32_original_wire_callbacks_and_errors() {
    let fixture = include_bytes!("../../../tests/fixtures/c-standalone-duration.bin");
    // SAFETY: All borrowed buffers/contexts and output slots remain independent and valid; each owner is released once.
    unsafe {
        let mut limits = LiblrhsmmRsDecodeLimits {
            max_array_entries: 0,
        };
        assert_eq!(liblrhsmm_rs_decode_limits_default(&mut limits), 0);
        assert_eq!(limits.max_array_entries, 16 * 1024 * 1024);
        let mut owner = null_mut();
        assert_eq!(
            liblrhsmm_rs_duration_read_bytes_f32(
                fixture.as_ptr(),
                fixture.len(),
                &limits,
                &mut owner
            ),
            0
        );
        let mut bytes = null_mut();
        assert_eq!(liblrhsmm_rs_duration_write_bytes_f32(owner, &mut bytes), 0);
        assert_eq!(copy(bytes), fixture);
        let native = DurationParameters::<f32>::read_from(fixture.as_slice()).unwrap();
        let mut descriptor = LiblrhsmmRsDurationParametersF32 {
            mean: 0.0,
            variance: 0.0,
            minimum: 0,
            maximum: 0,
            fixed_mean: 0,
            variance_floor: 0.0,
        };
        assert_eq!(liblrhsmm_rs_duration_get_f32(owner, &mut descriptor), 0);
        assert_eq!(descriptor.mean.to_bits(), native.mean.to_bits());
        assert_eq!(descriptor.variance.to_bits(), native.variance.to_bits());
        assert_eq!(
            (
                descriptor.minimum,
                descriptor.maximum,
                descriptor.fixed_mean
            ),
            (native.minimum, native.maximum, native.fixed_mean)
        );
        assert_eq!(
            descriptor.variance_floor.to_bits(),
            native.variance_floor.to_bits()
        );
        let mut reader = input(fixture);
        let mut stream = LiblrhsmmRsReadStream {
            context: (&mut reader as *mut Input).cast(),
            read: Some(read),
        };
        let mut decoded = null_mut();
        assert_eq!(
            liblrhsmm_rs_duration_read_stream_f32(&stream, &limits, &mut decoded),
            0
        );
        assert_eq!(reader.position, fixture.len());
        assert!(reader.calls > fixture.len());
        let mut writer = output();
        let mut sink = LiblrhsmmRsWriteStream {
            context: (&mut writer as *mut Output).cast(),
            write: Some(write),
            flush: Some(flush),
        };
        assert_eq!(liblrhsmm_rs_duration_write_stream_f32(decoded, &sink), 0);
        assert_eq!(writer.bytes, fixture);
        assert_eq!(writer.flushes, 0);
        assert_eq!(liblrhsmm_rs_write_stream_flush(&sink), 0);
        assert_eq!(writer.flushes, 1);
        writer.status = LIBLRHSMM_RS_IO_INTERRUPTED;
        assert_eq!(liblrhsmm_rs_write_stream_flush(&sink), 3);
        writer.status = LIBLRHSMM_RS_IO_ERROR;
        assert_eq!(liblrhsmm_rs_write_stream_flush(&sink), 3);
        writer.status = 0;
        assert_eq!(liblrhsmm_rs_duration_release_f32(&mut decoded), 0);
        reader = input(&[fixture.as_slice(), fixture.as_slice(), &[42]].concat());
        stream.context = (&mut reader as *mut Input).cast();
        for position in [fixture.len(), fixture.len() * 2] {
            assert_eq!(
                liblrhsmm_rs_duration_read_stream_prefix_f32(&stream, &limits, &mut decoded),
                0
            );
            assert_eq!(reader.position, position);
            assert_eq!(liblrhsmm_rs_duration_release_f32(&mut decoded), 0);
        }
        assert_eq!(reader.bytes[reader.position], 42);
        for end in 0..fixture.len() {
            assert_eq!(
                liblrhsmm_rs_duration_read_bytes_f32(fixture.as_ptr(), end, &limits, &mut decoded),
                3
            );
            assert!(decoded.is_null());
        }
        let mut trailing = fixture.to_vec();
        trailing.push(0);
        assert_eq!(
            liblrhsmm_rs_duration_read_bytes_f32(
                trailing.as_ptr(),
                trailing.len(),
                &limits,
                &mut decoded
            ),
            3
        );
        let tight = LiblrhsmmRsDecodeLimits {
            max_array_entries: 5,
        };
        assert_eq!(
            liblrhsmm_rs_duration_read_bytes_f32(
                fixture.as_ptr(),
                fixture.len(),
                &tight,
                &mut decoded
            ),
            3
        );
        let exact = LiblrhsmmRsDecodeLimits {
            max_array_entries: 6,
        };
        assert_eq!(
            liblrhsmm_rs_duration_read_bytes_f32(
                fixture.as_ptr(),
                fixture.len(),
                &exact,
                &mut decoded
            ),
            0
        );
        assert_eq!(liblrhsmm_rs_duration_release_f32(&mut decoded), 0);
        assert_eq!(
            liblrhsmm_rs_duration_read_bytes_f32(null(), usize::MAX, &limits, &mut decoded),
            2
        );
        assert_eq!(
            liblrhsmm_rs_duration_read_bytes_f32(null(), 1, &limits, &mut decoded),
            1
        );
        assert_eq!(
            liblrhsmm_rs_duration_read_bytes_f32(null(), 0, &limits, &mut decoded),
            3
        );
        assert_eq!(
            liblrhsmm_rs_duration_read_bytes_f32(
                fixture.as_ptr(),
                fixture.len(),
                null(),
                &mut decoded
            ),
            1
        );
        assert_eq!(
            liblrhsmm_rs_duration_read_bytes_f32(
                fixture.as_ptr(),
                fixture.len(),
                &limits,
                null_mut()
            ),
            1
        );
        reader = input(fixture);
        stream.context = (&mut reader as *mut Input).cast();
        assert_eq!(
            liblrhsmm_rs_duration_read_stream_f32(&stream, &limits, null_mut()),
            1
        );
        assert_eq!(reader.calls, 0);
        reader.interrupt = false;
        reader.status = LIBLRHSMM_RS_IO_ERROR;
        assert_eq!(
            liblrhsmm_rs_duration_read_stream_f32(&stream, &limits, &mut decoded),
            3
        );
        assert!(decoded.is_null());
        reader.status = 0;
        reader.oversized = true;
        assert_eq!(
            liblrhsmm_rs_duration_read_stream_f32(&stream, &limits, &mut decoded),
            3
        );
        stream.read = None;
        assert_eq!(
            liblrhsmm_rs_duration_read_stream_f32(&stream, &limits, &mut decoded),
            3
        );
        assert_eq!(
            liblrhsmm_rs_duration_read_stream_f32(null(), &limits, &mut decoded),
            1
        );
        writer = output();
        sink.context = (&mut writer as *mut Output).cast();
        writer.interrupt = false;
        writer.status = LIBLRHSMM_RS_IO_ERROR;
        assert_eq!(liblrhsmm_rs_duration_write_stream_f32(owner, &sink), 3);
        writer.status = 0;
        writer.oversized = true;
        assert_eq!(liblrhsmm_rs_duration_write_stream_f32(owner, &sink), 3);
        writer.oversized = false;
        writer.zero = true;
        assert_eq!(liblrhsmm_rs_duration_write_stream_f32(owner, &sink), 3);
        sink.write = None;
        assert_eq!(liblrhsmm_rs_duration_write_stream_f32(owner, &sink), 3);
        sink.flush = None;
        assert_eq!(liblrhsmm_rs_write_stream_flush(&sink), 3);
        assert_eq!(liblrhsmm_rs_write_stream_flush(null()), 1);
        assert_eq!(liblrhsmm_rs_duration_write_stream_f32(owner, null()), 1);
        assert_eq!(liblrhsmm_rs_duration_write_bytes_f32(owner, null_mut()), 1);
        assert_eq!(liblrhsmm_rs_decode_limits_default(null_mut()), 1);
        assert_eq!(liblrhsmm_rs_bytes_release(&mut bytes), 0);
        assert_eq!(liblrhsmm_rs_duration_release_f32(&mut owner), 0);
    }
}

#[test]
fn duration_serial_f64_original_wire_callbacks_and_errors() {
    let fixture = include_bytes!("../../../tests/fixtures/c-standalone-duration.bin");
    // SAFETY: All borrowed buffers/contexts and output slots remain independent and valid; each owner is released once.
    unsafe {
        let mut limits = LiblrhsmmRsDecodeLimits {
            max_array_entries: 0,
        };
        assert_eq!(liblrhsmm_rs_decode_limits_default(&mut limits), 0);
        assert_eq!(limits.max_array_entries, 16 * 1024 * 1024);
        let mut owner = null_mut();
        assert_eq!(
            liblrhsmm_rs_duration_read_bytes_f64(
                fixture.as_ptr(),
                fixture.len(),
                &limits,
                &mut owner
            ),
            0
        );
        let mut bytes = null_mut();
        assert_eq!(liblrhsmm_rs_duration_write_bytes_f64(owner, &mut bytes), 0);
        assert_eq!(copy(bytes), fixture);
        let native = DurationParameters::<f64>::read_from(fixture.as_slice()).unwrap();
        let mut descriptor = LiblrhsmmRsDurationParametersF64 {
            mean: 0.0,
            variance: 0.0,
            minimum: 0,
            maximum: 0,
            fixed_mean: 0,
            variance_floor: 0.0,
        };
        assert_eq!(liblrhsmm_rs_duration_get_f64(owner, &mut descriptor), 0);
        assert_eq!(descriptor.mean.to_bits(), native.mean.to_bits());
        assert_eq!(descriptor.variance.to_bits(), native.variance.to_bits());
        assert_eq!(
            (
                descriptor.minimum,
                descriptor.maximum,
                descriptor.fixed_mean
            ),
            (native.minimum, native.maximum, native.fixed_mean)
        );
        assert_eq!(
            descriptor.variance_floor.to_bits(),
            native.variance_floor.to_bits()
        );
        let mut reader = input(fixture);
        let mut stream = LiblrhsmmRsReadStream {
            context: (&mut reader as *mut Input).cast(),
            read: Some(read),
        };
        let mut decoded = null_mut();
        assert_eq!(
            liblrhsmm_rs_duration_read_stream_f64(&stream, &limits, &mut decoded),
            0
        );
        assert_eq!(reader.position, fixture.len());
        assert!(reader.calls > fixture.len());
        let mut writer = output();
        let mut sink = LiblrhsmmRsWriteStream {
            context: (&mut writer as *mut Output).cast(),
            write: Some(write),
            flush: Some(flush),
        };
        assert_eq!(liblrhsmm_rs_duration_write_stream_f64(decoded, &sink), 0);
        assert_eq!(writer.bytes, fixture);
        assert_eq!(writer.flushes, 0);
        assert_eq!(liblrhsmm_rs_write_stream_flush(&sink), 0);
        assert_eq!(writer.flushes, 1);
        writer.status = LIBLRHSMM_RS_IO_INTERRUPTED;
        assert_eq!(liblrhsmm_rs_write_stream_flush(&sink), 3);
        writer.status = LIBLRHSMM_RS_IO_ERROR;
        assert_eq!(liblrhsmm_rs_write_stream_flush(&sink), 3);
        writer.status = 0;
        assert_eq!(liblrhsmm_rs_duration_release_f64(&mut decoded), 0);
        reader = input(&[fixture.as_slice(), fixture.as_slice(), &[42]].concat());
        stream.context = (&mut reader as *mut Input).cast();
        for position in [fixture.len(), fixture.len() * 2] {
            assert_eq!(
                liblrhsmm_rs_duration_read_stream_prefix_f64(&stream, &limits, &mut decoded),
                0
            );
            assert_eq!(reader.position, position);
            assert_eq!(liblrhsmm_rs_duration_release_f64(&mut decoded), 0);
        }
        assert_eq!(reader.bytes[reader.position], 42);
        for end in 0..fixture.len() {
            assert_eq!(
                liblrhsmm_rs_duration_read_bytes_f64(fixture.as_ptr(), end, &limits, &mut decoded),
                3
            );
            assert!(decoded.is_null());
        }
        let mut trailing = fixture.to_vec();
        trailing.push(0);
        assert_eq!(
            liblrhsmm_rs_duration_read_bytes_f64(
                trailing.as_ptr(),
                trailing.len(),
                &limits,
                &mut decoded
            ),
            3
        );
        let tight = LiblrhsmmRsDecodeLimits {
            max_array_entries: 5,
        };
        assert_eq!(
            liblrhsmm_rs_duration_read_bytes_f64(
                fixture.as_ptr(),
                fixture.len(),
                &tight,
                &mut decoded
            ),
            3
        );
        let exact = LiblrhsmmRsDecodeLimits {
            max_array_entries: 6,
        };
        assert_eq!(
            liblrhsmm_rs_duration_read_bytes_f64(
                fixture.as_ptr(),
                fixture.len(),
                &exact,
                &mut decoded
            ),
            0
        );
        assert_eq!(liblrhsmm_rs_duration_release_f64(&mut decoded), 0);
        assert_eq!(
            liblrhsmm_rs_duration_read_bytes_f64(null(), usize::MAX, &limits, &mut decoded),
            2
        );
        assert_eq!(
            liblrhsmm_rs_duration_read_bytes_f64(null(), 1, &limits, &mut decoded),
            1
        );
        assert_eq!(
            liblrhsmm_rs_duration_read_bytes_f64(null(), 0, &limits, &mut decoded),
            3
        );
        assert_eq!(
            liblrhsmm_rs_duration_read_bytes_f64(
                fixture.as_ptr(),
                fixture.len(),
                null(),
                &mut decoded
            ),
            1
        );
        assert_eq!(
            liblrhsmm_rs_duration_read_bytes_f64(
                fixture.as_ptr(),
                fixture.len(),
                &limits,
                null_mut()
            ),
            1
        );
        reader = input(fixture);
        stream.context = (&mut reader as *mut Input).cast();
        assert_eq!(
            liblrhsmm_rs_duration_read_stream_f64(&stream, &limits, null_mut()),
            1
        );
        assert_eq!(reader.calls, 0);
        reader.interrupt = false;
        reader.status = LIBLRHSMM_RS_IO_ERROR;
        assert_eq!(
            liblrhsmm_rs_duration_read_stream_f64(&stream, &limits, &mut decoded),
            3
        );
        assert!(decoded.is_null());
        reader.status = 0;
        reader.oversized = true;
        assert_eq!(
            liblrhsmm_rs_duration_read_stream_f64(&stream, &limits, &mut decoded),
            3
        );
        stream.read = None;
        assert_eq!(
            liblrhsmm_rs_duration_read_stream_f64(&stream, &limits, &mut decoded),
            3
        );
        assert_eq!(
            liblrhsmm_rs_duration_read_stream_f64(null(), &limits, &mut decoded),
            1
        );
        writer = output();
        sink.context = (&mut writer as *mut Output).cast();
        writer.interrupt = false;
        writer.status = LIBLRHSMM_RS_IO_ERROR;
        assert_eq!(liblrhsmm_rs_duration_write_stream_f64(owner, &sink), 3);
        writer.status = 0;
        writer.oversized = true;
        assert_eq!(liblrhsmm_rs_duration_write_stream_f64(owner, &sink), 3);
        writer.oversized = false;
        writer.zero = true;
        assert_eq!(liblrhsmm_rs_duration_write_stream_f64(owner, &sink), 3);
        sink.write = None;
        assert_eq!(liblrhsmm_rs_duration_write_stream_f64(owner, &sink), 3);
        sink.flush = None;
        assert_eq!(liblrhsmm_rs_write_stream_flush(&sink), 3);
        assert_eq!(liblrhsmm_rs_write_stream_flush(null()), 1);
        assert_eq!(liblrhsmm_rs_duration_write_stream_f64(owner, null()), 1);
        assert_eq!(liblrhsmm_rs_duration_write_bytes_f64(owner, null_mut()), 1);
        assert_eq!(liblrhsmm_rs_decode_limits_default(null_mut()), 1);
        assert_eq!(liblrhsmm_rs_bytes_release(&mut bytes), 0);
        assert_eq!(liblrhsmm_rs_duration_release_f64(&mut owner), 0);
    }
}

#[test]
fn binary64_duration_output_retains_original_binary32_wire_rounding() {
    // SAFETY: Independent initialized storage and uniquely released owners.
    unsafe {
        let descriptor = LiblrhsmmRsDurationParametersF64 {
            mean: 1.0 + 2.0_f64.powi(-25),
            variance: 1.0 + 3.0 * 2.0_f64.powi(-25),
            minimum: -1,
            maximum: -1,
            fixed_mean: -1,
            variance_floor: -0.0,
        };
        let mut owner = null_mut();
        assert_eq!(liblrhsmm_rs_duration_create_f64(&descriptor, &mut owner), 0);
        let mut bytes = null_mut();
        assert_eq!(liblrhsmm_rs_duration_write_bytes_f64(owner, &mut bytes), 0);
        assert_eq!(
            copy(bytes),
            include_bytes!("../../../tests/fixtures/c-double-wire-duration.bin")
        );
        assert_eq!(liblrhsmm_rs_bytes_release(&mut bytes), 0);
        assert_eq!(liblrhsmm_rs_duration_release_f64(&mut owner), 0);
    }
}
