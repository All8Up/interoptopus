use crate::types::{Array, CharArray, EnumRenamedXYZ, FixedString, NestedArray, Vec3f32};
use interoptopus::ffi_function;

#[ffi_function]
pub fn array_1(x: Array) -> u8 {
    x.data[0]
}

// Apparently this is not valid C
// https://stackoverflow.com/questions/11656532/returning-an-array-using-c
//
// #[ffi_function]
// pub fn array_2(x: [u8; 16]) -> [u8; 16] {
//     x
// }

#[ffi_function]
pub fn array_2() -> Array {
    Array { data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16] }
}

#[ffi_function]
pub fn array_3(arr: &mut Array) {
    arr.data[0] = 42;
}

#[ffi_function]
pub fn nested_array_1() -> NestedArray {
    NestedArray {
        field_enum: EnumRenamedXYZ::X,
        field_vec: Vec3f32 { x: 1.0, y: 2.0, z: 3.0 },
        field_bool: true,
        field_int: 42,
        field_array: [1, 2, 3, 4, 5],
        field_array_2: [6, 7, 8, 9, 10],
        field_struct: Array { data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16] },
    }
}

#[ffi_function]
pub fn nested_array_2(result: &mut NestedArray) {
    result.field_enum = EnumRenamedXYZ::X;
    result.field_vec = Vec3f32 { x: 1.0, y: 2.0, z: 3.0 };
    result.field_bool = true;
    result.field_int = 42;
    result.field_array = [1, 2, 3, 4, 5];
    result.field_struct = Array { data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16] };
}

#[ffi_function]
pub fn nested_array_3(input: NestedArray) -> u8 {
    input.field_struct.data[1]
}

#[ffi_function]
pub fn char_array_1() -> CharArray {
    let mut result = CharArray { str: FixedString { data: [0; 32] }, str_2: FixedString { data: [0; 32] } };

    result.str.data[..14].copy_from_slice(b"Hello, World!\0");

    result
}

#[ffi_function]
pub fn char_array_2(arr: CharArray) -> CharArray {
    arr
}

#[ffi_function]
pub fn char_array_3(arr: &CharArray) -> u8 {
    arr.str.data[0]
}
