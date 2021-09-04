use std::error::Error;

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

    println!("Running plugin:");
    let min_extern = instance.get_typed_func::<(i32, i32), i32, _>(&mut store, "with_extern")?;
    println!("  min(1, 2): {}", min_extern.call(&mut store, (1, 2))?);
    println!("  min(-10, 10): {}", min_extern.call(&mut store, (-10, 10))?);
    println!("  min(2000, 2000): {}", min_extern.call(&mut store, (2000, 2000))?);

    Ok(())
}

