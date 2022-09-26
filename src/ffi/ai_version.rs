use std::os::raw::c_char;

extern "C" {
    pub fn AiGetVersion(
        arch: *mut c_char,
        major: *mut c_char,
        minor: *mut c_char,
        fix: *mut c_char,
    ) -> *const c_char;

    pub fn AiGetVersionInfo() -> *const c_char;

    pub fn AiGetCompileOptions() -> *const c_char;

    pub fn AiCheckAPIVersion(
        arch: *const c_char,
        major: *const c_char,
        minor: *const c_char,
    ) -> bool;

    pub fn AiSetAppString(appstr: *const c_char);

    pub fn AiGetCopyrightNotices(copyright_notice_type: AtCopyrightNoticeType) -> *const c_char;
}

#[repr(u32)]
#[doc = "Copyright Notice types"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AtCopyrightNoticeType {
    AI_COPYRIGHT_NOTICES_CORE = 0,
    AI_COPYRIGHT_NOTICES_PLUGINS = 1,
}
