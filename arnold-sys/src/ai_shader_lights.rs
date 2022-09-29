use ::std::os::raw::c_uint;

use super::{
    ai_closure::AtBSDF, ai_color::AtRGB, ai_nodes::AtNode, ai_shaderglobals::AtShaderGlobals,
    ai_string::AtString, ai_vector::AtVector,
};

/// Light sample data structure
#[repr(C)]
pub struct AtLightSample {
    /// pointer to light node
    pub Lp: *const AtNode,
    /// distance from P to light source
    pub Ldist: f32,
    /// normalized direction vector from P to the light sample
    pub Ld: AtVector,
    /// incident radiance, same as \\c Liu * (1 - \\c Lo)
    pub Li: AtRGB,
    /// unoccluded incident radiance
    pub Liu: AtRGB,
    /// shadow occlusion factor, (1,1,1) means fully occluded
    pub Lo: AtRGB,
    /// sample probability density function
    pub pdf: f32,
    /// bitmask of ray types that will return light hits from AiLightsTrace */
    pub trace_ray_types: u8,
}
extern "C" {
    /// \\name Light Loop Methods
    /// \\details
    /// This is the API for looping over light source samples.
    /// \\{
    pub fn AiLightsPrepare(sg: *mut AtShaderGlobals);

    pub fn AiLightsGetSample(sg: *mut AtShaderGlobals, sample: *mut AtLightSample) -> bool;

    pub fn AiLightsTrace(
        sg: *mut AtShaderGlobals,
        dir: *const AtVector,
        ray_type: u8,
        hits: *mut *mut AtLightSample,
    ) -> u32;

    pub fn AiLightsTraceRayTypes(sg: *mut AtShaderGlobals) -> u8;

    pub fn AiLightsResetCache(sg: *mut AtShaderGlobals);

    pub fn AiLightsIntegrateShadowMatte(sg: *mut AtShaderGlobals, bsdf: *mut AtBSDF) -> AtRGB;

    /// \\name Light Getter Methods
    /// \\details
    /// These getters are to avoid slow AiNodeGet*() calls to access light parameters.
    /// Only the most common attributes are exposed for now.
    /// \\{
    pub fn AiLightGetColor(node: *const AtNode) -> AtRGB;

    pub fn AiLightGetIntensity(node: *const AtNode) -> f32;

    pub fn AiLightGetDiffuse(node: *const AtNode) -> f32;

    pub fn AiLightGetSpecular(node: *const AtNode) -> f32;

    pub fn AiLightGetTransmission(node: *const AtNode) -> f32;

    pub fn AiLightGetSSS(node: *const AtNode) -> f32;

    pub fn AiLightGetVolume(node: *const AtNode) -> f32;

    pub fn AiLightGetInfluence(
        sg: *const AtShaderGlobals,
        node: *const AtNode,
        ray_type: u8,
    ) -> f32;

    /// \\name Light IES Format Parser
    /// \\details
    /// Not a shader specific call, this is called by plugins or host DCCs to parse IES files
    /// \\{
    pub fn AiLightIESLoad(
        filename: AtString,
        width: c_uint,
        height: c_uint,
        max_intensity: *mut f32,
        data: *mut f32,
    ) -> bool;
}
