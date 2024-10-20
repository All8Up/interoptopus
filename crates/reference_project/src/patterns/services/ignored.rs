use crate::patterns::result::{Error, FFIError};
use interoptopus::{ffi_service, ffi_service_ctor, ffi_service_method, ffi_type};

#[ffi_type(opaque)]
pub struct ServiceIgnoringMethods {}

#[ffi_service(error = "FFIError")]
impl ServiceIgnoringMethods {
    #[ffi_service_ctor]
    pub fn new() -> Result<Self, Error> {
        Ok(Self {})
    }

    #[ffi_service_method(ignore)]
    pub fn this_is_ignored(&mut self) -> Result<(), Error> {
        Ok(())
    }

    /// No FFI bindings are generated for non-pub methods.
    #[allow(unused)]
    fn not_exposed<T>(&mut self, _: T) -> Result<(), Error> {
        Ok(())
    }
}
