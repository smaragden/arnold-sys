pub const AI_MAX_THREADS: u32 = 256;
pub const AI_PRIORITY_LOWEST: u32 = 0;
pub const AI_PRIORITY_LOW: u32 = 1;
pub const AI_PRIORITY_NORMAL: u32 = 2;
pub const AI_PRIORITY_HIGH: u32 = 3;
extern "C" {
    #[doc = " \\defgroup ai_threads Thread Management API"]
    #[doc = ""]
    #[doc = " Implementation of platform-agnostic CPU threads."]
    #[doc = ""]
    #[doc = " \\{"]
    pub fn AiThreadCreate(
        fn_: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_uint,
        >,
        data: *mut ::std::os::raw::c_void,
        priority: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn AiThreadClose(thread: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn AiThreadWait(thread: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn AiThreadSelf() -> *mut ::std::os::raw::c_void;
}
