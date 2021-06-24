use interoptopus::ffi_type;
use interoptopus::patterns::success_enum::Success;

#[ffi_type(patterns(success_enum))]
#[repr(C)]
pub enum FFIError {
    Ok = 0,
    Null = 100,
    Fail = 200,
}

impl Success for FFIError {
    const SUCCESS: Self = Self::Ok;
    const NULL: Self = Self::Null;
}
