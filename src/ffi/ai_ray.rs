use ::std::os::raw::c_int;

use super::{
    ai_color::AtRGB, ai_shader_sample::AtScrSample, ai_shaderglobals::AtShaderGlobals,
    ai_string::AtString, ai_vector::AtVector,
};

pub const AI_RAY_UNDEFINED: u32 = 0;
pub const AI_RAY_CAMERA: u32 = 1;
pub const AI_RAY_SHADOW: u32 = 2;
pub const AI_RAY_DIFFUSE_TRANSMIT: u32 = 4;
pub const AI_RAY_SPECULAR_TRANSMIT: u32 = 8;
pub const AI_RAY_VOLUME: u32 = 16;
pub const AI_RAY_DIFFUSE_REFLECT: u32 = 32;
pub const AI_RAY_SPECULAR_REFLECT: u32 = 64;
pub const AI_RAY_SUBSURFACE: u32 = 128;
pub const AI_RAY_ALL_DIFFUSE: u32 = 36;
pub const AI_RAY_ALL_SPECULAR: u32 = 72;
pub const AI_RAY_ALL_REFLECT: u32 = 96;
pub const AI_RAY_ALL_TRANSMIT: u32 = 12;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtLightSample {
    _unused: [u8; 0],
}
#[doc = " Ray data structure"]
#[repr(C)]
pub struct AtRay {
    #[doc = "< Type of ray (\\c AI_RAY_CAMERA, etc)"]
    pub type_: u8,
    #[doc = "< Number of bounces so far (0 for camera rays)"]
    pub bounces: u8,
    #[doc = "< Number of diffuse bounces so far"]
    pub bounces_diffuse: u8,
    #[doc = "< Number of specular bounces so far"]
    pub bounces_specular: u8,
    #[doc = "< Number of reflection bounces so far"]
    pub bounces_reflect: u8,
    #[doc = "< Number of transmission bounces so far"]
    pub bounces_transmit: u8,
    #[doc = "< Number of volume bounces so far"]
    pub bounces_volume: u8,
    #[doc = "< Is the trace-set inclusive or exclusive?"]
    pub inclusive_traceset: bool,
    #[doc = "< Trace-set for this ray"]
    pub traceset: AtString,
    #[doc = "< Thread ID"]
    pub tid: u16,
    #[doc = "< Sub-pixel sample index when supersampling"]
    pub sindex: u16,
    #[doc = "< Raster-space X coordinate"]
    pub x: c_int,
    #[doc = "< Raster-space Y coordinate"]
    pub y: c_int,
    #[doc = "< Subpixel X coordinate in [0,1)"]
    pub px: f32,
    #[doc = "< Subpixel Y coordinate in [0,1)"]
    pub py: f32,
    #[doc = "< Ray origin"]
    pub origin: AtVector,
    #[doc = "< Unit ray direction"]
    pub dir: AtVector,
    #[doc = "< Minimum useful distance from the origin"]
    pub mindist: f32,
    #[doc = "< Maximum useful distance from the origin (volatile while ray is traced)"]
    pub maxdist: f32,
    #[doc = "< Parent shader globals (last shaded)"]
    pub psg: *const AtShaderGlobals,
    #[doc = "< Associated light sample, for shadow rays only"]
    pub light_sample: *const AtLightSample,
    #[doc = "< Ray weight, AI_RGB_WHITE for clean camera rays"]
    pub weight: AtRGB,
    #[doc = "< Time at which the ray was created, in [0,1)"]
    pub time: f32,
    #[doc = "< Partial derivative of ray origin wrt image-space X coordinate"]
    pub dOdx: AtVector,
    #[doc = "< Partial derivative of ray origin wrt image-space Y coordinate"]
    pub dOdy: AtVector,
    #[doc = "< Partial derivative of ray direction wrt image-space X coordinate"]
    pub dDdx: AtVector,
    #[doc = "< Partial derivative of ray direction wrt image-space Y coordinate"]
    pub dDdy: AtVector,
}
extern "C" {
    pub fn AiMakeRay(
        type_: u8,
        origin: *const AtVector,
        dir: *const AtVector,
        maxdist: f32,
        sg: *const AtShaderGlobals,
    ) -> AtRay;
}
extern "C" {
    pub fn AiReflectRay(ray: *mut AtRay, normal: *const AtVector, sg: *const AtShaderGlobals);
}
extern "C" {
    pub fn AiRefractRay(
        ray: *mut AtRay,
        normal: *const AtVector,
        n1: f32,
        n2: f32,
        sg: *const AtShaderGlobals,
    ) -> bool;
}
extern "C" {
    pub fn AiTrace(ray: *const AtRay, weight: *const AtRGB, sample: *mut AtScrSample) -> bool;
}
extern "C" {
    pub fn AiTraceBackground(ray: *const AtRay, sample: *mut AtScrSample);
}
extern "C" {
    pub fn AiTraceProbe(ray: *const AtRay, sgout: *mut AtShaderGlobals) -> bool;
}
