use common_ncodec::Value;
use plugin_sample_ncodec::PLUGIN_DATA;

use std::error::Error;

/// TODO document
pub fn setup_plugin() -> impl Fn() -> Result<(), Box<dyn Error>> {
    move || {
        println!("Running plugin {}", PLUGIN_DATA.plugin.name);

        let encoded = (PLUGIN_DATA.codec_encode)(&Value::Integer(1234));
        println!("encoded: {:?}", encoded);

        Ok(())
    }
}
