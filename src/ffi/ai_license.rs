pub const AI_LIC_SUCCESS: u32 = 0;
pub const AI_LIC_ERROR_CANTCONNECT: u32 = 1;
pub const AI_LIC_ERROR_INIT: u32 = 2;
pub const AI_LIC_ERROR_NOTFOUND: u32 = 3;
pub const AI_LIC_ERROR_NOTAVAILABLE: u32 = 4;
pub const AI_LIC_ERROR: i32 = -1;
extern "C" {
    #[doc = " \\defgroup ai_license Licensing API"]
    #[doc = ""]
    #[doc = " Arnold license system information."]
    #[doc = ""]
    #[doc = " \\{"]
    pub fn AiSetLicenseString(lic: *const ::std::os::raw::c_char);
}
#[doc = " License info structure. This is designed for RLM, so non-RLM licenses might"]
#[doc = " leave most fields uninitialized"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtLicenseInfo {
    #[doc = "< true if this is the license used by the current Arnold installation"]
    pub used: bool,
    #[doc = "< product name"]
    pub name: [::std::os::raw::c_char; 64usize],
    #[doc = "< product version"]
    pub ver: [::std::os::raw::c_char; 64usize],
    #[doc = "< expiration date"]
    pub exp: [::std::os::raw::c_char; 64usize],
    #[doc = "< product options"]
    pub options: [::std::os::raw::c_char; 64usize],
    #[doc = "< license count"]
    pub count: ::std::os::raw::c_int,
    #[doc = "< license count in use"]
    pub current_inuse: ::std::os::raw::c_int,
    #[doc = "< number of reservations in use"]
    pub current_resuse: ::std::os::raw::c_int,
    #[doc = "< HOST-BASED count"]
    pub hbased: ::std::os::raw::c_int,
    #[doc = "< license hold time"]
    pub hold: ::std::os::raw::c_int,
    #[doc = "< maximum roam time"]
    pub max_roam: ::std::os::raw::c_int,
    #[doc = "< maximum number of processes that can share this license"]
    pub max_share: ::std::os::raw::c_int,
    #[doc = "< minimum rlmremove time"]
    pub min_remove: ::std::os::raw::c_int,
    #[doc = "< license minimum checkout time"]
    pub min_checkout: ::std::os::raw::c_int,
    #[doc = "< minimum timeout time"]
    pub min_timeout: ::std::os::raw::c_int,
    #[doc = "< number of license reservations"]
    pub nres: ::std::os::raw::c_int,
    #[doc = "< number of roaming licenses allowed"]
    pub num_roam_allowed: ::std::os::raw::c_int,
    #[doc = "< number of licenses currently roaming (for roaming licenses)"]
    pub roaming: ::std::os::raw::c_int,
    #[doc = "< license share flags (share flags RLM_LA_SHARE_xxx are defined in license.h)"]
    pub share: ::std::os::raw::c_int,
    #[doc = "< license soft limit"]
    pub soft_limit: ::std::os::raw::c_int,
    #[doc = "< 1 if this license is a roaming license"]
    pub thisroam: ::std::os::raw::c_int,
    #[doc = "< current license timeout"]
    pub timeout: ::std::os::raw::c_int,
    #[doc = "< license timezone specification."]
    pub tz: ::std::os::raw::c_int,
    #[doc = "< if 0, this is a normal license. If non-zero, this is a token-based license"]
    pub tokens: ::std::os::raw::c_int,
    #[doc = "< license type (license type flags RLM_LA__xxx_TYPE are defined in license.h)"]
    pub type_: ::std::os::raw::c_int,
    #[doc = "< USER_BASED count"]
    pub ubased: ::std::os::raw::c_int,
}
extern "C" {
    pub fn AiLicenseGetInfo(
        licenses: *mut *mut AtLicenseInfo,
        n: *mut ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn AiLicenseIsAvailable() -> bool;
}