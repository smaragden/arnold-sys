use ::std::{os::raw::{c_uint, c_int, c_ulong, c_void}, option::Option};

use super::{
    ai_closure::{AtBSDF, AtClosureList},
    ai_color::AtRGB,
    ai_shaderglobals::AtShaderGlobals,
    ai_string::AtString,
    ai_vector::{AtVector, AtVectorDv},
};

pub const AI_MICROFACET_BECKMANN: u32 = 0;
pub const AI_MICROFACET_GGX: u32 = 1;
#[doc = "< Sampling the BSDF always returns the same direction"]
pub const AtBSDFLobeFlags_AI_BSDF_LOBE_SINGULAR: AtBSDFLobeFlags = 1;
#[doc = "< Sampling the BSDF lobe requires a wavelength"]
pub const AtBSDFLobeFlags_AI_BSDF_LOBE_WAVELENGTH_SAMPLE: AtBSDFLobeFlags = 2;
#[doc = "< If ray depth exceeded, use background color"]
pub const AtBSDFLobeFlags_AI_BSDF_LOBE_EXIT_BACKGROUND: AtBSDFLobeFlags = 4;
#[doc = "< If ray depth exceeded, use white color"]
pub const AtBSDFLobeFlags_AI_BSDF_LOBE_EXIT_WHITE: AtBSDFLobeFlags = 8;
#[doc = " BSDF Lobe flags"]
pub type AtBSDFLobeFlags = c_uint;
#[doc = " BSDF lobe information"]
#[repr(C)]
pub struct AtBSDFLobeInfo {
    pub ray_type: u8,
    pub flags: u8,
    pub label: AtString,
}
#[doc = " BSDF lobe bitmask"]
pub type AtBSDFLobeMask = u32;
pub const AI_BSDF_LOBE_MASK_NONE: AtBSDFLobeMask = 0;
#[doc = " BSDF lobe sample"]
#[repr(C)]
pub struct AtBSDFLobeSample {
    pub weight: AtRGB,
    pub reverse_pdf: f32,
    pub pdf: f32,
}
#[doc = " BSDF function table"]
#[doc = ""]
#[doc = " This structure is used to report the function pointers that the"]
#[doc = " renderer needs to call at runtime. The version field is used for runtime"]
#[doc = " compatibility checking."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtBSDFMethods {
    pub version: c_int,
    pub Init:
        Option<unsafe extern "C" fn(sg: *const AtShaderGlobals, bsdf: *mut AtBSDF)>,
    pub Eval: Option<
        unsafe extern "C" fn(
            bsdf: *const AtBSDF,
            wi: *const AtVector,
            lobe_mask: AtBSDFLobeMask,
            need_pdf: bool,
            out_lobes: *mut AtBSDFLobeSample,
        ) -> AtBSDFLobeMask,
    >,
    pub Sample: Option<
        unsafe extern "C" fn(
            bsdf: *const AtBSDF,
            rnd: AtVector,
            wavelength: f32,
            lobe_mask: AtBSDFLobeMask,
            need_pdf: bool,
            out_wi: *mut AtVectorDv,
            out_lobe_index: *mut c_int,
            out_lobes: *mut AtBSDFLobeSample,
        ) -> AtBSDFLobeMask,
    >,
    pub Albedo: Option<
        unsafe extern "C" fn(
            bsdf: *const AtBSDF,
            sg: *const AtShaderGlobals,
            lobe_mask: AtBSDFLobeMask,
        ) -> AtRGB,
    >,
    pub Merge: Option<
        unsafe extern "C" fn(bsdf: *mut AtBSDF, other_bsdf: *const AtBSDF) -> bool,
    >,
    pub Interior: Option<
        unsafe extern "C" fn(sg: *const AtShaderGlobals, bsdf: *mut AtBSDF) -> AtClosureList,
    >,
}
extern "C" {
    #[doc = " \\name Functions for implementing custom BSDFs"]
    #[doc = " \\{"]
    pub fn AiBSDF(
        sg: *const AtShaderGlobals,
        weight: *const AtRGB,
        methods: *const AtBSDFMethods,
        data_size: c_ulong,
    ) -> *mut AtBSDF;
}
extern "C" {
    pub fn AiBSDFGetMethods(bsdf: *const AtBSDF) -> *const AtBSDFMethods;
}
extern "C" {
    pub fn AiBSDFGetData(bsdf: *const AtBSDF) -> *mut c_void;
}
extern "C" {
    pub fn AiBSDFGetLobes(bsdf: *const AtBSDF) -> *const AtBSDFLobeInfo;
}
extern "C" {
    pub fn AiBSDFGetNumLobes(bsdf: *const AtBSDF) -> c_int;
}
extern "C" {
    pub fn AiBSDFGetWeight(bsdf: *const AtBSDF) -> AtRGB;
}
extern "C" {
    pub fn AiBSDFSetDirectIndirect(bsdf: *mut AtBSDF, weight_direct: f32, weight_indirect: f32);
}
extern "C" {
    pub fn AiBSDFGetDirectIndirect(
        bsdf: *const AtBSDF,
        weight_direct: *mut f32,
        weight_indirect: *mut f32,
    );
}
extern "C" {
    pub fn AiBSDFInitLobes(
        bsdf: *mut AtBSDF,
        lobes: *const AtBSDFLobeInfo,
        num_lobes: c_int,
    );
}
extern "C" {
    pub fn AiBSDFInitNormal(bsdf: *mut AtBSDF, N: *const AtVector, bounding: bool);
}
extern "C" {
    pub fn AiBSDFBumpShadow(Ns: *const AtVector, N: *const AtVector, Ld: *const AtVector) -> f32;
}
extern "C" {
    pub fn AiBSDFMinRoughness(sg: *const AtShaderGlobals) -> f32;
}
extern "C" {
    #[doc = " \\name Built-in BSDFs"]
    #[doc = " \\{"]
    pub fn AiOrenNayarBSDF(
        sg: *const AtShaderGlobals,
        weight: *const AtRGB,
        N: *const AtVector,
        r: f32,
        transmission: bool,
        label: AtString,
    ) -> *mut AtBSDF;
}
extern "C" {
    pub fn AiMicrofacetBSDF(
        sg: *const AtShaderGlobals,
        weight: *const AtRGB,
        distribution: c_int,
        N: *const AtVector,
        U: *const AtVector,
        eta: f32,
        rx: f32,
        ry: f32,
        exit_type: u8,
        label: AtString,
    ) -> *mut AtBSDF;
}
extern "C" {
    pub fn AiMicrofacetBSDF_private(
        sg: *const AtShaderGlobals,
        weight: *const AtRGB,
        distribution: c_int,
        N: *const AtVector,
        U: *const AtVector,
        ior: f32,
        rx: f32,
        ry: f32,
        exit_type: u8,
        label: AtString,
        dielectric_importance: i32,
    ) -> *mut AtBSDF;
}
extern "C" {
    pub fn AiMicrofacetRefractionBSDF(
        sg: *const AtShaderGlobals,
        weight: *const AtRGB,
        distribution: c_int,
        N: *const AtVector,
        U: *const AtVector,
        ior: f32,
        rx: f32,
        ry: f32,
        dispersion: f32,
        use_fresnel: bool,
        interior: AtClosureList,
        exit_type: u8,
        label: AtString,
    ) -> *mut AtBSDF;
}
extern "C" {
    pub fn AiMicrofacetRefractionBSDF_private(
        sg: *const AtShaderGlobals,
        weight: *const AtRGB,
        distribution: c_int,
        N: *const AtVector,
        U: *const AtVector,
        ior: f32,
        rx: f32,
        ry: f32,
        dispersion: f32,
        use_fresnel: bool,
        interior: AtClosureList,
        exit_type: u8,
        label: AtString,
        dielectric_importance: i32,
    ) -> *mut AtBSDF;
}
extern "C" {
    pub fn AiMicrofacetThinWallRefractionBSDF(
        sg: *const AtShaderGlobals,
        weight: *const AtRGB,
        distribution: c_int,
        N: *const AtVector,
        U: *const AtVector,
        eta: f32,
        rx: f32,
        ry: f32,
        exit_type: u8,
        label: AtString,
    ) -> *mut AtBSDF;
}
extern "C" {
    pub fn AiMicrofacetSetThinFilm(bsdf: *mut AtBSDF, thickness: f32, eta: f32);
}
extern "C" {
    pub fn AiMetalBSDF(
        sg: *const AtShaderGlobals,
        weight: *const AtRGB,
        distribution: c_int,
        N: *const AtVector,
        U: *const AtVector,
        n: *const AtRGB,
        k: *const AtRGB,
        rx: f32,
        ry: f32,
        label: AtString,
    ) -> *mut AtBSDF;
}
extern "C" {
    pub fn AidEonBSDF(
        sg: *const AtShaderGlobals,
        absorption: *const AtRGB,
        weights: *const AtRGB,
        tangent: *const AtVector,
        roughness: f32,
        eta: f32,
        tilt: f32,
        label: AtString,
    ) -> *mut AtBSDF;
}
extern "C" {
    pub fn AiZinkeBSDF(
        sg: *const AtShaderGlobals,
        weight: *const AtRGB,
        tangent: *const AtVector,
        label: AtString,
    ) -> *mut AtBSDF;
}
extern "C" {
    pub fn AiSheenBSDF(
        sg: *const AtShaderGlobals,
        weight: *const AtRGB,
        N: *const AtVector,
        r: f32,
        label: AtString,
    ) -> *mut AtBSDF;
}
extern "C" {
    #[doc = " \\name BSDF integration"]
    #[doc = " \\{"]
    pub fn AiBSDFIntegrate(
        sg: *mut AtShaderGlobals,
        direct: *mut AtRGB,
        indirect: *mut AtRGB,
        bsdf: *mut AtBSDF,
    );
}
extern "C" {
    pub fn AiBSDFAlbedo(sg: *const AtShaderGlobals, bsdf: *mut AtBSDF) -> AtRGB;
}
