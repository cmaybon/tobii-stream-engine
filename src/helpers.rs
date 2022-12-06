use std::os::raw::*;
use std::ffi::{CStr, CString};

pub unsafe fn c_char_ptr_to_string(array_ptr: *const c_char) -> String {
    CStr::from_ptr(array_ptr).to_str().unwrap().to_owned()
}
