use ::std::os::raw::c_int;

use super::{
    ai_color::{AtRGB, AtRGBA},
    ai_shaderglobals::AtShaderGlobals,
    ai_string::AtString,
    ai_vector::{AtVector, AtVector2},
};

extern "C" {
    pub fn AiVolumeSampleFltFunc(
        channel: AtString,
        sg: *const AtShaderGlobals,
        interp: c_int,
        value: *mut f32,
    ) -> bool;

    pub fn AiVolumeSampleRGBFunc(
        channel: AtString,
        sg: *const AtShaderGlobals,
        interp: c_int,
        value: *mut AtRGB,
    ) -> bool;

    pub fn AiVolumeSampleRGBAFunc(
        channel: AtString,
        sg: *const AtShaderGlobals,
        interp: c_int,
        value: *mut AtRGBA,
    ) -> bool;

    pub fn AiVolumeSampleVecFunc(
        channel: AtString,
        sg: *const AtShaderGlobals,
        interp: c_int,
        value: *mut AtVector,
    ) -> bool;

    pub fn AiVolumeSampleVec2Func(
        channel: AtString,
        sg: *const AtShaderGlobals,
        interp: c_int,
        value: *mut AtVector2,
    ) -> bool;
}
