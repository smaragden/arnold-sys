use ::std::os::raw::c_int;

use super::ai_shaderglobals::AtShaderGlobals;

/// \\struct AtSampler
/// Opaque data type for a sampler
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtSampler {
    _unused: [u8; 0],
}
/// \\struct AtSamplerIterator
/// Opaque data type for a sampler iterator
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtSamplerIterator {
    _unused: [u8; 0],
}
extern "C" {
    pub fn AiSampler(seed: u32, nsamples: c_int, ndim: c_int) -> *mut AtSampler;
}
extern "C" {
    pub fn AiSamplerIterator(
        sampler: *const AtSampler,
        sg: *const AtShaderGlobals,
    ) -> *mut AtSamplerIterator;
}
extern "C" {
    pub fn AiSamplerGetSample(iterator: *mut AtSamplerIterator, sample: *mut f32) -> bool;
}
extern "C" {
    pub fn AiSamplerGetSampleCount(iterator: *const AtSamplerIterator) -> c_int;
}
extern "C" {
    pub fn AiSamplerGetSampleInvCount(iterator: *const AtSamplerIterator) -> f32;
}
extern "C" {
    pub fn AiSamplerDestroy(sampler: *mut AtSampler);
}
