use super::ai_vector::{AtVector, AtVector2};

extern "C" {
    #[doc = " \\name Perlin Noise"]
    #[doc = " \\{"]
    pub fn AiPerlin2(p: AtVector2) -> f32;
}
extern "C" {
    pub fn AiPerlin3(p: AtVector) -> f32;
}
extern "C" {
    pub fn AiPerlin4(p: AtVector, time: f32) -> f32;
}
extern "C" {
    #[doc = " \\name Periodic Perlin Noise"]
    #[doc = " \\{"]
    pub fn AiPeriodicPerlin2(
        p: AtVector2,
        periodx: ::std::os::raw::c_int,
        periody: ::std::os::raw::c_int,
    ) -> f32;
}
extern "C" {
    pub fn AiPeriodicPerlin3(
        p: AtVector,
        periodx: ::std::os::raw::c_int,
        periody: ::std::os::raw::c_int,
        periodz: ::std::os::raw::c_int,
    ) -> f32;
}
extern "C" {
    pub fn AiPeriodicPerlin4(
        p: AtVector,
        time: f32,
        periodx: ::std::os::raw::c_int,
        periody: ::std::os::raw::c_int,
        periodz: ::std::os::raw::c_int,
        periodt: ::std::os::raw::c_int,
    ) -> f32;
}
extern "C" {
    #[doc = " \\name Summed-Noise"]
    #[doc = " \\{"]
    pub fn AiNoise2(
        p: AtVector2,
        octaves: ::std::os::raw::c_int,
        distortion: f32,
        lacunarity: f32,
    ) -> f32;
}
extern "C" {
    pub fn AiNoise3(
        p: AtVector,
        octaves: ::std::os::raw::c_int,
        distortion: f32,
        lacunarity: f32,
    ) -> f32;
}
extern "C" {
    pub fn AiNoise4(
        p: AtVector,
        time: f32,
        octaves: ::std::os::raw::c_int,
        distortion: f32,
        lacunarity: f32,
    ) -> f32;
}
extern "C" {
    pub fn AiVNoise2(
        p: AtVector2,
        octaves: ::std::os::raw::c_int,
        distortion: f32,
        lacunarity: f32,
    ) -> AtVector2;
}
extern "C" {
    pub fn AiVNoise3(
        p: AtVector,
        octaves: ::std::os::raw::c_int,
        distortion: f32,
        lacunarity: f32,
    ) -> AtVector;
}
extern "C" {
    pub fn AiVNoise4(
        p: AtVector,
        time: f32,
        octaves: ::std::os::raw::c_int,
        distortion: f32,
        lacunarity: f32,
    ) -> AtVector;
}
extern "C" {
    pub fn AiCellNoise2(p: AtVector2) -> f32;
}
extern "C" {
    pub fn AiCellNoise3(p: AtVector) -> f32;
}
extern "C" {
    pub fn AiCellNoise4(p: AtVector, t: f32) -> f32;
}
extern "C" {
    pub fn AiVCellNoise2(p: AtVector2) -> AtVector2;
}
extern "C" {
    pub fn AiVCellNoise3(p: AtVector) -> AtVector;
}
extern "C" {
    pub fn AiVCellNoise4(p: AtVector, t: f32) -> AtVector;
}
