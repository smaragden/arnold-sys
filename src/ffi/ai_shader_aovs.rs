use super::{
    ai_color::{AtRGB, AtRGBA},
    ai_matrix::AtMatrix,
    ai_shaderglobals::AtShaderGlobals,
    ai_string::AtString,
    ai_vector::{AtVector, AtVector2},
};

pub const AI_AOV_BLEND_NONE: u32 = 0;
pub const AI_AOV_BLEND_OPACITY: u32 = 1;

extern "C" {
    #[doc = " \\name AOV-writing Functions"]
    #[doc = ""]
    #[doc = " \\details"]
    #[doc = " These functions allow the user to write AOVs.  Shaders may write"]
    #[doc = " AOVs indiscriminately without hesitation because the system"]
    #[doc = " understands the \"context\" in which an AOV write is performed. This means"]
    #[doc = " Arnold can store AOVs when it \"makes sense\" and ignore writes when"]
    #[doc = " it doesn't."]
    #[doc = ""]
    #[doc = " For example, Arnold will not store \"deep\" AOV values (AOVs which are written"]
    #[doc = " at different depths for a singe pixel-sample) unless a driver specifically"]
    #[doc = " requests \"deep\" AOV values.  An example of a driver requiring \"deep\" AOV"]
    #[doc = " values would be one which writes deep-texture/volume-shadow files."]
    #[doc = ""]
    #[doc = " Arnold will only perform AOV writes from within the main"]
    #[doc = " trunk of a pixel-sample ray-tree.  AOV writes made from inside a side-branch"]
    #[doc = " of the primary ray's ray-tree (such as a reflection) will be ignored.  Also,"]
    #[doc = " AOV writes are ignored during shadow evaluations."]
    #[doc = ""]
    #[doc = " \\param sg    the current shader globals structure (pixel-sample state)"]
    #[doc = " \\param name  the name of the AOV to write into"]
    #[doc = " \\param val   the value to write into the AOV"]
    #[doc = " \\returns     a boolean indicating whether the write was successfully stored or not."]
    #[doc = "              A failure could indicate that the AOV is not enabled, or is"]
    #[doc = "              of the wrong type,  or is being written in the wrong \"state\" (such"]
    #[doc = "              as not from within the trunk of the primary ray-tree)."]
    #[doc = " \\{"]
    pub fn AiAOVSetBool(sg: *mut AtShaderGlobals, name: AtString, val: bool) -> bool;
}
extern "C" {
    pub fn AiAOVSetInt(
        sg: *mut AtShaderGlobals,
        name: AtString,
        val: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn AiAOVSetFlt(sg: *mut AtShaderGlobals, name: AtString, val: f32) -> bool;
}
extern "C" {
    pub fn AiAOVSetRGB(sg: *mut AtShaderGlobals, name: AtString, val: AtRGB) -> bool;
}
extern "C" {
    pub fn AiAOVSetRGBA(sg: *mut AtShaderGlobals, name: AtString, val: AtRGBA) -> bool;
}
extern "C" {
    pub fn AiAOVSetVec(sg: *mut AtShaderGlobals, name: AtString, val: AtVector) -> bool;
}
extern "C" {
    pub fn AiAOVSetVec2(sg: *mut AtShaderGlobals, name: AtString, val: AtVector2) -> bool;
}
extern "C" {
    pub fn AiAOVSetMatrix(sg: *mut AtShaderGlobals, name: AtString, val: AtMatrix) -> bool;
}
extern "C" {
    pub fn AiAOVSetPtr(
        sg: *mut AtShaderGlobals,
        name: AtString,
        val: *mut ::std::os::raw::c_void,
    ) -> bool;
}
extern "C" {
    #[doc = " \\name AOV-reading Functions"]
    #[doc = ""]
    #[doc = " \\details"]
    #[doc = " These functions allow the user to read AOVs."]
    #[doc = ""]
    #[doc = " \\param sg    the current shader globals structure (pixel-sample state)"]
    #[doc = " \\param name  the name of the AOV to read from"]
    #[doc = " \\param val   the variable to store the read value"]
    #[doc = " \\returns     a boolean indicating whether the read was successfully done or not."]
    #[doc = "              A failure could indicate that the AOV is not enabled, or is"]
    #[doc = "              of the wrong type."]
    #[doc = " \\{"]
    pub fn AiAOVGetBool(sg: *const AtShaderGlobals, name: AtString, val: *mut bool) -> bool;
}
extern "C" {
    pub fn AiAOVGetInt(
        sg: *const AtShaderGlobals,
        name: AtString,
        val: *mut ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    pub fn AiAOVGetFlt(sg: *const AtShaderGlobals, name: AtString, val: *mut f32) -> bool;
}
extern "C" {
    pub fn AiAOVGetRGB(sg: *const AtShaderGlobals, name: AtString, val: *mut AtRGB) -> bool;
}
extern "C" {
    pub fn AiAOVGetRGBA(sg: *const AtShaderGlobals, name: AtString, val: *mut AtRGBA) -> bool;
}
extern "C" {
    pub fn AiAOVGetVec(sg: *const AtShaderGlobals, name: AtString, val: *mut AtVector) -> bool;
}
extern "C" {
    pub fn AiAOVGetVec2(sg: *const AtShaderGlobals, name: AtString, val: *mut AtVector2) -> bool;
}
extern "C" {
    pub fn AiAOVGetMatrix(sg: *const AtShaderGlobals, name: AtString, val: *mut AtMatrix) -> bool;
}
extern "C" {
    pub fn AiAOVGetPtr(
        sg: *const AtShaderGlobals,
        name: AtString,
        val: *mut *mut ::std::os::raw::c_void,
    ) -> bool;
}
extern "C" {
    pub fn AiAOVEnabled(name: AtString, type_: u8) -> bool;
}
extern "C" {
    pub fn AiAOVRegister(
        name: *const ::std::os::raw::c_char,
        type_: u8,
        blend_mode: ::std::os::raw::c_int,
    ) -> bool;
}
