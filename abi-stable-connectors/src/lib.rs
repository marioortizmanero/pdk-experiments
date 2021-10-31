#![allow(dead_code)] // Prototyping

mod connectors;
mod sink;
mod source;

use crate::{connectors::Connector, sink::SinkManagerBuilder, source::SourceManagerBuilder};
use abi_stable::{erased_types::TD_Opaque, library::RootModule};
use anyhow::{anyhow, Result};
use common_abi_stable_connectors::{
    event::{EventSerializer, RawEventSerializer_TO},
    sink::SinkContext,
    source::SourceContext,
    ConnectorMod_Ref,
};
use std::time::Duration;
use tokio::time;

/// For benchmarking reasons, setting up the plugin and running it is a two step
/// process. Thus, the setup is done when calling this function, and it can be
/// ran when calling the returned closure.
///
/// This program acts as the `ConnectorManager`. For simplicity's sake, the
/// `source_addr` and `sink_addr` are actually useless, so we don't have a way
/// to communicate between the source/sink and the connector; they'll stop by
/// themselves for the demo.
#[tokio::main]
pub async fn run_plugin(path: &str) -> Result<()> {
    // This entire function will act as the `connector_task` function in the
    // `ConnectorManager`.
    let plugin = ConnectorMod_Ref::load_from_file(path.as_ref())?;

    // First we obtain the pointer to the initialization function, which may
    // fail in case the plugin is incorrectly implemented.
    let new_fn = plugin
        .new()
        .ok_or_else(|| anyhow!("method `new` not found"))?;

    // We initialize the plugin, obtaining a raw dynamic connector type. In
    // order to use it easily from now on, we wrap it under a `Connector`
    // concrete type.
    let raw_connector = new_fn();
    let mut connector = Connector(raw_connector);
    println!("Default codec: {}", connector.default_codec());

    // Note that plugins don't necessarily have to export a sink or source
    launch_source(&mut connector).await?;
    launch_sink(&mut connector).await?;

    // Since there's no communication with the source or sink, we'll just leave
    // them running for a few seconds for the demo.
    println!(
        "The runtime will now wait for 15 seconds with the source and sink \
            running in a separate task."
    );
    time::sleep(Duration::from_secs(15)).await;
    println!("Stopping");

    Ok(())
}

async fn launch_source(connector: &mut Connector) -> Result<()> {
    // The builder will already spawn the source in a separate task.
    let source_builder = SourceManagerBuilder::default(); // Spawns source in task
    let source_context = SourceContext::default(); // Stub for now

    connector
        .create_source(source_context, source_builder)
        .await
        .map_err(|e| anyhow!(e))?;

    Ok(())
}

async fn launch_sink(connector: &mut Connector) -> Result<()> {
    // Constructing the event serializer and turning it into an opaque type so
    // that it can be passed through the FFI boundary.
    //
    // In this case it doesn't make sense to downcast back to an
    // `EventSerializer` because the full functionality is already exported in
    // `RawEventSerializer`, so we use `TD_Opaque`.
    let serializer = EventSerializer::build();
    let serializer = RawEventSerializer_TO::from_value(serializer, TD_Opaque);

    // The builder will already spawn the source in a separate task.
    let sink_builder = SinkManagerBuilder { serializer }; // Spawns source in task
    let sink_context = SinkContext::default(); // Stub for now

    connector
        .create_sink(sink_context, sink_builder)
        .await
        .map_err(|e| anyhow!(e))?;

    Ok(())
}
