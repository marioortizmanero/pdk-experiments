use common_abi_stable_connectors::{
    event::{Event, OpaqueEventSerializer},
    sink::{RawSink_TO, SinkContext, SinkReply},
    value::Value,
    Result,
};

use abi_stable::std_types::RBox;
use tokio::{
    io::{self, AsyncBufReadExt, BufReader},
    task,
};

// This is actually saved in the `SinkManager`, and it's used in order to
// communicate with the pipeline (start/pause/link/etc). In this example it's
// just a stub.
#[derive(Default)]
pub struct SinkAddr(String);

/// Works the same way as tremor's builder for sinks: it's simply used to spawn
/// it into a separate task and other boilerplate.
pub struct SinkManagerBuilder {
    pub serializer: OpaqueEventSerializer,
}
impl SinkManagerBuilder {
    pub fn spawn(self, sink: RawSink_TO<'static, RBox<()>>, ctx: SinkContext) -> Result<SinkAddr> {
        let sink = Sink(sink); // wrapping it up
        let manager = SinkManager {
            sink,
            ctx,
            serializer: self.serializer,
        };
        // spawn manager task
        task::spawn(manager.run());

        Ok(SinkAddr::default())
    }
}

pub type ResultVec = Result<Vec<SinkReply>>;

// Just like `Connector`, this wraps the FFI dynamic source with `abi_stable`
// types so that it's easier to use with `std`.
pub struct Sink(pub RawSink_TO<'static, RBox<()>>);
impl Sink {
    #[inline]
    pub fn on_event(
        &mut self,
        input: &str,
        event: Event,
        ctx: &SinkContext,
        serializer: &mut OpaqueEventSerializer,
        start: u64,
    ) -> ResultVec {
        self.0
            .on_event(input.into(), event, ctx, serializer, start)
            .unwrap()
            .map(Into::into) // RVec -> Vec
            .map_err(Into::into) // RBoxError -> Box<dyn Error>
            .into() // RResult -> Result
    }

    #[inline]
    pub fn on_signal(
        &mut self,
        signal: Event,
        ctx: &SinkContext,
        serializer: &mut OpaqueEventSerializer,
    ) -> ResultVec {
        self.0
            .on_signal(signal, ctx, serializer)
            .unwrap()
            .map(Into::into) // RVec -> Vec
            .map_err(Into::into) // RBoxError -> Box<dyn Error>
            .into() // RResult -> Result
    }

    #[inline]
    pub fn auto_ack(&self) -> bool {
        self.0.auto_ack()
    }

    #[inline]
    pub fn asynchronous(&self) -> bool {
        self.0.asynchronous()
    }
}

// The runner of the sink, which receives events from the pipeline (stdin in
// this case) and forwards them to the sinks. `on_event` could be made async so
// that internal operations aren't blocking thanks to the crate `async_ffi`, but
// I'll leave it like that for now for simplicity.
pub struct SinkManager {
    pub sink: Sink,
    pub ctx: SinkContext,
    pub serializer: OpaqueEventSerializer,
}
impl SinkManager {
    pub async fn run(mut self) -> Result<()> {
        // No communication for simplicity as well. This should actually receive
        // the messages from the pipelines, and also from the sink itself.
        let stdin = io::stdin();
        let mut stdin = BufReader::new(stdin);
        println!("The sink reads line events from console! Try writing a line to stdin.");
        let mut id = 0;
        loop {
            // Generating an event, only one at a time for now
            let mut input = String::new();
            stdin.read_line(&mut input).await?;
            let data = Value::String(input.into());
            let event = Event { id, data };
            id += 1;

            match self
                .sink
                .on_event("/in", event, &self.ctx, &mut self.serializer, 0)
            {
                Ok(reply) => eprintln!("Sink reply: {:?}", reply),
                Err(e) => eprintln!("Error notifying new event: {}", e),
            }
        }
    }
}
