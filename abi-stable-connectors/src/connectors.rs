use crate::{sink, source};

use abi_stable::std_types::{
    RBox,
    ROption::{RNone, RSome},
    RResult::{RErr, ROk},
};
use common_abi_stable_connectors::{
    connectors::{ConnectorContext, ConnectorState, RawConnector_TO},
    reconnect,
    sink::SinkContext,
    source::SourceContext,
    Result,
};

// The higher level connector interface, which wraps the raw connector from the
// plugin. This is only used in the runtime itself because the common library
// only needs the basic components that are shared.
//
// This may introduce a very small overhead in some cases, so maybe it's worth
// not converting types from `abi_stable` into `std`; it's not really so bad
// using them after all. Though for functions like `create_source` and similars
// it's not worth complicating ourselves because they're only ran once.
//
// Note that the implementaion currently ignores the `MayPanic` types by
// unwrapping them, but this could be handled gracefully to avoid aborting the
// runtime. This is safe because thanks to `MayPanic`, the panic will occur here
// in the runtime rather than in the plugin.
pub struct Connector(pub RawConnector_TO<'static, RBox<()>>);
impl Connector {
    pub async fn create_source(
        &mut self,
        source_context: SourceContext,
        builder: source::SourceManagerBuilder,
    ) -> Result<Option<source::SourceAddr>> {
        match self.0.create_source(source_context.clone()).unwrap() {
            ROk(RSome(raw_source)) => builder.spawn(raw_source, source_context).map(Some),
            ROk(RNone) => Ok(None),
            RErr(err) => Err(err.into()),
        }
    }

    pub async fn create_sink(
        &mut self,
        sink_context: SinkContext,
        builder: sink::SinkManagerBuilder,
    ) -> Result<Option<sink::SinkAddr>> {
        match self.0.create_sink(sink_context.clone()).unwrap() {
            ROk(RSome(raw_sink)) => builder.spawn(raw_sink, sink_context).map(Some),
            ROk(RNone) => Ok(None),
            RErr(err) => Err(err.into()),
        }
    }

    pub async fn connect(
        &mut self,
        ctx: &ConnectorContext,
        notifier: reconnect::ConnectionLostNotifier,
    ) -> Result<bool> {
        self.0
            .connect(ctx, notifier)
            .unwrap()
            .map_err(Into::into) // RBoxError -> Box<dyn Error>
            .into() // RResult -> Result
    }

    pub async fn on_start(&mut self, ctx: &ConnectorContext) -> Result<ConnectorState> {
        self.0
            .on_start(ctx)
            .unwrap()
            .map_err(Into::into) // RBoxError -> Box<dyn Error>
            .into() // RResult -> Result
    }

    pub async fn on_pause(&mut self, ctx: &ConnectorContext) {
        self.0.on_pause(ctx).unwrap()
    }

    pub async fn on_resume(&mut self, ctx: &ConnectorContext) {
        self.0.on_resume(ctx).unwrap()
    }

    pub async fn on_stop(&mut self, ctx: &ConnectorContext) {
        self.0.on_stop(ctx).unwrap()
    }

    pub fn default_codec(&self) -> &str {
        self.0.default_codec().into()
    }
}
