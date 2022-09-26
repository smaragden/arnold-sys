use ::std::os::raw::{c_int, c_uint, c_void};

use super::{
    ai_array::AtArray,
    ai_color::{AtRGB, AtRGBA},
    ai_shaderglobals::AtShaderGlobals,
    ai_string::AtString,
    ai_vector::{AtVector, AtVector2},
};

#[doc = " This represents a message iterator. The actual contents of this struct are"]
#[doc = " private."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtMessageIterator {
    _unused: [u8; 0],
}
extern "C" {
    pub fn AiMessageIterator(sg: *const AtShaderGlobals) -> *mut AtMessageIterator;
}
extern "C" {
    pub fn AiMessageIteratorGetNext(
        iterator: *mut AtMessageIterator,
        msg_name: *mut AtString,
        msg_type: *mut c_int,
    ) -> bool;
}
extern "C" {
    pub fn AiMessageGetBoolFunc(
        arg1: *const AtShaderGlobals,
        arg2: AtString,
        arg3: *mut bool,
    ) -> bool;
}
extern "C" {
    pub fn AiMessageSetBoolFunc(arg1: *mut AtShaderGlobals, arg2: AtString, arg3: bool) -> bool;
}
extern "C" {
    pub fn AiMessageUnsetBoolFunc(arg1: *mut AtShaderGlobals, arg2: AtString) -> bool;
}
extern "C" {
    pub fn AiMessageGetByteFunc(
        arg1: *const AtShaderGlobals,
        arg2: AtString,
        arg3: *mut u8,
    ) -> bool;
}
extern "C" {
    pub fn AiMessageSetByteFunc(arg1: *mut AtShaderGlobals, arg2: AtString, arg3: u8) -> bool;
}
extern "C" {
    pub fn AiMessageUnsetByteFunc(arg1: *mut AtShaderGlobals, arg2: AtString) -> bool;
}
extern "C" {
    pub fn AiMessageGetIntFunc(
        arg1: *const AtShaderGlobals,
        arg2: AtString,
        arg3: *mut c_int,
    ) -> bool;
}
extern "C" {
    pub fn AiMessageSetIntFunc(arg1: *mut AtShaderGlobals, arg2: AtString, arg3: c_int) -> bool;
}
extern "C" {
    pub fn AiMessageUnsetIntFunc(arg1: *mut AtShaderGlobals, arg2: AtString) -> bool;
}
extern "C" {
    pub fn AiMessageGetUIntFunc(
        arg1: *const AtShaderGlobals,
        arg2: AtString,
        arg3: *mut c_uint,
    ) -> bool;
}
extern "C" {
    pub fn AiMessageSetUIntFunc(arg1: *mut AtShaderGlobals, arg2: AtString, arg3: c_uint) -> bool;
}
extern "C" {
    pub fn AiMessageUnsetUIntFunc(arg1: *mut AtShaderGlobals, arg2: AtString) -> bool;
}
extern "C" {
    pub fn AiMessageGetFltFunc(
        arg1: *const AtShaderGlobals,
        arg2: AtString,
        arg3: *mut f32,
    ) -> bool;
}
extern "C" {
    pub fn AiMessageSetFltFunc(arg1: *mut AtShaderGlobals, arg2: AtString, arg3: f32) -> bool;
}
extern "C" {
    pub fn AiMessageUnsetFltFunc(arg1: *mut AtShaderGlobals, arg2: AtString) -> bool;
}
extern "C" {
    pub fn AiMessageGetRGBFunc(
        arg1: *const AtShaderGlobals,
        arg2: AtString,
        arg3: *mut AtRGB,
    ) -> bool;
}
extern "C" {
    pub fn AiMessageSetRGBFunc(arg1: *mut AtShaderGlobals, arg2: AtString, arg3: AtRGB) -> bool;
}
extern "C" {
    pub fn AiMessageUnsetRGBFunc(arg1: *mut AtShaderGlobals, arg2: AtString) -> bool;
}
extern "C" {
    pub fn AiMessageGetRGBAFunc(
        arg1: *const AtShaderGlobals,
        arg2: AtString,
        arg3: *mut AtRGBA,
    ) -> bool;
}
extern "C" {
    pub fn AiMessageSetRGBAFunc(arg1: *mut AtShaderGlobals, arg2: AtString, arg3: AtRGBA) -> bool;
}
extern "C" {
    pub fn AiMessageUnsetRGBAFunc(arg1: *mut AtShaderGlobals, arg2: AtString) -> bool;
}
extern "C" {
    pub fn AiMessageGetVecFunc(
        arg1: *const AtShaderGlobals,
        arg2: AtString,
        arg3: *mut AtVector,
    ) -> bool;
}
extern "C" {
    pub fn AiMessageSetVecFunc(arg1: *mut AtShaderGlobals, arg2: AtString, arg3: AtVector) -> bool;
}
extern "C" {
    pub fn AiMessageUnsetVecFunc(arg1: *mut AtShaderGlobals, arg2: AtString) -> bool;
}
extern "C" {
    pub fn AiMessageGetVec2Func(
        arg1: *const AtShaderGlobals,
        arg2: AtString,
        arg3: *mut AtVector2,
    ) -> bool;
}
extern "C" {
    pub fn AiMessageSetVec2Func(
        arg1: *mut AtShaderGlobals,
        arg2: AtString,
        arg3: AtVector2,
    ) -> bool;
}
extern "C" {
    pub fn AiMessageUnsetVec2Func(arg1: *mut AtShaderGlobals, arg2: AtString) -> bool;
}
extern "C" {
    pub fn AiMessageGetStrFunc(
        arg1: *const AtShaderGlobals,
        arg2: AtString,
        arg3: *mut AtString,
    ) -> bool;
}
extern "C" {
    pub fn AiMessageSetStrFunc(arg1: *mut AtShaderGlobals, arg2: AtString, arg3: AtString) -> bool;
}
extern "C" {
    pub fn AiMessageUnsetStrFunc(arg1: *mut AtShaderGlobals, arg2: AtString) -> bool;
}
extern "C" {
    pub fn AiMessageGetPtrFunc(
        arg1: *const AtShaderGlobals,
        arg2: AtString,
        arg3: *mut *mut c_void,
    ) -> bool;
}
extern "C" {
    pub fn AiMessageSetPtrFunc(
        arg1: *mut AtShaderGlobals,
        arg2: AtString,
        arg3: *mut c_void,
    ) -> bool;
}
extern "C" {
    pub fn AiMessageUnsetPtrFunc(arg1: *mut AtShaderGlobals, arg2: AtString) -> bool;
}
extern "C" {
    pub fn AiMessageGetArrayFunc(
        arg1: *const AtShaderGlobals,
        arg2: AtString,
        arg3: *mut *mut AtArray,
    ) -> bool;
}
extern "C" {
    pub fn AiMessageSetArrayFunc(
        arg1: *mut AtShaderGlobals,
        arg2: AtString,
        arg3: *mut AtArray,
    ) -> bool;
}
extern "C" {
    pub fn AiMessageUnsetArrayFunc(arg1: *mut AtShaderGlobals, arg2: AtString) -> bool;
}
