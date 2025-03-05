/// Register a function with an [`InventoryBuilder`].
///
/// You must also annotate the function with [`#[ffi_function]`](crate::ffi_function).
///
/// # Example
///
/// ```rust
/// use interoptopus::{ffi_function, Inventory, InventoryBuilder, function};
///
/// #[ffi_function]
/// pub fn my_function() { }
///
/// pub fn inventory() -> Inventory {
///     InventoryBuilder::new()
///         .register(function!(my_function))
///         .validate()
///         .build()
/// }
/// ```
#[macro_export]
macro_rules! function {
    ($x:ty) => {{
        use $crate::lang::rust::FunctionInfo;
        // use $x as fnc;
        let info = <$x>::function_info();
        $crate::Symbol::Function(info)
    }};
}

/// Register an extra type with an [`InventoryBuilder`].
///
/// You must also annotate the type with [`#[ffi_type]`](crate::ffi_type) and `#[repr(C)]`.
///
/// Note, most types are registered automatically. You only need this to pass types not directly visible in
/// your function signatures, e.g., when accepting multiple types via a void pointer.
///
/// # Example
///
/// ```rust
/// use interoptopus::{ffi_type, Inventory, InventoryBuilder, extra_type};
///
/// #[ffi_type]
/// pub struct S<T> {
///     t: T
/// };
///
/// pub fn inventory() -> Inventory {
///     InventoryBuilder::new()
///         .register(extra_type!(S<f32>))
///         .validate()
///         .build()
/// }
#[macro_export]
macro_rules! extra_type {
    ($x:ty) => {{
        let info = <$x as $crate::lang::rust::CTypeInfo>::type_info();
        $crate::Symbol::Type(info)
    }};
}

/// Register a pattern with an [`InventoryBuilder`].
///
/// You only need to register [`LibraryPattern`](crate::patterns::LibraryPattern), as [`TypePattern`](crate::patterns::TypePattern) are detected automatically.
///
/// # Example
///
/// Note, as this example focuses on the `pattern` macro it omits the definition of `Error` and `MyFFIError`.
/// Their implementation can be found in the [`FFIError`](crate::patterns::result::FFIError) example.
///
/// ```rust
/// use interoptopus::{ffi_type, ffi_service, ffi_service_ctor, Inventory, InventoryBuilder, pattern};
///
/// # use std::fmt::{Display, Formatter};
/// # use interoptopus::patterns::result::FFIError;
/// #
/// # #[derive(Debug)]
/// # pub enum Error {
/// #     Bad,
/// # }
/// #
/// # impl Display for Error {
/// #     fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
/// #         Ok(())
/// #     }
/// # }
/// #
/// # impl std::error::Error for Error {}
/// #
/// # #[ffi_type(error)]
/// # pub enum MyFFIError {
/// #     Ok = 0,
/// #     NullPassed = 1,
/// #     Panic = 2,
/// #     OtherError = 3,
/// # }
/// #
/// # impl FFIError for MyFFIError {
/// #     const SUCCESS: Self = Self::Ok;
/// #     const NULL: Self = Self::NullPassed;
/// #     const PANIC: Self = Self::Panic;
/// # }
/// #
/// # impl From<Error> for MyFFIError {
/// #     fn from(x: Error) -> Self {
/// #         match x {
/// #             Error::Bad => Self::OtherError,
/// #         }
/// #     }
/// # }
/// #
///
/// #[ffi_type(opaque)]
/// pub struct SimpleService {
///     pub some_value: u32,
/// }
///
/// #[ffi_service(error = "MyFFIError", prefix = "simple_service_")]
/// impl SimpleService {
///
///     #[ffi_service_ctor]
///     pub fn new_with(some_value: u32) -> Result<Self, Error> {
///         Ok(Self { some_value })
///     }
/// }
///
/// pub fn inventory() -> Inventory {
///     InventoryBuilder::new()
///         .register(pattern!(SimpleService))
///         .validate()
///         .build()
/// }
#[macro_export]
macro_rules! pattern {
    ($x:path) => {{
        let info: $crate::patterns::LibraryPattern = <$x as $crate::patterns::LibraryPatternInfo>::pattern_info();
        $crate::Symbol::Pattern(info)
    }};
}

/// Register a constant with an [`InventoryBuilder`].
///
/// You must also annotate the constant with [`#[ffi_constant]`](crate::ffi_constant).
///
/// # Example
///
/// ```rust
/// use interoptopus::{ffi_constant, Inventory, InventoryBuilder, constant};
///
/// #[ffi_constant]
/// pub const MY_CONSTANT: u32 = 123;
///
/// pub fn inventory() -> Inventory {
///     InventoryBuilder::new()
///         .register(constant!(MY_CONSTANT))
///         .validate()
///         .build()
/// }
/// ```
#[macro_export]
macro_rules! constant {
    ($x:path) => {{
        use $crate::lang::rust::ConstantInfo;
        use $x as constant;
        let info = constant::constant_info();
        $crate::Symbol::Constant(info)
    }};
}
