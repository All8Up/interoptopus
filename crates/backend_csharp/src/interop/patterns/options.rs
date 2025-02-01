use crate::converter::to_typespecifier_in_rval;
use crate::Interop;
use interoptopus::lang::c::CompositeType;
use interoptopus::writer::IndentWriter;
use interoptopus::{indented, Error};

pub fn write_pattern_option(i: &Interop, w: &mut IndentWriter, slice: &CompositeType) -> Result<(), Error> {
    i.debug(w, "write_pattern_option")?;

    let context_type_name = slice.rust_name();
    let data_type = slice
        .fields()
        .iter()
        .find(|x| x.name().eq("t"))
        .expect("Option must contain field called 't'.")
        .the_type();

    let type_string = to_typespecifier_in_rval(data_type);
    let is_some = if i.rename_symbols { "isSome" } else { "is_some" };

    indented!(w, r"{} partial struct {}", i.visibility_types.to_access_modifier(), context_type_name)?;
    indented!(w, r"{{")?;

    // FromNullable
    indented!(w, [()], r"public static {} FromNullable({}? nullable)", context_type_name, type_string)?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"var result = new {}();", context_type_name)?;
    indented!(w, [()()], r"if (nullable.HasValue)")?;
    indented!(w, [()()], r"{{")?;
    indented!(w, [()()()], r"result.{} = 1;", is_some)?;
    indented!(w, [()()()], r"result.t = nullable.Value;")?;
    indented!(w, [()()], r"}}")?;
    w.newline()?;
    indented!(w, [()()], r"return result;")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;

    // ToNullable
    indented!(w, [()], r"public {}? ToNullable()", type_string)?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"return this.{} == 1 ? this.t : ({}?)null;", is_some, type_string)?;
    indented!(w, [()], r"}}")?;

    indented!(w, r"}}")?;
    w.newline()?;
    Ok(())
}
