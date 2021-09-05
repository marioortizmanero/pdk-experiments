use native_codec::setup_plugin;

use std::process;

fn main() {
    let run_plugin = setup_plugin();
    if let Err(e) = run_plugin() {
        eprintln!("Error when running the plugin: {}", e);
        process::exit(1);
    }
}
