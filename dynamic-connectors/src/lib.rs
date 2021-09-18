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

// TODO: docs
pub fn setup_plugin(path: &str) -> Result<impl Fn() -> Result<()>> {
    unsafe {
        let library = Library::new(path)?;

        // First, the string exported by the plugin is read. For FFI-safety and
        // thread-safety, this must be a function that returns `*const c_char`.
        let name_fn = library.get::<interface::NameFn>(interface::NAME_IDENT)?;
        let name: *const c_char = name_fn();
        // Unfortunately there is no way to make sure this part is safe. We have
        // to assume the address exported by the plugin is valid. Otherwise,
        // this part may cause an abort.
        let name = CStr::from_ptr(name);
        // Then, the string is converted to UTF-8.
        let name = name.to_str()?;

        // TODO: use `log`
        println!("Initializing plugin {}", name);

        // The same process is applied to the version string.
        let version_fn = library.get::<interface::CommonVersionFn>(interface::COMMON_VERSION_IDENT)?;
        let version = version_fn();
        let version = CStr::from_ptr(version);
        let version = version.to_str()?;

        // Making sure we can continue loading data
        if !version_matches(version) {
            println!("Version mismatch. Aborting.");
            return Err(Error::VersionMismatch(version.to_owned(), PKG_VERSION.to_owned()).into());
        }

        // And to the kind of plugin we're loading
        let kind_fn = library.get::<interface::KindFn>(interface::KIND_IDENT)?;
        let kind = kind_fn();
        let kind = CStr::from_ptr(kind);
        let kind = kind.to_str()?;

        match kind {
            "connector" => {
                let data = library.get::<*const ConnectorPlugin>(interface::DATA_IDENT)?.read();
                println!("Plugin data: {:?}", data);

            },
            _ => return Err(Error::UnknownKind(kind.to_owned()).into())
        }

        Ok(move || {
            println!("Running plugin");
            // TODO

            Ok(())
        })
    }
}
