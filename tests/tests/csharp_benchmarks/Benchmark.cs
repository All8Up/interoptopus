using System;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Interoptopus;
using My.Company;
using My.Company.Common;

static class Benchmark {

    const int Iterations = 100_000;

    static void Main(string[] args)
    {
        Console.WriteLine("Running benchmarks ...");

        MeasureResult result;
        var writer = new MarkdownTableWriter();

        long x = 0;
        var short_vec = SliceVec3f32.From(new Vec3f32[10]);
        var short_byte = SliceU8.From(new byte[10]);
        var short_byte_mut = SliceMutU8.From(new byte[10]);
        var long_vec = SliceVec3f32.From(new Vec3f32[100_000]);
        var tupled = new Tupled();
        var callback_huge_prealloc = new CallbackHugeVecSlice(x => x[0]);
        var serviceAsync = ServiceAsync.New();
        var hello_world = "hello world".Utf8();

        MeasureResult.Calibrate(Iterations, () => {});

        result = MeasureResult.Measure(Iterations, () => Interop.primitive_void());
        writer.Add("primitive_void()", result);

        result = MeasureResult.Measure(Iterations, () => Interop.primitive_u8(0));
        writer.Add("primitive_u8(0)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.primitive_u16(0));
        writer.Add("primitive_u16(0)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.primitive_u32(0));
        writer.Add("primitive_u32(0)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.primitive_u64(0));
        writer.Add("primitive_u64(0)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_ffi_option_1(OptionInner.None));
        writer.Add("pattern_ffi_option_1(OptionInner.None)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_ffi_slice_delegate(x => x[0]));
        writer.Add("pattern_ffi_slice_delegate(x => x[0])", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_ffi_slice_delegate_huge(x => x[0]));
        writer.Add("pattern_ffi_slice_delegate_huge(x => x[0])", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_ffi_slice_delegate_huge(callback_huge_prealloc));
        writer.Add("pattern_ffi_slice_delegate_huge(callback_huge_prealloc)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_ffi_slice_2(short_vec, 0));
        writer.Add("pattern_ffi_slice_2(short_vec, 0)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_ffi_slice_2(long_vec, 0));
        writer.Add("pattern_ffi_slice_2(long_vec, 0)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_ffi_slice_4(short_byte, short_byte_mut));
        writer.Add("pattern_ffi_slice_4(short_byte, short_byte)", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_ascii_pointer_1("hello world"));
        writer.Add("pattern_ascii_pointer_1('hello world')", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_string_10("hello world".Utf8()));
        writer.Add("pattern_string_10('hello world'.Utf8())", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_string_10(hello_world.Clone()));
        writer.Add("pattern_string_10(hello_world.Clone())", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_string_11(ref hello_world));
        writer.Add("pattern_string_11(ref hello_world)", result);

        result = MeasureResult.Measure(Iterations, () => "hello world".Utf8().Dispose());
        writer.Add("'hello world'.Utf8().Dispose()", result);

        result = MeasureResult.Measure(Iterations, () => hello_world.Clone().Dispose());
        writer.Add("hello_world.Clone().Dispose()", result);

        result = MeasureResult.Measure(Iterations, () => VecU8.Empty().Dispose());
        writer.Add("VecU8.Empty().Dispose()", result);

        result = MeasureResult.Measure(Iterations, () => VecUtf8String.Empty().Dispose());
        writer.Add("VecUtf8String.Empty().Dispose()", result);

        result = MeasureResult.Measure(Iterations, () => Interop.pattern_vec_1().Dispose());
        writer.Add("pattern_vec_u8_return().Dispose()", result);

        result = MeasureResult.Measure(Iterations, async () => { await new TaskCompletionSource().Task; });
        writer.Add("await new TaskCompletionSource().Task", result);

        result = MeasureResult.Measure(Iterations, async () => { await serviceAsync.Success(); });
        writer.Add("await serviceAsync.Success()", result);

        writer.Write("RESULTS.md");
    }
}
