use ::std::{
    option::Option,
    os::raw::{c_char, c_int, c_ulong, c_void},
};

use super::{ai_bbox::AtBBox2, ai_nodes::AtNode, ai_string::AtString};

#[repr(C)]
pub struct AtChannelLayout {
    pub channel_type: u8,
    pub type_: u8,
    pub x_stride: c_ulong,
    pub y_stride: c_ulong,
}
/// Color Manager Node methods structure
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtColorManagerNodeMethods {
    pub ColorManagerTransform: Option<
        unsafe extern "C" fn(
            arg1: *mut AtNode,
            arg2: AtString,
            arg3: bool,
            arg4: bool,
            arg5: *const AtBBox2,
            arg6: *mut c_void,
            arg7: *const AtChannelLayout,
            arg8: *mut c_void,
            arg9: *const AtChannelLayout,
        ) -> bool,
    >,
    pub ColorManagerGetDefaults:
        Option<unsafe extern "C" fn(arg1: *mut AtNode, arg2: *mut AtString, arg3: *mut AtString)>,
    pub ColorManagerGetChromaticities:
        Option<unsafe extern "C" fn(arg1: *mut AtNode, arg2: AtString, arg3: *mut f32) -> bool>,
    pub ColorManagerGetCustomAttributes: Option<
        unsafe extern "C" fn(
            arg1: *mut AtNode,
            arg2: AtString,
            arg3: *mut c_int,
            arg4: *mut *const c_char,
        ),
    >,
    pub ColorManagerGetNumColorSpaces:
        Option<unsafe extern "C" fn(arg1: *mut AtNode, arg2: AtString) -> c_int>,
    pub ColorManagerGetColorSpaceNameByIndex:
        Option<unsafe extern "C" fn(arg1: *mut AtNode, i: c_int, arg2: AtString) -> AtString>,
    pub ColorManagerGetNumFamilies: Option<unsafe extern "C" fn(arg1: *mut AtNode) -> c_int>,
    pub ColorManagerGetFamilyNameByIndex:
        Option<unsafe extern "C" fn(arg1: *mut AtNode, arg2: c_int) -> AtString>,
    pub ColorManagerColorSpaceIsLinear:
        Option<unsafe extern "C" fn(arg1: *mut AtNode, arg2: AtString) -> bool>,
}
extern "C" {
    pub fn AiColorManagerTransform(
        node: *mut AtNode,
        name: AtString,
        is_output: bool,
        dither: bool,
        roi: *const AtBBox2,
        src: *mut c_void,
        src_layout: *const AtChannelLayout,
        dst: *mut c_void,
        dst_layout: *const AtChannelLayout,
    ) -> bool;

    pub fn AiColorManagerGetDefaults(node: *mut AtNode, sRGB: *mut AtString, linear: *mut AtString);

    pub fn AiColorManagerGetChromaticities(
        node: *mut AtNode,
        space: AtString,
        chromaticities: *mut f32,
    ) -> bool;

    pub fn AiColorManagerGetCustomAttributes(
        node: *mut AtNode,
        space: AtString,
        num: *mut c_int,
        attributes: *mut *const c_char,
    );

    pub fn AiColorManagerGetNumColorSpaces(node: *mut AtNode, family: AtString) -> c_int;

    pub fn AiColorManagerGetColorSpaceNameByIndex(
        node: *mut AtNode,
        i: c_int,
        family: AtString,
    ) -> AtString;

    pub fn AiColorManagerGetNumFamilies(node: *mut AtNode) -> c_int;

    pub fn AiColorManagerGetFamilyNameByIndex(node: *mut AtNode, i: c_int) -> AtString;

    pub fn AiColorManagerColorSpaceIsLinear(node: *mut AtNode, cs: AtString) -> bool;
}
