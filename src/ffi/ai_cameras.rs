use super::{
    ai_color::AtRGB,
    ai_nodes::AtNode,
    ai_vector::{AtVector, AtVector2},
};

#[doc = " Camera ray creation inputs"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtCameraInput {
    #[doc = "< screen-space coordinates will range between"]
    #[doc = "(screen_window_min.x, screen_window_min.y/frame_aspect_ratio) and"]
    #[doc = "(screen_window_max.x, screen_window_max.y/frame_aspect_ratio)"]
    pub sx: f32,
    #[doc = "< screen-space coordinates will range between"]
    #[doc = "(screen_window_min.x, screen_window_min.y/frame_aspect_ratio) and"]
    #[doc = "(screen_window_max.x, screen_window_max.y/frame_aspect_ratio)"]
    pub sy: f32,
    #[doc = "< derivatives of the screen-space coordinates with respect to pixel coordinates"]
    pub dsx: f32,
    #[doc = "< derivatives of the screen-space coordinates with respect to pixel coordinates"]
    pub dsy: f32,
    #[doc = "< lens sampling coordinates in [0,1)^2"]
    pub lensx: f32,
    #[doc = "< lens sampling coordinates in [0,1)^2"]
    pub lensy: f32,
    #[doc = "< time relative to this camera (in [0,1))"]
    pub relative_time: f32,
}
#[doc = " Camera ray creation outputs"]
#[doc = ""]
#[doc = "  If the d*d* derivatives are left to their default value of zero,"]
#[doc = "  an accurate numerical estimate will be automatically computed for"]
#[doc = "  them to prevent catastrophic degradation of texture IO performance."]
#[doc = "  Note that this estimate might not be as good as analytically computed"]
#[doc = "  derivatives but will often be good enough."]
#[repr(C)]
pub struct AtCameraOutput {
    #[doc = "< ray origin in camera space (required)"]
    pub origin: AtVector,
    #[doc = "< ray direction in camera space (required)"]
    pub dir: AtVector,
    #[doc = "< derivative of the ray origin with respect to the pixel coordinates (optional - defaults to 0)"]
    pub dOdx: AtVector,
    #[doc = "< derivative of the ray origin with respect to the pixel coordinates (optional - defaults to 0)"]
    pub dOdy: AtVector,
    #[doc = "< derivative of the ray direction with respect to the pixel coordinates (optional - defaults to 0)"]
    pub dDdx: AtVector,
    #[doc = "< derivative of the ray direction with respect to the pixel coordinates (optional - defaults to 0)"]
    pub dDdy: AtVector,
    #[doc = "< weight of this ray (used for vignetting) (optional - defaults to AI_RGB_WHITE)"]
    pub weight: AtRGB,
}
#[doc = " Camera node methods structure"]
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
    #[doc = " \\name API Methods for Camera Writers"]
    #[doc = " \\{"]
    pub fn AiCameraInitialize(node: *mut AtNode);
}
extern "C" {
    pub fn AiCameraUpdate(node: *mut AtNode, plane_distance: bool);
}
