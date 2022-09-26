use super::{ai_color::AtRGB, ai_string::AtString};

#[doc = " \\defgroup ai_closure AtClosure API"]
#[doc = ""]
#[doc = " \\ref AtClosure type and related utilities."]
#[doc = ""]
#[doc = " \\{"]
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
#[doc = " Shader closure types"]
pub type AtClosureType = ::std::os::raw::c_uint;
extern "C" {
    pub fn AiClosureType(closure: *mut AtClosureBase) -> AtClosureType;
}
extern "C" {
    pub fn AiClosureWeight(closure: *mut AtClosureBase) -> AtRGB;
}
extern "C" {
    pub fn AiClosureSetWeight(closure: *mut AtClosureBase, weight: *const AtRGB);
}
extern "C" {
    pub fn AiClosureLabel(closure: *mut AtClosureBase) -> AtString;
}
extern "C" {
    pub fn AiClosureSetLabel(closure: *mut AtClosureBase, label: AtString);
}
extern "C" {
    pub fn AiClosureNext(closure: *mut AtClosureBase) -> *mut AtClosureBase;
}
extern "C" {
    pub fn AiClosureSetExtraDepth(closure: *mut AtClosureBase, extra_depth: u8);
}
extern "C" {
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
}
extern "C" {
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
