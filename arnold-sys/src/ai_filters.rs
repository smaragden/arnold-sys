use ::std::{
    option::Option,
    os::raw::{c_char, c_int, c_uint, c_void},
};

use super::{
    ai_color::{AtRGB, AtRGBA},
    ai_matrix::AtMatrix,
    ai_nodes::AtNode,
    ai_string::AtString,
    ai_vector::{AtVector, AtVector2},
};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtAOVSampleIterator {
    _unused: [u8; 0],
}
/// Filter Node methods structure
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtFilterNodeMethods {
    pub FilterOutputType: Option<unsafe extern "C" fn(arg1: *const AtNode, arg2: u8) -> u8>,
    pub FilterPixel: Option<
        unsafe extern "C" fn(
            arg1: *mut AtNode,
            arg2: *mut AtAOVSampleIterator,
            arg3: *mut c_void,
            arg4: u8,
        ),
    >,
}
extern "C" {
    /// \\name API Methods for Filter Writers
    /// \\{
    pub fn AiFilterInitialize(
        node: *mut AtNode,
        requires_depth: bool,
        required_aovs: *mut *const c_char,
    );

    pub fn AiFilterUpdate(node: *mut AtNode, width: f32);

    /// \\name API Methods to Loop over Samples
    /// \\{
    pub fn AiAOVSampleIteratorInitPixel(iter: *mut AtAOVSampleIterator, x: c_int, y: c_int);

    pub fn AiAOVSampleIteratorGetPixel(
        iter: *mut AtAOVSampleIterator,
        x: *mut c_int,
        y: *mut c_int,
    );

    pub fn AiAOVSampleIteratorReset(iter: *mut AtAOVSampleIterator);

    pub fn AiAOVSampleIteratorGetNext(iter: *mut AtAOVSampleIterator) -> bool;

    pub fn AiAOVSampleIteratorGetNextDepth(iter: *mut AtAOVSampleIterator) -> bool;

    pub fn AiAOVSampleIteratorGetOffset(iter: *const AtAOVSampleIterator) -> AtVector2;

    pub fn AiAOVSampleIteratorGetInvDensity(iter: *const AtAOVSampleIterator) -> f32;

    pub fn AiAOVSampleIteratorGetDepth(iter: *const AtAOVSampleIterator) -> c_int;

    pub fn AiAOVSampleIteratorHasValue(iter: *const AtAOVSampleIterator) -> bool;

    pub fn AiAOVSampleIteratorHasAOVValue(
        iter: *const AtAOVSampleIterator,
        name: AtString,
        type_: u8,
    ) -> bool;

    pub fn AiAOVSampleIteratorGetAOVName(iter: *const AtAOVSampleIterator) -> AtString;

    /// \\name API Methods to Get Sample Value from Iterator
    /// \\{
    pub fn AiAOVSampleIteratorGetBool(iter: *const AtAOVSampleIterator) -> bool;

    pub fn AiAOVSampleIteratorGetInt(iter: *const AtAOVSampleIterator) -> c_int;

    pub fn AiAOVSampleIteratorGetUInt(iter: *const AtAOVSampleIterator) -> c_uint;

    pub fn AiAOVSampleIteratorGetFlt(iter: *const AtAOVSampleIterator) -> f32;

    pub fn AiAOVSampleIteratorGetRGB(iter: *const AtAOVSampleIterator) -> AtRGB;

    pub fn AiAOVSampleIteratorGetRGBA(iter: *const AtAOVSampleIterator) -> AtRGBA;

    pub fn AiAOVSampleIteratorGetVec(iter: *const AtAOVSampleIterator) -> AtVector;

    pub fn AiAOVSampleIteratorGetVec2(iter: *const AtAOVSampleIterator) -> AtVector2;

    pub fn AiAOVSampleIteratorGetMatrix(iter: *const AtAOVSampleIterator) -> AtMatrix;

    pub fn AiAOVSampleIteratorGetPtr(iter: *const AtAOVSampleIterator) -> *const c_void;

    /// \\name API Methods to Get Sample Value from Iterator for an Arbitrary AOV
    /// \\{
    pub fn AiAOVSampleIteratorGetAOVBool(iter: *const AtAOVSampleIterator, name: AtString) -> bool;

    pub fn AiAOVSampleIteratorGetAOVInt(iter: *const AtAOVSampleIterator, name: AtString) -> c_int;

    pub fn AiAOVSampleIteratorGetAOVUInt(
        iter: *const AtAOVSampleIterator,
        name: AtString,
    ) -> c_uint;

    pub fn AiAOVSampleIteratorGetAOVFlt(iter: *const AtAOVSampleIterator, name: AtString) -> f32;

    pub fn AiAOVSampleIteratorGetAOVRGB(iter: *const AtAOVSampleIterator, name: AtString) -> AtRGB;

    pub fn AiAOVSampleIteratorGetAOVRGBA(
        iter: *const AtAOVSampleIterator,
        name: AtString,
    ) -> AtRGBA;

    pub fn AiAOVSampleIteratorGetAOVVec(
        iter: *const AtAOVSampleIterator,
        name: AtString,
    ) -> AtVector;

    pub fn AiAOVSampleIteratorGetAOVVec2(
        iter: *const AtAOVSampleIterator,
        name: AtString,
    ) -> AtVector2;

    pub fn AiAOVSampleIteratorGetAOVMatrix(
        iter: *const AtAOVSampleIterator,
        name: AtString,
    ) -> AtMatrix;

    pub fn AiAOVSampleIteratorGetAOVPtr(
        iter: *const AtAOVSampleIterator,
        name: AtString,
    ) -> *const c_void;
}
