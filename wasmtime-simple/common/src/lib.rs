/// Custom trait that can be converted into an object:
///
/// https://doc.rust-lang.org/reference/items/traits.html#object-safety
pub trait MinBuilder {
    fn min(&self, a: i32, b: i32) -> i32;
}
