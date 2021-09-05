use dynamic_codec::setup_plugin;

use std::{env, process};

fn usage() {
    println!("Usage: ./main PLUGIN_PATH");
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

    let run_plugin = match setup_plugin(&path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error when setting up the plugin: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = run_plugin() {
        eprintln!("Error when running the plugin: {}", e);
        process::exit(1);
    }
}
