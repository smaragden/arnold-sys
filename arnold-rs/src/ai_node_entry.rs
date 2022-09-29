use bitflags::bitflags;

bitflags! {
    pub struct NodeTypes: u32 {
        const AI_NODE_UNDEFINED = arnold_sys::ai_node_entry::AI_NODE_UNDEFINED;
        const AI_NODE_OPTIONS = arnold_sys::ai_node_entry::AI_NODE_OPTIONS;
        const AI_NODE_CAMERA = arnold_sys::ai_node_entry::AI_NODE_CAMERA;
        const AI_NODE_LIGHT = arnold_sys::ai_node_entry::AI_NODE_LIGHT;
        const AI_NODE_SHAPE = arnold_sys::ai_node_entry::AI_NODE_SHAPE;
        const AI_NODE_SHADER = arnold_sys::ai_node_entry::AI_NODE_SHADER;
        const AI_NODE_OVERRIDE = arnold_sys::ai_node_entry::AI_NODE_OVERRIDE;
        const AI_NODE_DRIVER = arnold_sys::ai_node_entry::AI_NODE_DRIVER;
        const AI_NODE_FILTER = arnold_sys::ai_node_entry::AI_NODE_FILTER;
        const AI_NODE_COLOR_MANAGER = arnold_sys::ai_node_entry::AI_NODE_COLOR_MANAGER;
        const AI_NODE_OPERATOR = arnold_sys::ai_node_entry::AI_NODE_OPERATOR;
        const AI_NODE_ALL = arnold_sys::ai_node_entry::AI_NODE_ALL;
        const AI_NODE_SHAPE_PROCEDURAL = arnold_sys::ai_node_entry::AI_NODE_SHAPE_PROCEDURAL;
        const AI_NODE_SHAPE_VOLUME = arnold_sys::ai_node_entry::AI_NODE_SHAPE_VOLUME;
        const AI_NODE_SHAPE_IMPLICIT = arnold_sys::ai_node_entry::AI_NODE_SHAPE_IMPLICIT;
    }
}
