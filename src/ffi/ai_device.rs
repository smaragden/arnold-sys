use super::{
    ai_array::AtArray,
    ai_render::{AtRenderErrorCode, AtRenderSession, AtRenderStatus},
    ai_string::AtString,
};

#[doc = "< Render using the CPU"]
pub const AtDeviceType_AI_DEVICE_TYPE_CPU: AtDeviceType = 0;
#[doc = "< Render using the GPU"]
pub const AtDeviceType_AI_DEVICE_TYPE_GPU: AtDeviceType = 1;
#[doc = " Device types"]
pub type AtDeviceType = ::std::os::raw::c_uint;
#[doc = "< Total memory on device"]
pub const AtDeviceMemory_AI_DEVICE_MEMORY_TOTAL: AtDeviceMemory = 0;
#[doc = "< Total free memory available to device"]
pub const AtDeviceMemory_AI_DEVICE_MEMORY_FREE: AtDeviceMemory = 1;
#[doc = "< Total used memory"]
pub const AtDeviceMemory_AI_DEVICE_MEMORY_USED: AtDeviceMemory = 2;
#[doc = " Queriable memory attributes"]
pub type AtDeviceMemory = ::std::os::raw::c_uint;
extern "C" {
    #[doc = " Returns if a given device is supported on the current system"]
    pub fn AiDeviceTypeIsSupported(device_type: AtDeviceType, reason: *mut AtString) -> bool;
}
extern "C" {
    #[doc = " Select render device"]
    pub fn AiDeviceSelect(
        render_session: *mut AtRenderSession,
        device_type: AtDeviceType,
        device_ids: *const AtArray,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Automatically select render device based on options"]
    pub fn AiDeviceAutoSelect(render_session: *mut AtRenderSession) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = " Returns the currently selected render device type"]
    pub fn AiDeviceGetSelectedType(render_session: *const AtRenderSession) -> AtDeviceType;
}
extern "C" {
    #[doc = " Returns the currently selected devices ids of a device type"]
    pub fn AiDeviceGetSelectedIds(
        render_session: *const AtRenderSession,
        device_type: AtDeviceType,
    ) -> *const AtArray;
}
extern "C" {
    #[doc = " Returns the number of available devices of a given type"]
    pub fn AiDeviceGetCount(device_type: AtDeviceType) -> ::std::os::raw::c_uint;
}
extern "C" {
    #[doc = " Returns the ids of available devices of a given type"]
    pub fn AiDeviceGetIds(device_type: AtDeviceType) -> *const AtArray;
}
extern "C" {
    #[doc = " Returns the name of a device"]
    pub fn AiDeviceGetName(
        device_type: AtDeviceType,
        device_id: ::std::os::raw::c_uint,
    ) -> AtString;
}
extern "C" {
    #[doc = " Returns memory information of a device"]
    pub fn AiDeviceGetMemoryMB(
        device_type: AtDeviceType,
        device_id: ::std::os::raw::c_uint,
        memory: AtDeviceMemory,
    ) -> ::std::os::raw::c_uint;
}
#[doc = " GPU cache population report callback."]
#[doc = "  This callback provides:"]
#[doc = "     - user_ptr:      the user data pointer passed to AiGPUCachePopulate"]
#[doc = "     - status:        status code as returned from AiGPUCachePopulateStatus (AI_RENDER_STATUS_FINISHED will occur only once, on completion)"]
#[doc = "     - fraction_done: the progress as a fraction in [0.0, 1.0]"]
#[doc = "     - msg:           a report string"]
pub type AtGPUCachePopulateCallback = ::std::option::Option<
    unsafe extern "C" fn(
        user_ptr: *mut ::std::os::raw::c_void,
        status: AtRenderStatus,
        fraction_done: f32,
        msg: *const ::std::os::raw::c_char,
    ),
>;
pub const AtGPUCachePopulateMode_AI_GPU_CACHE_POPULATE_BLOCKING: AtGPUCachePopulateMode = 0;
pub const AtGPUCachePopulateMode_AI_GPU_CACHE_POPULATE_NON_BLOCKING: AtGPUCachePopulateMode = 1;
pub type AtGPUCachePopulateMode = ::std::os::raw::c_uint;
extern "C" {
    #[doc = " Pre-populates the GPU program cache"]
    pub fn AiGPUCachePopulate(
        mode: AtGPUCachePopulateMode,
        num_proc: ::std::os::raw::c_uint,
        report_callback: AtGPUCachePopulateCallback,
        user_ptr: *mut ::std::os::raw::c_void,
    ) -> AtRenderErrorCode;
}
extern "C" {
    #[doc = " Poll for the current status of the GPU cache population"]
    pub fn AiGPUCachePopulateStatus(fraction_done: *mut f32) -> AtRenderStatus;
}
extern "C" {
    #[doc = " Estimate for how many more seconds remain for the GPU cache population"]
    pub fn AiGPUCachePopulateRemainingSeconds() -> f32;
}
extern "C" {
    #[doc = " Request immediate termination of the GPU cache population"]
    pub fn AiGPUCachePopulateTerminate();
}
extern "C" {
    #[doc = " Set the directory where the OptiX cache will be stored"]
    pub fn AiGPUCacheSetDirectory(dir_path: *const ::std::os::raw::c_char);
}
extern "C" {
    #[doc = " Get the directory specified via AiGPUCacheSetDirectory (or if not specified, the default)"]
    pub fn AiGPUCacheGetDirectory() -> AtString;
}
