use common_codec::{CodecPlugin, Value};

use libloading::Library;

pub fn run_plugin(path: &str) -> Result<(), libloading::Error> {
    unsafe {
        println!("Running plugin");
        let library = Library::new(path)?;

        let plugin = library.get::<*mut CodecPlugin>(b"PLUGIN")?.read();

        // TODO check plugin versions

        let encoded = (plugin.codec_encode)(&Value::Integer(1234));
        println!("encoded: {:?}", encoded);
    }

    Ok(())
}
