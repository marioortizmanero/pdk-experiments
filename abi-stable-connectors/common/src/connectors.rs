use abi_stable::{
    std_types::{
        RBox,
        ROption::{self, RNone},
        RResult::ROk,
        RStr, RString,
    },
    StableAbi,
};

use crate::{
    sink::{RawSink_TO, SinkContext},
    source::{RawSource_TO, SourceContext},
    RResult,
    util::MayPanic::{self, NoPanic}
};

// Stub
#[repr(C)]
#[derive(StableAbi)]
pub struct ConnectorContext(RString);

// Stub
#[repr(C)]
#[derive(StableAbi, Default)]
pub struct Attempt(RString);

// Stub
#[repr(C)]
#[derive(StableAbi, Default)]
pub struct TremorUrl(RString);

// Stub
#[repr(C)]
#[derive(StableAbi, Default)]
pub struct ConnectorState(RString);

// The low level connector trait used for the plugins, with types from
// abi_stable.
#[abi_stable::sabi_trait]
pub trait RawConnector: Send {
    // Note that the source spawning and handling isn't limited to the connector
    // side anymore. Since the communication details ought to be moved to the
    // runtime rather than the plugin for simplicity's sake, the source is
    // handled at the runtime as well, and it only exports synchronous basic
    // methods.
    //
    // In practical terms, this returns a `dyn RawSource` rather than
    // `SourceAddr`, and the latter is now created on the runtime. The builder
    // isn't passed here anymore either.
    fn create_source(
        &mut self,
        _source_context: SourceContext,
    ) -> MayPanic<RResult<ROption<RawSource_TO<'static, RBox<()>>>>> {
        NoPanic(ROk(RNone))
    }

    fn create_sink(
        &mut self,
        _sink_context: SinkContext,
    ) -> MayPanic<RResult<ROption<RawSink_TO<'static, RBox<()>>>>> {
        NoPanic(ROk(RNone))
    }

    fn connect(
        &mut self,
        ctx: &ConnectorContext,
        attempt: &Attempt,
    ) -> MayPanic<RResult<bool>>;

    fn on_start(&mut self, ctx: &ConnectorContext) -> MayPanic<RResult<ConnectorState>>;

    fn on_pause(&mut self, _ctx: &ConnectorContext) -> MayPanic<()> {
        NoPanic(())
    }
    fn on_resume(&mut self, _ctx: &ConnectorContext) -> MayPanic<()> {
        NoPanic(())
    }
    fn on_stop(&mut self, _ctx: &ConnectorContext) -> MayPanic<()> {
        NoPanic(())
    }

    fn default_codec(&self) -> RStr<'_>;
}
