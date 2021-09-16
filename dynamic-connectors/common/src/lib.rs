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
