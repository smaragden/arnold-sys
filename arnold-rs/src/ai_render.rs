#![allow(non_camel_case_types, non_snake_case)]

use std::os::raw::c_uint;

use crate::ai_universe::AtUniverse;

#[derive(Debug, Copy, Clone)]
pub struct AtRenderSession {
    pub raw: *mut arnold_sys::ai_render::AtRenderSession
}

impl AtRenderSession {
    pub fn raw(&self) -> *mut arnold_sys::ai_render::AtRenderSession {
        unsafe { &mut *self.raw }
    }
}

pub fn AiRenderSession(universe: Option<&AtUniverse>, mode: AtSessionMode) -> AtRenderSession {
    let universe = match universe {
        Some(universe) => universe.raw(),
        None => std::ptr::null_mut(),
    };
    AtRenderSession{
        raw: unsafe{arnold_sys::ai_render::AiRenderSession(universe, mode as u32)},
    }
}

#[allow(missing_docs)]
#[derive(Copy, Clone, Debug)]
pub enum AtSessionMode {
    AI_SESSION_BATCH = arnold_sys::ai_render::AtSessionMode_AI_SESSION_BATCH as isize,
    AI_SESSION_INTERACTIVE = arnold_sys::ai_render::AtSessionMode_AI_SESSION_INTERACTIVE as isize,
}

#[allow(missing_docs)]
#[derive(Copy, Clone, Debug)]
pub enum AtRenderMode {
    /// Render from a camera
    AI_RENDER_MODE_CAMERA = arnold_sys::ai_render::AtRenderMode_AI_RENDER_MODE_CAMERA as isize,
    /// Process arbitrary ray-tracing requests, acting as a \"ray server\"
    AI_RENDER_MODE_FREE = arnold_sys::ai_render::AtRenderMode_AI_RENDER_MODE_FREE as isize,
}

#[allow(missing_docs)]
#[derive(Copy, Clone, Debug)]
pub enum AtRenderErrorCode {
    AI_SUCCESS = arnold_sys::ai_render::AtRenderErrorCode_AI_SUCCESS as isize,
    /// render aborted
    AI_ABORT = arnold_sys::ai_render::AtRenderErrorCode_AI_ABORT as isize,
    /// camera not defined
    AI_ERROR_NO_CAMERA = arnold_sys::ai_render::AtRenderErrorCode_AI_ERROR_NO_CAMERA as isize,
    /// bad camera data
    AI_ERROR_BAD_CAMERA = arnold_sys::ai_render::AtRenderErrorCode_AI_ERROR_BAD_CAMERA as isize,
    /// usage not validated
    AI_ERROR_VALIDATION = arnold_sys::ai_render::AtRenderErrorCode_AI_ERROR_VALIDATION as isize,
    /// invalid render region
    AI_ERROR_RENDER_REGION = arnold_sys::ai_render::AtRenderErrorCode_AI_ERROR_RENDER_REGION as isize,
    /// render interrupted by user
    AI_INTERRUPT = arnold_sys::ai_render::AtRenderErrorCode_AI_INTERRUPT as isize,
    /// no rendering outputs
    AI_ERROR_NO_OUTPUTS = arnold_sys::ai_render::AtRenderErrorCode_AI_ERROR_NO_OUTPUTS as isize,
    /// Cannot create GPU context
    AI_ERROR_UNAVAILABLE_DEVICE = arnold_sys::ai_render::AtRenderErrorCode_AI_ERROR_UNAVAILABLE_DEVICE as isize,
    /// generic error
    AI_ERROR = arnold_sys::ai_render::AtRenderErrorCode_AI_ERROR as isize,
}

impl From<u32> for AtRenderErrorCode{
    fn from(code: u32) -> Self {
        match code {
            0 => AtRenderErrorCode::AI_SUCCESS,
            1 => AtRenderErrorCode::AI_ABORT,
            2 => AtRenderErrorCode::AI_ERROR_NO_CAMERA,
            3 => AtRenderErrorCode::AI_ERROR_BAD_CAMERA,
            4 => AtRenderErrorCode::AI_ERROR_VALIDATION,
            5 => AtRenderErrorCode::AI_ERROR_RENDER_REGION,
            6 => AtRenderErrorCode::AI_INTERRUPT,
            7 => AtRenderErrorCode::AI_ERROR_NO_OUTPUTS,
            8 => AtRenderErrorCode::AI_ERROR_UNAVAILABLE_DEVICE,
            9 => AtRenderErrorCode::AI_ERROR,
            _ => unreachable!("Invalid Error Code")
        }
    }
}
pub fn AiBegin(mode: AtSessionMode) {
    unsafe {arnold_sys::ai_render::AiBegin(mode as c_uint)};
}

pub fn AiEnd() {
    unsafe {arnold_sys::ai_render::AiEnd()};
}

pub fn AiRender(render_session: &AtRenderSession, mode: AtRenderMode) -> AtRenderErrorCode{
    unsafe{arnold_sys::ai_render::AiRender(render_session.raw(), mode as u32)}.into()
}
