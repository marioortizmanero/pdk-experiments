use crate::{RResult, event::{Event, OpaqueEventSerializer, EventPayload}, util::MayPanic::{self, NoPanic}};

use abi_stable::{std_types::{RString, RStr, RVec, RResult::ROk}, StableAbi, rvec};

/// Stub for now
#[repr(C)]
#[derive(StableAbi, Default, Clone)]
pub struct SinkContext(RString);

/// Simplified for now
#[repr(C)]
#[derive(StableAbi, Debug)]
pub enum SinkReply {
    None,
    Ack,
    Fail,
    CB,
}

/// Result for a sink function that may provide insights or response.
///
/// It can return None or Some(vec![]) if no insights/response were generated.
///
/// An insight is a contraflowevent containing control information for the
/// runtime like circuit breaker events, guaranteed delivery events, etc.
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
    ) -> MayPanic<ResultVec>;
    /// called when receiving a signal
    fn on_signal(
        &mut self,
        _signal: Event,
        _ctx: &SinkContext,
        _serializer: &mut OpaqueEventSerializer,
    ) -> MayPanic<ResultVec> {
        NoPanic(ROk(rvec![]))
    }

    /// Pull metrics from the sink
    fn metrics(&mut self, _timestamp: u64) -> MayPanic<RVec<EventPayload>> {
        NoPanic(rvec![])
    }

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
