use ::std::{
    option::Option,
    os::raw::{c_int, c_uint, c_void},
};

use super::{ai_map::AtParamValueMap, ai_nodes::AtNode, ai_universe::AtUniverse};

/// Procedural init method.
///
/// This method will be called first and should perform any initialization required
/// by your procedural. You probably want to create new nodes inside this method
/// but you should return them through `AtProcGetNode` and correctly return
/// the number of created nodes from `AtProcNumNodes`, otherwise the behavior
/// is undefined. Alternatively, if you know ahead of time exactly how many nodes
/// you are going to create, you can create them in `AtProcGetNode` too.
///
/// This method may be called concurrently with other uses of the same procedural
/// plugin, unless \"options.enable_threaded_procedurals\" is off.
///
/// \\param      node      This is the procedural node itself
/// \\param[out] user_ptr  This is a general-purpose, user-supplied data pointer that
///                       Arnold will pass along to the other procedural methods.
/// \\return               true upon success
pub type AtProcInit =
    Option<unsafe extern "C" fn(node: *mut AtNode, user_ptr: *mut *mut c_void) -> c_int>;
/// Procedural cleanup method.
///
/// This method will be called last and should perform any cleanup required
/// by your procedural. Make sure you release any memory you allocated that is no
/// longer needed by Arnold.
///
/// This method may be called concurrently with other uses of the same procedural
/// plugin.
///
/// \\param node      This is the procedural node itself
/// \\param user_ptr  User data pointer, as returned from `AtProcInit`
/// \\return          true upon success
pub type AtProcCleanup =
    Option<unsafe extern "C" fn(node: *const AtNode, user_ptr: *mut c_void) -> c_int>;
/// Procedural node count method.
///
/// This method will be called after initialization and should report the exact
/// number of nodes to be created. Alternatively, when the total number of nodes is
/// not known beforehand, it might return -1, and then Arnold will call the
/// `AtProcGetNode` method until it returns NULL to indicate no more nodes are
/// available.
///
/// This method may be called concurrently with other uses of the same procedural
/// plugin.
///
/// \\param node      This is the procedural node itself
/// \\param user_ptr  User data pointer, as returned from `AtProcInit`
/// \\return          The number of nodes in the procedural
pub type AtProcNumNodes =
    Option<unsafe extern "C" fn(node: *const AtNode, user_ptr: *mut c_void) -> c_int>;
/// Procedural node fetching method.
///
/// This method will be called once for each node to be created (as determined by
/// `AtProcNumNodes`). Note that if you created any node in `AtProcInit`, they
/// also should be returned here, otherwise the behaviour would be undefined.
///
/// If -1 was returned by `AtProcNumNodes`, this method should return NULL when
/// all nodes have been returned and there are no more available.
///
/// This method may be called concurrently with other uses of the same procedural
/// plugin.
///
/// \\param node      This is the procedural node itself
/// \\param user_ptr  User data pointer, as returned from `AtProcInit`
/// \\param i         Node index, in the range 0 to `AtProcNumNodes` - 1
/// \\return          The i'th node in the procedural
pub type AtProcGetNode = Option<
    unsafe extern "C" fn(node: *const AtNode, user_ptr: *mut c_void, i: c_int) -> *mut AtNode,
>;
pub const AtProcViewportMode_AI_PROC_BOXES: AtProcViewportMode = 0;
pub const AtProcViewportMode_AI_PROC_POINTS: AtProcViewportMode = 1;
pub const AtProcViewportMode_AI_PROC_POLYGONS: AtProcViewportMode = 2;
/// Enum with the different modes available for a procedural viewport representation
pub type AtProcViewportMode = c_uint;
/// Procedural viewport representation method.
///
/// This method can be called to obtain a simplified representation of a procedural, made up of nodes that will be
/// created in the given universe.
///
/// This is an example implementation for a simple custom procedural:
/// ```c
/// procedural_viewport
/// {
///    if (mode == AI_PROC_BOXES)
///    {
///       AtNode* bbox_node = AiNode(universe, \"box\", \"bbox0\");
///       AiNodeSetVec(bbox_node, \"min\", -5, -5, -5);
///       AiNodeSetVec(bbox_node, \"max\", 5, 5, 5);
///    }
///    return AI_SUCCESS;
/// }
/// ```
///
/// \\param node              This is the procedural node itself
/// \\param universe          The universe where the new nodes will be created
/// \\param mode              The type of primitives used for the viewport representation
/// \\param params            List of optional parameters to be interpreted by the procedurals
/// \\return                  \\c AI_SUCCESS if no error, an error value otherwise
pub type AtProcViewport = Option<
    unsafe extern "C" fn(
        node: *const AtNode,
        universe: *mut AtUniverse,
        mode: AtProcViewportMode,
        params: *const AtParamValueMap,
    ) -> c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtProceduralNodeMethods {
    /// This is called before expanding the procedural
    pub Init: AtProcInit,
    /// This is called last and should clean up any (temporary) memory used by the procedural
    pub Cleanup: AtProcCleanup,
    /// This is called to find out how many nodes this procedural will generate
    pub NumNodes: AtProcNumNodes,
    /// This is called NumNodes times, once for each node the procedural creates
    pub GetNode: AtProcGetNode,
    /// This is called to get a viewport representation of the given procedural node
    pub ProceduralViewport: AtProcViewport,
}
/// Procedural function pointer entry-point symbol
///
/// A function pointer of this type can be set in the procedural funcptr parameter.
///
/// \\param[out] methods  List of procedural methods (some of which are optional) to be supplied by the user
/// \\return              true upon success
pub type AtProcFuncPtr =
    Option<unsafe extern "C" fn(methods: *mut AtProceduralNodeMethods) -> c_int>;
extern "C" {
    /// Procedural viewport representation method.
    ///
    /// Call this method to get a simplified representation of a procedural for a DCC viewport.
    /// The nodes are created in the given universe, and mode determines the type of representation
    /// (for example, bounding boxes, points, or polygons). The optional params allows you to pass in
    /// a variable number of paramater values to the method.
    ///
    /// This is an example of some code to get this representation from a procedural \"proc\":
    /// ```c
    /// // Create new universe to store the procedural representation
    /// AtUniverse* view_universe = AiUniverse();
    ///
    /// // Obtain bounding-box representation (one box for each object in the procedural)
    /// AtParamValueMap* params = AiParamValueMap();
    /// AiParamValueMapSetInt(params, AtString(\"param\"), 0);  // Example parameter
    /// AiProceduralViewport(proc, view_universe, AI_PROC_BOXES, params);
    /// AiParamValueMapDestroy(params);
    ///
    /// // After that, we can iterate over those nodes and get any kind of information
    /// AtNodeIterator* it = AiUniverseGetNodeIterator(view_universe, AI_NODE_SHAPE);
    /// while (!AiNodeIteratorFinished(it))
    /// {
    ///    AtNode* node = AiNodeIteratorGetNext(it);
    ///    printf(\"Node name: %s\\n\", AiNodeGetName(node));
    /// }
    /// AiNodeIteratorDestroy(it);
    /// ```
    ///
    /// Optional parameters allow further configuration of the representation:
    /// <table>
    /// <tr><th>Supported optional parameters
    /// <tr><td> *None yet*
    /// </table>
    ///
    /// \\param node              This is the procedural node itself
    /// \\param universe          The universe where the new nodes will be created
    /// \\param mode              The type of primitives used for the viewport representation
    /// \\param params            List of optional parameters to be interpreted by the procedurals
    /// \\return                  AI_SUCCESS if no error, an error value otherwise
    pub fn AiProceduralViewport(
        node: *const AtNode,
        universe: *mut AtUniverse,
        mode: AtProcViewportMode,
        params: *const AtParamValueMap,
    ) -> c_int;
}
