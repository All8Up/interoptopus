use crate::patterns::result::ErrorREMOVEME;
use crate::types::basic::{Tupled, Vec3f32};
use crate::types::bool::BoolField;
use interoptopus::ffi_function;
use interoptopus::pattern::result::Result;

#[ffi_function]
pub fn struct1(x: Tupled) -> Tupled {
    Tupled(x.0 * 2)
}

#[ffi_function]
pub fn struct2(_a: Vec3f32, _b: Option<&Tupled>) -> Result<(), ErrorREMOVEME> {
    Result::Ok(())
}

#[ffi_function]
pub fn struct3(x: BoolField) -> bool {
    x.val
}
