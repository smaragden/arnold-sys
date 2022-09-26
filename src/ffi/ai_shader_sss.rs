use super::{ai_color::AtRGB, ai_shaderglobals::AtShaderGlobals};

extern "C" {
    #[doc = " \\defgroup ai_shader_sss Sub-Surface Scattering API"]
    #[doc = ""]
    #[doc = " SSS utility functions."]
    #[doc = ""]
    #[doc = " \\{"]
    pub fn AiBSSRDFEmpirical(
        sg: *const AtShaderGlobals,
        direct: *mut AtRGB,
        indirect: *mut AtRGB,
        mfp: *const f32,
        albedo: *const f32,
        weight: *const AtRGB,
        num: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn AiShaderGlobalsFromSSS(sg: *const AtShaderGlobals) -> bool;
}
