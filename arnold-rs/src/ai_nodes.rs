#![allow(non_snake_case)]

use std::ffi::{CString, c_void};

use crate::{ai_universe::AtUniverse, ai_string::AtString, ai_array::AtArray};


pub struct AtNode {
    pub(crate) raw: *mut arnold_sys::ai_nodes::AtNode
}

impl AtNode {
    pub fn raw(&self) -> *mut arnold_sys::ai_nodes::AtNode {
        unsafe { &mut *self.raw }
    }
}

pub fn AiNode<T: Into<AtString>>(universe: Option<&AtUniverse>, nentry_name: T, name: Option<T>, parent: Option<AtNode>) -> AtNode {
    let universe = match universe {
        Some(universe) => universe.raw(),
        None => std::ptr::null_mut(),
    };

    let parent = match parent {
        Some(parent) => parent.raw(),
        None => std::ptr::null_mut(),
    };

    let name = match name {
        Some(name) => name.into(),
        None => AtString::null().into(),
    };

    let nentry_name = nentry_name.into().into();

    AtNode {
        raw: unsafe{arnold_sys::ai_nodes::AiNode(universe, nentry_name, name.into(), parent)},
    }
}

pub fn AiNodeLink(node:&AtNode, input: &str, target: Option<&AtNode>) -> bool {
    let target = match target {
        Some(target) => target.raw(),
        None => std::ptr::null_mut(),
    };
    let input = CString::new(input).unwrap();
    unsafe{arnold_sys::ai_nodes::AiNodeLink(node.raw(), input.as_ptr(), target)}
}


pub fn AiNodeSetStr<T: Into<AtString>>(node: &AtNode, param: T, str_: T) {
    unsafe { arnold_sys::ai_nodes::AiNodeSetStr(node.raw(), param.into().into(), str_.into().into()) };
}

pub fn AiNodeSetVec<T: Into<AtString>>(node: &AtNode, param: T, x: f32, y: f32, z: f32) {
    unsafe { arnold_sys::ai_nodes::AiNodeSetVec(node.raw(), param.into().into(), x, y, z) };
}

pub fn AiNodeSetInt<T: Into<AtString>>(node: &AtNode, param: T, val: i32) {
    unsafe { arnold_sys::ai_nodes::AiNodeSetInt(node.raw(), param.into().into(), val) };
}

pub fn AiNodeSetFlt<T: Into<AtString>>(node: &AtNode, param: T, val: f32) {
    unsafe { arnold_sys::ai_nodes::AiNodeSetFlt(node.raw(), param.into().into(), val) };
}

pub fn AiNodeSetRGB<T: Into<AtString>>(node: &AtNode, param: T, r: f32, g: f32, b: f32) {
    unsafe { arnold_sys::ai_nodes::AiNodeSetRGB(node.raw(), param.into().into(), r, g, b) };
}

pub fn AiNodeSetArray<T: Into<AtString>>(node: &AtNode, param: T, val: &AtArray) {
    unsafe { arnold_sys::ai_nodes::AiNodeSetArray(node.raw(), param.into().into(), val.raw()) };
}

pub fn AiNodeSetPtr<T: Into<AtString>>(node: &AtNode, param: T, val: &AtNode) {
    unsafe { arnold_sys::ai_nodes::AiNodeSetPtr(node.raw(), param.into().into(), val.raw() as *mut c_void) };
}
