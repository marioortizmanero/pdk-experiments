pub mod connectors;
pub mod event;
pub mod time;
pub mod value;

use crate::{connectors::{metronome::{Builder, Metronome}, source::{SourceContext, SourceManagerBuilder}}, url::TremorUrl};

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
    // TODO: for now should we use
    // https://docs.rs/abi_stable/0.10.2/abi_stable/external_types/crossbeam_channel/index.html
    // which is closer to what tremor actually uses (the only difference being
    // that it's not asynchronous), or something simpler like callbacks?
    move || {
        // TODO: this uses `Box<dyn Connector>`, how can we make that work for
        // dynamic loading?

        // TODO: perhaps it would be better to initialize the metronome directly
        // and use it as a connector or a source?
        //
        // I should remove the `async` parts first of all
        let metronome_builder = Builder::default();
        let connector = metronome_builder.from_config(&TremorUrl, 100);

        let source_builder = SourceManagerBuilder {
            qsize: todo!(),
            streams: todo!(),
            source_metrics_reporter: todo!(),
        };
        let source = connector.create_source(SourceContext, source_builder).await.unwrap().unwrap();

        source.

        /*
        println!("Running plugin {}", PLUGIN_DATA.plugin.name);

        let encoded = (PLUGIN_DATA.codec_encode)(&Value::Integer(1234));
        println!("encoded: {:?}", encoded);
        */

        Ok(())
    }
}
