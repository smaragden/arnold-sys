#![allow(non_camel_case_types, non_snake_case)]
use std::ffi::{c_int, CString};
use std::path::PathBuf;

use bitflags::bitflags;

pub use arnold_sys::ai_msg::AI_SEVERITY_ERROR;
pub use arnold_sys::ai_msg::AI_SEVERITY_FATAL;
pub use arnold_sys::ai_msg::AI_SEVERITY_INFO;
pub use arnold_sys::ai_msg::AI_SEVERITY_WARNING;

pub use arnold_sys::ai_msg::AI_LOG_ALL;
pub use arnold_sys::ai_msg::AI_LOG_ASS_PARSE;
pub use arnold_sys::ai_msg::AI_LOG_BACKTRACE;
pub use arnold_sys::ai_msg::AI_LOG_COLOR;
pub use arnold_sys::ai_msg::AI_LOG_DEBUG;
pub use arnold_sys::ai_msg::AI_LOG_ERRORS;
pub use arnold_sys::ai_msg::AI_LOG_INFO;
pub use arnold_sys::ai_msg::AI_LOG_MEMORY;
pub use arnold_sys::ai_msg::AI_LOG_NAN;
pub use arnold_sys::ai_msg::AI_LOG_NONE;
pub use arnold_sys::ai_msg::AI_LOG_PLUGINS;
pub use arnold_sys::ai_msg::AI_LOG_PROGRESS;
pub use arnold_sys::ai_msg::AI_LOG_STATS;
pub use arnold_sys::ai_msg::AI_LOG_TIMESTAMP;
pub use arnold_sys::ai_msg::AI_LOG_WARNINGS;

use crate::ai_universe::AtUniverse;


bitflags! {
    pub struct LogFlags: u32 {
        const AI_LOG_NONE = arnold_sys::ai_msg::AI_LOG_NONE;
        const AI_LOG_INFO = arnold_sys::ai_msg::AI_LOG_INFO;
        const AI_LOG_WARNINGS = arnold_sys::ai_msg::AI_LOG_WARNINGS;
        const AI_LOG_ERRORS = arnold_sys::ai_msg::AI_LOG_ERRORS;
        const AI_LOG_DEBUG = arnold_sys::ai_msg::AI_LOG_DEBUG;
        const AI_LOG_STATS = arnold_sys::ai_msg::AI_LOG_STATS;
        const AI_LOG_ASS_PARSE = arnold_sys::ai_msg::AI_LOG_ASS_PARSE;
        const AI_LOG_PLUGINS = arnold_sys::ai_msg::AI_LOG_PLUGINS;
        const AI_LOG_PROGRESS = arnold_sys::ai_msg::AI_LOG_PROGRESS;
        const AI_LOG_NAN = arnold_sys::ai_msg::AI_LOG_NAN;
        const AI_LOG_TIMESTAMP = arnold_sys::ai_msg::AI_LOG_TIMESTAMP;
        const AI_LOG_BACKTRACE = arnold_sys::ai_msg::AI_LOG_BACKTRACE;
        const AI_LOG_MEMORY = arnold_sys::ai_msg::AI_LOG_MEMORY;
        const AI_LOG_COLOR = arnold_sys::ai_msg::AI_LOG_COLOR;
        const AI_LOG_ALL = arnold_sys::ai_msg::AI_LOG_ALL;
    }
}

bitflags! {
    pub struct LogSeverity: u32 {
        const AI_SEVERITY_INFO = arnold_sys::ai_msg::AI_SEVERITY_INFO;
        const AI_SEVERITY_WARNING = arnold_sys::ai_msg::AI_SEVERITY_WARNING;
        const AI_SEVERITY_ERROR = arnold_sys::ai_msg::AI_SEVERITY_ERROR;
        const AI_SEVERITY_FATAL = arnold_sys::ai_msg::AI_SEVERITY_FATAL;
    }
}

pub fn AiMsgSetLogFileName<T: Into<PathBuf>>(filename: T) {
    let c_str = CString::new(filename.into().to_str().unwrap()).unwrap();
    unsafe { arnold_sys::ai_msg::AiMsgSetLogFileName(c_str.as_ptr()) };
}

pub fn AiMsgSetConsoleFlags(universe: Option<AtUniverse>, flags: LogFlags) {
    let universe = match universe {
        Some(universe) => universe.raw(),
        None => std::ptr::null_mut(),
    };
    unsafe {
        arnold_sys::ai_msg::AiMsgSetConsoleFlags(
            universe,
            flags.bits as c_int,
        )
    };
}
