use ::std::os::raw::c_uint;

use super::{ai_color::AtRGB, ai_string::AtString};

/// \\defgroup ai_closure AtClosure API
///
/// `AtClosure` type and related utilities.
///
/// \\{
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtBSDF {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtClosureBase {
    _unused: [u8; 0],
}
pub const AtClosureType_AI_CLOSURE_NONE: AtClosureType = 0;
pub const AtClosureType_AI_CLOSURE_BSDF: AtClosureType = 1;
pub const AtClosureType_AI_CLOSURE_BSSRDF: AtClosureType = 2;
pub const AtClosureType_AI_CLOSURE_EMISSION: AtClosureType = 3;
pub const AtClosureType_AI_CLOSURE_DEBUG: AtClosureType = 4;
pub const AtClosureType_AI_CLOSURE_TRANSPARENT: AtClosureType = 5;
pub const AtClosureType_AI_CLOSURE_MATTE: AtClosureType = 6;
pub const AtClosureType_AI_CLOSURE_BACKGROUND: AtClosureType = 7;
pub const AtClosureType_AI_CLOSURE_VOLUME_ABSORPTION: AtClosureType = 8;
pub const AtClosureType_AI_CLOSURE_VOLUME_SCATTERING: AtClosureType = 9;
pub const AtClosureType_AI_CLOSURE_VOLUME_EMISSION: AtClosureType = 10;
pub const AtClosureType_AI_CLOSURE_VOLUME_MATTE: AtClosureType = 11;
pub const AtClosureType_AI_CLOSURE_VOLUME_ATMOSPHERE: AtClosureType = 12;
pub const AtClosureType_AI_CLOSURE_DIELECTRIC: AtClosureType = 13;
/// Shader closure types
pub type AtClosureType = c_uint;
extern "C" {
    pub fn AiClosureType(closure: *mut AtClosureBase) -> AtClosureType;

    pub fn AiClosureWeight(closure: *mut AtClosureBase) -> AtRGB;

    pub fn AiClosureSetWeight(closure: *mut AtClosureBase, weight: *const AtRGB);

    pub fn AiClosureLabel(closure: *mut AtClosureBase) -> AtString;

    pub fn AiClosureSetLabel(closure: *mut AtClosureBase, label: AtString);

    pub fn AiClosureNext(closure: *mut AtClosureBase) -> *mut AtClosureBase;

    pub fn AiClosureSetExtraDepth(closure: *mut AtClosureBase, extra_depth: u8);

    pub fn AiClosureSetExtraSamples(closure: *mut AtClosureBase, extra_samples: u8);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtClosure {
    pub data: *mut AtClosureBase,
}
extern "C" {
    pub fn AiClosureListAdd(
        listA: *mut AtClosureBase,
        listB: *mut AtClosureBase,
    ) -> *mut AtClosureBase;

    pub fn AiClosureListWeight(
        list: *mut AtClosureBase,
        weight: *const AtRGB,
    ) -> *mut AtClosureBase;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtClosureList {
    pub data: *mut AtClosureBase,
}
