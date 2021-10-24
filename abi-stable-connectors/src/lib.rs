#![allow(dead_code)] // Prototyping

mod connectors;
mod sink;
mod source;

use crate::{connectors::Connector, source::SourceManagerBuilder};
use abi_stable::library::RootModule;
use anyhow::{anyhow, Result};
use common_abi_stable_connectors::{source::SourceContext, ConnectorMod_Ref};
use std::time::Duration;
use tokio::time;

/// For benchmarking reasons, setting up the plugin and running it is a two step
/// process. Thus, the setup is done when calling this function, and it can be
/// ran when calling the returned closure.
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

    let builder = SourceManagerBuilder::default(); // Spawns source in task
    let context = SourceContext::default(); // Stub for now
    let source_addr = connector
        .create_source(context, builder)
        .await
        .map_err(|e| anyhow!(e))?;

    // The plugin doesn't necessarily have to export a source.
    match source_addr {
        Some(_addr) => {
            // This part of the program acts as the `ConnectorManager`. For
            // simplicity's sake, the `source_addr` is actually useless, so we
            // don't have a way to communicate between the source and the
            // connector. Thus, the source can't be stopped with a message, and
            // it'll just work with a `sleep` in this example.
            println!(
                "Source detected in plugin. It'll run for 10 seconds in a \
                     separate task starting now."
            );
            time::sleep(Duration::from_secs(10)).await
        }
        None => println!("No source in plugin"),
    }

    Ok(())
}
