use crate::{RResult, event::{Event, OpaqueEventSerializer, EventPayload}};

use abi_stable::{std_types::{RString, RStr, RVec, RResult::ROk}, StableAbi, rvec};

// Stub for now
#[repr(C)]
#[derive(StableAbi, Default)]
pub struct SinkContext(RString);

// Stub for now
#[repr(C)]
#[derive(StableAbi, Default)]
pub struct SinkReply(RString);

/// Result for a sink function that may provide insights or response.
///
/// It can return None or Some(vec![]) if no insights/response were generated.
///
/// An insight is a contraflowevent containing control information for the runtime like
/// circuit breaker events, guaranteed delivery events, etc.
///
/// A response is an event generated from the sink delivery.
pub type ResultVec = RResult<RVec<SinkReply>>;

#[abi_stable::sabi_trait]
pub trait RawSink: Send {
    /// called when receiving an event
    fn on_event(
        &mut self,
        input: RStr<'_>,
        event: Event,
        ctx: &SinkContext,
        serializer: &mut OpaqueEventSerializer,
        start: u64,
    ) -> ResultVec;
    /// called when receiving a signal
    fn on_signal(
        &mut self,
        _signal: Event,
        _ctx: &SinkContext,
        _serializer: &mut OpaqueEventSerializer,
    ) -> ResultVec {
        ROk(rvec![])
    }

    /// Pull metrics from the sink
    fn metrics(&mut self, _timestamp: u64) -> RVec<EventPayload> {
        rvec![]
    }

    // lifecycle stuff
    /// called when started
    fn on_start(&mut self, _ctx: &mut SinkContext) {}
    /// called when paused
    fn on_pause(&mut self, _ctx: &mut SinkContext) {}
    /// called when resumed
    fn on_resume(&mut self, _ctx: &mut SinkContext) {}
    /// called when stopped
    fn on_stop(&mut self, _ctx: &mut SinkContext) {}

    // connectivity stuff
    /// called when sink lost connectivity
    fn on_connection_lost(&mut self, _ctx: &mut SinkContext) {}
    /// called when sink re-established connectivity
    fn on_connection_established(&mut self, _ctx: &mut SinkContext) {}

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
