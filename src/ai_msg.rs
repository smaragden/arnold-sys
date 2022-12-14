use ::std::{
    option::Option,
    os::raw::{c_char, c_int, c_uint, c_ulong, c_void},
};

use super::{ai_map::AtParamValueMap, ai_universe::AtUniverse};

pub const AI_SEVERITY_INFO: u32 = 0;
pub const AI_SEVERITY_WARNING: u32 = 1;
pub const AI_SEVERITY_ERROR: u32 = 2;
pub const AI_SEVERITY_FATAL: u32 = 3;
pub const AI_LOG_NONE: u32 = 0;
pub const AI_LOG_INFO: u32 = 1;
pub const AI_LOG_WARNINGS: u32 = 2;
pub const AI_LOG_ERRORS: u32 = 4;
pub const AI_LOG_DEBUG: u32 = 8;
pub const AI_LOG_STATS: u32 = 16;
pub const AI_LOG_ASS_PARSE: u32 = 32;
pub const AI_LOG_PLUGINS: u32 = 64;
pub const AI_LOG_PROGRESS: u32 = 128;
pub const AI_LOG_NAN: u32 = 256;
pub const AI_LOG_TIMESTAMP: u32 = 512;
pub const AI_LOG_BACKTRACE: u32 = 1024;
pub const AI_LOG_MEMORY: u32 = 2048;
pub const AI_LOG_COLOR: u32 = 4096;
pub const AI_LOG_ALL: u32 = 8191;

/// Custom message callback, as passed to AiMsgSetCallback()
pub type AtMsgCallBack = Option<
    unsafe extern "C" fn(logmask: c_int, severity: c_int, msg_string: *const c_char, tabs: c_int),
>;
/// Custom message callback, as passed to AiMsgRegisterCallback()
///
/// The following items will be passed to the callback through the \\c metadata list:
///
///  <table>
///  <tr><th>Name<th>Type<th>Description
///  <tr><td>universe<td> Universe*<td>Universe this log message refers to
///  </table>
pub type AtMsgExtendedCallBack = Option<
    unsafe extern "C" fn(
        logmask: c_int,
        severity: c_int,
        msg_string: *const c_char,
        metadata: *mut AtParamValueMap,
        user_ptr: *mut c_void,
    ),
>;
extern "C" {
    pub fn AiMsgSetLogFileName(filename: *const c_char);

    pub fn AiMsgSetLogFileFlags(universe: *const AtUniverse, flags: c_int);

    pub fn AiMsgSetConsoleFlags(universe: *const AtUniverse, flags: c_int);

    pub fn AiMsgGetLogFileFlags(universe: *const AtUniverse) -> c_int;

    pub fn AiMsgGetConsoleFlags(universe: *const AtUniverse) -> c_int;

    pub fn AiMsgSetMaxWarnings(max_warnings: c_int);

    pub fn AiMsgSetCallback(func: AtMsgCallBack);

    pub fn AiMsgAddCallback(func: AtMsgCallBack);

    pub fn AiMsgResetCallback();

    pub fn AiMsgRegisterCallback(
        func: AtMsgExtendedCallBack,
        mask: c_int,
        user_ptr: *mut c_void,
    ) -> c_uint;

    pub fn AiMsgDeregisterCallback(callback_id: c_uint);

    pub fn AiMsgSetCallbackMask(callback_id: c_uint, mask: c_int);

    pub fn AiMsgGetCallbackMask(callback_id: c_uint) -> c_int;

    pub fn AiMsgInfo(format: *const c_char, ...);

    pub fn AiMsgDebug(format: *const c_char, ...);

    pub fn AiMsgWarning(format: *const c_char, ...);

    pub fn AiMsgError(format: *const c_char, ...);

    pub fn AiMsgFatal(format: *const c_char, ...);

    pub fn AiMsgTab(tabinc: c_int);

    pub fn AiMsgUtilGetUsedMemory() -> c_ulong;

    pub fn AiMsgUtilGetElapsedTime() -> u32;
}
