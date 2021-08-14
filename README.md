
[![Latest Version]][crates.io]
[![docs]][docs.rs]
![MIT]
[![Rust](https://img.shields.io/badge/rust-1.53%2B-blue.svg?maxAge=3600)](https://github.com/ralfbiedert/interoptopus)
[![Rust](https://github.com/ralfbiedert/interoptopus/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/ralfbiedert/interoptopus/actions/workflows/rust.yml)

## Interoptopus 🐙

The polyglot bindings generator for your library (C#, C, Python, ...)


### Code you write ...

```rust
use interoptopus::{ffi_function, ffi_type, inventory};

#[ffi_type]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn my_function(input: Vec2) {
    println!("{}", input.x);
}

inventory!(ffi_inventory, [], [my_function], [], []);

```

### ... Interoptopus generates

| Language | Crate | Sample Output |
| --- | --- | --- |
| C# (incl. Unity) | [**interoptopus_backend_csharp**](https://crates.io/crates/interoptopus_backend_csharp) | [Interop.cs](https://github.com/ralfbiedert/interoptopus/blob/master/interoptopus_backend_csharp/tests/output/Interop.cs) |
| C | [**interoptopus_backend_c**](https://crates.io/crates/interoptopus_backend_c) | [my_header.h](https://github.com/ralfbiedert/interoptopus/blob/master/interoptopus_backend_c/tests/output/my_header.h) |
| Python [CFFI](https://cffi.readthedocs.io/en/latest/index.html) | [**interoptopus_backend_cpython_cffi**](https://crates.io/crates/interoptopus_backend_cpython_cffi) | [reference.py](https://github.com/ralfbiedert/interoptopus/blob/master/interoptopus_backend_cpython_cffi/tests/output/reference_project.py) |
| Your language | Write your own backend<sup>1</sup> | - |

<sup>1</sup> Create your own backend in just a few hours. No pull request needed. [Pinkie promise](https://github.com/ralfbiedert/interoptopus/blob/master/FAQ.md#new-backends).

### Getting Started 🍼

If you want to ...
- **create a new API** see the [**example projects**](https://github.com/ralfbiedert/interoptopus/tree/master/examples),
- **support a new language**, [**copy the C backend**](https://github.com/ralfbiedert/interoptopus/tree/master/interoptopus_backend_c).

### Features

- explicit, type-safe, **single source of truth** API definition in Rust,
- **quality-of-life [patterns](crate::patterns)** on **both sides** (e.g., [options](crate::patterns::option), [slices](crate::patterns::slice), [services](crate::patterns::service), ...)
- if your **project compiles your bindings should work**, <sup>&#42;*cough*&#42;</sup>
- easy to support new languages, fully **customizable**,
- **no scripts needed**, works from `cargo build` + `cargo test`.


Gated behind **feature flags**, these enable:

- `derive` - Proc macros such as `ffi_constant`, `ffi_function`, `ffi_type`.
- `serde` - Serde attributes on internal types.
- `log` - Invoke `log` on FFI errors (you still need actual logger).


### Supported Rust Constructs
See the [**reference project**](https://github.com/ralfbiedert/interoptopus/tree/master/interoptopus_reference_project/src); it lists all supported constructs including:
- [functions](https://github.com/ralfbiedert/interoptopus/blob/master/interoptopus_reference_project/src/functions.rs) (`extern "C"` functions and delegates)
- [types](https://github.com/ralfbiedert/interoptopus/blob/master/interoptopus_reference_project/src/types.rs) (composites, enums, opaques, references, pointers, ...)
- [constants](https://github.com/ralfbiedert/interoptopus/blob/master/interoptopus_reference_project/src/constants.rs) (primitive constants; results of const evaluation)
- [patterns](https://github.com/ralfbiedert/interoptopus/tree/master/interoptopus_reference_project/src/patterns) (ASCII pointers, options, slices, classes, ...)

As a rule of thumb we recommend to be slightly conservative with your signatures and always "think C", since other languages don't track lifetimes
well and it's is easy to accidentally pass an outlived pointer or doubly alias a `&mut X` on reentrant functions.


### Performance 🏁

Generated low-level bindings should be "zero cost" w.r.t. hand-crafted bindings for that language. However, even hand-crafted bindings
have an inherent, language-specific cost. For C# that cost can be almost 0, for Python CFFI it can be high. Patterns and convenience
helpers might add additional overhead.

If you **need API design guidance** the following (wip) [**C# call-cost table**](https://github.com/ralfbiedert/interoptopus/blob/master/interoptopus_backend_csharp/benches/BENCHMARK_RESULTS.md)<sup>🔥</sup> can help.

### Changelog

- **v0.8** - Moved testing functions to respective backend crates.
- **v0.7** - Patterns mostly use proc macros now for better FFI docs.
- **v0.6** - Renamed and clarified many patterns.
- **v0.5** - More ergonomic slice usage in Rust and FFI.
- **v0.4** - Enable logging support in auto-generated FFI calls.
- **v0.3** - Better compatibility with generics.
- **v0.2** - Introduced "patterns"; produces generally _working_ interop for C#.
- **v0.1** - Has generated C#, C, Python-CFFI bindings at least once.


### FAQ

- [FAQ and Safety Guides](https://github.com/ralfbiedert/interoptopus/blob/master/FAQ.md).

### Contributing

PRs are welcome.

- Submit small bug fixes directly. Major changes should be issues first.
- Anything that makes previously working bindings change behavior or stop compiling
is a major change;
- This doesn't mean we're opposed to breaking stuff just that
we'd like to talk about it before it happens.
- New features or patterns must be materialized in the reference project and accompanied by
an interop test (i.e., a backend test running C# / Python against a DLL invoking that code)
in at least one included backend.

[Latest Version]: https://img.shields.io/crates/v/interoptopus.svg
[crates.io]: https://crates.io/crates/interoptopus
[MIT]: https://img.shields.io/badge/license-MIT-blue.svg
[docs]: https://docs.rs/interoptopus/badge.svg
[docs.rs]: https://docs.rs/interoptopus/
