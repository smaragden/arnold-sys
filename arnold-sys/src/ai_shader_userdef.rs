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

    pub fn AiUserGetBoolFunc(arg1: AtString, arg2: *const AtShaderGlobals, arg3: *mut bool)
        -> bool;

    pub fn AiUserGetByteFunc(arg1: AtString, arg2: *const AtShaderGlobals, arg3: *mut u8) -> bool;

    pub fn AiUserGetIntFunc(arg1: AtString, arg2: *const AtShaderGlobals, arg3: *mut c_int)
        -> bool;

    pub fn AiUserGetUIntFunc(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut c_uint,
    ) -> bool;

    pub fn AiUserGetFltFunc(arg1: AtString, arg2: *const AtShaderGlobals, arg3: *mut f32) -> bool;

    pub fn AiUserGetRGBFunc(arg1: AtString, arg2: *const AtShaderGlobals, arg3: *mut AtRGB)
        -> bool;

    pub fn AiUserGetRGBAFunc(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut AtRGBA,
    ) -> bool;

    pub fn AiUserGetVecFunc(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut AtVector,
    ) -> bool;

    pub fn AiUserGetVec2Func(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut AtVector2,
    ) -> bool;

    pub fn AiUserGetStrFunc(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut AtString,
    ) -> bool;

    pub fn AiUserGetPtrFunc(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut *mut c_void,
    ) -> bool;

    pub fn AiUserGetNodeFunc(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut *mut AtNode,
    ) -> bool;

    pub fn AiUserGetArrayFunc(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut *mut AtArray,
    ) -> bool;

    pub fn AiUserGetMatrixFunc(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut AtMatrix,
    ) -> bool;

    pub fn AiUserGetDxyDerivativesFltFunc(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut f32,
        arg4: *mut f32,
    ) -> bool;

    pub fn AiUserGetDxyDerivativesRGBFunc(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut AtRGB,
        arg4: *mut AtRGB,
    ) -> bool;

    pub fn AiUserGetDxyDerivativesRGBAFunc(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut AtRGBA,
        arg4: *mut AtRGBA,
    ) -> bool;

    pub fn AiUserGetDxyDerivativesVecFunc(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut AtVector,
        arg4: *mut AtVector,
    ) -> bool;

    pub fn AiUserGetDxyDerivativesVec2Func(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut AtVector2,
        arg4: *mut AtVector2,
    ) -> bool;

    pub fn AiUserGetDxyDerivativesArrayFunc(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut *mut AtArray,
        arg4: *mut *mut AtArray,
    ) -> bool;

    pub fn AiUserGetDxyDerivativesMatrixFunc(
        arg1: AtString,
        arg2: *const AtShaderGlobals,
        arg3: *mut AtMatrix,
        arg4: *mut AtMatrix,
    ) -> bool;
}
