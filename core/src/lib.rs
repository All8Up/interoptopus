#![cfg_attr(docsrs, feature(doc_cfg))] // does this work?
//!
//! [![Latest Version]][crates.io]
//! [![docs]][docs.rs]
//! ![MIT]
//! [![Rust](https://img.shields.io/badge/rust-1.53%2B-blue.svg?maxAge=3600)](https://github.com/ralfbiedert/interoptopus)
//! [![Rust](https://github.com/ralfbiedert/interoptopus/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/ralfbiedert/interoptopus/actions/workflows/rust.yml)
//!
//! # Interoptopus 🐙
//!
//! The polyglot bindings generator for your library.
//!
//!
//! ## Code you write ...
//!
//! ```rust
//! use interoptopus::{ffi_function, ffi_type, inventory};
//!
//! #[ffi_type]
//! #[repr(C)]
//! pub struct Vec2 {
//!     pub x: f32,
//!     pub y: f32,
//! }
//!
//! #[ffi_function]
//! #[no_mangle]
//! pub extern "C" fn my_function(input: Vec2) {
//!     println!("{}", input.x);
//! }
//!
//! inventory!(ffi_inventory, [], [my_function], [], []);
//!
//! ```
//!
//! ## ... Interoptopus generates
//!
//! | Language | Crate | Sample Output |
//! | --- | --- | --- |
//! | C# | [**interoptopus_backend_csharp**](https://crates.io/crates/interoptopus_backend_csharp) | [Interop.cs](https://github.com/ralfbiedert/interoptopus/blob/master/backends/csharp/tests/output_safe/Interop.cs) |
//! | C | [**interoptopus_backend_c**](https://crates.io/crates/interoptopus_backend_c) | [my_header.h](https://github.com/ralfbiedert/interoptopus/blob/master/backends/c/tests/output/my_header.h) |
//! | Python<sup>1</sup> | [**interoptopus_backend_cpython_cffi**](https://crates.io/crates/interoptopus_backend_cpython_cffi) | [reference.py](https://github.com/ralfbiedert/interoptopus/blob/master/backends/cpython_cffi/tests/output/reference_project.py) |
//! | Other | Write your own backend<sup>2</sup> | - |
//!
//! <sup>1</sup> Using Python [CFFI](https://cffi.readthedocs.io/en/latest/index.html). <br>
//! <sup>2</sup> Create your own backend in just a few hours. No pull request needed. [Pinkie promise](https://github.com/ralfbiedert/interoptopus/blob/master/FAQ.md#new-backends).
//!
//! ## Getting Started 🍼
//!
//! If you want to ...
//! - **create a new API** see the [**hello world**](https://github.com/ralfbiedert/interoptopus/tree/master/examples/hello_world),
//! - **understand what's possible**, see the [**reference project**](https://github.com/ralfbiedert/interoptopus/tree/master/reference_project/src),
//! - **support a new language**, [**copy the C backend**](https://github.com/ralfbiedert/interoptopus/tree/master/backends/c).
//!
//! ## Features
//!
//! - explicit, **single source of truth** API definition in Rust,
//! - **quality-of-life [patterns](crate::patterns)** (e.g., [slices](crate::patterns::slice), [services](crate::patterns::service), ...)
//! - if your **project compiles your bindings should work**, <sup>&#42;*cough*&#42;</sup>
//! - easy to support new languages, fully **customizable**,
//! - **no scripts needed**, just `cargo build` + `cargo test`.
//!
//!
//! Gated behind **feature flags**, these enable:
//!
//! - `derive` - Proc macros such as `ffi_type`, ...
//! - `serde` - Serde attributes on internal types.
//! - `log` - Invoke [log](https://crates.io/crates/log) on FFI errors.
//!
//!
//! ## Supported Rust Constructs
//!
//! See the [**reference project**](https://github.com/ralfbiedert/interoptopus/tree/master/reference_project/src) for an overview:
//! - [functions](https://github.com/ralfbiedert/interoptopus/blob/master/reference_project/src/functions.rs) (`extern "C"` functions and delegates)
//! - [types](https://github.com/ralfbiedert/interoptopus/blob/master/reference_project/src/types.rs) (composites, enums, opaques, references, ...)
//! - [constants](https://github.com/ralfbiedert/interoptopus/blob/master/reference_project/src/constants.rs) (primitive constants; results of const evaluation)
//! - [patterns](https://github.com/ralfbiedert/interoptopus/tree/master/reference_project/src/patterns) (ASCII pointers, options, slices, classes, ...)
//!
//!
//!
//! ## Performance 🏁
//!
//! Generated low-level bindings are "zero cost" w.r.t. hand-crafted bindings for that language.
//!
//! That said, even hand-crafted bindings encounter some target-specific overhead
//! at the FFI boundary (e.g., marshalling or pinning in managed languages) For C# that cost
//! can be nanoseconds, for Python CFFI it can be microseconds.
//!
//! See this [**C# call-cost table**](https://github.com/ralfbiedert/interoptopus/blob/master/backends/csharp/benches/BENCHMARK_RESULTS.md)<sup>🔥</sup> for ballpark figures.
//!
//! ## Changelog
//!
//! - **v0.9** - 150x faster C# slices, Python type hints.
//! - **v0.8** - Moved testing functions to respective backends.
//! - **v0.7** - Make patterns proc macros for better FFI docs.
//! - **v0.6** - Renamed and clarified many patterns.
//! - **v0.5** - More ergonomic slice usage in Rust and FFI.
//! - **v0.4** - Enable logging support in auto-generated FFI calls.
//! - **v0.3** - Better compatibility with generics.
//! - **v0.2** - Introduced "patterns"; _working_ interop for C#.
//! - **v0.1** - First version.
//!
//!
//! ## FAQ
//!
//! - [FAQ and Safety Guides](https://github.com/ralfbiedert/interoptopus/blob/master/FAQ.md).
//!
//! ## Contributing
//!
//! PRs are welcome.
//!
//! - Submit small bug fixes directly. Major changes should be issues first.
//! - Anything that makes previously working bindings change behavior or stop compiling
//! is a major change;
//! - This doesn't mean we're opposed to breaking stuff just that
//! we'd like to talk about it before it happens.
//! - New features or patterns must be materialized in the reference project and accompanied by
//! an interop test (i.e., a backend test running C# / Python against a DLL invoking that code)
//! in at least one included backend.
//!
//! [Latest Version]: https://img.shields.io/crates/v/interoptopus.svg
//! [crates.io]: https://crates.io/crates/interoptopus
//! [MIT]: https://img.shields.io/badge/license-MIT-blue.svg
//! [docs]: https://docs.rs/interoptopus/badge.svg
//! [docs.rs]: https://docs.rs/interoptopus/

pub use crate::core::{merge_libraries, Library};
pub use error::Error;
pub use generators::Interop;
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))] // does this work?
pub use interoptopus_proc::{ffi_constant, ffi_function, ffi_service, ffi_service_ctor, ffi_service_method, ffi_surrogates, ffi_type};

mod core;
mod error;
mod generators;
pub mod patterns;
pub mod testing;
pub mod util;
pub mod writer;

pub mod lang {
    //! Abstractions for authors of backends.
    //!
    //! A a rule of thumb, types in the [`rust`](rust) module generate instances
    //! of types in the [`c`](c) module.
    //!
    //! Normal users of Interoptopus probably won't have to concern
    //! themselves with any of the items in this module.
    //! A notable exception to this rule is having to manually implement
    //! a [`CType`](`c::CType`) surrogate for un-owned types. See the
    //! [surrogates section in `ffi_type`](crate::ffi_type#surrogates).
    pub mod c;
    pub mod rust;
}

/// **The** macro to define your library, ties everything together!
///
/// This macro produces an "inventory function", which can be ingested by backends. The function
/// will have the signature `fn f() -> Library`, where [`Library`] represents all functions,
/// types, constants and documentation exported by this crate over the FFI boundary.
///
/// # Usage
///
/// This macro must be invoked with exactly 5 parameters:
///
/// ```ignore
/// # use interoptopus::inventory_function;
/// inventory_function!(symbol, consts, functions, extra_types, patterns);
/// ```
///
/// Where
/// - `symbol` - the name of the exported inventory function producing a [`Library`],
/// - `consts` - a list of [`#[ffi_constant]`](crate::ffi_constant) constants to include `[C1, C2, ...]`,
/// - `functions` - a list of [`#[ffi_function]`](crate::ffi_function) functions to include `[f1, f2, ...]`,
/// - `extra_types` - additional types not inferred from `functions`, e.g., when using `void` pointers.
/// - `patterns` - a list of [`LibraryPattern`](crate::patterns::LibraryPattern) to include `[p1, ...]`,
///
/// Any of `consts`, `functions` or `patters` can be an empty list `[]` instead. Most types are
/// inferred automatically based on the used functions.
///
/// # Example
///
/// ```rust
/// use interoptopus::{ffi_function, ffi_constant};
///
/// #[ffi_constant]
/// const MY_CONSTANT: u8 = 1;
///
/// #[ffi_function]
/// #[no_mangle]
/// pub extern "C" fn f(_x: u8) {}
///
/// interoptopus::inventory!(
///     my_inventory_function,
///     [ MY_CONSTANT ],
///     [ f ],
///     [], []
/// );
/// ```
///
/// You can then use `my_inventory_function`, which will return a [`Library`], in a backend to
/// produce bindings to your language.
///
#[macro_export]
macro_rules! inventory {
    (
        $export_function:ident,
        [
        $(
            $const:path
        ),* $(,)?
        ],
        [
        $(
            $function:path
        ),* $(,)?
        ],
        [
        $(
            $extra_type:ty
        ),* $(,)?
        ],
        [
        $(
            $pattern:path
        ),* $(,)?
        ]
    ) => {
        #[doc(hidden)]
        pub fn $export_function() -> $crate::Library {
            use $crate::lang::rust::FunctionInfo;
            use $crate::lang::rust::ConstantInfo;

            let mut constants: ::std::vec::Vec<$crate::lang::c::Constant> = ::std::vec::Vec::new();
            $(
                {
                    use $const as user_constant;
                    constants.push(user_constant::constant_info());
                }
            )*

            let mut functions: ::std::vec::Vec<$crate::lang::c::Function> = ::std::vec::Vec::new();
            $(
                {
                    use $function as user_function;
                    functions.push(user_function::function_info());
                }
            )*

            let mut extra_types: ::std::vec::Vec<$crate::lang::c::CType> = ::std::vec::Vec::new();
            $(
                {
                    extra_types.push(< $extra_type as $crate::lang::rust::CTypeInfo >::type_info() );
                }
            )*


            let mut patterns: ::std::vec::Vec<$crate::patterns::LibraryPattern> = ::std::vec::Vec::new();
            $(
                {
                    let pattern: $crate::patterns::LibraryPattern = < $pattern as  $crate::patterns::LibraryPatternInfo>::pattern_info();

                    match &pattern {
                        $crate::patterns::LibraryPattern::Service(class) => {
                            functions.push(class.destructor().clone());
                            functions.extend(class.constructors().iter().cloned());
                            functions.extend(class.methods().iter().cloned());
                        }
                    }

                    patterns.push(pattern);
                }
            )*


            $crate::Library::new(functions, constants, patterns, extra_types)
        }
    };
}
