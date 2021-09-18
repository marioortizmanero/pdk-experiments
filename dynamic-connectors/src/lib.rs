use common_dconnectors::{interface, ConnectorPlugin};

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
    VersionMismatch(String, String),
    #[error("unknown kind of plugin: {0}")]
    UnknownKind(String)
}

// This may be more advanced in the future. If semantic versioning is strictly
// followed in Tremor, we could ignore minor or patch upgrades.
fn version_matches(version: &str) -> bool {
    version == PKG_VERSION
}

unsafe fn get_str<'a>(library: &'a Library, ident: &[u8]) -> Result<&'a str> {
    // First, the string exported by the plugin is read. For FFI-safety and
    // thread-safety, this must be a function that returns `*const c_char`.
    let name_fn = library.get::<extern fn() -> *const c_char>(ident)?;
    let name: *const c_char = name_fn();

    // Unfortunately there is no way to make sure this part is safe. We have
    // to assume the address exported by the plugin is valid. Otherwise,
    // this part may cause an abort.
    let name = CStr::from_ptr(name);

    // Finally, the string is converted to UTF-8 and returned
    Ok(name.to_str()?)
}

// TODO: docs
pub fn setup_plugin(path: &str) -> Result<impl Fn() -> Result<()>> {
    unsafe {
        let library = Library::new(path)?;

        let name = get_str(&library, interface::NAME_IDENT)?;
        // TODO: use `log`
        println!("Initializing plugin {}", name);

        // Making sure we can continue loading data
        let version = get_str(&library, interface::COMMON_VERSION_IDENT)?;
        if !version_matches(version) {
            println!("Version mismatch. Aborting.");
            return Err(Error::VersionMismatch(version.to_owned(), PKG_VERSION.to_owned()).into());
        }

        let kind = get_str(&library, interface::KIND_IDENT)?;
        // This would match agains every possible kind of plugin and load it. In
        // this case we only support connectors, though, so we just have an
        // early return.
        if kind != "connector" {
            return Err(Error::UnknownKind(kind.to_owned()).into());
        }

        let data = library.get::<*const ConnectorPlugin>(interface::DATA_IDENT)?.read();
        println!("Plugin data: {:?}", data);

        // TODO: How to deal with lifetimes? We want to avoid the library from
        // being dropped. We also want to be able to call this function multiple
        // times, and concurrently, for the tests.
        Ok(move || {
            println!("Running plugin");

            Ok(())
        })
    }
}
