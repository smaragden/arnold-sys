use super::{
    ai_array::AtArray, ai_map::AtParamValueMap, ai_nodes::AtNode, ai_universe::AtUniverse,
};

#[doc = "< no error"]
pub const AtMaterialxErrorCode_AI_MATX_SUCCESS: AtMaterialxErrorCode = 0;
#[doc = "< error loading the document"]
pub const AtMaterialxErrorCode_AI_MATX_ERROR_LOAD_DOCUMENT: AtMaterialxErrorCode = 1;
#[doc = "< no materials found"]
pub const AtMaterialxErrorCode_AI_MATX_ERROR_NO_MATERIALS: AtMaterialxErrorCode = 2;
#[doc = " MaterialX error codes"]
pub type AtMaterialxErrorCode = ::std::os::raw::c_uint;
extern "C" {
    pub fn AiMaterialxWrite(
        universe: *const AtUniverse,
        filename: *const ::std::os::raw::c_char,
        look_name: *const ::std::os::raw::c_char,
        properties: *const ::std::os::raw::c_char,
        relative: bool,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn AiMaterialxWriteMaterial(
        filename: *const ::std::os::raw::c_char,
        material_name: *const ::std::os::raw::c_char,
        surface: *const AtNode,
        volume: *const AtNode,
        displacement: *const AtNode,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn AiMaterialxGetLookNames(filename: *const ::std::os::raw::c_char) -> *mut AtArray;
}
extern "C" {
    pub fn AiMaterialxGetMaterialNames(filename: *const ::std::os::raw::c_char) -> *mut AtArray;
}
extern "C" {
    pub fn AiMaterialxReadMaterials(
        universe: *mut AtUniverse,
        filename: *const ::std::os::raw::c_char,
        params: *const AtParamValueMap,
        nodes: *mut AtArray,
    ) -> ::std::os::raw::c_int;
}
