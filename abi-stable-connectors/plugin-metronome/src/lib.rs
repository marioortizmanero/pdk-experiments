use abi_stable::{
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
    connectors::{ConnectorContext, ConnectorState, RawConnector, RawConnector_TO},
    reconnect,
    source::{RawSource, RawSource_TO, SourceContext, SourceReply},
    ConnectorMod, ConnectorMod_Ref, RResult,
};

use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
struct Metronome {
    interval: Duration,
    next: Instant,
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
        let now = Instant::now();
        if self.next < now {
            self.next = now + self.interval;

            let data = format!("Next event at {:?}, now {:?}", self.next, now);
            ROk(SourceReply::Data(data.into()))
        } else {
            let remaining = (self.next - now).as_millis() as u64;
            ROk(SourceReply::Empty(remaining))
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
    let metronome = Metronome {
        interval: Duration::from_secs(1),
        next: Instant::now(),
    };
    // We don't need to be able to downcast the connector back to the original
    // type, so we just pass it as an opaque type.
    RawConnector_TO::from_value(metronome, TD_Opaque)
}
