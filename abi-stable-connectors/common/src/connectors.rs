// Problem #1 (you should've seen it coming :P):
//
// In the connectors plugin interface I have a single `new` function that
// exports a `Connector` dynamic trait from the plugin. The runtime can then use
// that as a generic connector just like how Tremor does now. On the
// plugin-side, the `create_{source,sink}` methods call `builder.spawn`, which
// relies on the fact that the type implements `Source` or `Sink`. This spawns
// the new task and communicates with the connector. As I said, this happens on
// the implementor side, so the runtime doesn't know if the concrete type
// implements `Sink` or `Source`, only that it's a `Connector`, and the plugin
// handles the rest itself.
//
// However, since we wanted to simplify the plugin interface as much as
// possible, the communication details should happen on the runtime rather than
// on the plugin. What I mean is that, instead of calling `builder.spawn` on the
// plugin and creating the channel on the plugin, it should happen on the
// runtime. Thus, the whole idea of `create_{source,sink}` is now somewhat
// pointless, because it's handled by the runtime. We have a `dyn Connector`,
// with which we can't know if `Source` or `Sink` are implemented as well. We'd
// need `dyn (Connector + Source + Sink)` for that, but `Source` and `Sink` are
// actually optional, so it depends on the plugin anyway.
//
// There are two ways to fix this:
//
// * The `new` function returns a `dyn (Connector + Source + Sink)` instead and
//   has fields to make sure `Source` or `Sink` are properly implemented. All of
//   the connectors implement `Source` and `Sink` always, but we can make it
//   optional by adding a marker or something like that.
//
//   Spoiler: that won't work with just `abi_stable` anyway, lol. Only with
//   `cglue`, which makes it possible to have groups of traits. So it *would* be
//   possible, but unnecessarily complicated and not an ideal solution anyway.
// * We add `new_
//
// Sorry for getting your hopes up lol. For the millionth time in a row, it's
// not as easy as I thought.

use abi_stable::{std_types::{
    RBox,
    ROption::{self, RNone, RSome},
    RResult::{RErr, ROk},
    RStr, RString,
}, StableAbi};

use crate::{
    RResult,
    source::{self, SourceContext, RawSource_TO},
    reconnect,
    sink::{self, SinkContext, RawSink_TO},
};

// For more complex types we need to wrap them as opaque types.
pub use crate::wrappers::ConnectorContext;

// pub type ConnectorContext = i32;
#[repr(C)]
#[derive(StableAbi, Default)]
pub struct ConnectorState(RString);

// The low level connector trait used for the plugins, with types from
// abi_stable.
#[abi_stable::sabi_trait]
pub trait RawConnector: Send {
    // Note that the source implementation isn't limited to the connector side
    // anymore. Since the communication details ought to be moved to the runtime
    // rather than the plugin for simplicity's sake, the source is handled at
    // the runtime as well, and it only exports synchronous basic methods.
    //
    // In practical terms, this returns a `dyn RawSource` rather than
    // `SourceAddr`, and the latter is now created on the runtime. The builder
    // isn't passed here anymore either.
    fn create_source(
        &mut self,
        _source_context: SourceContext,
    ) -> RResult<ROption<RawSource_TO<'static, RBox<()>>>> {
        ROk(RNone)
    }

    fn create_sink(
        &mut self,
        _sink_context: SinkContext,
    ) -> RResult<ROption<RawSink_TO<'static, RBox<()>>>> {
        unimplemented!("only sources are implemented for now")
    }

    /* async */
    fn connect(
        &mut self,
        ctx: &ConnectorContext,
        notifier: reconnect::ConnectionLostNotifier,
    ) -> RResult<bool>;

    /* async */
    fn on_start(&mut self, ctx: &ConnectorContext) -> RResult<ConnectorState>;

    /* async */
    fn on_pause(&mut self, _ctx: &ConnectorContext) {}
    /* async */
    fn on_resume(&mut self, _ctx: &ConnectorContext) {}
    /* async */
    fn on_stop(&mut self, _ctx: &ConnectorContext) {}

    fn default_codec(&self) -> RStr<'_>;
}
