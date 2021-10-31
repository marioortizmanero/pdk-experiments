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
    util::MayPanic::{self, NoPanic},
    ConnectorMod, ConnectorMod_Ref, RResult,
};

use std::{
    panic::{self, AssertUnwindSafe},
    time::{Duration, Instant},
};

// Note that the struct itself in the plugin doesn't need to use `abi_stable`,
// since we're using `dyn RawConnector` as the public interface rather than
// `Metronome` (it's an opaque type).
#[derive(Clone, Debug)]
struct Metronome {
    interval: Duration,
    next: Instant,
}

impl RawConnector for Metronome {
    fn create_source(
        &mut self,
        _source_context: SourceContext,
    ) -> MayPanic<RResult<ROption<RawSource_TO<'static, RBox<()>>>>> {
        // NOTE: we don't want panics through FFI! That would be undefined
        // behaviour, so we have to handle them -- manually for now.
        panic::catch_unwind(|| {
            let metronome = self.clone();
            // We don't need to be able to downcast the connector back to the original
            // type, so we just pass it as an opaque type.
            ROk(RSome(RawSource_TO::from_value(metronome, TD_Opaque)))
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

impl RawSource for Metronome {
    /// NOTE: Unfortunately, mutable types (`self` here) are not panic-safe by
    /// default (they don't implement `UnwindSafe`), which means that they can't
    /// just be used inside a `catch_unwind` closure:
    ///
    /// https://doc.rust-lang.org/stable/std/panic/trait.UnwindSafe.html#who-implements-unwindsafe
    ///
    /// There are two ways to approach this:
    ///
    /// * Use an ugly workaround to only apply mutability in a `.map` after the
    ///   `catch_unwind` call.
    /// * Use `AssertUnwindSafe` if we're sure we can guarantee that the code is
    ///   *minimally* exception-safe, i.e. it doesn't violate memory safety in
    ///   case a panic occurs
    ///   (https://doc.rust-lang.org/nomicon/exception-safety.html#exception-safety).
    ///
    /// This code doesn't cause memory unsafety during unwinding, so we can just
    /// use `AssertUnwindSafe` in this case.
    fn pull_data(&mut self, _pull_id: u64, _ctx: &SourceContext) -> MayPanic<RResult<SourceReply>> {
        panic::catch_unwind(AssertUnwindSafe(|| {
            // Even though this functionality may seem simple and panic-free,
            // it could occur in the addition operation, for example.
            let now = Instant::now();
            if self.next < now {
                self.next = now + self.interval;
                let data = format!("Next event at {:?}, now {:?}", self.next, now);

                ROk(SourceReply::Data(data.into()))
            } else {
                let remaining = (self.next - now).as_millis() as u64;

                ROk(SourceReply::Empty(remaining))
            }
        }))
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
    let metronome = Metronome {
        interval: Duration::from_secs(1),
        next: Instant::now(),
    };
    // We don't need downcasting back to the original type, mainly because the
    // runtime doesn't have access to it. Thus, we use `TD_Opaque` always.
    RawConnector_TO::from_value(metronome, TD_Opaque)
}
