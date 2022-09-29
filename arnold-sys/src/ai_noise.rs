use ::std::os::raw::c_int;

use super::ai_vector::{AtVector, AtVector2};

extern "C" {
    /// \\name Perlin Noise
    /// \\{
    pub fn AiPerlin2(p: AtVector2) -> f32;

    pub fn AiPerlin3(p: AtVector) -> f32;

    pub fn AiPerlin4(p: AtVector, time: f32) -> f32;

    /// \\name Periodic Perlin Noise
    /// \\{
    pub fn AiPeriodicPerlin2(p: AtVector2, periodx: c_int, periody: c_int) -> f32;

    pub fn AiPeriodicPerlin3(p: AtVector, periodx: c_int, periody: c_int, periodz: c_int) -> f32;

    pub fn AiPeriodicPerlin4(
        p: AtVector,
        time: f32,
        periodx: c_int,
        periody: c_int,
        periodz: c_int,
        periodt: c_int,
    ) -> f32;

    /// \\name Summed-Noise
    /// \\{
    pub fn AiNoise2(p: AtVector2, octaves: c_int, distortion: f32, lacunarity: f32) -> f32;

    pub fn AiNoise3(p: AtVector, octaves: c_int, distortion: f32, lacunarity: f32) -> f32;

    pub fn AiNoise4(
        p: AtVector,
        time: f32,
        octaves: c_int,
        distortion: f32,
        lacunarity: f32,
    ) -> f32;

    pub fn AiVNoise2(p: AtVector2, octaves: c_int, distortion: f32, lacunarity: f32) -> AtVector2;

    pub fn AiVNoise3(p: AtVector, octaves: c_int, distortion: f32, lacunarity: f32) -> AtVector;

    pub fn AiVNoise4(
        p: AtVector,
        time: f32,
        octaves: c_int,
        distortion: f32,
        lacunarity: f32,
    ) -> AtVector;

    pub fn AiCellNoise2(p: AtVector2) -> f32;

    pub fn AiCellNoise3(p: AtVector) -> f32;

    pub fn AiCellNoise4(p: AtVector, t: f32) -> f32;

    pub fn AiVCellNoise2(p: AtVector2) -> AtVector2;

    pub fn AiVCellNoise3(p: AtVector) -> AtVector;

    pub fn AiVCellNoise4(p: AtVector, t: f32) -> AtVector;
}
