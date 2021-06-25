//! A reference project for [Interoptopus](https://github.com/ralfbiedert/interoptopus).
//!
//! This project tries to use every Interoptopus feature at least once.
//! When submitting new features or making changes to existing ones the types and functions in
//! here will ensure existing backends still work as expected.
//!
//! Note, many items here are deliberately not documented as testing how and if documentation
//! is generated is part of the test.

pub mod constants;
pub mod functions;
/// Reference implementations of patterns.
pub mod patterns {
    pub mod ascii_pointer;
    pub mod option;
    pub mod service_generated;
    pub mod service_manual;
    pub mod slice;
    pub mod success_enum;
}
pub mod types;

interoptopus::inventory_function!(
    ffi_inventory,
    [constants::U8, constants::F32_MIN_POSITIVE, constants::COMPUTED_I32],
    [
        functions::primitive_void,
        functions::primitive_void2,
        functions::primitive_bool,
        functions::primitive_u8,
        functions::primitive_u16,
        functions::primitive_u32,
        functions::primitive_u64,
        functions::primitive_i8,
        functions::primitive_i16,
        functions::primitive_i32,
        functions::primitive_i64,
        functions::ptr,
        functions::ptr_mut,
        functions::ptr_ptr,
        functions::ref_simple,
        functions::ref_mut_simple,
        functions::ref_option,
        functions::ref_mut_option,
        functions::tupled,
        functions::complex_args_1,
        functions::complex_args_2,
        functions::callback,
        functions::generic_1,
        functions::generic_2,
        functions::documented,
        functions::ambiguous_1,
        functions::ambiguous_2,
        functions::ambiguous_3,
        functions::namespaced_type,
        patterns::ascii_pointer::pattern_ascii_pointer_1,
        patterns::ascii_pointer::pattern_ascii_pointer_len,
        patterns::slice::pattern_ffi_slice_1,
        patterns::slice::pattern_ffi_slice_2,
        patterns::slice::pattern_ffi_slice_delegate,
        patterns::option::pattern_ffi_option_1,
        patterns::option::pattern_ffi_option_2
    ],
    [
        patterns::service_manual::my_service_pattern_context,
        patterns::service_generated::simple_service_pattern
    ]
);
