use ::std::os::raw::c_uint;

/// RGB color
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtRGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
/// RGB color + alpha
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtRGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
extern "C" {
    /// Check to see if an RGB color has any corrupted components (nan or infinite).
    pub fn AiRGBIsFinite(rgba: *const AtRGB) -> bool;

    /// Check to see if an RGBA color has any corrupted components (nan or infinite).
    pub fn AiRGBAIsFinite(rgba: *const AtRGBA) -> bool;

    pub fn AiColorHeatMap(
        map_colors: *const AtRGB,
        map_values: *const f32,
        map_length: c_uint,
        lookup: f32,
    ) -> AtRGB;

    #[link_name = "\u{1}_ZL12AI_RGB_BLACK"]
    pub static AI_RGB_BLACK: AtRGB;

    #[link_name = "\u{1}_ZL11AI_RGB_ZERO"]
    pub static AI_RGB_ZERO: AtRGB;

    #[link_name = "\u{1}_ZL10AI_RGB_RED"]
    pub static AI_RGB_RED: AtRGB;

    #[link_name = "\u{1}_ZL12AI_RGB_GREEN"]
    pub static AI_RGB_GREEN: AtRGB;

    #[link_name = "\u{1}_ZL11AI_RGB_BLUE"]
    pub static AI_RGB_BLUE: AtRGB;

    #[link_name = "\u{1}_ZL13AI_RGB_50GREY"]
    pub static AI_RGB_50GREY: AtRGB;

    #[link_name = "\u{1}_ZL12AI_RGB_WHITE"]
    pub static AI_RGB_WHITE: AtRGB;

    #[link_name = "\u{1}_ZL12AI_RGBA_ZERO"]
    pub static AI_RGBA_ZERO: AtRGBA;

    #[link_name = "\u{1}_ZL11AI_RGBA_RED"]
    pub static AI_RGBA_RED: AtRGBA;

    #[link_name = "\u{1}_ZL13AI_RGBA_GREEN"]
    pub static AI_RGBA_GREEN: AtRGBA;

    #[link_name = "\u{1}_ZL12AI_RGBA_BLUE"]
    pub static AI_RGBA_BLUE: AtRGBA;

    #[link_name = "\u{1}_ZL14AI_RGBA_50GREY"]
    pub static AI_RGBA_50GREY: AtRGBA;

    #[link_name = "\u{1}_ZL13AI_RGBA_WHITE"]
    pub static AI_RGBA_WHITE: AtRGBA;
}
