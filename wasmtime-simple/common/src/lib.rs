/// The interface that must be shared between the plugin and the main binary,
/// though we don't really need it for wasm.
pub type MinFunction = fn(i32, i32) -> i32;
