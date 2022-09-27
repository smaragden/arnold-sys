use ::std::os::raw::{c_char, c_int};

#[doc = " String-based enumerated data type"]
#[doc = ""]
#[doc = " \\ref AtNode parameters of type \\c AI_TYPE_ENUM are stored as \\ref AtEnum."]
#[doc = " The \\ref AtEnum data type is just an array of strings that ends with"]
#[doc = " a NULL string (not an empty string, but a NULL pointer)."]
#[doc = ""]
#[doc = " Note that, in order to keep backwards compatibility, any value strings"]
#[doc = " beginning with a digit will be interpreted as a number."]
pub type AtEnum = *mut *const c_char;
extern "C" {
    pub fn AiEnumGetValue(enum_type: AtEnum, string: *const c_char) -> c_int;
}
extern "C" {
    pub fn AiEnumGetString(enum_type: AtEnum, index: c_int) -> *const c_char;
}
