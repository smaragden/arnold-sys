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
}
extern "C" {
    pub fn AiArrayAllocate(nelements: u32, nkeys: u8, type_: u8) -> *mut AtArray;
}
extern "C" {
    pub fn AiArrayDestroy(array: *mut AtArray);
}
extern "C" {
    pub fn AiArrayConvert(
        nelements: u32,
        nkeys: u8,
        type_: u8,
        data: *const c_void,
    ) -> *mut AtArray;
}
extern "C" {
    pub fn AiArrayResize(array: *mut AtArray, nelements: u32, nkeys: u8);
}
extern "C" {
    pub fn AiArrayCopy(array: *const AtArray) -> *mut AtArray;
}
extern "C" {
    pub fn AiArraySetKey(array: *mut AtArray, key: u8, data: *const c_void) -> bool;
}
extern "C" {
    pub fn AiArrayMap(array: *mut AtArray) -> *mut c_void;
}
extern "C" {
    pub fn AiArrayMapKey(array: *mut AtArray, key: u8) -> *mut c_void;
}
extern "C" {
    pub fn AiArrayUnmap(array: *mut AtArray);
}
extern "C" {
    pub fn AiArrayGetNumElements(array: *const AtArray) -> u32;
}
extern "C" {
    pub fn AiArrayGetNumKeys(array: *const AtArray) -> u8;
}
extern "C" {
    pub fn AiArrayGetType(array: *const AtArray) -> u8;
}
extern "C" {
    pub fn AiArrayGetDataSize(array: *const AtArray) -> c_ulong;
}
extern "C" {
    pub fn AiArrayGetKeySize(array: *const AtArray) -> c_ulong;
}
extern "C" {
    pub fn AiArrayInterpolateVec(array: *const AtArray, time: f32, idx: u32) -> AtVector;
}
extern "C" {
    pub fn AiArrayInterpolateRGB(array: *const AtArray, time: f32, idx: u32) -> AtRGB;
}
extern "C" {
    pub fn AiArrayInterpolateRGBA(array: *const AtArray, time: f32, idx: u32) -> AtRGBA;
}
extern "C" {
    pub fn AiArrayInterpolateFlt(array: *const AtArray, time: f32, idx: u32) -> f32;
}
extern "C" {
    pub fn AiArrayInterpolateMtx(array: *const AtArray, time: f32, idx: u32) -> AtMatrix;
}
extern "C" {
    /// \\name AtArray Getters
    ///
    /// \\details
    /// The following getter functions return the i'th element in an array of the
    /// given type.
    /// In case of out-of-bounds access, an error message is generated
    /// \\{
    pub fn AiArrayGetBool(a: *const AtArray, i: u32) -> bool;
}
extern "C" {
    pub fn AiArrayGetByte(a: *const AtArray, i: u32) -> u8;
}
extern "C" {
    pub fn AiArrayGetInt(a: *const AtArray, i: u32) -> c_int;
}
extern "C" {
    pub fn AiArrayGetUInt(a: *const AtArray, i: u32) -> u32;
}
extern "C" {
    pub fn AiArrayGetFlt(a: *const AtArray, i: u32) -> f32;
}
extern "C" {
    pub fn AiArrayGetRGB(a: *const AtArray, i: u32) -> AtRGB;
}
extern "C" {
    pub fn AiArrayGetRGBA(a: *const AtArray, i: u32) -> AtRGBA;
}
extern "C" {
    pub fn AiArrayGetVec2(a: *const AtArray, i: u32) -> AtVector2;
}
extern "C" {
    pub fn AiArrayGetVec(a: *const AtArray, i: u32) -> AtVector;
}
extern "C" {
    pub fn AiArrayGetMtx(a: *const AtArray, i: u32) -> AtMatrix;
}
extern "C" {
    pub fn AiArrayGetStr(a: *const AtArray, i: u32) -> AtString;
}
extern "C" {
    pub fn AiArrayGetPtr(a: *const AtArray, i: u32) -> *mut c_void;
}
extern "C" {
    pub fn AiArrayGetArray(a: *const AtArray, i: u32) -> *mut AtArray;
}
extern "C" {
    /// \\name AtArray Setters
    ///
    /// \\details
    /// The following functions write an element of a given type into the i'th position
    /// in an array. If the write was succesful, these functions will return true, otherwise
    /// a detailed error message will be logged and false will be returned.
    ///
    /// \\{
    pub fn AiArraySetBool(a: *mut AtArray, i: u32, val: bool) -> bool;
}
extern "C" {
    pub fn AiArraySetByte(a: *mut AtArray, i: u32, val: u8) -> bool;
}
extern "C" {
    pub fn AiArraySetInt(a: *mut AtArray, i: u32, val: c_int) -> bool;
}
extern "C" {
    pub fn AiArraySetUInt(a: *mut AtArray, i: u32, val: u32) -> bool;
}
extern "C" {
    pub fn AiArraySetFlt(a: *mut AtArray, i: u32, val: f32) -> bool;
}
extern "C" {
    pub fn AiArraySetRGB(a: *mut AtArray, i: u32, val: AtRGB) -> bool;
}
extern "C" {
    pub fn AiArraySetRGBA(a: *mut AtArray, i: u32, val: AtRGBA) -> bool;
}
extern "C" {
    pub fn AiArraySetVec2(a: *mut AtArray, i: u32, val: AtVector2) -> bool;
}
extern "C" {
    pub fn AiArraySetVec(a: *mut AtArray, i: u32, val: AtVector) -> bool;
}
extern "C" {
    pub fn AiArraySetMtx(a: *mut AtArray, i: u32, val: AtMatrix) -> bool;
}
extern "C" {
    pub fn AiArraySetStr(a: *mut AtArray, i: u32, val: AtString) -> bool;
}
extern "C" {
    pub fn AiArraySetPtr(a: *mut AtArray, i: u32, val: *mut c_void) -> bool;
}
extern "C" {
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
