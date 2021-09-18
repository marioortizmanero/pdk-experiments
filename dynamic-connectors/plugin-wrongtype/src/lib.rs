// This will fail because the name has the wrong type, but the runtime should
// keep running and end gracefully. This should not actually happen if the macro
// `define_plugin` is used.
#[no_mangle]
pub static PLUGIN_NAME: i32 = 1234;
