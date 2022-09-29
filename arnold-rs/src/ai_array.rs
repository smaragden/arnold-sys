#![allow(non_camel_case_types, non_snake_case)]

use std::ffi::c_void;
use crate::{ai_nodes::AtNode, ai_string::AtString};

pub struct AtArray {
    pub(crate) raw: *mut arnold_sys::ai_array::AtArray
}

impl AtArray {
    pub fn raw(&self) -> *mut arnold_sys::ai_array::AtArray {
        unsafe { &mut *self.raw }
    }
}

// As we dond have variadics, lets just piggyback on AiArrayConvert
// THIS IS FAR FROM SAFE!!
pub fn AiArray<T>(nelements: u32, nkeys: u8, type_: u32, values: &[T]) -> AtArray{
    AtArray{
        raw: unsafe{ arnold_sys::ai_array::AiArrayConvert(nelements, nkeys, type_ as u8, values.as_ptr() as *mut c_void)}
    }
}

pub fn AiArrayAllocate(nelements: u32, nkeys: u8, type_: u32) -> AtArray{
    AtArray{
        raw: unsafe{arnold_sys::ai_array::AiArrayAllocate(nelements, nkeys, type_ as u8)}
    }
}

pub fn AiArraySetBool(a: &AtArray, i: u32, val: bool) -> bool{
    unsafe {arnold_sys::ai_array::AiArraySetBool(a.raw(), i, val)}
}

pub fn AiArraySetByte(a: &AtArray, i: u32, val: u8) -> bool{
    unsafe {arnold_sys::ai_array::AiArraySetByte(a.raw(), i, val)}
}

pub fn AiArraySetInt(a: &AtArray, i: u32, val: i32) -> bool{
    unsafe {arnold_sys::ai_array::AiArraySetInt(a.raw(), i, val)}
}

pub fn AiArraySetUInt(a: &AtArray, i: u32, val: u32) -> bool{
    unsafe {arnold_sys::ai_array::AiArraySetUInt(a.raw(), i, val)}
}

pub fn AiArraySetFlt(a: &AtArray, i: u32, val: f32) -> bool{
    unsafe {arnold_sys::ai_array::AiArraySetFlt(a.raw(), i, val)}
}

/*
pub fn AiArraySetRGB(a: &AtArray, i: u32, val: AtRGB) -> bool{
    unsafe {arnold_sys::ai_array::AiArraySetRGB(a.raw(), i, val)}
}

pub fn AiArraySetRGBA(a: &AtArray, i: u32, val: AtRGBA) -> bool{
    unsafe {arnold_sys::ai_array::AiArraySetRGBA(a.raw(), i, val)}
}

pub fn AiArraySetVec2(a: &AtArray, i: u32, val: AtVector2) -> bool{
    unsafe {arnold_sys::ai_array::AiArraySetVec2(a.raw(), i, val)}
}

pub fn AiArraySetVec(a: &AtArray, i: u32, val: AtVector) -> bool{
    unsafe {arnold_sys::ai_array::AiArraySetVec(a.raw(), i, val)}
}

pub fn AiArraySetMtx(a: &AtArray, i: u32, val: AtMatrix) -> bool{
    unsafe {arnold_sys::ai_array::AiArraySetMtx(a.raw(), i, val)}
}
*/

pub fn AiArraySetStr<T: Into<AtString>>(a: &AtArray, i: u32, val: T) -> bool{
    let val = val.into().into();
    unsafe {arnold_sys::ai_array::AiArraySetStr(a.raw(), i, val)}
}
pub fn AiArraySetPtr(a: &AtArray, i: u32, val: &AtNode) -> bool{
    unsafe {arnold_sys::ai_array::AiArraySetPtr(a.raw(), i, val.raw() as *mut c_void)}
}

pub fn AiArraySetArray(a: &AtArray, i: u32, val: &AtArray) -> bool{
    unsafe {arnold_sys::ai_array::AiArraySetArray(a.raw(), i, val.raw())}
}
