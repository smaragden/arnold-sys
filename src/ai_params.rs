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

/// Actual parameter value for each supported type
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtParamValue {
    pub data: [u64; 2usize],
}
/// This represents a parameter of a given node type in Arnold.
///
/// This holds details like the name, type and default value. The actual
/// contents of this struct are private.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtParamEntry {
    _unused: [u8; 0],
}
extern "C" {
    /// \\name AtParamEntry Methods
    /// \\{
    pub fn AiParamGetName(pentry: *const AtParamEntry) -> AtString;

    pub fn AiParamGetType(pentry: *const AtParamEntry) -> u8;

    pub fn AiParamGetSubType(pentry: *const AtParamEntry) -> u8;

    pub fn AiParamGetDefault(pentry: *const AtParamEntry) -> *const AtParamValue;

    pub fn AiParamGetEnum(pentry: *const AtParamEntry) -> AtEnum;

    pub fn AiParamGetTypeName(type_: u8) -> *const c_char;

    pub fn AiParamGetTypeSize(type_: u8) -> c_int;
}
/// This represents a user-declared parameter in Arnold (user-data).
///
/// This holds details like name, type, and category.  The actual
/// contents of this struct are private.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtUserParamEntry {
    _unused: [u8; 0],
}
extern "C" {
    /// \\name AtUserParamEntry Methods
    /// \\{
    pub fn AiUserParamGetName(upentry: *const AtUserParamEntry) -> *const c_char;

    pub fn AiUserParamGetType(upentry: *const AtUserParamEntry) -> u8;

    pub fn AiUserParamGetArrayType(upentry: *const AtUserParamEntry) -> u8;

    pub fn AiUserParamGetCategory(upentry: *const AtUserParamEntry) -> u8;

    /// Returns whether an AtParamValue of type src_type can be converted to
    /// dst_type.
    ///
    /// For instance, \\c AiParamTypeConvertible(AI_TYPE_FLOAT, AI_TYPE_INT)
    /// would return true since an AtParamValue containing an int can be converted
    /// into an AtParamValue containing a float.
    ///
    /// \\param  dst_type  type of the destination value
    /// \\param  src_type  type of the source value
    pub fn AiParamTypeConvertible(dst_type: u8, src_type: u8) -> bool;

    pub fn AiNodeParamByte(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        pdefault: u8,
    );

    pub fn AiNodeParamInt(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        pdefault: c_int,
    );

    pub fn AiNodeParamUInt(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        pdefault: c_uint,
    );

    pub fn AiNodeParamBool(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        pdefault: bool,
    );

    pub fn AiNodeParamFlt(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        pdefault: f32,
    );

    pub fn AiNodeParamRGB(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        r: f32,
        g: f32,
        b: f32,
    );

    pub fn AiNodeParamRGBA(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    );

    pub fn AiNodeParamVec(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        x: f32,
        y: f32,
        z: f32,
    );

    pub fn AiNodeParamVec2(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        x: f32,
        y: f32,
    );

    pub fn AiNodeParamStr(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        pdefault: *const c_char,
    );

    pub fn AiNodeParamPtr(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        pdefault: *mut c_void,
    );

    pub fn AiNodeParamNode(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        pdefault: *mut AtNode,
    );

    pub fn AiNodeParamArray(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        pdefault: *mut AtArray,
    );

    pub fn AiNodeParamMtx(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        matrix: AtMatrix,
    );

    pub fn AiNodeParamEnum(
        params: *mut AtList,
        varoffset: c_int,
        pname: *const c_char,
        pdefault: c_int,
        enum_type: AtEnum,
    );

    pub fn AiNodeParamClosure(params: *mut AtList, varoffset: c_int, pname: *const c_char);

    pub fn AiNodeOutputByte(params: *mut AtList, pname: *const c_char);

    pub fn AiNodeOutputInt(params: *mut AtList, pname: *const c_char);

    pub fn AiNodeOutputUInt(params: *mut AtList, pname: *const c_char);

    pub fn AiNodeOutputBool(params: *mut AtList, pname: *const c_char);

    pub fn AiNodeOutputFlt(params: *mut AtList, pname: *const c_char);

    pub fn AiNodeOutputRGB(params: *mut AtList, pname: *const c_char);

    pub fn AiNodeOutputRGBA(params: *mut AtList, pname: *const c_char);

    pub fn AiNodeOutputVec(params: *mut AtList, pname: *const c_char);

    pub fn AiNodeOutputVec2(params: *mut AtList, pname: *const c_char);

    pub fn AiNodeOutputStr(params: *mut AtList, pname: *const c_char);

    pub fn AiNodeOutputPtr(params: *mut AtList, pname: *const c_char);

    pub fn AiNodeOutputNode(params: *mut AtList, pname: *const c_char);

    pub fn AiNodeOutputArray(params: *mut AtList, pname: *const c_char, array_type: c_int);

    pub fn AiNodeOutputMtx(params: *mut AtList, pname: *const c_char);

    pub fn AiNodeOutputEnum(params: *mut AtList, pname: *const c_char, enum_type: AtEnum);

    pub fn AiNodeOutputClosure(params: *mut AtList, pname: *const c_char);
}
