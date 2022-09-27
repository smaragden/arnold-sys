use ::std::os::raw::c_void;

use super::ai_string::AtString;

extern "C" {
    /// \\defgroup ai_allocate Memory Allocation API
    ///
    /// Memory allocation and deallocation.
    ///
    /// \\{
    pub fn AiMalloc(size: usize) -> *mut c_void;

    pub fn AiRealloc(addr: *mut c_void, size: usize) -> *mut c_void;

    pub fn AiFree(addr: *mut c_void);

    pub fn AiAddMemUsage(size: i64, category: AtString);
}
