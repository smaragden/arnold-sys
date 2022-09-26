use ::std::{os::raw::{c_uint, c_void}, option::Option};

use super::{
    ai_api::AtBlockingCall, ai_array::AtArray, ai_nodes::AtNode, ai_string::AtString,
    ai_universe::AtUniverse,
};

#[doc = " \\defgroup ai_render Rendering API"]
#[doc = ""]
#[doc = "  Render process operation and querying."]
#[doc = ""]
#[doc = " \\{"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtRenderSession {
    _unused: [u8; 0],
}

#[doc = "< Render from a camera"]
pub const AtRenderMode_AI_RENDER_MODE_CAMERA: AtRenderMode = 0;
#[doc = "< Process arbitrary ray-tracing requests, acting as a \"ray server\""]
pub const AtRenderMode_AI_RENDER_MODE_FREE: AtRenderMode = 1;
#[doc = " Render modes"]
pub type AtRenderMode = c_uint;
#[doc = "< no error"]
pub const AtRenderErrorCode_AI_SUCCESS: AtRenderErrorCode = 0;
#[doc = "< render aborted"]
pub const AtRenderErrorCode_AI_ABORT: AtRenderErrorCode = 1;
#[doc = "< camera not defined"]
pub const AtRenderErrorCode_AI_ERROR_NO_CAMERA: AtRenderErrorCode = 2;
#[doc = "< bad camera data"]
pub const AtRenderErrorCode_AI_ERROR_BAD_CAMERA: AtRenderErrorCode = 3;
#[doc = "< usage not validated"]
pub const AtRenderErrorCode_AI_ERROR_VALIDATION: AtRenderErrorCode = 4;
#[doc = "< invalid render region"]
pub const AtRenderErrorCode_AI_ERROR_RENDER_REGION: AtRenderErrorCode = 5;
#[doc = "< render interrupted by user"]
pub const AtRenderErrorCode_AI_INTERRUPT: AtRenderErrorCode = 6;
#[doc = "< no rendering outputs"]
pub const AtRenderErrorCode_AI_ERROR_NO_OUTPUTS: AtRenderErrorCode = 7;
#[doc = "< Cannot create GPU context"]
pub const AtRenderErrorCode_AI_ERROR_UNAVAILABLE_DEVICE: AtRenderErrorCode = 8;
#[doc = "< generic error"]
pub const AtRenderErrorCode_AI_ERROR: AtRenderErrorCode = 9;
#[doc = " Render error codes"]
pub type AtRenderErrorCode = c_uint;
#[doc = "< batch mode, extra (possibly destructive) optimizations allowed"]
pub const AtSessionMode_AI_SESSION_BATCH: AtSessionMode = 0;
#[doc = "< interactive mode, can read/write nodes after rendering"]
pub const AtSessionMode_AI_SESSION_INTERACTIVE: AtSessionMode = 1;
#[doc = " Session mode"]
#[doc = ""]
#[doc = " The session mode indicates to the renderer what the purpose of the session"]
#[doc = " is. When it is intended to set up the scene fully, render, and afterwards"]
#[doc = " end the session without modifying or reading parameters from existing nodes,"]
#[doc = " use AI_SESSION_BATCH mode.  If instead the nodes will be modified after render"]
#[doc = " or have their parameters accessed for some other reason, and then another"]
#[doc = " render invoked, use AI_SESSION_INTERACTIVE."]
pub type AtSessionMode = c_uint;
#[doc = "< no updates ready; check render status or error code"]
pub const AtDisplayOutput_AI_DISPLAY_OUTPUT_NONE: AtDisplayOutput = 0;
#[doc = "< interactive output updated fully, display on screen"]
pub const AtDisplayOutput_AI_DISPLAY_OUTPUT_INTERACTIVE: AtDisplayOutput = 1;
#[doc = "< interactive output updated but not the whole image; put on screen anyway"]
pub const AtDisplayOutput_AI_DISPLAY_OUTPUT_PARTIAL_INTERACTIVE: AtDisplayOutput = 2;
#[doc = "< all outputs are getting updated, any output can be displayed"]
pub const AtDisplayOutput_AI_DISPLAY_OUTPUT_ALL: AtDisplayOutput = 3;
#[doc = " Outputs ready for display"]
#[doc = ""]
#[doc = " This indicates which outputs are ready for display during interactive"]
#[doc = " rendering.  For faster interactive rendering, Arnold may only provide pixel"]
#[doc = " data on the main interactive output to improve responsiveness for the user."]
#[doc = " If the render gets to keep going, then Arnold will switch over to providing"]
#[doc = " data for all AOVs/outputs."]
pub type AtDisplayOutput = c_uint;
#[doc = "< Before \\ref AiRenderBegin(), or after \\ref AiRenderEnd()"]
pub const AtRenderStatus_AI_RENDER_STATUS_NOT_STARTED: AtRenderStatus = 0;
#[doc = "< Update callback paused the render or \\ref AiRenderInterrupt() called"]
pub const AtRenderStatus_AI_RENDER_STATUS_PAUSED: AtRenderStatus = 1;
#[doc = "< Update callback is restarting the render"]
pub const AtRenderStatus_AI_RENDER_STATUS_RESTARTING: AtRenderStatus = 2;
#[doc = "< Currently actively rendering passes"]
pub const AtRenderStatus_AI_RENDER_STATUS_RENDERING: AtRenderStatus = 3;
#[doc = "< Render done, but \\ref AiRenderEnd() not called yet"]
pub const AtRenderStatus_AI_RENDER_STATUS_FINISHED: AtRenderStatus = 4;
#[doc = "< Render failed, \\ref AiRenderEnd() will return the actual error code (\\ref AtRenderErrorCode)"]
pub const AtRenderStatus_AI_RENDER_STATUS_FAILED: AtRenderStatus = 5;
#[doc = " Status of the current render"]
pub type AtRenderStatus = c_uint;
#[doc = "< Callback invoked after render is interrupted and paused, can change the scene"]
pub const AtRenderUpdateType_AI_RENDER_UPDATE_INTERRUPT: AtRenderUpdateType = 0;
#[doc = "< Callback invoked just before render pass is to begin, can change the scene"]
pub const AtRenderUpdateType_AI_RENDER_UPDATE_BEFORE_PASS: AtRenderUpdateType = 1;
#[doc = "< Callback invoked during render pass after some pixel data is ready; not currently invoked, but may be in a future release"]
pub const AtRenderUpdateType_AI_RENDER_UPDATE_DURING_PASS: AtRenderUpdateType = 2;
#[doc = "< Callback invoked after a non-final render pass is done, can change the scene"]
pub const AtRenderUpdateType_AI_RENDER_UPDATE_AFTER_PASS: AtRenderUpdateType = 3;
#[doc = "< Callback invoked after the final render pass is done, can change the scene"]
pub const AtRenderUpdateType_AI_RENDER_UPDATE_FINISHED: AtRenderUpdateType = 4;
#[doc = "< Callback invoked when an error or abort occurs, and the render has failed"]
pub const AtRenderUpdateType_AI_RENDER_UPDATE_ERROR: AtRenderUpdateType = 5;
#[doc = "< Callback invoked for an imager update"]
pub const AtRenderUpdateType_AI_RENDER_UPDATE_IMAGERS: AtRenderUpdateType = 6;
#[doc = " Reason for invoking the render update callback"]
pub type AtRenderUpdateType = c_uint;
#[doc = " \\brief Additional useful information about the render, received in the render callback"]
#[doc = ""]
#[doc = " This provides additional information for the render host to decide what to show"]
#[doc = " such as bucket corners, do special handling during fast interactive passes,"]
#[doc = " and so on.  It also reports the actual sample settings for a given pass, as"]
#[doc = " fast interactive passes may have degraded (lowered) the sample settings in"]
#[doc = " order to achieve higher interactivity.  The final pass will always use the"]
#[doc = " original sample settings, however."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtRenderUpdateInfo {
    #[doc = "< Pointer to the render session that is being executed (in case the callback is used for several render sessions)"]
    pub render_session: *mut AtRenderSession,
    #[doc = "< Whether the outputs to be shown are none, partial, interactive, or all outputs"]
    pub outputs_to_display: AtDisplayOutput,
    #[doc = "< The pass number we're on out of the total number of passes, useful for indicating progress to the user"]
    pub pass_index: u32,
    #[doc = "< The total passes we expect to render, assuming no interruptions or errors"]
    pub total_passes: u32,
    #[doc = "< The actual AA_samples for the current pass (options.AA_samples remains the original value)"]
    pub current_AA_samples: i32,
    #[doc = "< The actual AA_samples_max for the current pass (options.AA_samples_max remains the original value)"]
    pub current_AA_samples_max: i32,
    #[doc = "< The actual GI_diffuse_samples for the current pass (options.GI_diffuse_samples remains the original value)"]
    pub current_GI_diffuse_samples: i32,
    #[doc = "< The actual GI_specular_samples for the current pass (options.GI_specular_samples remains the original value)"]
    pub current_GI_specular_samples: i32,
    #[doc = "< The actual GI_transmission_samples for the current pass (options.GI_transmission_samples remains the original value)"]
    pub current_GI_transmission_samples: i32,
    #[doc = "< The actual GI_sss_samples for the current pass (options.GI_sss_samples remains the original value)"]
    pub current_GI_sss_samples: i32,
    #[doc = "< The actual GI_volume_samples for the current pass (options.GI_volume_samples remains the original value)"]
    pub current_GI_volume_samples: i32,
}
#[doc = " \\brief Render update callback"]
#[doc = ""]
#[doc = " This is called in the following circumstances:"]
#[doc = "   - A render pass is about to start: update_type is"]
#[doc = "     \\ref AI_RENDER_UPDATE_BEFORE_PASS, and update_info->outputs_to_display"]
#[doc = "     will not be \\ref AI_DISPLAY_OUTPUT_NONE.  This is a chance to prepare for"]
#[doc = "     the outputs that will be displayed during this render pass. It is also"]
#[doc = "     safe to change the scene at this point."]
#[doc = "   - The render is ongoing and some number of buckets are finished: status is"]
#[doc = "     update_type is \\ref AI_RENDER_UPDATE_DURING_PASS, and"]
#[doc = "     update_info->outputs_to_display is \\ref AI_DISPLAY_OUTPUT_PARTIAL_INTERACTIVE"]
#[doc = "     or \\ref AI_DISPLAY_OUTPUT_ALL. It is NOT safe to change the scene at this"]
#[doc = "     point.  Note that the callback is not currently invoked with this update"]
#[doc = "     type, but may be in a future release."]
#[doc = "   - A render pass is done: update_type is \\ref AI_RENDER_UPDATE_AFTER_PASS,"]
#[doc = "     and update_info->outputs_to_display will not be \\ref AI_DISPLAY_OUTPUT_NONE."]
#[doc = "     It is safe to change the scene at this point."]
#[doc = "   - Rendering is completed: update_type is \\ref AI_RENDER_UPDATE_FINISHED,"]
#[doc = "     and update_info->outputs_to_display will not be \\ref AI_DISPLAY_OUTPUT_NONE."]
#[doc = "     It is safe to change the scene at this point."]
#[doc = "   - Rendering was interrupted: update_type is \\ref AI_RENDER_UPDATE_INTERRUPT,"]
#[doc = "     and update_info->outputs_to_display is \\ref AI_DISPLAY_OUTPUT_NONE if the"]
#[doc = "     interrupt happened very early in the pass, otherwise it will be"]
#[doc = "     \\ref AI_DISPLAY_OUTPUT_PARTIAL_INTERACTIVE. It is safe to change the scene"]
#[doc = "     at this point."]
#[doc = "   - Rendering has failed: update_type is \\ref AI_RENDER_UPDATE_ERROR, and"]
#[doc = "     update_info->outputs_to_display is \\ref AI_DISPLAY_OUTPUT_NONE. The"]
#[doc = "     callback should return \\ref AI_RENDER_STATUS_FAILED and host code call"]
#[doc = "     \\ref AiRenderEnd() to get the error code.  It is NOT safe to change the"]
#[doc = "     scene at this point."]
#[doc = " ."]
#[doc = " If you need the new render status within the callback, use the following"]
#[doc = " table which provides the equivalent status from the value of update_type:"]
#[doc = " <table>"]
#[doc = "  <caption id=\"render_update_type_to_status\">Render status and render update type mapping</caption>"]
#[doc = "  <tr><th>AtRenderUpdateType</th><th>AtRenderStatus</th></tr>"]
#[doc = "  <tr><td>\\ref AI_RENDER_UPDATE_INTERRUPT</td><td>\\ref AI_RENDER_STATUS_PAUSED</td></tr>"]
#[doc = "  <tr><td>\\ref AI_RENDER_UPDATE_BEFORE_PASS</td><td>\\ref AI_RENDER_STATUS_RENDERING</td></tr>"]
#[doc = "  <tr><td>\\ref AI_RENDER_UPDATE_DURING_PASS</td><td>\\ref AI_RENDER_STATUS_RENDERING</td></tr>"]
#[doc = "  <tr><td>\\ref AI_RENDER_UPDATE_AFTER_PASS</td><td>\\ref AI_RENDER_STATUS_RENDERING</td></tr>"]
#[doc = "  <tr><td>\\ref AI_RENDER_UPDATE_FINISHED</td><td>\\ref AI_RENDER_STATUS_FINISHED</td></tr>"]
#[doc = "  <tr><td>\\ref AI_RENDER_UPDATE_ERROR</td><td>\\ref AI_RENDER_STATUS_FAILED</td></tr>"]
#[doc = " </table>"]
#[doc = ""]
#[doc = " The callback returns the next status desired, but should never return"]
#[doc = " \\ref AI_RENDER_STATUS_NOT_STARTED as that is reserved."]
#[doc = ""]
#[doc = " If you change the scene during the callback, it is recommended that you return"]
#[doc = " \\ref AI_RENDER_STATUS_RESTARTING to go to the beginning of the pass sequence."]
#[doc = " When the scene doesn't change you probably want to return the status as listed"]
#[doc = " above in the \\ref render_update_type_to_status table."]
#[doc = ""]
#[doc = " \\note It is optional to have a render update callback, but is quite helpful"]
#[doc = " especially for interactive and progressive rendering (IPR). Aside from the"]
#[doc = " information in \\ref AtRenderUpdateInfo all of the controlling of the render"]
#[doc = " can be done without a callback, usually by polling \\ref AiRenderGetStatus()"]
#[doc = " for state changes."]
#[doc = ""]
#[doc = " \\param private_data  Pointer passed to \\ref AiRenderBegin(), used for whatever the callback needs"]
#[doc = " \\param update_type   Indicates when the callback is being called"]
#[doc = " \\param update_info   Extra pass, sample, and output information"]
#[doc = " \\return              The next \\ref AtRenderStatus desired"]
pub type AtRenderUpdateCallback = Option<
    unsafe extern "C" fn(
        private_data: *mut c_void,
        update_type: AtRenderUpdateType,
        update_info: *const AtRenderUpdateInfo,
    ) -> AtRenderStatus,
>;
extern "C" {
    pub fn AiBegin(mode: AtSessionMode);
}
extern "C" {
    pub fn AiEnd();
}
extern "C" {
    pub fn AiArnoldIsActive() -> bool;
}
extern "C" {
    pub fn AiRenderSession(universe: *mut AtUniverse, mode: AtSessionMode) -> *mut AtRenderSession;
}
extern "C" {
    pub fn AiRenderSessionGetUniverse(render_session: *const AtRenderSession) -> *mut AtUniverse;
}
extern "C" {
    pub fn AiRenderSessionGetOptions(render_session: *const AtRenderSession) -> *const AtNode;
}
extern "C" {
    pub fn AiRenderSessionDestroy(render_session: *mut AtRenderSession);
}
extern "C" {
    pub fn AiGetSessionMode(render_session: *const AtRenderSession) -> AtSessionMode;
}
extern "C" {
    pub fn AiRenderAddInteractiveOutput(render_session: *mut AtRenderSession, output_index: u32);
}
extern "C" {
    pub fn AiRenderIsInteractiveOutput(
        render_session: *mut AtRenderSession,
        output_index: u32,
    ) -> bool;
}
extern "C" {
    pub fn AiRenderRemoveInteractiveOutput(
        render_session: *mut AtRenderSession,
        output_index: u32,
    ) -> bool;
}
extern "C" {
    pub fn AiRenderRemoveAllInteractiveOutputs(render_session: *mut AtRenderSession);
}
extern "C" {
    pub fn AiRenderSetInteractiveOutput(output_index: u32);
}
extern "C" {
    pub fn AiRenderGetInteractiveOutput() -> u32;
}
extern "C" {
    pub fn AiRenderSetHintBool(
        render_session: *mut AtRenderSession,
        hint: AtString,
        value: bool,
    ) -> bool;
}
extern "C" {
    pub fn AiRenderSetHintInt(
        render_session: *mut AtRenderSession,
        hint: AtString,
        value: i32,
    ) -> bool;
}
extern "C" {
    pub fn AiRenderSetHintFlt(
        render_session: *mut AtRenderSession,
        hint: AtString,
        value: f32,
    ) -> bool;
}
extern "C" {
    pub fn AiRenderSetHintStr(
        render_session: *mut AtRenderSession,
        hint: AtString,
        value: AtString,
    ) -> bool;
}
extern "C" {
    pub fn AiRenderSetHintArray(
        render_session: *mut AtRenderSession,
        hint: AtString,
        value: *mut AtArray,
    ) -> bool;
}
extern "C" {
    pub fn AiRenderGetHintBool(
        render_session: *const AtRenderSession,
        hint: AtString,
        value: *mut bool,
    ) -> bool;
}
extern "C" {
    pub fn AiRenderGetHintInt(
        render_session: *const AtRenderSession,
        hint: AtString,
        value: *mut i32,
    ) -> bool;
}
extern "C" {
    pub fn AiRenderGetHintFlt(
        render_session: *const AtRenderSession,
        hint: AtString,
        value: *mut f32,
    ) -> bool;
}
extern "C" {
    pub fn AiRenderGetHintStr(
        render_session: *const AtRenderSession,
        hint: AtString,
        value: *mut AtString,
    ) -> bool;
}
extern "C" {
    pub fn AiRenderGetHintArray(
        render_session: *const AtRenderSession,
        hint: AtString,
        value: *mut *const AtArray,
    ) -> bool;
}
extern "C" {
    pub fn AiRenderBegin(
        render_session: *mut AtRenderSession,
        mode: AtRenderMode,
        update_callback: AtRenderUpdateCallback,
        callback_private_data: *mut c_void,
    ) -> AtRenderErrorCode;
}
extern "C" {
    pub fn AiRenderEnd(render_session: *mut AtRenderSession) -> AtRenderErrorCode;
}
extern "C" {
    pub fn AiRenderGetStatus(render_session: *const AtRenderSession) -> AtRenderStatus;
}
extern "C" {
    pub fn AiRenderInterrupt(render_session: *mut AtRenderSession, blocking: AtBlockingCall);
}
extern "C" {
    pub fn AiRenderAbort(render_session: *mut AtRenderSession, blocking: AtBlockingCall);
}
extern "C" {
    pub fn AiRenderResume(render_session: *mut AtRenderSession);
}
extern "C" {
    pub fn AiRenderRestart(render_session: *mut AtRenderSession);
}
extern "C" {
    pub fn AiRenderIsAnyActive() -> bool;
}
extern "C" {
    pub fn AiRender(render_session: *mut AtRenderSession, mode: AtRenderMode) -> AtRenderErrorCode;
}
extern "C" {
    pub fn AiRendering() -> bool;
}
