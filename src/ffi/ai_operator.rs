use super::{ai_array::AtArray, ai_nodes::AtNode, ai_string::AtString, ai_universe::AtUniverse};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtCookContext {
    _unused: [u8; 0],
}
#[doc = " \\defgroup ai_operator Operator API"]
#[doc = ""]
#[doc = " Operator node creation and operation."]
#[doc = ""]
#[doc = " \\details"]
#[doc = " This API is used to create graphs of operator nodes which can procedurally perform scene"]
#[doc = " inspection, creation, and late binding modifications at render time."]
#[doc = ""]
#[doc = " The operator graph defines a non-destructive set of operations, where the graph is"]
#[doc = " evaluated from a given node in the graph, denoted as target operator. The target operator"]
#[doc = " is defined in the options and can be set using \\ref AiOpSetTarget. Any graphs not connected"]
#[doc = " to the target operator will be ignored."]
#[doc = ""]
#[doc = " An operator that is connected to the target will be initialized and cooked if it needs"]
#[doc = " to do any work. That is determined based on the presence of a selection parameter and"]
#[doc = " if the operator is enabled."]
#[doc = " An enabled operator without a selection will always be initialized and cooked at least"]
#[doc = " once, otherwise it's only initialized and cooked if the selection expression matches one"]
#[doc = " or more nodes in the scene."]
#[doc = ""]
#[doc = " Operators can recursively create other operator nodes and optionally pass child data"]
#[doc = " to them (see \\ref AiOpSetChildData. Operators can also declare their own user data,"]
#[doc = " where it is initialized/used in \\ref AtOpInit, used in \\ref AtOpCook, and deleted in"]
#[doc = " \\ref AtOpCleanup."]
#[doc = " Note that no information flows through the graph."]
#[doc = ""]
#[doc = " Operators are evaluated during the pre-render initialization process which is done in parallel."]
#[doc = " It is therefore important that the code in an operator node is designed to be re-entrant."]
#[doc = ""]
#[doc = " \\{"]
pub type AtOpCleanupChildData =
    ::std::option::Option<unsafe extern "C" fn(child_data: *mut ::std::os::raw::c_void) -> bool>;
extern "C" {
    pub fn AiOpSetTarget(universe: *mut AtUniverse, node: *mut AtNode) -> bool;
}
extern "C" {
    pub fn AiOpGetTarget(universe: *const AtUniverse) -> *mut AtNode;
}
extern "C" {
    pub fn AiOpGetInputs(op: *mut AtNode) -> *mut AtArray;
}
extern "C" {
    pub fn AiOpLink(from: *mut AtNode, to: *mut AtNode, index: ::std::os::raw::c_int) -> bool;
}
extern "C" {
    pub fn AiOpUnlinkInputByIndex(to: *mut AtNode, index: ::std::os::raw::c_uint) -> bool;
}
extern "C" {
    pub fn AiOpUnlink(from: *mut AtNode, to: *mut AtNode) -> bool;
}
extern "C" {
    pub fn AiOpSetChildData(
        op: *mut AtNode,
        child_data: *mut ::std::os::raw::c_void,
        cleanup: AtOpCleanupChildData,
    );
}
extern "C" {
    pub fn AiOpMatchNodeSelection(
        node: *mut AtNode,
        selection: AtString,
        relative: bool,
        target: *mut AtNode,
    ) -> bool;
}
extern "C" {
    pub fn AiOpCookContextGetCookScope(cook_context: *mut AtCookContext) -> *mut AtNode;
}
#[doc = " Operator init method."]
#[doc = ""]
#[doc = " This method will be called first if the operator needs to do any work to perform any"]
#[doc = " initialization required by the operator."]
#[doc = ""]
#[doc = " The operator is only initialized once in a batch session but in an IPR session this"]
#[doc = " method is called every time the operator is dirtied. This method may be called"]
#[doc = " concurrently with other uses of the same operator plugin."]
#[doc = ""]
#[doc = " @param op         operator node (itself)"]
#[doc = " @param user_data  general-purpose, user-supplied data pointer that Arnold"]
#[doc = "                   will pass along to the other operator methods"]
#[doc = " @return           true if successful, false otherwise"]
pub type AtOpInit = ::std::option::Option<
    unsafe extern "C" fn(op: *mut AtNode, user_data: *mut *mut ::std::os::raw::c_void) -> bool,
>;
#[doc = " Operator cleanup method."]
#[doc = ""]
#[doc = " This method is called if the operator node is deleted or if the render session"]
#[doc = " has finished, where it should perform any cleanup required by the operator."]
#[doc = " Make sure to release any memory you allocated that is no longer needed by Arnold."]
#[doc = ""]
#[doc = " @param op         operator node (itself)"]
#[doc = " @param user_data  general-purpose, user-supplied data pointer as returned from"]
#[doc = "                   \\ref AtOpInit."]
#[doc = " @return           true if successful, false otherwise"]
pub type AtOpCleanup = ::std::option::Option<
    unsafe extern "C" fn(op: *mut AtNode, user_data: *mut ::std::os::raw::c_void) -> bool,
>;
#[doc = " Operator cook method which operates on the cooked node. The cooked node is either"]
#[doc = " the operator itself (no selection parameter), or any node in the scene which matches"]
#[doc = " the selection expression."]
#[doc = ""]
#[doc = " Therefore, this method is called one or more times if the operator needs to do any"]
#[doc = " work, depending on how many nodes it has to operate on. The operator may cook multiple"]
#[doc = " nodes concurrently."]
#[doc = ""]
#[doc = " This method may have access to user data and child data when cooking. The user data"]
#[doc = " is a general-purpose pointer that is used by this operator and is initialized and"]
#[doc = " cleaned up as part of the operator's life cycle. The child data is a general-purpose"]
#[doc = " pointer that can be passed to an operator after creating it, where the cleanup method"]
#[doc = " provided is called when the child data is cleaned up (see \\ref AtOpCleanupChildData)."]
#[doc = ""]
#[doc = " Selection expressions support wildcards when matching parameter names and selections"]
#[doc = " can match multiple parameters. The matched parameter names are accessible to allow"]
#[doc = " customizing the operator's behavior based on the matched parameters."]
#[doc = ""]
#[doc = " @param node             node being cooked"]
#[doc = " @param op               operator node (itself)"]
#[doc = " @param child_data       general-purpose data pointer that may be passed here by the"]
#[doc = "                         operator's creator"]
#[doc = " @param user_data        general-purpose, user-supplied data pointer as returned from"]
#[doc = "                         \\ref AtOpInit."]
#[doc = " @param matching_params  names of parameters that matched the selection expression"]
#[doc = " @param cook_context     context specific information about the cook (see e.g."]
#[doc = "                         \\ref AtOpCookContextGetCookReference)"]
#[doc = " @return                 true if successful, false otherwise"]
pub type AtOpCook = ::std::option::Option<
    unsafe extern "C" fn(
        node: *mut AtNode,
        op: *mut AtNode,
        child_data: *mut ::std::os::raw::c_void,
        user_data: *mut ::std::os::raw::c_void,
        matching_params: *const AtArray,
        cook_context: *mut AtCookContext,
    ) -> bool,
>;
#[doc = " Operator post cook method which is called once per operator instance after all the"]
#[doc = " cook calls for all operators are finished. The method is only called if an operator"]
#[doc = " was cooked by being part of a graph that was connected to the options or a procedural."]
#[doc = ""]
#[doc = " @param op         operator node (itself)"]
#[doc = " @param user_data  general-purpose, user-supplied data pointer as returned from"]
#[doc = "                   \\ref AtOpInit."]
#[doc = " @return           true if successful, false otherwise"]
pub type AtOpPostCook = ::std::option::Option<
    unsafe extern "C" fn(op: *mut AtNode, user_data: *mut ::std::os::raw::c_void) -> bool,
>;
#[doc = " Cleanup method for child data which is passed to other operators. The lifetime of"]
#[doc = " the child data may differ from the operator that created it so we need to provide"]
#[doc = " this function to perform the cleanup when calling \\ref AiOpSetChildData."]
#[doc = ""]
#[doc = " @param child_data  general-purpose data pointer that is passed to an operator"]
#[doc = " @return            true if successful, false otherwise"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtOperatorNodeMethods {
    pub Init: AtOpInit,
    pub Cleanup: AtOpCleanup,
    pub Cook: AtOpCook,
    pub PostCook: AtOpPostCook,
}
#[doc = " Operator function pointer entry-point symbol"]
#[doc = ""]
#[doc = " \\param[out] methods  List of operator methods"]
#[doc = " \\return              true upon success"]
pub type AtOpFuncPtr = ::std::option::Option<
    unsafe extern "C" fn(methods: *mut AtOperatorNodeMethods) -> ::std::os::raw::c_int,
>;
