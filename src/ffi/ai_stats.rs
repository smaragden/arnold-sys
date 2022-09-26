use super::{ai_nodes::AtNode, ai_string::AtString};

#[doc = "< Overwrite the file if it exists"]
pub const AtStatsMode_AI_STATS_MODE_OVERWRITE: AtStatsMode = 0;
#[doc = "< Append data to existing statistics file"]
pub const AtStatsMode_AI_STATS_MODE_APPEND: AtStatsMode = 1;
#[doc = " Output mode for structured statistics"]
pub type AtStatsMode = ::std::os::raw::c_uint;
extern "C" {
    pub fn AiStatsGetMode() -> AtStatsMode;
}
extern "C" {
    pub fn AiStatsGetFileName() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn AiStatsSetMode(mode: AtStatsMode);
}
extern "C" {
    pub fn AiStatsSetFileName(filename: *const ::std::os::raw::c_char);
}
extern "C" {
    #[doc = " JSON file to which profiling traces should be written to.  This is written"]
    #[doc = " in the Trace Event format which is viewable in Google Chrome at"]
    #[doc = " chrome://tracing/ . Setting to NULL will disable output."]
    pub fn AiProfileSetFileName(filename: *const ::std::os::raw::c_char);
}
extern "C" {
    #[doc = " Get the JSON filename to which profiling traces will be written to."]
    #[doc = " \\see AiProfileSetFileName()"]
    pub fn AiProfileGetFileName() -> AtString;
}

extern "C" {
    #[doc = " @private"]
    #[doc = " Finalize profile entry"]
    #[doc = ""]
    #[doc = " \\warning Please do not call AiProfileUpdate() directly."]
    #[doc = " \\see AiProfileEnd()"]
    pub fn AiProfileUpdate(
        start_counter: u64,
        end_counter: u64,
        name: *const ::std::os::raw::c_char,
        node: *const AtNode,
    );
}
extern "C" {
    #[doc = " @private"]
    #[doc = " Get profile counter"]
    #[doc = ""]
    #[doc = " \\warning Please use AiProfileBlock() instead of directly using"]
    #[doc = " AiProfileCounter() and AiProfileEnd()."]
    pub fn AiProfileCounter() -> u64;
}
#[doc = " @private"]
#[repr(C)]
#[derive(Debug)]
pub struct AiProfileBlockRAII {
    pub start_counter: u64,
    pub name: *const ::std::os::raw::c_char,
    pub node: *const AtNode,
}
