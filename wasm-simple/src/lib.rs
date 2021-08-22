// use common_ws::{MinBuilder};

use std::{fs, error::Error};

use wasmer::{imports, Instance, Module, Store, NativeFunc};

pub fn run_plugin(path: &str) -> Result<(), Box<dyn Error>> {
    // For reference, Feather also reads the plugins with `fs::read`:
    // https://github.com/feather-rs/feather/blob/07c64678f80ff77be3dbd3d99fbe5558b4e72c97/quill/cargo-quill/src/main.rs#L107
    let module_wat = fs::read(&path)?;
    let store = Store::default();
    let module = Module::new(&store, &module_wat)?;
    // No imports needed; the object will be empty for now
    let import_object = imports! {};
    let instance = Instance::new(&module, &import_object)?;

    println!("With extern:");
    let min_extern: NativeFunc<(i32, i32), i32> = instance.exports.get_native_function("with_extern")?;
    println!("  min(1, 2): {}", min_extern.call(1, 2)?);
    println!("  min(-10, 10): {}", min_extern.call(-10, 10)?);
    println!("  min(2000, 2000): {}", min_extern.call(2000, 2000)?);

    /*
    println!("With extern + dyn:");
    // Sample implementor of the trait that's dynamically dispatched
    struct MinImplementor;
    impl MinBuilder for MinImplementor {
        fn min(&self, a: i32, b: i32) -> i32 {
            a.min(b)
        }
    }
    let builder = Box::new(MinImplementor {}) as Box<dyn MinBuilder>;
    let min_extern_generics: Func<(&Box<dyn MinBuilder>, i32, i32), i32> =
        instance.exports.get("with_extern_generics")?;
    println!("  min(1, 2): {}", min_extern_generics.call(builder, 1, 2)?);
    println!(
        "  min(-10, 10): {}",
        min_extern_generics.call(builder, -10, 10)?
    );
    println!(
        "  min(2000, 2000): {}",
        min_extern_generics.call(builder, 2000, 2000)?
    );
    */

    Ok(())
}

