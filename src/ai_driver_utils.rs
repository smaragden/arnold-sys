use ::std::os::raw::c_int;

extern "C" {
    /// \\name Quantization and Dithering
    /// \\{
    pub fn AiQuantize8bit(x: c_int, y: c_int, i: c_int, value: f32, dither: bool) -> u8;

    pub fn AiQuantize16bit(x: c_int, y: c_int, i: c_int, value: f32, dither: bool) -> u16;
}
