use ::std::os::raw::{c_int, c_uint};

use super::{
    ai_bbox::AtBBox, ai_map::AtParamValueMap, ai_node_entry::AtNodeEntry, ai_nodes::AtNode,
    ai_render::AtRenderSession, ai_string::AtString,
};

pub const AI_CACHE_TEXTURE: u32 = 1;
pub const AI_CACHE_BACKGROUND: u32 = 2;
pub const AI_CACHE_QUAD: u32 = 4;
pub const AI_CACHE_ALL: u32 = 65535;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtNodeIterator {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtNodeEntryIterator {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtAOVIterator {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct AtAOVEntry {
    /// AOV name
    pub name: AtString,
    /// Data type
    pub type_: u8,
    /// Blend mode
    pub blend_mode: c_int,
    /// Optional light path expression
    pub expression: AtString,
}
/// \\struct AtUniverse
///
/// This represents a universe in Arnold.
///
/// \\see AiUniverse
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtUniverse {
    _unused: [u8; 0],
}
extern "C" {
    /// \\name Methods
    /// \\{
    pub fn AiUniverse() -> *mut AtUniverse;

    pub fn AiUniverseDestroy(universe: *mut AtUniverse);

    pub fn AiUniverseCacheFlush(universe: *const AtUniverse, cache_flags: c_int) -> bool;

    pub fn AiUniverseGetOptions(universe: *const AtUniverse) -> *mut AtNode;

    pub fn AiUniverseGetCamera(universe: *const AtUniverse) -> *mut AtNode;

    pub fn AiUniverseGetSceneBounds(universe: *const AtUniverse) -> AtBBox;

    pub fn AiUniverseGetNodeIterator(
        universe: *const AtUniverse,
        node_mask: c_uint,
    ) -> *mut AtNodeIterator;

    pub fn AiUniverseGetNodeEntryIterator(node_mask: c_uint) -> *mut AtNodeEntryIterator;

    pub fn AiUniverseGetAOVIterator(universe: *const AtUniverse) -> *mut AtAOVIterator;

    pub fn AiUniverseAddDefaultNodes(universe: *mut AtUniverse, params: *const AtParamValueMap);

    pub fn AiUniverseGetRenderSession(universe: *const AtUniverse) -> *mut AtRenderSession;

    pub fn AiUniverseIsActive() -> bool;

    /// \\name Node Iterator API
    /// \\{
    pub fn AiNodeIteratorDestroy(iter: *mut AtNodeIterator);

    pub fn AiNodeIteratorGetNext(iter: *mut AtNodeIterator) -> *mut AtNode;

    pub fn AiNodeIteratorFinished(iter: *const AtNodeIterator) -> bool;

    /// \\name Node Entry Iterator API
    /// \\{
    pub fn AiNodeEntryIteratorDestroy(iter: *mut AtNodeEntryIterator);

    pub fn AiNodeEntryIteratorGetNext(iter: *mut AtNodeEntryIterator) -> *mut AtNodeEntry;

    pub fn AiNodeEntryIteratorFinished(iter: *const AtNodeEntryIterator) -> bool;

    /// \\name AOV Iterator API
    /// \\{
    pub fn AiAOVIteratorDestroy(iter: *mut AtAOVIterator);

    pub fn AiAOVIteratorGetNext(iter: *mut AtAOVIterator) -> *const AtAOVEntry;

    pub fn AiAOVIteratorFinished(iter: *const AtAOVIterator) -> bool;
}
