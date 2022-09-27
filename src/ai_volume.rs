use ::std::{
    option::Option,
    os::raw::{c_char, c_int, c_void},
};

use super::{
    ai_array::AtArray, ai_bbox::AtBBox, ai_nodes::AtNode, ai_params::AtParamValue,
    ai_shaderglobals::AtShaderGlobals, ai_string::AtString, ai_vector::AtVector,
};

pub const AI_VOLUME_INTERP_CLOSEST: u32 = 0;
pub const AI_VOLUME_INTERP_TRILINEAR: u32 = 1;
pub const AI_VOLUME_INTERP_TRICUBIC: u32 = 2;

/// Volume data, as returned by `AtVolumeCreate`
#[repr(C)]
pub struct AtVolumeData {
    /// Volume plugin private data, used how the plugin likes
    pub private_info: *mut c_void,
    /// Bounding box for this volume, plugin is responsible for also including volume_padding from the node
    pub bbox: AtBBox,
    /// Recommended step size for ray marching through this data
    pub auto_step_size: f32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtVolumeIntersectionInfo {
    _unused: [u8; 0],
}
extern "C" {
    pub fn AiVolumeAddIntersection(info: *const AtVolumeIntersectionInfo, t0: f32, t1: f32);

    pub fn AiVolumeMergeIntersection(
        info: *const AtVolumeIntersectionInfo,
        t0: f32,
        t1: f32,
        prim_id: u32,
    );

    pub fn AiVolumeFileGetChannels(filename: *const c_char) -> *mut AtArray;

    pub fn AiVolumeFileGetBBox(filename: *const c_char, channels: *const AtArray) -> AtBBox;
}
/// Volume plugin volume creation.
///
/// This method will be called for each volume node before usage.
///
/// \\param      node         Owner node where the volume was requested
/// \\param[out] data         Volume data with all fields to be set by the callback
/// \\return                  true if volume creation succeeded
pub type AtVolumeCreate =
    Option<unsafe extern "C" fn(node: *const AtNode, data: *mut AtVolumeData) -> bool>;
/// Volume plugin volume update.
///
/// This method will be called for each volume node before each render
/// pass or scene change, so that the volume can be update accordingly. If not
/// provided, the volume will be destroyed and recreated.
///
/// \\param      node         Owner node where the volume was requested
/// \\param[out] data         Volume data with all fields to be set by the callback
/// \\return                  true if volume data was modified
pub type AtVolumeUpdate =
    Option<unsafe extern "C" fn(node: *const AtNode, data: *mut AtVolumeData) -> bool>;
/// Volume plugin volume cleanup method.
///
/// This method will be called once for each volume that was created by a call to
/// `AtVolumeCreate` to allow a chance to clean up any private data.
///
/// \\param data      Volume data returned from `AtVolumeCreate`
/// \\return          true upon success
pub type AtVolumeCleanup =
    Option<unsafe extern "C" fn(node: *const AtNode, data: *mut AtVolumeData) -> bool>;
/// Volume plugin sample method.
///
/// This method will be called concurrently to sample data from a given channel
/// with the specified interpolation.  Implementors should use sg->Po as the
/// sampling position.
///
/// \\param      data      Volume data returned from `AtVolumeCreate`
/// \\param      channel   Data channel name from the volume or implicit surface
/// \\param      sg        Shader globals for the sampling context
/// \\param      interp    Volume interpolation quality, one of \\c AI_VOLUME_INTERP_*
/// \\param[out] value     Resulting sampled value, matching the type output in out_type
/// \\param[out] type      Resulting value type, one of \\c AI_TYPE_FLOAT, \\c AI_TYPE_VECTOR2, \\c AI_TYPE_RGB, \\c AI_TYPE_RGBA, or \\c AI_TYPE_VECTOR
/// \\return               true upon success
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
/// Volume plugin gradient method.
///
/// This method will be called concurrently to sample the gradient from a given
/// channel with the specified interpolation.  Implementors should use sg->Po as
/// the sampling position.  Note that this is generally only used for implicit
/// surfaces, so if the plugin is only outputting density volume data this
/// method can just return false and do no other work.  Also note that this will
/// also only make sense for scalar channels, such as signed distance fields.
///
/// \\param      data      Volume data returned from `AtVolumePluginCreateVolume`
/// \\param      channel   Data channel name from the volume or implicit surface
/// \\param      sg        Shader globals for the sampling context
/// \\param      interp    Volume interpolation quality, one of \\c AI_VOLUME_INTERP_*
/// \\param[out] gradient  Resulting sampled gradient
/// \\return               true upon success
pub type AtVolumeGradient = Option<
    unsafe extern "C" fn(
        data: *const AtVolumeData,
        channel: AtString,
        sg: *const AtShaderGlobals,
        interp: c_int,
        gradient: *mut AtVector,
    ) -> bool,
>;
/// Volume plugin method for submitting extents along a ray where there is data.
///
/// For each ray interval where there is volumetric data to be integrated this
/// callback should call `AiVolumeAddIntersection` to submit the extent
/// along the ray.  Any extra distance from the node's volume_padding parameter
/// must be accounted for and added to each interval.  Note that for implicits,
/// the ray extents should encompass the interval where there is data such as
/// signed-distance field values, like in narrow-band level sets.  The implicit
/// solver will then find the true ray intersection with the surface.
///
/// \\warning
/// Any ray extents submitted that overlap will have the shaders run more than
/// once for each extent.  If shaders should only be run once, then those extents
/// should be merged and submitted just once to `AiVolumeAddIntersection` instead.
///
/// \\param data      Volume data returned from `AtVolumeCreate`
/// \\param info      Opaque ptr to intersection info passed to `AiVolumeAddIntersection`
/// \\param tid       Current thread ID, use for thread-local access as needed
/// \\param time      Time at which the volume is being sampled (for motion blur)
/// \\param origin    Ray origin in object space
/// \\param direction Ray direction, normalized and in object space
/// \\param t0        Start of the source ray interval in which to check for extents
/// \\param t1        End of the source ray interval in which to check for extents
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
    /// This method is called to load/create a volume
    pub Create: AtVolumeCreate,
    /// This method is called to update a volume before render passes or on scene changes
    pub Update: AtVolumeUpdate,
    /// This method is called to clean up a volume when it's no longer needed
    pub Cleanup: AtVolumeCleanup,
    /// This method is called to get all tightly-bounded extents along a ray where the volume exists
    pub RayExtents: AtVolumeRayExtents,
    /// This method is called to sample a volume's named channel using a given point and interpolation
    pub Sample: AtVolumeSample,
    /// This method is called to sample the gradient of a volume's named channel using a given point and interpolation
    pub Gradient: AtVolumeGradient,
}
