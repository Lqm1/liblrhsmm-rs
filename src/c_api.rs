//! Opt-in C ABI. Unsafe code is confined to this module and its children.
//! Status zero means success, one invalid pointer, two invalid range,
//! three invalid configuration/calculation and four a caught unwinding panic.
//! Pointer validation cannot establish allocation validity or exclusive access.
//! Floating-point results use output storage to avoid i686 float-return rounding.
#![deny(unsafe_op_in_unsafe_fn)]

mod bytes;
pub use bytes::*;
mod io;
pub use io::{
    LIBLRHSMM_RS_IO_ERROR, LIBLRHSMM_RS_IO_INTERRUPTED, LIBLRHSMM_RS_IO_SUCCESS,
    LiblrhsmmRsReadStream, LiblrhsmmRsWriteStream, liblrhsmm_rs_write_stream_flush,
};
#[cfg(feature = "serialization")]
mod duration_serial;
#[cfg(feature = "serialization")]
pub use duration_serial::*;

#[cfg(feature = "serialization")]
mod gaussian_serial;
#[cfg(feature = "serialization")]
pub use gaussian_serial::*;

#[cfg(feature = "serialization")]
mod stream_serial;
#[cfg(feature = "serialization")]
pub use stream_serial::*;

#[cfg(feature = "serialization")]
mod model_serial;
#[cfg(feature = "serialization")]
pub use model_serial::*;

mod duration_statistics;
pub use duration_statistics::*;

mod prepared_duration;
pub use prepared_duration::*;

mod gaussian_statistics;
pub use gaussian_statistics::*;

mod prepared_gaussian;
pub use prepared_gaussian::*;

mod model;
pub use model::*;
mod model_builder;
pub use model_builder::*;

mod stream;
pub use stream::*;
mod stream_builder;
pub use stream_builder::*;

mod gaussian;
pub use gaussian::*;

mod duration;
pub use duration::*;

mod numerical;
pub use numerical::{
    LIBLRHSMM_RS_EXPONENTIAL_TABLE, LIBLRHSMM_RS_LOG_ADD_TABLE, LiblrhsmmRsNumericalConstants,
};
mod numerical_functions;
use crate::ModelError;
pub use numerical_functions::*;
use std::panic::{UnwindSafe, catch_unwind};

fn range<T>(pointer: *const T, count: usize) -> Result<(), u32> {
    if count > i32::MAX as usize || count > isize::MAX as usize / size_of::<T>() {
        return Err(2);
    }
    if count != 0 && (pointer.is_null() || !pointer.is_aligned()) {
        return Err(1);
    }
    Ok(())
}
fn boundary<T>(operation: impl FnOnce() -> Result<T, ModelError> + UnwindSafe) -> Result<T, u32> {
    match catch_unwind(operation) {
        Ok(Ok(value)) => Ok(value),
        Ok(Err(_)) => Err(3),
        Err(payload) => {
            std::mem::forget(payload);
            Err(4)
        }
    }
}
unsafe fn scalar_result<T: Copy>(
    output: *mut T,
    operation: impl FnOnce() -> Result<T, ModelError> + UnwindSafe,
) -> u32 {
    if let Err(status) = range(output.cast_const(), 1) {
        return status;
    }
    match boundary(operation) {
        Ok(value) => {
            // SAFETY: Caller supplies aligned exclusively writable output storage.
            unsafe { output.write(value) };
            0
        }
        Err(status) => status,
    }
}
/// First revision of this package's additive C interface.
#[unsafe(no_mangle)]
pub extern "C" fn liblrhsmm_rs_abi_version() -> u32 {
    1
}

#[cfg(test)]
mod tests {
    #[test]
    fn boundary_does_not_drop_a_panicking_panic_payload() {
        struct Payload;
        impl Drop for Payload {
            fn drop(&mut self) {
                panic!("caught payload must not be dropped at the C boundary");
            }
        }
        assert_eq!(
            super::boundary::<()>(|| std::panic::panic_any(Payload)),
            Err(4)
        );
        assert_eq!(
            super::boundary::<()>(|| Err(crate::ModelError("test calculation failure"))),
            Err(3)
        );
    }
}
