use common_ds::MinFunction;

/// This is what the plugin publicly exports. It has to use `#[no_mangle]` so
/// that its name is known when loading from the main binary.
///
/// The first approach exposes the function via a pointer, which is saved as a
/// static variable.
#[no_mangle]
pub static with_static: MinFunction = min;

pub extern "C" fn min(a: i32, b: i32) -> i32 {
    a.min(b)
}

/// This is the second approach, where the function is directly exposed via
/// dynamic linking, using the Rust ABI.
#[no_mangle]
pub extern "C" fn with_extern(a: i32, b: i32) -> i32 {
    a.min(b)
}
