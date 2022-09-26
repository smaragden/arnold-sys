use super::{
    ai_closure::{AtClosure, AtClosureList},
    ai_color::AtRGB,
    ai_shaderglobals::AtShaderGlobals,
    ai_vector::AtVector,
};

extern "C" {
    #[doc = " \\defgroup ai_shader_closure Shader Closure API"]
    #[doc = ""]
    #[doc = "  \\ref AtClosure creation. Shaders can return these instead of final colors."]
    #[doc = ""]
    #[doc = "  \\details"]
    #[doc = "  Closures describe the way surfaces and volumes scatter light,"]
    #[doc = "  leaving the lights loops and integration to Arnold."]
    #[doc = ""]
    #[doc = "  For creating BSDF closures, an \\c AtBSDF can be wrapped in a closure:"]
    #[doc = "  \\code"]
    #[doc = "  AtBSDF *bsdf = AiOrenNayarBSDF(sg, sg->Nf);"]
    #[doc = "  sg->out.CLOSURE() = AtClosure(diffuse_color, bsdf);"]
    #[doc = "  \\endcode"]
    #[doc = ""]
    #[doc = " \\{"]
    pub fn AiClosureEmpiricalBSSRDF(
        sg: *const AtShaderGlobals,
        weight: *const AtRGB,
        mfp: *const AtVector,
        albedo: *const AtRGB,
    ) -> AtClosure;
}
extern "C" {
    pub fn AiClosureRandomWalkBSSRDF(
        sg: *const AtShaderGlobals,
        weight: *const AtRGB,
        mfp: *const AtVector,
        albedo: *const AtRGB,
        g: f32,
    ) -> AtClosure;
}
extern "C" {
    pub fn AiClosureRandomWalkV2BSSRDF(
        sg: *const AtShaderGlobals,
        weight: *const AtRGB,
        mfp: *const AtVector,
        albedo: *const AtRGB,
        g: f32,
    ) -> AtClosure;
}
extern "C" {
    pub fn AiClosureBSSRDFSetDirectIndirect(
        closure: AtClosure,
        weight_direct: f32,
        weight_indirect: f32,
    );
}
extern "C" {
    pub fn AiClosureEmission(sg: *const AtShaderGlobals, weight: *const AtRGB) -> AtClosure;
}
extern "C" {
    pub fn AiClosureTransparent(
        sg: *const AtShaderGlobals,
        weight: *const AtRGB,
        interior: AtClosureList,
        importance: i32,
    ) -> AtClosure;
}
extern "C" {
    pub fn AiClosureMatte(sg: *const AtShaderGlobals, weight: *const AtRGB) -> AtClosure;
}
extern "C" {
    pub fn AiClosureBackground(sg: *const AtShaderGlobals, weight: *const AtRGB) -> AtClosure;
}
extern "C" {
    pub fn AiClosureVolumeAbsorption(sg: *const AtShaderGlobals, weight: *const AtRGB)
        -> AtClosure;
}
extern "C" {
    pub fn AiClosureVolumeEmission(sg: *const AtShaderGlobals, weight: *const AtRGB) -> AtClosure;
}
extern "C" {
    pub fn AiClosureVolumeHenyeyGreenstein(
        sg: *const AtShaderGlobals,
        absorption: *const AtRGB,
        scattering: *const AtRGB,
        emission: *const AtRGB,
        g: f32,
    ) -> AtClosure;
}
extern "C" {
    pub fn AiClosureVolumeMatte(sg: *const AtShaderGlobals, weight: *const AtRGB) -> AtClosure;
}
extern "C" {
    pub fn AiClosureVolumeAtmosphere(
        sg: *const AtShaderGlobals,
        emission: *const AtRGB,
        transparent: *const AtRGB,
        matte: *const AtRGB,
    ) -> AtClosure;
}
