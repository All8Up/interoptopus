//! Like a regular [`Option`] but FFI safe.
use crate::lang::c::{CType, CompositeType, Field, PrimitiveType};
use crate::lang::rust::CTypeInfo;

use crate::patterns::TypePattern;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An option-like type at the FFI boundary where a regular [`Option`] doesn't work.
///
/// # C API
///
/// The option will be considered `Some` if and only if `is_some` is `1`. All
/// other values mean `None`.
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Debug, Copy, Clone, PartialEq, Default, Deserialize, Serialize))]
#[cfg_attr(not(feature = "serde"), derive(Debug, Copy, Clone, PartialEq, Default))]
pub struct FFIOption<T> {
    t: T,
    is_some: u8,
}

impl<T> FFIOption<T> {
    pub const fn some(data: T) -> Self {
        Self { is_some: 1, t: data }
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn into_option(self) -> Option<T> {
        match self.is_some {
            1 => Option::Some(self.t),
            _ => Option::None,
        }
    }

    pub const fn as_ref(&self) -> Option<&T> {
        match self.is_some {
            1 => Option::Some(&self.t),
            _ => Option::None,
        }
    }

    pub fn as_mut(&mut self) -> Option<&mut T> {
        match self.is_some {
            1 => Option::Some(&mut self.t),
            _ => Option::None,
        }
    }

    pub const fn is_some(&self) -> bool {
        self.is_some == 1
    }

    pub const fn is_none(&self) -> bool {
        !self.is_some()
    }

    /// Get the value or panic.
    ///
    /// # Panics
    ///
    /// Panics if the value is `None`.
    #[track_caller]
    pub fn unwrap(self) -> T {
        if self.is_some == 1 {
            self.t
        } else {
            panic!("Trying to unwrap None value");
        }
    }

    /// Get the value as a mutable reference or panic.
    ///
    /// # Panics
    ///
    /// Panics if the value is `None`.
    #[track_caller]
    pub fn unwrap_as_mut(&mut self) -> &mut T {
        if self.is_some == 1 {
            &mut self.t
        } else {
            panic!("Trying to unwrap None value");
        }
    }
}

impl<T: Default> FFIOption<T> {
    pub fn none() -> Self {
        Self { is_some: 0, t: T::default() }
    }
}

impl<T: Default> From<Option<T>> for FFIOption<T> {
    fn from(option: Option<T>) -> Self {
        match option {
            Option::None => Self::none(),
            Option::Some(t) => Self::some(t),
        }
    }
}

impl<T> CTypeInfo for FFIOption<T>
where
    T: CTypeInfo,
{
    fn type_info() -> CType {
        let fields = vec![
            Field::new("t".to_string(), T::type_info()),
            Field::new("is_some".to_string(), CType::Primitive(PrimitiveType::U8)),
        ];

        let composite = CompositeType::new(format!("FFIOption{}", T::type_info().name_within_lib()), fields);
        CType::Pattern(TypePattern::Option(composite))
    }
}

#[cfg(test)]
mod test {
    use crate::patterns::option::FFIOption;

    #[test]
    fn can_create() {
        assert!(FFIOption::some(100).is_some());
        assert!(FFIOption::<u8>::none().is_none());
    }
}
