pub mod metronome;
pub mod prelude;
pub mod sink;
pub mod source;

use serde::{Deserialize, Serialize};

use crate::{
    connectors::{
        sink::{SinkAddr, SinkContext, SinkManagerBuilder},
        source::{SourceAddr, SourceContext, SourceManagerBuilder},
    },
    errors::Result,
    url::TremorUrl,
};

pub mod reconnect {
    /// NOTE: simplification of the real type
    pub struct ConnectionLostNotifier;
}

/// NOTE: simplification of the real type
pub struct ConnectorContext;

/// state of a connector
#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ConnectorState {
    /// connector has been initialized, but not yet started
    Initialized,
    /// connector is running
    Running,
    /// connector has been paused
    Paused,
    /// connector was stopped
    Stopped,
    /// connector failed to start
    Failed,
}

/// A Connector connects the tremor runtime to the outside world.
///
/// It can be a source of events, as such it is polled for new data.
/// It can also be a sink for events, as such events are sent to it from pipelines.
/// A connector can act as sink and source or just as one of those.
///
/// A connector encapsulates the establishment and maintenance of connections to the outside world,
/// such as tcp connections, file handles etc. etc.
///
/// It is a meta entity on top of the sink and source part.
/// The connector has its own control plane and is an artefact in the tremor repository.
/// It controls the sink and source parts which are connected to the rest of the runtime via links to pipelines.
pub trait Connector: Send {
    /// create a source part for this connector if applicable
    ///
    /// This function is called exactly once upon connector creation.
    /// If this connector does not act as a source, return `Ok(None)`.
    /* async */ fn create_source(
        &mut self,
        _source_context: SourceContext,
        _builder: source::SourceManagerBuilder,
    ) -> Result<Option<source::SourceAddr>> {
        Ok(None)
    }

    /// Create a sink part for this connector if applicable
    ///
    /// This function is called exactly once upon connector creation.
    /// If this connector does not act as a sink, return `Ok(None)`.
    /* async */ fn create_sink(
        &mut self,
        _sink_context: SinkContext,
        _builder: sink::SinkManagerBuilder,
    ) -> Result<Option<sink::SinkAddr>> {
        Ok(None)
    }

    /// Attempt to connect to the outside world.
    /// Return `Ok(true)` if a connection could be established.
    /// This method will be retried if it fails or returns `Ok(false)`.
    ///
    /// To notify the runtime of the main connectivity being lost, a `notifier` is passed in.
    /// Call `notifier.notify().await` as the last thing when you notice the connection is lost.
    /// This is well suited when handling the connection in another task.
    /* async */ fn connect(
        &mut self,
        ctx: &ConnectorContext,
        notifier: reconnect::ConnectionLostNotifier,
    ) -> Result<bool>;

    /// called once when the connector is started
    /// `connect` will be called after this for the first time, leave connection attempts in `connect`.
    /* async */ fn on_start(&mut self, ctx: &ConnectorContext) -> Result<ConnectorState>;

    /// called when the connector pauses
    /* async */ fn on_pause(&mut self, _ctx: &ConnectorContext) {}
    /// called when the connector resumes
    /* async */ fn on_resume(&mut self, _ctx: &ConnectorContext) {}
    /// called when the connector is stopped
    /* async */ fn on_stop(&mut self, _ctx: &ConnectorContext) {}

    /// returns the default codec for this connector
    fn default_codec(&self) -> &str;
}
