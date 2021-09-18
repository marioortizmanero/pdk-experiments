use abi_stable::std_types::{RStr, RSlice, RSliceMut};

/// These are the identifiers and types that should be used between the plugin
/// and runtime to import and export functionality.
pub mod interface {
    pub const NAME_IDENT: &[u8] = b"get_name";
    pub const COMMON_VERSION_IDENT: &[u8] = b"get_version";
    pub const KIND_IDENT: &[u8] = b"get_kind";
    pub const DATA_IDENT: &[u8] = b"PLUGIN_DATA";
}

// TODO: consider all possible syntaxes for this macro, take into account that
// it should be future-proof.
#[doc(hidden)]
#[macro_export]
macro_rules! internal_define_plugin {
    (
        name: $name:literal,
        kind: $kind:literal,
        data_type: $data_type:ty,
        data: $data:expr
    ) => {
        // NOTE: in order to export data, it must be FFI-safe. This means that
        // we have to use types such as `*const c_char`. `CStr` is a wrapper and
        // not `repr(C)`, so that won't work, for instance.
        //
        // Furthermore, the exported data must be thread-safe (implement
        // `Sync`). Since `*const c_char` itself does not implement `Sync`
        // because it's a pointer, we have to use a function that returns the
        // string instead.

        static __NAME: &str = concat!($name, "\0");
        static __KIND: &str = concat!($kind, "\0");
        // TODO: this is actually core's version, we need common's version
        static __VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "\0");

        // TODO: do we need to specify the kind of plugin in a string? or can we
        // just use an enum with data fields for now?

        // TODO: research error handling, what happens if the plugin includes
        // incorrect metadata? Can we avoid aborts? I should make more plugins
        // with incorrect values to check that they don't crash the runtime.

        #[no_mangle]
        pub extern fn get_name() -> *const ::std::os::raw::c_char {
            __NAME.as_ptr() as _
        }

        #[no_mangle]
        pub extern fn get_kind() -> *const ::std::os::raw::c_char {
            __KIND.as_ptr() as _
        }

        #[no_mangle]
        pub extern fn get_version() -> *const ::std::os::raw::c_char {
            __VERSION.as_ptr() as _
        }

        /// This should only be loaded after checking for version mismatches, so
        /// we may use custom types now.
        ///
        /// The `PluginData` type is thread-safe, so we don't need to use a
        /// function for this.
        #[no_mangle]
        pub static PLUGIN_DATA: $data_type = $data;
    }
}

/// This macro helps define a connector plugin consistently without the usual
/// boilerplate that is required.
#[macro_export]
macro_rules! define_connector_plugin {
    (
        name: $name:literal,
        data: $data:expr
    ) => {

        $crate::internal_define_plugin! {
            name: $name,
            kind: "connector",
            data_type: ConnectorPlugin,
            data: $data
        }
    }
}

// TODO

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ConnectorPlugin<'input> {
    pub mime_types: RSlice<'static, RStr<'static>>,
    pub something: unsafe extern fn(data: RSliceMut<'input, u8>) -> i32,
}
