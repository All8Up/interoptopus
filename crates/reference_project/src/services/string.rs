use crate::patterns::callback::StringCallback;
use crate::patterns::result::ErrorREMOVEME;
use interoptopus::{ffi, ffi_service, ffi_type};

/// Some struct we want to expose as a class.
#[ffi_type(opaque)]
pub struct ServiceStrings {}

// Regular implementation of methods.
#[ffi_service]
impl ServiceStrings {
    pub fn new() -> ffi::Result<Self, ErrorREMOVEME> {
        ffi::Result::Ok(Self {})
    }

    pub fn pass_cstr(&mut self, _: ffi::CStrPointer) {}

    pub fn return_cstr(&mut self) -> ffi::CStrPointer {
        ffi::CStrPointer::empty()
    }

    pub fn callback_string(&self, s: ffi::String, cb: StringCallback) {
        cb.call(s.clone());
    }
}
