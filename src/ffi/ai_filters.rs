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
#[doc = " Filter Node methods structure"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtFilterNodeMethods {
    pub FilterOutputType:
        ::std::option::Option<unsafe extern "C" fn(arg1: *const AtNode, arg2: u8) -> u8>,
    pub FilterPixel: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut AtNode,
            arg2: *mut AtAOVSampleIterator,
            arg3: *mut ::std::os::raw::c_void,
            arg4: u8,
        ),
    >,
}
extern "C" {
    #[doc = " \\name API Methods for Filter Writers"]
    #[doc = " \\{"]
    pub fn AiFilterInitialize(
        node: *mut AtNode,
        requires_depth: bool,
        required_aovs: *mut *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn AiFilterUpdate(node: *mut AtNode, width: f32);
}
extern "C" {
    #[doc = " \\name API Methods to Loop over Samples"]
    #[doc = " \\{"]
    pub fn AiAOVSampleIteratorInitPixel(
        iter: *mut AtAOVSampleIterator,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn AiAOVSampleIteratorGetPixel(
        iter: *mut AtAOVSampleIterator,
        x: *mut ::std::os::raw::c_int,
        y: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn AiAOVSampleIteratorReset(iter: *mut AtAOVSampleIterator);
}
extern "C" {
    pub fn AiAOVSampleIteratorGetNext(iter: *mut AtAOVSampleIterator) -> bool;
}
extern "C" {
    pub fn AiAOVSampleIteratorGetNextDepth(iter: *mut AtAOVSampleIterator) -> bool;
}
extern "C" {
    pub fn AiAOVSampleIteratorGetOffset(iter: *const AtAOVSampleIterator) -> AtVector2;
}
extern "C" {
    pub fn AiAOVSampleIteratorGetInvDensity(iter: *const AtAOVSampleIterator) -> f32;
}
extern "C" {
    pub fn AiAOVSampleIteratorGetDepth(iter: *const AtAOVSampleIterator) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn AiAOVSampleIteratorHasValue(iter: *const AtAOVSampleIterator) -> bool;
}
extern "C" {
    pub fn AiAOVSampleIteratorHasAOVValue(
        iter: *const AtAOVSampleIterator,
        name: AtString,
        type_: u8,
    ) -> bool;
}
extern "C" {
    pub fn AiAOVSampleIteratorGetAOVName(iter: *const AtAOVSampleIterator) -> AtString;
}
extern "C" {
    #[doc = " \\name API Methods to Get Sample Value from Iterator"]
    #[doc = " \\{"]
    pub fn AiAOVSampleIteratorGetBool(iter: *const AtAOVSampleIterator) -> bool;
}
extern "C" {
    pub fn AiAOVSampleIteratorGetInt(iter: *const AtAOVSampleIterator) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn AiAOVSampleIteratorGetUInt(iter: *const AtAOVSampleIterator) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn AiAOVSampleIteratorGetFlt(iter: *const AtAOVSampleIterator) -> f32;
}
extern "C" {
    pub fn AiAOVSampleIteratorGetRGB(iter: *const AtAOVSampleIterator) -> AtRGB;
}
extern "C" {
    pub fn AiAOVSampleIteratorGetRGBA(iter: *const AtAOVSampleIterator) -> AtRGBA;
}
extern "C" {
    pub fn AiAOVSampleIteratorGetVec(iter: *const AtAOVSampleIterator) -> AtVector;
}
extern "C" {
    pub fn AiAOVSampleIteratorGetVec2(iter: *const AtAOVSampleIterator) -> AtVector2;
}
extern "C" {
    pub fn AiAOVSampleIteratorGetMatrix(iter: *const AtAOVSampleIterator) -> AtMatrix;
}
extern "C" {
    pub fn AiAOVSampleIteratorGetPtr(
        iter: *const AtAOVSampleIterator,
    ) -> *const ::std::os::raw::c_void;
}
extern "C" {
    #[doc = " \\name API Methods to Get Sample Value from Iterator for an Arbitrary AOV"]
    #[doc = " \\{"]
    pub fn AiAOVSampleIteratorGetAOVBool(iter: *const AtAOVSampleIterator, name: AtString) -> bool;
}
extern "C" {
    pub fn AiAOVSampleIteratorGetAOVInt(
        iter: *const AtAOVSampleIterator,
        name: AtString,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn AiAOVSampleIteratorGetAOVUInt(
        iter: *const AtAOVSampleIterator,
        name: AtString,
    ) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn AiAOVSampleIteratorGetAOVFlt(iter: *const AtAOVSampleIterator, name: AtString) -> f32;
}
extern "C" {
    pub fn AiAOVSampleIteratorGetAOVRGB(iter: *const AtAOVSampleIterator, name: AtString) -> AtRGB;
}
extern "C" {
    pub fn AiAOVSampleIteratorGetAOVRGBA(
        iter: *const AtAOVSampleIterator,
        name: AtString,
    ) -> AtRGBA;
}
extern "C" {
    pub fn AiAOVSampleIteratorGetAOVVec(
        iter: *const AtAOVSampleIterator,
        name: AtString,
    ) -> AtVector;
}
extern "C" {
    pub fn AiAOVSampleIteratorGetAOVVec2(
        iter: *const AtAOVSampleIterator,
        name: AtString,
    ) -> AtVector2;
}
extern "C" {
    pub fn AiAOVSampleIteratorGetAOVMatrix(
        iter: *const AtAOVSampleIterator,
        name: AtString,
    ) -> AtMatrix;
}
extern "C" {
    pub fn AiAOVSampleIteratorGetAOVPtr(
        iter: *const AtAOVSampleIterator,
        name: AtString,
    ) -> *const ::std::os::raw::c_void;
}
