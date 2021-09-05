// use std::

/// TODO
#[derive(Debug, Clone)]
pub enum Value<'input> {
    String(String),
    Integer(i32),
    Raw(&'input [u8])
}


#[derive(Debug, Clone)]
pub enum Error {
    One,
    Two,
    Three
}

/// This time we don't need to keep track of versions
#[derive(Debug, Clone)]
pub struct Plugin {
    /// The canonical name for this codec
    pub name: &'static str,
}

pub struct CodecPlugin<'input> {
    /// TODO
    pub plugin: Plugin,

    /// supported mime types
    /// as <base>/<subtype>
    ///
    /// e.g. application/json
    ///
    /// The returned mime types should be unique to this codec
    pub mime_types: &'static [&'static str],

    /// Decode a binary, into an Value
    /// If `None` is returned, no data could be encoded, but we don't exactly triggered an error condition.
    ///
    /// # Errors
    ///  * if we can't decode the data
    pub codec_decode: fn(
        data: &'input mut [u8],
        ingest_ns: u64,
    ) -> Result<Option<Value<'input>>, Error>,

    /// Encodes a Value into a binary
    ///
    /// # Errors
    ///  * If the encoding fails
    pub codec_encode: fn(data: &Value) -> Result<Vec<u8>, Error>,
}
