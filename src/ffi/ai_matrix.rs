use super::ai_vector::{AtHPoint, AtVector};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtMatrix {
    pub data: [[f32; 4usize]; 4usize],
}
extern "C" {
    pub fn AiM4Identity() -> AtMatrix;
}
extern "C" {
    pub fn AiM4Translation(t: *const AtVector) -> AtMatrix;
}
extern "C" {
    pub fn AiM4RotationX(x: f32) -> AtMatrix;
}
extern "C" {
    pub fn AiM4RotationY(y: f32) -> AtMatrix;
}
extern "C" {
    pub fn AiM4RotationZ(z: f32) -> AtMatrix;
}
extern "C" {
    pub fn AiM4Scaling(s: *const AtVector) -> AtMatrix;
}
extern "C" {
    pub fn AiM4Frame(
        o: *const AtVector,
        u: *const AtVector,
        v: *const AtVector,
        w: *const AtVector,
    ) -> AtMatrix;
}
extern "C" {
    pub fn AiM4PointByMatrixMult(m: *const AtMatrix, pin: *const AtVector) -> AtVector;
}
extern "C" {
    pub fn AiM4HPointByMatrixMult(m: *const AtMatrix, pin: *const AtHPoint) -> AtHPoint;
}
extern "C" {
    pub fn AiM4VectorByMatrixMult(m: *const AtMatrix, vin: *const AtVector) -> AtVector;
}
extern "C" {
    pub fn AiM4VectorByMatrixTMult(m: *const AtMatrix, vin: *const AtVector) -> AtVector;
}
extern "C" {
    pub fn AiM4Mult(ma: *const AtMatrix, mb: *const AtMatrix) -> AtMatrix;
}
extern "C" {
    pub fn AiM4Transpose(min: *const AtMatrix) -> AtMatrix;
}
extern "C" {
    pub fn AiM4Invert(min: *const AtMatrix) -> AtMatrix;
}
extern "C" {
    pub fn AiM4Determinant(m: *const AtMatrix) -> f32;
}
extern "C" {
    pub fn AiM4Lerp(t: f32, ma: *const AtMatrix, mb: *const AtMatrix) -> AtMatrix;
}
extern "C" {
    pub fn AiM4IsIdentity(m: *const AtMatrix) -> bool;
}
extern "C" {
    pub fn AiM4IsSingular(m: *const AtMatrix) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_ZL14AI_M4_IDENTITY"]
    pub static AI_M4_IDENTITY: AtMatrix;
}
extern "C" {
    #[link_name = "\u{1}_ZL10AI_M4_ZERO"]
    pub static AI_M4_ZERO: AtMatrix;
}
