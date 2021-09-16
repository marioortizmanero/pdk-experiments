use crate::{errors::Result, event::{EventPayload, Event, EventSerializer}};

/// NOTE: simplification of the real type
pub struct SinkContext;

/// NOTE: simplification of the real type
pub struct SinkManagerBuilder;

/// NOTE: simplification of the real type
pub struct SinkReply;

/// Result for a sink function that may provide insights or response.
///
/// It can return None or Some(vec![]) if no insights/response were generated.
///
/// An insight is a contraflowevent containing control information for the runtime like
/// circuit breaker events, guaranteed delivery events, etc.
///
/// A response is an event generated from the sink delivery.
pub type ResultVec = Result<Vec<SinkReply>>;

/// connector sink - receiving events
#[async_trait::async_trait]
pub trait Sink: Send {
    /// called when receiving an event
    async fn on_event(
        &mut self,
        input: &str,
        event: Event,
        ctx: &SinkContext,
        serializer: &mut EventSerializer,
        start: u64,
    ) -> ResultVec;
    /// called when receiving a signal
    async fn on_signal(
        &mut self,
        _signal: Event,
        _ctx: &SinkContext,
        _serializer: &mut EventSerializer,
    ) -> ResultVec {
        Ok(vec![])
    }

    /// Pull metrics from the sink
    fn metrics(&mut self, _timestamp: u64) -> Vec<EventPayload> {
        vec![]
    }

    // lifecycle stuff
    /// called when started
    async fn on_start(&mut self, _ctx: &mut SinkContext) {}
    /// called when paused
    async fn on_pause(&mut self, _ctx: &mut SinkContext) {}
    /// called when resumed
    async fn on_resume(&mut self, _ctx: &mut SinkContext) {}
    /// called when stopped
    async fn on_stop(&mut self, _ctx: &mut SinkContext) {}

    // connectivity stuff
    /// called when sink lost connectivity
    async fn on_connection_lost(&mut self, _ctx: &mut SinkContext) {}
    /// called when sink re-established connectivity
    async fn on_connection_established(&mut self, _ctx: &mut SinkContext) {}

    /// if `true` events are acknowledged/failed automatically by the sink manager.
    /// Such sinks should return SinkReply::None from on_event or SinkReply::Fail if they fail immediately.
    ///
    /// if `false` events need to be acked/failed manually by the sink impl
    fn auto_ack(&self) -> bool {
        true
    }

    /// if true events are sent asynchronously, not necessarily when `on_event` returns.
    /// if false events can be considered delivered once `on_event` returns.
    fn asynchronous(&self) -> bool {
        false
    }
}
