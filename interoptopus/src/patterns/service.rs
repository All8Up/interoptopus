//! Bundles function with common receiver into a `class` or service in object oriented languages.<sup>🚧</sup>
//!
//! Services are defined via the [**pattern_service_manual**](crate::pattern_service_manual) macro and consists of the following items:
//!
//! - an [opaque](OpaqueType) **receiver** type,
//! - a **constructor** function of signature `fn(mmR, ...) -> E`
//! - **destructor** function of signature `unsafe fn(mmR) -> E`
//! - arbitrary many **methods** of signature `fn(mR, ...) -> ?`
//!
//! where
//!
//! - `mmR` refers to a `&mut *mut Receiver`, `*mut *mut Receiver` or `Option<&mut *mut Receiver>`,
//! - `mR` refers to a `&mut Receiver`, `*mut Receiver` or `Option<&mut Receiver>`,
//! - `E` to a [success enum](crate::patterns::success_enum),
//! - `...` ane `?` to arbitrary types.
//!
//! The constructor:
//! - must return `E::SUCCESS` if and only if it successfully wrote an instance of `Receiver` into `mmR`
//! - may only write a valid instance or leave the parameter at `null`
//!
//! The `unsafe` destructor:
//! - must handle any values produced by a constructor that successfully returned,
//! - if it successfully completes, it must write a null pointer into `mmR`,
//!
//! Violation of any of these conditions may cause UB in FFI calls as the generated bindings rely on this contract.
//! In addition the bindings will guarantee (and the destructor's signature should indicate) that it
//!
//! - will only invoked if the constructor returned `E::SUCCESS`,
//! - the value for `mmR` will be exactly the value returned by a previous constructor invocation,

use crate::lang::c::{CType, Function, OpaqueType};
use crate::patterns::success_enum::Success;
use crate::patterns::TypePattern;

macro_rules! impl_failure_default {
    ($t:ty, $x:expr) => {
        impl FailureDefault for $t {
            fn failure_default() -> Self {
                $x
            }
        }
    };
}

/// For types that can be returned by a generated service what to return when things went wrong?
pub trait FailureDefault {
    fn failure_default() -> Self;
}

impl<T> FailureDefault for T
where
    T: Success,
{
    fn failure_default() -> Self {
        T::NULL
    }
}

impl_failure_default!(u8, 0);
impl_failure_default!(u16, 0);
impl_failure_default!(u32, 0);
impl_failure_default!(u64, 0);
impl_failure_default!(i8, 0);
impl_failure_default!(i16, 0);
impl_failure_default!(i32, 0);
impl_failure_default!(i64, 0);
impl_failure_default!(f32, f32::NAN);
impl_failure_default!(f64, f64::NAN);
impl_failure_default!((), ());

/// Combines a receiver, constructor, destructor and multiple methods in one entity.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Service {
    the_type: OpaqueType,
    constructor: Function,
    destructor: Function,
    methods: Vec<Function>,
}

impl Service {
    pub fn new(constructor: Function, destructor: Function, methods: Vec<Function>) -> Self {
        let the_type = extract_obvious_opaque_from_parameter(
            constructor
                .signature()
                .params()
                .first()
                .expect("Constructor must have at least one parameter")
                .the_type(),
        )
        .expect("First parameter must point to opaque.");

        Self {
            the_type,
            constructor,
            destructor,
            methods,
        }
    }

    /// Checks if the signature of this service is compatible with the `Service` pattern, panic with
    /// error message otherwise.
    ///
    /// This function is mainly called during compile time therefore panicking with a good error
    /// message is beneficial.
    pub fn assert_valid(&self) {
        let constructor_fist_parameter = self
            .constructor
            .signature()
            .params()
            .get(0)
            .expect("Constructor for must have at least one parameter.");

        match &constructor_fist_parameter.the_type() {
            CType::ReadWritePointer(x) => match **x {
                CType::ReadWritePointer(ref x) => match **x {
                    CType::Opaque(_) => {}
                    _ => panic!("First parameter must be opaque type"),
                },
                _ => panic!("First parameter must be opaque type"),
            },
            CType::Opaque(_) => {}
            _ => panic!("First parameter must be RwPointer(RwPointer(Opaque)) type"),
        }

        let destructor_first_parameter = self
            .destructor
            .signature()
            .params()
            .get(0)
            .expect("Constructor for must have at least one parameter.");

        match &destructor_first_parameter.the_type() {
            CType::ReadWritePointer(x) => match **x {
                CType::ReadWritePointer(ref x) => match **x {
                    CType::Opaque(_) => {}
                    _ => panic!("First parameter must be opaque type"),
                },
                _ => panic!("First parameter must be opaque type"),
            },
            CType::Opaque(_) => {}
            _ => panic!("First parameter must be RwPointer(RwPointer(Opaque)) type"),
        }

        match self.constructor.signature().rval() {
            CType::Pattern(TypePattern::SuccessEnum(_)) => {}
            _ => panic!("Constructor must return a success enum."),
        }

        match self.destructor.signature().rval() {
            CType::Pattern(TypePattern::SuccessEnum(_)) => {}
            _ => panic!("Destructor must return a success enum."),
        }
    }

    pub fn the_type(&self) -> &OpaqueType {
        &self.the_type
    }

    pub fn constructor(&self) -> &Function {
        &self.constructor
    }

    pub fn destructor(&self) -> &Function {
        &self.destructor
    }

    pub fn methods(&self) -> &[Function] {
        &self.methods
    }
}

/// Defines a [`Service`] pattern, usually producing a class in OO languages.
///
/// ```ignore
/// pattern_service_manual!(
///     my_service_pattern_context,
///     types::Context,
///     functions::pattern_service_create,
///     functions::pattern_service_destroy
///     [
///         functions::pattern_service_method,
///     ]
/// );
/// ```
#[macro_export]
macro_rules! pattern_service_manual {
    (
        $pattern_name:ident,
        $constructor:path,
        $destructor:path,
        [$(
            $method:path
        ),*]
    ) => {
        pub(crate) fn $pattern_name() -> interoptopus::patterns::service::Service {
            use interoptopus::lang::rust::CTypeInfo;
            use interoptopus::lang::rust::FunctionInfo;

            let mut methods = Vec::new();

            {
                $({
                    use $method as x;
                    methods.push(x::function_info());
                })*
            }

            let ctor = {
                use $constructor as x;
                x::function_info()
            };

            let dtor = {
                use $destructor as x;
                x::function_info()
            };

            let rval = interoptopus::patterns::service::Service::new(
                ctor, dtor, methods
            );

            rval.assert_valid();
            rval
        }
    };
}

/// Defines a [`Service`] pattern _and_ generate FFI wrapper code.
#[macro_export]
macro_rules! pattern_service_generated {
    (
        $export_function:ident,
        $opaque:ty,
        $ctor:ident($($ctor_param:ident: $ctor_param_type:ty),*) -> $ctor_error:ty: $method_ctor:ident,
        $dtor:ident() -> $dtor_error:ty,
        [
            $(
                $method_as_fn:ident($self_ty:ty $(, $param:ident: $param_type:ty)*) -> $t:ty: $method:ident
            ),*
        ],
        [
            $(
                $manual_method:ident
            ),*
        ]
    ) => {
        #[interoptopus::ffi_function]
        #[no_mangle]
        pub extern "C" fn $ctor(context_ptr: Option<&mut *mut $opaque>, $( $ctor_param: $ctor_param_type),*) -> $ctor_error {
            if let Some(context) = context_ptr {
                let raw = std::panic::catch_unwind(|| {
                    let boxed = Box::new(<$opaque>::$method_ctor($($ctor_param),*));
                    Box::into_raw(boxed)
                });

                if let Ok(raw) = raw {
                    *context = raw;
                } else {
                    return <$ctor_error as interoptopus::patterns::success_enum::Success>::PANIC;
                }

                <$ctor_error as interoptopus::patterns::success_enum::Success>::SUCCESS
            } else {
                <$ctor_error as interoptopus::patterns::success_enum::Success>::NULL
            }
        }

        #[interoptopus::ffi_function]
        #[no_mangle]
        pub extern "C" fn $dtor(context_ptr: Option<&mut *mut $opaque>) -> $dtor_error {
            if let Some(context) = context_ptr {

                {
                    unsafe { Box::from_raw(*context) };
                }

                *context = std::ptr::null_mut();

                <$dtor_error as interoptopus::patterns::success_enum::Success>::SUCCESS
            } else {
                <$dtor_error as interoptopus::patterns::success_enum::Success>::NULL
            }
        }

        $(
            #[interoptopus::ffi_function]
            #[no_mangle]
            pub extern "C" fn $method_as_fn(context_ptr: Option<$self_ty>, $( $param: $param_type),* ) -> $t {
                if let Some(context) = context_ptr {

                    if let Ok(rval) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        <$opaque>::$method(context, $($param),*)
                    })) {
                        rval.into()
                    } else {
                        return < $t as interoptopus::patterns::service::FailureDefault > :: failure_default();
                    }

                } else {
                    < $t as interoptopus::patterns::service::FailureDefault > :: failure_default()
                }
            }
        )*


        // Generate export function
        interoptopus::pattern_service_manual!($export_function, $ctor, $dtor, [
            $($method_as_fn),*
            $(,$manual_method)*
        ]);
    };
}

/// Walks the type until it finds the first "obvious" Opaque.
///
/// An Opaque is obvious if it is at a singular position (e.g., `*const Opaque`),
/// but not within the fields of a struct.
fn extract_obvious_opaque_from_parameter(param: &CType) -> Option<OpaqueType> {
    match param {
        CType::Primitive(_) => None,
        CType::Enum(_) => None,
        CType::Opaque(x) => Some(x.clone()),
        CType::Composite(_) => None,
        CType::FnPointer(_) => None,
        CType::ReadPointer(x) => extract_obvious_opaque_from_parameter(x),
        CType::ReadWritePointer(x) => extract_obvious_opaque_from_parameter(x),
        CType::Pattern(_) => None,
    }
}
