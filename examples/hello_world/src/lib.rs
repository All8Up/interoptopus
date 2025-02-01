use interoptopus::{ffi_function, ffi_type};

/// A simple type in our FFI layer.
#[ffi_type]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

/// Function using the type.
#[ffi_function]
pub fn my_function(input: Vec2) -> Vec2 {
    input
}

// We just trick a unit test into producing our bindings, here for C#
#[test]
#[rustfmt::skip]
fn generate_bindings() {
    use interoptopus::{function, Generate, InventoryBuilder};
    use interoptopus_backend_csharp::{Config, Generator};

    // In a real project this should be a freestanding `my_inventory()` function inside
    // your FFI or build crate.
    let inventory = InventoryBuilder::new()
        .register(function!(my_function))
        .validate()
        .inventory();

    let config = Config::default();

    Generator::new(config, inventory)
        .write_file("bindings/Interop.cs")
        .unwrap();
}
