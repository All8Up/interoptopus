// Automatically generated by Interoptopus.

using System;
using System.Collections;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using My.Company;

namespace My.Company
{
    public static partial class InteropClass
    {
        public const string NativeLib = "unity_hot_reload";


        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "do_math")]
        public static extern uint do_math(uint x);

    }


}
