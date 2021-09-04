/// Using `pub` is enough to export the function.
///
/// In the case of Wasm, `static` isn't worth using to export the function,
/// since globals can only be integers and it would require some more
/// complicated conversion.
#[no_mangle]
pub fn with_extern(a: i32, b: i32) -> i32 {
    a.min(b)
}
