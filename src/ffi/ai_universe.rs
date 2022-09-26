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
    #[doc = "< AOV name"]
    pub name: AtString,
    #[doc = "< Data type"]
    pub type_: u8,
    #[doc = "< Blend mode"]
    pub blend_mode: ::std::os::raw::c_int,
    #[doc = "< Optional light path expression"]
    pub expression: AtString,
}
#[doc = " \\struct AtUniverse"]
#[doc = ""]
#[doc = " This represents a universe in Arnold."]
#[doc = ""]
#[doc = " \\see AiUniverse"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtUniverse {
    _unused: [u8; 0],
}
extern "C" {
    #[doc = " \\name Methods"]
    #[doc = " \\{"]
    pub fn AiUniverse() -> *mut AtUniverse;
}
extern "C" {
    pub fn AiUniverseDestroy(universe: *mut AtUniverse);
}
extern "C" {
    pub fn AiUniverseCacheFlush(
        universe: *const AtUniverse,
        cache_flags: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn AiUniverseGetOptions(universe: *const AtUniverse) -> *mut AtNode;
}
extern "C" {
    pub fn AiUniverseGetCamera(universe: *const AtUniverse) -> *mut AtNode;
}
extern "C" {
    pub fn AiUniverseGetSceneBounds(universe: *const AtUniverse) -> AtBBox;
}
extern "C" {
    pub fn AiUniverseGetNodeIterator(
        universe: *const AtUniverse,
        node_mask: ::std::os::raw::c_uint,
    ) -> *mut AtNodeIterator;
}
extern "C" {
    pub fn AiUniverseGetNodeEntryIterator(
        node_mask: ::std::os::raw::c_uint,
    ) -> *mut AtNodeEntryIterator;
}
extern "C" {
    pub fn AiUniverseGetAOVIterator(universe: *const AtUniverse) -> *mut AtAOVIterator;
}
extern "C" {
    pub fn AiUniverseAddDefaultNodes(universe: *mut AtUniverse, params: *const AtParamValueMap);
}
extern "C" {
    pub fn AiUniverseGetRenderSession(universe: *const AtUniverse) -> *mut AtRenderSession;
}
extern "C" {
    pub fn AiUniverseIsActive() -> bool;
}
extern "C" {
    #[doc = " \\name Node Iterator API"]
    #[doc = " \\{"]
    pub fn AiNodeIteratorDestroy(iter: *mut AtNodeIterator);
}
extern "C" {
    pub fn AiNodeIteratorGetNext(iter: *mut AtNodeIterator) -> *mut AtNode;
}
extern "C" {
    pub fn AiNodeIteratorFinished(iter: *const AtNodeIterator) -> bool;
}
extern "C" {
    #[doc = " \\name Node Entry Iterator API"]
    #[doc = " \\{"]
    pub fn AiNodeEntryIteratorDestroy(iter: *mut AtNodeEntryIterator);
}
extern "C" {
    pub fn AiNodeEntryIteratorGetNext(iter: *mut AtNodeEntryIterator) -> *mut AtNodeEntry;
}
extern "C" {
    pub fn AiNodeEntryIteratorFinished(iter: *const AtNodeEntryIterator) -> bool;
}
extern "C" {
    #[doc = " \\name AOV Iterator API"]
    #[doc = " \\{"]
    pub fn AiAOVIteratorDestroy(iter: *mut AtAOVIterator);
}
extern "C" {
    pub fn AiAOVIteratorGetNext(iter: *mut AtAOVIterator) -> *const AtAOVEntry;
}
extern "C" {
    pub fn AiAOVIteratorFinished(iter: *const AtAOVIterator) -> bool;
}
