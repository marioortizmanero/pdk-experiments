use tremor_core::MinFunction;

/// This is what the plugin publicly exports. It has to use `#[no_mangle]` so
/// that its name is known when loading from the main binary.
#[no_mangle]
pub static plugin_function: MinFunction = min;

fn min(a: i32, b: i32) -> i32 {
    a.min(b)
}
