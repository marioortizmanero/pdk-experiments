use std::borrow::Cow;
use std::collections::HashMap;

pub type Object<'value> = HashMap<Cow<'value, str>, Value<'value>>;
pub type Bytes<'value> = Cow<'value, [u8]>;

/// Static tape node
#[derive(Debug, Clone, Copy)]
pub enum StaticNode {
    /// A signed 64 bit integer.
    I64(i64),
    #[cfg(feature = "128bit")]
    /// A signed 128 bit integer.
    I128(i128),
    /// An unsigned 64 bit integer.
    U64(u64),
    #[cfg(feature = "128bit")]
    /// An unsigned 128 bit integer.
    U128(u128),
    /// A floating point value
    F64(f64),
    /// A boolean value
    Bool(bool),
    /// The null value
    Null,
}

/// Borrowed JSON-DOM Value, consider using the `ValueTrait`
/// to access its content
#[derive(Debug, Clone)]
pub enum Value<'value> {
    /// Static values
    Static(StaticNode),
    /// string type
    String(Cow<'value, str>),
    /// array type
    Array(Vec<Value<'value>>),
    /// object type
    Object(Box<Object<'value>>),
    /// A binary type
    Bytes(Bytes<'value>),
}
