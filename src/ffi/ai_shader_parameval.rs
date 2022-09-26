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
}
extern "C" {
    pub fn AiShaderEvalParamFuncInt(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn AiShaderEvalParamFuncUInt(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> c_uint;
}
extern "C" {
    pub fn AiShaderEvalParamFuncBool(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> bool;
}
extern "C" {
    pub fn AiShaderEvalParamFuncFlt(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> f32;
}
extern "C" {
    pub fn AiShaderEvalParamFuncRGB(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> AtRGB;
}
extern "C" {
    pub fn AiShaderEvalParamFuncRGBA(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> AtRGBA;
}
extern "C" {
    pub fn AiShaderEvalParamFuncVec(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> AtVector;
}
extern "C" {
    pub fn AiShaderEvalParamFuncVec2(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> AtVector2;
}
extern "C" {
    pub fn AiShaderEvalParamFuncStr(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> AtString;
}
extern "C" {
    pub fn AiShaderEvalParamFuncPtr(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> *mut c_void;
}
extern "C" {
    pub fn AiShaderEvalParamFuncArray(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> *mut AtArray;
}
extern "C" {
    pub fn AiShaderEvalParamFuncMtx(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> *mut AtMatrix;
}
extern "C" {
    pub fn AiShaderEvalParamFuncEnum(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> c_int;
}
extern "C" {
    pub fn AiShaderEvalParamFuncClosure(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> AtClosureList;
}
extern "C" {
    pub fn AiShaderEvalParamFuncOpacity(
        sg: *mut AtShaderGlobals,
        node: *const AtNode,
        pid: c_int,
    ) -> AtRGB;
}
