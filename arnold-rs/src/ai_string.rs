use std::ffi::{CString, c_char};

#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub struct AtString {
    pub(crate) data: *const c_char
}

impl AtString {
    pub fn from(s: &str) -> Self {
        let c_str = CString::new(s).unwrap();
        AtString {
            data: unsafe { arnold_sys::ai_string::AiCreateAtStringData_private(c_str.as_ptr())},
        }
    }

    pub fn null() -> Self { // FIXME: Better name?
        AtString {
            data: unsafe { arnold_sys::ai_string::AiCreateAtStringData_private(std::ptr::null_mut())},
        }
    }

    pub(crate) fn copy(other: &Self) -> Self {
        AtString {
            data: other.raw()
        }
    }

    pub(crate) fn raw(self) -> *const c_char {
        self.data
    }
}

impl From<&str> for AtString {
    fn from(s: &str) -> Self {
        AtString::from(s)
    }
}

impl From<&AtString> for AtString {
    fn from(s: &AtString) -> Self {
        AtString::copy(s)
    }
}

impl From<AtString> for arnold_sys::ai_string::AtString {
    fn from(s: AtString) -> Self {
        arnold_sys::ai_string::AtString {
            data: s.raw()
        }
    }
}

impl From<&AtString> for arnold_sys::ai_string::AtString {
    fn from(s: &AtString) -> Self {
        arnold_sys::ai_string::AtString {
            data: s.raw()
        }
    }
}

/*
impl Into<AtString> for &str {
    fn into(self) -> AtString {
        AtString::from(self)
    }
}

impl Into<AtString> for &AtString {
    fn into(self) -> AtString {
        AtString::copy(self)
    }
}

impl Into<arnold_sys::ai_string::AtString> for AtString {
    fn into(self) -> arnold_sys::ai_string::AtString {
        arnold_sys::ai_string::AtString{ data: self.data }
    }
}

impl Into<arnold_sys::ai_string::AtString> for &AtString {
    fn into(self) -> arnold_sys::ai_string::AtString {
        arnold_sys::ai_string::AtString{ data: self.data }
    }
}
*/
