use crate::Interop;
use crate::converter::{to_typespecifier_in_param, to_typespecifier_in_sync_fn_rval};
use interoptopus::backend::IndentWriter;
use interoptopus::lang::Type;
use interoptopus::pattern::TypePattern;
use interoptopus::pattern::callback::AsyncCallback;
use interoptopus::{Error, indented};

pub fn write_pattern_async_trampoline(i: &Interop, w: &mut IndentWriter, asynk: &AsyncCallback) -> Result<(), Error> {
    i.debug(w, "write_pattern_async_trampoline")?;

    let inner = to_typespecifier_in_param(asynk.target());

    let task_completion_source = match asynk.target() {
        Type::Pattern(TypePattern::Result(x)) if matches!(x.t(), Type::Pattern(TypePattern::Utf8String(_))) => "TaskCompletionSource<string>".to_string(),
        Type::Pattern(TypePattern::Result(x)) if x.t().is_void() => "TaskCompletionSource".to_string(),
        Type::Pattern(TypePattern::Result(x)) => format!("TaskCompletionSource<{}>", to_typespecifier_in_sync_fn_rval(x.t())),
        x => format!("TaskCompletionSource<{}>", to_typespecifier_in_sync_fn_rval(x)),
    };

    let task = match asynk.target() {
        Type::Pattern(TypePattern::Result(x)) if matches!(x.t(), Type::Pattern(TypePattern::Utf8String(_))) => "Task<string>".to_string(),
        Type::Pattern(TypePattern::Result(x)) if x.t().is_void() => "Task".to_string(),
        Type::Pattern(TypePattern::Result(x)) => format!("Task<{}>", to_typespecifier_in_sync_fn_rval(x.t())),
        x => format!("Task<{}>", to_typespecifier_in_sync_fn_rval(x)),
    };

    indented!(w, r"public struct AsyncTrampoline{inner}")?;
    indented!(w, r"{{")?;
    indented!(w, [()], r"private static ulong Id = 0;")?;
    indented!(w, [()], r"private static Dictionary<ulong, {task_completion_source}> InFlight = new(1024);")?;
    indented!(w, [()], r"private AsyncCallbackCommon _delegate;")?;
    indented!(w, [()], r"private IntPtr _callback_ptr;")?;
    w.newline()?;
    i.inline_hint(w, 1)?;
    indented!(w, [()], r"public AsyncTrampoline{inner}()")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"_delegate = Call;")?;
    indented!(w, [()()], r"_callback_ptr = Marshal.GetFunctionPointerForDelegate(_delegate);")?;
    indented!(w, [()], r"}}")?;
    w.newline()?;
    i.inline_hint(w, 1)?;
    indented!(w, [()], r"void Call(IntPtr data, IntPtr csPtr)")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"{task_completion_source} tcs;")?;
    indented!(w, [()()], r"")?;
    indented!(w, [()()], r"lock (InFlight) {{ InFlight.Remove((ulong) csPtr, out tcs); }}")?;
    indented!(w, [()()], r"")?;
    indented!(w, [()()], r"var unmanaged = Marshal.PtrToStructure<{inner}.Unmanaged>(data);")?;
    indented!(w, [()()], r"var managed = unmanaged.ToManaged();")?;
    match asynk.target() {
        Type::Pattern(TypePattern::Result(x)) => {
            if x.t().is_void() {
                indented!(w, [()()], r"if (managed.IsOk) {{ tcs.SetResult(); }}")?;
            } else {
                indented!(w, [()()], r"if (managed.IsOk) {{ tcs.SetResult(managed.AsOk()); }}")?;
            }
            indented!(w, [()()], r"else {{ tcs.SetException(new InteropException()); }}")?;
        }
        _ => indented!(w, [()()], r"tcs.SetResult(managed);")?,
    }
    indented!(w, [()], r"}}")?;
    w.newline()?;
    i.inline_hint(w, 1)?;
    indented!(w, [()], r"public (AsyncCallbackCommonNative, {task}) NewCall()")?;
    indented!(w, [()], r"{{")?;
    indented!(w, [()()], r"var tcs = new {task_completion_source}();")?;
    indented!(w, [()()], r"var id = Id++;")?;
    indented!(w, [()()], r"")?;
    indented!(w, [()()], r"lock (InFlight) {{ InFlight.TryAdd(id, tcs); }}")?;
    indented!(w, [()()], r"")?;
    indented!(w, [()()], r"var ac = new AsyncCallbackCommonNative {{")?;
    indented!(w, [()()()], r"_ptr = _callback_ptr,")?;
    indented!(w, [()()()], r"_ts = (IntPtr) id,")?;
    indented!(w, [()()], r"}};")?;
    w.newline()?;
    indented!(w, [()()], r"return (ac, tcs.Task);")?;
    indented!(w, [()], r"}}")?;
    indented!(w, r"}}")?;

    Ok(())
}

pub fn write_pattern_async_trampoline_initializers(i: &Interop, w: &mut IndentWriter) -> Result<(), Error> {
    i.debug(w, "write_pattern_async_trampoline_initializers")?;

    for the_type in i.inventory.ctypes() {
        if let Type::Pattern(TypePattern::AsyncCallback(c)) = the_type {
            write_pattern_async_trampoline_initializer(i, w, c)?;
        }
    }

    Ok(())
}

pub fn write_pattern_async_trampoline_initializer(i: &Interop, w: &mut IndentWriter, asynk: &AsyncCallback) -> Result<(), Error> {
    i.debug(w, "write_pattern_async_trampoline_initializer")?;
    let inner = to_typespecifier_in_param(asynk.target());

    indented!(w, r"public static AsyncTrampoline{inner} _trampoline{inner} = new();")?;

    Ok(())
}
