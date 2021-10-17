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

/// For benchmarking reasons, setting up the plugin and running it is a two step
/// process. Thus, the setup is done when calling this function, and it can be
/// ran when calling the returned closure.
pub fn setup_plugin() -> impl Fn() -> Result<()> {
    move || {
        // For this example we skip the whole concept of `World` and `Manager`.
        // This acts directy as a `connectors::Manager`, which initializes and
        // runs a list of known actions over a single connector (instead of
        // multiple of them, for simplicity as well).
        let metronome_builder = Builder::default();
        let connector = metronome_builder.from_config(&TremorUrl, 100);

        let source_builder = SourceManagerBuilder {
            qsize: todo!(),
            streams: todo!(),
            source_metrics_reporter: todo!(),
        };
        let source = connector.create_source(SourceContext, source_builder).unwrap().unwrap();


        Ok(())
    }
}
