use anyhow::Error;
use interoptopus::Bindings;
use interoptopus_backend_csharp::{InteropBuilder, WriteTypes};
use interoptopus_reference_project::ffi_inventory;
use tests::backend_csharp::{common_namespace_mappings, run_dotnet_command_if_installed};
use tests::validate_output;

#[test]
fn reference_benchmarks_prerequisites() -> Result<(), Error> {
    let generated_common = InteropBuilder::new()
        .inventory(ffi_inventory())
        .namespace_id("common")
        .namespace_mappings(common_namespace_mappings())
        .dll_name("interoptopus_reference_project")
        .write_types(WriteTypes::NamespaceAndInteroptopusGlobal)
        .debug(true)
        .build()?
        .to_string()?;

    let generated_other = InteropBuilder::new()
        .inventory(ffi_inventory())
        .namespace_mappings(common_namespace_mappings())
        .dll_name("interoptopus_reference_project")
        .write_types(WriteTypes::Namespace)
        .debug(true)
        .build()?
        .to_string()?;

    validate_output!("tests/csharp_reference_project", "Interop.common.cs", generated_common.as_str());
    validate_output!("tests/csharp_reference_project", "Interop.cs", generated_other.as_str());

    run_dotnet_command_if_installed("tests/csharp_reference_project", "test")?;

    Ok(())
}
