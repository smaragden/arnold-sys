use super::ai_vector::AtVector;
#[doc = " 3D axis-aligned bounding box (uses single-precision)"]
#[repr(C)]
pub struct AtBBox {
    pub min: AtVector,
    pub max: AtVector,
}
#[doc = " 2D axis-aligned bounding box (uses integers)"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtBBox2 {
    pub minx: ::std::os::raw::c_int,
    pub miny: ::std::os::raw::c_int,
    pub maxx: ::std::os::raw::c_int,
    pub maxy: ::std::os::raw::c_int,
}
extern "C" {
    #[doc = " \\name Constants"]
    #[doc = " \\{"]
    #[link_name = "\u{1}_ZL12AI_BBOX_ZERO"]
    pub static AI_BBOX_ZERO: AtBBox;
}
