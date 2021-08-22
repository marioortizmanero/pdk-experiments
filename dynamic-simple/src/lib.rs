use common_ds::{MinBuilder, MinBuilderFunction, MinFunction};

use libloading::Library;

pub fn run_plugin(path: &str, bench_mode: bool) -> Result<(), libloading::Error> {
    unsafe {
        println!("Running plugin");
        let library = Library::new(path)?;

        // The static alternative is only explored to learn more about it, but
        // it doesn't need to be ran in the benchmarks.
        if !bench_mode {
            println!("With static:");
            let min_static = library.get::<*mut MinFunction>(b"with_static\0")?.read();
            println!("  min(1, 2): {}", min_static(1, 2));
            println!("  min(-10, 10): {}", min_static(-10, 10));
            println!("  min(2000, 2000): {}", min_static(2000, 2000));
        }

        println!("With extern:");
        let min_extern = library.get::<MinFunction>(b"with_extern\0")?;
        println!("  min(1, 2): {}", min_extern(1, 2));
        println!("  min(-10, 10): {}", min_extern(-10, 10));
        println!("  min(2000, 2000): {}", min_extern(2000, 2000));

        println!("With extern + dyn:");
        // Sample implementor of the trait that's dynamically dispatched
        struct MinImplementor;
        impl MinBuilder for MinImplementor {
            fn min(&self, a: i32, b: i32) -> i32 {
                a.min(b)
            }
        }
        let builder = Box::new(MinImplementor {}) as Box<dyn MinBuilder>;
        let min_extern_dyn = library.get::<MinBuilderFunction>(b"with_extern_dyn\0")?;
        println!("  min(1, 2): {}", min_extern_dyn(&builder, 1, 2));
        println!("  min(-10, 10): {}", min_extern_dyn(&builder, -10, 10));
        println!(
            "  min(2000, 2000): {}",
            min_extern_dyn(&builder, 2000, 2000)
        );
    }

    Ok(())
}
