use ::std::{
    mem::transmute,
    os::raw::{c_char, c_uint, c_void},
};

use super::{
    ai_color::AtRGBA, ai_matrix::AtMatrix, ai_shaderglobals::AtShaderGlobals, ai_string::AtString,
    ai_universe::AtUniverse,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
pub const AI_WRAP_PERIODIC: u32 = 0;
pub const AI_WRAP_BLACK: u32 = 1;
pub const AI_WRAP_CLAMP: u32 = 2;
pub const AI_WRAP_MIRROR: u32 = 3;
pub const AI_WRAP_FILE: u32 = 4;
pub const AI_TEXTURE_CLOSEST: u32 = 0;
pub const AI_TEXTURE_BILINEAR: u32 = 1;
pub const AI_TEXTURE_BICUBIC: u32 = 2;
pub const AI_TEXTURE_SMART_BICUBIC: u32 = 3;
pub const AI_TEXTURE_MIPMODE_DEFAULT: u32 = 0;
pub const AI_TEXTURE_MIPMODE_NONE: u32 = 1;
pub const AI_TEXTURE_MIPMODE_ONE: u32 = 2;
pub const AI_TEXTURE_MIPMODE_TRILINEAR: u32 = 3;
pub const AI_TEXTURE_MIPMODE_ANISOTROPIC: u32 = 4;

/// Structure that holds all of the available texture map look-up options
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtTextureParams {
    /// value for nonexistent channels (e.g. alpha)
    pub fill: f32,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    /// starting channel index to read from
    pub start_channel: u8,
    /// mipmap level bias
    pub mipmap_bias: i8,
    pub _bitfield_align_2: [u8; 0],
    pub _bitfield_2: __BindgenBitfieldUnit<[u8; 1usize]>,
    /// scale S coordinate
    pub scale_s: f32,
    /// scale T coordinate
    pub scale_t: f32,
    /// multiplicative widening of look-ups on the S axis
    pub width_s: f32,
    /// multiplicative widening of look-ups on the T axis
    pub width_t: f32,
    /// additive blur in look-ups along the S axis
    pub blur_s: f32,
    /// additive blur in look-ups along the T axis
    pub blur_t: f32,
}
impl AtTextureParams {
    #[inline]
    pub fn filter(&self) -> u8 {
        unsafe { transmute(self._bitfield_1.get(0usize, 2u8) as u8) }
    }
    #[inline]
    pub fn set_filter(&mut self, val: u8) {
        unsafe {
            let val: u8 = transmute(val);
            self._bitfield_1.set(0usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn wrap_s(&self) -> u8 {
        unsafe { transmute(self._bitfield_1.get(2usize, 3u8) as u8) }
    }
    #[inline]
    pub fn set_wrap_s(&mut self, val: u8) {
        unsafe {
            let val: u8 = transmute(val);
            self._bitfield_1.set(2usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn wrap_t(&self) -> u8 {
        unsafe { transmute(self._bitfield_1.get(5usize, 3u8) as u8) }
    }
    #[inline]
    pub fn set_wrap_t(&mut self, val: u8) {
        unsafe {
            let val: u8 = transmute(val);
            self._bitfield_1.set(5usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        filter: u8,
        wrap_s: u8,
        wrap_t: u8,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 2u8, {
            let filter: u8 = unsafe { transmute(filter) };
            filter as u64
        });
        __bindgen_bitfield_unit.set(2usize, 3u8, {
            let wrap_s: u8 = unsafe { transmute(wrap_s) };
            wrap_s as u64
        });
        __bindgen_bitfield_unit.set(5usize, 3u8, {
            let wrap_t: u8 = unsafe { transmute(wrap_t) };
            wrap_t as u64
        });
        __bindgen_bitfield_unit
    }
    #[inline]
    pub fn mipmap_mode(&self) -> u8 {
        unsafe { transmute(self._bitfield_2.get(0usize, 3u8) as u8) }
    }
    #[inline]
    pub fn set_mipmap_mode(&mut self, val: u8) {
        unsafe {
            let val: u8 = transmute(val);
            self._bitfield_2.set(0usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn single_channel(&self) -> bool {
        unsafe { transmute(self._bitfield_2.get(3usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_single_channel(&mut self, val: bool) {
        unsafe {
            let val: u8 = transmute(val);
            self._bitfield_2.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn flip_s(&self) -> bool {
        unsafe { transmute(self._bitfield_2.get(4usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_flip_s(&mut self, val: bool) {
        unsafe {
            let val: u8 = transmute(val);
            self._bitfield_2.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn flip_t(&self) -> bool {
        unsafe { transmute(self._bitfield_2.get(5usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_flip_t(&mut self, val: bool) {
        unsafe {
            let val: u8 = transmute(val);
            self._bitfield_2.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn swap_st(&self) -> bool {
        unsafe { transmute(self._bitfield_2.get(6usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_swap_st(&mut self, val: bool) {
        unsafe {
            let val: u8 = transmute(val);
            self._bitfield_2.set(6usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_2(
        mipmap_mode: u8,
        single_channel: bool,
        flip_s: bool,
        flip_t: bool,
        swap_st: bool,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 3u8, {
            let mipmap_mode: u8 = unsafe { transmute(mipmap_mode) };
            mipmap_mode as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let single_channel: u8 = unsafe { transmute(single_channel) };
            single_channel as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let flip_s: u8 = unsafe { transmute(flip_s) };
            flip_s as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let flip_t: u8 = unsafe { transmute(flip_t) };
            flip_t as u64
        });
        __bindgen_bitfield_unit.set(6usize, 1u8, {
            let swap_st: u8 = unsafe { transmute(swap_st) };
            swap_st as u64
        });
        __bindgen_bitfield_unit
    }
}
extern "C" {
    pub fn AiTextureParamsSetDefaults(params: *mut AtTextureParams);
}
/// \\struct AtTextureHandle
///
/// \\brief  Structure that holds a handle for a given texture
///
/// Texture handles can be created through `AiTextureHandleCreate` and destroyed
/// with `AiTextureHandleDestroy`. Texture lookups can be done with `AiTextureHandleAccess`,
/// which is faster than `AiTextureAccess`.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtTextureHandle {
    _unused: [u8; 0],
}
extern "C" {
    pub fn AiTextureHandleCreate(
        filename: *const c_char,
        color_space: AtString,
    ) -> *mut AtTextureHandle;
}
extern "C" {
    pub fn AiTextureHandleAccess(
        sg: *const AtShaderGlobals,
        handle: *mut AtTextureHandle,
        params: *const AtTextureParams,
        success: *mut bool,
    ) -> AtRGBA;
}
extern "C" {
    pub fn AiTextureHandleDestroy(handle: *mut AtTextureHandle);
}
extern "C" {
    pub fn AiTextureAccess(
        sg: *const AtShaderGlobals,
        filename: AtString,
        color_space: AtString,
        params: *const AtTextureParams,
        success: *mut bool,
    ) -> AtRGBA;
}
extern "C" {
    pub fn AiTextureLoad(
        filename: AtString,
        use_float: bool,
        miplevel: c_uint,
        image: *mut c_void,
    ) -> bool;
}
extern "C" {
    pub fn AiTextureGetResolution(
        filename: *const c_char,
        width: *mut c_uint,
        height: *mut c_uint,
    ) -> bool;
}
extern "C" {
    pub fn AiTextureGetNumChannels(filename: *const c_char, num_channels: *mut c_uint) -> bool;
}
extern "C" {
    pub fn AiTextureGetChannelName(filename: *const c_char, channel_index: c_uint)
        -> *const c_char;
}
extern "C" {
    pub fn AiTextureGetFormat(filename: *const c_char, format: *mut c_uint) -> bool;
}
extern "C" {
    pub fn AiTextureGetBitDepth(filename: *const c_char, bit_depth: *mut c_uint) -> bool;
}
extern "C" {
    pub fn AiTextureGetMatrices(
        filename: *const c_char,
        world_to_screen: *mut AtMatrix,
        world_to_camera: *mut AtMatrix,
    ) -> bool;
}
extern "C" {
    pub fn AiTextureInvalidate(filename: *const c_char);
}
pub const AtMakeTxStatus_AiTxPending: AtMakeTxStatus = 0;
pub const AtMakeTxStatus_AiTxError: AtMakeTxStatus = 1;
pub const AtMakeTxStatus_AiTxUpdated: AtMakeTxStatus = 2;
pub const AtMakeTxStatus_AiTxUpdate_unneeded: AtMakeTxStatus = 3;
pub const AtMakeTxStatus_AiTxAborted: AtMakeTxStatus = 4;
/// Status of AiMakeTx jobs.
pub type AtMakeTxStatus = c_uint;
extern "C" {
    pub fn AiMakeTx(filename: *const c_char, flags: *const c_char, universe: *const AtUniverse);
}
extern "C" {
    pub fn AiMakeTxWaitJob(
        statuses: *mut *mut AtMakeTxStatus,
        source_files: *mut *mut *const c_char,
        num_submitted_textures: *mut c_uint,
    ) -> c_uint;
}
extern "C" {
    pub fn AiMakeTxAbort(
        statuses: *mut *mut AtMakeTxStatus,
        source_files: *mut *mut *const c_char,
        num_submitted_textures: *mut c_uint,
    );
}
