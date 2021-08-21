use tremor_core::{MinFunction, MinBuilder};

#[no_mangle]
pub static with_static: MinFunction = with_extern;

/// Using `pub` is enough to export the function.
///
/// In the case of Wasm, `static` isn't worth using to export the function,
/// since globals can only be integers and it would require some more
/// complicated conversion.
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
