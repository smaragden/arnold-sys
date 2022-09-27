use ::std::{
    option::Option,
    os::raw::{c_int, c_uint, c_void},
};

use super::{ai_array::AtArray, ai_nodes::AtNode, ai_string::AtString, ai_universe::AtUniverse};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtCookContext {
    _unused: [u8; 0],
}
/// \\defgroup ai_operator Operator API
///
/// Operator node creation and operation.
///
/// \\details
/// This API is used to create graphs of operator nodes which can procedurally perform scene
/// inspection, creation, and late binding modifications at render time.
///
/// The operator graph defines a non-destructive set of operations, where the graph is
/// evaluated from a given node in the graph, denoted as target operator. The target operator
/// is defined in the options and can be set using `AiOpSetTarget`. Any graphs not connected
/// to the target operator will be ignored.
///
/// An operator that is connected to the target will be initialized and cooked if it needs
/// to do any work. That is determined based on the presence of a selection parameter and
/// if the operator is enabled.
/// An enabled operator without a selection will always be initialized and cooked at least
/// once, otherwise it's only initialized and cooked if the selection expression matches one
/// or more nodes in the scene.
///
/// Operators can recursively create other operator nodes and optionally pass child data
/// to them (see `AiOpSetChildData`. Operators can also declare their own user data,
/// where it is initialized/used in `AtOpInit`, used in `AtOpCook`, and deleted in
/// `AtOpCleanup`.
/// Note that no information flows through the graph.
///
/// Operators are evaluated during the pre-render initialization process which is done in parallel.
/// It is therefore important that the code in an operator node is designed to be re-entrant.
///
/// \\{
pub type AtOpCleanupChildData = Option<unsafe extern "C" fn(child_data: *mut c_void) -> bool>;
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
    pub fn AiOpLink(from: *mut AtNode, to: *mut AtNode, index: c_int) -> bool;
}
extern "C" {
    pub fn AiOpUnlinkInputByIndex(to: *mut AtNode, index: c_uint) -> bool;
}
extern "C" {
    pub fn AiOpUnlink(from: *mut AtNode, to: *mut AtNode) -> bool;
}
extern "C" {
    pub fn AiOpSetChildData(
        op: *mut AtNode,
        child_data: *mut c_void,
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
/// Operator init method.
///
/// This method will be called first if the operator needs to do any work to perform any
/// initialization required by the operator.
///
/// The operator is only initialized once in a batch session but in an IPR session this
/// method is called every time the operator is dirtied. This method may be called
/// concurrently with other uses of the same operator plugin.
///
/// @param op         operator node (itself)
/// @param user_data  general-purpose, user-supplied data pointer that Arnold
///                   will pass along to the other operator methods
/// @return           true if successful, false otherwise
pub type AtOpInit =
    Option<unsafe extern "C" fn(op: *mut AtNode, user_data: *mut *mut c_void) -> bool>;
/// Operator cleanup method.
///
/// This method is called if the operator node is deleted or if the render session
/// has finished, where it should perform any cleanup required by the operator.
/// Make sure to release any memory you allocated that is no longer needed by Arnold.
///
/// @param op         operator node (itself)
/// @param user_data  general-purpose, user-supplied data pointer as returned from
///                   `AtOpInit`.
/// @return           true if successful, false otherwise
pub type AtOpCleanup =
    Option<unsafe extern "C" fn(op: *mut AtNode, user_data: *mut c_void) -> bool>;
/// Operator cook method which operates on the cooked node. The cooked node is either
/// the operator itself (no selection parameter), or any node in the scene which matches
/// the selection expression.
///
/// Therefore, this method is called one or more times if the operator needs to do any
/// work, depending on how many nodes it has to operate on. The operator may cook multiple
/// nodes concurrently.
///
/// This method may have access to user data and child data when cooking. The user data
/// is a general-purpose pointer that is used by this operator and is initialized and
/// cleaned up as part of the operator's life cycle. The child data is a general-purpose
/// pointer that can be passed to an operator after creating it, where the cleanup method
/// provided is called when the child data is cleaned up (see `AtOpCleanupChildData`).
///
/// Selection expressions support wildcards when matching parameter names and selections
/// can match multiple parameters. The matched parameter names are accessible to allow
/// customizing the operator's behavior based on the matched parameters.
///
/// @param node             node being cooked
/// @param op               operator node (itself)
/// @param child_data       general-purpose data pointer that may be passed here by the
///                         operator's creator
/// @param user_data        general-purpose, user-supplied data pointer as returned from
///                         `AtOpInit`.
/// @param matching_params  names of parameters that matched the selection expression
/// @param cook_context     context specific information about the cook (see e.g.
///                         `AtOpCookContextGetCookReference`)
/// @return                 true if successful, false otherwise
pub type AtOpCook = Option<
    unsafe extern "C" fn(
        node: *mut AtNode,
        op: *mut AtNode,
        child_data: *mut c_void,
        user_data: *mut c_void,
        matching_params: *const AtArray,
        cook_context: *mut AtCookContext,
    ) -> bool,
>;
/// Operator post cook method which is called once per operator instance after all the
/// cook calls for all operators are finished. The method is only called if an operator
/// was cooked by being part of a graph that was connected to the options or a procedural.
///
/// @param op         operator node (itself)
/// @param user_data  general-purpose, user-supplied data pointer as returned from
///                   `AtOpInit`.
/// @return           true if successful, false otherwise
pub type AtOpPostCook =
    Option<unsafe extern "C" fn(op: *mut AtNode, user_data: *mut c_void) -> bool>;
/// Cleanup method for child data which is passed to other operators. The lifetime of
/// the child data may differ from the operator that created it so we need to provide
/// this function to perform the cleanup when calling `AiOpSetChildData`.
///
/// @param child_data  general-purpose data pointer that is passed to an operator
/// @return            true if successful, false otherwise
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtOperatorNodeMethods {
    pub Init: AtOpInit,
    pub Cleanup: AtOpCleanup,
    pub Cook: AtOpCook,
    pub PostCook: AtOpPostCook,
}
/// Operator function pointer entry-point symbol
///
/// \\param[out] methods  List of operator methods
/// \\return              true upon success
pub type AtOpFuncPtr = Option<unsafe extern "C" fn(methods: *mut AtOperatorNodeMethods) -> c_int>;
