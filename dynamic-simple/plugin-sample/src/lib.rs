use tremor_core::MinFunction;

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

/// This is the second approach, where the function is directly exposed via
/// dynamic linking, using the Rust ABI.
#[no_mangle]
pub fn with_extern(a: i32, b: i32) -> i32 {
    a.min(b)
}

// /// Same but with generics
// #[no_mangle]
// pub fn with_extern_generics<T: Ord>(a: T, b: T) -> T {
//     a.min(b)
// }

// /// This is the second approach, where the function is directly exposed via
// /// dynamic linking, using the Rust ABI.
// #[no_mangle]
// pub fn with_extern(a: Box<dyn Ord>, b: Box<dyn Ord>) -> Box<dyn Ord> {
//     a.min(b)
// }
