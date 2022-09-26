use ::std::os::raw::{c_int, c_uint, c_void};

use super::{
    ai_array::AtArray,
    ai_color::{AtRGB, AtRGBA},
    ai_matrix::AtMatrix,
    ai_nodes::AtNode,
    ai_params::AtUserParamEntry,
    ai_shaderglobals::AtShaderGlobals,
    ai_string::AtString,
    ai_vector::{AtVector, AtVector2},
};

extern "C" {
    pub fn AiUserGetParameterFunc(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
    ) -> *const AtUserParamEntry;
}
extern "C" {
    pub fn AiUserGetBoolFunc(arg1: AtString, arg2: *const AtShaderGlobals, arg3: *mut bool)
        -> bool;
}
extern "C" {
    pub fn AiUserGetByteFunc(arg1: AtString, arg2: *const AtShaderGlobals, arg3: *mut u8) -> bool;
}
extern "C" {
    pub fn AiUserGetIntFunc(arg1: AtString, arg2: *const AtShaderGlobals, arg3: *mut c_int)
        -> bool;
}
extern "C" {
    pub fn AiUserGetUIntFunc(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut c_uint,
    ) -> bool;
}
extern "C" {
    pub fn AiUserGetFltFunc(arg1: AtString, arg2: *const AtShaderGlobals, arg3: *mut f32) -> bool;
}
extern "C" {
    pub fn AiUserGetRGBFunc(arg1: AtString, arg2: *const AtShaderGlobals, arg3: *mut AtRGB)
        -> bool;
}
extern "C" {
    pub fn AiUserGetRGBAFunc(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut AtRGBA,
    ) -> bool;
}
extern "C" {
    pub fn AiUserGetVecFunc(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut AtVector,
    ) -> bool;
}
extern "C" {
    pub fn AiUserGetVec2Func(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut AtVector2,
    ) -> bool;
}
extern "C" {
    pub fn AiUserGetStrFunc(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut AtString,
    ) -> bool;
}
extern "C" {
    pub fn AiUserGetPtrFunc(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut *mut c_void,
    ) -> bool;
}
extern "C" {
    pub fn AiUserGetNodeFunc(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut *mut AtNode,
    ) -> bool;
}
extern "C" {
    pub fn AiUserGetArrayFunc(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut *mut AtArray,
    ) -> bool;
}
extern "C" {
    pub fn AiUserGetMatrixFunc(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut AtMatrix,
    ) -> bool;
}
extern "C" {
    pub fn AiUserGetDxyDerivativesFltFunc(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut f32,
        arg4: *mut f32,
    ) -> bool;
}
extern "C" {
    pub fn AiUserGetDxyDerivativesRGBFunc(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut AtRGB,
        arg4: *mut AtRGB,
    ) -> bool;
}
extern "C" {
    pub fn AiUserGetDxyDerivativesRGBAFunc(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut AtRGBA,
        arg4: *mut AtRGBA,
    ) -> bool;
}
extern "C" {
    pub fn AiUserGetDxyDerivativesVecFunc(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut AtVector,
        arg4: *mut AtVector,
    ) -> bool;
}
extern "C" {
    pub fn AiUserGetDxyDerivativesVec2Func(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut AtVector2,
        arg4: *mut AtVector2,
    ) -> bool;
}
extern "C" {
    pub fn AiUserGetDxyDerivativesArrayFunc(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut *mut AtArray,
        arg4: *mut *mut AtArray,
    ) -> bool;
}
extern "C" {
    pub fn AiUserGetDxyDerivativesMatrixFunc(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut AtMatrix,
        arg4: *mut AtMatrix,
    ) -> bool;
}
