use ::std::os::raw::c_void;

use super::ai_string::AtString;

extern "C" {
    #[doc = " \\defgroup ai_allocate Memory Allocation API"]
    #[doc = ""]
    #[doc = " Memory allocation and deallocation."]
    #[doc = ""]
    #[doc = " \\{"]
    pub fn AiMalloc(size: usize) -> *mut c_void;
}
extern "C" {
    pub fn AiRealloc(addr: *mut c_void, size: usize)
        -> *mut c_void;
}
extern "C" {
    pub fn AiFree(addr: *mut c_void);
}
extern "C" {
    pub fn AiAddMemUsage(size: i64, category: AtString);
}
