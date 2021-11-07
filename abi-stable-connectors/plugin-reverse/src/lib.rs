//! Simple plugin to test panic safety.

use abi_stable::{
    export_root_module,
    prefix_type::PrefixTypeTrait,
    rstr, rvec, sabi_extern_fn,
    std_types::{
        RBox,
        ROption::{self, RSome},
        RResult::ROk,
        RStr,
    },
    type_level::downcasting::TD_Opaque,
};

use common_abi_stable_connectors::{
    connectors::{
        Attempt, ConnectorContext, ConnectorState, RawConnector, RawConnector_TO, TremorUrl,
    },
    event::{Event, OpaqueEventSerializer},
    sink::{RawSink, RawSink_TO, ResultVec, SinkContext, SinkReply},
    util::MayPanic::{self, NoPanic},
    value::Value,
    ConnectorMod, ConnectorMod_Ref, RResult,
};

use std::panic;

// Note that the struct itself in the plugin doesn't need to use `abi_stable`,
// since we're using `dyn RawConnector` as the public interface rather than
// `Reverse` (it's an opaque type).
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

    fn connect(&mut self, _ctx: &ConnectorContext, _attempt: &Attempt) -> MayPanic<RResult<bool>> {
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
            }
            Value::Integer(_) => NoPanic(ROk(rvec![SinkReply::Fail])),
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
pub fn new(_url: &TremorUrl, _config: ROption<Value>) -> RawConnector_TO<'static, RBox<()>> {
    // We don't need downcasting back to the original type, mainly because the
    // runtime doesn't have access to it. Thus, we use `TD_Opaque` always.
    RawConnector_TO::from_value(Reverse, TD_Opaque)
}
