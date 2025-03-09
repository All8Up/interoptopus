//! Raw `*const char` pointer on C-level but a (ASCII) `string` like in supported languages.
//!
//! # Example
//!
//! In your library you can accept (ASCII- / C-) strings like this:
//!
//! ```
//! use interoptopus::ffi_function;
//! use interoptopus::patterns::string::CStrPointer;
//!
//! #[ffi_function]
//! #[no_mangle]
//! pub extern "C" fn call_with_string(s: CStrPointer) {
//!     //
//! }
//! ```
//!
//! Backends supporting this pattern might generate the equivalent to the following pseudo-code:
//!
//! ```csharp
//! void call_with_string(string s);
//! ```
//!
//! Backends not supporting this pattern, and C FFI, will see the equivalent of the following C code:
//! ```c
//! void call_with_string(uint8_t* s);
//! ```
//!
use crate::lang::c::{CType, CompositeType, Documentation, Field, Layout, Meta, PrimitiveType, Representation};
use crate::lang::rust::CTypeInfo;
use crate::patterns::TypePattern;
use crate::Error;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::mem::forget;
use std::option::Option::None;
use std::os::raw::c_char;
use std::ptr::null;

static EMPTY: &[u8] = b"\0";

/// Represents a `*const char` on FFI level pointing to an `0x0` terminated ASCII string.
///
/// # Antipattern
///
/// It's discouraged to use [`FFIOption`](crate::patterns::option::Option) with [`CStrPointer`]
/// and some backend might not generate proper bindings (like C#).
///
/// Instead use [`CStrPointer`] alone since it already has a pointer that's nullable.
/// In this case, [`CStrPointer::as_c_str()`] will return [`None`] and [`CStrPointer::as_str`]
/// will return an [`Error::Null`].
#[repr(transparent)]
#[derive(Debug)]
pub struct CStrPointer<'a> {
    ptr: *const c_char,
    _phantom: PhantomData<&'a ()>,
}

// Safety: `CStrPointer` is a transparent wrapper around a pointer. From Rust
//         we only allow safe construction, from interop it's up to the FFI caller.
unsafe impl Send for CStrPointer<'_> {}
unsafe impl Sync for CStrPointer<'_> {}

impl Default for CStrPointer<'_> {
    fn default() -> Self {
        Self { ptr: null(), _phantom: PhantomData::default() }
    }
}

impl<'a> CStrPointer<'a> {
    #[must_use]
    pub fn empty() -> Self {
        Self { ptr: EMPTY.as_ptr().cast(), _phantom: PhantomData::default() }
    }

    /// Create a `CStrPointer` from a `&[u8]` slice reference.
    ///
    /// The parameter `cstr_with_nul` must contain nul (`0x0`), but it does not need to contain nul
    /// at the end.
    ///
    /// # Errors
    /// Can fail if the string contains a nul.
    pub fn from_slice_with_nul(cstr_with_nul: &'a [u8]) -> Result<Self, Error> {
        // Check we actually contain one `0x0`.
        if !cstr_with_nul.contains(&0) {
            return Err(Error::NulTerminated);
        }

        // Can't do this, C# treats ASCII as extended and bytes > 127 might show up, which
        // is going to be a problem when returning a string we previously accepted.
        //
        // Any previous characters must not be extended ASCII.
        // if ascii_with_nul.iter().any(|x| *x > 127) {
        //     return Err(Error::Ascii);
        // }

        Ok(Self { ptr: cstr_with_nul.as_ptr().cast(), _phantom: PhantomData::default() })
    }

    /// Create a pointer from a `CStr`.
    #[must_use]
    pub fn from_cstr(cstr: &'a CStr) -> Self {
        Self { ptr: cstr.as_ptr(), _phantom: PhantomData::default() }
    }

    /// Create a [`CStr`] for the pointer.
    #[must_use]
    pub fn as_c_str(&self) -> Option<&'a CStr> {
        if self.ptr.is_null() {
            None
        } else {
            // TODO: Write something about safety
            unsafe { Some(CStr::from_ptr(self.ptr)) }
        }
    }

    /// Attempts to return a Rust `str`.
    ///
    /// # Errors
    /// Can fail if the string was null.
    pub fn as_str(&self) -> Result<&'a str, Error> {
        Ok(self.as_c_str().ok_or(Error::Null)?.to_str()?)
    }
}

unsafe impl CTypeInfo for CStrPointer<'_> {
    fn type_info() -> CType {
        CType::Pattern(TypePattern::CStrPointer)
    }
}

#[cfg(test)]
mod test {
    use crate::patterns::string::CStrPointer;
    use std::ffi::CString;

    #[test]
    fn can_create() {
        let s = "hello world";
        let cstr = CString::new(s).unwrap();

        let ptr_some = CStrPointer::from_cstr(&cstr);

        assert_eq!(s, ptr_some.as_str().unwrap());
    }

    #[test]
    fn from_slice_with_nul_works() {
        let s = b"hello\0world";
        let ptr_some = CStrPointer::from_slice_with_nul(&s[..]).unwrap();

        assert_eq!("hello", ptr_some.as_str().unwrap());
    }

    #[test]
    fn from_slice_with_nul_fails_if_not_nul() {
        let s = b"hello world";
        let ptr_some = CStrPointer::from_slice_with_nul(&s[..]);

        assert!(ptr_some.is_err());
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct String {
    ptr: *mut u8,
    len: u64,
    capacity: u64,
}

unsafe impl Send for String {}
unsafe impl Sync for String {}

impl String {
    #[must_use]
    pub fn from_string(mut s: std::string::String) -> Self {
        let ptr = s.as_mut_ptr();
        let capacity = s.capacity() as u64;
        let len = s.len() as u64;
        forget(s);
        Self { ptr, len, capacity }
    }

    pub fn as_str(&self) -> &str {
        if self.ptr.is_null() {
            return "";
        }

        unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.ptr, self.len as usize)) }
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn into_string(self) -> std::string::String {
        let rval = unsafe { std::string::String::from_raw_parts(self.ptr, self.len as usize, self.capacity as usize) };
        forget(self);
        rval
    }
}

impl Clone for String {
    fn clone(&self) -> Self {
        String::from_string(self.as_str().to_string())
    }
}

impl Drop for String {
    #[allow(clippy::cast_possible_truncation)]
    fn drop(&mut self) {
        if self.ptr.is_null() {
            return;
        }
        unsafe {
            let _ = std::string::String::from_raw_parts(self.ptr, self.len as usize, self.capacity as usize);
        }
    }
}

unsafe impl CTypeInfo for String {
    #[rustfmt::skip]
    fn type_info() -> CType {
        let fields = vec![
            Field::new("ptr".to_string(), CType::ReadWritePointer(Box::new(CType::Primitive(PrimitiveType::U8)))),
            Field::new("len".to_string(), CType::Primitive(PrimitiveType::U64)),
            Field::new("capacity".to_string(), CType::Primitive(PrimitiveType::U64)),
        ];

        let doc = Documentation::from_lines(vec![
            " UTF-8 string marshalling helper.".to_string(),
            " A highly dangerous 'use once type' that has ownership semantics!".to_string(),
            " Once passed over an FFI boundary 'the other side' is meant to own".to_string(),
            " (and free) it. Rust handles that fine, but if in C# you put this".to_string(),
            " in a struct and then call Rust multiple times with that struct ".to_string(),
            " you'll free the same pointer multiple times, and get UB!".to_string(),
        ]);
        let repr = Representation::new(Layout::C, None);
        let meta = Meta::with_documentation(doc);
        let composite = CompositeType::with_meta_repr("Utf8String".to_string(), fields, meta, repr);
        CType::Pattern(TypePattern::Utf8String(composite))
    }
}
