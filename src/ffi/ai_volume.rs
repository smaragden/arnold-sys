use ::std::{os::raw::{c_void, c_char, c_int}, option::Option};

use super::{
    ai_array::AtArray, ai_bbox::AtBBox, ai_nodes::AtNode, ai_params::AtParamValue,
    ai_shaderglobals::AtShaderGlobals, ai_string::AtString, ai_vector::AtVector,
};

pub const AI_VOLUME_INTERP_CLOSEST: u32 = 0;
pub const AI_VOLUME_INTERP_TRILINEAR: u32 = 1;
pub const AI_VOLUME_INTERP_TRICUBIC: u32 = 2;

#[doc = " Volume data, as returned by \\ref AtVolumeCreate"]
#[repr(C)]
pub struct AtVolumeData {
    #[doc = "< Volume plugin private data, used how the plugin likes"]
    pub private_info: *mut c_void,
    #[doc = "< Bounding box for this volume, plugin is responsible for also including volume_padding from the node"]
    pub bbox: AtBBox,
    #[doc = "< Recommended step size for ray marching through this data"]
    pub auto_step_size: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtVolumeIntersectionInfo {
    _unused: [u8; 0],
}
extern "C" {
    pub fn AiVolumeAddIntersection(info: *const AtVolumeIntersectionInfo, t0: f32, t1: f32);
}
extern "C" {
    pub fn AiVolumeMergeIntersection(
        info: *const AtVolumeIntersectionInfo,
        t0: f32,
        t1: f32,
        prim_id: u32,
    );
}
extern "C" {
    pub fn AiVolumeFileGetChannels(filename: *const c_char) -> *mut AtArray;
}
extern "C" {
    pub fn AiVolumeFileGetBBox(
        filename: *const c_char,
        channels: *const AtArray,
    ) -> AtBBox;
}
#[doc = " Volume plugin volume creation."]
#[doc = ""]
#[doc = " This method will be called for each volume node before usage."]
#[doc = ""]
#[doc = " \\param      node         Owner node where the volume was requested"]
#[doc = " \\param[out] data         Volume data with all fields to be set by the callback"]
#[doc = " \\return                  true if volume creation succeeded"]
pub type AtVolumeCreate = Option<
    unsafe extern "C" fn(node: *const AtNode, data: *mut AtVolumeData) -> bool,
>;
#[doc = " Volume plugin volume update."]
#[doc = ""]
#[doc = " This method will be called for each volume node before each render"]
#[doc = " pass or scene change, so that the volume can be update accordingly. If not"]
#[doc = " provided, the volume will be destroyed and recreated."]
#[doc = ""]
#[doc = " \\param      node         Owner node where the volume was requested"]
#[doc = " \\param[out] data         Volume data with all fields to be set by the callback"]
#[doc = " \\return                  true if volume data was modified"]
pub type AtVolumeUpdate = Option<
    unsafe extern "C" fn(node: *const AtNode, data: *mut AtVolumeData) -> bool,
>;
#[doc = " Volume plugin volume cleanup method."]
#[doc = ""]
#[doc = " This method will be called once for each volume that was created by a call to"]
#[doc = " \\ref AtVolumeCreate to allow a chance to clean up any private data."]
#[doc = ""]
#[doc = " \\param data      Volume data returned from \\ref AtVolumeCreate"]
#[doc = " \\return          true upon success"]
pub type AtVolumeCleanup = Option<
    unsafe extern "C" fn(node: *const AtNode, data: *mut AtVolumeData) -> bool,
>;
#[doc = " Volume plugin sample method."]
#[doc = ""]
#[doc = " This method will be called concurrently to sample data from a given channel"]
#[doc = " with the specified interpolation.  Implementors should use sg->Po as the"]
#[doc = " sampling position."]
#[doc = ""]
#[doc = " \\param      data      Volume data returned from \\ref AtVolumeCreate"]
#[doc = " \\param      channel   Data channel name from the volume or implicit surface"]
#[doc = " \\param      sg        Shader globals for the sampling context"]
#[doc = " \\param      interp    Volume interpolation quality, one of \\c AI_VOLUME_INTERP_*"]
#[doc = " \\param[out] value     Resulting sampled value, matching the type output in out_type"]
#[doc = " \\param[out] type      Resulting value type, one of \\c AI_TYPE_FLOAT, \\c AI_TYPE_VECTOR2, \\c AI_TYPE_RGB, \\c AI_TYPE_RGBA, or \\c AI_TYPE_VECTOR"]
#[doc = " \\return               true upon success"]
pub type AtVolumeSample = Option<
    unsafe extern "C" fn(
        data: *const AtVolumeData,
        channel: AtString,
        sg: *const AtShaderGlobals,
        interp: c_int,
        value: *mut AtParamValue,
        type_: *mut u8,
    ) -> bool,
>;
#[doc = " Volume plugin gradient method."]
#[doc = ""]
#[doc = " This method will be called concurrently to sample the gradient from a given"]
#[doc = " channel with the specified interpolation.  Implementors should use sg->Po as"]
#[doc = " the sampling position.  Note that this is generally only used for implicit"]
#[doc = " surfaces, so if the plugin is only outputting density volume data this"]
#[doc = " method can just return false and do no other work.  Also note that this will"]
#[doc = " also only make sense for scalar channels, such as signed distance fields."]
#[doc = ""]
#[doc = " \\param      data      Volume data returned from \\ref AtVolumePluginCreateVolume"]
#[doc = " \\param      channel   Data channel name from the volume or implicit surface"]
#[doc = " \\param      sg        Shader globals for the sampling context"]
#[doc = " \\param      interp    Volume interpolation quality, one of \\c AI_VOLUME_INTERP_*"]
#[doc = " \\param[out] gradient  Resulting sampled gradient"]
#[doc = " \\return               true upon success"]
pub type AtVolumeGradient = Option<
    unsafe extern "C" fn(
        data: *const AtVolumeData,
        channel: AtString,
        sg: *const AtShaderGlobals,
        interp: c_int,
        gradient: *mut AtVector,
    ) -> bool,
>;
#[doc = " Volume plugin method for submitting extents along a ray where there is data."]
#[doc = ""]
#[doc = " For each ray interval where there is volumetric data to be integrated this"]
#[doc = " callback should call \\ref AiVolumeAddIntersection to submit the extent"]
#[doc = " along the ray.  Any extra distance from the node's volume_padding parameter"]
#[doc = " must be accounted for and added to each interval.  Note that for implicits,"]
#[doc = " the ray extents should encompass the interval where there is data such as"]
#[doc = " signed-distance field values, like in narrow-band level sets.  The implicit"]
#[doc = " solver will then find the true ray intersection with the surface."]
#[doc = ""]
#[doc = " \\warning"]
#[doc = " Any ray extents submitted that overlap will have the shaders run more than"]
#[doc = " once for each extent.  If shaders should only be run once, then those extents"]
#[doc = " should be merged and submitted just once to \\ref AiVolumeAddIntersection instead."]
#[doc = ""]
#[doc = " \\param data      Volume data returned from \\ref AtVolumeCreate"]
#[doc = " \\param info      Opaque ptr to intersection info passed to \\ref AiVolumeAddIntersection"]
#[doc = " \\param tid       Current thread ID, use for thread-local access as needed"]
#[doc = " \\param time      Time at which the volume is being sampled (for motion blur)"]
#[doc = " \\param origin    Ray origin in object space"]
#[doc = " \\param direction Ray direction, normalized and in object space"]
#[doc = " \\param t0        Start of the source ray interval in which to check for extents"]
#[doc = " \\param t1        End of the source ray interval in which to check for extents"]
pub type AtVolumeRayExtents = Option<
    unsafe extern "C" fn(
        data: *const AtVolumeData,
        info: *const AtVolumeIntersectionInfo,
        tid: u16,
        time: f32,
        origin: *const AtVector,
        direction: *const AtVector,
        t0: f32,
        t1: f32,
    ),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtVolumeNodeMethods {
    #[doc = "< This method is called to load/create a volume"]
    pub Create: AtVolumeCreate,
    #[doc = "< This method is called to update a volume before render passes or on scene changes"]
    pub Update: AtVolumeUpdate,
    #[doc = "< This method is called to clean up a volume when it's no longer needed"]
    pub Cleanup: AtVolumeCleanup,
    #[doc = "< This method is called to get all tightly-bounded extents along a ray where the volume exists"]
    pub RayExtents: AtVolumeRayExtents,
    #[doc = "< This method is called to sample a volume's named channel using a given point and interpolation"]
    pub Sample: AtVolumeSample,
    #[doc = "< This method is called to sample the gradient of a volume's named channel using a given point and interpolation"]
    pub Gradient: AtVolumeGradient,
}
