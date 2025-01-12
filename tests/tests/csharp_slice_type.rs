use anyhow::Error;
use interoptopus::patterns::slice::FFISlice;
use interoptopus::{ffi_function, function, Interop, Inventory, InventoryBuilder};
use interoptopus_backend_csharp::{ConfigBuilder, Generator};
use tests::backend_csharp::common_namespace_mappings;
use tests::validate_output;

#[ffi_function]
fn sample_function(_: FFISlice<u8>) {}

fn ffi_inventory() -> Inventory {
    InventoryBuilder::new().register(function!(sample_function)).inventory()
}

#[test]
fn spans_work() -> Result<(), Error> {
    let inventory = ffi_inventory();
    let config = ConfigBuilder::default().namespace_mappings(common_namespace_mappings()).build()?;
    let generated = Generator::new(config, inventory).write_string()?;

    validate_output!("tests", "csharp_slice_type.cs", generated.as_str());

    Ok(())
}
