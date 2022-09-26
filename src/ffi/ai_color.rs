#[doc = " RGB color"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtRGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
#[doc = " RGB color + alpha"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtRGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
extern "C" {
    #[doc = " Check to see if an RGB color has any corrupted components (nan or infinite)."]
    pub fn AiRGBIsFinite(rgba: *const AtRGB) -> bool;
}
extern "C" {
    #[doc = " Check to see if an RGBA color has any corrupted components (nan or infinite)."]
    pub fn AiRGBAIsFinite(rgba: *const AtRGBA) -> bool;
}
extern "C" {
    pub fn AiColorHeatMap(
        map_colors: *const AtRGB,
        map_values: *const f32,
        map_length: ::std::os::raw::c_uint,
        lookup: f32,
    ) -> AtRGB;
}
extern "C" {
    #[link_name = "\u{1}_ZL12AI_RGB_BLACK"]
    pub static AI_RGB_BLACK: AtRGB;
}
extern "C" {
    #[link_name = "\u{1}_ZL11AI_RGB_ZERO"]
    pub static AI_RGB_ZERO: AtRGB;
}
extern "C" {
    #[link_name = "\u{1}_ZL10AI_RGB_RED"]
    pub static AI_RGB_RED: AtRGB;
}
extern "C" {
    #[link_name = "\u{1}_ZL12AI_RGB_GREEN"]
    pub static AI_RGB_GREEN: AtRGB;
}
extern "C" {
    #[link_name = "\u{1}_ZL11AI_RGB_BLUE"]
    pub static AI_RGB_BLUE: AtRGB;
}
extern "C" {
    #[link_name = "\u{1}_ZL13AI_RGB_50GREY"]
    pub static AI_RGB_50GREY: AtRGB;
}
extern "C" {
    #[link_name = "\u{1}_ZL12AI_RGB_WHITE"]
    pub static AI_RGB_WHITE: AtRGB;
}
extern "C" {
    #[link_name = "\u{1}_ZL12AI_RGBA_ZERO"]
    pub static AI_RGBA_ZERO: AtRGBA;
}
extern "C" {
    #[link_name = "\u{1}_ZL11AI_RGBA_RED"]
    pub static AI_RGBA_RED: AtRGBA;
}
extern "C" {
    #[link_name = "\u{1}_ZL13AI_RGBA_GREEN"]
    pub static AI_RGBA_GREEN: AtRGBA;
}
extern "C" {
    #[link_name = "\u{1}_ZL12AI_RGBA_BLUE"]
    pub static AI_RGBA_BLUE: AtRGBA;
}
extern "C" {
    #[link_name = "\u{1}_ZL14AI_RGBA_50GREY"]
    pub static AI_RGBA_50GREY: AtRGBA;
}
extern "C" {
    #[link_name = "\u{1}_ZL13AI_RGBA_WHITE"]
    pub static AI_RGBA_WHITE: AtRGBA;
}
