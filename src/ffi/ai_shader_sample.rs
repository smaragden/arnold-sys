use super::{ai_color::AtRGB, ai_nodes::AtNode, ai_vector::AtVector};

/// Used to hold the result of AiTrace() calls
#[repr(C)]
pub struct AtScrSample {
    /// color
    pub color: AtRGB,
    /// opacity
    pub opacity: AtRGB,
    /// alpha
    pub alpha: f32,
    /// 3D hit point
    pub point: AtVector,
    /// triangle ID of hit
    pub face: u32,
    /// pointer to object hit
    pub obj: *const AtNode,
    /// hit distance (Z depth)
    pub z: f32,
}
