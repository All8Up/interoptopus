use interoptopus::ffi_type;
use interoptopus::lang::c::{CType, CompositeType, Field, Function, PrimitiveType};
use interoptopus::lang::rust::{CTypeInfo, CallbackXY, FunctionInfo};
use interoptopus::patterns::ascii_pointer::AsciiPointer;
use interoptopus::patterns::slice::FFISlice;
use interoptopus::patterns::success_enum::Success;
use std::marker::PhantomData;

// Let's assume we can't implement `CTypeInfo` for this.
#[repr(C)]
pub struct SomeForeignType {
    x: u32,
}

// Surrogate we can use instead of `SomeForeignType`
pub fn some_foreign_type() -> CType {
    let mut composite = CompositeType::new("SomeForeignType".to_string());
    composite.add_field(Field::new("x".to_string(), CType::Primitive(PrimitiveType::U32)));
    CType::Composite(composite)
}

#[ffi_type]
#[repr(C)]
pub struct Empty {}

#[ffi_type(opaque)]
pub struct Opaque {
    _internal: *const Vec3f32,
}

#[ffi_type]
#[repr(C)]
pub struct Generic<'a, T>
where
    T: 'static,
    T: CTypeInfo,
{
    pub x: &'a T,
}

#[ffi_type(skip(p))]
#[repr(C)]
pub struct Phantom<'a, T>
where
    T: 'static,
    T: CTypeInfo,
{
    pub x: u32,
    pub p: PhantomData<&'a T>,
}

#[ffi_type]
#[repr(C)]
pub struct Vec3f32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[ffi_type(surrogates(foreign1 = "some_foreign_type", foreign2 = "some_foreign_type"))]
#[repr(C)]
pub struct Container {
    pub foreign1: SomeForeignType,
    pub foreign2: SomeForeignType,
}

#[ffi_type(patterns(success_enum))]
#[repr(C)]
pub enum FFIError {
    Ok = 0,
    Null = 100,
    Fail = 200,
}

impl Success for FFIError {
    const SUCCESS: Self = Self::Ok;
}

/// Documented enum.
#[ffi_type]
#[repr(C)]
pub enum EnumDocumented {
    /// Variant A.
    A,
    /// Variant B.
    B,
}

/// Documented struct.
#[ffi_type]
#[repr(C)]
pub struct StructDocumented {
    /// Documented field.
    pub x: f32,
}

#[ffi_type]
#[repr(C)]
pub struct UseAsciiStringPattern<'a> {
    pub ascii_string: AsciiPointer<'a>,
}

// Used for the `context_init(**Context)`, `context_use(*Context)`,
// `context_destroy(**Context)` pattern that acts like constructors / methods / destructors.
#[ffi_type(opaque)]
#[repr(C)]
pub struct Context {
    pub(crate) some_field: u32,
}

// Doesn't need annotations.
pub type Callbacku8u8 = extern "C" fn(u8) -> u8;

// This does not work since we can't express the for<'x> bounds in our CTypeInfo implementation.
// pub type CallbackFFISlice = extern "C" fn(FFISlice<u8>) -> u8;

pub type CallbackFFISlice<'a> = CallbackXY<FFISlice<'a, u8>, u8>;

pub mod ambiguous1 {
    use interoptopus::ffi_type;

    #[ffi_type(name = "Vec1")]
    #[repr(C)]
    pub struct Vec {
        pub x: f32,
        pub y: f32,
    }

    #[ffi_type(name = "Status1")]
    #[repr(C)]
    pub enum Status {
        X = 1,
        Y = 2,
    }
}

pub mod ambiguous2 {
    use interoptopus::ffi_type;

    #[ffi_type(name = "Vec2")]
    #[repr(C)]
    pub struct Vec {
        pub x: f64,
        pub z: f64,
    }

    #[ffi_type(name = "Status2")]
    #[repr(C)]
    pub enum Status {
        X = 100,
        Z = 200,
    }
}
