use tremor_core::{MinFunction, MinBuilder};

/// This is what the plugin publicly exports. It has to use `#[no_mangle]` so
/// that its name is known when loading from the main binary.
///
/// The first approach exposes the function via a pointer, which is saved as a
/// static variable.
#[no_mangle]
pub static with_static: MinFunction = min;

fn min(a: i32, b: i32) -> i32 {
    a.min(b)
}

/// Using `pub` is enough to export the function.
#[no_mangle]
pub fn with_extern(a: i32, b: i32) -> i32 {
    a.min(b)
}

/// Attempting to export a generic function in the shared library.
///
/// Note that generic functions must be mangled, so trying to set `#[no_mangle]`
/// will raise a warning.
pub fn with_extern_generics<M: MinBuilder>(builder: M, a: i32, b: i32) -> i32 {
    builder.min(a, b)
}

/// Instead of using generics, trying with dynamic dispatching.
#[no_mangle]
pub fn with_extern_dyn(builder: &Box<dyn MinBuilder>, a: i32, b: i32) -> i32 {
    builder.min(a, b)
}
