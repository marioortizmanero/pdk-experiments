use abi_stable::{
    std_types::{
        RBox, RResult::{ROk, RErr}, ROption::{RSome, RNone}
    }
};

use common_abi_stable_connectors::{
    Result, connectors::{RawConnector_TO, ConnectorContext, reconnect, ConnectorState},
    source::{SourceContext, RawSource},
    sink::{SinkContext}
};

use crate::{
    source::{self, SourceManagerBuilder, SourceAddr},
    sink::{self},
};

// The higher level connector interface, which wraps the raw connector from the
// plugin.
pub struct Connector(pub RawConnector_TO<'static, RBox<()>>);
impl Connector {
    pub async fn create_source(
        &mut self,
        source_context: SourceContext,
        builder: source::SourceManagerBuilder,
    ) -> Result<Option<source::SourceAddr>> {
        match self.0.create_source(source_context) {
            ROk(RSome(raw_source)) => {
                builder.spawn(raw_source, source_context).map(Some)
            },
            ROk(RNone) => Ok(None),
            RErr(err) => Err(err),
        }
    }

    pub async fn create_sink(
        &mut self,
        sink_context: SinkContext,
        builder: sink::SinkManagerBuilder,
    ) -> Result<Option<sink::SinkAddr>> {
        // NOTE: the structure should be almost the same as `create_source`
        unimplemented!("only sources are implemented for now")
    }

    pub async fn connect(
        &mut self,
        ctx: &ConnectorContext,
        notifier: reconnect::ConnectionLostNotifier,
    ) -> Result<bool> {
        self.0.connect(ctx, notifier).into()
    }

    pub async fn on_start(&mut self, ctx: &ConnectorContext) -> Result<ConnectorState> {
        self.0.on_start(ctx).into()
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
