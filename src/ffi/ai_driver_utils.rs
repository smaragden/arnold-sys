use ::std::os::raw::c_int;

extern "C" {
    #[doc = " \\name Quantization and Dithering"]
    #[doc = " \\{"]
    pub fn AiQuantize8bit(
        x: c_int,
        y: c_int,
        i: c_int,
        value: f32,
        dither: bool,
    ) -> u8;
}
extern "C" {
    pub fn AiQuantize16bit(
        x: c_int,
        y: c_int,
        i: c_int,
        value: f32,
        dither: bool,
    ) -> u16;
}
