use abi_stable::std_types::{RStr, RSlice, RSliceMut};

/// These are the identifiers that should be used between the plugin and runtime
/// to import and export functionality.
pub mod idents {
    pub const NAME: &[u8] = b"PLUGIN_NAME";
    pub const COMMON_VERSION: &[u8] = b"PLUGIN_COMMON_VERSION";
    pub const DATA: &[u8] = b"PLUGIN_DATA";
}

// TODO: consider all possible syntaxes for this macro, take into account that
// it should be future-proof.
#[macro_export]
macro_rules! define_plugin {
    (
        name: $name:literal,
        data: $data:expr
    ) => {
        // NOTE: in order to export strings, we have to use the type
        // `*const c_char`; `CStr` is a wrapper and not FFI-safe.

        // TODO: do we need to specify the kind of plugin in a string? or can we
        // just use an enum with data fields for now?

        // TODO: research error handling, what happens if the plugin includes
        // incorrect metadata? Can we avoid aborts? I should make more plugins
        // with incorrect values to check that they don't crash the runtime.

        /// This may be checked before version mismatches, so it must use
        /// `*char`.
        #[no_mangle]
        pub static PLUGIN_NAME: *const ::std::os::raw::c_char
            = concat!($name, "\0").as_ptr() as _;

        /// Same, this may be checked before version mismatches.
        #[no_mangle]
        pub static PLUGIN_COMMON_VERSION: *const ::std::os::raw::c_char
            = concat!(env!("CARGO_PKG_VERSION"), "\0").as_ptr() as _;

        /// This should only be loaded after checking for version mismatches, so
        /// we may use custom types now.
        #[no_mangle]
        pub static PLUGIN_DATA: crate::PluginData = $data;
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
