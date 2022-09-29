#![allow(non_camel_case_types, non_snake_case)]
use std::{
    ffi::{c_int, CString},
    path::PathBuf,
};

use crate::{ai_node_entry::NodeTypes, ai_universe::AtUniverse};

pub fn AiASSWrite<T: Into<PathBuf>>(
    universe: Option<&AtUniverse>,
    filename: T,
    mask: NodeTypes,
    open_procs: bool,
    binary: bool,
) -> Result<(), String> {
    let universe = match universe {
        Some(universe) => universe.raw(),
        None => std::ptr::null_mut(),
    };

    let filename = CString::new(filename.into().to_str().unwrap()).unwrap();

    let result = unsafe {
        arnold_sys::ai_dotass::AiASSWrite(
            universe,
            filename.as_ptr(),
            mask.bits() as c_int,
            open_procs,
            binary,
        )
    };
    if result != 0 {
        return Err(String::from("Failed to write ass file"));
    }
    Ok(())
}
