use ::std::os::raw::{c_char, c_int, c_void};

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
    /// \\name AOV-writing Functions
    ///
    /// \\details
    /// These functions allow the user to write AOVs.  Shaders may write
    /// AOVs indiscriminately without hesitation because the system
    /// understands the \"context\" in which an AOV write is performed. This means
    /// Arnold can store AOVs when it \"makes sense\" and ignore writes when
    /// it doesn't.
    ///
    /// For example, Arnold will not store \"deep\" AOV values (AOVs which are written
    /// at different depths for a singe pixel-sample) unless a driver specifically
    /// requests \"deep\" AOV values.  An example of a driver requiring \"deep\" AOV
    /// values would be one which writes deep-texture/volume-shadow files.
    ///
    /// Arnold will only perform AOV writes from within the main
    /// trunk of a pixel-sample ray-tree.  AOV writes made from inside a side-branch
    /// of the primary ray's ray-tree (such as a reflection) will be ignored.  Also,
    /// AOV writes are ignored during shadow evaluations.
    ///
    /// \\param sg    the current shader globals structure (pixel-sample state)
    /// \\param name  the name of the AOV to write into
    /// \\param val   the value to write into the AOV
    /// \\returns     a boolean indicating whether the write was successfully stored or not.
    ///              A failure could indicate that the AOV is not enabled, or is
    ///              of the wrong type,  or is being written in the wrong \"state\" (such
    ///              as not from within the trunk of the primary ray-tree).
    /// \\{
    pub fn AiAOVSetBool(sg: *mut AtShaderGlobals, name: AtString, val: bool) -> bool;
}
extern "C" {
    pub fn AiAOVSetInt(sg: *mut AtShaderGlobals, name: AtString, val: c_int) -> bool;
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
    pub fn AiAOVSetPtr(sg: *mut AtShaderGlobals, name: AtString, val: *mut c_void) -> bool;
}
extern "C" {
    /// \\name AOV-reading Functions
    ///
    /// \\details
    /// These functions allow the user to read AOVs.
    ///
    /// \\param sg    the current shader globals structure (pixel-sample state)
    /// \\param name  the name of the AOV to read from
    /// \\param val   the variable to store the read value
    /// \\returns     a boolean indicating whether the read was successfully done or not.
    ///              A failure could indicate that the AOV is not enabled, or is
    ///              of the wrong type.
    /// \\{
    pub fn AiAOVGetBool(sg: *const AtShaderGlobals, name: AtString, val: *mut bool) -> bool;
}
extern "C" {
    pub fn AiAOVGetInt(sg: *const AtShaderGlobals, name: AtString, val: *mut c_int) -> bool;
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
    pub fn AiAOVGetPtr(sg: *const AtShaderGlobals, name: AtString, val: *mut *mut c_void) -> bool;
}
extern "C" {
    pub fn AiAOVEnabled(name: AtString, type_: u8) -> bool;
}
extern "C" {
    pub fn AiAOVRegister(name: *const c_char, type_: u8, blend_mode: c_int) -> bool;
}
