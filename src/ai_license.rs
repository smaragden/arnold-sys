use ::std::os::raw::{c_char, c_int, c_uint};

pub const AI_LIC_SUCCESS: u32 = 0;
pub const AI_LIC_ERROR_CANTCONNECT: u32 = 1;
pub const AI_LIC_ERROR_INIT: u32 = 2;
pub const AI_LIC_ERROR_NOTFOUND: u32 = 3;
pub const AI_LIC_ERROR_NOTAVAILABLE: u32 = 4;
pub const AI_LIC_ERROR: i32 = -1;
extern "C" {
    /// \\defgroup ai_license Licensing API
    ///
    /// Arnold license system information.
    ///
    /// \\{
    pub fn AiSetLicenseString(lic: *const c_char);
}
/// License info structure. This is designed for RLM, so non-RLM licenses might
/// leave most fields uninitialized
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtLicenseInfo {
    /// true if this is the license used by the current Arnold installation
    pub used: bool,
    /// product name
    pub name: [c_char; 64usize],
    /// product version
    pub ver: [c_char; 64usize],
    /// expiration date
    pub exp: [c_char; 64usize],
    /// product options
    pub options: [c_char; 64usize],
    /// license count
    pub count: c_int,
    /// license count in use
    pub current_inuse: c_int,
    /// number of reservations in use
    pub current_resuse: c_int,
    /// HOST-BASED count
    pub hbased: c_int,
    /// license hold time
    pub hold: c_int,
    /// maximum roam time
    pub max_roam: c_int,
    /// maximum number of processes that can share this license
    pub max_share: c_int,
    /// minimum rlmremove time
    pub min_remove: c_int,
    /// license minimum checkout time
    pub min_checkout: c_int,
    /// minimum timeout time
    pub min_timeout: c_int,
    /// number of license reservations
    pub nres: c_int,
    /// number of roaming licenses allowed
    pub num_roam_allowed: c_int,
    /// number of licenses currently roaming (for roaming licenses)
    pub roaming: c_int,
    /// license share flags (share flags RLM_LA_SHARE_xxx are defined in license.h)
    pub share: c_int,
    /// license soft limit
    pub soft_limit: c_int,
    /// 1 if this license is a roaming license
    pub thisroam: c_int,
    /// current license timeout
    pub timeout: c_int,
    /// license timezone specification.
    pub tz: c_int,
    /// if 0, this is a normal license. If non-zero, this is a token-based license
    pub tokens: c_int,
    /// license type (license type flags RLM_LA__xxx_TYPE are defined in license.h)
    pub type_: c_int,
    /// USER_BASED count
    pub ubased: c_int,
}

extern "C" {
    pub fn AiLicenseGetInfo(licenses: *mut *mut AtLicenseInfo, n: *mut c_uint) -> c_int;

    pub fn AiLicenseIsAvailable() -> bool;
}
