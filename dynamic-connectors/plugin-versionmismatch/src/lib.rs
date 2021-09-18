use std::os::raw::c_char;

/// While this field is valid, the version number is not compatible
#[no_mangle]
pub static PLUGIN_COMMON_VERSION: *const c_char = "0.0.0".as_ptr() as _;
