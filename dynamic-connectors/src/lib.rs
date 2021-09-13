// use common_dconnectors::{CodecPlugin, Value};

use anyhow::Result;

use libloading::Library;

// TODO: docs
pub fn setup_plugin(path: &str) -> Result<impl Fn() -> Result<()>> {
    unsafe {
        /*
        let library = Library::new(path)?;

        let plugin = library.get::<*mut ConnectorPlugin>(b"PLUGIN_DATA")?.read();
        */

        // TODO check plugin versioning

        Ok(move || {
            println!("Running plugin");
            // TODO

            Ok(())
        })
    }
}
