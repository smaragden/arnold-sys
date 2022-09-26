extern "C" {
    #[doc = " \\name Quantization and Dithering"]
    #[doc = " \\{"]
    pub fn AiQuantize8bit(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        i: ::std::os::raw::c_int,
        value: f32,
        dither: bool,
    ) -> u8;
}
extern "C" {
    pub fn AiQuantize16bit(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        i: ::std::os::raw::c_int,
        value: f32,
        dither: bool,
    ) -> u16;
}
