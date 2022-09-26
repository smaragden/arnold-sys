/* automatically generated by rust-bindgen 0.60.1 */

use super::{
    ai_matrix::AtMatrix, ai_nodes::AtNode, ai_params::AtParamValue, ai_string::AtString,
    ai_vector::AtVector,
};

pub const AI_CONTEXT_SURFACE: u32 = 0;
pub const AI_CONTEXT_VOLUME: u32 = 1;
pub const AI_CONTEXT_BACKGROUND: u32 = 2;
pub const AI_CONTEXT_DISPLACEMENT: u32 = 3;
pub const AI_CONTEXT_IMPORTANCE: u32 = 5;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtBucket {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtLightSample {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtShaderGlobalsPrivateInfo {
    _unused: [u8; 0],
}
#[doc = " Shader globals data structure"]
#[doc = ""]
#[doc = " \\details"]
#[doc = " An AtShaderGlobals structure represents a \\e shading \\e context that holds"]
#[doc = " all the information accesible to the different types of shaders. For"]
#[doc = " example, after a camera ray hits the surface of an object, a shading"]
#[doc = " context of type \\c AI_CONTEXT_SURFACE is created that contains, amongst"]
#[doc = " other things, local geometric properties such as the surface normal, the"]
#[doc = " UV surface parameters, etc."]
#[repr(C)]
pub struct AtShaderGlobals {
    #[doc = "< X raster-space coordinate of this ray tree"]
    pub x: i32,
    #[doc = "< Y raster-space coordinate of this ray tree"]
    pub y: i32,
    #[doc = "< subpixel X coordinate of this ray in [0,1)"]
    pub px: f32,
    #[doc = "< subpixel Y coordinate of this ray in [0,1)"]
    pub py: f32,
    #[doc = "< subpixel sample index"]
    pub si: u16,
    #[doc = "< transparency index"]
    pub transp_index: u16,
    #[doc = "< ray origin (typically the camera position)"]
    pub Ro: AtVector,
    #[doc = "< ray direction (normalized)"]
    pub Rd: AtVector,
    #[doc = "< ray length (|Ro-P|)"]
    pub Rl: f32,
    #[doc = "< thread ID"]
    pub tid: u16,
    #[doc = "< ray type"]
    pub Rt: u8,
    #[doc = "< recursion level for the ray that created this hit"]
    pub bounces: u8,
    #[doc = "< ray diffuse depth level"]
    pub bounces_diffuse: u8,
    #[doc = "< ray specular depth level"]
    pub bounces_specular: u8,
    #[doc = "< ray reflection depth level"]
    pub bounces_reflect: u8,
    #[doc = "< ray transmission depth level"]
    pub bounces_transmit: u8,
    #[doc = "< ray volume depth level"]
    pub bounces_volume: u8,
    #[doc = "< force hemispherical lighting (use only upper hemisphere)"]
    pub fhemi: bool,
    #[doc = "< absolute time, between shutter-open and shutter-close"]
    pub time: f32,
    #[doc = "< pointer to the object being shaded"]
    pub Op: *mut AtNode,
    #[doc = "< pointer to the procedural object (if exists)"]
    pub proc_: *mut AtNode,
    #[doc = "< pointer to the current shader"]
    pub shader: *mut AtNode,
    #[doc = "< parent shader globals (last shaded)"]
    pub psg: *const AtShaderGlobals,
    #[doc = "< shading point in object-space"]
    pub Po: AtVector,
    #[doc = "< shading point in world-space"]
    pub P: AtVector,
    #[doc = "< surface derivative wrt screen X-axis"]
    pub dPdx: AtVector,
    #[doc = "< surface derivative wrt screen Y-axis"]
    pub dPdy: AtVector,
    #[doc = "< shading normal (in object space during displacement)"]
    pub N: AtVector,
    #[doc = "< face-forward shading normal"]
    pub Nf: AtVector,
    #[doc = "< geometric normal"]
    pub Ng: AtVector,
    #[doc = "< face-forward geometric normal"]
    pub Ngf: AtVector,
    #[doc = "< smoothed normal (same as N but without bump)"]
    pub Ns: AtVector,
    #[doc = "< barycentric coordinate (aka alpha, or u)"]
    pub bu: f32,
    #[doc = "< barycentric coordinate (aka beta, or v)"]
    pub bv: f32,
    #[doc = "< U surface parameter"]
    pub u: f32,
    #[doc = "< V surface parameter"]
    pub v: f32,
    #[doc = "< primitive ID (triangle, curve segment, etc)"]
    pub fi: u32,
    #[doc = "< local-to-world matrix transform"]
    pub M: AtMatrix,
    #[doc = "< world-to-local matrix transform"]
    pub Minv: AtMatrix,
    #[doc = "< array of active lights at this shading context"]
    pub lights: *mut *mut AtNode,
    #[doc = "< light sample (for light filter shaders)"]
    pub light_filter: *mut AtLightSample,
    #[doc = "< number of active lights at this shading context"]
    pub nlights: u32,
    #[doc = "< surface derivative wrt U parameter"]
    pub dPdu: AtVector,
    #[doc = "< surface derivative wrt V parameter"]
    pub dPdv: AtVector,
    #[doc = "< ray direction derivative wrt screen X-axis"]
    pub dDdx: AtVector,
    #[doc = "< ray direction derivative wrt screen Y-axis"]
    pub dDdy: AtVector,
    #[doc = "< surface normal derivative wrt screen X-axis"]
    pub dNdx: AtVector,
    #[doc = "< surface normal derivative wrt screen Y-axis"]
    pub dNdy: AtVector,
    #[doc = "< U derivative wrt screen X-axis"]
    pub dudx: f32,
    #[doc = "< U derivative wrt screen Y-axis"]
    pub dudy: f32,
    #[doc = "< V derivative wrt screen X-axis"]
    pub dvdx: f32,
    #[doc = "< V derivative wrt screen Y-axis"]
    pub dvdy: f32,
    #[doc = "< if true, don't trace shadow rays"]
    pub skip_shadow: bool,
    #[doc = "< type of shading context"]
    pub sc: u8,
    #[doc = "< is the trace-set inclusive?"]
    pub inclusive_traceset: bool,
    #[doc = "< trace-set to assign to rays made from this SG"]
    pub traceset: AtString,
    #[doc = "< shader output"]
    pub out: AtParamValue,
    #[doc = "< extra information for internal use"]
    pub privateinfo: *mut AtShaderGlobalsPrivateInfo,
}
extern "C" {
    pub fn AiShaderGlobals() -> *mut AtShaderGlobals;
}
extern "C" {
    pub fn AiShaderGlobalsDestroy(sg: *mut AtShaderGlobals);
}
