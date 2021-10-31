//! Simple plugin to test panic safety.

use abi_stable::{
    rvec,
    export_root_module,
    prefix_type::PrefixTypeTrait,
    rstr, sabi_extern_fn,
    std_types::{
        RBox,
        ROption::{self, RSome},
        RResult::ROk,
        RStr,
    },
    type_level::downcasting::TD_Opaque,
};

use common_abi_stable_connectors::{
    value::Value,
    connectors::{ConnectorContext, ConnectorState, RawConnector, RawConnector_TO},
    event::{Event, OpaqueEventSerializer},
    reconnect,
    sink::{RawSink, RawSink_TO, ResultVec, SinkContext, SinkReply},
    util::MayPanic::{self, NoPanic},
    ConnectorMod, ConnectorMod_Ref, RResult,
};

use std::panic;

// Note that the struct itself in the plugin doesn't need to use `abi_stable`,
// since we're using `dyn RawConnector` as the public interface rather than
// `Metronome`.
#[derive(Clone, Debug)]
struct Reverse;

impl RawConnector for Reverse {
    fn create_sink(
        &mut self,
        _sink_context: SinkContext,
    ) -> MayPanic<RResult<ROption<RawSink_TO<'static, RBox<()>>>>> {
        // NOTE: we don't want panics through FFI! That would be undefined
        // behaviour, so we have to handle them -- manually for now.
        panic::catch_unwind(|| {
            let reverse = self.clone();
            // We don't need to be able to downcast the connector back to the original
            // type, so we just pass it as an opaque type.
            ROk(RSome(RawSink_TO::from_value(reverse, TD_Opaque)))
        })
        .into()
    }

    fn connect(
        &mut self,
        _ctx: &ConnectorContext,
        _notifier: reconnect::ConnectionLostNotifier,
    ) -> MayPanic<RResult<bool>> {
        NoPanic(ROk(true))
    }

    fn on_start(&mut self, _ctx: &ConnectorContext) -> MayPanic<RResult<ConnectorState>> {
        NoPanic(ROk(ConnectorState::default()))
    }

    fn default_codec(&self) -> RStr<'_> {
        rstr!("application/json")
    }
}

impl RawSink for Reverse {
    fn on_event(
        &mut self,
        _input: RStr<'_>,
        event: Event,
        _ctx: &SinkContext,
        _serializer: &mut OpaqueEventSerializer,
        _start: u64,
    ) -> MayPanic<ResultVec> where {
        match event.data {
            Value::String(s) => {
                // Note that this isn't how a string should be reversed
                // properly; rather than iterating the characters it should be
                // UTF-8 graphemes.
                let reverse = s.trim().chars().rev().collect::<String>();
                println!("Reverse: {}", reverse);

                NoPanic(ROk(rvec![SinkReply::Ack]))
            },
            Value::Integer(_) => {
                NoPanic(ROk(rvec![SinkReply::Fail]))
            }
        }
    }
}

/// Exports the root module of this library.
///
/// This code isn't run until the layout of the type it returns is checked.
#[export_root_module]
fn instantiate_root_module() -> ConnectorMod_Ref {
    ConnectorMod { new }.leak_into_prefix()
}

#[sabi_extern_fn]
pub fn new() -> RawConnector_TO<'static, RBox<()>> {
    RawConnector_TO::from_value(Reverse, TD_Opaque)
}