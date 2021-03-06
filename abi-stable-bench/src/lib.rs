//! The plugin is given with its full path instead of a directory for more
//! reliable benchmarking.

use abi_stable::library::RootModule;
use anyhow::{anyhow, Result};
use common_sabi_simple::MinMod_Ref;

/// For benchmarking reasons, setting up the plugin and running it is a two step
/// process. Thus, the setup is done when calling this function, and it can be
/// ran when calling the returned closure.
pub fn setup_plugin(path: &str) -> Result<impl FnMut(i32, i32) -> i32> {
    let plugin = MinMod_Ref::load_from_file(path.as_ref())?;

    // First we obtain the function pointer, which may fail in case the plugin
    // is incorrectly implemented.
    let new_fn = plugin
        .new()
        .ok_or_else(|| anyhow!("method `new` not found"))?;

    // We initialize the plugin, obtaining a state.
    let mut state = new_fn();

    // Same for the `min` function
    let min_fn = plugin
        .min()
        .ok_or_else(|| anyhow!("method `min` not found"))?;

    Ok(move |a, b| min_fn(&mut state, a, b))
}
