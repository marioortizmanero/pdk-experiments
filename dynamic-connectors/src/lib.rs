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
    Io(#[from] io::Error)
}

/// This may be more advanced in the future. If semantic versioning is strictly
/// followed in Tremor, we could ignore minor or patch upgrades.
fn version_matches(version: &str) -> bool {
    version == PKG_VERSION
}

// TODO: how should we handle extensions? Should we use a single one such as
// `.module`, or `.tremorplugin`, or platform-dependent ones such as `.dll`,
// `.so` and `.mod`?
//
// Using a single one could be easier to distribute but then we might run into
// problems if a plugin isn't cross-platform. For example, a dev could create a
// plugin that's only available for Windows, but since it's using the same
// extension, Tremor would attempt to load it on other platforms such as Linux.
fn extension_matches(file: &Path) -> bool {
    match file.extension() {
        Some(ext) => true,
        _ => false
    }

    if cfg!(linux) {
        "so"
    } else if cfg!(windows) {
        "dll"
    } else if cfg!(darwin) {
        "mod"
    }
}

// TODO: how should we look for plugins? can we do it recursively or could this
// crash (e.g. the user selects `/` as the path)?
/// Recursively looks for plugins in a directory
fn find_plugins<P, S>(dir: P) -> Result<impl IntoIterator<Item = S>>
    where
        P: AsRef<Path>,
        S: AsRef<PathBuf>
{
    // TODO
    if dir.is_dir() {
        fs::read_dir(dir)?
            .map(|node| node.map(|node| node.path()
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                find_plugins(&path)?;
            }
        }
    }
    Ok(())
}

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
