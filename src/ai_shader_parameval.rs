use ::std::os::raw::{c_int, c_uint, c_void};

use super::{
    ai_array::AtArray,
    ai_closure::AtClosureList,
    ai_color::{AtRGB, AtRGBA},
    ai_matrix::AtMatrix,
    ai_nodes::AtNode,
    ai_shaderglobals::AtShaderGlobals,
    ai_string::AtString,
    ai_vector::{AtVector, AtVector2},
};

extern "C" {
    pub fn AiShaderEvalParamFuncByte(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> u8;

    pub fn AiShaderEvalParamFuncInt(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> c_int;

    pub fn AiShaderEvalParamFuncUInt(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> c_uint;

    pub fn AiShaderEvalParamFuncBool(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> bool;

    pub fn AiShaderEvalParamFuncFlt(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> f32;

    pub fn AiShaderEvalParamFuncRGB(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> AtRGB;

    pub fn AiShaderEvalParamFuncRGBA(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> AtRGBA;

    pub fn AiShaderEvalParamFuncVec(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> AtVector;

    pub fn AiShaderEvalParamFuncVec2(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> AtVector2;

    pub fn AiShaderEvalParamFuncStr(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> AtString;

    pub fn AiShaderEvalParamFuncPtr(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> *mut c_void;

    pub fn AiShaderEvalParamFuncArray(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> *mut AtArray;

    pub fn AiShaderEvalParamFuncMtx(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> *mut AtMatrix;

    pub fn AiShaderEvalParamFuncEnum(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> c_int;

    pub fn AiShaderEvalParamFuncClosure(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> AtClosureList;

    pub fn AiShaderEvalParamFuncOpacity(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> AtRGB;
}
