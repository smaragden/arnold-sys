use ::std::option::Option;

use super::{ai_nodes::AtNode, ai_shaderglobals::AtShaderGlobals};

#[doc = " Shader Node methods structure"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtShaderNodeMethods {
    pub Evaluate: Option<unsafe extern "C" fn(arg1: *mut AtNode, arg2: *mut AtShaderGlobals)>,
}
extern "C" {
    #[doc = " \\name Node Methods"]
    #[doc = " \\{"]
    pub fn AiShaderEvaluate(node: *mut AtNode, sg: *mut AtShaderGlobals);
}
