#[doc = " Used by dynamically-linked nodes to return node info"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AtNodeLib {
    #[doc = "< type of node (\\c AI_NODE_SHADER, \\c AI_NODE_CAMERA, etc)"]
    pub node_type: ::std::os::raw::c_int,
    #[doc = "< output type for shader nodes (\\c AI_TYPE_RGB, etc)"]
    pub output_type: u8,
    #[doc = "< name of this plug-in node (\"lambert\", etc)"]
    pub name: *const ::std::os::raw::c_char,
    #[doc = "< pointer to this node's methods"]
    pub methods: *const ::std::os::raw::c_void,
    #[doc = "< Arnold version that this node was compiled against"]
    pub version: [::std::os::raw::c_char; 32usize],
}
extern "C" {
    pub fn AiLoadPlugins(directory: *const ::std::os::raw::c_char);
}
