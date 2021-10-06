use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn get_name() -> *const c_char {
    panic!("Something went wrong!")
}
