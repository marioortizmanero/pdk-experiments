use common_ds::{MinFunction, MinBuilder};

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
///
/// Using `dylib`, this is equivalent to using `extern "Rust" fn with_extern`.
///
/// Exporting functions is both possible with `extern` and `static`, but the
/// former makes more sense for Rust-to-Rust FFI because it's just simpler and
/// more straightforward.
#[no_mangle]
pub extern "C" fn with_extern(a: i32, b: i32) -> i32 {
    a.min(b)
}

/*
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
*/
