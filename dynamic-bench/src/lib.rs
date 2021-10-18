//! The plugin is given with its full path instead of a directory for more
//! reliable benchmarking.

use common_dynamic_bench::{interface, ConnectorPlugin};

use std::{ffi::CStr, io, mem::ManuallyDrop, os::raw::c_char};

use anyhow::Result;
use libloading::Library;

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("version mismatch: {0} incompatible with {1}")]
    VersionMismatch(String, String),
    #[error("unknown kind of plugin: {0}")]
    UnknownPluginKind(String),
    #[error("input/output: {0}")]
    Io(#[from] io::Error),
}

/// Versions are compatible only if they fully match.
fn version_matches(version: &str) -> bool {
    version == PKG_VERSION
}

/// Obtaining a string from the metadata of the library.
unsafe fn get_str<'a>(library: &'a Library, ident: &[u8]) -> Result<&'a str> {
    // First, the string exported by the plugin is read. For FFI-safety and
    // thread-safety, this must be a function that returns `*const c_char`.
    let name_fn = library.get::<extern "C" fn() -> *const c_char>(ident)?;
    let name: *const c_char = name_fn();

    // Unfortunately there is no way to make sure this part is safe. We have
    // to assume the address exported by the plugin is valid. Otherwise,
    // this part may cause an abort.
    let name = CStr::from_ptr(name);

    // Finally, the string is converted to UTF-8 and returned
    Ok(name.to_str()?)
}

/// For benchmarking reasons, setting up the plugin and running it is a two step
/// process. Thus, the setup is done when calling this function, and it can be
/// ran when calling the returned closure.
pub fn setup_plugin(path: &str) -> Result<impl FnMut(i32, i32) -> i32> {
    unsafe {
        // In order to use the library outside of this setup we need to handle
        // its lifetime properly. However, since plugin unloading is
        // unsupported, in order to simplify this we can just leak the library
        // for now.
        let library = ManuallyDrop::new(Box::new(Library::new(path)?));

        // Making sure we can continue loading data
        let version = get_str(&library, interface::COMMON_VERSION_IDENT)?;
        if !version_matches(version) {
            return Err(Error::VersionMismatch(version.to_owned(), PKG_VERSION.to_owned()).into());
        }

        let kind = get_str(&library, interface::KIND_IDENT)?;
        // This would match against every possible kind of plugin and load it.
        // In this case we only support connectors, though, so we just have an
        // early return.
        if kind != "connector" {
            return Err(Error::UnknownPluginKind(kind.to_owned()).into());
        }

        // Accessing the plugin metadata
        let data = library
            .get::<*const ConnectorPlugin>(interface::DATA_IDENT)?
            .read();

        // Initializing the plugin
        let state = (data.new)();

        // The plugin can be ran at a later time by calling this closure
        Ok(move |a, b| (data.min)(state, a, b))
    }
}
