use ::std::os::raw::c_int;

use super::ai_vector::AtVector;

/// 3D axis-aligned bounding box (uses single-precision)
#[repr(C)]
pub struct AtBBox {
    pub min: AtVector,
    pub max: AtVector,
}
/// 2D axis-aligned bounding box (uses integers)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtBBox2 {
    pub minx: c_int,
    pub miny: c_int,
    pub maxx: c_int,
    pub maxy: c_int,
}
extern "C" {
    /// \\name Constants
    /// \\{
    #[link_name = "\u{1}_ZL12AI_BBOX_ZERO"]
    pub static AI_BBOX_ZERO: AtBBox;
}
