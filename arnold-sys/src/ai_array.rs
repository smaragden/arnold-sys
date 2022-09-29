use ::std::os::raw::{c_int, c_ulong, c_void};

use super::{
    ai_color::{AtRGB, AtRGBA},
    ai_matrix::AtMatrix,
    ai_string::AtString,
    ai_vector::{AtVector, AtVector2},
};

/// Generic array data type
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtArray {
    _unused: [u8; 0],
}
extern "C" {
    pub fn AiArray(nelements: u32, nkeys: u8, type_: c_int, ...) -> *mut AtArray;

    pub fn AiArrayAllocate(nelements: u32, nkeys: u8, type_: u8) -> *mut AtArray;

    pub fn AiArrayDestroy(array: *mut AtArray);

    pub fn AiArrayConvert(
        nelements: u32,
        nkeys: u8,
        type_: u8,
        data: *const c_void,
    ) -> *mut AtArray;

    pub fn AiArrayResize(array: *mut AtArray, nelements: u32, nkeys: u8);

    pub fn AiArrayCopy(array: *const AtArray) -> *mut AtArray;

    pub fn AiArraySetKey(array: *mut AtArray, key: u8, data: *const c_void) -> bool;

    pub fn AiArrayMap(array: *mut AtArray) -> *mut c_void;

    pub fn AiArrayMapKey(array: *mut AtArray, key: u8) -> *mut c_void;

    pub fn AiArrayUnmap(array: *mut AtArray);

    pub fn AiArrayGetNumElements(array: *const AtArray) -> u32;

    pub fn AiArrayGetNumKeys(array: *const AtArray) -> u8;

    pub fn AiArrayGetType(array: *const AtArray) -> u8;

    pub fn AiArrayGetDataSize(array: *const AtArray) -> c_ulong;

    pub fn AiArrayGetKeySize(array: *const AtArray) -> c_ulong;

    pub fn AiArrayInterpolateVec(array: *const AtArray, time: f32, idx: u32) -> AtVector;

    pub fn AiArrayInterpolateRGB(array: *const AtArray, time: f32, idx: u32) -> AtRGB;

    pub fn AiArrayInterpolateRGBA(array: *const AtArray, time: f32, idx: u32) -> AtRGBA;

    pub fn AiArrayInterpolateFlt(array: *const AtArray, time: f32, idx: u32) -> f32;

    pub fn AiArrayInterpolateMtx(array: *const AtArray, time: f32, idx: u32) -> AtMatrix;

    /// \\name AtArray Getters
    ///
    /// \\details
    /// The following getter functions return the i'th element in an array of the
    /// given type.
    /// In case of out-of-bounds access, an error message is generated
    /// \\{
    pub fn AiArrayGetBool(a: *const AtArray, i: u32) -> bool;

    pub fn AiArrayGetByte(a: *const AtArray, i: u32) -> u8;

    pub fn AiArrayGetInt(a: *const AtArray, i: u32) -> c_int;

    pub fn AiArrayGetUInt(a: *const AtArray, i: u32) -> u32;

    pub fn AiArrayGetFlt(a: *const AtArray, i: u32) -> f32;

    pub fn AiArrayGetRGB(a: *const AtArray, i: u32) -> AtRGB;

    pub fn AiArrayGetRGBA(a: *const AtArray, i: u32) -> AtRGBA;

    pub fn AiArrayGetVec2(a: *const AtArray, i: u32) -> AtVector2;

    pub fn AiArrayGetVec(a: *const AtArray, i: u32) -> AtVector;

    pub fn AiArrayGetMtx(a: *const AtArray, i: u32) -> AtMatrix;

    pub fn AiArrayGetStr(a: *const AtArray, i: u32) -> AtString;

    pub fn AiArrayGetPtr(a: *const AtArray, i: u32) -> *mut c_void;

    pub fn AiArrayGetArray(a: *const AtArray, i: u32) -> *mut AtArray;

    /// \\name AtArray Setters
    ///
    /// \\details
    /// The following functions write an element of a given type into the i'th position
    /// in an array. If the write was succesful, these functions will return true, otherwise
    /// a detailed error message will be logged and false will be returned.
    ///
    /// \\{
    pub fn AiArraySetBool(a: *mut AtArray, i: u32, val: bool) -> bool;

    pub fn AiArraySetByte(a: *mut AtArray, i: u32, val: u8) -> bool;

    pub fn AiArraySetInt(a: *mut AtArray, i: u32, val: c_int) -> bool;

    pub fn AiArraySetUInt(a: *mut AtArray, i: u32, val: u32) -> bool;

    pub fn AiArraySetFlt(a: *mut AtArray, i: u32, val: f32) -> bool;

    pub fn AiArraySetRGB(a: *mut AtArray, i: u32, val: AtRGB) -> bool;

    pub fn AiArraySetRGBA(a: *mut AtArray, i: u32, val: AtRGBA) -> bool;

    pub fn AiArraySetVec2(a: *mut AtArray, i: u32, val: AtVector2) -> bool;

    pub fn AiArraySetVec(a: *mut AtArray, i: u32, val: AtVector) -> bool;

    pub fn AiArraySetMtx(a: *mut AtArray, i: u32, val: AtMatrix) -> bool;

    pub fn AiArraySetStr(a: *mut AtArray, i: u32, val: AtString) -> bool;

    pub fn AiArraySetPtr(a: *mut AtArray, i: u32, val: *mut c_void) -> bool;

    pub fn AiArraySetArray(a: *mut AtArray, i: u32, val: *mut AtArray) -> bool;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct POD_tempf2 {
    pub f: [f32; 2usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct POD_tempf3 {
    pub f: [f32; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct POD_tempf4 {
    pub f: [f32; 4usize],
}
