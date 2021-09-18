use common_dconnectors::idents;

use std::{
    os::raw::c_char,
    ffi::CStr
};

use anyhow::Result;
use libloading::Library;

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("version mismatch: {0} incompatible with {1}")]
    VersionMismatch(String, String)
}

// This may be more advanced in the future. If semantic versioning is strictly
// followed in Tremor, we could ignore minor or patch upgrades.
fn version_matches(version: &str) -> bool {
    version == PKG_VERSION
}

// TODO: docs
pub fn setup_plugin(path: &str) -> Result<impl Fn() -> Result<()>> {
    unsafe {
        let library = Library::new(path)?;

        // First, the string exported by the plugin is read. For FFI-safety,
        // this must be a `*const c_char`.
        let name = library.get::<*const c_char>(idents::NAME)?;
        // Unfortunately there is no way to make sure this part is safe. We have
        // to assume the address exported by the plugin is valid. Otherwise,
        // this part may cause an abort.
        let name = CStr::from_ptr(*name);
        // Then, the string is converted to UTF-8.
        let name = name.to_str()?;

        // TODO: use `log`
        println!("Initializing plugin {}", name);

        // The same process is applied to the version string.
        let version = library.get::<*const c_char>(idents::COMMON_VERSION)?;
        let version = CStr::from_ptr(*version);
        let version = version.to_str()?;

        // Making sure we can continue loading data
        if !version_matches(version) {
            println!("Version mismatch. Aborting.");
            return Err(
        }

        let data = library.get::<*const c_char>(idents::DATA)?;

        Ok(move || {
            println!("Running plugin");
            // TODO

            Ok(())
        })
    }
}
