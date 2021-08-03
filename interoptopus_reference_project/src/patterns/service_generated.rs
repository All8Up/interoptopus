use crate::patterns::success_enum::FFIError;
use interoptopus::patterns::primitives::FFIBool;
use interoptopus::patterns::service::ServiceReturn;
use interoptopus::patterns::slice::{FFISlice, FFISliceMut};
use interoptopus::{ffi_function, ffi_service, pattern_service_generated};
use some_rust_module::{Error, SimpleService};

pub mod some_rust_module {
    use crate::patterns::success_enum::FFIError;
    use interoptopus::patterns::primitives::FFIBool;
    use interoptopus::patterns::slice::{FFISlice, FFISliceMut};
    use interoptopus::{ffi_function, ffi_service, ffi_service_ctor, ffi_service_method, ffi_type};
    use std::fmt::{Display, Formatter};

    // An error we use in a Rust library
    #[derive(Debug)]
    pub enum Error {
        Bad,
    }

    impl Display for Error {
        fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
            Ok(())
        }
    }

    impl std::error::Error for Error {}

    // Some struct we want to expose as a class.
    #[ffi_type(opaque)]
    #[derive(Default)]
    pub struct SimpleService {
        pub some_value: u32,
    }

    // Regular implementation of methods.
    #[ffi_service(debug, error = "FFIError")]
    impl SimpleService {
        /// The constructor must return a `Result<Self, Error>`.
        #[ffi_service_ctor]
        pub fn new_with(some_value: u32) -> Result<Self, Error> {
            Ok(Self { some_value })
        }

        /// Methods returning a Result<(), _> are the default and do not
        /// need annotations.
        pub fn method_result(&self, _: u32) -> Result<(), Error> {
            Ok(())
        }

        #[ffi_service_method(direct)]
        pub fn method_value(&self, x: u32) -> u32 {
            x
        }

        /// This method should be documented.
        ///
        /// Multiple lines.
        #[ffi_service_method(direct)]
        pub fn method_void(&self) {}

        #[ffi_service_method(direct)]
        pub fn method_mut_self(&mut self, slice: FFISlice<u8>) -> u8 {
            *slice.as_slice().get(0).unwrap_or(&0)
        }

        #[ffi_service_method(direct)]
        pub fn method_mut_self_void(&mut self, _slice: FFISlice<FFIBool>) {}

        #[ffi_service_method(direct)]
        pub fn method_mut_self_ref(&mut self, x: &u8, _y: &mut u8) -> u8 {
            *x
        }

        #[ffi_service_method(direct)]
        pub fn method_mut_self_ref_slice(&mut self, x: &u8, _y: &mut u8, _slice: FFISlice<u8>) -> u8 {
            *x
        }

        #[ffi_service_method(direct)]
        pub fn method_mut_self_ref_slice_limited<'a, 'b>(&mut self, x: &u8, _y: &mut u8, _slice: FFISlice<'a, u8>, _slice2: FFISlice<'b, u8>) -> u8 {
            *x
        }

        #[ffi_service_method(direct)]
        pub fn method_mut_self_ffi_error(&mut self, _slice: FFISliceMut<u8>) -> FFIError {
            FFIError::Ok
        }
    }
}

#[ffi_function]
#[no_mangle]
pub fn simple_service_ext_util(_ptr: &SimpleService) {}

// Needed for Error to FFIError conversion.
impl<T> From<Result<T, Error>> for FFIError {
    fn from(x: Result<T, Error>) -> Self {
        match x {
            Ok(_) => Self::Ok,
            Err(Error::Bad) => Self::Fail,
        }
    }
}

/// An extra exposed method.
#[ffi_function]
#[no_mangle]
pub extern "C" fn simple_service_extra_method(_context: Option<&mut SimpleService>) -> u32 {
    0
}

// Generate all FFI helpers.
// pattern_service_generated!(
//   simple_service_pattern,
//   SimpleService,
//   simple_service_create(x: u32) -> FFIError: new_with,
//   simple_service_destroy() -> FFIError,
//   [
//       simple_service_result(&mut SimpleService, x: u32) -> FFIError: method_result
//   ],
//   [
//       simple_service_value(&mut SimpleService, x: u32) -> u32: method_value,
//       simple_service_mut_self(&mut SimpleService, slice: FFISlice<u8>) -> u8: method_mut_self,
//       simple_service_mut_self_void(&mut SimpleService, slice: FFISlice<FFIBool>) -> (): method_mut_self_void,
//       simple_service_mut_self_ref(&mut SimpleService, x: &u8, _y: &mut u8) -> u8: method_mut_self_ref,
//       simple_service_mut_self_ref_slice(&mut SimpleService, x: &u8, _y: &mut u8, _slice: FFISlice<u8>) -> u8: method_mut_self_ref_slice,
//       simple_service_mut_self_ref_slice_limited<'a, 'b>(&mut SimpleService, x: &u8, _y: &mut u8, _slice: FFISlice<'a, u8>, _slice2: FFISlice<'b, u8>) -> u8: method_mut_self_ref_slice_limited,
//       simple_service_mut_self_ffi_error(&mut SimpleService, slice: FFISliceMut<u8>) -> FFIError: method_mut_self_ffi_error,
//       simple_service_void(&SimpleService) -> (): method_void
//   ],
//   [
//       simple_service_extra_method
//   ]
// );
