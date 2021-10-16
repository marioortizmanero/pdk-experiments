use common_ds::MinFunction;

use libloading::Library;

pub fn run_plugin(path: &str) -> Result<(), libloading::Error> {
    unsafe {
        log::info!("Running plugin");
        let library = Library::new(path)?;

        log::info!("With static:");
        let min_static = library.get::<*mut MinFunction>(b"with_static\0")?.read();
        log::info!("  min(1, 2): {}", min_static(1, 2));
        log::info!("  min(-10, 10): {}", min_static(-10, 10));
        log::info!("  min(2000, 2000): {}", min_static(2000, 2000));

        log::info!("With extern:");
        let min_extern = library.get::<MinFunction>(b"with_extern\0")?;
        log::info!("  min(1, 2): {}", min_extern(1, 2));
        log::info!("  min(-10, 10): {}", min_extern(-10, 10));
        log::info!("  min(2000, 2000): {}", min_extern(2000, 2000));
    }

    Ok(())
}
