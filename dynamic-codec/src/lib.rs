use common_codec::{CodecPlugin, Value};

use anyhow::Result;

use libloading::Library;

// TODO: docs
pub fn setup_plugin(path: &str) -> Result<impl Fn() -> Result<()>> {
    unsafe {
        let library = Library::new(path)?;

        let plugin = library.get::<*mut CodecPlugin>(b"PLUGIN_DATA")?.read();
        // TODO check plugin versions

        Ok(move || {
            println!("Running plugin");
            let encoded = (plugin.codec_encode)(&Value::Integer(1234));
            println!("encoded: {:?}", encoded);

            Ok(())
        })
    }
}
