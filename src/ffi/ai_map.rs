use super::{
    ai_array::AtArray,
    ai_color::AtRGB,
    ai_params::AtParamValue,
    ai_string::AtString,
    ai_vector::{AtVector, AtVector2},
};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtParamValueMap {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtParamValueMapIterator {
    _unused: [u8; 0],
}
extern "C" {
    #[doc = " \\name AtParamValueMap Methods"]
    #[doc = " \\{"]
    pub fn AiParamValueMap() -> *mut AtParamValueMap;
}
extern "C" {
    pub fn AiParamValueMapDestroy(map: *mut AtParamValueMap);
}
extern "C" {
    pub fn AiParamValueMapGetIterator(map: *const AtParamValueMap) -> *mut AtParamValueMapIterator;
}
extern "C" {
    pub fn AiParamValueMapMerge(target_map: *mut AtParamValueMap, src_map: *const AtParamValueMap);
}
extern "C" {
    pub fn AiParamValueMapClone(src_map: *const AtParamValueMap) -> *mut AtParamValueMap;
}
extern "C" {
    pub fn AiParamValueMapSetBool(map: *mut AtParamValueMap, name: AtString, value: bool);
}
extern "C" {
    pub fn AiParamValueMapSetInt(
        map: *mut AtParamValueMap,
        name: AtString,
        value: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn AiParamValueMapSetFlt(map: *mut AtParamValueMap, name: AtString, value: f32);
}
extern "C" {
    pub fn AiParamValueMapSetRGB(map: *mut AtParamValueMap, name: AtString, value: AtRGB);
}
extern "C" {
    pub fn AiParamValueMapSetVec(map: *mut AtParamValueMap, name: AtString, value: AtVector);
}
extern "C" {
    pub fn AiParamValueMapSetVec2(map: *mut AtParamValueMap, name: AtString, value: AtVector2);
}
extern "C" {
    pub fn AiParamValueMapSetStr(map: *mut AtParamValueMap, name: AtString, value: AtString);
}
extern "C" {
    pub fn AiParamValueMapSetArray(map: *mut AtParamValueMap, name: AtString, value: *mut AtArray);
}
extern "C" {
    pub fn AiParamValueMapSetPtr(
        map: *mut AtParamValueMap,
        name: AtString,
        value: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn AiParamValueMapGetBool(
        map: *const AtParamValueMap,
        name: AtString,
        value: *mut bool,
    ) -> bool;
}
extern "C" {
    pub fn AiParamValueMapGetInt(
        map: *const AtParamValueMap,
        name: AtString,
        value: *mut ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn AiParamValueMapGetFlt(
        map: *const AtParamValueMap,
        name: AtString,
        value: *mut f32,
    ) -> bool;
}
extern "C" {
    pub fn AiParamValueMapGetRGB(
        map: *const AtParamValueMap,
        name: AtString,
        value: *mut AtRGB,
    ) -> bool;
}
extern "C" {
    pub fn AiParamValueMapGetVec(
        map: *const AtParamValueMap,
        name: AtString,
        value: *mut AtVector,
    ) -> bool;
}
extern "C" {
    pub fn AiParamValueMapGetVec2(
        map: *const AtParamValueMap,
        name: AtString,
        value: *mut AtVector2,
    ) -> bool;
}
extern "C" {
    pub fn AiParamValueMapGetStr(
        map: *const AtParamValueMap,
        name: AtString,
        value: *mut AtString,
    ) -> bool;
}
extern "C" {
    pub fn AiParamValueMapGetArray(
        map: *const AtParamValueMap,
        name: AtString,
        value: *mut *mut AtArray,
    ) -> bool;
}
extern "C" {
    pub fn AiParamValueMapGetPtr(
        map: *const AtParamValueMap,
        name: AtString,
        value: *mut *mut ::std::os::raw::c_void,
    ) -> bool;
}
#[repr(C)]
pub struct AtParamValueMapEntry {
    pub name: AtString,
    pub type_: u8,
    pub value: AtParamValue,
}
extern "C" {
    #[doc = " \\name AtParamValueMap Methods"]
    #[doc = " \\{"]
    pub fn AiParamValueMapIteratorDestroy(iter: *mut AtParamValueMapIterator);
}
extern "C" {
    pub fn AiParamValueMapIteratorGetNext(
        iter: *mut AtParamValueMapIterator,
    ) -> *const AtParamValueMapEntry;
}
extern "C" {
    pub fn AiParamValueMapIteratorFinished(iter: *const AtParamValueMapIterator) -> bool;
}
