// This will crash because the name has the wrong type, and the runtime assumes
// the types are valid. This should not actually happen if the macro
// `define_plugin` is used.
#[no_mangle]
pub extern "C" fn get_name() -> i32 {
    1234
}
