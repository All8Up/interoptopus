use crate::converter::{
    function_name_to_csharp_name, function_parameter_to_csharp_typename, function_rval_to_csharp_typename, is_owned_slice, pattern_to_native_in_signature,
    to_typespecifier_in_async_fn_rval, to_typespecifier_in_param, to_typespecifier_in_sync_fn_rval,
};
use crate::interop::docs::write_documentation;
use crate::{FunctionNameFlavor, Interop};
use interoptopus::backend::{IndentWriter, WriteFor};
use interoptopus::lang::{Function, Primitive, SugaredReturnType, Type};
use interoptopus::pattern::TypePattern;
use interoptopus::{Error, indented};
use std::iter::zip;

pub fn write_functions(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    for function in i.inventory.functions() {
        if i.should_emit_by_meta(function.meta()) {
            write_function(i, w, function, WriteFor::Code)?;
            w.newline()?;
        }
    }

    Ok(())
}

pub fn write_function(i: &Interop, w: &mut IndentWriter, function: &Function, write_for: WriteFor) -> Result<(), Error> {
    i.debug(w, "write_function")?;
    if write_for == WriteFor::Code {
        write_documentation(w, function.meta().documentation())?;
        write_function_annotation(i, w, function)?;
    }
    write_function_declaration(i, w, function, false)?;
    w.newline()?;
    write_function_overload(i, w, function, write_for)?;

    Ok(())
}

pub fn write_function_annotation(_i: &Interop, w: &mut IndentWriter, function: &Function) -> Result<(), Error> {
    indented!(w, r#"[LibraryImport(NativeLib, EntryPoint = "{}")]"#, function.name())?;

    if *function.signature().rval() == Type::Primitive(Primitive::Bool) {
        indented!(w, r"[return: MarshalAs(UnmanagedType.U1)]")?;
    }

    Ok(())
}

pub fn write_function_declaration(i: &Interop, w: &mut IndentWriter, function: &Function, has_body: bool) -> Result<(), Error> {
    i.debug(w, "write_function_declaration")?;

    let rval = function_rval_to_csharp_typename(function);
    let name = function_name_to_csharp_name(
        function,
        if i.rename_symbols {
            FunctionNameFlavor::CSharpMethodNameWithClass
        } else {
            FunctionNameFlavor::RawFFIName
        },
    );

    let mut params = Vec::new();

    let native = i.has_custom_marshalled_delegate(function.signature());
    let visibility = "public ";

    for p in function.signature().params() {
        let the_type = function_parameter_to_csharp_typename(p);
        let name = p.name();

        if native && matches!(p.the_type(), Type::FnPointer(_) | Type::Pattern(TypePattern::NamedCallback(_))) {
            let suffix = if matches!(p.the_type(), Type::FnPointer(_)) { "_native" } else { "" };
            params.push(format!("{the_type}{suffix} {name}"));
        } else {
            params.push(format!("{the_type} {name}"));
        }
    }

    let line_ending = if has_body { "" } else { ";" };
    let partial = if has_body { "" } else { "partial " };

    i.inline_hint(w, 0)?;
    indented!(w, r"{}static {}{} {}({}){}", visibility, partial, rval, name, params.join(", "), line_ending)
}

#[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
pub fn write_function_overload(i: &Interop, w: &mut IndentWriter, function: &Function, write_for: WriteFor) -> Result<(), Error> {
    i.debug(w, "write_function_overload")?;

    let has_overload = i.has_overloadable(function.signature());

    // If there is nothing to write, don't do it
    if !has_overload {
        i.debug(w, &format!("no overload for {}", function.name()))?;
        return Ok(());
    }

    let async_rval = function.sugared_return_type();

    let mut to_pin_name = Vec::new();
    let mut to_pin_slice_type = Vec::new();
    let mut to_invoke = Vec::new();
    let mut to_wrap_name = Vec::new();
    let mut to_wrap_type = Vec::new();

    let raw_name = function_name_to_csharp_name(
        function,
        if i.rename_symbols {
            FunctionNameFlavor::CSharpMethodNameWithClass
        } else {
            FunctionNameFlavor::RawFFIName
        },
    );

    let rval = to_typespecifier_in_async_fn_rval(&function.sugared_return_type());

    let mut params = Vec::new();
    for p in function.signature().params() {
        let name = p.name();
        let native = pattern_to_native_in_signature(i, p);
        let the_type = function_parameter_to_csharp_typename(p);

        let mut fallback = || {
            if native.contains("ref ") {
                to_invoke.push(format!("ref {name}"));
            } else {
                to_invoke.push(name.to_string());
            }
        };

        match p.the_type() {
            Type::Pattern(TypePattern::Slice(x) | TypePattern::SliceMut(x)) => {
                if is_owned_slice(x) {
                    to_wrap_name.push(name);
                    to_wrap_type.push(to_typespecifier_in_param(p.the_type()));
                    to_invoke.push(format!("{name}_wrapped"));
                } else {
                    to_pin_name.push(name);
                    to_pin_slice_type.push(the_type);
                    to_invoke.push(format!("{name}_slice"));
                }
            }
            Type::Pattern(TypePattern::NamedCallback(_)) => {
                to_wrap_name.push(name);
                to_wrap_type.push(to_typespecifier_in_param(p.the_type()));
                to_invoke.push(format!("{name}_wrapped"));
            }
            Type::Pattern(TypePattern::Utf8String(_)) => {
                to_wrap_name.push(name);
                to_wrap_type.push(to_typespecifier_in_param(p.the_type()));
                to_invoke.push(format!("{name}_wrapped"));
            }
            Type::ReadPointer(x) | Type::ReadWritePointer(x) => match &**x {
                Type::Pattern(x) => match x {
                    TypePattern::Slice(_) => {
                        to_pin_name.push(name);
                        to_pin_slice_type.push(the_type.replace("ref ", ""));
                        to_invoke.push(format!("ref {name}_slice"));
                    }
                    TypePattern::SliceMut(_) => {
                        to_pin_name.push(name);
                        to_pin_slice_type.push(the_type.replace("ref ", ""));
                        to_invoke.push(format!("ref {name}_slice"));
                    }
                    _ => fallback(),
                },
                _ => fallback(),
            },
            _ => fallback(),
        }

        params.push(format!("{native} {name}"));
    }

    if matches!(async_rval, SugaredReturnType::Async(_)) {
        params.pop();
        to_invoke.pop();
        to_invoke.push("cb".to_string());
    }

    let signature = format!(r"public static unsafe {} {}({})", rval, raw_name, params.join(", "));
    if write_for == WriteFor::Docs {
        indented!(w, r"{};", signature)?;
        return Ok(());
    }

    if write_for == WriteFor::Code {
        write_documentation(w, function.meta().documentation())?;
    }

    i.inline_hint(w, 0)?;
    indented!(w, "{}", signature)?;
    indented!(w, r"{{")?;

    if let SugaredReturnType::Async(ref x) = async_rval {
        let task_completion_source = match x {
            Type::Pattern(TypePattern::Result(x)) if matches!(x.t(), Type::Pattern(TypePattern::Utf8String(_))) => "TaskCompletionSource<string>".to_string(),
            Type::Pattern(TypePattern::Result(x)) if x.t().is_void() => "TaskCompletionSource".to_string(),
            Type::Pattern(TypePattern::Result(x)) => format!("TaskCompletionSource<{}>", to_typespecifier_in_sync_fn_rval(x.t())),
            x => format!("TaskCompletionSource<{}>", to_typespecifier_in_sync_fn_rval(x)),
        };

        indented!(w, [()], r"var cs = new {task_completion_source}();")?;
        indented!(w, [()], r"GCHandle pinned = default;")?;
        indented!(w, [()], r"var cb = new AsyncHelper((x) => {{")?;
        indented!(w, [()()], r"var unmanaged = Marshal.PtrToStructure<{}.Unmanaged>(x);", to_typespecifier_in_param(x))?;
        indented!(w, [()()], r"var marshaller = new {}.Marshaller(unmanaged);", to_typespecifier_in_param(x))?;
        indented!(w, [()()], r"var managed = marshaller.ToManaged();")?;
        match x {
            Type::Pattern(TypePattern::Result(x)) => {
                if x.t().is_void() {
                    indented!(w, [()()], r"if (managed.IsOk) {{ cs.SetResult(); }}")?;
                } else {
                    indented!(w, [()()], r"if (managed.IsOk) {{ cs.SetResult(managed.AsOk()); }}")?;
                }
                indented!(w, [()()], r"else {{ cs.SetException(new InteropException()); }}")?;
            }
            _ => indented!(w, [()()], r"cs.SetResult(managed);")?,
        }
        indented!(w, [()()], r"pinned.Free();")?;
        indented!(w, [()], r"}});")?;
        indented!(w, [()], r"pinned = GCHandle.Alloc(cb);")?;
    }

    if !to_pin_name.is_empty() {
        for (pin_var, slice_struct) in to_pin_name.iter().zip(to_pin_slice_type.iter()) {
            indented!(w, [()], r"fixed (void* ptr_{} = {})", pin_var, pin_var)?;
            indented!(w, [()], r"{{")?;
            indented!(w, [()()], r"var {}_slice = new {}(new IntPtr(ptr_{}), (ulong) {}.Length);", pin_var, slice_struct, pin_var, pin_var)?;
            w.indent();
        }
    }

    for (n, t) in zip(&to_wrap_name, &to_wrap_type) {
        indented!(w, [()], r"var {}_wrapped = new {}({});", n, t, n)?;
    }

    indented!(w, [()], r"try")?;
    indented!(w, [()], r"{{")?;

    let fn_name = function_name_to_csharp_name(
        function,
        if i.rename_symbols {
            FunctionNameFlavor::CSharpMethodNameWithClass
        } else {
            FunctionNameFlavor::RawFFIName
        },
    );

    let call = format!(r"{}({})", fn_name, to_invoke.join(", "));

    match function.signature().rval() {
        Type::Pattern(TypePattern::CStrPointer) => {
            indented!(w, [()()], r"var s = {};", call)?;
            indented!(w, [()()], r"return Marshal.PtrToStringAnsi(s);")?;
        }
        Type::Primitive(Primitive::Void) => {
            indented!(w, [()()], r"{};", call)?;
        }
        _ if matches!(async_rval, SugaredReturnType::Async(_)) => {
            indented!(w, [()()], r"{call}.AsOk();")?;
            indented!(w, [()()], r"return cs.Task;")?;
        }
        _ => {
            indented!(w, [()()], r"return {call};")?;
        }
    }
    indented!(w, [()], r"}}")?;
    indented!(w, [()], r"finally")?;
    indented!(w, [()], r"{{")?;
    for n in to_wrap_name {
        indented!(w, [()()], r"{}_wrapped.Dispose();", n)?;
    }
    indented!(w, [()], r"}}")?;

    if !to_pin_name.is_empty() {
        for _ in &to_pin_name {
            w.unindent();
            indented!(w, [()], r"}}")?;
        }
    }

    if matches!(async_rval, SugaredReturnType::Async(_)) {
        indented!(w, [()], r"return cs.Task;")?;
    }

    indented!(w, r"}}")
}
