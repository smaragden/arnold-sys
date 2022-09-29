use ::std::os::raw::{c_char, c_uint};

use super::{ai_nodes::AtNode, ai_string::AtString};

/// Overwrite the file if it exists
pub const AtStatsMode_AI_STATS_MODE_OVERWRITE: AtStatsMode = 0;
/// Append data to existing statistics file
pub const AtStatsMode_AI_STATS_MODE_APPEND: AtStatsMode = 1;
/// Output mode for structured statistics
pub type AtStatsMode = c_uint;
extern "C" {
    pub fn AiStatsGetMode() -> AtStatsMode;

    pub fn AiStatsGetFileName() -> *const c_char;

    pub fn AiStatsSetMode(mode: AtStatsMode);

    pub fn AiStatsSetFileName(filename: *const c_char);

    /// JSON file to which profiling traces should be written to.  This is written
    /// in the Trace Event format which is viewable in Google Chrome at
    /// chrome://tracing/ . Setting to NULL will disable output.
    pub fn AiProfileSetFileName(filename: *const c_char);

    /// Get the JSON filename to which profiling traces will be written to.
    /// \\see AiProfileSetFileName()
    pub fn AiProfileGetFileName() -> AtString;

    /// @private
    /// Finalize profile entry
    ///
    /// \\warning Please do not call AiProfileUpdate() directly.
    /// \\see AiProfileEnd()
    pub fn AiProfileUpdate(
        start_counter: u64,
        end_counter: u64,
        name: *const c_char,
        node: *const AtNode,
    );

    /// @private
    /// Get profile counter
    ///
    /// \\warning Please use AiProfileBlock() instead of directly using
    /// AiProfileCounter() and AiProfileEnd().
    pub fn AiProfileCounter() -> u64;
}
/// @private
#[repr(C)]
#[derive(Debug)]
pub struct AiProfileBlockRAII {
    pub start_counter: u64,
    pub name: *const c_char,
    pub node: *const AtNode,
}
