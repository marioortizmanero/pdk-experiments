/// The interface that must be shared between the plugin and the main binary.
pub type MinFunction = fn(i32, i32) -> i32;

/// Custom trait that can be converted into an object:
///
/// https://doc.rust-lang.org/reference/items/traits.html#object-safety
pub trait MinBuilder {
    fn min(&self, a: i32, b: i32) -> i32;
}
pub type MinBuilderFunction = fn(&Box<dyn MinBuilder>, i32, i32) -> i32;
