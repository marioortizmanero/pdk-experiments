use tremor_core::{MinBuilder, MinBuilderFunction, MinFunction};

use std::{env, process};

use wasmer_runtime::{error::Error as WasmerError, imports, instantiate, Func, Global, Value};

fn usage() {
    println!("Usage: ./main PLUGIN_PATH");
}

static WASM: &'static [u8] =
    include_bytes!("../plugin-sample/target/wasm32-unknown-unknown/release/plugin_sample.wasm");

fn run_plugin(_path: &str) -> Result<(), WasmerError> {
    let import_object = imports! {};
    let instance = instantiate(WASM, &import_object)?;
    let context = instance.context();

    println!("With extern:");
    let min_extern: Func<(i32, i32), i32> = instance.exports.get("with_extern")?;
    println!("  min(1, 2): {}", min_extern.call(1, 2)?);
    println!("  min(-10, 10): {}", min_extern.call(-10, 10)?);
    println!("  min(2000, 2000): {}", min_extern.call(2000, 2000)?);

    println!("With extern:");
    let ptr: Global = instance.exports.get("with_static")?;
    let min_extern = context.memory(ptr.get().to_u128() as u32);
    min_extern.view::<Func<(i32, i32), i32>>();
    println!("what {:?}", min_extern);
    // println!("  min(1, 2): {}", min_extern.call(1, 2)?);
    // println!("  min(-10, 10): {}", min_extern.call(-10, 10)?);
    // println!("  min(2000, 2000): {}", min_extern.call(2000, 2000)?);

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

    if let Err(e) = run_plugin(&path) {
        eprintln!("Error when running the plugin: {}", e)
    }
}
