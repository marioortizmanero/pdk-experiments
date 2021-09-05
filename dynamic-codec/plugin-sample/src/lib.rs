use common_codec::{CodecPlugin, Plugin, Error, Value};
use abi_stable::{
    rstr, rslice, std_types::{RSliceMut, RVec, RResult, RResult::RErr, ROption}
};

#[no_mangle]
pub static PLUGIN: CodecPlugin = CodecPlugin {
    plugin: Plugin {
        name: rstr!("json"),
        abi_stable_version: rstr!("TODO"),
        tremor_version: rstr!("TODO"),
    },
    mime_types: rslice! [
        rstr!("application/json")
    ],
    codec_decode,
    codec_encode,
};

unsafe extern fn codec_decode<'input>(
    data: RSliceMut<'input, u8>,
    ingest_ns: u64,
) -> RResult<ROption<Value<'input>>, Error> {
    RErr(Error::One)
}

unsafe extern fn codec_encode(data: &Value) -> RResult<RVec<u8>, Error> {
    RErr(Error::One)
}
