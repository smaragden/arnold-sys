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
    /// \\name AtParamValueMap Methods
    /// \\{
    pub fn AiParamValueMap() -> *mut AtParamValueMap;

    pub fn AiParamValueMapDestroy(map: *mut AtParamValueMap);

    pub fn AiParamValueMapGetIterator(map: *const AtParamValueMap) -> *mut AtParamValueMapIterator;

    pub fn AiParamValueMapMerge(target_map: *mut AtParamValueMap, src_map: *const AtParamValueMap);

    pub fn AiParamValueMapClone(src_map: *const AtParamValueMap) -> *mut AtParamValueMap;

    pub fn AiParamValueMapSetBool(map: *mut AtParamValueMap, name: AtString, value: bool);

    pub fn AiParamValueMapSetInt(
        map: *mut AtParamValueMap,
        name: AtString,
        value: ::std::os::raw::c_int,
    );

    pub fn AiParamValueMapSetFlt(map: *mut AtParamValueMap, name: AtString, value: f32);

    pub fn AiParamValueMapSetRGB(map: *mut AtParamValueMap, name: AtString, value: AtRGB);

    pub fn AiParamValueMapSetVec(map: *mut AtParamValueMap, name: AtString, value: AtVector);

    pub fn AiParamValueMapSetVec2(map: *mut AtParamValueMap, name: AtString, value: AtVector2);

    pub fn AiParamValueMapSetStr(map: *mut AtParamValueMap, name: AtString, value: AtString);

    pub fn AiParamValueMapSetArray(map: *mut AtParamValueMap, name: AtString, value: *mut AtArray);

    pub fn AiParamValueMapSetPtr(
        map: *mut AtParamValueMap,
        name: AtString,
        value: *mut ::std::os::raw::c_void,
    );

    pub fn AiParamValueMapGetBool(
        map: *const AtParamValueMap,
        name: AtString,
        value: *mut bool,
    ) -> bool;

    pub fn AiParamValueMapGetInt(
        map: *const AtParamValueMap,
        name: AtString,
        value: *mut ::std::os::raw::c_int,
    ) -> bool;

    pub fn AiParamValueMapGetFlt(
        map: *const AtParamValueMap,
        name: AtString,
        value: *mut f32,
    ) -> bool;

    pub fn AiParamValueMapGetRGB(
        map: *const AtParamValueMap,
        name: AtString,
        value: *mut AtRGB,
    ) -> bool;

    pub fn AiParamValueMapGetVec(
        map: *const AtParamValueMap,
        name: AtString,
        value: *mut AtVector,
    ) -> bool;

    pub fn AiParamValueMapGetVec2(
        map: *const AtParamValueMap,
        name: AtString,
        value: *mut AtVector2,
    ) -> bool;

    pub fn AiParamValueMapGetStr(
        map: *const AtParamValueMap,
        name: AtString,
        value: *mut AtString,
    ) -> bool;

    pub fn AiParamValueMapGetArray(
        map: *const AtParamValueMap,
        name: AtString,
        value: *mut *mut AtArray,
    ) -> bool;

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
    /// \\name AtParamValueMap Methods
    /// \\{
    pub fn AiParamValueMapIteratorDestroy(iter: *mut AtParamValueMapIterator);

    pub fn AiParamValueMapIteratorGetNext(
        iter: *mut AtParamValueMapIterator,
    ) -> *const AtParamValueMapEntry;

    pub fn AiParamValueMapIteratorFinished(iter: *const AtParamValueMapIterator) -> bool;
}
