use ::std::os::raw::{c_char, c_int, c_void};

/// Used by dynamically-linked nodes to return node info
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtNodeLib {
    /// type of node (\\c AI_NODE_SHADER, \\c AI_NODE_CAMERA, etc)
    pub node_type: c_int,
    /// output type for shader nodes (\\c AI_TYPE_RGB, etc)
    pub output_type: u8,
    /// name of this plug-in node (\"lambert\", etc)
    pub name: *const c_char,
    /// pointer to this node's methods
    pub methods: *const c_void,
    /// Arnold version that this node was compiled against
    pub version: [c_char; 32usize],
}
extern "C" {
    pub fn AiLoadPlugins(directory: *const c_char);
}
