#![allow(non_camel_case_types, non_snake_case)]

use crate::ai_nodes::AtNode;

#[derive(Debug, Copy, Clone)]
pub struct AtUniverse {
    pub raw: *mut arnold_sys::ai_universe::AtUniverse
}

impl AtUniverse {
    pub fn raw(&self) -> *mut arnold_sys::ai_universe::AtUniverse {
        unsafe { &mut *self.raw }
    }
}

pub fn AiUniverseGetOptions(universe: Option<&AtUniverse>) -> AtNode {
    let universe = match universe {
        Some(universe) => universe.raw(),
        None => std::ptr::null_mut(),
    };

    AtNode {
        raw: unsafe{arnold_sys::ai_universe::AiUniverseGetOptions(universe)},
    }
}
