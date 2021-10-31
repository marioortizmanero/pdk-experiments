use abi_stable::std_types::RBox;
use std::time::Duration;
use tokio::{task, time};

use common_abi_stable_connectors::{
    sink::{RawSink_TO, SinkContext, SinkReply},
    Result,
};

// This is actually saved in the `SinkManager`, and it's used in order to
// communicate with the pipeline (start/pause/link/etc). So in this example it's
// just a stub.
#[derive(Default)]
pub struct SinkAddr(String);

/// Works the same way as tremor's builder for sinks: it's simply used to spawn
/// it into a separate task.
#[derive(Default)]
pub struct SinkManagerBuilder;
impl SinkManagerBuilder {
    pub fn spawn(
        self,
        source: RawSink_TO<'static, RBox<()>>,
        ctx: SinkContext,
    ) -> Result<SinkAddr> {
        let source = Sink(source); // wrapping it up
        let manager = SinkManager { source, ctx };
        // spawn manager task
        task::spawn(manager.run());

        Ok(SinkAddr::default())
    }
}

// Just like `Connector`, this wraps the FFI dynamic source with `abi_stable`
// types so that it's easier to use with `std`.
pub struct Sink(pub RawSink_TO<'static, RBox<()>>);
impl Sink {
    fn pull_data(&mut self, pull_id: u64, ctx: &SinkContext) -> Result<SinkReply> {
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
//
// Note that it uses `dyn` instead of generics now.
pub struct SinkManager {
    pub source: Sink,
    pub ctx: SinkContext,
}
impl SinkManager {
    pub async fn run(mut self) -> Result<()> {
        // No communication for simplicity as well. This should actually send
        // the messages to the `out` and `err` pipelines.
        loop {
            todo!();
        }
    }
}
