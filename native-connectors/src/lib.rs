pub mod connectors;
pub mod event;
pub mod time;
pub mod value;

use connectors::metronome::{Builder, Metronome};

use anyhow::Result;

/// default stream id if streams dont make sense
pub const DEFAULT_STREAM_ID: u64 = 0;
/// default pull id if pulls arent tracked
pub const DEFAULT_PULL_ID: u64 = 0;

pub mod url {
    /// NOTE: simplification of the real type
    pub struct TremorUrl;
}
pub mod errors {
    /// NOTE: simplification of the real type
    pub type Error = Box<dyn std::error::Error>;
    pub struct ErrorKind;
    pub type Result<T> = std::result::Result<T, Error>;
}

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
