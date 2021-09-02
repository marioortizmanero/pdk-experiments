use common_ws::MinBuilder;

use std::{fs, error::Error};

use wasmtime::{Engine, Store, Module, Instance};

pub struct WasmState;

pub fn run_plugin(path: &str) -> Result<(), Box<dyn Error>> {
    let engine = Engine::default();
    let module = Module::from_file(&engine, &path)?;

    let mut store = Store::new(
        &engine,
        WasmState {},
    );
    let imports = [];
    let instance = Instance::new(&mut store, &module, &imports)?;

    println!("With extern:");
    let min_extern = instance.get_typed_func::<(i32, i32), i32, _>(&mut store, "with_extern")?;
    println!("  min(1, 2): {}", min_extern.call(&mut store, (1, 2))?);
    println!("  min(-10, 10): {}", min_extern.call(&mut store, (-10, 10))?);
    println!("  min(2000, 2000): {}", min_extern.call(&mut store, (2000, 2000))?);
    /*

    println!("With extern + dyn:");
    // Sample implementor of the trait that's dynamically dispatched
    struct MinImplementor;
    impl MinBuilder for MinImplementor {
        fn min(&self, a: i32, b: i32) -> i32 {
            a.min(b)
        }
    }
    let builder: Box<dyn MinBuilder> = Box::new(MinImplementor {});

    // The plugin should export a memory under the name "memory" as a
    // convention. This can be checked with:
    //
    // ```
    // dbg!(instance.exports.iter().collect::<Vec<_>>());
    // ```
    let memory = instance.get_memory("memory")?;
    let view = memory.view::<u8>();
    // Converting the parameter into an array of bytes and writing it into
    // Wasm's memory directly.
    let bytes = bincode::serialize(&builder)?;
    let len = bytes.len();
    let builder_ptr = 1;
    // TODO: does this work?
    // view.copy_from(bytes);
    for (cell, byte) in view[builder_ptr..len + builder_ptr].iter().zip(bytes.iter()) {
        cell.set(*byte)
    }

    println!("is wasi: {}", wasmer_wasi::is_wasi_module(&module));

    // Finally we can call the plugin's exported function with the pointer we
    // just wrote to.
    let min_extern_generics = instance.exports.get_native_function::<(i32, i32, i32), i32>("with_extern_dyn")?;

    let builder_ptr = builder_ptr as i32;
    match min_extern_generics.call(builder_ptr, 1, 2) {
        Ok(_) => println!("ok"),
        Err(e) => println!("err {} // {:#?} :(", e.message(), e.trace())
    }
    println!("  min(1, 2): {}", min_extern_generics.call(builder_ptr, 1, 2)?);
    println!(
        "  min(-10, 10): {}",
        min_extern_generics.call(builder_ptr, -10, 10)?
    );
    println!(
        "  min(2000, 2000): {}",
        min_extern_generics.call(builder_ptr, 2000, 2000)?
    );
    */

    Ok(())
}

