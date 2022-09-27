use super::{
    ai_color::AtRGB,
    ai_nodes::AtNode,
    ai_vector::{AtVector, AtVector2},
};

/// Camera ray creation inputs
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtCameraInput {
    /// screen-space coordinates will range between
    ///(screen_window_min.x, screen_window_min.y/frame_aspect_ratio) and
    ///(screen_window_max.x, screen_window_max.y/frame_aspect_ratio)
    pub sx: f32,
    /// screen-space coordinates will range between
    ///(screen_window_min.x, screen_window_min.y/frame_aspect_ratio) and
    ///(screen_window_max.x, screen_window_max.y/frame_aspect_ratio)
    pub sy: f32,
    /// derivatives of the screen-space coordinates with respect to pixel coordinates
    pub dsx: f32,
    /// derivatives of the screen-space coordinates with respect to pixel coordinates
    pub dsy: f32,
    /// lens sampling coordinates in [0,1)^2
    pub lensx: f32,
    /// lens sampling coordinates in [0,1)^2
    pub lensy: f32,
    /// time relative to this camera (in [0,1))
    pub relative_time: f32,
}
/// Camera ray creation outputs
///
///  If the d*d* derivatives are left to their default value of zero,
///  an accurate numerical estimate will be automatically computed for
///  them to prevent catastrophic degradation of texture IO performance.
///  Note that this estimate might not be as good as analytically computed
///  derivatives but will often be good enough.
#[repr(C)]
pub struct AtCameraOutput {
    /// ray origin in camera space (required)
    pub origin: AtVector,
    /// ray direction in camera space (required)
    pub dir: AtVector,
    /// derivative of the ray origin with respect to the pixel coordinates (optional - defaults to 0)
    pub dOdx: AtVector,
    /// derivative of the ray origin with respect to the pixel coordinates (optional - defaults to 0)
    pub dOdy: AtVector,
    /// derivative of the ray direction with respect to the pixel coordinates (optional - defaults to 0)
    pub dDdx: AtVector,
    /// derivative of the ray direction with respect to the pixel coordinates (optional - defaults to 0)
    pub dDdy: AtVector,
    /// weight of this ray (used for vignetting) (optional - defaults to AI_RGB_WHITE)
    pub weight: AtRGB,
}
/// Camera node methods structure
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtCameraNodeMethods {
    pub CreateRay: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const AtNode,
            arg2: *const AtCameraInput,
            arg3: *mut AtCameraOutput,
            tid: u16,
        ),
    >,
    pub ReverseRay: ::std::option::Option<
        unsafe extern "C" fn(
            node: *const AtNode,
            Po: *const AtVector,
            relative_time: f32,
            Ps: *mut AtVector2,
        ) -> bool,
    >,
}
extern "C" {
    /// \\name API Methods for Camera Writers
    /// \\{
    pub fn AiCameraInitialize(node: *mut AtNode);
}
extern "C" {
    pub fn AiCameraUpdate(node: *mut AtNode, plane_distance: bool);
}
