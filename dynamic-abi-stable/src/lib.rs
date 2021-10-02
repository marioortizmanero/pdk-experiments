use abi_stable::library::RootModule;
use common_dsabi::MinMod_Ref;
use anyhow::{Result, anyhow};

pub fn run_plugin(path: &str) -> Result<()> {
    let plugin = MinMod_Ref::load_from_file(path.as_ref())?;
    println!("Loading plugin {}", MinMod_Ref::NAME);

    // First we obtain the function pointer, which may fail in case the plugin
    // is incorrectly implemented.
    let new_fn = plugin.new().ok_or(anyhow!("method `new` not found"))?;

    // We initialize the plugin, obtaining a state.
    let mut state = new_fn();

    // Same for the `min` function
    let min_fn = plugin.min().ok_or(anyhow!("method `min` not found"))?;

    println!("initial state: {:?}", state);
    println!("  min(1, 2): {}", min_fn(&mut state, 1, 2));
    println!("  min(-10, 10): {}", min_fn(&mut state, -10, 10));
    println!("  min(2000, 2000): {}", min_fn(&mut state, 2000, 2000));
    println!("final state: {:?}", state);

    Ok(())
}
