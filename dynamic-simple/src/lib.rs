use common_ds::MinFunction;

use libloading::Library;

pub fn run_plugin(path: &str) -> Result<(), libloading::Error> {
    unsafe {
        println!("Running plugin");
        let library = Library::new(path)?;

        println!("With static:");
        let min_static = library.get::<*mut MinFunction>(b"with_static\0")?.read();
        println!("  min(1, 2): {}", min_static(1, 2));
        println!("  min(-10, 10): {}", min_static(-10, 10));
        println!("  min(2000, 2000): {}", min_static(2000, 2000));

        println!("With extern:");
        let min_extern = library.get::<MinFunction>(b"with_extern\0")?;
        println!("  min(1, 2): {}", min_extern(1, 2));
        println!("  min(-10, 10): {}", min_extern(-10, 10));
        println!("  min(2000, 2000): {}", min_extern(2000, 2000));
    }

    Ok(())
}
