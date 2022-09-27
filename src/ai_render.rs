use ::std::{
    option::Option,
    os::raw::{c_uint, c_void},
};

use super::{
    ai_api::AtBlockingCall, ai_array::AtArray, ai_nodes::AtNode, ai_string::AtString,
    ai_universe::AtUniverse,
};

/// \\defgroup ai_render Rendering API
///
///  Render process operation and querying.
///
/// \\{
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtRenderSession {
    _unused: [u8; 0],
}

/// Render from a camera
pub const AtRenderMode_AI_RENDER_MODE_CAMERA: AtRenderMode = 0;
/// Process arbitrary ray-tracing requests, acting as a \"ray server\"
pub const AtRenderMode_AI_RENDER_MODE_FREE: AtRenderMode = 1;
/// Render modes
pub type AtRenderMode = c_uint;
/// no error
pub const AtRenderErrorCode_AI_SUCCESS: AtRenderErrorCode = 0;
/// render aborted
pub const AtRenderErrorCode_AI_ABORT: AtRenderErrorCode = 1;
/// camera not defined
pub const AtRenderErrorCode_AI_ERROR_NO_CAMERA: AtRenderErrorCode = 2;
/// bad camera data
pub const AtRenderErrorCode_AI_ERROR_BAD_CAMERA: AtRenderErrorCode = 3;
/// usage not validated
pub const AtRenderErrorCode_AI_ERROR_VALIDATION: AtRenderErrorCode = 4;
/// invalid render region
pub const AtRenderErrorCode_AI_ERROR_RENDER_REGION: AtRenderErrorCode = 5;
/// render interrupted by user
pub const AtRenderErrorCode_AI_INTERRUPT: AtRenderErrorCode = 6;
/// no rendering outputs
pub const AtRenderErrorCode_AI_ERROR_NO_OUTPUTS: AtRenderErrorCode = 7;
/// Cannot create GPU context
pub const AtRenderErrorCode_AI_ERROR_UNAVAILABLE_DEVICE: AtRenderErrorCode = 8;
/// generic error
pub const AtRenderErrorCode_AI_ERROR: AtRenderErrorCode = 9;
/// Render error codes
pub type AtRenderErrorCode = c_uint;
/// batch mode, extra (possibly destructive) optimizations allowed
pub const AtSessionMode_AI_SESSION_BATCH: AtSessionMode = 0;
/// interactive mode, can read/write nodes after rendering
pub const AtSessionMode_AI_SESSION_INTERACTIVE: AtSessionMode = 1;
/// Session mode
///
/// The session mode indicates to the renderer what the purpose of the session
/// is. When it is intended to set up the scene fully, render, and afterwards
/// end the session without modifying or reading parameters from existing nodes,
/// use AI_SESSION_BATCH mode.  If instead the nodes will be modified after render
/// or have their parameters accessed for some other reason, and then another
/// render invoked, use AI_SESSION_INTERACTIVE.
pub type AtSessionMode = c_uint;
/// no updates ready; check render status or error code
pub const AtDisplayOutput_AI_DISPLAY_OUTPUT_NONE: AtDisplayOutput = 0;
/// interactive output updated fully, display on screen
pub const AtDisplayOutput_AI_DISPLAY_OUTPUT_INTERACTIVE: AtDisplayOutput = 1;
/// interactive output updated but not the whole image; put on screen anyway
pub const AtDisplayOutput_AI_DISPLAY_OUTPUT_PARTIAL_INTERACTIVE: AtDisplayOutput = 2;
/// all outputs are getting updated, any output can be displayed
pub const AtDisplayOutput_AI_DISPLAY_OUTPUT_ALL: AtDisplayOutput = 3;
/// Outputs ready for display
///
/// This indicates which outputs are ready for display during interactive
/// rendering.  For faster interactive rendering, Arnold may only provide pixel
/// data on the main interactive output to improve responsiveness for the user.
/// If the render gets to keep going, then Arnold will switch over to providing
/// data for all AOVs/outputs.
pub type AtDisplayOutput = c_uint;
/// Before `AiRenderBegin`(), or after `AiRenderEnd`()
pub const AtRenderStatus_AI_RENDER_STATUS_NOT_STARTED: AtRenderStatus = 0;
/// Update callback paused the render or `AiRenderInterrupt`() called
pub const AtRenderStatus_AI_RENDER_STATUS_PAUSED: AtRenderStatus = 1;
/// Update callback is restarting the render
pub const AtRenderStatus_AI_RENDER_STATUS_RESTARTING: AtRenderStatus = 2;
/// Currently actively rendering passes
pub const AtRenderStatus_AI_RENDER_STATUS_RENDERING: AtRenderStatus = 3;
/// Render done, but `AiRenderEnd`() not called yet
pub const AtRenderStatus_AI_RENDER_STATUS_FINISHED: AtRenderStatus = 4;
/// Render failed, `AiRenderEnd`() will return the actual error code (`AtRenderErrorCode`)
pub const AtRenderStatus_AI_RENDER_STATUS_FAILED: AtRenderStatus = 5;
/// Status of the current render
pub type AtRenderStatus = c_uint;
/// Callback invoked after render is interrupted and paused, can change the scene
pub const AtRenderUpdateType_AI_RENDER_UPDATE_INTERRUPT: AtRenderUpdateType = 0;
/// Callback invoked just before render pass is to begin, can change the scene
pub const AtRenderUpdateType_AI_RENDER_UPDATE_BEFORE_PASS: AtRenderUpdateType = 1;
/// Callback invoked during render pass after some pixel data is ready; not currently invoked, but may be in a future release
pub const AtRenderUpdateType_AI_RENDER_UPDATE_DURING_PASS: AtRenderUpdateType = 2;
/// Callback invoked after a non-final render pass is done, can change the scene
pub const AtRenderUpdateType_AI_RENDER_UPDATE_AFTER_PASS: AtRenderUpdateType = 3;
/// Callback invoked after the final render pass is done, can change the scene
pub const AtRenderUpdateType_AI_RENDER_UPDATE_FINISHED: AtRenderUpdateType = 4;
/// Callback invoked when an error or abort occurs, and the render has failed
pub const AtRenderUpdateType_AI_RENDER_UPDATE_ERROR: AtRenderUpdateType = 5;
/// Callback invoked for an imager update
pub const AtRenderUpdateType_AI_RENDER_UPDATE_IMAGERS: AtRenderUpdateType = 6;
/// Reason for invoking the render update callback
pub type AtRenderUpdateType = c_uint;
/// \\brief Additional useful information about the render, received in the render callback
///
/// This provides additional information for the render host to decide what to show
/// such as bucket corners, do special handling during fast interactive passes,
/// and so on.  It also reports the actual sample settings for a given pass, as
/// fast interactive passes may have degraded (lowered) the sample settings in
/// order to achieve higher interactivity.  The final pass will always use the
/// original sample settings, however.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtRenderUpdateInfo {
    /// Pointer to the render session that is being executed (in case the callback is used for several render sessions)
    pub render_session: *mut AtRenderSession,
    /// Whether the outputs to be shown are none, partial, interactive, or all outputs
    pub outputs_to_display: AtDisplayOutput,
    /// The pass number we're on out of the total number of passes, useful for indicating progress to the user
    pub pass_index: u32,
    /// The total passes we expect to render, assuming no interruptions or errors
    pub total_passes: u32,
    /// The actual AA_samples for the current pass (options.AA_samples remains the original value)
    pub current_AA_samples: i32,
    /// The actual AA_samples_max for the current pass (options.AA_samples_max remains the original value)
    pub current_AA_samples_max: i32,
    /// The actual GI_diffuse_samples for the current pass (options.GI_diffuse_samples remains the original value)
    pub current_GI_diffuse_samples: i32,
    /// The actual GI_specular_samples for the current pass (options.GI_specular_samples remains the original value)
    pub current_GI_specular_samples: i32,
    /// The actual GI_transmission_samples for the current pass (options.GI_transmission_samples remains the original value)
    pub current_GI_transmission_samples: i32,
    /// The actual GI_sss_samples for the current pass (options.GI_sss_samples remains the original value)
    pub current_GI_sss_samples: i32,
    /// The actual GI_volume_samples for the current pass (options.GI_volume_samples remains the original value)
    pub current_GI_volume_samples: i32,
}
/// \\brief Render update callback
///
/// This is called in the following circumstances:
///   - A render pass is about to start: update_type is
///     `AI`_RENDER_UPDATE_BEFORE_PASS, and update_info->outputs_to_display
///     will not be `AI`_DISPLAY_OUTPUT_NONE.  This is a chance to prepare for
///     the outputs that will be displayed during this render pass. It is also
///     safe to change the scene at this point.
///   - The render is ongoing and some number of buckets are finished: status is
///     update_type is `AI`_RENDER_UPDATE_DURING_PASS, and
///     update_info->outputs_to_display is `AI`_DISPLAY_OUTPUT_PARTIAL_INTERACTIVE
///     or `AI`_DISPLAY_OUTPUT_ALL. It is NOT safe to change the scene at this
///     point.  Note that the callback is not currently invoked with this update
///     type, but may be in a future release.
///   - A render pass is done: update_type is `AI`_RENDER_UPDATE_AFTER_PASS,
///     and update_info->outputs_to_display will not be `AI`_DISPLAY_OUTPUT_NONE.
///     It is safe to change the scene at this point.
///   - Rendering is completed: update_type is `AI`_RENDER_UPDATE_FINISHED,
///     and update_info->outputs_to_display will not be `AI`_DISPLAY_OUTPUT_NONE.
///     It is safe to change the scene at this point.
///   - Rendering was interrupted: update_type is `AI`_RENDER_UPDATE_INTERRUPT,
///     and update_info->outputs_to_display is `AI`_DISPLAY_OUTPUT_NONE if the
///     interrupt happened very early in the pass, otherwise it will be
///     `AI`_DISPLAY_OUTPUT_PARTIAL_INTERACTIVE. It is safe to change the scene
///     at this point.
///   - Rendering has failed: update_type is `AI`_RENDER_UPDATE_ERROR, and
///     update_info->outputs_to_display is `AI`_DISPLAY_OUTPUT_NONE. The
///     callback should return `AI`_RENDER_STATUS_FAILED and host code call
///     `AiRenderEnd`() to get the error code.  It is NOT safe to change the
///     scene at this point.
/// .
/// If you need the new render status within the callback, use the following
/// table which provides the equivalent status from the value of update_type:
/// <table>
///  <caption id=\"render_update_type_to_status\">Render status and render update type mapping</caption>
///  <tr><th>AtRenderUpdateType</th><th>AtRenderStatus</th></tr>
///  <tr><td>`AI`_RENDER_UPDATE_INTERRUPT</td><td>`AI`_RENDER_STATUS_PAUSED</td></tr>
///  <tr><td>`AI`_RENDER_UPDATE_BEFORE_PASS</td><td>`AI`_RENDER_STATUS_RENDERING</td></tr>
///  <tr><td>`AI`_RENDER_UPDATE_DURING_PASS</td><td>`AI`_RENDER_STATUS_RENDERING</td></tr>
///  <tr><td>`AI`_RENDER_UPDATE_AFTER_PASS</td><td>`AI`_RENDER_STATUS_RENDERING</td></tr>
///  <tr><td>`AI`_RENDER_UPDATE_FINISHED</td><td>`AI`_RENDER_STATUS_FINISHED</td></tr>
///  <tr><td>`AI`_RENDER_UPDATE_ERROR</td><td>`AI`_RENDER_STATUS_FAILED</td></tr>
/// </table>
///
/// The callback returns the next status desired, but should never return
/// `AI`_RENDER_STATUS_NOT_STARTED as that is reserved.
///
/// If you change the scene during the callback, it is recommended that you return
/// `AI`_RENDER_STATUS_RESTARTING to go to the beginning of the pass sequence.
/// When the scene doesn't change you probably want to return the status as listed
/// above in the `render`_update_type_to_status table.
///
/// \\note It is optional to have a render update callback, but is quite helpful
/// especially for interactive and progressive rendering (IPR). Aside from the
/// information in `AtRenderUpdateInfo` all of the controlling of the render
/// can be done without a callback, usually by polling `AiRenderGetStatus`()
/// for state changes.
///
/// \\param private_data  Pointer passed to `AiRenderBegin`(), used for whatever the callback needs
/// \\param update_type   Indicates when the callback is being called
/// \\param update_info   Extra pass, sample, and output information
/// \\return              The next `AtRenderStatus` desired
pub type AtRenderUpdateCallback = Option<
    unsafe extern "C" fn(
        private_data: *mut c_void,
        update_type: AtRenderUpdateType,
        update_info: *const AtRenderUpdateInfo,
    ) -> AtRenderStatus,
>;
extern "C" {
    pub fn AiBegin(mode: AtSessionMode);

    pub fn AiEnd();

    pub fn AiArnoldIsActive() -> bool;

    pub fn AiRenderSession(universe: *mut AtUniverse, mode: AtSessionMode) -> *mut AtRenderSession;

    pub fn AiRenderSessionGetUniverse(render_session: *const AtRenderSession) -> *mut AtUniverse;

    pub fn AiRenderSessionGetOptions(render_session: *const AtRenderSession) -> *const AtNode;

    pub fn AiRenderSessionDestroy(render_session: *mut AtRenderSession);

    pub fn AiGetSessionMode(render_session: *const AtRenderSession) -> AtSessionMode;

    pub fn AiRenderAddInteractiveOutput(render_session: *mut AtRenderSession, output_index: u32);

    pub fn AiRenderIsInteractiveOutput(
        render_session: *mut AtRenderSession,
        output_index: u32,
    ) -> bool;

    pub fn AiRenderRemoveInteractiveOutput(
        render_session: *mut AtRenderSession,
        output_index: u32,
    ) -> bool;

    pub fn AiRenderRemoveAllInteractiveOutputs(render_session: *mut AtRenderSession);

    pub fn AiRenderSetInteractiveOutput(output_index: u32);

    pub fn AiRenderGetInteractiveOutput() -> u32;

    pub fn AiRenderSetHintBool(
        render_session: *mut AtRenderSession,
        hint: AtString,
        value: bool,
    ) -> bool;

    pub fn AiRenderSetHintInt(
        render_session: *mut AtRenderSession,
        hint: AtString,
        value: i32,
    ) -> bool;

    pub fn AiRenderSetHintFlt(
        render_session: *mut AtRenderSession,
        hint: AtString,
        value: f32,
    ) -> bool;

    pub fn AiRenderSetHintStr(
        render_session: *mut AtRenderSession,
        hint: AtString,
        value: AtString,
    ) -> bool;

    pub fn AiRenderSetHintArray(
        render_session: *mut AtRenderSession,
        hint: AtString,
        value: *mut AtArray,
    ) -> bool;

    pub fn AiRenderGetHintBool(
        render_session: *const AtRenderSession,
        hint: AtString,
        value: *mut bool,
    ) -> bool;

    pub fn AiRenderGetHintInt(
        render_session: *const AtRenderSession,
        hint: AtString,
        value: *mut i32,
    ) -> bool;

    pub fn AiRenderGetHintFlt(
        render_session: *const AtRenderSession,
        hint: AtString,
        value: *mut f32,
    ) -> bool;

    pub fn AiRenderGetHintStr(
        render_session: *const AtRenderSession,
        hint: AtString,
        value: *mut AtString,
    ) -> bool;

    pub fn AiRenderGetHintArray(
        render_session: *const AtRenderSession,
        hint: AtString,
        value: *mut *const AtArray,
    ) -> bool;

    pub fn AiRenderBegin(
        render_session: *mut AtRenderSession,
        mode: AtRenderMode,
        update_callback: AtRenderUpdateCallback,
        callback_private_data: *mut c_void,
    ) -> AtRenderErrorCode;

    pub fn AiRenderEnd(render_session: *mut AtRenderSession) -> AtRenderErrorCode;

    pub fn AiRenderGetStatus(render_session: *const AtRenderSession) -> AtRenderStatus;

    pub fn AiRenderInterrupt(render_session: *mut AtRenderSession, blocking: AtBlockingCall);

    pub fn AiRenderAbort(render_session: *mut AtRenderSession, blocking: AtBlockingCall);

    pub fn AiRenderResume(render_session: *mut AtRenderSession);

    pub fn AiRenderRestart(render_session: *mut AtRenderSession);

    pub fn AiRenderIsAnyActive() -> bool;

    pub fn AiRender(render_session: *mut AtRenderSession, mode: AtRenderMode) -> AtRenderErrorCode;

    pub fn AiRendering() -> bool;
}
