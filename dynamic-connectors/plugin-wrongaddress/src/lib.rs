use std::os::raw::c_char;

// This will crash because the given address is invalid (it points to an
// arbitrary part of memory), and the runtime assumes the pointers are valid.
// This should not actually happen if the macro `define_plugin` is used.
#[no_mangle]
pub extern "C" fn get_name() -> *const c_char {
    0 as *const i8
}
