use abi_stable::{
    export_root_module,
    prefix_type::PrefixTypeTrait,
    rstr, sabi_extern_fn,
    std_types::{
        RBox,
        ROption::{self, RSome},
        RResult::ROk,
        RStr, RString,
    },
    type_level::downcasting::TD_Opaque,
};

use common_abi_stable_connectors::{
    connectors::{ConnectorContext, ConnectorState, RawConnector, RawConnector_TO},
    reconnect,
    source::{RawSource, RawSource_TO, SourceContext, SourceReply},
    ConnectorMod, ConnectorMod_Ref, RResult,
};

use std::time::SystemTime;

/// Get a nanosecond timestamp
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn nanotime() -> u64 {
    // TODO we want to turn this into u128 eventually
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        // ALLOW: If this happens, now() is BEFORE the unix epoch, this is so bad panicing is the least of our problems
        .expect("Our time was before the unix epoc, this is really bad!")
        .as_nanos() as u64
}

#[derive(Clone, Debug, Default)]
struct Metronome {
    next: u64,
}

impl RawConnector for Metronome {
    // TODO: remove `RResult` here?
    fn create_source(
        &mut self,
        _source_context: SourceContext,
    ) -> RResult<ROption<RawSource_TO<'static, RBox<()>>>> {
        let metronome = self.clone();
        // We don't need to be able to downcast the connector back to the original
        // type, so we just pass it as an opaque type.
        ROk(RSome(RawSource_TO::from_value(metronome, TD_Opaque)))
    }

    /* async */
    fn connect(
        &mut self,
        _ctx: &ConnectorContext,
        _notifier: reconnect::ConnectionLostNotifier,
    ) -> RResult<bool> {
        ROk(true)
    }

    /* async */
    fn on_start(&mut self, _ctx: &ConnectorContext) -> RResult<ConnectorState> {
        ROk(ConnectorState::default())
    }

    fn default_codec(&self) -> RStr<'_> {
        rstr!("application/json")
    }
}

impl RawSource for Metronome {
    fn pull_data(&mut self, _pull_id: u64, _ctx: &SourceContext) -> RResult<SourceReply> {
        let now = nanotime();
        if self.next < now {
            let data = RString::new();
            ROk(SourceReply::Data(data))
        } else {
            ROk(SourceReply::Sleep(self.next - now))
        }
    }

    fn is_transactional(&self) -> bool {
        false
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
    let metronome = Metronome::default();
    // We don't need to be able to downcast the connector back to the original
    // type, so we just pass it as an opaque type.
    RawConnector_TO::from_value(metronome, TD_Opaque)
}
