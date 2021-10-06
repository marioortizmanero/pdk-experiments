use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn get_name() -> *const c_char {
    b"versionmismatch\0".as_ptr() as _
}

/// This will gracefully fail because this version is considered incompatible
/// with `common` v0.0.1.
#[no_mangle]
pub extern "C" fn get_version() -> *const c_char {
    b"0.0.0\0".as_ptr() as _
}
