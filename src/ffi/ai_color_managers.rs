use super::{ai_bbox::AtBBox2, ai_nodes::AtNode, ai_string::AtString};

#[repr(C)]
pub struct AtChannelLayout {
    pub channel_type: u8,
    pub type_: u8,
    pub x_stride: ::std::os::raw::c_ulong,
    pub y_stride: ::std::os::raw::c_ulong,
}
#[doc = " Color Manager Node methods structure"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtColorManagerNodeMethods {
    pub ColorManagerTransform: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut AtNode,
            arg2: AtString,
            arg3: bool,
            arg4: bool,
            arg5: *const AtBBox2,
            arg6: *mut ::std::os::raw::c_void,
            arg7: *const AtChannelLayout,
            arg8: *mut ::std::os::raw::c_void,
            arg9: *const AtChannelLayout,
        ) -> bool,
    >,
    pub ColorManagerGetDefaults: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut AtNode, arg2: *mut AtString, arg3: *mut AtString),
    >,
    pub ColorManagerGetChromaticities: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut AtNode, arg2: AtString, arg3: *mut f32) -> bool,
    >,
    pub ColorManagerGetCustomAttributes: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut AtNode,
            arg2: AtString,
            arg3: *mut ::std::os::raw::c_int,
            arg4: *mut *const ::std::os::raw::c_char,
        ),
    >,
    pub ColorManagerGetNumColorSpaces: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut AtNode, arg2: AtString) -> ::std::os::raw::c_int,
    >,
    pub ColorManagerGetColorSpaceNameByIndex: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut AtNode,
            i: ::std::os::raw::c_int,
            arg2: AtString,
        ) -> AtString,
    >,
    pub ColorManagerGetNumFamilies:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut AtNode) -> ::std::os::raw::c_int>,
    pub ColorManagerGetFamilyNameByIndex: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut AtNode, arg2: ::std::os::raw::c_int) -> AtString,
    >,
    pub ColorManagerColorSpaceIsLinear:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut AtNode, arg2: AtString) -> bool>,
}
extern "C" {
    pub fn AiColorManagerTransform(
        node: *mut AtNode,
        name: AtString,
        is_output: bool,
        dither: bool,
        roi: *const AtBBox2,
        src: *mut ::std::os::raw::c_void,
        src_layout: *const AtChannelLayout,
        dst: *mut ::std::os::raw::c_void,
        dst_layout: *const AtChannelLayout,
    ) -> bool;
}
extern "C" {
    pub fn AiColorManagerGetDefaults(node: *mut AtNode, sRGB: *mut AtString, linear: *mut AtString);
}
extern "C" {
    pub fn AiColorManagerGetChromaticities(
        node: *mut AtNode,
        space: AtString,
        chromaticities: *mut f32,
    ) -> bool;
}
extern "C" {
    pub fn AiColorManagerGetCustomAttributes(
        node: *mut AtNode,
        space: AtString,
        num: *mut ::std::os::raw::c_int,
        attributes: *mut *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn AiColorManagerGetNumColorSpaces(
        node: *mut AtNode,
        family: AtString,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn AiColorManagerGetColorSpaceNameByIndex(
        node: *mut AtNode,
        i: ::std::os::raw::c_int,
        family: AtString,
    ) -> AtString;
}
extern "C" {
    pub fn AiColorManagerGetNumFamilies(node: *mut AtNode) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn AiColorManagerGetFamilyNameByIndex(
        node: *mut AtNode,
        i: ::std::os::raw::c_int,
    ) -> AtString;
}
extern "C" {
    pub fn AiColorManagerColorSpaceIsLinear(node: *mut AtNode, cs: AtString) -> bool;
}
