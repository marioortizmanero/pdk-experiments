use abi_stable::std_types::RStr;

use anyhow::Result;
use libloading::Library;

pub fn run_plugin(path: &str) -> Result<()> {
    unsafe {
        let library = Library::new(path)?;

        // First, the string exported by the plugin is read. It's read using the
        // definition from `abi_stable` v0.8, but the plugin uses v0.9.
        let shared = library.get::<*const RStr>(b"SHARED")?;
        println!("Variable in the plugin: '{}'", **shared);

        Ok(())
    }
}
