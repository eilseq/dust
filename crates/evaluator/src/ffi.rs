//! Handles communication with Haskell's TidalCycles via FFI.

use event_handling::{parse_tidal_json, Event};
use libc::c_char;
use std::ffi::{CStr, CString};

#[link(name = "tidalparseffi", kind = "static")]
extern "C" {
    fn eval_pattern_c(input: *const c_char, arc: *const c_char) -> *mut c_char;
}

/// Calls Haskell FFI and parses events.
pub fn eval_pattern(pattern: &str, arc_length: &str) -> Result<Vec<Event>, String> {
    let c_pattern = CString::new(pattern).map_err(|_| "CString conversion failed".to_string())?;
    let c_arc = CString::new(arc_length).map_err(|_| "CString conversion failed".to_string())?;

    let result_ptr = unsafe { eval_pattern_c(c_pattern.as_ptr(), c_arc.as_ptr()) };
    if result_ptr.is_null() {
        return Err("Failed to evaluate pattern".to_string());
    }

    let result = unsafe { CStr::from_ptr(result_ptr) }
        .to_string_lossy()
        .into_owned();

    parse_tidal_json(&result)
}
