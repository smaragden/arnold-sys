use super::{ai_map::AtParamValueMap, ai_nodes::AtNode, ai_universe::AtUniverse};

#[doc = " Procedural init method."]
#[doc = ""]
#[doc = " This method will be called first and should perform any initialization required"]
#[doc = " by your procedural. You probably want to create new nodes inside this method"]
#[doc = " but you should return them through \\ref AtProcGetNode and correctly return"]
#[doc = " the number of created nodes from \\ref AtProcNumNodes, otherwise the behavior"]
#[doc = " is undefined. Alternatively, if you know ahead of time exactly how many nodes"]
#[doc = " you are going to create, you can create them in \\ref AtProcGetNode too."]
#[doc = ""]
#[doc = " This method may be called concurrently with other uses of the same procedural"]
#[doc = " plugin, unless \"options.enable_threaded_procedurals\" is off."]
#[doc = ""]
#[doc = " \\param      node      This is the procedural node itself"]
#[doc = " \\param[out] user_ptr  This is a general-purpose, user-supplied data pointer that"]
#[doc = "                       Arnold will pass along to the other procedural methods."]
#[doc = " \\return               true upon success"]
pub type AtProcInit = ::std::option::Option<
    unsafe extern "C" fn(
        node: *mut AtNode,
        user_ptr: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
#[doc = " Procedural cleanup method."]
#[doc = ""]
#[doc = " This method will be called last and should perform any cleanup required"]
#[doc = " by your procedural. Make sure you release any memory you allocated that is no"]
#[doc = " longer needed by Arnold."]
#[doc = ""]
#[doc = " This method may be called concurrently with other uses of the same procedural"]
#[doc = " plugin."]
#[doc = ""]
#[doc = " \\param node      This is the procedural node itself"]
#[doc = " \\param user_ptr  User data pointer, as returned from \\ref AtProcInit"]
#[doc = " \\return          true upon success"]
pub type AtProcCleanup = ::std::option::Option<
    unsafe extern "C" fn(
        node: *const AtNode,
        user_ptr: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
#[doc = " Procedural node count method."]
#[doc = ""]
#[doc = " This method will be called after initialization and should report the exact"]
#[doc = " number of nodes to be created. Alternatively, when the total number of nodes is"]
#[doc = " not known beforehand, it might return -1, and then Arnold will call the"]
#[doc = " \\ref AtProcGetNode method until it returns NULL to indicate no more nodes are"]
#[doc = " available."]
#[doc = ""]
#[doc = " This method may be called concurrently with other uses of the same procedural"]
#[doc = " plugin."]
#[doc = ""]
#[doc = " \\param node      This is the procedural node itself"]
#[doc = " \\param user_ptr  User data pointer, as returned from \\ref AtProcInit"]
#[doc = " \\return          The number of nodes in the procedural"]
pub type AtProcNumNodes = ::std::option::Option<
    unsafe extern "C" fn(
        node: *const AtNode,
        user_ptr: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
#[doc = " Procedural node fetching method."]
#[doc = ""]
#[doc = " This method will be called once for each node to be created (as determined by"]
#[doc = " \\ref AtProcNumNodes). Note that if you created any node in \\ref AtProcInit, they"]
#[doc = " also should be returned here, otherwise the behaviour would be undefined."]
#[doc = ""]
#[doc = " If -1 was returned by \\ref AtProcNumNodes, this method should return NULL when"]
#[doc = " all nodes have been returned and there are no more available."]
#[doc = ""]
#[doc = " This method may be called concurrently with other uses of the same procedural"]
#[doc = " plugin."]
#[doc = ""]
#[doc = " \\param node      This is the procedural node itself"]
#[doc = " \\param user_ptr  User data pointer, as returned from \\ref AtProcInit"]
#[doc = " \\param i         Node index, in the range 0 to \\ref AtProcNumNodes - 1"]
#[doc = " \\return          The i'th node in the procedural"]
pub type AtProcGetNode = ::std::option::Option<
    unsafe extern "C" fn(
        node: *const AtNode,
        user_ptr: *mut ::std::os::raw::c_void,
        i: ::std::os::raw::c_int,
    ) -> *mut AtNode,
>;
pub const AtProcViewportMode_AI_PROC_BOXES: AtProcViewportMode = 0;
pub const AtProcViewportMode_AI_PROC_POINTS: AtProcViewportMode = 1;
pub const AtProcViewportMode_AI_PROC_POLYGONS: AtProcViewportMode = 2;
#[doc = " Enum with the different modes available for a procedural viewport representation"]
pub type AtProcViewportMode = ::std::os::raw::c_uint;
#[doc = " Procedural viewport representation method."]
#[doc = ""]
#[doc = " This method can be called to obtain a simplified representation of a procedural, made up of nodes that will be"]
#[doc = " created in the given universe."]
#[doc = ""]
#[doc = " This is an example implementation for a simple custom procedural:"]
#[doc = " \\code"]
#[doc = " procedural_viewport"]
#[doc = " {"]
#[doc = "    if (mode == AI_PROC_BOXES)"]
#[doc = "    {"]
#[doc = "       AtNode* bbox_node = AiNode(universe, \"box\", \"bbox0\");"]
#[doc = "       AiNodeSetVec(bbox_node, \"min\", -5, -5, -5);"]
#[doc = "       AiNodeSetVec(bbox_node, \"max\", 5, 5, 5);"]
#[doc = "    }"]
#[doc = "    return AI_SUCCESS;"]
#[doc = " }"]
#[doc = " \\endcode"]
#[doc = ""]
#[doc = " \\param node              This is the procedural node itself"]
#[doc = " \\param universe          The universe where the new nodes will be created"]
#[doc = " \\param mode              The type of primitives used for the viewport representation"]
#[doc = " \\param params            List of optional parameters to be interpreted by the procedurals"]
#[doc = " \\return                  \\c AI_SUCCESS if no error, an error value otherwise"]
pub type AtProcViewport = ::std::option::Option<
    unsafe extern "C" fn(
        node: *const AtNode,
        universe: *mut AtUniverse,
        mode: AtProcViewportMode,
        params: *const AtParamValueMap,
    ) -> ::std::os::raw::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtProceduralNodeMethods {
    #[doc = "< This is called before expanding the procedural"]
    pub Init: AtProcInit,
    #[doc = "< This is called last and should clean up any (temporary) memory used by the procedural"]
    pub Cleanup: AtProcCleanup,
    #[doc = "< This is called to find out how many nodes this procedural will generate"]
    pub NumNodes: AtProcNumNodes,
    #[doc = "< This is called NumNodes times, once for each node the procedural creates"]
    pub GetNode: AtProcGetNode,
    #[doc = "< This is called to get a viewport representation of the given procedural node"]
    pub ProceduralViewport: AtProcViewport,
}
#[doc = " Procedural function pointer entry-point symbol"]
#[doc = ""]
#[doc = " A function pointer of this type can be set in the procedural funcptr parameter."]
#[doc = ""]
#[doc = " \\param[out] methods  List of procedural methods (some of which are optional) to be supplied by the user"]
#[doc = " \\return              true upon success"]
pub type AtProcFuncPtr = ::std::option::Option<
    unsafe extern "C" fn(methods: *mut AtProceduralNodeMethods) -> ::std::os::raw::c_int,
>;
extern "C" {
    #[doc = " Procedural viewport representation method."]
    #[doc = ""]
    #[doc = " Call this method to get a simplified representation of a procedural for a DCC viewport."]
    #[doc = " The nodes are created in the given universe, and mode determines the type of representation"]
    #[doc = " (for example, bounding boxes, points, or polygons). The optional params allows you to pass in"]
    #[doc = " a variable number of paramater values to the method."]
    #[doc = ""]
    #[doc = " This is an example of some code to get this representation from a procedural \"proc\":"]
    #[doc = " \\code"]
    #[doc = " // Create new universe to store the procedural representation"]
    #[doc = " AtUniverse* view_universe = AiUniverse();"]
    #[doc = ""]
    #[doc = " // Obtain bounding-box representation (one box for each object in the procedural)"]
    #[doc = " AtParamValueMap* params = AiParamValueMap();"]
    #[doc = " AiParamValueMapSetInt(params, AtString(\"param\"), 0);  // Example parameter"]
    #[doc = " AiProceduralViewport(proc, view_universe, AI_PROC_BOXES, params);"]
    #[doc = " AiParamValueMapDestroy(params);"]
    #[doc = ""]
    #[doc = " // After that, we can iterate over those nodes and get any kind of information"]
    #[doc = " AtNodeIterator* it = AiUniverseGetNodeIterator(view_universe, AI_NODE_SHAPE);"]
    #[doc = " while (!AiNodeIteratorFinished(it))"]
    #[doc = " {"]
    #[doc = "    AtNode* node = AiNodeIteratorGetNext(it);"]
    #[doc = "    printf(\"Node name: %s\\n\", AiNodeGetName(node));"]
    #[doc = " }"]
    #[doc = " AiNodeIteratorDestroy(it);"]
    #[doc = " \\endcode"]
    #[doc = ""]
    #[doc = " Optional parameters allow further configuration of the representation:"]
    #[doc = " <table>"]
    #[doc = " <tr><th>Supported optional parameters"]
    #[doc = " <tr><td> *None yet*"]
    #[doc = " </table>"]
    #[doc = ""]
    #[doc = " \\param node              This is the procedural node itself"]
    #[doc = " \\param universe          The universe where the new nodes will be created"]
    #[doc = " \\param mode              The type of primitives used for the viewport representation"]
    #[doc = " \\param params            List of optional parameters to be interpreted by the procedurals"]
    #[doc = " \\return                  AI_SUCCESS if no error, an error value otherwise"]
    pub fn AiProceduralViewport(
        node: *const AtNode,
        universe: *mut AtUniverse,
        mode: AtProcViewportMode,
        params: *const AtParamValueMap,
    ) -> ::std::os::raw::c_int;
}
