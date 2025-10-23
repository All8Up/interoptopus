mod constants;
mod defines;
mod docs;
mod functions;
mod imports;
mod types;

pub use functions::write_function_declaration;
use std::fs::File;
use std::path::Path;
pub use types::write_type_definition;

use crate::interop::constants::write_constants;
use crate::interop::defines::{write_custom_defines, write_ifdefcpp, write_ifndef};
use crate::interop::docs::write_file_header_comments;
use crate::interop::functions::write_functions;
use crate::interop::imports::write_imports;
use crate::interop::types::write_type_definitions;
use derive_builder::Builder;
use heck::{ToLowerCamelCase, ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use interoptopus::inventory::Inventory;
use interoptopus_backend_utils::{Error, IndentWriter};

use include_dir::{Dir, include_dir};

/// Embed the templates.
/// TODO: Add the ability to export these to a folder for the user.
/// TODO: Allow the user to override these templates.
static TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/template");

/// How to lay out functions.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum Functions {
    Typedefs,
    #[default]
    ForwardDeclarations,
}

/// How to name enum variants
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum EnumVariants {
    #[default]
    WithEnumName,
    VariantName,
}

/// How to indent (Allman, K&R, ...)
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum Indentation {
    /// Braces on their own lines, not indented
    Allman,
    /// Opening brace on same line as declaration, closing brace on own line, not intended
    KAndR,
    /// Braces on their own lines, intended by two spaces
    GNU,
    /// Braces on their own lines, intended level with members
    #[default]
    Whitesmiths,
}

/// Naming style, like lower or UPPER case.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum NameCase {
    /// Names all in lowercase without spacing e.g. 'thetypename'
    Lower,
    /// Names all in uppercase without spacing e.g. 'THETYPENAME'
    #[default]
    Upper,
    /// Names in mixed case starting with lowercase without spacing e.g. 'theTypeName'
    LowerCamel,
    /// Names in mixed case starting with uppercase without spacing e.g. '`TheTypeName`'
    UpperCamel,
    /// Names in lower case with '_' as spacing e.g. '`the_type_name`'
    Snake,
    /// Names in upper case with '_' as spacing e.g. '`THE_TYPE_NAME`'
    ShoutySnake,
}

pub trait ToNamingStyle {
    fn to_naming_style(&self, style: &NameCase) -> String;
}

impl ToNamingStyle for String {
    fn to_naming_style(&self, style: &NameCase) -> String {
        self.as_str().to_naming_style(style)
    }
}

impl ToNamingStyle for &str {
    fn to_naming_style(&self, style: &NameCase) -> String {
        match style {
            NameCase::Lower => self.to_lowercase(),
            NameCase::Upper => self.to_uppercase(),
            NameCase::LowerCamel => self.to_lower_camel_case(),
            NameCase::UpperCamel => self.to_upper_camel_case(),
            NameCase::Snake => self.to_snake_case(),
            NameCase::ShoutySnake => self.to_shouty_snake_case(),
        }
    }
}

/// Documentation style used in generated C code
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum DocStyle {
    // No documentation comments are added to header file
    None,
    // Documentation is added inline above relevant declaration
    #[default]
    Inline,
}

/// Generates C header files, **get this with [`InteropBuilder`]**.🐙
#[derive(Clone, Debug, Builder, Default)]
#[builder(default)]
pub struct Interop {
    /// Whether to write conditional directives like `#ifndef _X`.
    #[builder(default = "true")]
    directives: bool,
    /// Whether to write standard imports like `#include <stdint.h>`.
    #[builder(default = "true")]
    imports: bool,
    #[builder(default = "vec![\"<stdint.h>\".to_string(), \"<stdbool.h>\".to_string(), \"<sys/types.h>\".to_string()]")]
    includes: Vec<String>,
    /// The `_X` in `#ifndef _X` to be used.
    #[builder(default = "\"interoptopus_generated\".to_string()")]
    ifndef: String,
    /// Multiline string with custom `#define` values.
    #[builder(setter(into))]
    defines: Vec<String>,
    /// Prefix to be applied to any function, e.g., `__DLLATTR`.
    #[builder(setter(into))]
    function_attribute: String,
    /// Comment at the very beginning of the file, e.g., `// (c) My Company.`
    #[builder(setter(into))]
    file_header_comment: String,
    /// How to prefix everything, e.g., `my_company_`, will be capitalized for constants.
    #[builder(setter(into))]
    pub(crate) prefix: String,
    /// How to indent code
    #[builder(setter(into))]
    indentation: Indentation,
    /// How to add code documentation
    #[builder(setter(into))]
    documentation: DocStyle,
    /// How to convert type names
    #[builder(setter(into))]
    pub(crate) type_naming: NameCase,
    /// How to convert enum variant names
    #[builder(setter(into))]
    pub(crate) enum_variant_naming: NameCase,
    /// How to convert const names
    #[builder(setter(into))]
    pub(crate) const_naming: NameCase,
    /// How to convert function parameter names
    #[builder(setter(into))]
    function_parameter_naming: NameCase,
    /// How to emit enum variants
    #[builder(setter(into))]
    pub(crate) enum_variant_style: EnumVariants,
    /// How to emit functions
    #[builder(setter(into))]
    function_style: Functions,
    pub(crate) inventory: Inventory,
}

impl Interop {
    /// Creates a new [`InteropBuilder`].
    #[must_use]
    pub fn builder() -> InteropBuilder {
        InteropBuilder::new()
    }

    pub(crate) fn inventory(&self) -> &Inventory {
        &self.inventory
    }

    /// Generates FFI binding code and writes them to the [`IndentWriter`].
    ///
    /// # Errors
    /// Can result in an error if I/O failed.
    pub fn write_to(&self, w: &mut IndentWriter) -> Result<(), Error> {
        let mut tera = tera::Tera::default();
        TEMPLATES.files().for_each(|file| {
            let path = file.path().to_str().unwrap();
            let contents = file.contents_utf8().unwrap();
            println!("Adding template: {}", path);
            tera.add_raw_template(path, contents).unwrap();
        });

        use tera::to_value;
        let mut context = tera::Context::new();
        context.insert(
            "structure",
            &tera::Value::Object(tera::Map::from_iter(
                [
                    ("header".to_string(), to_value(&self.file_header_comment).unwrap()),
                    ("footer".to_string(), to_value("// My test footer.").unwrap()),
                ]
                .into_iter()
                .collect::<tera::Map<String, tera::Value>>(),
            )),
        );
        context.insert(
            "config",
            &tera::Value::Object(tera::Map::from_iter(
                [
                    ("cpp_compat".to_string(), to_value(&self.directives).unwrap()),
                    ("ifndef".to_string(), to_value(&self.ifndef).unwrap()),
                    ("imports".to_string(), to_value(&self.imports).unwrap()),
                ]
                .into_iter()
                .collect::<tera::Map<String, tera::Value>>(),
            )),
        );
        context.insert("includes", &to_value(&self.includes).unwrap());
        context.insert("defines", &to_value(&self.defines).unwrap());

        // Insert by index and use registered functions to break them down for rendering.
        context.insert("constants", &to_value(&self.inventory.constants().iter().enumerate().map(|(i, _)| i).collect::<Vec<usize>>()).unwrap());
        context.insert("types", &to_value(&self.inventory.c_types().iter().enumerate().map(|(i, _)| i).collect::<Vec<usize>>()).unwrap());
        context.insert("functions", &to_value(&self.inventory.functions().iter().enumerate().map(|(i, _)| i).collect::<Vec<usize>>()).unwrap());

        struct TypeName {
            types: Vec<interoptopus::lang::Type>,
        }
        impl tera::Function for TypeName {
            fn call(&self, args: &std::collections::HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
                let name = args.get("index").ok_or_else(|| tera::Error::msg("type_name: missing argument 'name'"))?;
                let index = name.as_u64().ok_or_else(|| tera::Error::msg("type_name: argument 'name' not a u64"))? as usize;
                let _ty = self.types.get(index).ok_or_else(|| tera::Error::msg("type_name: index out of bounds"))?;
                let item = _ty.name_within_lib();

                Ok(tera::Value::String(item))
            }
        }
        tera.register_function("type_name", TypeName { types: self.inventory.c_types().clone().into() });

        struct FunctionName {
            functions: Vec<interoptopus::lang::Function>,
        }
        impl tera::Function for FunctionName {
            fn call(&self, args: &std::collections::HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
                let name = args.get("index").ok_or_else(|| tera::Error::msg("function_name: missing argument 'name'"))?;
                let index = name.as_u64().ok_or_else(|| tera::Error::msg("function_name: argument 'name' not a u64"))? as usize;
                let _fn = self.functions.get(index).ok_or_else(|| tera::Error::msg("function_name: index out of bounds"))?;
                let item = _fn.name();
                Ok(tera::Value::String(item.into()))
            }
        }
        tera.register_function("function_name", FunctionName { functions: self.inventory.functions().clone().into() });

        let string = tera.render("main.tpl", &context).unwrap();
        w.writer().write_all(string.as_bytes())?;

        //- write_ifndef(self, w, |w| {
        //-     write_ifdefcpp(self, w, |w| {
        //         if self.imports {
        //             write_imports(self, w)?;
        //             w.newline()?;
        //         }

        //         write_custom_defines(self, w)?;
        //         w.newline()?;

        //         write_constants(self, w)?;
        //         w.newline()?;

        //         write_type_definitions(self, w)?;
        //         w.newline()?;

        //         write_functions(self, w)?;

        //         Ok(())
        //     })?;

        //     Ok(())
        // })?;

        Ok(())
    }

    /// Convenience method to write FFI bindings to the specified file with default indentation.
    ///
    /// # Errors
    /// Can result in an error if I/O failed.
    pub fn write_file<P: AsRef<Path>>(&self, file_name: P) -> Result<(), Error> {
        let mut file = File::create(file_name)?;
        let mut writer = IndentWriter::new(&mut file);

        self.write_to(&mut writer)
    }

    /// Convenience method to write FFI bindings to a string.
    ///
    /// # Errors
    /// Can result in an error if I/O failed.
    pub fn to_string(&self) -> Result<String, Error> {
        let mut vec = Vec::new();
        let mut writer = IndentWriter::new(&mut vec);
        self.write_to(&mut writer)?;
        Ok(String::from_utf8(vec)?)
    }
}

impl InteropBuilder {
    /// Creates a new builder instance, **start here**.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
