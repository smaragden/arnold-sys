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
    #[doc = " \\name Lighting Functions"]
    #[doc = " \\{"]
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
}
extern "C" {
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
}
extern "C" {
    pub fn AiDirectDiffuse(N: *const AtVector, sg: *mut AtShaderGlobals) -> AtRGB;
}
extern "C" {
    pub fn AiIndirectDiffuse(
        N: *const AtVector,
        sg: *mut AtShaderGlobals,
        weight: *const AtRGB,
    ) -> AtRGB;
}
extern "C" {
    #[doc = " \\name BSDF Utility Functions"]
    #[doc = " \\{"]
    pub fn AiReflect(I: *const AtVector, N: *const AtVector) -> AtVector;
}
extern "C" {
    pub fn AiReflectWithDerivs(I: *const AtVectorDv, N: *const AtVectorDv) -> AtVectorDv;
}
extern "C" {
    pub fn AiReflectSafe(I: *const AtVector, N: *const AtVector, Ng: *const AtVector) -> AtVector;
}
extern "C" {
    pub fn AiRefract(
        I: *const AtVector,
        N: *const AtVector,
        T: *mut AtVector,
        n1: f32,
        n2: f32,
    ) -> bool;
}
extern "C" {
    pub fn AiRefractWithDerivs(
        I: *const AtVectorDv,
        N: *const AtVectorDv,
        T: *mut AtVectorDv,
        n1: f32,
        n2: f32,
    ) -> bool;
}
extern "C" {
    pub fn AiSchlickFresnel(N: *const AtVector, Rd: *const AtVector, Krn: f32) -> f32;
}
extern "C" {
    pub fn AiSchlickFresnelRGB(N: *const AtVector, Rd: *const AtVector, Krn: *const AtRGB)
        -> AtRGB;
}
extern "C" {
    pub fn AiConductorFresnel(
        N: *const AtVector,
        Rd: *const AtVector,
        n: *const AtRGB,
        k: *const AtRGB,
    ) -> AtRGB;
}
extern "C" {
    pub fn AiDielectricFresnel(N: *const AtVector, Rd: *const AtVector, eta: f32) -> f32;
}
extern "C" {
    pub fn AiArtisticToConductorFresnel(
        reflectivity: *const AtRGB,
        edgetint: *const AtRGB,
        n: *mut AtRGB,
        k: *mut AtRGB,
    );
}
extern "C" {
    pub fn AiFaceForward(N: *mut AtVector, I: *const AtVector);
}
#[doc = " This function pointer points to float-based bump-mapping function for"]
#[doc = " use by AiShaderGlobalsEvaluateBump()."]
#[doc = ""]
#[doc = " This function would return the displacement height for the given shading context."]
#[doc = " This function would be called three times (on three different shading contexts)."]
#[doc = " The returned displacements would be used to generate a triangle from which a"]
#[doc = " normal is calculated."]
#[doc = ""]
#[doc = " \\param sg    the current shading context"]
#[doc = " \\param data  user-defined data pointer"]
#[doc = " \\return bump/displacement height for the current shading context"]
pub type AtFloatBumpEvaluator = ::std::option::Option<
    unsafe extern "C" fn(sg: *mut AtShaderGlobals, data: *mut ::std::os::raw::c_void) -> f32,
>;
extern "C" {
    #[doc = " \\name Utility Functions"]
    #[doc = " \\{"]
    pub fn AiShaderGlobalsGetTriangle(
        sg: *const AtShaderGlobals,
        key: ::std::os::raw::c_int,
        p: *mut AtVector,
    ) -> bool;
}
extern "C" {
    pub fn AiShaderGlobalsGetVertexNormals(
        sg: *const AtShaderGlobals,
        key: ::std::os::raw::c_int,
        n: *mut AtVector,
    ) -> bool;
}
extern "C" {
    pub fn AiShaderGlobalsGetVertexUVs(
        sg: *const AtShaderGlobals,
        uvset: AtString,
        uv: *mut AtVector2,
    ) -> bool;
}
extern "C" {
    pub fn AiShaderGlobalsGetPolygon(
        sg: *const AtShaderGlobals,
        key: ::std::os::raw::c_int,
        p: *mut AtVector,
    ) -> u32;
}
extern "C" {
    pub fn AiShaderGlobalsGetUniformID(sg: *const AtShaderGlobals) -> u32;
}
extern "C" {
    pub fn AiShaderGlobalsGetPositionAtTime(
        sg: *const AtShaderGlobals,
        time: f32,
        P: *mut AtVector,
        N: *mut AtVector,
        Ng: *mut AtVector,
        Ns: *mut AtVector,
    );
}
extern "C" {
    pub fn AiShaderGlobalsGetPixelMotionVector(
        sg: *const AtShaderGlobals,
        time0: f32,
        time1: f32,
    ) -> AtVector2;
}
extern "C" {
    pub fn AiShaderGlobalsGetBBoxLocal(sg: *const AtShaderGlobals) -> AtBBox;
}
extern "C" {
    pub fn AiShaderGlobalsGetBBoxWorld(sg: *const AtShaderGlobals) -> AtBBox;
}
extern "C" {
    pub fn AiShaderGlobalsGetShader(sg: *const AtShaderGlobals) -> *mut AtNode;
}
extern "C" {
    pub fn AiShaderGlobalsGetSelectedOutput(sg: *const AtShaderGlobals) -> i32;
}
extern "C" {
    pub fn AiShaderGlobalsTransformNormal(
        sg: *const AtShaderGlobals,
        N: AtVector,
        space: ::std::os::raw::c_int,
    ) -> AtVector;
}
extern "C" {
    pub fn AiShaderGlobalsTransformPoint(
        sg: *const AtShaderGlobals,
        P: AtVector,
        space: ::std::os::raw::c_int,
    ) -> AtVector;
}
extern "C" {
    pub fn AiShaderGlobalsTransformVector(
        sg: *const AtShaderGlobals,
        V: AtVector,
        space: ::std::os::raw::c_int,
    ) -> AtVector;
}
extern "C" {
    pub fn AiShaderGlobalsSetTraceSet(sg: *mut AtShaderGlobals, set: AtString, inclusive: bool);
}
extern "C" {
    pub fn AiShaderGlobalsUnsetTraceSet(sg: *mut AtShaderGlobals);
}
extern "C" {
    pub fn AiShaderGlobalsQuickAlloc(
        sg: *const AtShaderGlobals,
        size: u32,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn AiShaderGlobalsEvaluateBump(
        sg: *mut AtShaderGlobals,
        bump_func: AtFloatBumpEvaluator,
        data: *mut ::std::os::raw::c_void,
    ) -> AtVector;
}
extern "C" {
    pub fn AiShaderGlobalsStochasticOpacity(
        sg: *mut AtShaderGlobals,
        opacity: *const AtRGB,
    ) -> AtRGB;
}
extern "C" {
    pub fn AiShaderGlobalsArea(sg: *const AtShaderGlobals) -> f32;
}
extern "C" {
    pub fn AiShaderGlobalsEdgeLength(sg: *const AtShaderGlobals) -> f32;
}
extern "C" {
    pub fn AiWireframe(
        sg: *const AtShaderGlobals,
        line_width: f32,
        raster_space: bool,
        edge_type: ::std::os::raw::c_int,
    ) -> f32;
}
extern "C" {
    pub fn AiShaderGlobalsIsObjectMatte(sg: *const AtShaderGlobals) -> bool;
}
extern "C" {
    #[doc = " \\name Camera Information"]
    #[doc = " \\{"]
    pub fn AiCameraGetShutterStart() -> f32;
}
extern "C" {
    pub fn AiCameraGetShutterEnd() -> f32;
}
extern "C" {
    pub fn AiCameraToWorldMatrix(node: *const AtNode, time: f32, out: *mut AtMatrix);
}
extern "C" {
    pub fn AiWorldToCameraMatrix(node: *const AtNode, time: f32, out: *mut AtMatrix);
}
extern "C" {
    pub fn AiWorldToScreenMatrix(node: *const AtNode, time: f32, out: *mut AtMatrix);
}
extern "C" {
    #[doc = " \\name Environment Mappings"]
    #[doc = " \\{"]
    pub fn AiMappingMirroredBall(dir: *const AtVector, u: *mut f32, v: *mut f32);
}
extern "C" {
    pub fn AiMappingAngularMap(dir: *const AtVector, u: *mut f32, v: *mut f32);
}
extern "C" {
    pub fn AiMappingLatLong(dir: *const AtVector, u: *mut f32, v: *mut f32);
}
extern "C" {
    #[doc = " \\name Derivatives for Environment Mappings"]
    #[doc = " \\{"]
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
}
extern "C" {
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
}
extern "C" {
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