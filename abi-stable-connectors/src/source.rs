use abi_stable::std_types::RBox;
use std::time::Duration;
use tokio::{task, time};

use common_abi_stable_connectors::{
    source::{RawSource_TO, SourceContext, SourceReply},
    Result,
};

// This is actually saved in the `SourceManager`, and it's used in order to
// communicate with the pipeline (start/pause/link/etc). So in this example it's
// just a stub.
#[derive(Default)]
pub struct SourceAddr(String);

/// Works the same way as tremor's builder for sources: it's simply used to
/// spawn it into a separate task.
#[derive(Default)]
pub struct SourceManagerBuilder;
impl SourceManagerBuilder {
    pub fn spawn(
        self,
        source: RawSource_TO<'static, RBox<()>>,
        ctx: SourceContext,
    ) -> Result<SourceAddr> {
        let source = Source(source); // wrapping it up
        let manager = SourceManager { source, ctx };
        // spawn manager task
        task::spawn(manager.run());

        Ok(SourceAddr::default())
    }
}

// Just like `Connector`, this wraps the FFI dynamic source with `abi_stable`
// types so that it's easier to use with `std`.
pub struct Source(pub RawSource_TO<'static, RBox<()>>);
impl Source {
    fn pull_data(&mut self, pull_id: u64, ctx: &SourceContext) -> Result<SourceReply> {
        self.0
            .pull_data(pull_id, ctx)
            .unwrap()
            .map_err(Into::into) // RBoxError -> Box<dyn Error>
            .into() // RResult -> Result
    }

    fn is_transactional(&self) -> bool {
        self.0.is_transactional()
    }
}

// The runner of the source, which pulls the events continuously. This could be
// made async so that internal operations aren't blocking thanks to the crate
// `async_ffi`, but I'll leave it like that for now for simplicity.
pub struct SourceManager {
    pub source: Source,
    pub ctx: SourceContext,
}
impl SourceManager {
    pub async fn run(mut self) -> Result<()> {
        // No communication for simplicity as well. This should actually send
        // the messages to the `out` and `err` pipelines.
        loop {
            let data = self.source.pull_data(0, &self.ctx);
            match data {
                Ok(SourceReply::Empty(ms)) => {
                    println!("No data available, sleeping {} ms", ms);
                    time::sleep(Duration::from_millis(ms)).await;
                }
                Ok(SourceReply::Data(data)) => {
                    println!("Sending '{}' to pipeline", data)
                }
                Err(e) => {
                    eprintln!("Error in source: {}", e);
                }
            }
        }
    }
}
