//! Useful when `extern "C" fn()` delegate types give compile errors.

use crate::lang::c::{CType, FnPointerType, FunctionSignature, Parameter};
use crate::lang::rust::CTypeInfo;

/// A `fn(X) -> Y` callback.
#[repr(transparent)]
pub struct CallbackXY<X0, R> {
    ptr: extern "C" fn(X0) -> R,
}

impl<X0, R> CallbackXY<X0, R> {
    pub fn new(ptr: extern "C" fn(X0) -> R) -> Self {
        Self { ptr }
    }

    pub fn call(&self, x0: X0) -> R {
        (self.ptr)(x0)
    }
}

unsafe impl<X0, R> CTypeInfo for CallbackXY<X0, R>
where
    X0: CTypeInfo,
    R: CTypeInfo,
{
    fn type_info() -> CType {
        let sig = FunctionSignature::new(vec![Parameter::new("x0".to_string(), X0::type_info())], R::type_info());
        CType::FnPointer(FnPointerType::new(sig))
    }
}

/// A `fn(X1, X2) -> Y` callback.
#[repr(transparent)]
pub struct CallbackXXY<X0, X1, R> {
    ptr: extern "C" fn(X0, X1) -> R,
}

impl<X0, X1, R> CallbackXXY<X0, X1, R> {
    pub fn new(ptr: extern "C" fn(X0, X1) -> R) -> Self {
        Self { ptr }
    }

    pub fn call(&self, x0: X0, x1: X1) -> R {
        (self.ptr)(x0, x1)
    }
}

unsafe impl<X0, X1, R> CTypeInfo for CallbackXXY<X0, X1, R>
where
    X0: CTypeInfo,
    X1: CTypeInfo,
    R: CTypeInfo,
{
    fn type_info() -> CType {
        let sig = FunctionSignature::new(
            vec![Parameter::new("x0".to_string(), X0::type_info()), Parameter::new("x1".to_string(), X1::type_info())],
            R::type_info(),
        );
        CType::FnPointer(FnPointerType::new(sig))
    }
}

unsafe impl<X0, X1, R> CTypeInfo for Option<CallbackXXY<X0, X1, R>>
where
    X0: CTypeInfo,
    X1: CTypeInfo,
    R: CTypeInfo,
{
    fn type_info() -> CType {
        let sig = FunctionSignature::new(
            vec![Parameter::new("x0".to_string(), X0::type_info()), Parameter::new("x1".to_string(), X1::type_info())],
            R::type_info(),
        );
        CType::FnPointer(FnPointerType::new(sig))
    }
}

#[cfg(test)]
mod test {
    use crate::patterns::callbacks::CallbackXY;

    extern "C" fn f(x: u32) -> u32 {
        x + x
    }

    #[test]
    fn can_create() {
        let callback = CallbackXY::new(f);

        assert_eq!(callback.call(100), 200);
    }
}
