mod connectors;

use connectors::metronome::{Metronome, Builder};

use anyhow::Result;

/// TODO document
pub fn setup_plugin() -> impl Fn() -> Result<()> {
    move || {
        let conn = Builder::from_config();
        /*
        println!("Running plugin {}", PLUGIN_DATA.plugin.name);

        let encoded = (PLUGIN_DATA.codec_encode)(&Value::Integer(1234));
        println!("encoded: {:?}", encoded);
        */

        Ok(())
    }
}
