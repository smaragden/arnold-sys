use ::std::{
    option::Option,
    os::raw::{c_char, c_int, c_uint, c_void},
};

use super::{
    ai_array::AtArray,
    ai_render::{AtRenderErrorCode, AtRenderSession, AtRenderStatus},
    ai_string::AtString,
};

/// Render using the CPU
pub const AtDeviceType_AI_DEVICE_TYPE_CPU: AtDeviceType = 0;
/// Render using the GPU
pub const AtDeviceType_AI_DEVICE_TYPE_GPU: AtDeviceType = 1;
/// Device types
pub type AtDeviceType = c_uint;
/// Total memory on device
pub const AtDeviceMemory_AI_DEVICE_MEMORY_TOTAL: AtDeviceMemory = 0;
/// Total free memory available to device
pub const AtDeviceMemory_AI_DEVICE_MEMORY_FREE: AtDeviceMemory = 1;
/// Total used memory
pub const AtDeviceMemory_AI_DEVICE_MEMORY_USED: AtDeviceMemory = 2;
/// Queriable memory attributes
pub type AtDeviceMemory = c_uint;
extern "C" {
    /// Returns if a given device is supported on the current system
    pub fn AiDeviceTypeIsSupported(device_type: AtDeviceType, reason: *mut AtString) -> bool;
}
extern "C" {
    /// Select render device
    pub fn AiDeviceSelect(
        render_session: *mut AtRenderSession,
        device_type: AtDeviceType,
        device_ids: *const AtArray,
    ) -> c_int;
}
extern "C" {
    /// Automatically select render device based on options
    pub fn AiDeviceAutoSelect(render_session: *mut AtRenderSession) -> c_int;
}
extern "C" {
    /// Returns the currently selected render device type
    pub fn AiDeviceGetSelectedType(render_session: *const AtRenderSession) -> AtDeviceType;
}
extern "C" {
    /// Returns the currently selected devices ids of a device type
    pub fn AiDeviceGetSelectedIds(
        render_session: *const AtRenderSession,
        device_type: AtDeviceType,
    ) -> *const AtArray;
}
extern "C" {
    /// Returns the number of available devices of a given type
    pub fn AiDeviceGetCount(device_type: AtDeviceType) -> c_uint;
}
extern "C" {
    /// Returns the ids of available devices of a given type
    pub fn AiDeviceGetIds(device_type: AtDeviceType) -> *const AtArray;
}
extern "C" {
    /// Returns the name of a device
    pub fn AiDeviceGetName(device_type: AtDeviceType, device_id: c_uint) -> AtString;
}
extern "C" {
    /// Returns memory information of a device
    pub fn AiDeviceGetMemoryMB(
        device_type: AtDeviceType,
        device_id: c_uint,
        memory: AtDeviceMemory,
    ) -> c_uint;
}
/// GPU cache population report callback.
/// 
/// This callback provides:
/// 
/// | Syntax        | Description                                        |
/// | ------------- | -------------------------------------------------- |
/// | user_ptr      | the user data pointer passed to AiGPUCachePopulate |
/// | status        | the user data pointer passed to AiGPUCachePopulate |
/// | fraction_done | the progress as a fraction in `[0.0, 1.0]`         |
/// | msg           | a report string                                    |
pub type AtGPUCachePopulateCallback = Option<
    unsafe extern "C" fn(
        user_ptr: *mut c_void,
        status: AtRenderStatus,
        fraction_done: f32,
        msg: *const c_char,
    ),
>;
pub const AtGPUCachePopulateMode_AI_GPU_CACHE_POPULATE_BLOCKING: AtGPUCachePopulateMode = 0;
pub const AtGPUCachePopulateMode_AI_GPU_CACHE_POPULATE_NON_BLOCKING: AtGPUCachePopulateMode = 1;
pub type AtGPUCachePopulateMode = c_uint;
extern "C" {
    /// Pre-populates the GPU program cache
    pub fn AiGPUCachePopulate(
        mode: AtGPUCachePopulateMode,
        num_proc: c_uint,
        report_callback: AtGPUCachePopulateCallback,
        user_ptr: *mut c_void,
    ) -> AtRenderErrorCode;
}
extern "C" {
    /// Poll for the current status of the GPU cache population
    pub fn AiGPUCachePopulateStatus(fraction_done: *mut f32) -> AtRenderStatus;
}
extern "C" {
    /// Estimate for how many more seconds remain for the GPU cache population
    pub fn AiGPUCachePopulateRemainingSeconds() -> f32;
}
extern "C" {
    /// Request immediate termination of the GPU cache population
    pub fn AiGPUCachePopulateTerminate();
}
extern "C" {
    /// Set the directory where the OptiX cache will be stored
    pub fn AiGPUCacheSetDirectory(dir_path: *const c_char);
}
extern "C" {
    /// Get the directory specified via AiGPUCacheSetDirectory (or if not specified, the default)
    pub fn AiGPUCacheGetDirectory() -> AtString;
}
