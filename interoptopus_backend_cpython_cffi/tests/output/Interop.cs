from cffi import FFI

api_definition = """




#define C1 1
#define C2 1
#define C3 -100

typedef enum EnumDocumented
    {
    A = 0,
    B = 1,
    } EnumDocumented;

typedef enum FFIError
    {
    Ok = 0,
    Fail = 200,
    } FFIError;

typedef struct Opaque Opaque;

typedef struct Empty
    {
    } Empty;

typedef struct Phantom
    {
    uint32_t x;
    } Phantom;

typedef struct SomeForeignType
    {
    uint32_t x;
    } SomeForeignType;

typedef struct StructDocumented
    {
    float x;
    } StructDocumented;

typedef struct Vec3f32
    {
    float x;
    float y;
    float z;
    } Vec3f32;

typedef uint8_t (*fptr_fn_u8_rval_u8)(uint8_t x0);

typedef struct Generic
    {
    uint32_t* x;
    } Generic;

typedef struct UseAciiStringPattern
    {
    uint8_t* asii_string;
    } UseAciiStringPattern;


uint8_t callback(fptr_fn_u8_rval_u8 callback, uint8_t value);
FFIError complex_1(Vec3f32 _a, Empty* _b);
Opaque* complex_2(SomeForeignType _cmplx);
EnumDocumented documented(StructDocumented _x);
uint32_t generic(Generic x, Phantom _y);
uint8_t pattern_simple(uint8_t* x, UseAciiStringPattern y);
bool primitive_bool(bool x);
int16_t primitive_i16(int16_t x);
int32_t primitive_i32(int32_t x);
int64_t primitive_i64(int64_t x);
int8_t primitive_i8(int8_t x);
uint16_t primitive_u16(uint16_t x);
uint32_t primitive_u32(uint32_t x);
uint64_t primitive_u64(uint64_t x);
uint8_t primitive_u8(uint8_t x);
void primitive_void();
void primitive_void2();
int64_t* ptr(int64_t* x);
int64_t* ptr_mut(int64_t* x);
bool ptr_option(int64_t* x);
bool ptr_option_mut(int64_t* x);
int64_t** ptr_ptr(int64_t** x);
int64_t* ptr_simple(int64_t* x);
int64_t* ptr_simple_mut(int64_t* x);
"""


ffi = FFI()
ffi.cdef(api_definition)
_api = None


def init_api(dll):
    """Initializes this library, call with path to DLL."""
    global _api
    _api = ffi.dlopen(dll)





C1 = 1

C2 = 1

C3 = -100


class EnumDocumented:
    """Documented enum."""
    A = 0
    B = 1


class FFIError:
    """"""
    Ok = 0
    Fail = 200




class raw:
    """Raw access to all exported functions."""
    def callback(callback, value):
        """"""
        global _api
        return _api.callback(callback, value)


    def complex_1(_a, _b):
        """"""
        global _api
        return _api.complex_1(_a, _b)


    def complex_2(_cmplx):
        """"""
        global _api
        return _api.complex_2(_cmplx)


    def documented(_x):
        """This function has documentation."""
        global _api
        return _api.documented(_x)


    def generic(x, _y):
        """"""
        global _api
        return _api.generic(x, _y)


    def pattern_simple(x, y):
        """"""
        global _api
        return _api.pattern_simple(x, y)


    def primitive_bool(x):
        """"""
        global _api
        return _api.primitive_bool(x)


    def primitive_i16(x):
        """"""
        global _api
        return _api.primitive_i16(x)


    def primitive_i32(x):
        """"""
        global _api
        return _api.primitive_i32(x)


    def primitive_i64(x):
        """"""
        global _api
        return _api.primitive_i64(x)


    def primitive_i8(x):
        """"""
        global _api
        return _api.primitive_i8(x)


    def primitive_u16(x):
        """"""
        global _api
        return _api.primitive_u16(x)


    def primitive_u32(x):
        """"""
        global _api
        return _api.primitive_u32(x)


    def primitive_u64(x):
        """"""
        global _api
        return _api.primitive_u64(x)


    def primitive_u8(x):
        """"""
        global _api
        return _api.primitive_u8(x)


    def primitive_void():
        """"""
        global _api
        return _api.primitive_void()


    def primitive_void2():
        """"""
        global _api
        return _api.primitive_void2()


    def ptr(x):
        """"""
        global _api
        return _api.ptr(x)


    def ptr_mut(x):
        """"""
        global _api
        return _api.ptr_mut(x)


    def ptr_option(x):
        """"""
        global _api
        return _api.ptr_option(x)


    def ptr_option_mut(x):
        """"""
        global _api
        return _api.ptr_option_mut(x)


    def ptr_ptr(x):
        """"""
        global _api
        return _api.ptr_ptr(x)


    def ptr_simple(x):
        """"""
        global _api
        return _api.ptr_simple(x)


    def ptr_simple_mut(x):
        """"""
        global _api
        return _api.ptr_simple_mut(x)






