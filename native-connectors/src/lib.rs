mod connectors;

use connectors::Connector;

use anyhow::Result;

/// TODO document
pub fn setup_plugin() -> impl Fn() -> Result<()> {
    move || {
        /*
        println!("Running plugin {}", PLUGIN_DATA.plugin.name);

        let encoded = (PLUGIN_DATA.codec_encode)(&Value::Integer(1234));
        println!("encoded: {:?}", encoded);
        */

        Ok(())
    }
}
