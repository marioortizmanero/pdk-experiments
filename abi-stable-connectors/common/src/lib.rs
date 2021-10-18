use abi_stable::{
    declare_root_module_statics,
    library::RootModule,
    package_version_strings,
    sabi_types::VersionStrings,
    std_types::{
        RBox,
        ROption::{self, RNone},
        RResult::{ROk, RErr},
        RStr, RString,
    },
    StableAbi,
};

// Stubs for the original trait. We can't use `()` because it's not FFI-safe.
pub mod source {
    pub type SourceManagerBuilder = i32;
    pub type SourceAddr = i32;
}
pub mod sink {
    pub type SinkManagerBuilder = i32;
    pub type SinkAddr = i32;
}
pub mod reconnect {
    pub type ConnectionLostNotifier = i32;
}
pub type ConnectorContext = i32;
pub type ConnectorState = i32;
pub type SinkContext = i32;
pub type SourceContext = i32;
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
            ROk(source) => Ok(source.into()),
            RErr(err) => Err(err)
        }
    }

    pub async fn create_sink(
        &mut self,
        sink_context: SinkContext,
        builder: sink::SinkManagerBuilder,
    ) -> Result<Option<sink::SinkAddr>> {
        match self.0.create_sink(sink_context, builder) {
            ROk(sink) => Ok(sink.into()),
            RErr(err) => Err(err)
        }
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

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix))]
pub struct ConnectorMod {
    pub new: extern "C" fn() -> RawConnector_TO<'static, RBox<()>>,
}

// Marking `MinMod` as the main module in this plugin. Note that `MinMod_Ref` is
// a pointer to the prefix of `MinMod`.
impl RootModule for ConnectorMod_Ref {
    // The name of the dynamic library
    const BASE_NAME: &'static str = "connector";
    // The name of the library for logging and similars
    const NAME: &'static str = "connector";
    // The version of this plugin's crate
    const VERSION_STRINGS: VersionStrings = package_version_strings!();

    // Implements the `RootModule::root_module_statics` function, which is the
    // only required implementation for the `RootModule` trait.
    declare_root_module_statics! {ConnectorMod_Ref}
}
