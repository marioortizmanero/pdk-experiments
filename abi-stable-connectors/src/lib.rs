//! The plugin is given with its full path instead of a directory for more
//! reliable benchmarking.

mod source;
mod sink;
mod connectors;

use abi_stable::library::RootModule;
use anyhow::{anyhow, Result};
use tokio::time;
use std::time::Duration;
use common_abi_stable_connectors::{source::SourceContext, ConnectorMod_Ref};
use crate::{source::SourceManagerBuilder, connectors::Connector};

/// For benchmarking reasons, setting up the plugin and running it is a two step
/// process. Thus, the setup is done when calling this function, and it can be
/// ran when calling the returned closure.
#[tokio::main]
pub async fn run_plugin(path: &str) -> Result<()> {
    let plugin = ConnectorMod_Ref::load_from_file(path.as_ref())?;

    // First we obtain the function pointer, which may fail in case the plugin
    // is incorrectly implemented.
    let new_fn = plugin
        .new()
        .ok_or_else(|| anyhow!("method `new` not found"))?;

    // We initialize the plugin, obtaining a state.
    let raw_connector = new_fn();
    let connector = Connector(raw_connector);
    println!("Default codec: {}", connector.default_codec());

    let builder = SourceManagerBuilder::default();
    let context = SourceContext::default();
    let source_addr = connector.create_source(context, builder).await.map_err(|e| anyhow!(e))?;
    match source_addr {
        Some(addr) => {
            // This part of the program acts as the `ConnectorManager`. For
            // simplicity's sake, the `source_addr` is actually useless, so we
            // don't have a way to communicate between the source and the
            // connector. Thus, the source can't be stopped with a message, and
            // it'll just work with a `sleep` in this example.
            println!("Source detected in plugin. It'll run for 10 seconds in a \
                     separate task starting now.");
            time::sleep(Duration::from_secs(10)).await
        },
        None => println!("No source in plugin")
    }

    Ok(())
}
