use super::{
    ai_closure::AtBSDF, ai_color::AtRGB, ai_nodes::AtNode, ai_shaderglobals::AtShaderGlobals,
    ai_string::AtString, ai_vector::AtVector,
};

#[doc = " Light sample data structure"]
#[repr(C)]
pub struct AtLightSample {
    #[doc = "< pointer to light node"]
    pub Lp: *const AtNode,
    #[doc = "< distance from P to light source"]
    pub Ldist: f32,
    #[doc = "< normalized direction vector from P to the light sample"]
    pub Ld: AtVector,
    #[doc = "< incident radiance, same as \\c Liu * (1 - \\c Lo)"]
    pub Li: AtRGB,
    #[doc = "< unoccluded incident radiance"]
    pub Liu: AtRGB,
    #[doc = "< shadow occlusion factor, (1,1,1) means fully occluded"]
    pub Lo: AtRGB,
    #[doc = "< sample probability density function"]
    pub pdf: f32,
    #[doc = "< bitmask of ray types that will return light hits from AiLightsTrace */"]
    pub trace_ray_types: u8,
}
extern "C" {
    #[doc = " \\name Light Loop Methods"]
    #[doc = " \\details"]
    #[doc = " This is the API for looping over light source samples."]
    #[doc = " \\{"]
    pub fn AiLightsPrepare(sg: *mut AtShaderGlobals);
}
extern "C" {
    pub fn AiLightsGetSample(sg: *mut AtShaderGlobals, sample: *mut AtLightSample) -> bool;
}
extern "C" {
    pub fn AiLightsTrace(
        sg: *mut AtShaderGlobals,
        dir: *const AtVector,
        ray_type: u8,
        hits: *mut *mut AtLightSample,
    ) -> u32;
}
extern "C" {
    pub fn AiLightsTraceRayTypes(sg: *mut AtShaderGlobals) -> u8;
}
extern "C" {
    pub fn AiLightsResetCache(sg: *mut AtShaderGlobals);
}
extern "C" {
    pub fn AiLightsIntegrateShadowMatte(sg: *mut AtShaderGlobals, bsdf: *mut AtBSDF) -> AtRGB;
}
extern "C" {
    #[doc = " \\name Light Getter Methods"]
    #[doc = " \\details"]
    #[doc = " These getters are to avoid slow AiNodeGet*() calls to access light parameters."]
    #[doc = " Only the most common attributes are exposed for now."]
    #[doc = " \\{"]
    pub fn AiLightGetColor(node: *const AtNode) -> AtRGB;
}
extern "C" {
    pub fn AiLightGetIntensity(node: *const AtNode) -> f32;
}
extern "C" {
    pub fn AiLightGetDiffuse(node: *const AtNode) -> f32;
}
extern "C" {
    pub fn AiLightGetSpecular(node: *const AtNode) -> f32;
}
extern "C" {
    pub fn AiLightGetTransmission(node: *const AtNode) -> f32;
}
extern "C" {
    pub fn AiLightGetSSS(node: *const AtNode) -> f32;
}
extern "C" {
    pub fn AiLightGetVolume(node: *const AtNode) -> f32;
}
extern "C" {
    pub fn AiLightGetInfluence(
        sg: *const AtShaderGlobals,
        node: *const AtNode,
        ray_type: u8,
    ) -> f32;
}
extern "C" {
    #[doc = " \\name Light IES Format Parser"]
    #[doc = " \\details"]
    #[doc = " Not a shader specific call, this is called by plugins or host DCCs to parse IES files"]
    #[doc = " \\{"]
    pub fn AiLightIESLoad(
        filename: AtString,
        width: ::std::os::raw::c_uint,
        height: ::std::os::raw::c_uint,
        max_intensity: *mut f32,
        data: *mut f32,
    ) -> bool;
}
