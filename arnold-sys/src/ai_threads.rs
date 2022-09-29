use ::std::{
    option::Option,
    os::raw::{c_int, c_uint, c_void},
};

pub const AI_MAX_THREADS: u32 = 256;
pub const AI_PRIORITY_LOWEST: u32 = 0;
pub const AI_PRIORITY_LOW: u32 = 1;
pub const AI_PRIORITY_NORMAL: u32 = 2;
pub const AI_PRIORITY_HIGH: u32 = 3;
extern "C" {
    /// \\defgroup ai_threads Thread Management API
    ///
    /// Implementation of platform-agnostic CPU threads.
    ///
    /// \\{
    pub fn AiThreadCreate(
        fn_: Option<unsafe extern "C" fn(arg1: *mut c_void) -> c_uint>,
        data: *mut c_void,
        priority: c_int,
    ) -> *mut c_void;

    pub fn AiThreadClose(thread: *mut c_void);

    pub fn AiThreadWait(thread: *mut c_void);

    pub fn AiThreadSelf() -> *mut c_void;
}
