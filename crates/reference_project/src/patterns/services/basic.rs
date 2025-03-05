use crate::patterns::result::{Error, FFIError};
use interoptopus::{ffi_service, ffi_service_ctor, ffi_type};

#[ffi_type(opaque)]
pub struct ServiceBasic {}

#[ffi_service(error = "FFIError")]
impl ServiceBasic {
    #[ffi_service_ctor]
    pub fn new() -> Result<Self, Error> {
        Ok(Self {})
    }
}
