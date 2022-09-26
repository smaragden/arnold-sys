extern "C" {
    #[doc = " @cond internal"]
    pub fn AiCreateAtStringData_private(
        arg1: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn AiAtStringLength(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn AiAtStringHash(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_ulong;
}
#[doc = " Arnold String allows for fast string comparisons.  Since it is expensive to"]
#[doc = " create, try to create an AtString once in a preprocess, for instance in"]
#[doc = " \\c node_initialize, and then reuse it instead of creating it over and over, for"]
#[doc = " instance, in \\c shader_evaluate where it could get called millions of times."]
#[doc = " For string constants, use a <tt>static const</tt> in the function, or better"]
#[doc = " yet, in the global scope:"]
#[doc = ""]
#[doc = " \\code"]
#[doc = " static const AtString some_string(\"some_string\"); // this is created only once"]
#[doc = " void Foo() {"]
#[doc = "    func_that_uses_string(some_string);"]
#[doc = " }"]
#[doc = " \\endcode"]
#[doc = ""]
#[doc = " The underlying string data contained by an AtString will exist over the"]
#[doc = " entire lifetime of the Arnold library.  If you unload the Arnold library in"]
#[doc = " your process, any AtString objects still in existence will become invalid"]
#[doc = " and using them will likely result in a crash or other undefined behavior."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtString {
    pub data: *const ::std::os::raw::c_char,
}
#[doc = " Functor class to use as a hasher when you want to make a hash map or"]
#[doc = " hash set using AtString as a key."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtStringHash {
    pub _address: u8,
}
