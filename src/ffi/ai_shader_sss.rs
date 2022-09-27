use ::std::os::raw::c_uint;

use super::{ai_color::AtRGB, ai_shaderglobals::AtShaderGlobals};

extern "C" {
    /// \\defgroup ai_shader_sss Sub-Surface Scattering API
    ///
    /// SSS utility functions.
    ///
    /// \\{
    pub fn AiBSSRDFEmpirical(
        sg: *const AtShaderGlobals,
        direct: *mut AtRGB,
        indirect: *mut AtRGB,
        mfp: *const f32,
        albedo: *const f32,
        weight: *const AtRGB,
        num: c_uint,
    );
}
extern "C" {
    pub fn AiShaderGlobalsFromSSS(sg: *const AtShaderGlobals) -> bool;
}
