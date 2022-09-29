use ::std::os::raw::c_uint;

pub const AI_ENABLE_DEPRECATION_WARNINGS: u32 = 1;
/// synchronous, blocking call; returns when the task is done
pub const AtBlockingCall_AI_BLOCKING: AtBlockingCall = 0;
/// asynchronous, non-blocking call; returns ASAP, task completes in the background
pub const AtBlockingCall_AI_NON_BLOCKING: AtBlockingCall = 1;
/// Whether a function call is blocking (synchronous) or not (asynchronous)
pub type AtBlockingCall = c_uint;
