use super::{
    ai_array::AtArray,
    ai_color::{AtRGB, AtRGBA},
    ai_matrix::AtMatrix,
    ai_node_entry::AtNodeEntry,
    ai_params::AtUserParamEntry,
    ai_string::AtString,
    ai_universe::AtUniverse,
    ai_vector::{AtVector, AtVector2},
};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtNode {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtUserParamIterator {
    _unused: [u8; 0],
}
extern "C" {
    #[doc = " \\name AtNode Methods"]
    #[doc = " \\{"]
    pub fn AiNode(
        universe: *mut AtUniverse,
        nentry_name: AtString,
        name: AtString,
        parent: *const AtNode,
    ) -> *mut AtNode;
}
extern "C" {
    pub fn AiNodeLookUpByName(
        universe: *const AtUniverse,
        name: AtString,
        parent: *const AtNode,
    ) -> *mut AtNode;
}
extern "C" {
    pub fn AiNodeDeclare(
        node: *mut AtNode,
        param: AtString,
        declaration: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn AiNodeLookUpUserParameter(
        node: *const AtNode,
        param: AtString,
    ) -> *const AtUserParamEntry;
}
extern "C" {
    pub fn AiNodeIs(node: *const AtNode, str_: AtString) -> bool;
}
extern "C" {
    pub fn AiNodeReset(node: *mut AtNode);
}
extern "C" {
    pub fn AiNodeResetParameter(node: *mut AtNode, param: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn AiNodeClone(
        node: *const AtNode,
        new_name: AtString,
        parent: *const AtNode,
    ) -> *mut AtNode;
}
extern "C" {
    pub fn AiNodeDestroy(node: *mut AtNode) -> bool;
}
extern "C" {
    pub fn AiNodeReplace(old_node: *mut AtNode, new_node: *mut AtNode, remove: bool);
}
extern "C" {
    pub fn AiNodeLink(
        src: *mut AtNode,
        input: *const ::std::os::raw::c_char,
        target: *mut AtNode,
    ) -> bool;
}
extern "C" {
    pub fn AiNodeLinkOutput(
        src: *mut AtNode,
        output: *const ::std::os::raw::c_char,
        target: *mut AtNode,
        input: *const ::std::os::raw::c_char,
    ) -> bool;
}
extern "C" {
    pub fn AiNodeUnlink(node: *mut AtNode, input: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn AiNodeIsLinked(node: *const AtNode, input: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    pub fn AiNodeGetLink(
        node: *const AtNode,
        input: *const ::std::os::raw::c_char,
        comp: *mut ::std::os::raw::c_int,
    ) -> *mut AtNode;
}
extern "C" {
    pub fn AiNodeGetLinkOutput(
        node: *const AtNode,
        input: *const ::std::os::raw::c_char,
        output_param: *mut ::std::os::raw::c_int,
        output_comp: *mut ::std::os::raw::c_int,
    ) -> *mut AtNode;
}
extern "C" {
    pub fn AiNodeGetName(node: *const AtNode) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn AiNodeGetNodeEntry(node: *const AtNode) -> *const AtNodeEntry;
}
extern "C" {
    pub fn AiNodeGetLocalData(node: *const AtNode) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn AiNodeSetLocalData(node: *mut AtNode, data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn AiNodeGetPluginData(node: *const AtNode) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn AiNodeSetDisabled(node: *mut AtNode, disabled: bool);
}
extern "C" {
    pub fn AiNodeIsDisabled(node: *const AtNode) -> bool;
}
extern "C" {
    pub fn AiNodeGetParent(node: *const AtNode) -> *mut AtNode;
}
extern "C" {
    pub fn AiNodeGetUniverse(node: *const AtNode) -> *mut AtUniverse;
}
extern "C" {
    pub fn AiNodeGetUserParamIterator(node: *const AtNode) -> *mut AtUserParamIterator;
}
extern "C" {
    #[doc = " \\name AtUserParamIterator Methods"]
    #[doc = " \\{"]
    pub fn AiUserParamIteratorDestroy(iter: *mut AtUserParamIterator);
}
extern "C" {
    pub fn AiUserParamIteratorGetNext(iter: *mut AtUserParamIterator) -> *const AtUserParamEntry;
}
extern "C" {
    pub fn AiUserParamIteratorFinished(iter: *const AtUserParamIterator) -> bool;
}
extern "C" {
    #[doc = " \\name Parameter Writers"]
    #[doc = " \\{"]
    pub fn AiNodeSetByte(node: *mut AtNode, param: AtString, val: u8);
}
extern "C" {
    pub fn AiNodeSetInt(node: *mut AtNode, param: AtString, val: ::std::os::raw::c_int);
}
extern "C" {
    pub fn AiNodeSetUInt(node: *mut AtNode, param: AtString, val: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn AiNodeSetBool(node: *mut AtNode, param: AtString, val: bool);
}
extern "C" {
    pub fn AiNodeSetFlt(node: *mut AtNode, param: AtString, val: f32);
}
extern "C" {
    pub fn AiNodeSetPtr(node: *mut AtNode, param: AtString, val: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn AiNodeSetArray(node: *mut AtNode, param: AtString, val: *mut AtArray);
}
extern "C" {
    pub fn AiNodeSetMatrix(node: *mut AtNode, param: AtString, val: AtMatrix);
}
extern "C" {
    pub fn AiNodeSetStr(node: *mut AtNode, param: AtString, str_: AtString);
}
extern "C" {
    pub fn AiNodeSetRGB(node: *mut AtNode, param: AtString, r: f32, g: f32, b: f32);
}
extern "C" {
    pub fn AiNodeSetRGBA(node: *mut AtNode, param: AtString, r: f32, g: f32, b: f32, a: f32);
}
extern "C" {
    pub fn AiNodeSetVec(node: *mut AtNode, param: AtString, x: f32, y: f32, z: f32);
}
extern "C" {
    pub fn AiNodeSetVec2(node: *mut AtNode, param: AtString, x: f32, y: f32);
}
extern "C" {
    pub fn AiNodeSetAttributes(node: *mut AtNode, attributes: *const ::std::os::raw::c_char);
}
extern "C" {
    #[doc = " \\name Parameter Readers"]
    #[doc = " \\{"]
    pub fn AiNodeGetByte(node: *const AtNode, param: AtString) -> u8;
}
extern "C" {
    pub fn AiNodeGetInt(node: *const AtNode, param: AtString) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn AiNodeGetUInt(node: *const AtNode, param: AtString) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn AiNodeGetBool(node: *const AtNode, param: AtString) -> bool;
}
extern "C" {
    pub fn AiNodeGetFlt(node: *const AtNode, param: AtString) -> f32;
}
extern "C" {
    pub fn AiNodeGetRGB(node: *const AtNode, param: AtString) -> AtRGB;
}
extern "C" {
    pub fn AiNodeGetRGBA(node: *const AtNode, param: AtString) -> AtRGBA;
}
extern "C" {
    pub fn AiNodeGetVec(node: *const AtNode, param: AtString) -> AtVector;
}
extern "C" {
    pub fn AiNodeGetVec2(node: *const AtNode, param: AtString) -> AtVector2;
}
extern "C" {
    pub fn AiNodeGetStr(node: *const AtNode, param: AtString) -> AtString;
}
extern "C" {
    pub fn AiNodeGetPtr(node: *const AtNode, param: AtString) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn AiNodeGetArray(node: *const AtNode, param: AtString) -> *mut AtArray;
}
extern "C" {
    pub fn AiNodeGetMatrix(node: *const AtNode, param: AtString) -> AtMatrix;
}
