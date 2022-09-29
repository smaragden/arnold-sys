use ::std::{
    option::Option,
    os::raw::{c_char, c_int, c_void},
};

use super::{
    ai_nodes::AtNode,
    ai_params::AtList,
    ai_params::{AtParamEntry, AtParamValue},
    ai_render::AtRenderSession,
    ai_string::AtString,
};

pub const AI_NODE_UNDEFINED: u32 = 0;
pub const AI_NODE_OPTIONS: u32 = 1;
pub const AI_NODE_CAMERA: u32 = 2;
pub const AI_NODE_LIGHT: u32 = 4;
pub const AI_NODE_SHAPE: u32 = 8;
pub const AI_NODE_SHADER: u32 = 16;
pub const AI_NODE_OVERRIDE: u32 = 32;
pub const AI_NODE_DRIVER: u32 = 64;
pub const AI_NODE_FILTER: u32 = 128;
pub const AI_NODE_COLOR_MANAGER: u32 = 2048;
pub const AI_NODE_OPERATOR: u32 = 4096;
pub const AI_NODE_ALL: u32 = 65535;
pub const AI_NODE_SHAPE_PROCEDURAL: u32 = 256;
pub const AI_NODE_SHAPE_VOLUME: u32 = 512;
pub const AI_NODE_SHAPE_IMPLICIT: u32 = 1024;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtParamIterator {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtMetaDataIterator {
    _unused: [u8; 0],
}
/// \\struct AtNodeEntry
///
/// This represents a node type in Arnold. There is a node entry for each
/// installed node, whether built-in or plug-in. The actual contents of this
/// struct are private.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtNodeEntry {
    _unused: [u8; 0],
}
/// Methods common to all nodes
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtCommonMethods {
    pub PluginInitialize: Option<unsafe extern "C" fn(arg1: *mut *mut c_void) -> bool>,
    pub PluginCleanup: Option<unsafe extern "C" fn(arg1: *mut c_void)>,
    pub Parameters: Option<unsafe extern "C" fn(arg1: *mut AtList, arg2: *mut AtNodeEntry)>,
    pub Initialize: Option<unsafe extern "C" fn(arg1: *mut AtRenderSession, arg2: *mut AtNode)>,
    pub Update: Option<unsafe extern "C" fn(arg1: *mut AtRenderSession, arg2: *mut AtNode)>,
    pub Finish: Option<unsafe extern "C" fn(arg1: *mut AtNode)>,
}
/// Node methods
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtNodeMethods {
    /// common methods
    pub cmethods: *const AtCommonMethods,
    /// derived methods
    pub dmethods: *const c_void,
}
#[repr(C)]
pub struct AtMetaDataEntry {
    pub name: AtString,
    pub param: AtString,
    pub type_: u8,
    pub value: AtParamValue,
}
extern "C" {
    /// \\name AtNodeEntry Methods
    /// \\{
    pub fn AiNodeEntryLookUp(name: AtString) -> *const AtNodeEntry;

    pub fn AiNodeEntryGetName(nentry: *const AtNodeEntry) -> *const c_char;

    pub fn AiNodeEntryGetNameAtString(nentry: *const AtNodeEntry) -> AtString;

    pub fn AiNodeEntryGetType(nentry: *const AtNodeEntry) -> c_int;

    pub fn AiNodeEntryGetTypeName(nentry: *const AtNodeEntry) -> *const c_char;

    pub fn AiNodeEntryGetDerivedType(nentry: *const AtNodeEntry) -> c_int;

    pub fn AiNodeEntryGetDerivedTypeName(nentry: *const AtNodeEntry) -> *const c_char;

    pub fn AiNodeEntryGetOutputType(nentry: *const AtNodeEntry) -> c_int;

    pub fn AiNodeEntryGetFilename(nentry: *const AtNodeEntry) -> *const c_char;

    pub fn AiNodeEntryGetVersion(nentry: *const AtNodeEntry) -> *const c_char;

    pub fn AiNodeEntryGetCount(nentry: *const AtNodeEntry) -> c_int;

    pub fn AiNodeEntryGetNumParams(nentry: *const AtNodeEntry) -> c_int;

    pub fn AiNodeEntryGetParameter(nentry: *const AtNodeEntry, i: c_int) -> *const AtParamEntry;

    pub fn AiNodeEntryLookUpParameter(
        nentry: *const AtNodeEntry,
        param: AtString,
    ) -> *const AtParamEntry;

    pub fn AiNodeEntryGetNumOutputs(nentry: *const AtNodeEntry) -> c_int;

    pub fn AiNodeEntryGetOutput(nentry: *const AtNodeEntry, i: c_int) -> *const AtParamEntry;

    pub fn AiNodeEntryLookUpOutput(
        nentry: *const AtNodeEntry,
        param: AtString,
    ) -> *const AtParamEntry;

    pub fn AiNodeEntryGetParamIterator(nentry: *const AtNodeEntry) -> *mut AtParamIterator;

    pub fn AiNodeEntryGetMetaDataIterator(
        nentry: *const AtNodeEntry,
        param: *const c_char,
    ) -> *mut AtMetaDataIterator;

    pub fn AiNodeEntryInstall(
        type_: c_int,
        output_type: u8,
        name: *const c_char,
        filename: *const c_char,
        methods: *const AtNodeMethods,
        version: *const c_char,
    );

    pub fn AiNodeEntryUninstall(name: *const c_char);

    /// \\name AtParamIterator Methods
    /// \\{
    pub fn AiParamIteratorDestroy(iter: *mut AtParamIterator);

    pub fn AiParamIteratorGetNext(iter: *mut AtParamIterator) -> *const AtParamEntry;

    pub fn AiParamIteratorFinished(iter: *const AtParamIterator) -> bool;

    /// \\name AtMetaDataIterator Methods
    /// \\{
    pub fn AiMetaDataIteratorDestroy(iter: *mut AtMetaDataIterator);

    pub fn AiMetaDataIteratorGetNext(iter: *mut AtMetaDataIterator) -> *const AtMetaDataEntry;

    pub fn AiMetaDataIteratorFinished(iter: *const AtMetaDataIterator) -> bool;
}
