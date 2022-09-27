use ::std::{
    option::Option,
    os::raw::{c_int, c_void},
};

use super::{
    ai_bbox::AtBBox,
    ai_color::AtRGB,
    ai_matrix::AtMatrix,
    ai_nodes::AtNode,
    ai_shaderglobals::AtShaderGlobals,
    ai_string::AtString,
    ai_vector::{AtVector, AtVector2, AtVectorDv},
};

pub const AI_WORLD_TO_OBJECT: u32 = 1;
pub const AI_OBJECT_TO_WORLD: u32 = 2;
pub const AI_WIREFRAME_TRIANGLES: u32 = 0;
pub const AI_WIREFRAME_POLYGONS: u32 = 1;
pub const AI_WIREFRAME_PATCHES: u32 = 2;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtSampler {
    _unused: [u8; 0],
}
extern "C" {
    /// \\name Lighting Functions
    /// \\{
    pub fn AiOcclusion(
        N: *const AtVector,
        Ng: *const AtVector,
        sg: *mut AtShaderGlobals,
        mint: f32,
        maxt: f32,
        spread: f32,
        falloff: f32,
        sampler: *const AtSampler,
        Nbent: *mut AtVector,
    ) -> AtRGB;

    pub fn AiSelfOcclusion(
        N: *const AtVector,
        Ng: *const AtVector,
        sg: *mut AtShaderGlobals,
        mint: f32,
        maxt: f32,
        spread: f32,
        falloff: f32,
        sampler: *const AtSampler,
        Nbent: *mut AtVector,
    ) -> AtRGB;

    pub fn AiDirectDiffuse(N: *const AtVector, sg: *mut AtShaderGlobals) -> AtRGB;

    pub fn AiIndirectDiffuse(
        N: *const AtVector,
        sg: *mut AtShaderGlobals,
        weight: *const AtRGB,
    ) -> AtRGB;

    /// \\name BSDF Utility Functions
    /// \\{
    pub fn AiReflect(I: *const AtVector, N: *const AtVector) -> AtVector;

    pub fn AiReflectWithDerivs(I: *const AtVectorDv, N: *const AtVectorDv) -> AtVectorDv;

    pub fn AiReflectSafe(I: *const AtVector, N: *const AtVector, Ng: *const AtVector) -> AtVector;

    pub fn AiRefract(
        I: *const AtVector,
        N: *const AtVector,
        T: *mut AtVector,
        n1: f32,
        n2: f32,
    ) -> bool;

    pub fn AiRefractWithDerivs(
        I: *const AtVectorDv,
        N: *const AtVectorDv,
        T: *mut AtVectorDv,
        n1: f32,
        n2: f32,
    ) -> bool;

    pub fn AiSchlickFresnel(N: *const AtVector, Rd: *const AtVector, Krn: f32) -> f32;

    pub fn AiSchlickFresnelRGB(N: *const AtVector, Rd: *const AtVector, Krn: *const AtRGB)
        -> AtRGB;

    pub fn AiConductorFresnel(
        N: *const AtVector,
        Rd: *const AtVector,
        n: *const AtRGB,
        k: *const AtRGB,
    ) -> AtRGB;

    pub fn AiDielectricFresnel(N: *const AtVector, Rd: *const AtVector, eta: f32) -> f32;

    pub fn AiArtisticToConductorFresnel(
        reflectivity: *const AtRGB,
        edgetint: *const AtRGB,
        n: *mut AtRGB,
        k: *mut AtRGB,
    );

    pub fn AiFaceForward(N: *mut AtVector, I: *const AtVector);
}
/// This function pointer points to float-based bump-mapping function for
/// use by AiShaderGlobalsEvaluateBump().
///
/// This function would return the displacement height for the given shading context.
/// This function would be called three times (on three different shading contexts).
/// The returned displacements would be used to generate a triangle from which a
/// normal is calculated.
///
/// \\param sg    the current shading context
/// \\param data  user-defined data pointer
/// \\return bump/displacement height for the current shading context
pub type AtFloatBumpEvaluator =
    Option<unsafe extern "C" fn(sg: *mut AtShaderGlobals, data: *mut c_void) -> f32>;
extern "C" {
    /// \\name Utility Functions
    /// \\{
    pub fn AiShaderGlobalsGetTriangle(
        sg: *const AtShaderGlobals,
        key: c_int,
        p: *mut AtVector,
    ) -> bool;

    pub fn AiShaderGlobalsGetVertexNormals(
        sg: *const AtShaderGlobals,
        key: c_int,
        n: *mut AtVector,
    ) -> bool;

    pub fn AiShaderGlobalsGetVertexUVs(
        sg: *const AtShaderGlobals,
        uvset: AtString,
        uv: *mut AtVector2,
    ) -> bool;

    pub fn AiShaderGlobalsGetPolygon(
        sg: *const AtShaderGlobals,
        key: c_int,
        p: *mut AtVector,
    ) -> u32;

    pub fn AiShaderGlobalsGetUniformID(sg: *const AtShaderGlobals) -> u32;

    pub fn AiShaderGlobalsGetPositionAtTime(
        sg: *const AtShaderGlobals,
        time: f32,
        P: *mut AtVector,
        N: *mut AtVector,
        Ng: *mut AtVector,
        Ns: *mut AtVector,
    );

    pub fn AiShaderGlobalsGetPixelMotionVector(
        sg: *const AtShaderGlobals,
        time0: f32,
        time1: f32,
    ) -> AtVector2;

    pub fn AiShaderGlobalsGetBBoxLocal(sg: *const AtShaderGlobals) -> AtBBox;

    pub fn AiShaderGlobalsGetBBoxWorld(sg: *const AtShaderGlobals) -> AtBBox;

    pub fn AiShaderGlobalsGetShader(sg: *const AtShaderGlobals) -> *mut AtNode;

    pub fn AiShaderGlobalsGetSelectedOutput(sg: *const AtShaderGlobals) -> i32;

    pub fn AiShaderGlobalsTransformNormal(
        sg: *const AtShaderGlobals,
        N: AtVector,
        space: c_int,
    ) -> AtVector;

    pub fn AiShaderGlobalsTransformPoint(
        sg: *const AtShaderGlobals,
        P: AtVector,
        space: c_int,
    ) -> AtVector;

    pub fn AiShaderGlobalsTransformVector(
        sg: *const AtShaderGlobals,
        V: AtVector,
        space: c_int,
    ) -> AtVector;

    pub fn AiShaderGlobalsSetTraceSet(sg: *mut AtShaderGlobals, set: AtString, inclusive: bool);

    pub fn AiShaderGlobalsUnsetTraceSet(sg: *mut AtShaderGlobals);

    pub fn AiShaderGlobalsQuickAlloc(sg: *const AtShaderGlobals, size: u32) -> *mut c_void;

    pub fn AiShaderGlobalsEvaluateBump(
        sg: *mut AtShaderGlobals,
        bump_func: AtFloatBumpEvaluator,
        data: *mut c_void,
    ) -> AtVector;

    pub fn AiShaderGlobalsStochasticOpacity(
        sg: *mut AtShaderGlobals,
        opacity: *const AtRGB,
    ) -> AtRGB;

    pub fn AiShaderGlobalsArea(sg: *const AtShaderGlobals) -> f32;

    pub fn AiShaderGlobalsEdgeLength(sg: *const AtShaderGlobals) -> f32;

    pub fn AiWireframe(
        sg: *const AtShaderGlobals,
        line_width: f32,
        raster_space: bool,
        edge_type: c_int,
    ) -> f32;

    pub fn AiShaderGlobalsIsObjectMatte(sg: *const AtShaderGlobals) -> bool;

    /// \\name Camera Information
    /// \\{
    pub fn AiCameraGetShutterStart() -> f32;

    pub fn AiCameraGetShutterEnd() -> f32;

    pub fn AiCameraToWorldMatrix(node: *const AtNode, time: f32, out: *mut AtMatrix);

    pub fn AiWorldToCameraMatrix(node: *const AtNode, time: f32, out: *mut AtMatrix);

    pub fn AiWorldToScreenMatrix(node: *const AtNode, time: f32, out: *mut AtMatrix);

    /// \\name Environment Mappings
    /// \\{
    pub fn AiMappingMirroredBall(dir: *const AtVector, u: *mut f32, v: *mut f32);

    pub fn AiMappingAngularMap(dir: *const AtVector, u: *mut f32, v: *mut f32);

    pub fn AiMappingLatLong(dir: *const AtVector, u: *mut f32, v: *mut f32);

    /// \\name Derivatives for Environment Mappings
    /// \\{
    pub fn AiMappingMirroredBallDerivs(
        dir: *const AtVector,
        dDdx: *const AtVector,
        dDdy: *const AtVector,
        u: *mut f32,
        v: *mut f32,
        dudx: *mut f32,
        dudy: *mut f32,
        dvdx: *mut f32,
        dvdy: *mut f32,
    );

    pub fn AiMappingAngularMapDerivs(
        dir: *const AtVector,
        dDdx: *const AtVector,
        dDdy: *const AtVector,
        u: *mut f32,
        v: *mut f32,
        dudx: *mut f32,
        dudy: *mut f32,
        dvdx: *mut f32,
        dvdy: *mut f32,
    );

    pub fn AiMappingLatLongDerivs(
        dir: *const AtVector,
        dDdx: *const AtVector,
        dDdy: *const AtVector,
        u: *mut f32,
        v: *mut f32,
        dudx: *mut f32,
        dudy: *mut f32,
        dvdx: *mut f32,
        dvdy: *mut f32,
    );
}
