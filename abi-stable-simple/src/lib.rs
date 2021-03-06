use abi_stable::library::RootModule;
use anyhow::Result;
use common_sabi_simple::MinMod_Ref;

pub fn run_plugin(path: &str) -> Result<()> {
    let plugin = MinMod_Ref::load_from_directory(path.as_ref())?;
    println!("Loading plugin {}", MinMod_Ref::NAME);

    // First we obtain the function pointer. This is not an `Option` because
    // `new` is defined before `min`, the last prefix field.
    let new_fn = plugin.new();

    // We initialize the plugin, obtaining a state.
    let mut state = new_fn();

    // Same for the `min` function
    let min_fn = plugin.min();

    println!("initial state: {:?}", state);
    println!("  min(1, 2): {}", min_fn(&mut state, 1, 2));
    println!("  min(-10, 10): {}", min_fn(&mut state, -10, 10));
    println!("  min(2000, 2000): {}", min_fn(&mut state, 2000, 2000));
    println!("final state: {:?}", state);

    Ok(())
}
