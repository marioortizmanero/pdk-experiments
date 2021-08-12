use tremor_core::MinFunction;

use std::{env, process};

use libloading::Library;

fn usage() {
    println!("Usage: ./main PLUGIN_PATH");
}

fn run_plugin(path: &str) -> Result<(), libloading::Error> {
    unsafe {
        let library = Library::new(path)?;
        let min = library.get::<*mut MinFunction>(b"plugin_function\0")?.read();
        println!("Running plugin:");
        println!("  min(1, 2): {}", min(1, 2));
        println!("  min(-10, 10): {}", min(-10, 10));
        println!("  min(2000, 2000): {}", min(2000, 2000));
    }

    Ok(())
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

    if let Err(e) =  run_plugin(&path) {
        eprintln!("Error when loading the library: {}", e)
    }
}
