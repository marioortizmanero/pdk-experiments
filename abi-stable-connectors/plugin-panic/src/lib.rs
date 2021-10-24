//! Simple plugin to test panic safety.

use abi_stable::{
    export_root_module,
    prefix_type::PrefixTypeTrait,
    rstr, sabi_extern_fn,
    std_types::{RBox, ROption, RResult::ROk, RStr},
    type_level::downcasting::TD_Opaque,
};

use common_abi_stable_connectors::{
    connectors::{ConnectorContext, ConnectorState, RawConnector, RawConnector_TO},
    reconnect,
    source::{RawSource, RawSource_TO, SourceContext, SourceReply},
    util::MayPanic::{self, NoPanic},
    ConnectorMod, ConnectorMod_Ref, RResult,
};

use std::panic;

// Note that the struct itself in the plugin doesn't need to use `abi_stable`,
// since we're using `dyn RawConnector` as the public interface rather than
// `Metronome`.
#[derive(Clone, Debug)]
struct Panic;

impl RawConnector for Panic {
    fn create_source(
        &mut self,
        _source_context: SourceContext,
    ) -> MayPanic<RResult<ROption<RawSource_TO<'static, RBox<()>>>>> {
        panic::catch_unwind(|| panic!("Oh no! Who would've known the `plugin-panic` panicked!"))
            .into()
    }

    /* async */
    fn connect(
        &mut self,
        _ctx: &ConnectorContext,
        _notifier: reconnect::ConnectionLostNotifier,
    ) -> MayPanic<RResult<bool>> {
        NoPanic(ROk(true))
    }

    /* async */
    fn on_start(&mut self, _ctx: &ConnectorContext) -> MayPanic<RResult<ConnectorState>> {
        NoPanic(ROk(ConnectorState::default()))
    }

    fn default_codec(&self) -> RStr<'_> {
        rstr!("application/json")
    }
}

impl RawSource for Panic {
    fn pull_data(&mut self, _pull_id: u64, _ctx: &SourceContext) -> MayPanic<RResult<SourceReply>> {
        panic::catch_unwind(|| panic!("Oh no! Who would've known the `plugin-panic` panicked!"))
            .into()
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
    RawConnector_TO::from_value(Panic, TD_Opaque)
}
