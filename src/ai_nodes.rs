use ::std::os::raw::{c_char, c_int, c_uint, c_void};

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
    /// \\name AtNode Methods
    /// \\{
    pub fn AiNode(
        universe: *mut AtUniverse,
        nentry_name: AtString,
        name: AtString,
        parent: *const AtNode,
    ) -> *mut AtNode;

    pub fn AiNodeLookUpByName(
        universe: *const AtUniverse,
        name: AtString,
        parent: *const AtNode,
    ) -> *mut AtNode;

    pub fn AiNodeDeclare(node: *mut AtNode, param: AtString, declaration: *const c_char) -> bool;

    pub fn AiNodeLookUpUserParameter(
        node: *const AtNode,
        param: AtString,
    ) -> *const AtUserParamEntry;

    pub fn AiNodeIs(node: *const AtNode, str_: AtString) -> bool;

    pub fn AiNodeReset(node: *mut AtNode);

    pub fn AiNodeResetParameter(node: *mut AtNode, param: *const c_char);

    pub fn AiNodeClone(
        node: *const AtNode,
        new_name: AtString,
        parent: *const AtNode,
    ) -> *mut AtNode;

    pub fn AiNodeDestroy(node: *mut AtNode) -> bool;

    pub fn AiNodeReplace(old_node: *mut AtNode, new_node: *mut AtNode, remove: bool);

    pub fn AiNodeLink(src: *mut AtNode, input: *const c_char, target: *mut AtNode) -> bool;

    pub fn AiNodeLinkOutput(
        src: *mut AtNode,
        output: *const c_char,
        target: *mut AtNode,
        input: *const c_char,
    ) -> bool;

    pub fn AiNodeUnlink(node: *mut AtNode, input: *const c_char) -> bool;

    pub fn AiNodeIsLinked(node: *const AtNode, input: *const c_char) -> bool;

    pub fn AiNodeGetLink(
        node: *const AtNode,
        input: *const c_char,
        comp: *mut c_int,
    ) -> *mut AtNode;

    pub fn AiNodeGetLinkOutput(
        node: *const AtNode,
        input: *const c_char,
        output_param: *mut c_int,
        output_comp: *mut c_int,
    ) -> *mut AtNode;

    pub fn AiNodeGetName(node: *const AtNode) -> *const c_char;

    pub fn AiNodeGetNodeEntry(node: *const AtNode) -> *const AtNodeEntry;

    pub fn AiNodeGetLocalData(node: *const AtNode) -> *mut c_void;

    pub fn AiNodeSetLocalData(node: *mut AtNode, data: *mut c_void);

    pub fn AiNodeGetPluginData(node: *const AtNode) -> *mut c_void;

    pub fn AiNodeSetDisabled(node: *mut AtNode, disabled: bool);

    pub fn AiNodeIsDisabled(node: *const AtNode) -> bool;

    pub fn AiNodeGetParent(node: *const AtNode) -> *mut AtNode;

    pub fn AiNodeGetUniverse(node: *const AtNode) -> *mut AtUniverse;

    pub fn AiNodeGetUserParamIterator(node: *const AtNode) -> *mut AtUserParamIterator;

    /// \\name AtUserParamIterator Methods
    /// \\{
    pub fn AiUserParamIteratorDestroy(iter: *mut AtUserParamIterator);

    pub fn AiUserParamIteratorGetNext(iter: *mut AtUserParamIterator) -> *const AtUserParamEntry;

    pub fn AiUserParamIteratorFinished(iter: *const AtUserParamIterator) -> bool;

    /// \\name Parameter Writers
    /// \\{
    pub fn AiNodeSetByte(node: *mut AtNode, param: AtString, val: u8);

    pub fn AiNodeSetInt(node: *mut AtNode, param: AtString, val: c_int);

    pub fn AiNodeSetUInt(node: *mut AtNode, param: AtString, val: c_uint);

    pub fn AiNodeSetBool(node: *mut AtNode, param: AtString, val: bool);

    pub fn AiNodeSetFlt(node: *mut AtNode, param: AtString, val: f32);

    pub fn AiNodeSetPtr(node: *mut AtNode, param: AtString, val: *mut c_void);

    pub fn AiNodeSetArray(node: *mut AtNode, param: AtString, val: *mut AtArray);

    pub fn AiNodeSetMatrix(node: *mut AtNode, param: AtString, val: AtMatrix);

    pub fn AiNodeSetStr(node: *mut AtNode, param: AtString, str_: AtString);

    pub fn AiNodeSetRGB(node: *mut AtNode, param: AtString, r: f32, g: f32, b: f32);

    pub fn AiNodeSetRGBA(node: *mut AtNode, param: AtString, r: f32, g: f32, b: f32, a: f32);

    pub fn AiNodeSetVec(node: *mut AtNode, param: AtString, x: f32, y: f32, z: f32);

    pub fn AiNodeSetVec2(node: *mut AtNode, param: AtString, x: f32, y: f32);

    pub fn AiNodeSetAttributes(node: *mut AtNode, attributes: *const c_char);

    /// \\name Parameter Readers
    /// \\{
    pub fn AiNodeGetByte(node: *const AtNode, param: AtString) -> u8;

    pub fn AiNodeGetInt(node: *const AtNode, param: AtString) -> c_int;

    pub fn AiNodeGetUInt(node: *const AtNode, param: AtString) -> c_uint;

    pub fn AiNodeGetBool(node: *const AtNode, param: AtString) -> bool;

    pub fn AiNodeGetFlt(node: *const AtNode, param: AtString) -> f32;

    pub fn AiNodeGetRGB(node: *const AtNode, param: AtString) -> AtRGB;

    pub fn AiNodeGetRGBA(node: *const AtNode, param: AtString) -> AtRGBA;

    pub fn AiNodeGetVec(node: *const AtNode, param: AtString) -> AtVector;

    pub fn AiNodeGetVec2(node: *const AtNode, param: AtString) -> AtVector2;

    pub fn AiNodeGetStr(node: *const AtNode, param: AtString) -> AtString;

    pub fn AiNodeGetPtr(node: *const AtNode, param: AtString) -> *mut c_void;

    pub fn AiNodeGetArray(node: *const AtNode, param: AtString) -> *mut AtArray;

    pub fn AiNodeGetMatrix(node: *const AtNode, param: AtString) -> AtMatrix;
}
