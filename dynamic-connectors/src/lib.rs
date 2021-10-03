use common_dconnectors::{interface, ConnectorPlugin};

use std::{ffi::{CStr, OsStr}, fs, io, os::raw::c_char, path::{Path, PathBuf}};

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
    #[error("invalid path: {0}")]
    Path(String)
}

/// Versions are compatible only if they fully match.
fn version_matches(version: &str) -> bool {
    version == PKG_VERSION
}

/// We just check the extension depending on the Operating System.
fn extension_matches(file: &Path) -> bool {
    file
        .extension()
        .map(|ext| ext == std::env::consts::DLL_EXTENSION)
        .unwrap_or(false)
}

/// The runtime looks for plugins in a directory non-recursively
pub fn find_plugins<P>(dir: P) -> Result<impl IntoIterator<Item = PathBuf>>
    where
        P: AsRef<Path>,
{
    if !dir.as_ref().is_dir() {
        return Err(Error::Path("not a directory".to_owned()).into());
    }

    let iter = fs::read_dir(dir)?
        .filter_map(|node| Some(node.ok()?.path()))
        .filter(|path| extension_matches(&path));
    Ok(iter)
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
        // This would match against every possible kind of plugin and load it.
        // In this case we only support connectors, though, so we just have an
        // early return.
        if kind != "connector" {
            return Err(Error::UnknownPluginKind(kind.to_owned()).into());
        }

        let data = library
            .get::<*const ConnectorPlugin>(interface::DATA_IDENT)?
            .read();
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
