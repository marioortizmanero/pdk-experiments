//! This defines the types that must be exported in a crate when creating a
//! plugin.
//!
//! In order to load the plugin, one must ensure that the versions of this crate
//! match for both the runtime and the plugin. Basic metadata such as versioning
//! must be defined with fully stable types. For example, we can't use
//! `abi_stable::RStr` to save the version; there might be a version mismatch of
//! `abi_stable` between the crate and the plugin, and since this happens
//! *before* making sure they're compatible, it may not work.
//!
//! All of this is simplified if the macro `define_plugin` is used.
//!
//! For now, the version checks are done for the entire crate for simplicity,
//! but in the future we can make this more flexible. For instance, if a version
//! bump in this crate only modifies structures for codec plugins, the rest of
//! the plugins would still work. This would require versioning each of the
//! types of plugins and bumping them as they're modified. Note that this is
//! prone to human errors however, as it's a manual process.
//!
//! There may be helpful crates like https://github.com/doctorn/obake.

use abi_stable::{RStr, RSlice, RSliceMut, RVec, ROption};

// TODO: consider all possible syntaxes for this macro, take into account that
// it should be future-proof.
macro_rules! define_plugin {
    (
        name: $name:literal,
        data: $data:expr
    ) => {
        // TODO: do we need to specify the kind of plugin in a string? or can we
        // just use an enum with data fields for now?

        // TODO: research error handling, what happens if the plugin includes
        // incorrect metadata? Can we avoid aborts? I should make more plugins
        // with incorrect values to check that they don't crash the runtime.

        /// This may be checked before version mismatches, so it must use
        /// `*char`.
        #[no_mangle]
        pub static PLUGIN_NAME: *const c_char = concat!($name, "\0");

        /// Same, this must use `*char` in order to be considered stable.
        #[no_mangle]
        pub static PLUGIN_COMMON_VERSION: *const c_char = concat!(env!("PLUGIN_COMMON_VERSION"), "\0");

        /// This should only be loaded after checking for version mismatches, so
        /// we may use custom types now.
        #[no_mangle]
        pub static PLUGIN_DATA: PluginData = $data;
    }
}

// TODO

#[repr(C)]
pub struct Error;

#[repr(C)]
#[derive(Debug, Clone)]
pub enum PluginData<'input> {
    Connector {
        mime_types: RSlice<'static, RStr<'static>>,
        something: unsafe extern fn(data: RSliceMut<'input, u8>) -> i32,
    },
    Codec {

    }
}
