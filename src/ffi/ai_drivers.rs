use super::{
    ai_bbox::AtBBox2, ai_matrix::AtMatrix, ai_node_entry::AtNodeEntry, ai_nodes::AtNode,
    ai_string::AtString,
};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtOutputIterator {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtAOVSampleIterator {
    _unused: [u8; 0],
}
#[doc = " Driver Node methods structure"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtDriverNodeMethods {
    pub DriverSupportsPixelType:
        ::std::option::Option<unsafe extern "C" fn(arg1: *const AtNode, arg2: u8) -> bool>,
    pub DriverExtension:
        ::std::option::Option<unsafe extern "C" fn() -> *mut *const ::std::os::raw::c_char>,
    pub DriverOpen: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut AtNode,
            arg2: *mut AtOutputIterator,
            arg3: AtBBox2,
            arg4: AtBBox2,
            arg5: ::std::os::raw::c_int,
        ),
    >,
    pub DriverNeedsBucket: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut AtNode,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
            arg4: ::std::os::raw::c_int,
            arg5: ::std::os::raw::c_int,
            arg6: u16,
        ) -> bool,
    >,
    pub DriverPrepareBucket: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut AtNode,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
            arg4: ::std::os::raw::c_int,
            arg5: ::std::os::raw::c_int,
            arg6: u16,
        ),
    >,
    pub DriverProcessBucket: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut AtNode,
            arg2: *mut AtOutputIterator,
            arg3: *mut AtAOVSampleIterator,
            arg4: ::std::os::raw::c_int,
            arg5: ::std::os::raw::c_int,
            arg6: ::std::os::raw::c_int,
            arg7: ::std::os::raw::c_int,
            arg8: u16,
        ),
    >,
    pub DriverWriteBucket: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut AtNode,
            arg2: *mut AtOutputIterator,
            arg3: *mut AtAOVSampleIterator,
            arg4: ::std::os::raw::c_int,
            arg5: ::std::os::raw::c_int,
            arg6: ::std::os::raw::c_int,
            arg7: ::std::os::raw::c_int,
        ),
    >,
    pub DriverClose:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut AtNode, arg2: *mut AtOutputIterator)>,
}
extern "C" {
    #[doc = " \\name API for Driver Writers"]
    #[doc = " \\{"]
    pub fn AiDriverInitialize(node: *mut AtNode, supports_multiple_outputs: bool);
}
extern "C" {
    pub fn AiRawDriverInitialize(
        node: *mut AtNode,
        required_aovs: *mut *const ::std::os::raw::c_char,
        requires_depth: bool,
    );
}
extern "C" {
    pub fn AiDriverGetMatrices(world_to_camera: *mut AtMatrix, world_to_screen: *mut AtMatrix);
}
extern "C" {
    pub fn AiDriverExtension(node_entry: *const AtNodeEntry) -> *mut *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn AiOutputIteratorGetNext(
        iter: *mut AtOutputIterator,
        output_name: *mut AtString,
        pixel_type: *mut ::std::os::raw::c_int,
        bucket_data: *mut *const ::std::os::raw::c_void,
    ) -> bool;
}
extern "C" {
    pub fn AiOutputIteratorReset(iter: *mut AtOutputIterator);
}
extern "C" {
    pub fn AiOutputIteratorGetFilter(iter: *mut AtOutputIterator) -> *mut AtNode;
}
extern "C" {
    pub fn AiOutputIteratorIsHalf(iter: *mut AtOutputIterator) -> bool;
}
extern "C" {
    pub fn AiOutputIteratorGetLayerName(iter: *mut AtOutputIterator) -> AtString;
}
extern "C" {
    pub fn AiOutputIteratorGetCamera(iter: *mut AtOutputIterator) -> *mut AtNode;
}
extern "C" {
    pub fn AiFindDriverType(extension: *const ::std::os::raw::c_char) -> *const AtNodeEntry;
}
