use super::{ai_color::AtRGB, ai_nodes::AtNode, ai_vector::AtVector};

#[doc = " Used to hold the result of AiTrace() calls"]
#[repr(C)]
pub struct AtScrSample {
    #[doc = "< color"]
    pub color: AtRGB,
    #[doc = "< opacity"]
    pub opacity: AtRGB,
    #[doc = "< alpha"]
    pub alpha: f32,
    #[doc = "< 3D hit point"]
    pub point: AtVector,
    #[doc = "< triangle ID of hit"]
    pub face: u32,
    #[doc = "< pointer to object hit"]
    pub obj: *const AtNode,
    #[doc = "< hit distance (Z depth)"]
    pub z: f32,
}
