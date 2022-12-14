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
/// Shader globals data structure
///
/// \\details
/// An AtShaderGlobals structure represents a \\e shading \\e context that holds
/// all the information accesible to the different types of shaders. For
/// example, after a camera ray hits the surface of an object, a shading
/// context of type \\c AI_CONTEXT_SURFACE is created that contains, amongst
/// other things, local geometric properties such as the surface normal, the
/// UV surface parameters, etc.
#[repr(C)]
pub struct AtShaderGlobals {
    /// X raster-space coordinate of this ray tree
    pub x: i32,
    /// Y raster-space coordinate of this ray tree
    pub y: i32,
    /// subpixel X coordinate of this ray in [0,1)
    pub px: f32,
    /// subpixel Y coordinate of this ray in [0,1)
    pub py: f32,
    /// subpixel sample index
    pub si: u16,
    /// transparency index
    pub transp_index: u16,
    /// ray origin (typically the camera position)
    pub Ro: AtVector,
    /// ray direction (normalized)
    pub Rd: AtVector,
    /// ray length (|Ro-P|)
    pub Rl: f32,
    /// thread ID
    pub tid: u16,
    /// ray type
    pub Rt: u8,
    /// recursion level for the ray that created this hit
    pub bounces: u8,
    /// ray diffuse depth level
    pub bounces_diffuse: u8,
    /// ray specular depth level
    pub bounces_specular: u8,
    /// ray reflection depth level
    pub bounces_reflect: u8,
    /// ray transmission depth level
    pub bounces_transmit: u8,
    /// ray volume depth level
    pub bounces_volume: u8,
    /// force hemispherical lighting (use only upper hemisphere)
    pub fhemi: bool,
    /// absolute time, between shutter-open and shutter-close
    pub time: f32,
    /// pointer to the object being shaded
    pub Op: *mut AtNode,
    /// pointer to the procedural object (if exists)
    pub proc_: *mut AtNode,
    /// pointer to the current shader
    pub shader: *mut AtNode,
    /// parent shader globals (last shaded)
    pub psg: *const AtShaderGlobals,
    /// shading point in object-space
    pub Po: AtVector,
    /// shading point in world-space
    pub P: AtVector,
    /// surface derivative wrt screen X-axis
    pub dPdx: AtVector,
    /// surface derivative wrt screen Y-axis
    pub dPdy: AtVector,
    /// shading normal (in object space during displacement)
    pub N: AtVector,
    /// face-forward shading normal
    pub Nf: AtVector,
    /// geometric normal
    pub Ng: AtVector,
    /// face-forward geometric normal
    pub Ngf: AtVector,
    /// smoothed normal (same as N but without bump)
    pub Ns: AtVector,
    /// barycentric coordinate (aka alpha, or u)
    pub bu: f32,
    /// barycentric coordinate (aka beta, or v)
    pub bv: f32,
    /// U surface parameter
    pub u: f32,
    /// V surface parameter
    pub v: f32,
    /// primitive ID (triangle, curve segment, etc)
    pub fi: u32,
    /// local-to-world matrix transform
    pub M: AtMatrix,
    /// world-to-local matrix transform
    pub Minv: AtMatrix,
    /// array of active lights at this shading context
    pub lights: *mut *mut AtNode,
    /// light sample (for light filter shaders)
    pub light_filter: *mut AtLightSample,
    /// number of active lights at this shading context
    pub nlights: u32,
    /// surface derivative wrt U parameter
    pub dPdu: AtVector,
    /// surface derivative wrt V parameter
    pub dPdv: AtVector,
    /// ray direction derivative wrt screen X-axis
    pub dDdx: AtVector,
    /// ray direction derivative wrt screen Y-axis
    pub dDdy: AtVector,
    /// surface normal derivative wrt screen X-axis
    pub dNdx: AtVector,
    /// surface normal derivative wrt screen Y-axis
    pub dNdy: AtVector,
    /// U derivative wrt screen X-axis
    pub dudx: f32,
    /// U derivative wrt screen Y-axis
    pub dudy: f32,
    /// V derivative wrt screen X-axis
    pub dvdx: f32,
    /// V derivative wrt screen Y-axis
    pub dvdy: f32,
    /// if true, don't trace shadow rays
    pub skip_shadow: bool,
    /// type of shading context
    pub sc: u8,
    /// is the trace-set inclusive?
    pub inclusive_traceset: bool,
    /// trace-set to assign to rays made from this SG
    pub traceset: AtString,
    /// shader output
    pub out: AtParamValue,
    /// extra information for internal use
    pub privateinfo: *mut AtShaderGlobalsPrivateInfo,
}
extern "C" {
    pub fn AiShaderGlobals() -> *mut AtShaderGlobals;
}
extern "C" {
    pub fn AiShaderGlobalsDestroy(sg: *mut AtShaderGlobals);
}
