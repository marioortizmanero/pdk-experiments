use abi_stable::std_types::{RStr, RSlice, RSliceMut, RResult, ROption, RVec, RString};

/// TODO
#[repr(C)]
#[derive(Debug, Clone)]
pub enum Value<'input> {
    String(RString),
    Integer(i32),
    Raw(RSlice<'input, u8>)
}


#[repr(C)]
#[derive(Debug, Clone)]
pub enum Error {
    One,
    Two,
    Three
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Plugin {
    /// The canonical name for this codec
    pub name: RStr<'static>,

    /// TODO
    pub abi_stable_version: RStr<'static>,

    /// TODO
    pub tremor_version: RStr<'static>,
}

#[repr(C)]
pub struct CodecPlugin<'input> {
    /// TODO
    pub plugin: Plugin,

    /// supported mime types
    /// as <base>/<subtype>
    ///
    /// e.g. application/json
    ///
    /// The returned mime types should be unique to this codec
    pub mime_types: RSlice<'static, RStr<'static>>,

    /// Decode a binary, into an Value
    /// If `None` is returned, no data could be encoded, but we don't exactly triggered an error condition.
    ///
    /// # Errors
    ///  * if we can't decode the data
    pub codec_decode: unsafe extern fn(
        data: RSliceMut<'input, u8>,
        ingest_ns: u64,
    ) -> RResult<ROption<Value<'input>>, Error>,

    /// Encodes a Value into a binary
    ///
    /// # Errors
    ///  * If the encoding fails
    pub codec_encode: unsafe extern fn(data: &Value) -> RResult<RVec<u8>, Error>,
}
