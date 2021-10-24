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
// plugin. This is only needed in the runtime itself because the common library
// only needs the basic components that are shared.
//
// This may introduce a very small overhead in some cases, so maybe it's worth
// not converting types from `abi_stable` into `std` (it's not really so bad).
// For functions like `create_source` and similars it's not worth it because
// they're only ran once.
pub struct Connector(pub RawConnector_TO<'static, RBox<()>>);
impl Connector {
    pub async fn create_source(
        &mut self,
        source_context: SourceContext,
        builder: source::SourceManagerBuilder,
    ) -> Result<Option<source::SourceAddr>> {
        match self.0.create_source(source_context.clone()) {
            ROk(RSome(raw_source)) => builder.spawn(raw_source, source_context).map(Some),
            ROk(RNone) => Ok(None),
            RErr(err) => Err(err.into()),
        }
    }

    pub async fn create_sink(
        &mut self,
        _sink_context: SinkContext,
        _builder: sink::SinkManagerBuilder,
    ) -> Result<Option<sink::SinkAddr>> {
        // TODO: the structure should be almost the same as `create_source`
        unimplemented!("only sources are implemented for now")
    }

    pub async fn connect(
        &mut self,
        ctx: &ConnectorContext,
        notifier: reconnect::ConnectionLostNotifier,
    ) -> Result<bool> {
        self.0
            .connect(ctx, notifier)
            .map_err(Into::into) // RBoxError -> Box<dyn Error>
            .into() // RResult -> Result
    }

    pub async fn on_start(&mut self, ctx: &ConnectorContext) -> Result<ConnectorState> {
        self.0
            .on_start(ctx)
            .map_err(Into::into) // RBoxError -> Box<dyn Error>
            .into() // RResult -> Result
    }

    pub async fn on_pause(&mut self, ctx: &ConnectorContext) {
        self.0.on_pause(ctx)
    }

    pub async fn on_resume(&mut self, ctx: &ConnectorContext) {
        self.0.on_resume(ctx)
    }

    pub async fn on_stop(&mut self, ctx: &ConnectorContext) {
        self.0.on_stop(ctx)
    }

    pub fn default_codec(&self) -> &str {
        self.0.default_codec().into()
    }
}
