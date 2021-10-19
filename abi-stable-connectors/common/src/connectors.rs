use abi_stable::{std_types::{
    RBox,
    ROption::{self, RNone, RSome},
    RResult::{RErr, ROk},
    RStr, RString,
}, StableAbi};

use crate::source::{self, SourceContext};

// For more complex types we need to wrap them as opaque types.
use crate::wrappers::ConnectorContext;

// Stubs for the original trait. We can't use `()` because it's not FFI-safe.
pub mod sink {
    use super::{StableAbi, RString};

    #[repr(C)]
    #[derive(StableAbi, Default)]
    pub struct SinkManagerBuilder(RString);

    #[repr(C)]
    #[derive(StableAbi, Default)]
    pub struct SinkAddr(RString);
}
pub mod reconnect {
    use super::{StableAbi, RString};

    #[repr(C)]
    #[derive(StableAbi, Default)]
    pub struct ConnectionLostNotifier(RString);
}
// pub type ConnectorContext = i32;
#[repr(C)]
#[derive(StableAbi, Default)]
pub struct ConnectorState(RString);
#[repr(C)]
#[derive(StableAbi, Default)]
pub struct SinkContext(RString);
pub type RResult<T> = abi_stable::std_types::RResult<T, RString>;
pub type Result<T> = std::result::Result<T, RString>;

// The low level connector trait used for the plugins, with types from
// abi_stable.
#[abi_stable::sabi_trait]
pub trait RawConnector: Send {
    /* async */
    fn create_source(
        &mut self,
        _source_context: SourceContext,
        _builder: source::SourceManagerBuilder,
    ) -> RResult<ROption<source::SourceAddr>> {
        ROk(RNone)
    }

    /* async */
    fn create_sink(
        &mut self,
        _sink_context: SinkContext,
        _builder: sink::SinkManagerBuilder,
    ) -> RResult<ROption<sink::SinkAddr>> {
        ROk(RNone)
    }

    /* async */
    fn connect(
        &mut self,
        ctx: &ConnectorContext,
        notifier: reconnect::ConnectionLostNotifier,
    ) -> RResult<bool>;

    /* async */
    fn on_start(&mut self, ctx: &ConnectorContext) -> RResult<ConnectorState>;

    /* async */
    fn on_pause(&mut self, _ctx: &ConnectorContext) {}
    /* async */
    fn on_resume(&mut self, _ctx: &ConnectorContext) {}
    /* async */
    fn on_stop(&mut self, _ctx: &ConnectorContext) {}

    fn default_codec(&self) -> RStr<'_>;
}

// The higher level connector interface, which wraps the raw connector from the
// plugin.
pub struct Connector(pub RawConnector_TO<'static, RBox<()>>);
impl Connector {
    pub async fn create_source(
        &mut self,
        source_context: SourceContext,
        builder: source::SourceManagerBuilder,
    ) -> Result<Option<source::SourceAddr>> {
        match self.0.create_source(source_context, builder) {
            ROk(RSome(source)) => {
                let source = Source(source);
                builder.spawn(source, source_context).map(Some)
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
