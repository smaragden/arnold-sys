use ::std::os::raw::{c_char, c_ulong};

extern "C" {
    /// @cond internal
    pub fn AiCreateAtStringData_private(arg1: *const c_char) -> *const c_char;
}
extern "C" {
    pub fn AiAtStringLength(arg1: *const c_char) -> c_ulong;
}
extern "C" {
    pub fn AiAtStringHash(arg1: *const c_char) -> c_ulong;
}
/// Arnold String allows for fast string comparisons.  Since it is expensive to
/// create, try to create an AtString once in a preprocess, for instance in
/// \\c node_initialize, and then reuse it instead of creating it over and over, for
/// instance, in \\c shader_evaluate where it could get called millions of times.
/// For string constants, use a <tt>static const</tt> in the function, or better
/// yet, in the global scope:
///
/// ```c
/// static const AtString some_string(\"some_string\"); // this is created only once
/// void Foo() {
///    func_that_uses_string(some_string);
/// }
/// ```
///
/// The underlying string data contained by an AtString will exist over the
/// entire lifetime of the Arnold library.  If you unload the Arnold library in
/// your process, any AtString objects still in existence will become invalid
/// and using them will likely result in a crash or other undefined behavior.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtString {
    pub data: *const c_char,
}
/// Functor class to use as a hasher when you want to make a hash map or
/// hash set using AtString as a key.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtStringHash {
    pub _address: u8,
}
