use ::std::os::raw::{c_char, c_int, c_uint, c_void};

use super::{
    ai_array::AtArray, ai_enum::AtEnum, ai_matrix::AtMatrix, ai_nodes::AtNode, ai_string::AtString,
};

pub const AI_TYPE_BYTE: u32 = 0;
pub const AI_TYPE_INT: u32 = 1;
pub const AI_TYPE_UINT: u32 = 2;
pub const AI_TYPE_BOOLEAN: u32 = 3;
pub const AI_TYPE_FLOAT: u32 = 4;
pub const AI_TYPE_RGB: u32 = 5;
pub const AI_TYPE_RGBA: u32 = 6;
pub const AI_TYPE_VECTOR: u32 = 7;
pub const AI_TYPE_VECTOR2: u32 = 9;
pub const AI_TYPE_STRING: u32 = 10;
pub const AI_TYPE_POINTER: u32 = 11;
pub const AI_TYPE_NODE: u32 = 12;
pub const AI_TYPE_ARRAY: u32 = 13;
pub const AI_TYPE_MATRIX: u32 = 14;
pub const AI_TYPE_ENUM: u32 = 15;
pub const AI_TYPE_CLOSURE: u32 = 16;
pub const AI_TYPE_USHORT: u32 = 17;
pub const AI_TYPE_HALF: u32 = 18;
pub const AI_TYPE_UNDEFINED: u32 = 255;
pub const AI_TYPE_NONE: u32 = 255;
pub const AI_USERDEF_UNDEFINED: u32 = 0;
pub const AI_USERDEF_CONSTANT: u32 = 1;
pub const AI_USERDEF_UNIFORM: u32 = 2;
pub const AI_USERDEF_VARYING: u32 = 3;
pub const AI_USERDEF_INDEXED: u32 = 4;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtList {
    _unused: [u8; 0],
}

#[doc = " Actual parameter value for each supported type"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtParamValue {
    pub data: [u64; 2usize],
}
#[doc = " This represents a parameter of a given node type in Arnold."]
#[doc = ""]
#[doc = " This holds details like the name, type and default value. The actual"]
#[doc = " contents of this struct are private."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtParamEntry {
    _unused: [u8; 0],
}
extern "C" {
    #[doc = " \\name AtParamEntry Methods"]
    #[doc = " \\{"]
    pub fn AiParamGetName(pentry: *const AtParamEntry) -> AtString;
}
extern "C" {
    pub fn AiParamGetType(pentry: *const AtParamEntry) -> u8;
}
extern "C" {
    pub fn AiParamGetSubType(pentry: *const AtParamEntry) -> u8;
}
extern "C" {
    pub fn AiParamGetDefault(pentry: *const AtParamEntry) -> *const AtParamValue;
}
extern "C" {
    pub fn AiParamGetEnum(pentry: *const AtParamEntry) -> AtEnum;
}
extern "C" {
    pub fn AiParamGetTypeName(type_: u8) -> *const c_char;
}
extern "C" {
    pub fn AiParamGetTypeSize(type_: u8) -> c_int;
}
#[doc = " This represents a user-declared parameter in Arnold (user-data)."]
#[doc = ""]
#[doc = " This holds details like name, type, and category.  The actual"]
#[doc = " contents of this struct are private."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtUserParamEntry {
    _unused: [u8; 0],
}
extern "C" {
    #[doc = " \\name AtUserParamEntry Methods"]
    #[doc = " \\{"]
    pub fn AiUserParamGetName(upentry: *const AtUserParamEntry) -> *const c_char;
}
extern "C" {
    pub fn AiUserParamGetType(upentry: *const AtUserParamEntry) -> u8;
}
extern "C" {
    pub fn AiUserParamGetArrayType(upentry: *const AtUserParamEntry) -> u8;
}
extern "C" {
    pub fn AiUserParamGetCategory(upentry: *const AtUserParamEntry) -> u8;
}
extern "C" {
    #[doc = " Returns whether an AtParamValue of type src_type can be converted to"]
    #[doc = " dst_type."]
    #[doc = ""]
    #[doc = " For instance, \\c AiParamTypeConvertible(AI_TYPE_FLOAT, AI_TYPE_INT)"]
    #[doc = " would return true since an AtParamValue containing an int can be converted"]
    #[doc = " into an AtParamValue containing a float."]
    #[doc = ""]
    #[doc = " \\param  dst_type  type of the destination value"]
    #[doc = " \\param  src_type  type of the source value"]
    pub fn AiParamTypeConvertible(dst_type: u8, src_type: u8) -> bool;
}
extern "C" {
    pub fn AiNodeParamByte(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        pdefault: u8,
    );
}
extern "C" {
    pub fn AiNodeParamInt(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        pdefault: c_int,
    );
}
extern "C" {
    pub fn AiNodeParamUInt(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        pdefault: c_uint,
    );
}
extern "C" {
    pub fn AiNodeParamBool(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        pdefault: bool,
    );
}
extern "C" {
    pub fn AiNodeParamFlt(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        pdefault: f32,
    );
}
extern "C" {
    pub fn AiNodeParamRGB(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        r: f32,
        g: f32,
        b: f32,
    );
}
extern "C" {
    pub fn AiNodeParamRGBA(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    );
}
extern "C" {
    pub fn AiNodeParamVec(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        x: f32,
        y: f32,
        z: f32,
    );
}
extern "C" {
    pub fn AiNodeParamVec2(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        x: f32,
        y: f32,
    );
}
extern "C" {
    pub fn AiNodeParamStr(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        pdefault: *const c_char,
    );
}
extern "C" {
    pub fn AiNodeParamPtr(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        pdefault: *mut c_void,
    );
}
extern "C" {
    pub fn AiNodeParamNode(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        pdefault: *mut AtNode,
    );
}
extern "C" {
    pub fn AiNodeParamArray(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        pdefault: *mut AtArray,
    );
}
extern "C" {
    pub fn AiNodeParamMtx(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        matrix: AtMatrix,
    );
}
extern "C" {
    pub fn AiNodeParamEnum(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        pdefault: c_int,
        enum_type: AtEnum,
    );
}
extern "C" {
    pub fn AiNodeParamClosure(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
    );
}
extern "C" {
    pub fn AiNodeOutputByte(params: *mut AtList, pname: *const c_char);
}
extern "C" {
    pub fn AiNodeOutputInt(params: *mut AtList, pname: *const c_char);
}
extern "C" {
    pub fn AiNodeOutputUInt(params: *mut AtList, pname: *const c_char);
}
extern "C" {
    pub fn AiNodeOutputBool(params: *mut AtList, pname: *const c_char);
}
extern "C" {
    pub fn AiNodeOutputFlt(params: *mut AtList, pname: *const c_char);
}
extern "C" {
    pub fn AiNodeOutputRGB(params: *mut AtList, pname: *const c_char);
}
extern "C" {
    pub fn AiNodeOutputRGBA(params: *mut AtList, pname: *const c_char);
}
extern "C" {
    pub fn AiNodeOutputVec(params: *mut AtList, pname: *const c_char);
}
extern "C" {
    pub fn AiNodeOutputVec2(params: *mut AtList, pname: *const c_char);
}
extern "C" {
    pub fn AiNodeOutputStr(params: *mut AtList, pname: *const c_char);
}
extern "C" {
    pub fn AiNodeOutputPtr(params: *mut AtList, pname: *const c_char);
}
extern "C" {
    pub fn AiNodeOutputNode(params: *mut AtList, pname: *const c_char);
}
extern "C" {
    pub fn AiNodeOutputArray(
        params: *mut AtList,
        pname: *const c_char,
        array_type: c_int,
    );
}
extern "C" {
    pub fn AiNodeOutputMtx(params: *mut AtList, pname: *const c_char);
}
extern "C" {
    pub fn AiNodeOutputEnum(
        params: *mut AtList,
        pname: *const c_char,
        enum_type: AtEnum,
    );
}
extern "C" {
    pub fn AiNodeOutputClosure(params: *mut AtList, pname: *const c_char);
}
