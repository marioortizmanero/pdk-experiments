use common_ncodec::{CodecPlugin, Plugin, Error, Value};

#[no_mangle]
pub static PLUGIN_DATA: CodecPlugin = CodecPlugin {
    plugin: Plugin {
        name: "json",
    },
    mime_types: &[
        "application/json"
    ],
    codec_decode,
    codec_encode,
};

fn codec_decode<'input>(
    data: &'input mut [u8],
    ingest_ns: u64,
) -> Result<Option<Value<'input>>, Error> {
    Err(Error::One)
}

fn codec_encode(data: &Value) -> Result<Vec<u8>, Error> {
    Err(Error::One)
}
