pub const AI_X: u32 = 0;
pub const AI_Y: u32 = 1;
pub const AI_Z: u32 = 2;
/// 3D point (single precision)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
/// 2D point
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtVector2 {
    pub x: f32,
    pub y: f32,
}
/// Homogeneous point
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtHPoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
/// Vector with differentials
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtVectorDv {
    pub val: AtVector,
    pub dx: AtVector,
    pub dy: AtVector,
}
extern "C" {
    /// Check whether a vector has all valid components (not NaN and not infinite)
    pub fn AiV3IsFinite(a: *const AtVector) -> bool;

    /// Build an orthonormal basis aligned with vector N (Frisvad's method).
    pub fn AiV3BuildLocalFrame(u: *mut AtVector, v: *mut AtVector, N: *const AtVector);

    /// Build an orthonormal basis aligned with vector N (polar method).
    pub fn AiV3BuildLocalFramePolar(u: *mut AtVector, v: *mut AtVector, N: *const AtVector);

    #[link_name = "\u{1}_ZL10AI_P3_ZERO"]
    pub static AI_P3_ZERO: AtVector;

    #[link_name = "\u{1}_ZL10AI_V3_ZERO"]
    pub static AI_V3_ZERO: AtVector;

    #[link_name = "\u{1}_ZL10AI_V3_HALF"]
    pub static AI_V3_HALF: AtVector;

    #[link_name = "\u{1}_ZL9AI_V3_ONE"]
    pub static AI_V3_ONE: AtVector;

    #[link_name = "\u{1}_ZL7AI_V3_X"]
    pub static AI_V3_X: AtVector;

    #[link_name = "\u{1}_ZL7AI_V3_Y"]
    pub static AI_V3_Y: AtVector;

    #[link_name = "\u{1}_ZL7AI_V3_Z"]
    pub static AI_V3_Z: AtVector;

    #[link_name = "\u{1}_ZL10AI_V3_NEGX"]
    pub static AI_V3_NEGX: AtVector;

    #[link_name = "\u{1}_ZL10AI_V3_NEGY"]
    pub static AI_V3_NEGY: AtVector;

    #[link_name = "\u{1}_ZL10AI_V3_NEGZ"]
    pub static AI_V3_NEGZ: AtVector;

    #[link_name = "\u{1}_ZL10AI_P2_ZERO"]
    pub static AI_P2_ZERO: AtVector2;

    #[link_name = "\u{1}_ZL9AI_P2_ONE"]
    pub static AI_P2_ONE: AtVector2;
}
