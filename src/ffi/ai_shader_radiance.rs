use ::std::os::raw::c_int;

use super::{ai_color::AtRGB, ai_nodes::AtNode, ai_vector::AtVector};

extern "C" {
    #[doc = " \\defgroup ai_shader_radiance Irradiance / Radiance API"]
    #[doc = ""]
    #[doc = " Radiance and irradiance lookup functions."]
    #[doc = ""]
    #[doc = " \\{"]
    pub fn AiIrradiance(p: *const AtVector, n: *const AtVector, tid: c_int, pid: u32) -> AtRGB;
}
extern "C" {
    pub fn AiRadiance(
        p: *const AtVector,
        dir: *const AtVector,
        n: *const AtVector,
        obj: *mut AtNode,
        face: u32,
        u: f32,
        v: f32,
        shader: *mut AtNode,
        tid: c_int,
        pid: u32,
    ) -> AtRGB;
}
