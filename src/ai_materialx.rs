use ::std::os::raw::{c_char, c_uint};

use super::{
    ai_array::AtArray, ai_map::AtParamValueMap, ai_nodes::AtNode, ai_universe::AtUniverse,
};

/// no error
pub const AtMaterialxErrorCode_AI_MATX_SUCCESS: AtMaterialxErrorCode = 0;
/// error loading the document
pub const AtMaterialxErrorCode_AI_MATX_ERROR_LOAD_DOCUMENT: AtMaterialxErrorCode = 1;
/// no materials found
pub const AtMaterialxErrorCode_AI_MATX_ERROR_NO_MATERIALS: AtMaterialxErrorCode = 2;
/// MaterialX error codes
pub type AtMaterialxErrorCode = c_uint;
extern "C" {
    pub fn AiMaterialxWrite(
        universe: *const AtUniverse,
        filename: *const c_char,
        look_name: *const c_char,
        properties: *const c_char,
        relative: bool,
    ) -> ::std::os::raw::c_int;

    pub fn AiMaterialxWriteMaterial(
        filename: *const c_char,
        material_name: *const c_char,
        surface: *const AtNode,
        volume: *const AtNode,
        displacement: *const AtNode,
    ) -> ::std::os::raw::c_int;

    pub fn AiMaterialxGetLookNames(filename: *const c_char) -> *mut AtArray;

    pub fn AiMaterialxGetMaterialNames(filename: *const c_char) -> *mut AtArray;

    pub fn AiMaterialxReadMaterials(
        universe: *mut AtUniverse,
        filename: *const c_char,
        params: *const AtParamValueMap,
        nodes: *mut AtArray,
    ) -> ::std::os::raw::c_int;
}
