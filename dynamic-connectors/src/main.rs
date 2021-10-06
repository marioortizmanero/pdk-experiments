use dynamic_connectors::{find_plugins, setup_plugin};

use std::{env, process};

fn usage() {
    println!("Usage: ./main PLUGINS_DIR");
}

fn main() {
    let mut args = env::args();
    let path = match args.nth(1) {
        Some(path) => path,
        None => {
            usage();
            process::exit(1);
        }
    };

    let plugins = find_plugins(&path).unwrap_or_else(|e| {
        eprintln!("Couldn't find plugins in {}: {}", path, e);
        process::exit(1);
    });

    for plugin in plugins {
        let plugin = plugin.to_str().unwrap();
        let run_plugin = setup_plugin(&plugin).unwrap_or_else(|e| {
            eprintln!("Error when setting up the plugin: {}", e);
            process::exit(1);
        });

        if let Err(e) = run_plugin() {
            eprintln!("Error when running the plugin: {}", e);
            process::exit(1);
        }
    }
}
