use crate::{
    errors::Result,
    event::{EventOriginUri, EventPayload},
};
use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, Sender},
    thread,
};

/// NOTE: simplification of the real type
pub struct SourceContext;

/// NOTE: simplification of the real type
/// reply from `Source::on_event`
#[derive(Debug)]
pub enum SourceReply {
    // an already structured event payload
    Structured {
        origin_uri: EventOriginUri,
        payload: HashMap<String, String>,
        stream: u64,
    },
    /// no new data/event, wait for the given ms
    Empty(u64),
}

/// NOTE: simplification of the real type
/// Messages a Source can receive
pub enum SourceMsg {}

/// source part of a connector
pub trait Source: Send {
    /// Pulls an event from the source if one exists
    /// `idgen` is passed in so the source can inspect what event id it would get if it was producing 1 event from the pulled data
    /* async */
    fn pull_data(&mut self, pull_id: u64, ctx: &SourceContext) -> Result<SourceReply>;
    /// This callback is called when the data provided from
    /// pull_event did not create any events, this is needed for
    /// linked sources that require a 1:1 mapping between requests
    /// and responses, we're looking at you REST
    /* async */
    fn on_no_events(&mut self, _pull_id: u64, _stream: u64, _ctx: &SourceContext) -> Result<()> {
        Ok(())
    }

    /// Pulls custom metrics from the source
    fn metrics(&mut self, _timestamp: u64) -> Vec<EventPayload> {
        vec![]
    }

    ///////////////////////////
    /// lifecycle callbacks ///
    ///////////////////////////

    /// called when the source is started. This happens only once in the whole source lifecycle, before any other callbacks
    /* async */
    fn on_start(&mut self, _ctx: &mut SourceContext) {}
    /// called when the source is explicitly paused as result of a user/operator interaction
    /// in contrast to `on_cb_close` which happens automatically depending on downstream pipeline or sink connector logic.
    /* async */
    fn on_pause(&mut self, _ctx: &mut SourceContext) {}
    /// called when the source is explicitly resumed from being paused
    /* async */
    fn on_resume(&mut self, _ctx: &mut SourceContext) {}
    /// called when the source is stopped. This happens only once in the whole source lifecycle, as the very last callback
    /* async */
    fn on_stop(&mut self, _ctx: &mut SourceContext) {}

    // circuit breaker callbacks
    /// called when we receive a `close` Circuit breaker event from any connected pipeline
    /// Expected reaction is to pause receiving messages, which is handled automatically by the runtime
    /// Source implementations might want to close connections or signal a pause to the upstream entity it connects to if not done in the connector (the default)
    // TODO: add info of Cb event origin (port, origin_uri)?
    /* async */
    fn on_cb_close(&mut self, _ctx: &mut SourceContext) {}
    /// Called when we receive a `open` Circuit breaker event from any connected pipeline
    /// This means we can start/continue polling this source for messages
    /// Source implementations might want to start establishing connections if not done in the connector (the default)
    /* async */
    fn on_cb_open(&mut self, _ctx: &mut SourceContext) {}

    // guaranteed delivery callbacks
    /// an event has been acknowledged and can be considered delivered
    /// multiple acks for the same set of ids are always possible
    /* async */
    fn ack(&mut self, _stream_id: u64, _pull_id: u64) {}
    /// an event has failed along its way and can be considered failed
    /// multiple fails for the same set of ids are always possible
    /* async */
    fn fail(&mut self, _stream_id: u64, _pull_id: u64) {}

    // connectivity stuff
    /// called when connector lost connectivity
    /* async */
    fn on_connection_lost(&mut self, _ctx: &mut SourceContext) {}
    /// called when connector re-established connectivity
    /* async */
    fn on_connection_established(&mut self, _ctx: &mut SourceContext) {}

    /// Is this source transactional or can acks/fails be ignored
    fn is_transactional(&self) -> bool {
        false
    }

    /// if `true` this source is polled for data even if it is not connected to
    /// any pipeline and is not terminated if it is completely disconnected.
    fn keep_alive(&self) -> bool {
        false
    }
}

/// address of a source
#[derive(Clone, Debug)]
pub struct SourceAddr {
    /// the actual address
    pub addr: Sender<SourceMsg>,
}

#[allow(clippy::module_name_repetitions)]
pub struct SourceManagerBuilder {
    qsize: usize,
    streams: Streams,
    source_metrics_reporter: SourceReporter,
}

/// NOTE: simplification of the real type
impl SourceManagerBuilder {
    pub fn spawn<S>(self, source: S, ctx: SourceContext) -> Result<SourceAddr>
    where
        S: Source + Send + 'static,
    {
        let qsize = self.qsize;
        let name = ctx.url.short_id("c-src"); // connector source
        let (source_tx, source_rx) = bounded(qsize);
        let manager = SourceManager::new(source, ctx, self, source_rx);
        // Spawn manager thread. NOTE: this is simplified, removing
        // `SourceManager`.
        thread::spawn(move || {});

        Ok(SourceAddr { addr: source_tx })
    }
}

/// NOTE: simplification of the real type
pub(crate) struct SourceManager<S>
where
    S: Source,
{
    source: S,
    ctx: SourceContext,
    rx: Receiver<SourceMsg>,
}
impl<S> SourceManager<S>
where
    S: Source,
{
    fn new(
        source: S,
        ctx: SourceContext,
        builder: SourceManagerBuilder,
        rx: Receiver<SourceMsg>,
    ) -> Self {
        Self { source, ctx, rx }
    }

    fn run(mut self) -> Result<()> {
        let mut pull_counter: u64 = 0;
        loop {
            match self.source.pull_data(pull_counter, &self.ctx) {
                Ok(SourceReply::Structured {
                    origin_uri,
                    payload,
                    stream,
                }) => {
                    let ingest_ns = nanotime();
                    let stream_state = self.streams.get_or_create_stream(stream)?;
                    let event = build_event(
                        stream_state,
                        pull_counter,
                        ingest_ns,
                        payload,
                        origin_uri,
                        self.is_transactional,
                    );
                    let error = self.route_events(vec![(OUT, event)]).await;
                    if error {
                        self.source.fail(stream, pull_counter).await;
                    }
                }
                Ok(SourceReply::Empty(wait_ms)) => {
                    thread::sleep_ms(wait_ms as u32);
                }
                Err(e) => println!("[Source] Error pulling data: {}", e),
            }
        }
    }
}
