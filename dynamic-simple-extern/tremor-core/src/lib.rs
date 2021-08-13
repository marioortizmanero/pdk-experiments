/// The interface that must be shared between the plugin and the main binary.
pub type MinFunction = unsafe fn(i32, i32) -> i32;
