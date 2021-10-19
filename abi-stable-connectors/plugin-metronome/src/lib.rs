use abi_stable::{
    export_root_module,
    rvec,
    type_level::downcasting::TD_Opaque,
    prefix_type::PrefixTypeTrait,
    rstr, sabi_extern_fn,
    std_types::{RBox, RStr, RResult::ROk},
};

use common_abi_stable_connectors::{
    connectors::{RawConnector, RawConnector_TO, RResult, ConnectorContext},
    source::{SourceReply, RawSource}
};

struct Metronome;

impl RawConnector for Metronome {
    fn create_source(
        &mut self,
        source_context: SourceContext,
    ) -> Result<Option<todo!()>> {
        let metronome = self.clone();
        // We don't need to be able to downcast the connector back to the original
        // type, so we just pass it as an opaque type.
        RawSource_TO::from_value(metronome, TD_Opaque)
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
        ROk(0)
    }

    fn default_codec(&self) -> RStr<'_> {
        rstr!("application/json")
    }
}

impl RawSource for Metronome {
    fn pull_data(&mut self, pull_id: u64, _ctx: &SourceContext) -> Result<SourceReply> {
        let now = nanotime();
        if self.next < now {
            let data = rvec![];
            Ok(SourceReply::Data(data))
        } else {
            Ok(SourceReply::Sleep(self.next - now))
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
    let metronome = Metronome;
    // We don't need to be able to downcast the connector back to the original
    // type, so we just pass it as an opaque type.
    RawConnector_TO::from_value(metronome, TD_Opaque)
}
